//! Staging module oc612 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc612`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x00358780, rail_man_disable_rail, "RailMan_DisableRail")];

/// `RailMan_DisableRail` (0x00358780): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn rail_man_disable_rail(c: &mut Cpu) {
    if c.pc != 0x00358780 {
        return ee::s11::f_00358780__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00358780;
        loop {
            match pc {
                0x00358780 => {
                    c.cycles += 20;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[RA][0] = sx(0x003587b0);
                    c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x002d1bd8, 0x003587b0)?;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V0][0] = c.g[S0][0] & 0xff;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));

                    if c.g[V1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                        pc = 0x003587d8;
                        continue;
                    }
                    pc = 0x003587d0;
                    continue;
                }
                0x003587d0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S4][0] = 0u64;
                    if t {
                        pc = 0x003587fc;
                        continue;
                    }
                    pc = 0x003587d8;
                    continue;
                }
                0x003587d8 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) >> 8);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x003587f8;
                        continue;
                    }
                    pc = 0x003587f4;
                    continue;
                }
                0x003587f4 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x003587f8;
                    continue;
                }
                0x003587f8 => {
                    c.cycles += 1;
                    c.g[S4][0] = c.g[V0][0];
                    pc = 0x003587fc;
                    continue;
                }
                0x003587fc => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
                    c.g[S2][0] = 0u64;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x24));
                    if t {
                        pc = 0x00358970;
                        continue;
                    }
                    pc = 0x0035880c;
                    continue;
                }
                0x0035880c => {
                    c.cycles += 3;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 20);
                    c.g[S3][0] = 0xffffffffffffffff;
                    pc = 0x00358818;
                    continue;
                }
                0x00358818 => {
                    c.cycles += 44;
                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x6c));
                    c.swc1(0, (c.g[SP][0] as u32));
                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x70));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x74));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x78));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x7c));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
                    c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x80));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1c));
                    c.g[RA][0] = sx(0x00358854);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x002d1be0, 0x00358854)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[RA][0] = sx(0x00358870);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
                    call_ptr(c, 0x00328f28, 0x00358870)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[A1][0] = 0x2;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A0][0] = ((c.g[S3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[T0][0] = ((c.g[S3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    if c.g[A0][0] != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                    c.g[A2][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    if c.g[T0][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = sx(0x003588b8);
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x003284b8, 0x003588b8)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x003588c8;
                    continue;
                }
                0x003588c8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x003588d4;
                    continue;
                }
                0x003588d4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x003588e0;
                    continue;
                }
                0x003588e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x003588ec;
                    continue;
                }
                0x003588ec => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x003588f8;
                    continue;
                }
                0x003588f8 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x00358904;
                    continue;
                }
                0x00358904 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x00358910;
                    continue;
                }
                0x00358910 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x00358924;
                        continue;
                    }
                    pc = 0x0035891c;
                    continue;
                }
                0x0035891c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0035894c;
                        continue;
                    }
                    pc = 0x00358924;
                    continue;
                }
                0x00358924 => {
                    c.cycles += 3;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    c.g[A0][0] = 0u64;
                    pc = 0x00358930;
                    continue;
                }
                0x00358930 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0035891c) {
                            return Ok(());
                        }
                        pc = 0x0035891c;
                        continue;
                    }
                    pc = 0x0035893c;
                    continue;
                }
                0x0035893c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x00358930) {
                            return Ok(());
                        }
                        pc = 0x00358930;
                        continue;
                    }
                    pc = 0x00358948;
                    continue;
                }
                0x00358948 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x0035894c;
                    continue;
                }
                0x0035894c => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
                        pc = 0x00358964;
                        continue;
                    }
                    pc = 0x00358954;
                    continue;
                }
                0x00358954 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0035895c);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317e50, 0x0035895c)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x20));
                    pc = 0x00358964;
                    continue;
                }
                0x00358964 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                    if t {
                        if back_edge_unwinds(c, 0x00358818) {
                            return Ok(());
                        }
                        pc = 0x00358818;
                        continue;
                    }
                    pc = 0x00358970;
                    continue;
                }
                0x00358970 => {
                    c.cycles += 9;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x90));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00358780, 0x00358994);
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
    fn rail_man_disable_rail_matches() {
        check(
            ee::s11::f_00358780__generated,
            rail_man_disable_rail,
            200,
            &|c, seed| {
                machine(c, seed, 0x8780, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00358780;
            },
        );
    }
}
