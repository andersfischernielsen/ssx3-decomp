//! Staging module oc611 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc611`.

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
    0x0035a780,
    multi_spline_modifier_remove_node_instances,
    "MultiSplineModifier_RemoveNodeInstances",
)];

/// `MultiSplineModifier_RemoveNodeInstances` (0x0035a780): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn multi_spline_modifier_remove_node_instances(c: &mut Cpu) {
    if c.pc != 0x0035a780 {
        return ee::s11::f_0035a780__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0035a780;
        loop {
            match pc {
                0x0035a780 => {
                    c.cycles += 13;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = 0x1;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S4][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x0035a8f4;
                        continue;
                    }
                    pc = 0x0035a7b4;
                    continue;
                }
                0x0035a7b4 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x44));
                    pc = 0x0035a7b8;
                    continue;
                }
                0x0035a7b8 => {
                    c.cycles += 37;
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[RA][0] = sx(0x0035a7c8);
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x002d1be0, 0x0035a7c8)?;
                    c.g[A1][0] = 0x60;
                    c.g[S0][0] = c.g[V0][0];
                    c.mult(0, 5, 18, 5);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x3c));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                    c.g[RA][0] = sx(0x0035a7f4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x30));
                    call_ptr(c, 0x00328f28, 0x0035a7f4)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A0][0] = ((c.g[S4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[T0][0] = ((c.g[S4][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[A1][0] = 0u64;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[A3][0] = c.g[SP][0];
                    if c.g[T0][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x0035a83c);
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x003284b8, 0x0035a83c)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035a8a0;
                        continue;
                    }
                    pc = 0x0035a84c;
                    continue;
                }
                0x0035a84c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035a8a0;
                        continue;
                    }
                    pc = 0x0035a858;
                    continue;
                }
                0x0035a858 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035a8a0;
                        continue;
                    }
                    pc = 0x0035a864;
                    continue;
                }
                0x0035a864 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035a8a0;
                        continue;
                    }
                    pc = 0x0035a870;
                    continue;
                }
                0x0035a870 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0035a8a0;
                        continue;
                    }
                    pc = 0x0035a87c;
                    continue;
                }
                0x0035a87c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        pc = 0x0035a8a0;
                        continue;
                    }
                    pc = 0x0035a888;
                    continue;
                }
                0x0035a888 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0035a8cc;
                        continue;
                    }
                    pc = 0x0035a894;
                    continue;
                }
                0x0035a894 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));

                    if c.g[V0][0] == 0u64 {
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                        pc = 0x0035a8a8;
                        continue;
                    }
                    pc = 0x0035a8a0;
                    continue;
                }
                0x0035a8a0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0035a8cc;
                        continue;
                    }
                    pc = 0x0035a8a8;
                    continue;
                }
                0x0035a8a8 => {
                    c.cycles += 2;
                    c.g[A0][0] = 0u64;
                    pc = 0x0035a8b0;
                    continue;
                }
                0x0035a8b0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0035a8a0) {
                            return Ok(());
                        }
                        pc = 0x0035a8a0;
                        continue;
                    }
                    pc = 0x0035a8bc;
                    continue;
                }
                0x0035a8bc => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0035a8b0) {
                            return Ok(());
                        }
                        pc = 0x0035a8b0;
                        continue;
                    }
                    pc = 0x0035a8c8;
                    continue;
                }
                0x0035a8c8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x0035a8cc;
                    continue;
                }
                0x0035a8cc => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                        pc = 0x0035a8e4;
                        continue;
                    }
                    pc = 0x0035a8d4;
                    continue;
                }
                0x0035a8d4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0035a8dc);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317e50, 0x0035a8dc)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                    pc = 0x0035a8e4;
                    continue;
                }
                0x0035a8e4 => {
                    c.cycles += 4;
                    c.g[S2][0] = c.g[S1][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x44));
                        if back_edge_unwinds(c, 0x0035a7b8) {
                            return Ok(());
                        }
                        pc = 0x0035a7b8;
                        continue;
                    }
                    pc = 0x0035a8f4;
                    continue;
                }
                0x0035a8f4 => {
                    c.cycles += 8;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0035a780, 0x0035a914);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    #[test]
    fn multi_spline_modifier_remove_node_instances_matches() {
        check(
            ee::s11::f_0035a780__generated,
            multi_spline_modifier_remove_node_instances,
            200,
            &|c, seed| {
                machine(c, seed, 0xa780, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0035a780;
            },
        );
    }
}
