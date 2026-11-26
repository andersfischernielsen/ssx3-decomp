//! Staging module oc833 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc833`.

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
    0x0018_8f28,
    oc833_fe_state_options_game_init,
    "cFEStateOptionsGame_Init",
)];

/// Address of the options record the three setting rows copy (`getOptions`).
#[cfg(test)]
pub(crate) const OC833_OPTIONS: u32 = 0x0053_5610;

/// `lw 0x40(s1)`, then the call `UIScreen_FindObject(screen, v0)` (the hash
/// the previous call returned) returning to `ret`.
fn oc833_find(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret)
}

/// Row setup: the string-table entry of the row's slot, hashed, and its list widget found.
fn oc833_slot_lookup(c: &mut Cpu, ret_hash: u32, ret_find: u32) -> Flow {
    c.cycles += 13;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfd8));
    c.g[S2][0] = 0x440000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[S3][0] = 0x440000;
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    call_ptr(c, 0x0031_7670, ret_hash)?;
    oc833_find(c, ret_find)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 0x460000;
    Ok(())
}

/// One `addEntryByStringID(list, hash(str), index)`: the string pointer is
/// `0x460000 - sub` in `a0`.
fn oc833_add_entry(c: &mut Cpu, sub: u32, idx: u64, r_hash: u32, r_add: u32) -> Flow {
    c.g[A0][0] = 0x460000;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(sub));
    call_ptr(c, 0x0031_7670, r_hash)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = idx;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0039_a568, r_add)
}

/// The inlined `getOptions()` copy (0x288 bytes from 0x535610 to `sp+0x10`)
/// in its aligned form (the source address is a constant with `& 7 == 0`, so
/// the unaligned word loop of the generated body is never taken). `regs`
/// are the registers loaded from +0/+8/+0x10/+0x18 of each 32-byte block,
/// `head` the loop head address.
fn oc833_copy_options(c: &mut Cpu, regs: [usize; 4], head: u32) {
    c.g[V1][0] = 0x530000;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
    c.g[V0][0] = c.g[V1][0] & 0x7;
    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.cycles += 1;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x280));
    loop {
        c.cycles += 11;
        let v1 = c.g[V1][0] as u32;
        let a0 = c.g[A0][0] as u32;
        c.g[regs[0]][0] = c.ld(v1);
        c.g[regs[1]][0] = c.ld(v1.wrapping_add(0x8));
        c.g[regs[2]][0] = c.ld(v1.wrapping_add(0x10));
        c.g[regs[3]][0] = c.ld(v1.wrapping_add(0x18));
        c.sd(a0, c.g[regs[0]][0]);
        c.sd(a0.wrapping_add(0x8), c.g[regs[1]][0]);
        c.sd(a0.wrapping_add(0x10), c.g[regs[2]][0]);
        c.sd(a0.wrapping_add(0x18), c.g[regs[3]][0]);
        c.g[V1][0] = sx(v1.wrapping_add(0x20));
        let again = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = sx(a0.wrapping_add(0x20));
        if !again {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The last 8 bytes of the options copy (unaligned form).
fn oc833_copy_tail(c: &mut Cpu, tail: usize) {
    let v1 = c.g[V1][0] as u32;
    let a0 = c.g[A0][0] as u32;
    c.ldl(tail, v1.wrapping_add(0x7));
    c.ldr(tail, v1);
    c.sdl(tail, a0.wrapping_add(0x7));
    c.sdr(tail, a0);
}

/// Label row: `setUnicodeStringByID(find(table[slot]), hash(string))` for the
/// two tables (`s2`/`s3` hold 0x440000).
#[allow(clippy::too_many_arguments)]
fn oc833_label(
    c: &mut Cpu,
    off: u32,
    base: usize,
    imm: u32,
    r1: u32,
    r2: u32,
    imm2: u32,
    r3: u32,
    r4: u32,
) -> Flow {
    c.cycles += 12;
    c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(off));
    c.g[V1][0] = sx((c.g[base][0] as u32).wrapping_add(imm));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
    call_ptr(c, 0x0031_7670, r1)?;
    oc833_find(c, r2)?;
    c.g[S0][0] = c.g[V0][0];
    let missing = c.g[S0][0] == 0u64;
    c.g[V0][0] = 0x440000;
    if !missing {
        c.cycles += 5;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(imm2));
        call_ptr(c, 0x0031_7670, r3)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, r4)?;
    }
    Ok(())
}

/// `cFEStateOptionsGame_Init(self)` (`onCreateScreen`): creates the "game
/// options" screen, hides two objects, selects menu entry 1, and fills the
/// three setting rows (language: skipped, its slot is -1; units: mph/kph
/// list; widescreen: three entries) with their list entries, current
/// selection from the options record at 0x535610, and the two label texts.
/// The row slots at `self+0x48..0x4a` are stored by the function itself and
/// re-read after the calls, so every row is ported (the language row is
/// reached only if a callee changed the slot).
pub fn oc833_fe_state_options_game_init(c: &mut Cpu) {
    if c.pc != 0x0018_8f28 {
        return ee::s02::f_00188f28__entries(c);
    }
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2f0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
        c.g[A0][0] = 0x460000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x2a0), c.g[RA][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2130));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        call_ptr(c, 0x0031_7670, 0x0018_8f54)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x0039_7cc0, 0x0018_8f68)?;
        let none = c.g[V0][0] == 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        if !none {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0u64;
            call_ptr(c, 0x0039_c870, 0x0018_8f80)?;
        }
        c.cycles += 44;
        call_ptr(c, 0x0014_dc80, 0x0018_8f88)?;
        c.g[A1][0] = 0x4;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0018_8f94)?;
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[A1][0] = 0x2;
        c.g[V0][0] = 0x1;
        c.g[A0][0] = 0x4a0000;
        c.sb((c.g[S1][0] as u32).wrapping_add(0x48), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14f8));
        c.sb((c.g[S1][0] as u32).wrapping_add(0x4a), c.g[A1][0]);
        c.sb((c.g[S1][0] as u32).wrapping_add(0x49), c.g[V0][0]);
        call_ptr(c, 0x0031_7670, 0x0018_8fb8)?;
        oc833_find(c, 0x0018_8fc4)?;
        c.g[A0][0] = 0x4a0000;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1500));
        call_ptr(c, 0x0031_7670, 0x0018_8fd4)?;
        oc833_find(c, 0x0018_8fe0)?;
        // uiObj->setVisible(0); uiObj->setStatic(1) (vtable slots 0x48 and 0x40)
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0018_8ff8)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0018_9010)?;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
        call_ptr(c, 0x0031_7670, 0x0018_901c)?;
        oc833_find(c, 0x0018_9028)?;
        let none = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[V0][0];
        if !none {
            c.cycles += 2;
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0039_b960, 0x0018_9038)?;
        }

        // Row 0: language list (slot -1 as stored above).
        c.cycles += 3;
        c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x48));
        let neg = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0x440000;
        if !neg {
            oc833_slot_lookup(c, 0x0018_9060, 0x0018_906c)?;
            if c.g[S0][0] != 0u64 {
                c.cycles += 32;
                oc833_add_entry(c, 0x2120, 0, 0x0018_9080, 0x0018_9090)?;
                oc833_add_entry(c, 0x2110, 1, 0x0018_909c, 0x0018_90ac)?;
                oc833_add_entry(c, 0x2100, 2, 0x0018_90b8, 0x0018_90c8)?;
                oc833_add_entry(c, 0x20f0, 3, 0x0018_90d4, 0x0018_90e4)?;
                oc833_copy_options(c, [A1, A2, A3, T0], 0x0018_9154);
                c.cycles += 12;
                oc833_copy_tail(c, V0);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 22);
                c.g[A1][0] &= 0x7;
                call_ptr(c, 0x0039_a768, 0x0018_91a8)?;
                let missing = c.g[V0][0] == 0u64;
                c.g[A1][0] = c.lbu(c.g[SP][0] as u32);
                if !missing {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0039_a7a8, 0x0018_91b8)?;
                }
            }
            oc833_label(
                c,
                0x48,
                S2,
                0xfb8,
                0x0018_91d0,
                0x0018_91dc,
                0xfc8,
                0x0018_91f0,
                0x0018_91fc,
            )?;
            oc833_label(
                c,
                0x48,
                S3,
                0xff8,
                0x0018_9214,
                0x0018_9220,
                0xfe8,
                0x0018_9234,
                0x0018_9240,
            )?;
        }

        // Row 1: units (mph / kph).
        c.cycles += 3;
        c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
        let neg = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0x440000;
        if !neg {
            oc833_slot_lookup(c, 0x0018_9268, 0x0018_9274)?;
            if c.g[S0][0] != 0u64 {
                c.cycles += 18;
                oc833_add_entry(c, 0x20e0, 0, 0x0018_9288, 0x0018_9298)?;
                oc833_add_entry(c, 0x20d0, 0, 0x0018_92a4, 0x0018_92b4)?;
                oc833_copy_options(c, [T0, A1, A2, A3], 0x0018_9324);
                c.cycles += 9;
                oc833_copy_tail(c, T0);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 19);
                c.g[A1][0] &= 0x1;
                call_ptr(c, 0x0039_a7a8, 0x0018_9374)?;
            }
            oc833_label(
                c,
                0x49,
                S2,
                0xfb8,
                0x0018_938c,
                0x0018_9398,
                0xfcc,
                0x0018_93ac,
                0x0018_93b8,
            )?;
            oc833_label(
                c,
                0x49,
                S3,
                0xff8,
                0x0018_93d0,
                0x0018_93dc,
                0xfec,
                0x0018_93f0,
                0x0018_93fc,
            )?;
        }

        // Row 2: widescreen (off / 16:9 / animorphic).
        c.cycles += 3;
        c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x4a));
        let neg = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0x440000;
        if !neg {
            oc833_slot_lookup(c, 0x0018_9424, 0x0018_9430)?;
            if c.g[S0][0] != 0u64 {
                c.cycles += 25;
                oc833_add_entry(c, 0x20c0, 0, 0x0018_9444, 0x0018_9454)?;
                oc833_add_entry(c, 0x20b0, 0, 0x0018_9460, 0x0018_9470)?;
                oc833_add_entry(c, 0x2098, 0, 0x0018_947c, 0x0018_948c)?;
                oc833_copy_options(c, [A3, T0, A1, A2], 0x0018_94fc);
                c.cycles += 9;
                oc833_copy_tail(c, A3);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32) >> 20);
                c.g[A1][0] &= 0x3;
                call_ptr(c, 0x0039_a7a8, 0x0018_954c)?;
            }
            oc833_label(
                c,
                0x4a,
                S2,
                0xfb8,
                0x0018_9564,
                0x0018_9570,
                0xfd0,
                0x0018_9584,
                0x0018_9590,
            )?;
            oc833_label(
                c,
                0x4a,
                S3,
                0xff8,
                0x0018_95a8,
                0x0018_95b4,
                0xff0,
                0x0018_95c8,
                0x0018_95d4,
            )?;
        }

        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x2a0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2f0));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;
    use std::sync::atomic::AtomicUsize;

    /// Gate word and magic of this module's fakes (clear of the fake log at
    /// DATA+0x100000).
    pub(crate) const OC833_CTRL: u32 = DATA + 0x2f_1000;
    pub(crate) const OC833_MAGIC: u32 = 0x4f43_3833;
    /// Slot bytes (`self+0x48..0x4a`) the string-hash fake stores, one word.
    pub(crate) const OC833_SLOTS: u32 = DATA + 0x2f_1010;
    /// Count of `UIScreen_FindObject` calls and the results table (16 words).
    pub(crate) const OC833_FINDN: u32 = DATA + 0x2f_1020;
    pub(crate) const OC833_FIND: u32 = DATA + 0x2f_1040;
    /// Bit i set: the i-th `UIListBox_FindItemByValue` call answers 0.
    pub(crate) const OC833_MISS: u32 = DATA + 0x2f_1030;

    /// The standard fake, then the answers this test steers: the slot bytes
    /// (after each string hash, so the function's own stores are replaced
    /// before its re-reads), the found widgets, the list item lookup.
    fn oc833_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        match addr {
            0x0031_7670 => {
                let s1 = c.g[S1][0] as u32;
                let w = c.bus.read32(OC833_SLOTS);
                if w != 0 {
                    for i in 0..3u32 {
                        let b = (w >> (8 * i)) as u8;
                        c.bus.write8(s1.wrapping_add(0x48 + i), b);
                    }
                }
            }
            0x0039_d860 => {
                let k = c.bus.read32(OC833_FINDN);
                c.bus.write32(OC833_FINDN, k + 1);
                c.g[V0][0] = c.bus.read32(OC833_FIND + 4 * (k % 16)) as u64;
            }
            0x0039_a768 => {
                if c.bus.read32(OC833_MISS) >> (n % 32) & 1 != 0 {
                    c.g[V0][0] = 0;
                }
            }
            _ => {}
        }
    }

    macro_rules! oc833_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC833_CTRL) != OC833_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc833_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc833_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc833_fakes!(
        (
            oc833_fake_317670,
            OC833_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc833_fake_397cc0,
            OC833_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc833_fake_39c870,
            OC833_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc833_fake_14dc80,
            OC833_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc833_fake_14dd58,
            OC833_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc833_fake_39d860,
            OC833_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc833_fake_39b960,
            OC833_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
        (
            oc833_fake_39a768,
            OC833_PREV_39A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039_a768
        ),
        (
            oc833_fake_39a7a8,
            OC833_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039_a7a8
        ),
        (
            oc833_fake_39a568,
            OC833_PREV_39A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039_a568
        ),
        (
            oc833_fake_3a1000,
            OC833_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
    );

    /// `cFEStateOptionsGame_Init`: screen created or not, the object and
    /// widget lookups finding or missing widgets, the row slots as the
    /// function stores them or replaced (so the language row and every
    /// negative-slot skip run), list-item lookup hits and misses, and callee
    /// unwinds at random calls. The options record holds random data.
    #[test]
    fn fe_state_options_game_init_matches() {
        oc833_install_fakes();
        check(
            ee::s02::f_00188f28__generated,
            oc833_fe_state_options_game_init,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x8331, 60);
                c.bus.write32(OC833_CTRL, OC833_MAGIC);
                let slots = match seed % 8 {
                    0..=3 => 0,
                    4 => 0x0001_0200,
                    5 => 0x00ff_ff01,
                    6 => 0x0002_0100,
                    _ => 0x00ff_0201,
                };
                c.bus.write32(OC833_SLOTS, slots);
                c.bus.write32(OC833_MISS, r.next_u32());
                let obj = fakes::ptr(&mut r);
                let vt = fakes::vtable(c, &mut r, &[0x48, 0x40]);
                c.bus.write32(obj + 8, vt);
                c.bus.write32(OC833_FIND, obj);
                for k in 1..16u32 {
                    let v = if (seed >> (k % 5 + 3)) & 1 == 1 && k % 3 == 0 {
                        0
                    } else {
                        fakes::ptr(&mut r)
                    };
                    c.bus.write32(OC833_FIND + 4 * k, v);
                }
                for i in 0..(0x290 / 8) {
                    c.bus.write64(
                        OC833_OPTIONS + 8 * i,
                        ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
                    );
                }
                let this = DATA + 0x30_0000 + (r.next_u32() & 0xf_fff0);
                c.g[A0][0] = sx(this);
                c.g[RA][0] = sx(0x0020_0000 + 0x10 * seed);
                c.pc = 0x0018_8f28;
            },
        );
    }
}
