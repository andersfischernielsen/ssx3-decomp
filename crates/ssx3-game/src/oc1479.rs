//! Staging module oc1479 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1479`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x003a_38f0, lbl_003a38f0, "lbl_003A38F0")];

/// `lbl_003A38F0`: jump-table case of the `UIVector` copy loop. Stores `f1`
/// at `*(t0 + 0x7c) + (a3 << 4) + 8` and continues at the loop tail 0x3a39e0.
/// (The generated fall-through to 0x3a3904 is guarded by a constant `true`.)
pub fn lbl_003a38f0(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003a_38f0 => {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 4);
                pc = 0x003a_38f8;
                continue;
            }
            0x003a_38f8 => {
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.pc = 0x003a_39e0;
                call_indirect(c, 0x003a_39e0);
                return;
            }
            _ => {
                c.jump_out(pc, 0x003a_38f0, 0x003a_3904);
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

    /// `t0` points at a struct whose word at +0x7c is the destination buffer,
    /// `a3` the element index; `t2`/`a1`/`a2` feed the loop tail so that most
    /// seeds loop again (faked call to 0x3a3820) and some return. Seeds with
    /// `seed % 8 == 1` start at the resume entry 0x3a38f8.
    #[test]
    fn lbl_003a38f0_matches() {
        covered(
            ssx3_ee::s13::f_003a38f0__generated,
            lbl_003a38f0,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3a38f0, 1);
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
                c.pc = if seed % 8 == 1 { 0x003a_38f8 } else { 0x003a_38f0 };
                if seed % 8 == 1 {
                    c.g[V1][0] = sx(DATA + 0x3000);
                    c.g[V0][0] = (r.next_u32() % 8 * 16) as u64;
                }
            },
        );
    }
}
