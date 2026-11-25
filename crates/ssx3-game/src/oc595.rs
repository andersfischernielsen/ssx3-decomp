//! Staging module oc595 (wave l586a.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc595`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0034_eeb8,
    floating_alloc_bound_box,
    "Floating_AllocBoundBox",
)];

/// `void Floating_AllocBoundBox(cFloatingNode* self)` (0x34eeb8): allocates the
/// 0x30-byte bound box (`EA_MemAllocNamed`, name 0x48e890, flags 0x20000000)
/// into `self + 0x78`, then fills it with `WorldObj_GetBoundBox(self, box,
/// box + 0x20)`.
pub fn floating_alloc_bound_box(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x1770));
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x30;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0034_eee0)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        let box_ptr = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x78).sw(c, box_ptr);
        c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
        Callee {
            addr: 0x0035_0570,
            f: ee::s11::f_00350570,
        }
        .call(c, 0x0034_eef4)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// The dlmalloc arena the bound-box allocation falls back to (the gp+0xdd8
    /// pool switch stays 0), initialised as `rbc_test_allocator` does.
    fn floating_test_heap(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = 0x004f_f250;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
    }

    #[test]
    fn floating_alloc_bound_box_matches() {
        check(
            ee::s11::f_0034eeb8__generated,
            floating_alloc_bound_box,
            4,
            &|c, seed| {
                let mut r = Rng(0x0034_eeb8 ^ seed as u64);
                scramble(c, &mut r);
                floating_test_heap(c);
                c.g[A0][0] = (DATA + 0x1000 + 0x100 * (seed % 4)) as u64;
                c.g[RA][0] = crate::difftest::RA as u64;
                c.pc = 0x0034_eeb8;
            },
        );
    }
}
