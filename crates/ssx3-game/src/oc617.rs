//! Staging module oc617 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc617`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x003a7818,
    section_allocator_free_all,
    "SectionAllocator_FreeAll",
)];

/// `SectionAllocator_FreeAll` (0x003a7818): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn section_allocator_free_all(c: &mut Cpu) {
    if c.pc != 0x003a7818 {
        return ee::s13::f_003a7818__generated(c);
    }
    guest(c, |c| {
        'b_3a7864: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            if t {
                break 'b_3a7864;
            } else {
                'l_3a7838: loop {
                    c.cycles += 10;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[RA][0] = sx(0x003a784c);
                    c.g[A1][0] = c.g[V0][0];
                    Callee {
                        addr: 0x003a7790,
                        f: ee::s13::WorldBlockPool_FreeBlock,
                    }
                    .call(c, 0x003a784c)?;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[S0][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[S0][0] != 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003a7838);
                        }
                        continue 'l_3a7838;
                    } else {
                        c.cycles += 1;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_3a7864;
                    }
                }
            }
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    #[test]
    fn section_allocator_free_all_matches() {
        check(
            ee::s13::f_003a7818__generated,
            section_allocator_free_all,
            200,
            &|c, seed| {
                machine(c, seed, 0x7818, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x003a7818;
            },
        );
    }
}
