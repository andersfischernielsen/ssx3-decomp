//! Staging module oc1527 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1527`.

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
    (0x001f_f7a8, tail_return_0x101, "lbl_001FF7A8"),
    (0x0025_63f8, dnas_auth_update, "DNASAuth_Update"),
    (0x002c_7478, c_ps2_console_config_ctor, "cPS2ConsoleConfig_ctor"),
];

/// `0x001ff7a8` (no name; the shared tail of the code before
/// `OVScreenRewardList_GetEntryText`): returns 0x101 after reloading `ra` from
/// the caller's 0x10-byte frame. Entries: 0x001ff7a8 (the start) and 0x001ff7ac
/// (the return); any other `pc` is a jump out.
pub fn tail_return_0x101(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x001ff7a8 => {
                c.cycles += 1;
                c.g[V0][0] = 0x101;
                pc = 0x001ff7ac;
                continue;
            }
            0x001ff7ac => {
                c.cycles += 3;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.pc = t;
                return;
            }
            _ => {
                c.jump_out(pc, 0x001ff7a8, 0x001ff7b8);
                return;
            }
        }
    }
}

/// `void DNASAuth_Update(DNASAuth* self)` (0x002563f8): runs one step of the DNAS
/// authentication state machine, chosen by the state word at +0: 0 loads the
/// overlay, 1 the config, 2 authenticates (the jump table at 0x480410), other
/// states do nothing. Kept as the generated block machine, so every resume
/// entry (after an unwound callee) also runs natively.
pub fn dnas_auth_update(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x002563f8 => {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[A1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x480000;
                if t { pc = 0x00256454; continue; }
                pc = 0x00256414; continue;
            }
            0x00256414 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x410));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                let t = c.g[A0][0] as u32;
                pc = t; continue;
            }
            0x0025642c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00256434);
                c.g[A0][0] = c.g[A1][0];
                call_ptr(c, 0x00256460, 0x00256434)?;
                let t = true;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                if t { pc = 0x00256458; continue; }
                pc = 0x0025643c; continue;
            }
            0x0025643c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00256444);
                c.g[A0][0] = c.g[A1][0];
                call_ptr(c, 0x00256510, 0x00256444)?;
                let t = true;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                if t { pc = 0x00256458; continue; }
                pc = 0x0025644c; continue;
            }
            0x0025644c => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x00256454);
                c.g[A0][0] = c.g[A1][0];
                call_ptr(c, 0x002565e0, 0x00256454)?;
                pc = 0x00256454; continue;
            }
            0x00256454 => {
                c.cycles += 1;
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                pc = 0x00256458; continue;
            }
            0x00256458 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x002563f8, 0x00256460); return Ok(()); }
        }
    }
    });
}

/// `cPS2ConsoleConfig::cPS2ConsoleConfig(this)` (0x002c7478): fills the console
/// config from the system queries: +0x24 = 0, +0x34 = the string at 0x486798,
/// +0x28 (video mode flag), +0x2c = 1, +0x30 (region index through the jump table
/// at 0x486580, 0..7), +0 and +4 (language and aspect ratio), then the
/// date/time bytes (BCD) into +8..+0x1c (a default 2000-ish date when the query
/// fails), and finally +0x20 = world->+0x18. Returns this. Kept as the generated
/// block machine, so every resume entry also runs natively.
pub fn c_ps2_console_config_ctor(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x002c7478 => {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.g[V0][0] = 0x480000;
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6798));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S5][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.sw((c.g[S5][0] as u32).wrapping_add(0x24), 0u64);
                c.g[RA][0] = sx(0x002c74b0);
                c.sw((c.g[S5][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                call_ptr(c, 0x0040baa8, 0x002c74b0)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] != 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x002c74c8; continue; }
                pc = 0x002c74bc; continue;
            }
            0x002c74bc => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let t = true;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                if t { pc = 0x002c74cc; continue; }
                pc = 0x002c74c8; continue;
            }
            0x002c74c8 => {
                c.cycles += 1;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x28), 0u64);
                pc = 0x002c74cc; continue;
            }
            0x002c74cc => {
                c.cycles += 7;
                c.g[S0][0] = 0x1;
                c.g[RA][0] = sx(0x002c74d8);
                c.sw((c.g[S5][0] as u32).wrapping_add(0x2c), c.g[S0][0]);
                call_ptr(c, 0x0040b9e8, 0x002c74d8)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x480000;
                if t { pc = 0x002c755c; continue; }
                pc = 0x002c74e8; continue;
            }
            0x002c74e8 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6580));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                let t = c.g[A0][0] as u32;
                pc = t; continue;
            }
            0x002c7500 => {
                c.cycles += 2;
                let t = true;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x30), 0u64);
                if t { pc = 0x002c7560; continue; }
                pc = 0x002c7508; continue;
            }
            0x002c7508 => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let t = true;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                if t { pc = 0x002c7560; continue; }
                pc = 0x002c7514; continue;
            }
            0x002c7514 => {
                c.cycles += 3;
                c.g[V0][0] = 0x2;
                let t = true;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                if t { pc = 0x002c7560; continue; }
                pc = 0x002c7520; continue;
            }
            0x002c7520 => {
                c.cycles += 3;
                c.g[V0][0] = 0x3;
                let t = true;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                if t { pc = 0x002c7560; continue; }
                pc = 0x002c752c; continue;
            }
            0x002c752c => {
                c.cycles += 3;
                c.g[V0][0] = 0x4;
                let t = true;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                if t { pc = 0x002c7560; continue; }
                pc = 0x002c7538; continue;
            }
            0x002c7538 => {
                c.cycles += 3;
                c.g[V0][0] = 0x5;
                let t = true;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                if t { pc = 0x002c7560; continue; }
                pc = 0x002c7544; continue;
            }
            0x002c7544 => {
                c.cycles += 3;
                c.g[V0][0] = 0x6;
                let t = true;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                if t { pc = 0x002c7560; continue; }
                pc = 0x002c7550; continue;
            }
            0x002c7550 => {
                c.cycles += 3;
                c.g[V0][0] = 0x7;
                let t = true;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                if t { pc = 0x002c7560; continue; }
                pc = 0x002c755c; continue;
            }
            0x002c755c => {
                c.cycles += 1;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x30), c.g[S0][0]);
                pc = 0x002c7560; continue;
            }
            0x002c7560 => {
                c.cycles += 6;
                c.g[RA][0] = sx(0x002c7568);
                call_ptr(c, 0x0040bb38, 0x002c7568)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = 0x1;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                if t { pc = 0x002c758c; continue; }
                pc = 0x002c7578; continue;
            }
            0x002c7578 => {
                c.cycles += 2;
                if c.g[V0][0] != 0u64 { c.sw((c.g[S5][0] as u32), 0u64); pc = 0x002c7590; continue; }
                pc = 0x002c7580; continue;
            }
            0x002c7580 => {
                c.cycles += 3;
                c.g[V0][0] = 0x2;
                if c.g[V1][0] != c.g[V0][0] { c.sw((c.g[S5][0] as u32), 0u64); pc = 0x002c7590; continue; }
                pc = 0x002c758c; continue;
            }
            0x002c758c => {
                c.cycles += 1;
                c.sw((c.g[S5][0] as u32), c.g[V1][0]);
                pc = 0x002c7590; continue;
            }
            0x002c7590 => {
                c.cycles += 5;
                c.g[RA][0] = sx(0x002c7598);
                call_ptr(c, 0x0040bc08, 0x002c7598)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x1;
                if t { pc = 0x002c75b4; continue; }
                pc = 0x002c75a4; continue;
            }
            0x002c75a4 => {
                c.cycles += 2;
                if c.g[V1][0] != c.g[V0][0] { c.sw((c.g[S5][0] as u32).wrapping_add(0x4), 0u64); pc = 0x002c75b8; continue; }
                pc = 0x002c75ac; continue;
            }
            0x002c75ac => {
                c.cycles += 2;
                let t = true;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                if t { pc = 0x002c75b8; continue; }
                pc = 0x002c75b4; continue;
            }
            0x002c75b4 => {
                c.cycles += 1;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x4), 0u64);
                pc = 0x002c75b8; continue;
            }
            0x002c75b8 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002c75c0);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00402520, 0x002c75c0)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x7d3;
                if t { pc = 0x002c766c; continue; }
                pc = 0x002c75c8; continue;
            }
            0x002c75c8 => {
                c.cycles += 41;
                c.g[RA][0] = sx(0x002c75d0);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0040bfe0, 0x002c75d0)?;
                c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x7));
                c.g[V1][0] = 0xa;
                c.g[T2][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x3));
                c.g[T3][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x2));
                c.g[T1][0] = sx((c.g[V0][0] as u32) >> 4);
                c.g[A3][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x1));
                c.g[T6][0] = sx((c.g[T2][0] as u32) >> 4);
                c.g[T0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x6));
                c.g[T5][0] = sx((c.g[T3][0] as u32) >> 4);
                c.g[A0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x5));
                c.g[T4][0] = sx((c.g[A3][0] as u32) >> 4);
                c.g[A1][0] = sx((c.g[T0][0] as u32) >> 4);
                c.mult(0, 9, 9, 3);
                c.g[A2][0] = sx((c.g[A0][0] as u32) >> 4);
                c.mult(0, 14, 14, 3);
                c.mult(0, 6, 6, 3);
                c.g[V0][0] = c.g[V0][0] & 0xf;
                c.mult(0, 13, 13, 3);
                c.g[T2][0] = c.g[T2][0] & 0xf;
                c.mult(0, 12, 12, 3);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
                c.mult(0, 5, 5, 3);
                c.g[T3][0] = c.g[T3][0] & 0xf;
                c.g[A3][0] = c.g[A3][0] & 0xf;
                c.g[T0][0] = c.g[T0][0] & 0xf;
                c.g[A0][0] = c.g[A0][0] & 0xf;
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[T6][0] as u32)));
                c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[T5][0] as u32)));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[T4][0] as u32)));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7d0));
                c.sw((c.g[S5][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                c.sw((c.g[S5][0] as u32).wrapping_add(0xc), c.g[T3][0]);
                c.sw((c.g[S5][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                c.sw((c.g[S5][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.sw((c.g[S5][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                let t = true;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                if t { pc = 0x002c768c; continue; }
                pc = 0x002c766c; continue;
            }
            0x002c766c => {
                c.cycles += 8;
                c.g[V1][0] = 0xa;
                c.g[A0][0] = 0xe;
                c.sw((c.g[S5][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                c.sw((c.g[S5][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                c.sw((c.g[S5][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                c.sw((c.g[S5][0] as u32).wrapping_add(0x8), 0u64);
                c.sw((c.g[S5][0] as u32).wrapping_add(0xc), 0u64);
                c.sw((c.g[S5][0] as u32).wrapping_add(0x10), 0u64);
                pc = 0x002c768c; continue;
            }
            0x002c768c => {
                c.cycles += 13;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V0][0] = c.g[S5][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sw((c.g[S5][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x002c7478, 0x002c76c0); return Ok(()); }
        }
    }
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, elf_image, fill, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (own address and magic in the
    /// machine's RAM, so other tests' machines run the real bodies). The
    /// results the fakes return sit after it: word `16 + 4 * index` holds
    /// `1 + value` (0 = leave the fakes' default).
    const OC1527_CTRL: u32 = DATA + 0x2e_0000;
    const OC1527_MAGIC: u32 = 0x1527_f00d;

    /// The logged call; the result comes from the target's mailbox word.
    fn oc1527_fake(c: &mut Cpu, target: u32, idx: u32) {
        fakes::fake_call(c, target);
        let w = c.bus.read32(OC1527_CTRL + 0x10 + 4 * idx);
        if w != 0 {
            c.g[V0][0] = (w - 1) as u64;
        }
    }

    /// Forces the result of the callee with fake index `idx`.
    fn oc1527_result(c: &mut Cpu, idx: u32, value: u32) {
        c.bus.write32(OC1527_CTRL + 0x10 + 4 * idx, value + 1);
    }

    /// Hooked fakes, scrambled registers, the game image, the gate on.
    fn oc1527_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> crate::difftest::Rng {
        oc1527_hook();
        let r = machine(c, seed, salt, calls);
        elf_image(c);
        c.bus.write32(OC1527_CTRL, OC1527_MAGIC);
        r
    }

    fn oc1527_fake_00256460(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1527_fake_00256460), ee::s06::f_00256460__generated);
        }
        oc1527_fake(c, 0x00256460, 0);
    }

    fn oc1527_fake_00256510(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1527_fake_00256510), ee::s06::f_00256510__generated);
        }
        oc1527_fake(c, 0x00256510, 1);
    }

    fn oc1527_fake_002565e0(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1527_fake_002565e0), ee::s06::f_002565e0__generated);
        }
        oc1527_fake(c, 0x002565e0, 2);
    }

    fn oc1527_fake_0040baa8(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1527_fake_0040baa8), ee::s15::f_0040baa8__generated);
        }
        oc1527_fake(c, 0x0040baa8, 3);
    }

    fn oc1527_fake_0040b9e8(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1527_fake_0040b9e8), ee::s15::f_0040b9e8__generated);
        }
        oc1527_fake(c, 0x0040b9e8, 4);
    }

    fn oc1527_fake_0040bb38(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1527_fake_0040bb38), ee::s15::f_0040bb38__generated);
        }
        oc1527_fake(c, 0x0040bb38, 5);
    }

    fn oc1527_fake_0040bc08(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1527_fake_0040bc08), ee::s15::f_0040bc08__generated);
        }
        oc1527_fake(c, 0x0040bc08, 6);
    }

    fn oc1527_fake_00402520(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1527_fake_00402520), ee::s15::f_00402520__generated);
        }
        oc1527_fake(c, 0x00402520, 7);
    }

    fn oc1527_fake_0040bfe0(c: &mut Cpu) {
        if c.bus.read32(OC1527_CTRL) != OC1527_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc1527_fake_0040bfe0), ee::s15::f_0040bfe0__generated);
        }
        oc1527_fake(c, 0x0040bfe0, 8);
    }

    /// Puts every fake in its override slot (once per process).
    fn oc1527_hook() {
        fakes::hook(addr_of_mut!(ee::s06::OVERRIDE_00256460), oc1527_fake_00256460, fakes::prev_cell(oc1527_fake_00256460));
        fakes::hook(addr_of_mut!(ee::s06::OVERRIDE_00256510), oc1527_fake_00256510, fakes::prev_cell(oc1527_fake_00256510));
        fakes::hook(addr_of_mut!(ee::s06::OVERRIDE_002565E0), oc1527_fake_002565e0, fakes::prev_cell(oc1527_fake_002565e0));
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_0040BAA8), oc1527_fake_0040baa8, fakes::prev_cell(oc1527_fake_0040baa8));
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_0040B9E8), oc1527_fake_0040b9e8, fakes::prev_cell(oc1527_fake_0040b9e8));
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_0040BB38), oc1527_fake_0040bb38, fakes::prev_cell(oc1527_fake_0040bb38));
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_0040BC08), oc1527_fake_0040bc08, fakes::prev_cell(oc1527_fake_0040bc08));
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_00402520), oc1527_fake_00402520, fakes::prev_cell(oc1527_fake_00402520));
        fakes::hook(addr_of_mut!(ee::s15::OVERRIDE_0040BFE0), oc1527_fake_0040bfe0, fakes::prev_cell(oc1527_fake_0040bfe0));
    }

    /// `0x001ff7a8`: from the start, from the return entry and from an
    /// address that is no entry (a jump out).
    #[test]
    fn tail_return_0x101_matches() {
        check(
            ee::s05::f_001ff7a8__generated,
            tail_return_0x101,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1ff7a8, 0);
                let sp = c.g[SP][0] as u32;
                fill_frame(c, sp, seed);
                c.pc = match seed % 3 {
                    0 => 0x001f_f7a8,
                    1 => 0x001f_f7ac,
                    _ => 0x001f_f7b0,
                };
            },
        );
    }

    /// A saved `ra` in the frame at `sp`.
    fn fill_frame(c: &mut Cpu, sp: u32, seed: u32) {
        let mut r = crate::difftest::Rng(0x1ff7a8 ^ seed as u64);
        fill(c, &mut r, sp, 4);
    }

    /// `DNASAuth_Update`: every state (0, 1, 2 through the jump table, 3..5
    /// no-op arms of the table, 6+ and negative skip it), the start and each
    /// resume entry, and an unwind at each call.
    #[test]
    fn dnas_auth_update_matches() {
        const RESUMES: [u32; 6] = [
            0x0025_6414,
            0x0025_642c,
            0x0025_643c,
            0x0025_644c,
            0x0025_6454,
            0x0025_6458,
        ];
        check(
            ee::s06::f_002563f8__generated,
            dnas_auth_update,
            240,
            &|c, seed| {
                let _ = oc1527_machine(c, seed, 0x2563f8, 2);
                let obj = DATA + 0x1000;
                let state = match seed % 9 {
                    8 => 0xffff_fff0,
                    n => n,
                };
                c.bus.write32(obj, state);
                c.g[A0][0] = obj as u64;
                c.pc = 0x0025_63f8;
                if seed % 5 == 4 {
                    c.pc = RESUMES[(seed / 5) as usize % RESUMES.len()];
                    c.g[A1][0] = obj as u64;
                    c.g[V1][0] = sx(state);
                    c.g[V0][0] = 0x0048_0000;
                }
            },
        );
    }

    /// The paths of the state machine: the cycle totals differ per state.
    #[test]
    fn dnas_auth_update_fixture_covers_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..60 {
            let mut c = crate::difftest::fresh();
            let _ = oc1527_machine(&mut c, seed, 0x2563f8, 0);
            let obj = DATA + 0x1000;
            c.bus.write32(obj, seed % 9);
            c.g[A0][0] = obj as u64;
            c.pc = 0x0025_63f8;
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s06::f_002563f8__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 4, "fixture reaches too few paths: {}", seen.len());
    }

    /// The world the constructor reads at the end.
    const OC1527_WORLD: u32 = DATA + 0x1800;

    /// Seeds the constructor fixture: `this` at DATA+0x1000, every callee's
    /// result chosen by mixed radices of the seed, resume starts for some seeds.
    fn oc1527_ctor_setup(c: &mut Cpu, seed: u32, calls: u32) {
        const RESUMES: [u32; 13] = [
            0x002c_74bc,
            0x002c_74c8,
            0x002c_74cc,
            0x002c_7500,
            0x002c_7508,
            0x002c_755c,
            0x002c_7560,
            0x002c_7578,
            0x002c_7590,
            0x002c_75b4,
            0x002c_75b8,
            0x002c_75c8,
            0x002c_768c,
        ];
        let mut r = oc1527_machine(c, seed, 0x2c7478, calls);
        let this = DATA + 0x1000;
        c.g[A0][0] = this as u64;
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC1527_WORLD);
        c.bus.write32(OC1527_WORLD + 0x18, r.next_u32());
        // 0x40baa8 (video flag), 0x40b9e8 (region 0..9), 0x40bb38 (language
        // 0..3), 0x40bc08 (aspect 0..2), 0x402520 (date query), 0x40bfe0.
        oc1527_result(c, 3, if seed % 3 == 0 { 0 } else { 5 });
        oc1527_result(c, 4, (seed / 3) % 10);
        oc1527_result(c, 5, (seed / 5) % 4);
        oc1527_result(c, 6, (seed / 7) % 3);
        oc1527_result(c, 7, (seed / 11) % 2);
        c.pc = 0x002c_7478;
        if seed % 7 == 6 {
            c.pc = RESUMES[(seed / 7) as usize % RESUMES.len()];
            c.g[S5][0] = this as u64;
            c.g[S0][0] = 1;
        }
    }

    /// `cPS2ConsoleConfig_ctor`: every arm of the region jump table (0..7 and
    /// out of range), the language and aspect arms, the date present and
    /// absent, the resume entries and an unwind at each of the six calls.
    #[test]
    fn c_ps2_console_config_ctor_matches() {
        check(
            ee::s08::f_002c7478__generated,
            c_ps2_console_config_ctor,
            400,
            &|c, seed| oc1527_ctor_setup(c, seed, 6),
        );
    }

    /// The constructor fixture reaches many distinct paths.
    #[test]
    fn c_ps2_console_config_ctor_fixture_covers_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..400 {
            if seed % 7 == 6 {
                continue;
            }
            let mut c = crate::difftest::fresh();
            oc1527_ctor_setup(&mut c, seed, 0);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s08::f_002c7478__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 12, "fixture reaches too few paths: {}", seen.len());
    }
}
