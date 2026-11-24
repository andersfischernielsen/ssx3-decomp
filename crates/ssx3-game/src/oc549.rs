//! Staging module oc549 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc549`.

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
    &[(0x00276198, script_list_take_from, "ScriptList_TakeFrom")];

/// `ScriptList_TakeFrom` (0x00276198): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn script_list_take_from(c: &mut Cpu) {
    if c.pc != 0x00276198 {
        return ee::s07::f_00276198__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00276198;
        loop {
            match pc {
                0x00276198 => {
                    c.cycles += 15;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.g[RA][0] = sx(0x002761c0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    Callee {
                        addr: 0x00283d28,
                        f: ee::s07::ScriptCmdQueue_Count,
                    }
                    .call(c, 0x002761c0)?;
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xa4));
                    c.g[V0][0] = 0x3;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                    if t {
                        pc = 0x002761dc;
                        continue;
                    }
                    pc = 0x002761d4;
                    continue;
                }
                0x002761d4 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00276250;
                        continue;
                    }
                    pc = 0x002761dc;
                    continue;
                }
                0x002761dc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002761e4);
                    c.g[S2][0] = c.g[A0][0];
                    Callee {
                        addr: 0x00283d28,
                        f: ee::s07::ScriptCmdQueue_Count,
                    }
                    .call(c, 0x002761e4)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S2][0];
                    if t {
                        pc = 0x0027624c;
                        continue;
                    }
                    pc = 0x002761ec;
                    continue;
                }
                0x002761ec => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x00276204;
                        continue;
                    }
                    pc = 0x002761f4;
                    continue;
                }
                0x002761f4 => {
                    c.cycles += 1;
                    pc = 0x002761f8;
                    continue;
                }
                0x002761f8 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00276200);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x00283c80,
                        f: ee::s07::ScriptCmdQueue_Push,
                    }
                    .call(c, 0x00276200)?;
                    c.g[A0][0] = c.g[S2][0];
                    pc = 0x00276204;
                    continue;
                }
                0x00276204 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0027620c);
                    c.g[A1][0] = c.g[SP][0];
                    Callee {
                        addr: 0x00283dc0,
                        f: ee::s07::ScriptCmdQueue_Pop,
                    }
                    .call(c, 0x0027620c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    if t {
                        if back_edge_unwinds(c, 0x002761f8) {
                            return Ok(());
                        }
                        pc = 0x002761f8;
                        continue;
                    }
                    pc = 0x00276214;
                    continue;
                }
                0x00276214 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00276220);
                    c.g[A1][0] = 0x1;
                    Callee {
                        addr: 0x00276868,
                        f: ee::s07::ScriptList_Stop,
                    }
                    .call(c, 0x00276220)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa4));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00276250;
                        continue;
                    }
                    pc = 0x0027622c;
                    continue;
                }
                0x0027622c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00276250;
                        continue;
                    }
                    pc = 0x00276238;
                    continue;
                }
                0x00276238 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00276248);
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x00277450,
                        f: ee::s07::ScriptList_PrepareEntry,
                    }
                    .call(c, 0x00276248)?;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                    pc = 0x0027624c;
                    continue;
                }
                0x0027624c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    pc = 0x00276250;
                    continue;
                }
                0x00276250 => {
                    c.cycles += 7;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00276198, 0x0027626c);
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
    fn script_list_take_from_matches() {
        check(
            ee::s07::f_00276198__generated,
            script_list_take_from,
            200,
            &|c, seed| {
                machine(c, seed, 0x6198, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00276198;
            },
        );
    }
}
