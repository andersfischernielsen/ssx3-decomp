//! Staging module oc579 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc579`.

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
    0x0032df28,
    sphere_tree_cache_get_masks,
    "SphereTreeCache_GetMasks",
)];

/// `SphereTreeCache_GetMasks` (0x0032df28): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn sphere_tree_cache_get_masks(c: &mut Cpu) {
    if c.pc != 0x0032df28 {
        return ee::s11::f_0032df28__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0032df28;
        loop {
            match pc {
                0x0032df28 => {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef8));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = c.lw((c.g[A0][0] as u32));
                    if t {
                        pc = 0x0032e008;
                        continue;
                    }
                    pc = 0x0032df50;
                    continue;
                }
                0x0032df50 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xedc));
                    c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[V0][0] != c.g[S0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee0));
                    if t {
                        pc = 0x0032df78;
                        continue;
                    }
                    pc = 0x0032df6c;
                    continue;
                }
                0x0032df6c => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xed8));
                    let t = true;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    if t {
                        pc = 0x0032e0e8;
                        continue;
                    }
                    pc = 0x0032df78;
                    continue;
                }
                0x0032df78 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V1][0] = 0u64;
                    if t {
                        pc = 0x0032dfa4;
                        continue;
                    }
                    pc = 0x0032df80;
                    continue;
                }
                0x0032df80 => {
                    c.cycles += 2;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[A2][0];
                    pc = 0x0032df88;
                    continue;
                }
                0x0032df88 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                    let t = c.g[V0][0] == c.g[S0][0];
                    c.g[A2][0] = sx((c.g[V1][0] as u32) << 2);
                    if t {
                        pc = 0x0032e030;
                        continue;
                    }
                    pc = 0x0032df94;
                    continue;
                }
                0x0032df94 => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0032df88) {
                            return Ok(());
                        }
                        pc = 0x0032df88;
                        continue;
                    }
                    pc = 0x0032dfa4;
                    continue;
                }
                0x0032dfa4 => {
                    c.cycles += 21;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee4));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xed8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[RA][0] = sx(0x0032dfc0);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x0032b620, 0x0032dfc0)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee4));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xedc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[S0][0]);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee4));
                    c.sw((c.g[S1][0] as u32), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee4));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.div(0, 2, 3);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0xee4), c.g[V0][0]);

                    if c.g[V1][0] == 0u64 {
                        pc = 0x0032dff8;
                        continue;
                    }
                    pc = 0x0032dff8;
                    continue;
                }
                0x0032dff8 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xed8));
                    c.g[A0][0] = c.hi[0];
                    let t = true;
                    c.sw((c.g[GP][0] as u32).wrapping_add(0xee4), c.g[A0][0]);
                    if t {
                        pc = 0x0032e0dc;
                        continue;
                    }
                    pc = 0x0032e008;
                    continue;
                }
                0x0032e008 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xeec));
                    c.g[A1][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    let t = c.g[V0][0] != c.g[S0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef0));
                    if t {
                        pc = 0x0032e050;
                        continue;
                    }
                    pc = 0x0032e024;
                    continue;
                }
                0x0032e024 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee8));
                    let t = true;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    if t {
                        pc = 0x0032e0e8;
                        continue;
                    }
                    pc = 0x0032e030;
                    continue;
                }
                0x0032e030 => {
                    c.cycles += 4;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xed8));
                    let t = true;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    if t {
                        pc = 0x0032e0e8;
                        continue;
                    }
                    pc = 0x0032e040;
                    continue;
                }
                0x0032e040 => {
                    c.cycles += 4;
                    c.sw((c.g[S1][0] as u32), c.g[V1][0]);
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee8));
                    let t = true;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    if t {
                        pc = 0x0032e0e8;
                        continue;
                    }
                    pc = 0x0032e050;
                    continue;
                }
                0x0032e050 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V1][0] = 0u64;
                    if t {
                        pc = 0x0032e07c;
                        continue;
                    }
                    pc = 0x0032e058;
                    continue;
                }
                0x0032e058 => {
                    c.cycles += 2;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[A2][0];
                    pc = 0x0032e060;
                    continue;
                }
                0x0032e060 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                    let t = c.g[V0][0] == c.g[S0][0];
                    c.g[A2][0] = sx((c.g[V1][0] as u32) << 2);
                    if t {
                        if back_edge_unwinds(c, 0x0032e040) {
                            return Ok(());
                        }
                        pc = 0x0032e040;
                        continue;
                    }
                    pc = 0x0032e06c;
                    continue;
                }
                0x0032e06c => {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0032e060) {
                            return Ok(());
                        }
                        pc = 0x0032e060;
                        continue;
                    }
                    pc = 0x0032e07c;
                    continue;
                }
                0x0032e07c => {
                    c.cycles += 21;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef4));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[RA][0] = sx(0x0032e098);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                    call_ptr(c, 0x0032b620, 0x0032e098)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef4));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xeec));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.sw((c.g[V1][0] as u32), c.g[S0][0]);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef4));
                    c.sw((c.g[S1][0] as u32), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef4));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xef0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.div(0, 2, 3);
                    c.sw((c.g[GP][0] as u32).wrapping_add(0xef4), c.g[V0][0]);

                    if c.g[V1][0] == 0u64 {
                        pc = 0x0032e0d0;
                        continue;
                    }
                    pc = 0x0032e0d0;
                    continue;
                }
                0x0032e0d0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xee8));
                    c.g[A0][0] = c.hi[0];
                    c.sw((c.g[GP][0] as u32).wrapping_add(0xef4), c.g[A0][0]);
                    pc = 0x0032e0dc;
                    continue;
                }
                0x0032e0dc => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    pc = 0x0032e0e8;
                    continue;
                }
                0x0032e0e8 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0032df28, 0x0032e100);
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
    fn sphere_tree_cache_get_masks_matches() {
        check(
            ee::s11::f_0032df28__generated,
            sphere_tree_cache_get_masks,
            200,
            &|c, seed| {
                machine(c, seed, 0xdf28, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0032df28;
            },
        );
    }
}
