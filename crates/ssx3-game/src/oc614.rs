//! Staging module oc614 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc614`.

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
    &[(0x0035f218, tex_flip_modifier_init, "TexFlipModifier_Init")];

/// `TexFlipModifier_Init` (0x0035f218): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn tex_flip_modifier_init(c: &mut Cpu) {
    if c.pc != 0x0035f218 {
        return ee::s12::f_0035f218__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0035f218;
        loop {
            match pc {
                0x0035f218 => {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.g[A2][0] = 0u64;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x94));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                    if t {
                        pc = 0x0035f284;
                        continue;
                    }
                    pc = 0x0035f244;
                    continue;
                }
                0x0035f244 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x94));
                    if t {
                        pc = 0x0035f26c;
                        continue;
                    }
                    pc = 0x0035f24c;
                    continue;
                }
                0x0035f24c => {
                    c.cycles += 1;
                    pc = 0x0035f250;
                    continue;
                }
                0x0035f250 => {
                    c.cycles += 4;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[A2][0] as u32) << 2);
                    if t {
                        pc = 0x0035f284;
                        continue;
                    }
                    pc = 0x0035f260;
                    continue;
                }
                0x0035f260 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    pc = 0x0035f26c;
                    continue;
                }
                0x0035f26c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                        if back_edge_unwinds(c, 0x0035f250) {
                            return Ok(());
                        }
                        pc = 0x0035f250;
                        continue;
                    }
                    pc = 0x0035f27c;
                    continue;
                }
                0x0035f27c => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                    pc = 0x0035f284;
                    continue;
                }
                0x0035f284 => {
                    c.cycles += 11;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[RA][0] = sx(0x0035f290);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0035f378,
                        f: ee::s12::TexFlipModifier_SetFrame,
                    }
                    .call(c, 0x0035f290)?;
                    c.g[A2][0] = 0xc4;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                    c.g[RA][0] = sx(0x0035f2a0);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x003e6448, 0x0035f2a0)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[A2][0] = 0u64;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                    if t {
                        pc = 0x0035f368;
                        continue;
                    }
                    pc = 0x0035f2b0;
                    continue;
                }
                0x0035f2b0 => {
                    c.cycles += 4;
                    c.g[T3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x28));
                    c.g[T4][0] = 0x14;
                    c.g[T5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    pc = 0x0035f2c0;
                    continue;
                }
                0x0035f2c0 => {
                    c.cycles += 7;
                    c.g[A0][0] = sx((c.g[A2][0] as u32) << 2);
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                    let t = c.g[V0][0] == 0u64;
                    c.mult(0, 3, 6, 12);
                    if t {
                        pc = 0x0035f348;
                        continue;
                    }
                    pc = 0x0035f2dc;
                    continue;
                }
                0x0035f2dc => {
                    c.cycles += 11;
                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[T2][0] = 0xffffffffffffffff;
                    c.g[A1][0] = sx((c.g[T5][0] as u32).wrapping_add(0x2));
                    c.g[A3][0] = 0x2;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[A0][0] as u32), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xc));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    pc = 0x0035f308;
                    continue;
                }
                0x0035f308 => {
                    c.cycles += 7;
                    c.sh((c.g[A1][0] as u32), c.g[T2][0]);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[A3][0] as i64) >= 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                    if t {
                        if back_edge_unwinds(c, 0x0035f308) {
                            return Ok(());
                        }
                        pc = 0x0035f308;
                        continue;
                    }
                    pc = 0x0035f324;
                    continue;
                }
                0x0035f324 => {
                    c.cycles += 9;
                    c.mult(0, 2, 6, 12);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x4));
                    let t = true;
                    c.sh((c.g[A1][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                    if t {
                        pc = 0x0035f354;
                        continue;
                    }
                    pc = 0x0035f348;
                    continue;
                }
                0x0035f348 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[V0][0] as u32), c.g[T0][0]);
                    pc = 0x0035f354;
                    continue;
                }
                0x0035f354 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                    c.g[A2][0] = c.g[T1][0];
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        if back_edge_unwinds(c, 0x0035f2c0) {
                            return Ok(());
                        }
                        pc = 0x0035f2c0;
                        continue;
                    }
                    pc = 0x0035f368;
                    continue;
                }
                0x0035f368 => {
                    c.cycles += 4;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0035f218, 0x0035f378);
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
    fn tex_flip_modifier_init_matches() {
        check(
            ee::s12::f_0035f218__generated,
            tex_flip_modifier_init,
            200,
            &|c, seed| {
                machine(c, seed, 0xf218, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x0035f218;
            },
        );
    }
}
