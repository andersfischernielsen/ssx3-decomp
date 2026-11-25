//! Staging module oc581 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc581`.

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
    0x00327f18,
    rider_sphere_tree_collide_sphere_tree,
    "RiderSphereTree_CollideSphereTree",
)];

/// `RiderSphereTree_CollideSphereTree` (0x00327f18): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn rider_sphere_tree_collide_sphere_tree(c: &mut Cpu) {
    if c.pc != 0x00327f18 {
        return ee::s10::f_00327f18__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00327f18;
        loop {
            match pc {
                0x00327f18 => {
                    c.cycles += 17;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S3][0] = c.g[A1][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S5][0] = c.g[A3][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x98));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S6][0] = c.g[A2][0];
                    if t {
                        pc = 0x00327f6c;
                        continue;
                    }
                    pc = 0x00327f5c;
                    continue;
                }
                0x00327f5c => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00327f64);
                    call_ptr(c, 0x0032df28, 0x00327f64)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x98));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                    pc = 0x00327f6c;
                    continue;
                }
                0x00327f6c => {
                    c.cycles += 12;
                    c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x80));
                    c.lwc1(12, (c.g[S3][0] as u32).wrapping_add(0x20));
                    c.g[S7][0] = c.g[A2][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = 0u64;
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00327f94);
                    c.g[T2][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0032cdb0,
                        f: ee::s10::SphereTree_CollideSphereNode,
                    }
                    .call(c, 0x00327f94)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x00328004;
                        continue;
                    }
                    pc = 0x00327f9c;
                    continue;
                }
                0x00327f9c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        pc = 0x00327fb0;
                        continue;
                    }
                    pc = 0x00327fa8;
                    continue;
                }
                0x00327fa8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        pc = 0x00328004;
                        continue;
                    }
                    pc = 0x00327fb0;
                    continue;
                }
                0x00327fb0 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x00328000;
                        continue;
                    }
                    pc = 0x00327fb8;
                    continue;
                }
                0x00327fb8 => {
                    c.cycles += 2;
                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x30));
                    pc = 0x00327fc0;
                    continue;
                }
                0x00327fc0 => {
                    c.cycles += 11;
                    c.lwc1(12, (c.g[S1][0] as u32));
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S7][0];
                    c.g[A3][0] = 0u64;
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00327fe4);
                    c.g[T2][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0032cdb0,
                        f: ee::s10::SphereTree_CollideSphereNode,
                    }
                    .call(c, 0x00327fe4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x00327fa8) {
                            return Ok(());
                        }
                        pc = 0x00327fa8;
                        continue;
                    }
                    pc = 0x00327fec;
                    continue;
                }
                0x00327fec => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x2c));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00327fc0) {
                            return Ok(());
                        }
                        pc = 0x00327fc0;
                        continue;
                    }
                    pc = 0x00328000;
                    continue;
                }
                0x00328000 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x00328004;
                    continue;
                }
                0x00328004 => {
                    c.cycles += 11;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00327f18, 0x00328030);
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
    fn rider_sphere_tree_collide_sphere_tree_matches() {
        check(
            ee::s10::f_00327f18__generated,
            rider_sphere_tree_collide_sphere_tree,
            200,
            &|c, seed| {
                machine(c, seed, 0x7f18, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00327f18;
            },
        );
    }
}
