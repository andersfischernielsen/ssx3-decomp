//! Staging module oc615 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc615`.

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
    0x003443a8,
    crowd_man3d_group_create_meshes,
    "CrowdMan3d_GroupCreateMeshes",
)];

/// `CrowdMan3d_GroupCreateMeshes` (0x003443a8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn crowd_man3d_group_create_meshes(c: &mut Cpu) {
    if c.pc != 0x003443a8 {
        return ee::s11::f_003443a8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x003443a8;
        loop {
            match pc {
                0x003443a8 => {
                    c.cycles += 22;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[RA][0] = sx(0x003443dc);
                    c.g[S2][0] = c.g[A0][0];
                    call_ptr(c, 0x002d1bd8, 0x003443dc)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[A1][0] & 0xff;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));

                    if c.g[V1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                        pc = 0x00344408;
                        continue;
                    }
                    pc = 0x00344400;
                    continue;
                }
                0x00344400 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[FP][0] = 0u64;
                    if t {
                        pc = 0x0034442c;
                        continue;
                    }
                    pc = 0x00344408;
                    continue;
                }
                0x00344408 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x00344428;
                        continue;
                    }
                    pc = 0x00344424;
                    continue;
                }
                0x00344424 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x00344428;
                    continue;
                }
                0x00344428 => {
                    c.cycles += 1;
                    c.g[FP][0] = c.g[V0][0];
                    pc = 0x0034442c;
                    continue;
                }
                0x0034442c => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x80));
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
                    c.g[A1][0] = 0x490000;
                    c.g[A2][0] = 0x20000000;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1f0), c.g[V0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x18a8));
                    c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                    c.g[A3][0] = 0u64;
                    c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1e8));
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                    c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x1e0));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x00344464);
                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 6);
                    call_ptr(c, 0x00317e30, 0x00344464)?;
                    let t = c.g[S0][0] == 0u64;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x00344470;
                        continue;
                    }
                    pc = 0x0034446c;
                    continue;
                }
                0x0034446c => {
                    c.cycles += 1;
                    c.g[A0][0] = 0xffffffffffffffff;
                    pc = 0x00344470;
                    continue;
                }
                0x00344470 => {
                    c.cycles += 7;
                    c.sw((c.g[S1][0] as u32), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f0));
                    c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x10c), 0u64);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x003444bc;
                        continue;
                    }
                    pc = 0x0034448c;
                    continue;
                }
                0x0034448c => {
                    c.cycles += 1;
                    pc = 0x00344490;
                    continue;
                }
                0x00344490 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                        pc = 0x003444ac;
                        continue;
                    }
                    pc = 0x0034449c;
                    continue;
                }
                0x0034449c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x10c), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    pc = 0x003444ac;
                    continue;
                }
                0x003444ac => {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                    if t {
                        if back_edge_unwinds(c, 0x00344490) {
                            return Ok(());
                        }
                        pc = 0x00344490;
                        continue;
                    }
                    pc = 0x003444bc;
                    continue;
                }
                0x003444bc => {
                    c.cycles += 3;
                    c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10c));
                    let t = c.g[S1][0] == 0u64;
                    c.g[A0][0] = 0xd0;
                    if t {
                        pc = 0x0034453c;
                        continue;
                    }
                    pc = 0x003444c8;
                    continue;
                }
                0x003444c8 => {
                    c.cycles += 13;
                    c.g[A1][0] = 0x490000;
                    c.mult(0, 4, 17, 4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1898));
                    c.g[A2][0] = 0x20000000;
                    c.g[A3][0] = 0u64;
                    c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1ec));
                    c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                    c.g[RA][0] = sx(0x003444ec);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x00317e30, 0x003444ec)?;
                    c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[V0][0] as u32), c.g[S1][0]);
                    let t = c.g[S1][0] == 0u64;
                    c.g[S0][0] = c.g[S4][0];
                    if t {
                        pc = 0x00344534;
                        continue;
                    }
                    pc = 0x003444fc;
                    continue;
                }
                0x003444fc => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x490000;
                    c.g[V1][0] = 0x490000;
                    c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0xaf0));
                    c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(0xad0));
                    c.g[S1][0] = 0xffffffffffffffff;
                    pc = 0x00344510;
                    continue;
                }
                0x00344510 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0034451c);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x84), c.g[S6][0]);
                    Callee {
                        addr: 0x003513d0,
                        f: ee::s11::IntersectInfo_Clear,
                    }
                    .call(c, 0x0034451c)?;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x84), c.g[S5][0]);
                    c.g[RA][0] = sx(0x0034452c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003612c0, 0x0034452c)?;
                    let t = c.g[S3][0] != c.g[S1][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd0));
                    if t {
                        if back_edge_unwinds(c, 0x00344510) {
                            return Ok(());
                        }
                        pc = 0x00344510;
                        continue;
                    }
                    pc = 0x00344534;
                    continue;
                }
                0x00344534 => {
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S7][0] as u32), c.g[S4][0]);
                    if t {
                        pc = 0x00344540;
                        continue;
                    }
                    pc = 0x0034453c;
                    continue;
                }
                0x0034453c => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x1ec), 0u64);
                    pc = 0x00344540;
                    continue;
                }
                0x00344540 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f0));
                    c.g[S0][0] = 0u64;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.g[S1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x0034458c;
                        continue;
                    }
                    pc = 0x00344554;
                    continue;
                }
                0x00344554 => {
                    c.cycles += 1;
                    c.g[S3][0] = 0u64;
                    pc = 0x00344558;
                    continue;
                }
                0x00344558 => {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));

                    if c.g[A1][0] == 0u64 {
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f0));
                        pc = 0x00344578;
                        continue;
                    }
                    pc = 0x00344564;
                    continue;
                }
                0x00344564 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1ec));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                    c.g[RA][0] = sx(0x00344574);
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0xd0));
                    Callee {
                        addr: 0x00351398,
                        f: ee::s11::IntersectInfo_SetSource,
                    }
                    .call(c, 0x00344574)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1f0));
                    pc = 0x00344578;
                    continue;
                }
                0x00344578 => {
                    c.cycles += 5;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                    if t {
                        if back_edge_unwinds(c, 0x00344558) {
                            return Ok(());
                        }
                        pc = 0x00344558;
                        continue;
                    }
                    pc = 0x0034458c;
                    continue;
                }
                0x0034458c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10c));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x003445d4;
                        continue;
                    }
                    pc = 0x00344598;
                    continue;
                }
                0x00344598 => {
                    c.cycles += 2;
                    c.g[S1][0] = 0u64;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1ec));
                    pc = 0x003445a0;
                    continue;
                }
                0x003445a0 => {
                    c.cycles += 13;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0x1dc));
                    c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0xd0));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x003445c4);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x003445c4)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10c));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1ec));
                        if back_edge_unwinds(c, 0x003445a0) {
                            return Ok(());
                        }
                        pc = 0x003445a0;
                        continue;
                    }
                    pc = 0x003445d4;
                    continue;
                }
                0x003445d4 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003445e0);
                    c.g[S0][0] = 0u64;
                    Callee {
                        addr: 0x00344aa0,
                        f: ee::s11::CrowdMan3d_GroupBuildMatrices,
                    }
                    .call(c, 0x003445e0)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x114), 0u64);
                    c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x94));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    let t = (c.g[V1][0] as i64) <= 0;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x110), c.g[V1][0]);
                    if t {
                        pc = 0x00344634;
                        continue;
                    }
                    pc = 0x003445f4;
                    continue;
                }
                0x003445f4 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x94));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[V1][0];
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x00344624;
                        continue;
                    }
                    pc = 0x00344608;
                    continue;
                }
                0x00344608 => {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    if t {
                        pc = 0x00344634;
                        continue;
                    }
                    pc = 0x00344618;
                    continue;
                }
                0x00344618 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                    pc = 0x00344624;
                    continue;
                }
                0x00344624 => {
                    c.cycles += 2;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x110));
                        if back_edge_unwinds(c, 0x00344608) {
                            return Ok(());
                        }
                        pc = 0x00344608;
                        continue;
                    }
                    pc = 0x0034462c;
                    continue;
                }
                0x0034462c => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x114), c.g[V0][0]);
                    pc = 0x00344634;
                    continue;
                }
                0x00344634 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32));
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0xc4;
                    c.g[RA][0] = sx(0x00344648);
                    c.g[S0][0] = 0u64;
                    call_ptr(c, 0x003e6448, 0x00344648)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x110));
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x118), c.g[V0][0]);
                    if t {
                        pc = 0x003446fc;
                        continue;
                    }
                    pc = 0x00344654;
                    continue;
                }
                0x00344654 => {
                    c.cycles += 5;
                    c.g[T2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x11c));
                    c.g[T3][0] = 0x14;
                    c.g[T4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x13c));
                    c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x94));
                    pc = 0x00344668;
                    continue;
                }
                0x00344668 => {
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                    let t = c.g[V1][0] == 0u64;
                    c.mult(0, 3, 16, 11);
                    if t {
                        pc = 0x003446dc;
                        continue;
                    }
                    pc = 0x00344680;
                    continue;
                }
                0x00344680 => {
                    c.cycles += 10;
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[T1][0] = 0xffffffffffffffff;
                    c.g[A1][0] = sx((c.g[T4][0] as u32).wrapping_add(0x2));
                    c.g[A2][0] = 0x2;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x13c));
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sw((c.g[A0][0] as u32), c.g[V1][0]);
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    pc = 0x003446a8;
                    continue;
                }
                0x003446a8 => {
                    c.cycles += 7;
                    c.sh((c.g[A1][0] as u32), c.g[T1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[A2][0] as i64) >= 0;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2));
                    if t {
                        if back_edge_unwinds(c, 0x003446a8) {
                            return Ok(());
                        }
                        pc = 0x003446a8;
                        continue;
                    }
                    pc = 0x003446c4;
                    continue;
                }
                0x003446c4 => {
                    c.cycles += 6;
                    c.mult(0, 3, 16, 11);
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                    c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x4));
                    let t = true;
                    c.sh((c.g[A0][0] as u32).wrapping_add(0x13c), c.g[V1][0]);
                    if t {
                        pc = 0x003446e8;
                        continue;
                    }
                    pc = 0x003446dc;
                    continue;
                }
                0x003446dc => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                    pc = 0x003446e8;
                    continue;
                }
                0x003446e8 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x110));
                    c.g[S0][0] = c.g[T0][0];
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x94));
                        if back_edge_unwinds(c, 0x00344668) {
                            return Ok(());
                        }
                        pc = 0x00344668;
                        continue;
                    }
                    pc = 0x003446fc;
                    continue;
                }
                0x003446fc => {
                    c.cycles += 12;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003443a8, 0x0034472c);
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
    fn crowd_man3d_group_create_meshes_matches() {
        check(
            ee::s11::f_003443a8__generated,
            crowd_man3d_group_create_meshes,
            200,
            &|c, seed| {
                machine(c, seed, 0x43a8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x003443a8;
            },
        );
    }
}
