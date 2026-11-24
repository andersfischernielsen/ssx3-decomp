//! Staging module oc477 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc477`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x00142a50, rider_base_dtor, "RiderBase_dtor")];

/// `RiderBase_dtor` (0x00142a50): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn rider_base_dtor(c: &mut Cpu) {
    if c.pc != 0x00142a50 {
        return ee::s01::f_00142a50__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00142a50;
        loop {
            match pc {
                0x00142a50 => {
                    c.cycles += 33;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2c0));
                    c.g[V0][0] = 0x460000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2b0));
                    c.g[V1][0] = 0x460000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2a0));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x290));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x78f0));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x250));
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x78c8));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x280));
                    c.g[T0][0] = c.g[V1][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x270));
                    c.g[V1][0] = 0x460000;
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x260));
                    c.g[T2][0] = c.g[V1][0];
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x240));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7880));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x230));
                    c.g[S2][0] = c.g[A1][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x220), c.g[RA][0]);
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xde8), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = 0x460000;
                    c.g[S6][0] = c.g[V0][0];
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6470));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x6d0), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    let t = c.g[S2][0] != 0u64;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
                    if t {
                        pc = 0x00142ba0;
                        continue;
                    }
                    pc = 0x00142ad4;
                    continue;
                }
                0x00142ad4 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.g[A2][0];
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
                    c.g[A3][0] = c.g[V0][0];
                    c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
                    pc = 0x00142ae8;
                    continue;
                }
                0x00142ae8 => {
                    c.cycles += 11;
                    c.g[A1][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T4][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[V0][0] as u32, c.g[A1][0]);
                    c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[T3][0]);
                    c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00142ae8) {
                            return Ok(());
                        }
                        pc = 0x00142ae8;
                        continue;
                    }
                    pc = 0x00142b14;
                    continue;
                }
                0x00142b14 => {
                    c.cycles += 14;
                    c.g[A0][0] = c.ld(c.g[V1][0] as u32);
                    c.sd(c.g[V0][0] as u32, c.g[A0][0]);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x78c8));
                    c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x7880));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x6e8), c.g[A3][0]);
                    c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[T1][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xdf0));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x108), c.g[V1][0]);
                    pc = 0x00142b4c;
                    continue;
                }
                0x00142b4c => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T4][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[T0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00142b4c) {
                            return Ok(());
                        }
                        pc = 0x00142b4c;
                        continue;
                    }
                    pc = 0x00142b78;
                    continue;
                }
                0x00142b78 => {
                    c.cycles += 10;
                    c.g[T0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T3][0]);
                    c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x7880));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x158), c.g[V1][0]);
                    pc = 0x00142ba0;
                    continue;
                }
                0x00142ba0 => {
                    c.cycles += 76;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd20));
                    c.g[RA][0] = sx(0x00142bac);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002f6388, 0x00142bac)?;
                    c.g[V1][0] = c.g[S2][0] & 0x1;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc70));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x20c), c.g[V1][0]);
                    c.g[RA][0] = sx(0x00142bc0);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002f0fe8, 0x00142bc0)?;
                    c.g[S2][0] = c.g[S2][0] & 0x2;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x214), c.g[A0][0]);
                    c.g[RA][0] = sx(0x00142bd8);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb40));
                    call_ptr(c, 0x002df1c8, 0x00142bd8)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x210), c.g[S2][0]);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb00));
                    c.g[RA][0] = sx(0x00142be8);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002e37d0, 0x00142be8)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xaf0));
                    c.g[RA][0] = sx(0x00142bf4);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002d4a70, 0x00142bf4)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xad0));
                    c.g[RA][0] = sx(0x00142c00);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002ef530, 0x00142c00)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c0));
                    c.g[RA][0] = sx(0x00142c0c);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002eac60, 0x00142c0c)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x610));
                    c.g[RA][0] = sx(0x00142c18);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002e64c0, 0x00142c18)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x520));
                    c.g[RA][0] = sx(0x00142c24);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002e83f0, 0x00142c24)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x470));
                    c.g[RA][0] = sx(0x00142c30);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002da8e8, 0x00142c30)?;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3b0));
                    c.g[RA][0] = sx(0x00142c3c);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002dcda0, 0x00142c3c)?;
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x200));
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f0));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c0));
                    c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.g[T4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x208), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x204), c.g[A2][0]);
                    c.g[A0][0] = 0x460000;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x200), c.g[A3][0]);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x76d8));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x21c), c.g[T1][0]);
                    c.g[A3][0] = c.g[V0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x218), c.g[T4][0]);
                    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[A2][0] = 0x460000;
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3a0));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3a0));
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7690));
                    c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x390));
                    c.g[T3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x370));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[V0][0]);
                    c.g[T5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x360));
                    c.g[T6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x350));
                    c.g[T7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c0));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3a0));
                    c.g[T8][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b0));
                    c.g[T9][0] = sx((c.g[S0][0] as u32).wrapping_add(0x290));
                    c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x230));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A0][0]);
                    c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x210));
                    c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d0));
                    c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x110));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3a0));
                    c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb0));
                    c.g[T1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
                    c.g[A2][0] = c.g[SP][0];
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A1][0]);
                    pc = 0x00142cd0;
                    continue;
                }
                0x00142cd0 => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[A3][0] as u32);
                    c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A2][0] as u32, c.g[T4][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                    let t = c.g[A3][0] != c.g[T1][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00142cd0) {
                            return Ok(());
                        }
                        pc = 0x00142cd0;
                        continue;
                    }
                    pc = 0x00142cfc;
                    continue;
                }
                0x00142cfc => {
                    c.cycles += 16;
                    c.g[A1][0] = 0x460000;
                    c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x76d8));
                    c.g[T1][0] = c.ld(c.g[A3][0] as u32);
                    c.sd(c.g[A2][0] as u32, c.g[T1][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A3][0] = 0x460000;
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7690));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T4][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T4][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00142d3c;
                    continue;
                }
                0x00142d3c => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00142d3c) {
                            return Ok(());
                        }
                        pc = 0x00142d3c;
                        continue;
                    }
                    pc = 0x00142d68;
                    continue;
                }
                0x00142d68 => {
                    c.cycles += 25;
                    c.g[A3][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A3][0]);
                    c.g[A0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7690));
                    c.g[A2][0] = 0x460000;
                    c.g[T0][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x72f8));
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x72b0));
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
                    c.g[S2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00142dcc;
                    continue;
                }
                0x00142dcc => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[S2][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00142dcc) {
                            return Ok(());
                        }
                        pc = 0x00142dcc;
                        continue;
                    }
                    pc = 0x00142df8;
                    continue;
                }
                0x00142df8 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x72f8));
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A1][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x72b0));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T4][0] = sx((c.g[T2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T4][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00142e38;
                    continue;
                }
                0x00142e38 => {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00142e38) {
                            return Ok(());
                        }
                        pc = 0x00142e38;
                        continue;
                    }
                    pc = 0x00142e64;
                    continue;
                }
                0x00142e64 => {
                    c.cycles += 25;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[V1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x72b0));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x77d0));
                    c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.g[T1][0] = c.g[A0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7788));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00142ec8;
                    continue;
                }
                0x00142ec8 => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T2][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00142ec8) {
                            return Ok(());
                        }
                        pc = 0x00142ec8;
                        continue;
                    }
                    pc = 0x00142ef4;
                    continue;
                }
                0x00142ef4 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x77d0));
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A1][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7788));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x20));
                    c.g[T2][0] = sx((c.g[T3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00142f34;
                    continue;
                }
                0x00142f34 => {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T4][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00142f34) {
                            return Ok(());
                        }
                        pc = 0x00142f34;
                        continue;
                    }
                    pc = 0x00142f60;
                    continue;
                }
                0x00142f60 => {
                    c.cycles += 25;
                    c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[T1][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[T4][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7788));
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x14));
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x74a0));
                    c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x74e8));
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[T1][0] = c.g[A0][0];
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00142fc4;
                    continue;
                }
                0x00142fc4 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[A0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00142fc4) {
                            return Ok(());
                        }
                        pc = 0x00142fc4;
                        continue;
                    }
                    pc = 0x00142ff0;
                    continue;
                }
                0x00142ff0 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x4));
                    c.g[T3][0] = 0x460000;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x74e8));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.g[A3][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x74a0));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[T5][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00143030;
                    continue;
                }
                0x00143030 => {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143030) {
                            return Ok(());
                        }
                        pc = 0x00143030;
                        continue;
                    }
                    pc = 0x0014305c;
                    continue;
                }
                0x0014305c => {
                    c.cycles += 25;
                    c.g[V0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V0][0]);
                    c.g[T0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[T1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[T5][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x74a0));
                    c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x75e0));
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[T3][0] = 0x460000;
                    c.g[A3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7598));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x001430c0;
                    continue;
                }
                0x001430c0 => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001430c0) {
                            return Ok(());
                        }
                        pc = 0x001430c0;
                        continue;
                    }
                    pc = 0x001430ec;
                    continue;
                }
                0x001430ec => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x75e0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7598));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[T6][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x0014312c;
                    continue;
                }
                0x0014312c => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x0014312c) {
                            return Ok(());
                        }
                        pc = 0x0014312c;
                        continue;
                    }
                    pc = 0x00143158;
                    continue;
                }
                0x00143158 => {
                    c.cycles += 25;
                    c.g[T4][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[T4][0]);
                    c.g[A3][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[T0][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7598));
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x73f0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x73a8));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x80));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x80));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x80));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x001431bc;
                    continue;
                }
                0x001431bc => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001431bc) {
                            return Ok(());
                        }
                        pc = 0x001431bc;
                        continue;
                    }
                    pc = 0x001431e8;
                    continue;
                }
                0x001431e8 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x80));
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x73f0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x73a8));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x80));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x90));
                    c.g[T2][0] = sx((c.g[T7][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00143228;
                    continue;
                }
                0x00143228 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143228) {
                            return Ok(());
                        }
                        pc = 0x00143228;
                        continue;
                    }
                    pc = 0x00143254;
                    continue;
                }
                0x00143254 => {
                    c.cycles += 25;
                    c.g[T4][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[T4][0]);
                    c.g[A3][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[T0][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x80));
                    c.g[A2][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x73a8));
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7200));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x71b8));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x001432b8;
                    continue;
                }
                0x001432b8 => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001432b8) {
                            return Ok(());
                        }
                        pc = 0x001432b8;
                        continue;
                    }
                    pc = 0x001432e4;
                    continue;
                }
                0x001432e4 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7200));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x71b8));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[T8][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00143324;
                    continue;
                }
                0x00143324 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143324) {
                            return Ok(());
                        }
                        pc = 0x00143324;
                        continue;
                    }
                    pc = 0x00143350;
                    continue;
                }
                0x00143350 => {
                    c.cycles += 25;
                    c.g[T4][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[T4][0]);
                    c.g[A3][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[T0][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x71b8));
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7108));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x70c0));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x14));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x001433b4;
                    continue;
                }
                0x001433b4 => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001433b4) {
                            return Ok(());
                        }
                        pc = 0x001433b4;
                        continue;
                    }
                    pc = 0x001433e0;
                    continue;
                }
                0x001433e0 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7108));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x70c0));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x20));
                    c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00143420;
                    continue;
                }
                0x00143420 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143420) {
                            return Ok(());
                        }
                        pc = 0x00143420;
                        continue;
                    }
                    pc = 0x0014344c;
                    continue;
                }
                0x0014344c => {
                    c.cycles += 25;
                    c.g[T4][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[T4][0]);
                    c.g[A3][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[T0][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x14));
                    c.g[A2][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x70c0));
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7010));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6fc8));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x58));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x58));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x58));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x001434b0;
                    continue;
                }
                0x001434b0 => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001434b0) {
                            return Ok(());
                        }
                        pc = 0x001434b0;
                        continue;
                    }
                    pc = 0x001434dc;
                    continue;
                }
                0x001434dc => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x58));
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7010));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6fc8));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x58));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x60));
                    c.g[T2][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x0014351c;
                    continue;
                }
                0x0014351c => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x0014351c) {
                            return Ok(());
                        }
                        pc = 0x0014351c;
                        continue;
                    }
                    pc = 0x00143548;
                    continue;
                }
                0x00143548 => {
                    c.cycles += 25;
                    c.g[T4][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[T4][0]);
                    c.g[A3][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[T0][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x58));
                    c.g[A2][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6fc8));
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6f18));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6ed0));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x14));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x001435ac;
                    continue;
                }
                0x001435ac => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001435ac) {
                            return Ok(());
                        }
                        pc = 0x001435ac;
                        continue;
                    }
                    pc = 0x001435d8;
                    continue;
                }
                0x001435d8 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6f18));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6ed0));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x20));
                    c.g[T2][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00143618;
                    continue;
                }
                0x00143618 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143618) {
                            return Ok(());
                        }
                        pc = 0x00143618;
                        continue;
                    }
                    pc = 0x00143644;
                    continue;
                }
                0x00143644 => {
                    c.cycles += 25;
                    c.g[T4][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[T4][0]);
                    c.g[A3][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[T0][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x14));
                    c.g[A2][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6ed0));
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6e20));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6dd8));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x200));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x200));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x200));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x001436a8;
                    continue;
                }
                0x001436a8 => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001436a8) {
                            return Ok(());
                        }
                        pc = 0x001436a8;
                        continue;
                    }
                    pc = 0x001436d4;
                    continue;
                }
                0x001436d4 => {
                    c.cycles += 18;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x208));
                    c.g[A2][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6e20));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A3][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A1][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[A3][0]);
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6dd8));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x208));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x0014371c;
                    continue;
                }
                0x0014371c => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T4][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x0014371c) {
                            return Ok(());
                        }
                        pc = 0x0014371c;
                        continue;
                    }
                    pc = 0x00143748;
                    continue;
                }
                0x00143748 => {
                    c.cycles += 26;
                    c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[T1][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[T3][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x6dd8));
                    c.g[V1][0] = 0x460000;
                    c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x208));
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6ce0));
                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6d28));
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[V0][0] = c.lw(c.g[T4][0] as u32);
                    c.g[T1][0] = c.g[A0][0];
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f0));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x001437b0;
                    continue;
                }
                0x001437b0 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[A0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001437b0) {
                            return Ok(());
                        }
                        pc = 0x001437b0;
                        continue;
                    }
                    pc = 0x001437dc;
                    continue;
                }
                0x001437dc => {
                    c.cycles += 17;
                    c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x204));
                    c.g[T4][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x6d28));
                    c.g[V0][0] = c.lw(c.g[T3][0] as u32);
                    c.g[V1][0] = c.ld(c.g[T1][0] as u32);
                    c.sd(c.g[T0][0] as u32, c.g[V1][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[T0][0] = 0x460000;
                    c.g[A2][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6ce0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw(c.g[T3][0] as u32);
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[T3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00143820;
                    continue;
                }
                0x00143820 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T4][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143820) {
                            return Ok(());
                        }
                        pc = 0x00143820;
                        continue;
                    }
                    pc = 0x0014384c;
                    continue;
                }
                0x0014384c => {
                    c.cycles += 26;
                    c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[T1][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[T3][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x6ce0));
                    c.g[V1][0] = 0x460000;
                    c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x204));
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6be8));
                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6c30));
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[V0][0] = c.lw(c.g[T4][0] as u32);
                    c.g[T1][0] = c.g[A0][0];
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x001438b4;
                    continue;
                }
                0x001438b4 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[A0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001438b4) {
                            return Ok(());
                        }
                        pc = 0x001438b4;
                        continue;
                    }
                    pc = 0x001438e0;
                    continue;
                }
                0x001438e0 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    c.g[T3][0] = 0x460000;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6c30));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.g[A3][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6be8));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x20));
                    c.g[T2][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00143920;
                    continue;
                }
                0x00143920 => {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143920) {
                            return Ok(());
                        }
                        pc = 0x00143920;
                        continue;
                    }
                    pc = 0x0014394c;
                    continue;
                }
                0x0014394c => {
                    c.cycles += 25;
                    c.g[V0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V0][0]);
                    c.g[T0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[T1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6be8));
                    c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x6b38));
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[T3][0] = 0x460000;
                    c.g[A3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6af0));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x001439b0;
                    continue;
                }
                0x001439b0 => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001439b0) {
                            return Ok(());
                        }
                        pc = 0x001439b0;
                        continue;
                    }
                    pc = 0x001439dc;
                    continue;
                }
                0x001439dc => {
                    c.cycles += 18;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x200));
                    c.g[A2][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6b38));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A3][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A1][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[A3][0]);
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6af0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x200));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00143a24;
                    continue;
                }
                0x00143a24 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T4][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143a24) {
                            return Ok(());
                        }
                        pc = 0x00143a24;
                        continue;
                    }
                    pc = 0x00143a50;
                    continue;
                }
                0x00143a50 => {
                    c.cycles += 26;
                    c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[T1][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[T3][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x6af0));
                    c.g[V1][0] = 0x460000;
                    c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x200));
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x69f8));
                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6a40));
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[V0][0] = c.lw(c.g[T4][0] as u32);
                    c.g[T1][0] = c.g[A0][0];
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00143ab8;
                    continue;
                }
                0x00143ab8 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[A0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143ab8) {
                            return Ok(());
                        }
                        pc = 0x00143ab8;
                        continue;
                    }
                    pc = 0x00143ae4;
                    continue;
                }
                0x00143ae4 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xa0));
                    c.g[T3][0] = 0x460000;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6a40));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.g[A3][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x69f8));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xa0));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb0));
                    c.g[T2][0] = sx((c.g[S7][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00143b24;
                    continue;
                }
                0x00143b24 => {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143b24) {
                            return Ok(());
                        }
                        pc = 0x00143b24;
                        continue;
                    }
                    pc = 0x00143b50;
                    continue;
                }
                0x00143b50 => {
                    c.cycles += 25;
                    c.g[V0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V0][0]);
                    c.g[T0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[T1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x69f8));
                    c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x6758));
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[T3][0] = 0x460000;
                    c.g[A3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6710));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x50));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x50));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x50));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00143bb4;
                    continue;
                }
                0x00143bb4 => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143bb4) {
                            return Ok(());
                        }
                        pc = 0x00143bb4;
                        continue;
                    }
                    pc = 0x00143be0;
                    continue;
                }
                0x00143be0 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x50));
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6758));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6710));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x50));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x60));
                    c.g[T2][0] = sx((c.g[FP][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00143c20;
                    continue;
                }
                0x00143c20 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T3][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143c20) {
                            return Ok(());
                        }
                        pc = 0x00143c20;
                        continue;
                    }
                    pc = 0x00143c4c;
                    continue;
                }
                0x00143c4c => {
                    c.cycles += 25;
                    c.g[T4][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[T4][0]);
                    c.g[A3][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[T0][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x50));
                    c.g[A2][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6710));
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6948));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6900));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00143cb0;
                    continue;
                }
                0x00143cb0 => {
                    c.cycles += 11;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143cb0) {
                            return Ok(());
                        }
                        pc = 0x00143cb0;
                        continue;
                    }
                    pc = 0x00143cdc;
                    continue;
                }
                0x00143cdc => {
                    c.cycles += 18;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x21c));
                    c.g[A2][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6948));
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A3][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A1][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[A3][0]);
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6900));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x21c));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00143d24;
                    continue;
                }
                0x00143d24 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T4][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[T0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143d24) {
                            return Ok(());
                        }
                        pc = 0x00143d24;
                        continue;
                    }
                    pc = 0x00143d50;
                    continue;
                }
                0x00143d50 => {
                    c.cycles += 31;
                    c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[T3][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x218));
                    c.g[A2][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6900));
                    c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x21c));
                    c.g[A1][0] = 0u64;
                    c.g[V1][0] = c.lw(c.g[T4][0] as u32);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[V0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[RA][0] = sx(0x00143d8c);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    Callee {
                        addr: 0x00136ad0,
                        f: ee::s01::RiderWipeoutState_dtor,
                    }
                    .call(c, 0x00143d8c)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x214));
                    c.g[A0][0] = 0x460000;
                    c.g[A2][0] = 0x460000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6618));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6660));
                    c.g[A2][0] = c.g[SP][0];
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[V0][0]);
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x214));
                    c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A0][0]);
                    c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A1][0]);
                    pc = 0x00143dcc;
                    continue;
                }
                0x00143dcc => {
                    c.cycles += 11;
                    c.g[T3][0] = c.ld(c.g[A3][0] as u32);
                    c.g[T4][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A2][0] as u32, c.g[T3][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T4][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                    let t = c.g[A3][0] != c.g[T0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143dcc) {
                            return Ok(());
                        }
                        pc = 0x00143dcc;
                        continue;
                    }
                    pc = 0x00143df8;
                    continue;
                }
                0x00143df8 => {
                    c.cycles += 18;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x214));
                    c.g[A1][0] = 0x460000;
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[T0][0] = c.ld(c.g[A3][0] as u32);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6660));
                    c.sd(c.g[A2][0] as u32, c.g[T0][0]);
                    c.g[A1][0] = c.g[S1][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6618));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x214));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00143e40;
                    continue;
                }
                0x00143e40 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T4][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143e40) {
                            return Ok(());
                        }
                        pc = 0x00143e40;
                        continue;
                    }
                    pc = 0x00143e6c;
                    continue;
                }
                0x00143e6c => {
                    c.cycles += 26;
                    c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[T1][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[T3][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x6618));
                    c.g[V1][0] = 0x460000;
                    c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x214));
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6520));
                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6568));
                    c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x6470));
                    c.g[V0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x4));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00143ed4;
                    continue;
                }
                0x00143ed4 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[A0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143ed4) {
                            return Ok(());
                        }
                        pc = 0x00143ed4;
                        continue;
                    }
                    pc = 0x00143f00;
                    continue;
                }
                0x00143f00 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[T3][0] = 0x460000;
                    c.g[T4][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A3][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[T4][0]);
                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6568));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6520));
                    c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x20));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00143f40;
                    continue;
                }
                0x00143f40 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T4][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[T0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00143f40) {
                            return Ok(());
                        }
                        pc = 0x00143f40;
                        continue;
                    }
                    pc = 0x00143f6c;
                    continue;
                }
                0x00143f6c => {
                    c.cycles += 13;
                    c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[T3][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x6520));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[T4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x210));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    let t = c.g[T4][0] == 0u64;
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    if t {
                        pc = 0x00143fac;
                        continue;
                    }
                    pc = 0x00143fa0;
                    continue;
                }
                0x00143fa0 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[RA][0] = sx(0x00143fac);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0011b978, 0x00143fac)?;
                    pc = 0x00143fac;
                    continue;
                }
                0x00143fac => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20c));

                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2b0));
                        pc = 0x00143fc4;
                        continue;
                    }
                    pc = 0x00143fb8;
                    continue;
                }
                0x00143fb8 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00143fc0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317e50, 0x00143fc0)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2b0));
                    pc = 0x00143fc4;
                    continue;
                }
                0x00143fc4 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2a0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x290));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x280));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x270));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x260));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x250));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x240));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x230));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x220));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2c0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00142a50, 0x00143ff0);
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
    fn rider_base_dtor_matches() {
        check(
            ee::s01::f_00142a50__generated,
            rider_base_dtor,
            200,
            &|c, seed| {
                machine(c, seed, 0x2a50, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00142a50;
            },
        );
    }
}
