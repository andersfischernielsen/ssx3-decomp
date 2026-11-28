//! Staging module oc1478 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1478`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_38c8, lbl_003a38c8, "lbl_003A38C8"),
    (0x003a_38dc, lbl_003a38dc, "lbl_003A38DC"),
    (0x003a_3904, lbl_003a3904, "lbl_003A3904"),
    (0x003a_39e0, lbl_003a39e0, "lbl_003A39E0"),
];

/// `lbl_003A38C8`: jump-table case of the `UIVector` copy loop. Stores `f1`
/// at `*(t0 + 0x7c) + (a3 << 4)` and continues at the loop tail 0x3a39e0.
/// (The generated fall-through to 0x3a38dc is guarded by a constant `true`.)
pub fn lbl_003a38c8(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003a_38c8 => {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 4);
                pc = 0x003a_38d0;
                continue;
            }
            0x003a_38d0 => {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.swc1(1, c.g[V0][0] as u32);
                c.pc = 0x003a_39e0;
                ee::s13::f_003a39e0(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x003a_38c8, 0x003a_38dc);
                return;
            }
        }
    }
}

/// `lbl_003A38DC`: same as 0x3a38c8 with the store at offset 4.
pub fn lbl_003a38dc(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003a_38dc => {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 4);
                pc = 0x003a_38e4;
                continue;
            }
            0x003a_38e4 => {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.pc = 0x003a_39e0;
                ee::s13::f_003a39e0(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x003a_38dc, 0x003a_38f0);
                return;
            }
        }
    }
}

/// `lbl_003A3904`: same with the store at offset 0xc. The code after the
/// unconditional tail jump (0x3a3918, 0x3a3920: a table jump through
/// 0x494170 + a0 * 4) is only reachable as a resume entry.
pub fn lbl_003a3904(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003a_3904 => {
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 4);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.pc = 0x003a_39e0;
                ee::s13::f_003a39e0(c);
                return;
            }
            0x003a_3918 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x0049_0000;
                if t {
                    c.pc = 0x003a_39e0;
                    ee::s13::f_003a39e0(c);
                    return;
                }
                pc = 0x003a_3920;
                continue;
            }
            0x003a_3920 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4170));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                pc = c.g[A0][0] as u32;
                continue;
            }
            _ => {
                c.jump_out(pc, 0x003a_3904, 0x003a_3938);
                return;
            }
        }
    }
}

/// `lbl_003A39E0`: loop tail of the `UIVector` copy: `t2 += 1; a2 += 4`;
/// while `t2 < a1` continue at the loop head 0x3a3820, else return.
pub fn lbl_003a39e0(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003a_39e0 => {
                c.cycles += 4;
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[T2][0] < c.g[A1][0]) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                if t {
                    c.pc = 0x003a_3820;
                    call_indirect(c, 0x003a_3820);
                    return;
                }
                pc = 0x003a_39f0;
                continue;
            }
            0x003a_39f0 => {
                c.cycles += 2;
                c.pc = c.g[RA][0] as u32;
                return;
            }
            _ => {
                c.jump_out(pc, 0x003a_39e0, 0x003a_39f8);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine};
    use crate::difftest::{fill, DATA, RA as TEST_RA};

    /// Shared fixture of the four jump-table fragments: `t0` points at a
    /// struct whose word at +0x7c is the destination buffer, `a3` the
    /// element index; the loop tail gets `t2`/`a1`/`a2` so that three of
    /// four seeds loop again (a faked call to 0x3a3820) and one returns.
    fn oc1478_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let mut r = machine(c, seed, salt, 1);
        let base = DATA + 0x2000;
        c.bus.write32(base + 0x7c, DATA + 0x3000);
        fill(c, &mut r, DATA + 0x3000, 0x40);
        c.g[T0][0] = sx(base);
        c.g[A3][0] = (r.next_u32() % 8) as u64;
        c.g[T2][0] = (r.next_u32() % 100) as u64;
        c.g[A1][0] = if seed % 4 == 0 {
            c.g[T2][0]
        } else {
            c.g[T2][0] + 2
        };
        c.g[A2][0] = sx(DATA + 0x4000);
        c.g[RA][0] = sx(TEST_RA);
        c.pc = entry;
    }

    #[test]
    fn lbl_003a38c8_matches() {
        covered(
            ssx3_ee::s13::f_003a38c8__generated,
            lbl_003a38c8,
            64,
            &|c, seed| oc1478_setup(c, seed, 0x3a38c8, 0x003a_38c8),
        );
    }

    #[test]
    fn lbl_003a38dc_matches() {
        covered(
            ssx3_ee::s13::f_003a38dc__generated,
            lbl_003a38dc,
            64,
            &|c, seed| oc1478_setup(c, seed, 0x3a38dc, 0x003a_38dc),
        );
    }

    /// Also the resume entries 0x3a3918 (v0 zero or not) and 0x3a3920 (table
    /// jump: entry 0 holds the function's own start, the others a stray
    /// address that goes to `jump_out`).
    #[test]
    fn lbl_003a3904_matches() {
        covered(
            ssx3_ee::s13::f_003a3904__generated,
            lbl_003a3904,
            96,
            &|c, seed| {
                let entry = match seed % 6 {
                    4 => 0x003a_3918,
                    5 => 0x003a_3920,
                    _ => 0x003a_3904,
                };
                oc1478_setup(c, seed, 0x3a3904, entry);
                if entry == 0x003a_3918 {
                    c.g[V0][0] = if seed % 12 == 4 { 0 } else { 1 };
                }
                if entry == 0x003a_3920 {
                    c.g[V0][0] = 1;
                    c.g[A0][0] = (seed % 2) as u64;
                    c.bus.write32(0x0049_4170, 0x003a_3904);
                    c.bus.write32(0x0049_4174, 0x0012_0000);
                }
            },
        );
    }

    #[test]
    fn lbl_003a39e0_matches() {
        covered(
            ssx3_ee::s13::f_003a39e0__generated,
            lbl_003a39e0,
            64,
            &|c, seed| {
                oc1478_setup(c, seed, 0x3a39e0, 0x003a_39e0);
                if seed % 8 == 1 {
                    c.pc = 0x003a_39f0;
                }
            },
        );
    }
}
