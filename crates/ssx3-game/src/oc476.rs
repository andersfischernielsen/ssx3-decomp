//! Staging module oc476 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc476`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x00125eb8, player_dtor, "Player_dtor")];

/// `Player_dtor` (0x00125eb8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn player_dtor(c: &mut Cpu) {
    if c.pc != 0x00125eb8 {
        return ee::s00::f_00125eb8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00125eb8;
        loop {
            match pc {
                0x00125eb8 => {
                    c.cycles += 38;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x450));
                    c.g[V0][0] = 0x460000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x440));
                    c.g[V1][0] = 0x460000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x430));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x420));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7cc8));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x410));
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7ca0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x400));
                    c.g[A3][0] = c.g[V1][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x3f0));
                    c.g[V1][0] = 0x460000;
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x3e0));
                    c.g[T1][0] = c.g[V1][0];
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x3d0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7c58));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x3c0));
                    c.g[S2][0] = 0x460000;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x3b0), c.g[RA][0]);
                    c.g[T7][0] = 0x460000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xde8), c.g[V0][0]);
                    c.g[T6][0] = 0x460000;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = 0x460000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6470));
                    c.g[T3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
                    c.g[T4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x6d0), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x300), c.g[A1][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
                    c.g[V0][0] = c.g[A1][0];
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    if t {
                        pc = 0x00126058;
                        continue;
                    }
                    pc = 0x00125f50;
                    continue;
                }
                0x00125f50 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.g[A2][0];
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
                    c.g[T0][0] = c.g[V0][0];
                    c.g[T5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x250));
                    pc = 0x00125f64;
                    continue;
                }
                0x00125f64 => {
                    c.cycles += 11;
                    c.g[A1][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[S3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[V0][0] as u32, c.g[A1][0]);
                    c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                    c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                    c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[S3][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[A0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00125f64) {
                            return Ok(());
                        }
                        pc = 0x00125f64;
                        continue;
                    }
                    pc = 0x00125f90;
                    continue;
                }
                0x00125f90 => {
                    c.cycles += 14;
                    c.g[A0][0] = c.ld(c.g[V1][0] as u32);
                    c.sd(c.g[V0][0] as u32, c.g[A0][0]);
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7ca0));
                    c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7c58));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x6e8), c.g[T0][0]);
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[T5][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe10));
                    c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x208), c.g[V1][0]);
                    pc = 0x00125fc8;
                    continue;
                }
                0x00125fc8 => {
                    c.cycles += 11;
                    c.g[S3][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00125fc8) {
                            return Ok(());
                        }
                        pc = 0x00125fc8;
                        continue;
                    }
                    pc = 0x00125ff4;
                    continue;
                }
                0x00125ff4 => {
                    c.cycles += 25;
                    c.g[A3][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7c58));
                    c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[T0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0xa0));
                    c.g[A3][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x80));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.g[A2][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x88));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.g[A0][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x90));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x98));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sw((c.g[T1][0] as u32).wrapping_add(0x6c0), c.g[T5][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x258), c.g[A3][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x2d0), c.g[A1][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x2d8), c.g[A2][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x2e0), c.g[A0][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x2e8), c.g[V1][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x2f0), c.g[T0][0]);
                    pc = 0x00126058;
                    continue;
                }
                0x00126058 => {
                    c.cycles += 85;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x300));
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc70));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb40));
                    c.g[V1][0] = c.g[V1][0] & 0x1;
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb00));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x350), c.g[V1][0]);
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xaf0));
                    c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xad0));
                    c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c0));
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xdf8));
                    c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x610));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x328), c.g[A0][0]);
                    c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3a0));
                    c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x360));
                    c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x350));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x308), c.g[A2][0]);
                    c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x35c), c.g[A3][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x34c), c.g[T0][0]);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x344), c.g[T1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x31c), c.g[T2][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x32c), c.g[S3][0]);
                    c.sq(11, (c.g[SP][0] as u32).wrapping_add(0x370));
                    c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x370));
                    c.sq(12, (c.g[SP][0] as u32).wrapping_add(0x380));
                    c.sq(14, (c.g[SP][0] as u32).wrapping_add(0x390));
                    c.g[RA][0] = sx(0x001260d8);
                    c.sq(15, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                    call_ptr(c, 0x002700d8, 0x001260d8)?;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x520));
                    c.lq(15, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x318), c.g[V0][0]);
                    c.g[A2][0] = 0x460000;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x78f0));
                    c.g[V1][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x78c8));
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x470));
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xde8), c.g[V0][0]);
                    c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3b0));
                    c.sw((c.g[A0][0] as u32).wrapping_add(0x6e8), c.g[V1][0]);
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6470));
                    c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd20));
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x390));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x358), c.g[T0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x324), c.g[T2][0]);
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x200));
                    c.lq(14, (c.g[SP][0] as u32).wrapping_add(0x390));
                    c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f0));
                    c.lq(11, (c.g[SP][0] as u32).wrapping_add(0x370));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A1][0]);
                    c.g[A2][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x7880));
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x230));
                    c.g[A0][0] = c.g[T3][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[A2][0]);
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x210));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x290));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x340), c.g[A2][0]);
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x110));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30c), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x348), c.g[A1][0]);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x320), c.g[A2][0]);
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c0));
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x300));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x33c), c.g[T0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x338), c.g[T2][0]);
                    c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xb0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x334), c.g[V0][0]);
                    c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x330), c.g[A1][0]);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                    c.g[A2][0] = c.g[A2][0] & 0x2;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x314), c.g[T0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x310), c.g[T2][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x304), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x360), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x354), c.g[A2][0]);
                    c.lq(12, (c.g[SP][0] as u32).wrapping_add(0x380));
                    pc = 0x001261ac;
                    continue;
                }
                0x001261ac => {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[V1][0] as u32);
                    c.g[T2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[A3][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001261ac) {
                            return Ok(());
                        }
                        pc = 0x001261ac;
                        continue;
                    }
                    pc = 0x001261d8;
                    continue;
                }
                0x001261d8 => {
                    c.cycles += 14;
                    c.g[A2][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A1][0] = c.g[T4][0];
                    c.sd(c.g[A0][0] as u32, c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x78c8));
                    c.g[A2][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x7880));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[T3][0]);
                    c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xdf0));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x108), c.g[V1][0]);
                    pc = 0x00126210;
                    continue;
                }
                0x00126210 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T2][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[T0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126210) {
                            return Ok(());
                        }
                        pc = 0x00126210;
                        continue;
                    }
                    pc = 0x0012623c;
                    continue;
                }
                0x0012623c => {
                    c.cycles += 58;
                    c.g[V0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A2][0] = sx((c.g[T6][0] as u32).wrapping_sub(0x7880));
                    c.g[A0][0] = c.g[T1][0];
                    c.g[A1][0] = 0u64;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[T4][0]);
                    c.g[V0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[RA][0] = sx(0x00126270);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x158), c.g[V0][0]);
                    call_ptr(c, 0x002f6388, 0x00126270)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x328));
                    c.g[RA][0] = sx(0x0012627c);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002f0fe8, 0x0012627c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x308));
                    c.g[RA][0] = sx(0x00126288);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002df1c8, 0x00126288)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x35c));
                    c.g[RA][0] = sx(0x00126294);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002e37d0, 0x00126294)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34c));
                    c.g[RA][0] = sx(0x001262a0);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002d4a70, 0x001262a0)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x344));
                    c.g[RA][0] = sx(0x001262ac);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002ef530, 0x001262ac)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x31c));
                    c.g[RA][0] = sx(0x001262b8);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002eac60, 0x001262b8)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x32c));
                    c.g[RA][0] = sx(0x001262c4);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002e64c0, 0x001262c4)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x318));
                    c.g[RA][0] = sx(0x001262d0);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002e83f0, 0x001262d0)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x358));
                    c.g[RA][0] = sx(0x001262dc);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002da8e8, 0x001262dc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x324));
                    c.g[RA][0] = sx(0x001262e8);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x002dcda0, 0x001262e8)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3a0));
                    c.g[T1][0] = 0x460000;
                    c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x76d8));
                    c.g[T2][0] = 0x460000;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[V0][0]);
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x6470));
                    c.g[A2][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3a0));
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7690));
                    c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
                    c.g[A2][0] = c.g[SP][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A0][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3a0));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A1][0]);
                    pc = 0x00126324;
                    continue;
                }
                0x00126324 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A3][0] as u32);
                    c.g[T2][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A2][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                    let t = c.g[A3][0] != c.g[T0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126324) {
                            return Ok(());
                        }
                        pc = 0x00126324;
                        continue;
                    }
                    pc = 0x00126350;
                    continue;
                }
                0x00126350 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                    c.g[A1][0] = 0x460000;
                    c.g[T0][0] = c.ld(c.g[A3][0] as u32);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x76d8));
                    c.sd(c.g[A2][0] as u32, c.g[T0][0]);
                    c.g[A1][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7690));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[S6][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00126390;
                    continue;
                }
                0x00126390 => {
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
                        if back_edge_unwinds(c, 0x00126390) {
                            return Ok(());
                        }
                        pc = 0x00126390;
                        continue;
                    }
                    pc = 0x001263bc;
                    continue;
                }
                0x001263bc => {
                    c.cycles += 26;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[V1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7690));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x72f8));
                    c.g[V0][0] = c.lw(c.g[S6][0] as u32);
                    c.g[A3][0] = 0x460000;
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x72b0));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00126424;
                    continue;
                }
                0x00126424 => {
                    c.cycles += 11;
                    c.g[T2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126424) {
                            return Ok(());
                        }
                        pc = 0x00126424;
                        continue;
                    }
                    pc = 0x00126450;
                    continue;
                }
                0x00126450 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x72f8));
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A1][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x72b0));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[S2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00126490;
                    continue;
                }
                0x00126490 => {
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
                        if back_edge_unwinds(c, 0x00126490) {
                            return Ok(());
                        }
                        pc = 0x00126490;
                        continue;
                    }
                    pc = 0x001264bc;
                    continue;
                }
                0x001264bc => {
                    c.cycles += 26;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[V1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x72b0));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x77d0));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = 0x460000;
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7788));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00126524;
                    continue;
                }
                0x00126524 => {
                    c.cycles += 11;
                    c.g[T2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126524) {
                            return Ok(());
                        }
                        pc = 0x00126524;
                        continue;
                    }
                    pc = 0x00126550;
                    continue;
                }
                0x00126550 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x77d0));
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A1][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7788));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x20));
                    c.g[T2][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00126590;
                    continue;
                }
                0x00126590 => {
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
                        if back_edge_unwinds(c, 0x00126590) {
                            return Ok(());
                        }
                        pc = 0x00126590;
                        continue;
                    }
                    pc = 0x001265bc;
                    continue;
                }
                0x001265bc => {
                    c.cycles += 26;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[V1][0] = 0x460000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7788));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x74e8));
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.g[A3][0] = 0x460000;
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x74a0));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00126624;
                    continue;
                }
                0x00126624 => {
                    c.cycles += 11;
                    c.g[T2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[S3][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126624) {
                            return Ok(());
                        }
                        pc = 0x00126624;
                        continue;
                    }
                    pc = 0x00126650;
                    continue;
                }
                0x00126650 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74e8));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x74a0));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[S4][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00126690;
                    continue;
                }
                0x00126690 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[S3][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126690) {
                            return Ok(());
                        }
                        pc = 0x00126690;
                        continue;
                    }
                    pc = 0x001266bc;
                    continue;
                }
                0x001266bc => {
                    c.cycles += 26;
                    c.g[V0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V0][0]);
                    c.g[T0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[T1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x74a0));
                    c.g[V1][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x75e0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7598));
                    c.g[S3][0] = 0x460000;
                    c.g[T1][0] = c.g[A0][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6470));
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00126724;
                    continue;
                }
                0x00126724 => {
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
                        if back_edge_unwinds(c, 0x00126724) {
                            return Ok(());
                        }
                        pc = 0x00126724;
                        continue;
                    }
                    pc = 0x00126750;
                    continue;
                }
                0x00126750 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                    c.g[T2][0] = 0x460000;
                    c.g[S3][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x75e0));
                    c.sd(c.g[T0][0] as u32, c.g[S3][0]);
                    c.g[A3][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7598));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[S5][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00126790;
                    continue;
                }
                0x00126790 => {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126790) {
                            return Ok(());
                        }
                        pc = 0x00126790;
                        continue;
                    }
                    pc = 0x001267bc;
                    continue;
                }
                0x001267bc => {
                    c.cycles += 26;
                    c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[T1][0] = 0x460000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6470));
                    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x7598));
                    c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = 0x460000;
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x73a8));
                    c.g[S3][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x73f0));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x80));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x80));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x80));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00126824;
                    continue;
                }
                0x00126824 => {
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
                        if back_edge_unwinds(c, 0x00126824) {
                            return Ok(());
                        }
                        pc = 0x00126824;
                        continue;
                    }
                    pc = 0x00126850;
                    continue;
                }
                0x00126850 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x80));
                    c.g[T2][0] = 0x460000;
                    c.g[S3][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x73f0));
                    c.sd(c.g[T0][0] as u32, c.g[S3][0]);
                    c.g[A3][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x73a8));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x80));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x90));
                    c.g[T2][0] = sx((c.g[S7][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00126890;
                    continue;
                }
                0x00126890 => {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126890) {
                            return Ok(());
                        }
                        pc = 0x00126890;
                        continue;
                    }
                    pc = 0x001268bc;
                    continue;
                }
                0x001268bc => {
                    c.cycles += 26;
                    c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[T1][0] = 0x460000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6470));
                    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x73a8));
                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x80));
                    c.g[V1][0] = 0x460000;
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x71b8));
                    c.g[S3][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x7200));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00126924;
                    continue;
                }
                0x00126924 => {
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
                        if back_edge_unwinds(c, 0x00126924) {
                            return Ok(());
                        }
                        pc = 0x00126924;
                        continue;
                    }
                    pc = 0x00126950;
                    continue;
                }
                0x00126950 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                    c.g[T2][0] = 0x460000;
                    c.g[S3][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x7200));
                    c.sd(c.g[T0][0] as u32, c.g[S3][0]);
                    c.g[A3][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x71b8));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[FP][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00126990;
                    continue;
                }
                0x00126990 => {
                    c.cycles += 11;
                    c.g[T0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126990) {
                            return Ok(());
                        }
                        pc = 0x00126990;
                        continue;
                    }
                    pc = 0x001269bc;
                    continue;
                }
                0x001269bc => {
                    c.cycles += 27;
                    c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[T1][0] = 0x460000;
                    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6470));
                    c.g[A1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x71b8));
                    c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = 0x460000;
                    c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x70c0));
                    c.g[S3][0] = 0x460000;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x7108));
                    c.g[T1][0] = c.g[A0][0];
                    c.g[T0][0] = c.g[SP][0];
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00126a28;
                    continue;
                }
                0x00126a28 => {
                    c.cycles += 11;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A3][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[T2][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[S3][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[S3][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126a28) {
                            return Ok(());
                        }
                        pc = 0x00126a28;
                        continue;
                    }
                    pc = 0x00126a54;
                    continue;
                }
                0x00126a54 => {
                    c.cycles += 18;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30c));
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x7108));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x70c0));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30c));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x20));
                    c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00126a9c;
                    continue;
                }
                0x00126a9c => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126a9c) {
                            return Ok(());
                        }
                        pc = 0x00126a9c;
                        continue;
                    }
                    pc = 0x00126ac8;
                    continue;
                }
                0x00126ac8 => {
                    c.cycles += 28;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[S3][0] = 0x460000;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30c));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7010));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x70c0));
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.g[T1][0] = c.g[A0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6fc8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x348));
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x58));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x58));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x58));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00126b38;
                    continue;
                }
                0x00126b38 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126b38) {
                            return Ok(());
                        }
                        pc = 0x00126b38;
                        continue;
                    }
                    pc = 0x00126b64;
                    continue;
                }
                0x00126b64 => {
                    c.cycles += 18;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x348));
                    c.g[A3][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7010));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x58));
                    c.g[A3][0] = 0x460000;
                    c.g[T2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6fc8));
                    c.sd(c.g[T0][0] as u32, c.g[T2][0]);
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x348));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x58));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x60));
                    c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00126bac;
                    continue;
                }
                0x00126bac => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126bac) {
                            return Ok(());
                        }
                        pc = 0x00126bac;
                        continue;
                    }
                    pc = 0x00126bd8;
                    continue;
                }
                0x00126bd8 => {
                    c.cycles += 28;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[S3][0] = 0x460000;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x348));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6f18));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6fc8));
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x58));
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.g[T1][0] = c.g[A0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6ed0));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x340));
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00126c48;
                    continue;
                }
                0x00126c48 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126c48) {
                            return Ok(());
                        }
                        pc = 0x00126c48;
                        continue;
                    }
                    pc = 0x00126c74;
                    continue;
                }
                0x00126c74 => {
                    c.cycles += 18;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x340));
                    c.g[A3][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6f18));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x14));
                    c.g[A3][0] = 0x460000;
                    c.g[T2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6ed0));
                    c.sd(c.g[T0][0] as u32, c.g[T2][0]);
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x340));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x20));
                    c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00126cbc;
                    continue;
                }
                0x00126cbc => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126cbc) {
                            return Ok(());
                        }
                        pc = 0x00126cbc;
                        continue;
                    }
                    pc = 0x00126ce8;
                    continue;
                }
                0x00126ce8 => {
                    c.cycles += 27;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[S3][0] = 0x460000;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x340));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6e20));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6ed0));
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.g[T1][0] = c.g[A0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6dd8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x200));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x200));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x200));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00126d54;
                    continue;
                }
                0x00126d54 => {
                    c.cycles += 11;
                    c.g[T2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[S3][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126d54) {
                            return Ok(());
                        }
                        pc = 0x00126d54;
                        continue;
                    }
                    pc = 0x00126d80;
                    continue;
                }
                0x00126d80 => {
                    c.cycles += 18;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x33c));
                    c.g[A1][0] = 0x460000;
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6e20));
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A1][0] = c.g[S1][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6dd8));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x33c));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00126dc8;
                    continue;
                }
                0x00126dc8 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126dc8) {
                            return Ok(());
                        }
                        pc = 0x00126dc8;
                        continue;
                    }
                    pc = 0x00126df4;
                    continue;
                }
                0x00126df4 => {
                    c.cycles += 27;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[S3][0] = 0x460000;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x33c));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6d28));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6dd8));
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.g[T1][0] = c.g[A0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6ce0));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f0));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00126e60;
                    continue;
                }
                0x00126e60 => {
                    c.cycles += 11;
                    c.g[T2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[S3][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126e60) {
                            return Ok(());
                        }
                        pc = 0x00126e60;
                        continue;
                    }
                    pc = 0x00126e8c;
                    continue;
                }
                0x00126e8c => {
                    c.cycles += 18;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x338));
                    c.g[A1][0] = 0x460000;
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6d28));
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A1][0] = c.g[S1][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6ce0));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x338));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00126ed4;
                    continue;
                }
                0x00126ed4 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126ed4) {
                            return Ok(());
                        }
                        pc = 0x00126ed4;
                        continue;
                    }
                    pc = 0x00126f00;
                    continue;
                }
                0x00126f00 => {
                    c.cycles += 28;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[S3][0] = 0x460000;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x338));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c30));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6ce0));
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.g[T1][0] = c.g[A0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6be8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x334));
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x00126f70;
                    continue;
                }
                0x00126f70 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126f70) {
                            return Ok(());
                        }
                        pc = 0x00126f70;
                        continue;
                    }
                    pc = 0x00126f9c;
                    continue;
                }
                0x00126f9c => {
                    c.cycles += 18;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x334));
                    c.g[A3][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6c30));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x14));
                    c.g[A3][0] = 0x460000;
                    c.g[T2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6be8));
                    c.sd(c.g[T0][0] as u32, c.g[T2][0]);
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x334));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x20));
                    c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00126fe4;
                    continue;
                }
                0x00126fe4 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00126fe4) {
                            return Ok(());
                        }
                        pc = 0x00126fe4;
                        continue;
                    }
                    pc = 0x00127010;
                    continue;
                }
                0x00127010 => {
                    c.cycles += 27;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[S3][0] = 0x460000;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x334));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6b38));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6be8));
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.g[T1][0] = c.g[A0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6af0));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x0012707c;
                    continue;
                }
                0x0012707c => {
                    c.cycles += 11;
                    c.g[T2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[S3][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x0012707c) {
                            return Ok(());
                        }
                        pc = 0x0012707c;
                        continue;
                    }
                    pc = 0x001270a8;
                    continue;
                }
                0x001270a8 => {
                    c.cycles += 18;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x330));
                    c.g[A1][0] = 0x460000;
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6b38));
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A1][0] = c.g[S1][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6af0));
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x330));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw(c.g[T0][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x001270f0;
                    continue;
                }
                0x001270f0 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001270f0) {
                            return Ok(());
                        }
                        pc = 0x001270f0;
                        continue;
                    }
                    pc = 0x0012711c;
                    continue;
                }
                0x0012711c => {
                    c.cycles += 28;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[S3][0] = 0x460000;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x330));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6a40));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6af0));
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.g[T1][0] = c.g[A0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x69f8));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x320));
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x0012718c;
                    continue;
                }
                0x0012718c => {
                    c.cycles += 11;
                    c.g[V0][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x0012718c) {
                            return Ok(());
                        }
                        pc = 0x0012718c;
                        continue;
                    }
                    pc = 0x001271b8;
                    continue;
                }
                0x001271b8 => {
                    c.cycles += 18;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x320));
                    c.g[A3][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6a40));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A3][0] = 0x460000;
                    c.g[T2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x69f8));
                    c.sd(c.g[T0][0] as u32, c.g[T2][0]);
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x320));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb0));
                    c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00127200;
                    continue;
                }
                0x00127200 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00127200) {
                            return Ok(());
                        }
                        pc = 0x00127200;
                        continue;
                    }
                    pc = 0x0012722c;
                    continue;
                }
                0x0012722c => {
                    c.cycles += 28;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[S3][0] = 0x460000;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x320));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6758));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x69f8));
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xa0));
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.g[T1][0] = c.g[A0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6710));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x314));
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x50));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x50));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x0012729c;
                    continue;
                }
                0x0012729c => {
                    c.cycles += 11;
                    c.g[V0][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x0012729c) {
                            return Ok(());
                        }
                        pc = 0x0012729c;
                        continue;
                    }
                    pc = 0x001272c8;
                    continue;
                }
                0x001272c8 => {
                    c.cycles += 18;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x314));
                    c.g[A3][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6758));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x50));
                    c.g[A3][0] = 0x460000;
                    c.g[T2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x6710));
                    c.sd(c.g[T0][0] as u32, c.g[T2][0]);
                    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x314));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x50));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x60));
                    c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00127310;
                    continue;
                }
                0x00127310 => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00127310) {
                            return Ok(());
                        }
                        pc = 0x00127310;
                        continue;
                    }
                    pc = 0x0012733c;
                    continue;
                }
                0x0012733c => {
                    c.cycles += 27;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[S3][0] = 0x460000;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x314));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6948));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6710));
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x50));
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.g[T1][0] = c.g[A0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6900));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x001273a8;
                    continue;
                }
                0x001273a8 => {
                    c.cycles += 11;
                    c.g[T2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[S3][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001273a8) {
                            return Ok(());
                        }
                        pc = 0x001273a8;
                        continue;
                    }
                    pc = 0x001273d4;
                    continue;
                }
                0x001273d4 => {
                    c.cycles += 18;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310));
                    c.g[A1][0] = 0x460000;
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6948));
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A1][0] = c.g[S1][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6900));
                    c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x0012741c;
                    continue;
                }
                0x0012741c => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T2][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[T0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x0012741c) {
                            return Ok(());
                        }
                        pc = 0x0012741c;
                        continue;
                    }
                    pc = 0x00127448;
                    continue;
                }
                0x00127448 => {
                    c.cycles += 32;
                    c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                    c.g[T2][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x304));
                    c.g[A2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x6900));
                    c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x310));
                    c.g[A1][0] = 0u64;
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[V0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[RA][0] = sx(0x00127484);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    call_ptr(c, 0x00136ad0, 0x00127484)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[A0][0] = 0x460000;
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = 0x460000;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6660));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[V0][0]);
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
                    c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6618));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A0][0]);
                    c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A1][0]);
                    pc = 0x001274c8;
                    continue;
                }
                0x001274c8 => {
                    c.cycles += 11;
                    c.g[T2][0] = c.ld(c.g[A3][0] as u32);
                    c.g[S3][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A2][0] as u32, c.g[T2][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x20));
                    let t = c.g[A3][0] != c.g[T0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001274c8) {
                            return Ok(());
                        }
                        pc = 0x001274c8;
                        continue;
                    }
                    pc = 0x001274f4;
                    continue;
                }
                0x001274f4 => {
                    c.cycles += 18;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
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
                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x0012753c;
                    continue;
                }
                0x0012753c => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[A3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x0012753c) {
                            return Ok(());
                        }
                        pc = 0x0012753c;
                        continue;
                    }
                    pc = 0x00127568;
                    continue;
                }
                0x00127568 => {
                    c.cycles += 27;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.g[S3][0] = 0x460000;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6568));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6618));
                    c.g[V1][0] = 0x460000;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6470));
                    c.g[T0][0] = 0x460000;
                    c.g[T1][0] = c.g[A0][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[A3][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6520));
                    c.g[T0][0] = c.g[SP][0];
                    c.g[T3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x40));
                    c.g[V0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), c.g[A2][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[A3][0]);
                    pc = 0x001275d4;
                    continue;
                }
                0x001275d4 => {
                    c.cycles += 11;
                    c.g[T2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[S3][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.ld((c.g[T1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[T0][0] as u32, c.g[T2][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[S3][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x20));
                    let t = c.g[T1][0] != c.g[T3][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x001275d4) {
                            return Ok(());
                        }
                        pc = 0x001275d4;
                        continue;
                    }
                    pc = 0x00127600;
                    continue;
                }
                0x00127600 => {
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = 0x460000;
                    c.g[A2][0] = c.ld(c.g[T1][0] as u32);
                    c.g[V1][0] = 0x460000;
                    c.sd(c.g[T0][0] as u32, c.g[A2][0]);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x6568));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6520));
                    c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x20));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    pc = 0x00127640;
                    continue;
                }
                0x00127640 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T2][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[S3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[A0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[S3][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let t = c.g[A2][0] != c.g[T0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if t {
                        if back_edge_unwinds(c, 0x00127640) {
                            return Ok(());
                        }
                        pc = 0x00127640;
                        continue;
                    }
                    pc = 0x0012766c;
                    continue;
                }
                0x0012766c => {
                    c.cycles += 13;
                    c.g[V0][0] = c.ld(c.g[A2][0] as u32);
                    c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sd(c.g[A1][0] as u32, c.g[V0][0]);
                    c.g[T1][0] = 0x460000;
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x6520));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x6c0), c.g[S1][0]);
                    c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[T2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x354));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    let t = c.g[T2][0] == 0u64;
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                    if t {
                        pc = 0x001276ac;
                        continue;
                    }
                    pc = 0x001276a0;
                    continue;
                }
                0x001276a0 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                    c.g[RA][0] = sx(0x001276ac);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x0011b978,
                        f: ee::s00::Rider_dtor,
                    }
                    .call(c, 0x001276ac)?;
                    pc = 0x001276ac;
                    continue;
                }
                0x001276ac => {
                    c.cycles += 3;
                    c.g[S3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x350));

                    if c.g[S3][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x440));
                        pc = 0x001276c4;
                        continue;
                    }
                    pc = 0x001276b8;
                    continue;
                }
                0x001276b8 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001276c0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00317e50, 0x001276c0)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x440));
                    pc = 0x001276c4;
                    continue;
                }
                0x001276c4 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x430));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x420));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x410));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x400));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x3f0));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x3e0));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x3d0));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x3c0));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3b0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x450));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00125eb8, 0x001276f0);
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
    fn player_dtor_matches() {
        check(
            ee::s00::f_00125eb8__generated,
            player_dtor,
            200,
            &|c, seed| {
                machine(c, seed, 0x5eb8, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.pc = 0x00125eb8;
            },
        );
    }
}
