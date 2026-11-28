//! Staging module oc1489 (wave wave275): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1489`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0017_fe64, fe_tail_call_320c48, "lbl_0017FE64")];

/// Tail of a front-end handler (0x17fe64; entries 0x17fe64 and 0x17fe68):
/// call `0x320c48(a0 = *(a2 + 0x18), a1 = 0x79)` and return through the
/// frame's saved `ra` (`sp += 0x10`). Resume entries other than 0x17fe64 and
/// 0x17fe68 are not reachable (the generated twin jumps out); both are kept
/// natively.
pub fn fe_tail_call_320c48(c: &mut Cpu) {
    let entry = c.pc;
    guest(c, |c| {
        match entry {
            0x0017_fe64 => {
                c.cycles += 1;
                c.g[A1][0] = 0x79;
            }
            0x0017_fe68 => {}
            _ => {
                ee::s02::f_0017fe64__entries(c);
                return Err(crate::call::Unwound);
            }
        }
        c.cycles += 5;
        let a2 = c.g[A2][0] as u32;
        let a0 = c.lw(a2.wrapping_add(0x18));
        c.g[A0][0] = a0;
        call_ptr(c, 0x0032_0c48, 0x0017_fe70)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine};
    use crate::difftest::DATA;

    /// Both entries (0x17fe64 and 0x17fe68), random `a2` struct, with the
    /// callee faked (sometimes unwinding).
    #[test]
    fn fe_tail_call_320c48_matches() {
        covered(
            ee::s02::f_0017fe64__generated,
            fe_tail_call_320c48,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x17_fe64, 1);
                c.pc = if seed % 2 == 0 { 0x0017_fe64 } else { 0x0017_fe68 };
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.g[SP][0] = (DATA + 0x1000) as u64;
                c.bus.write32(DATA + 0x2018, 0x1234_5678 ^ seed);
                c.bus.write64(DATA + 0x1000, 0x0030_0000 + 4 * seed as u64);
            },
        );
    }
}
