//! Staging module oc575 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc575`.

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
    0x00343c60,
    crowd_man3d_add_instance,
    "CrowdMan3d_AddInstance",
)];

/// `CrowdMan3d_AddInstance` (0x00343c60): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn crowd_man3d_add_instance(c: &mut Cpu) {
    if c.pc != 0x00343c60 {
        return ee::s11::f_00343c60__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00343c60;
        loop {
            match pc {
                0x00343c60 => {
                    c.cycles += 17;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[RA][0] = sx(0x00343c84);
                    c.g[S3][0] = c.g[A1][0];
                    call_ptr(c, 0x002d1bd8, 0x00343c84)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V0][0] = c.g[S3][0] & 0xff;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) >> 8);
                    if t {
                        pc = 0x00343cc0;
                        continue;
                    }
                    pc = 0x00343ca4;
                    continue;
                }
                0x00343ca4 => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[A0][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x00343cc4;
                        continue;
                    }
                    pc = 0x00343cc0;
                    continue;
                }
                0x00343cc0 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x00343cc4;
                    continue;
                }
                0x00343cc4 => {
                    c.cycles += 5;
                    c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x80));
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = 0x1f4;
                    pc = 0x00343cd8;
                    continue;
                }
                0x00343cd8 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[T0][0] as u32));
                    c.mult(0, 5, 6, 2);
                    c.g[A0][0] = sx((c.g[A0][0] as u32) >> 8);
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));

                    if c.g[A0][0] != c.g[V1][0] {
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                        pc = 0x00343dcc;
                        continue;
                    }
                    pc = 0x00343cf4;
                    continue;
                }
                0x00343cf4 => {
                    c.cycles += 5;
                    c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[V1][0] = 0u64;
                    c.g[S2][0] = 0u64;
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    pc = 0x00343d08;
                    continue;
                }
                0x00343d08 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = !(0u64 | c.g[V0][0]);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00343db4;
                        continue;
                    }
                    pc = 0x00343d18;
                    continue;
                }
                0x00343d18 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                    let t = (c.g[V0][0] as i64) > 0;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00343d2c;
                        continue;
                    }
                    pc = 0x00343d24;
                    continue;
                }
                0x00343d24 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00343d2c);
                    c.g[A1][0] = c.g[S3][0];
                    Callee {
                        addr: 0x00344368,
                        f: ee::s11::CrowdMan3d_GroupInit,
                    }
                    .call(c, 0x00343d2c)?;
                    pc = 0x00343d2c;
                    continue;
                }
                0x00343d2c => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x00343d34);
                    call_ptr(c, 0x002d1bd8, 0x00343d34)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V0][0] = c.g[S3][0] & 0xff;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                        pc = 0x00343d5c;
                        continue;
                    }
                    pc = 0x00343d54;
                    continue;
                }
                0x00343d54 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00343d80;
                        continue;
                    }
                    pc = 0x00343d5c;
                    continue;
                }
                0x00343d5c => {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) >> 8);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[A0][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x00343d7c;
                        continue;
                    }
                    pc = 0x00343d78;
                    continue;
                }
                0x00343d78 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x00343d7c;
                    continue;
                }
                0x00343d7c => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[V0][0];
                    pc = 0x00343d80;
                    continue;
                }
                0x00343d80 => {
                    c.cycles += 11;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[V0][0] = c.g[V0][0] | 0xfffd;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                    c.g[V1][0] = c.g[V1][0] | 0x4;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sw((c.g[S1][0] as u32), c.g[S3][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x114));
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                    if t {
                        pc = 0x00343eb8;
                        continue;
                    }
                    pc = 0x00343dac;
                    continue;
                }
                0x00343dac => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = 0u64;
                    if t {
                        pc = 0x00343ee0;
                        continue;
                    }
                    pc = 0x00343db4;
                    continue;
                }
                0x00343db4 => {
                    c.cycles += 4;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 32i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        if back_edge_unwinds(c, 0x00343d08) {
                            return Ok(());
                        }
                        pc = 0x00343d08;
                        continue;
                    }
                    pc = 0x00343dc4;
                    continue;
                }
                0x00343dc4 => {
                    c.cycles += 2;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x00343f1c;
                        continue;
                    }
                    pc = 0x00343dcc;
                    continue;
                }
                0x00343dcc => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A2][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x1f4;
                    if t {
                        if back_edge_unwinds(c, 0x00343cd8) {
                            return Ok(());
                        }
                        pc = 0x00343cd8;
                        continue;
                    }
                    pc = 0x00343dd8;
                    continue;
                }
                0x00343dd8 => {
                    c.cycles += 4;
                    c.g[A2][0] = 0xffffffffffff0000;
                    c.g[V1][0] = 0u64;
                    c.g[A2][0] = c.g[A2][0] | 0xffff;
                    pc = 0x00343de8;
                    continue;
                }
                0x00343de8 => {
                    c.cycles += 5;
                    c.mult(0, 4, 3, 2);
                    c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                    let t = c.g[V0][0] != c.g[A2][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00343f0c;
                        continue;
                    }
                    pc = 0x00343dfc;
                    continue;
                }
                0x00343dfc => {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32));
                    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                    c.g[V1][0] = 0u64;
                    c.g[S2][0] = 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 8);
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
                    c.sw((c.g[A1][0] as u32), c.g[V0][0]);
                    pc = 0x00343e18;
                    continue;
                }
                0x00343e18 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                    let t = c.g[V0][0] != c.g[A2][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x00343ef4;
                        continue;
                    }
                    pc = 0x00343e24;
                    continue;
                }
                0x00343e24 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                    let t = (c.g[V0][0] as i64) > 0;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x00343e38;
                        continue;
                    }
                    pc = 0x00343e30;
                    continue;
                }
                0x00343e30 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00343e38);
                    c.g[A1][0] = c.g[S3][0];
                    Callee {
                        addr: 0x00344368,
                        f: ee::s11::CrowdMan3d_GroupInit,
                    }
                    .call(c, 0x00343e38)?;
                    pc = 0x00343e38;
                    continue;
                }
                0x00343e38 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x00343e40);
                    call_ptr(c, 0x002d1bd8, 0x00343e40)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V0][0] = c.g[S3][0] & 0xff;
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32));

                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                        pc = 0x00343e68;
                        continue;
                    }
                    pc = 0x00343e60;
                    continue;
                }
                0x00343e60 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00343e8c;
                        continue;
                    }
                    pc = 0x00343e68;
                    continue;
                }
                0x00343e68 => {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[S3][0] as u32) >> 8);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[A0][0] as u32) >> 8);
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    if t {
                        pc = 0x00343e88;
                        continue;
                    }
                    pc = 0x00343e84;
                    continue;
                }
                0x00343e84 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x00343e88;
                    continue;
                }
                0x00343e88 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[V0][0];
                    pc = 0x00343e8c;
                    continue;
                }
                0x00343e8c => {
                    c.cycles += 11;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[V0][0] = c.g[V0][0] | 0xfffd;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                    c.g[V1][0] = c.g[V1][0] | 0x4;
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sw((c.g[S1][0] as u32), c.g[S3][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x114));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                    if t {
                        pc = 0x00343edc;
                        continue;
                    }
                    pc = 0x00343eb8;
                    continue;
                }
                0x00343eb8 => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x00343ec0);
                    call_ptr(c, 0x003177f0, 0x00343ec0)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x114));
                    c.divu(0, 2, 3);

                    if c.g[V1][0] == 0u64 {
                        pc = 0x00343ed0;
                        continue;
                    }
                    pc = 0x00343ed0;
                    continue;
                }
                0x00343ed0 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.hi[0];
                    let t = true;
                    c.sw((c.g[S2][0] as u32), c.g[A0][0]);
                    if t {
                        pc = 0x00343ee4;
                        continue;
                    }
                    pc = 0x00343edc;
                    continue;
                }
                0x00343edc => {
                    c.cycles += 1;
                    c.g[A0][0] = 0u64;
                    pc = 0x00343ee0;
                    continue;
                }
                0x00343ee0 => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32), c.g[A0][0]);
                    pc = 0x00343ee4;
                    continue;
                }
                0x00343ee4 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    let t = true;
                    c.sw((c.g[S0][0] as u32), c.g[V0][0]);
                    if t {
                        pc = 0x00343f18;
                        continue;
                    }
                    pc = 0x00343ef4;
                    continue;
                }
                0x00343ef4 => {
                    c.cycles += 4;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 32i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
                    if t {
                        if back_edge_unwinds(c, 0x00343e18) {
                            return Ok(());
                        }
                        pc = 0x00343e18;
                        continue;
                    }
                    pc = 0x00343f04;
                    continue;
                }
                0x00343f04 => {
                    c.cycles += 2;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        pc = 0x00343f1c;
                        continue;
                    }
                    pc = 0x00343f0c;
                    continue;
                }
                0x00343f0c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x1f4;
                    if t {
                        if back_edge_unwinds(c, 0x00343de8) {
                            return Ok(());
                        }
                        pc = 0x00343de8;
                        continue;
                    }
                    pc = 0x00343f18;
                    continue;
                }
                0x00343f18 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    pc = 0x00343f1c;
                    continue;
                }
                0x00343f1c => {
                    c.cycles += 6;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00343c60, 0x00343f34);
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
    fn crowd_man3d_add_instance_matches() {
        check(
            ee::s11::f_00343c60__generated,
            crowd_man3d_add_instance,
            200,
            &|c, seed| {
                machine(c, seed, 0x3c60, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00343c60;
            },
        );
    }
}
