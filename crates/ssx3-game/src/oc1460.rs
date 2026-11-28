//! Staging module oc1460 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1460`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0037_a610,
    ps2_graphics_man_v99,
    "PS2GraphicsMan_v99",
)];

/// `cPS2GraphicsMan::DrawCharModel(mdl, base, shadow)` (`PS2GraphicsMan_v99`, 0x0037a610): draws a
/// character model's chunks into the VIF/GIF packet ring; the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ps2_graphics_man_v99(c: &mut Cpu) {
    if c.pc != 0x0037a610 {
        return ee::s12::f_0037a610__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x0037a610;
    loop {
        match pc {
            0x0037a610 => {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x120));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1420));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.g[S1][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[RA][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A1][0]);
                let t = c.g[V0][0] != 0u64; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[A3][0]);
                if t { pc = 0x0037b514; continue; }
                pc = 0x0037a654; continue;
            }
            0x0037a654 => {
                c.cycles += 24;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 12);
                c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(2, (c.g[V1][0] as u32));
                c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x1b));
                c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x14));
                c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x23));
                c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x1c));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x24), c.g[A1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10d8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
                c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x3e0));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x3e4));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0037a6a8);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0037a6a8)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6b90));
                let t = c.g[V0][0] != 0u64; 
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                if t { pc = 0x0037a6c0; continue; }
                pc = 0x0037a6b4; continue;
            }
            0x0037a6b4 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0037a6bc);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x0037a6bc)?;
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                pc = 0x0037a6c0; continue;
            }
            0x0037a6c0 => {
                c.cycles += 19;
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x20));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x13e8));
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[T0][0]);
                c.g[V0][0] = c.lh((c.g[S4][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10b0));
                c.sh((c.g[A1][0] as u32).wrapping_add(0x12), c.g[V1][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                c.g[V0][0] = c.g[V0][0] | 0x30;
                c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                c.lq(3, (c.g[T0][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6b90));
                let t = c.g[V0][0] != 0u64; 
                c.sq(3, (c.g[SP][0] as u32));
                if t { pc = 0x0037a714; continue; }
                pc = 0x0037a70c; continue;
            }
            0x0037a70c => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0037a714);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x0037a714)?;
                pc = 0x0037a714; continue;
            }
            0x0037a714 => {
                c.cycles += 23;
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58a0));
                c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x5920));
                c.lqc2(10, (c.g[SP][0] as u32));
                c.lqc2(1, (c.g[S1][0] as u32));
                c.lqc2(2, (c.g[S1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 1, Src::Bc(10, X));
                c.vu0().madda(0b1111, 2, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 3, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 11, 4, Src::Bc(10, W));
                c.vu0().mul(0b0001, 11, 11, Src::Bc(5, W));
                c.vu0().max(0b0001, 11, 11, Src::Bc(0, X));
                c.vu0().mini(0b0001, 11, 11, Src::Bc(5, X));
                c.vu0().ftoi(0, 0b1111, 11, 11);
                c.vu0().mtir(1, 11, 3);
                c.cfc2(2, 1);
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 1025i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
                if t { pc = 0x0037a7dc; continue; }
                pc = 0x0037a770; continue;
            }
            0x0037a770 => {
                c.cycles += 4;
                c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6b90));
                let t = c.g[V1][0] != 0u64; 
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                if t { pc = 0x0037a788; continue; }
                pc = 0x0037a780; continue;
            }
            0x0037a780 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x0037a788);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x0037a788)?;
                pc = 0x0037a788; continue;
            }
            0x0037a788 => {
                c.cycles += 21;
                c.lqc2(5, (c.g[S0][0] as u32).wrapping_add(0x5920));
                c.lqc2(10, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(1, (c.g[S1][0] as u32));
                c.lqc2(2, (c.g[S1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 1, Src::Bc(10, X));
                c.vu0().madda(0b1111, 2, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 3, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 11, 4, Src::Bc(10, W));
                c.vu0().mul(0b0001, 11, 11, Src::Bc(5, W));
                c.vu0().max(0b0001, 11, 11, Src::Bc(0, X));
                c.vu0().mini(0b0001, 11, 11, Src::Bc(5, X));
                c.vu0().ftoi(0, 0b1111, 11, 11);
                c.vu0().mtir(1, 11, 3);
                c.cfc2(2, 1);
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                let t = true; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                if t { pc = 0x0037a7e4; continue; }
                pc = 0x0037a7dc; continue;
            }
            0x0037a7dc => {
                c.cycles += 2;
                c.g[A0][0] = 0x400;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A0][0]);
                pc = 0x0037a7e4; continue;
            }
            0x0037a7e4 => {
                c.cycles += 3;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                let t = c.g[A1][0] == 0u64; 
                c.g[A2][0] = 0xffffffffffffff83;
                if t { pc = 0x0037a8f4; continue; }
                pc = 0x0037a7f0; continue;
            }
            0x0037a7f0 => {
                c.cycles += 14;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V1][0] = 0xfffffffff00f0000;
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.sw((c.g[A1][0] as u32), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | 0x1c;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1440));
                let t = c.g[V1][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                if t { pc = 0x0037a83c; continue; }
                pc = 0x0037a828; continue;
            }
            0x0037a828 => {
                c.cycles += 5;
                c.g[V1][0] = 0xfffffffffffff07f;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                let t = true; 
                c.g[V0][0] = c.g[V0][0] | 0x880;
                if t { pc = 0x0037a84c; continue; }
                pc = 0x0037a83c; continue;
            }
            0x0037a83c => {
                c.cycles += 4;
                c.g[V1][0] = 0xfffffffffffff07f;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | 0x180;
                pc = 0x0037a84c; continue;
            }
            0x0037a84c => {
                c.cycles += 42;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[A1][0] = 0xfffffffffffffffc;
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x38));
                c.g[V1][0] = 0x1fff0000;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[T0][0] = 0xffffffffe0000000;
                c.g[V1][0] = c.g[V1][0] | 0xfc00;
                c.g[A0][0] = sx((c.g[A3][0] as u32) << 10);
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[T0][0] = c.g[T0][0] | 0x3ff;
                c.g[V0][0] = c.g[V0][0] | 0x2;
                c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = 0xfffffffffe7f0000;
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.g[T2][0] = 0x800000;
                c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[A1][0] = 0xffffffffffcf0000;
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                c.g[A2][0] = 0xfffffffffff00000;
                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = 0x10000;
                c.g[A2][0] = c.g[A2][0] | 0xfff;
                c.g[A3][0] = c.g[A3][0] | 0x4000;
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                c.sw((c.g[T1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                pc = 0x0037a8f4; continue;
            }
            0x0037a8f4 => {
                c.cycles += 4;
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = c.lh((c.g[T0][0] as u32).wrapping_add(0x48));
                let t = (c.g[V0][0] as i64) <= 0; 
                c.g[S1][0] = 0u64;
                if t { pc = 0x0037b4e4; continue; }
                pc = 0x0037a904; continue;
            }
            0x0037a904 => {
                c.cycles += 17;
                c.g[V0][0] = 0x0;
                c.g[V1][0] = 0x0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x80));
                c.g[A0][0] = 0x14000000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x70));
                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32) >> 3);
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
                c.g[A1][0] = 0x1fff0000;
                c.g[V0][0] = c.g[V0][0] >> 32;
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[A1][0] = c.g[A1][0] | 0xfc00;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[A1][0]);
                pc = 0x0037a948; continue;
            }
            0x0037a948 => {
                c.cycles += 10;
                c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = 0x3;
                c.g[RA][0] = sx(0x0037a958);
                c.g[A0][0] = c.g[S0][0];
                Callee { addr: 0x0037cbc8, f: ee::s12::PS2GraphicsMan_SetTexFunction }.call(c, 0x0037a958)?;
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
                
                if c.g[A2][0] != 0u64 { c.g[V1][0] = c.lw((c.g[S4][0] as u32)); pc = 0x0037ab68; continue; }
                pc = 0x0037a970; continue;
            }
            0x0037a970 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32));
                c.g[V0][0] = c.g[V0][0] & 0x8000;
                let t = c.g[V0][0] == 0u64; 
                c.g[V1][0] = 0xffffffffffffff83;
                if t { pc = 0x0037aa78; continue; }
                pc = 0x0037a980; continue;
            }
            0x0037a980 => {
                c.cycles += 62;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x38));
                c.g[A0][0] = 0xfffffffff00f0000;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[A0][0] | 0xffff;
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
                c.g[T6][0] = 0xfffffffffffffffc;
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[T2][0] = 0xffffffffe0000000;
                c.g[V0][0] = c.g[V0][0] | 0x14;
                c.g[T3][0] = sx((c.g[A3][0] as u32) << 10);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[T2][0] = c.g[T2][0] | 0x3ff;
                c.g[T3][0] = c.g[T3][0] & c.g[T0][0];
                c.g[A3][0] = 0xfffffffffe7f0000;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = c.g[A3][0] | 0xffff;
                c.g[V1][0] = c.lbu((c.g[S4][0] as u32).wrapping_add(0xa));
                c.g[T5][0] = 0x800000;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                c.g[T1][0] = 0xffffffffffcf0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 20);
                c.g[T1][0] = c.g[T1][0] | 0xffff;
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                c.g[T4][0] = 0x300000;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.g[A0][0] = 0xfffffffffffff07f;
                c.sw((c.g[A1][0] as u32), c.g[V0][0]);
                c.g[T0][0] = 0xfffffffffff00000;
                c.g[T0][0] = c.g[T0][0] | 0xfff;
                c.g[A2][0] = 0x50000;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[A2][0] = c.g[A2][0] | 0xc000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                c.g[V0][0] = c.g[V0][0] | 0x600;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T6][0];
                c.g[V0][0] = c.g[V0][0] | 0x2;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T5][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                let t = true; 
                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                if t { pc = 0x0037ab60; continue; }
                pc = 0x0037aa78; continue;
            }
            0x0037aa78 => {
                c.cycles += 15;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[A0][0] = 0xffffffffffffff83;
                c.g[V1][0] = 0xfffffffff00f0000;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.g[V1][0] | 0xffff;
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                c.g[V0][0] = c.g[V0][0] | 0x4;
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1440));
                let t = c.g[V1][0] != 0u64; 
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                if t { pc = 0x0037aac8; continue; }
                pc = 0x0037aab4; continue;
            }
            0x0037aab4 => {
                c.cycles += 5;
                c.g[A1][0] = 0xfffffffffffff07f;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                let t = true; 
                c.g[V0][0] = c.g[V0][0] | 0x880;
                if t { pc = 0x0037aad8; continue; }
                pc = 0x0037aac8; continue;
            }
            0x0037aac8 => {
                c.cycles += 4;
                c.g[A2][0] = 0xfffffffffffff07f;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | 0x180;
                pc = 0x0037aad8; continue;
            }
            0x0037aad8 => {
                c.cycles += 34;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = 0xfffffffffffffffc;
                c.g[A0][0] = 0xffffffffe0000000;
                c.g[A2][0] = 0xfffffffffe7f0000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[A0][0] | 0x3ff;
                c.g[A2][0] = c.g[A2][0] | 0xffff;
                c.g[T1][0] = 0x800000;
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[A1][0] = 0xffffffffffcf0000;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                c.g[A3][0] = 0xfffffffffff00000;
                c.g[T0][0] = 0x10000;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = c.g[A3][0] | 0xfff;
                c.g[T0][0] = c.g[T0][0] | 0x4000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
                pc = 0x0037ab60; continue;
            }
            0x0037ab60 => {
                c.cycles += 2;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S4][0] as u32));
                pc = 0x0037ab68; continue;
            }
            0x0037ab68 => {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 0x1;
                let t = c.g[V0][0] == 0u64; 
                c.g[T0][0] = 0x30000000;
                if t { pc = 0x0037ad78; continue; }
                pc = 0x0037ab74; continue;
            }
            0x0037ab74 => {
                c.cycles += 17;
                c.g[T7][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a00));
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = c.g[T7][0] | c.g[T0][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x7c), c.g[S1][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                c.g[A2][0] = sx((c.g[A3][0] as u32) << 3);
                c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                c.g[T4][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x3c));
                c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x14));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                let t = c.g[A1][0] == c.g[T4][0]; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                if t { pc = 0x0037acd0; continue; }
                pc = 0x0037abb8; continue;
            }
            0x0037abb8 => {
                c.cycles += 14;
                c.g[V1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[T1][0] = 0x6c000000;
                c.g[A0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x48));
                c.g[T6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S6][0] = 0x10000000;
                c.g[S5][0] = 0x8000;
                c.g[S5][0] = c.g[S5][0] << 45;
                c.g[T1][0] = c.g[T1][0] | 0x11;
                c.g[S3][0] = 0x1000000;
                c.g[S3][0] = c.g[S3][0] | 0x101;
                c.g[S2][0] = 0x60010000;
                c.g[S2][0] = c.g[S2][0] | 0x3ff;
                c.g[T5][0] = c.g[V1][0] | c.g[A0][0];
                c.g[S1][0] = 0x50000000;
                pc = 0x0037abf0; continue;
            }
            0x0037abf0 => {
                c.cycles += 18;
                c.g[A3][0] = c.lw((c.g[T0][0] as u32));
                c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[T3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 18);
                c.g[V1][0] = sx((c.g[A3][0] as u32) << 2);
                c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.g[V1][0] | c.g[S6][0];
                c.g[V0][0] = c.g[V0][0] | c.g[S3][0];
                c.sd((c.g[A2][0] as u32), c.g[V1][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), 0u64);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[S5][0]);
                let t = (c.g[A3][0] as i64) <= 0; 
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if t { pc = 0x0037ac80; continue; }
                pc = 0x0037ac38; continue;
            }
            0x0037ac38 => {
                c.cycles += 2;
                c.g[V1][0] = c.g[A3][0];
                pc = 0x0037ac40; continue;
            }
            0x0037ac40 => {
                c.cycles += 16;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                c.lqc2(1, (c.g[V0][0] as u32));
                c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[A2][0] as u32));
                c.sqc2(2, (c.g[A2][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[A2][0] as u32).wrapping_add(0x30));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                let t = c.g[V1][0] != 0u64; 
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                if t { if back_edge_unwinds(c, 0x0037ac40) { return Ok(()); } pc = 0x0037ac40; continue; }
                pc = 0x0037ac80; continue;
            }
            0x0037ac80 => {
                c.cycles += 20;
                c.g[V0][0] = c.g[A3][0] << 32;
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T5][0]);
                c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
                c.g[T0][0] = c.g[T2][0];
                c.sd((c.g[A2][0] as u32), c.g[V0][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                c.sw((c.g[T6][0] as u32), c.g[A2][0]);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.g[T3][0];
                c.g[V0][0] = c.lwu((c.g[V1][0] as u32));
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = c.g[V0][0] | c.g[S1][0];
                c.sd((c.g[A0][0] as u32), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                let t = c.g[A1][0] != c.g[T4][0]; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                if t { if back_edge_unwinds(c, 0x0037abf0) { return Ok(()); } pc = 0x0037abf0; continue; }
                pc = 0x0037acd0; continue;
            }
            0x0037acd0 => {
                c.cycles += 11;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = 0xfff0000;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18f0));
                c.g[T1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
                c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[A1][0] = 0x60000;
                if t { pc = 0x0037b4b0; continue; }
                pc = 0x0037acfc; continue;
            }
            0x0037acfc => {
                c.cycles += 31;
                c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[A1][0] | 0x8000;
                c.g[V1][0] = 0xffffffffffffffe0;
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                c.g[V1][0] = 0x30000000;
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
                c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.g[V1][0] = c.lw((c.g[A3][0] as u32));
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.ldr(5, (c.g[T0][0] as u32));
                c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0xf));
                c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.sdr(5, (c.g[V0][0] as u32));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T7][0]);
                let t = true; 
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                if t { pc = 0x0037b4a8; continue; }
                pc = 0x0037ad78; continue;
            }
            0x0037ad78 => {
                c.cycles += 6;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.g[V1][0] & 0x100;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x7c), c.g[S1][0]);
                let t = c.g[V0][0] == 0u64; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x70), c.g[A0][0]);
                if t { pc = 0x0037b4bc; continue; }
                pc = 0x0037ad90; continue;
            }
            0x0037ad90 => {
                c.cycles += 17;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a00));
                c.g[V0][0] = 0x0;
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                c.g[T0][0] = 0x30000000;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A1][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x398));
                c.g[A1][0] = sx((c.g[V0][0] as u32) >> 3);
                c.g[A0][0] = 0u64;
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S5][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x50));
                c.g[V1][0] = c.g[A3][0] | c.g[T0][0];
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V1][0]);
                c.g[V1][0] = 0u64;
                c.g[T4][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x40));
                let t = (c.g[S5][0] as i64) <= 0; 
                c.g[A2][0] = sx((c.g[V0][0] as u32) << 3);
                if t { pc = 0x0037ae14; continue; }
                pc = 0x0037add4; continue;
            }
            0x0037add4 => {
                c.cycles += 5;
                c.lwc1(1, (c.g[T4][0] as u32));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x276c));
                c.c_lt_s(0, 1);
                let t = true; 
                
                if t { pc = 0x0037ae08; continue; }
                pc = 0x0037ade8; continue;
            }
            0x0037ade8 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                if t { pc = 0x0037ae14; continue; }
                pc = 0x0037adf4; continue;
            }
            0x0037adf4 => {
                c.cycles += 5;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2768));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T4][0] as u32)));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.c_lt_s(1, 0);
                pc = 0x0037ae08; continue;
            }
            0x0037ae08 => {
                c.cycles += 2;
                let t = !c.fcc(); 
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                if t { if back_edge_unwinds(c, 0x0037ade8) { return Ok(()); } pc = 0x0037ade8; continue; }
                pc = 0x0037ae10; continue;
            }
            0x0037ae10 => {
                c.cycles += 1;
                c.g[V1][0] = 0x1;
                pc = 0x0037ae14; continue;
            }
            0x0037ae14 => {
                c.cycles += 2;
                let t = c.g[V1][0] == 0u64; 
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                if t { pc = 0x0037b208; continue; }
                pc = 0x0037ae1c; continue;
            }
            0x0037ae1c => {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x3c));
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[T1][0] = c.g[V1][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                let t = c.g[T1][0] == c.g[V0][0]; 
                c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                if t { pc = 0x0037b400; continue; }
                pc = 0x0037ae40; continue;
            }
            0x0037ae40 => {
                c.cycles += 12;
                c.g[V0][0] = 0x14000000;
                c.g[A0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[A1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x48));
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[A2][0] = 0x46480000;
                c.g[A0][0] = c.g[A0][0] | c.g[A1][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[A0][0]);
                c.g[S6][0] = c.g[A2][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                pc = 0x0037ae70; continue;
            }
            0x0037ae70 => {
                c.cycles += 27;
                c.g[A3][0] = c.lw((c.g[T0][0] as u32));
                c.g[V0][0] = 0x6c000000;
                c.g[V0][0] = c.g[V0][0] | 0x11;
                c.g[A0][0] = 0x10000000;
                c.g[V1][0] = sx((c.g[A3][0] as u32) << 18);
                c.g[A1][0] = 0x1000000;
                c.g[A1][0] = c.g[A1][0] | 0x101;
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.g[A0][0] = c.g[A0][0] | 0x2;
                c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[V0][0] | c.g[A0][0];
                c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
                c.g[V0][0] = 0x8000;
                c.g[V0][0] = c.g[V0][0] << 45;
                c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                c.sd((c.g[A2][0] as u32), c.g[A0][0]);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x8));
                c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x74), c.g[T0][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                c.g[T0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T0][0]);
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), 0u64);
                let t = (c.g[A3][0] as i64) <= 0; 
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                if t { pc = 0x0037af28; continue; }
                pc = 0x0037aedc; continue;
            }
            0x0037aedc => {
                c.cycles += 3;
                c.g[A0][0] = c.g[A1][0];
                c.g[V1][0] = c.g[A3][0];
                pc = 0x0037aee8; continue;
            }
            0x0037aee8 => {
                c.cycles += 16;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.lqc2(1, (c.g[V0][0] as u32));
                c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[A2][0] as u32));
                c.sqc2(2, (c.g[A2][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[A2][0] as u32).wrapping_add(0x30));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                let t = c.g[V1][0] != 0u64; 
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                if t { if back_edge_unwinds(c, 0x0037aee8) { return Ok(()); } pc = 0x0037aee8; continue; }
                pc = 0x0037af28; continue;
            }
            0x0037af28 => {
                c.cycles += 16;
                c.g[V1][0] = c.g[A3][0] << 32;
                c.g[V0][0] = 0x60010000;
                c.g[V0][0] = c.g[V0][0] | 0x3ff;
                c.g[A3][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                c.sd((c.g[A2][0] as u32), c.g[V1][0]);
                c.g[T3][0] = 0u64;
                c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
                c.sw((c.g[T0][0] as u32), c.g[A2][0]);
                c.g[T7][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = c.lw((c.g[T1][0] as u32));
                c.g[S1][0] = c.g[T7][0] ^ 0x1;
                let t = (c.g[T7][0] as i64) <= 0; 
                c.g[S1][0] = c.g[S1][0] & 0x1;
                if t { pc = 0x0037b1f0; continue; }
                pc = 0x0037af68; continue;
            }
            0x0037af68 => {
                c.cycles += 8;
                c.g[V1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x68));
                c.g[V0][0] = 0x11000000;
                c.mtc1(0, 2);
                c.g[FP][0] = 0x50000000;
                c.g[S3][0] = c.g[V1][0] | c.g[V0][0];
                c.g[S7][0] = 0x50000000;
                c.g[S7][0] = c.g[S7][0] | 0x1;
                c.g[S2][0] = c.g[S6][0] << 32;
                pc = 0x0037af88; continue;
            }
            0x0037af88 => {
                c.cycles += 24;
                c.g[V0][0] = c.lwu((c.g[A2][0] as u32));
                c.g[A1][0] = 0x64010000;
                c.g[A1][0] = c.g[A1][0] | 0x80b4;
                c.g[A1][0] = c.g[A1][0] << 32;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[T2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] | c.g[FP][0];
                c.sd((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[A0][0] = c.g[T2][0] << 32;
                c.g[A0][0] = c.g[A0][0] | c.g[S7][0];
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V1][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                c.lwc1(0, (c.g[T4][0] as u32));
                c.c_lt_s(0, 2);
                let t = !c.fcc(); 
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                if t { pc = 0x0037aff8; continue; }
                pc = 0x0037afe8; continue;
            }
            0x0037afe8 => {
                c.cycles += 4;
                c.g[A0][0] = 0x46480000;
                c.sd((c.g[V0][0] as u32).wrapping_add(0x28), c.g[S2][0]);
                let t = true; 
                c.g[A3][0] = c.g[A0][0];
                if t { pc = 0x0037b010; continue; }
                pc = 0x0037aff8; continue;
            }
            0x0037aff8 => {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[T4][0] as u32));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[A1][0] = 0x46480000;
                c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
                c.sd((c.g[V1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                c.g[A3][0] = c.g[A1][0];
                pc = 0x0037b010; continue;
            }
            0x0037b010 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
                let t = c.g[S1][0] != 0u64; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                if t { pc = 0x0037b030; continue; }
                pc = 0x0037b020; continue;
            }
            0x0037b020 => {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = ((c.g[T3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                
                if c.g[V0][0] == 0u64 { c.g[V0][0] = c.lwu((c.g[A2][0] as u32)); pc = 0x0037b038; continue; }
                pc = 0x0037b030; continue;
            }
            0x0037b030 => {
                c.cycles += 2;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lwu((c.g[A2][0] as u32));
                pc = 0x0037b038; continue;
            }
            0x0037b038 => {
                c.cycles += 25;
                c.g[A1][0] = 0x64010000;
                c.g[A1][0] = c.g[A1][0] | 0x80b4;
                c.g[A1][0] = c.g[A1][0] << 16;
                c.g[A1][0] = c.g[A1][0] | 0x1100;
                c.g[A1][0] = c.g[A1][0] << 16;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[T0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] | c.g[FP][0];
                c.sd((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[A0][0] = c.g[T0][0] << 32;
                c.g[A0][0] = c.g[A0][0] | c.g[S7][0];
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V1][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                c.lwc1(0, (c.g[T4][0] as u32));
                c.c_lt_s(0, 2);
                let t = !c.fcc(); 
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                if t { pc = 0x0037b0a8; continue; }
                pc = 0x0037b09c; continue;
            }
            0x0037b09c => {
                c.cycles += 3;
                c.g[V0][0] = c.g[A3][0] << 32;
                let t = true; 
                c.sd((c.g[V1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                if t { pc = 0x0037b0bc; continue; }
                pc = 0x0037b0a8; continue;
            }
            0x0037b0a8 => {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[T4][0] as u32));
                c.g[V1][0] = c.g[A3][0] << 32;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sd((c.g[A0][0] as u32).wrapping_add(0x28), c.g[V0][0]);
                pc = 0x0037b0bc; continue;
            }
            0x0037b0bc => {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[T1][0] = 0x1;
                c.g[V1][0] = ((c.g[T1][0] as i64) < (c.g[S5][0] as i64)) as u64;
                c.g[T9][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[T8][0] = sx((c.g[T3][0] as u32).wrapping_add(0x2));
                let t = c.g[V1][0] == 0u64; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                if t { pc = 0x0037b1ac; continue; }
                pc = 0x0037b0dc; continue;
            }
            0x0037b0dc => {
                c.cycles += 11;
                c.g[T3][0] = c.g[A3][0] << 32;
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2764));
                c.g[T6][0] = 0x50000000;
                c.g[T6][0] = c.g[T6][0] | 0x1;
                c.g[T5][0] = 0x64010000;
                c.g[T5][0] = c.g[T5][0] | 0x80b4;
                c.g[T5][0] = c.g[T5][0] << 32;
                c.g[A3][0] = c.g[S6][0] << 32;
                c.g[A2][0] = sx((c.g[T4][0] as u32).wrapping_add(0x4));
                c.g[T1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                pc = 0x0037b108; continue;
            }
            0x0037b108 => {
                c.cycles += 11;
                c.g[V0][0] = c.lhu((c.g[T2][0] as u32));
                c.g[V1][0] = c.lhu((c.g[T0][0] as u32));
                c.lwc1(0, (c.g[A2][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                c.c_lt_s(1, 0);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                let t = !c.fcc(); 
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                if t { pc = 0x0037b1a0; continue; }
                pc = 0x0037b134; continue;
            }
            0x0037b134 => {
                c.cycles += 27;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = c.g[T2][0] << 32;
                c.g[V0][0] = c.g[V0][0] | c.g[T6][0];
                c.g[A1][0] = c.g[T0][0] << 32;
                c.sd((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[A1][0] = c.g[A1][0] | c.g[T6][0];
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T5][0]);
                c.g[A0][0] = c.lw((c.g[A2][0] as u32));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[A0][0] | c.g[T3][0];
                c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V0][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V1][0] as u32).wrapping_add(0x28), 0u64);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V0][0] as u32).wrapping_add(0x30), c.g[T5][0]);
                c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V1][0] = c.g[V1][0] | c.g[A3][0];
                c.sd((c.g[A0][0] as u32).wrapping_add(0x38), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                pc = 0x0037b1a0; continue;
            }
            0x0037b1a0 => {
                c.cycles += 3;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1));
                let t = c.g[T1][0] != 0u64; 
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                if t { if back_edge_unwinds(c, 0x0037b108) { return Ok(()); } pc = 0x0037b108; continue; }
                pc = 0x0037b1ac; continue;
            }
            0x0037b1ac => {
                c.cycles += 17;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = 0x10000000;
                c.g[V0][0] = c.g[V0][0] | 0x1;
                c.g[T3][0] = c.g[T8][0];
                c.g[A2][0] = c.g[T9][0];
                c.sd((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[A1][0] = ((c.g[T3][0] as i64) < (c.g[T7][0] as i64)) as u64;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[S3][0]);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[S3][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                let t = c.g[A1][0] != 0u64; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                if t { if back_edge_unwinds(c, 0x0037af88) { return Ok(()); } pc = 0x0037af88; continue; }
                pc = 0x0037b1f0; continue;
            }
            0x0037b1f0 => {
                c.cycles += 4;
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x54));
                let t = c.g[T1][0] != c.g[A2][0]; 
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x74));
                if t { if back_edge_unwinds(c, 0x0037ae70) { return Ok(()); } pc = 0x0037ae70; continue; }
                pc = 0x0037b200; continue;
            }
            0x0037b200 => {
                c.cycles += 2;
                let t = true; 
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                if t { pc = 0x0037b404; continue; }
                pc = 0x0037b208; continue;
            }
            0x0037b208 => {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x3c));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                c.g[T5][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A3][0] = c.g[V1][0];
                let t = c.g[A3][0] == c.g[T5][0]; 
                c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                if t { pc = 0x0037b400; continue; }
                pc = 0x0037b228; continue;
            }
            0x0037b228 => {
                c.cycles += 20;
                c.g[V0][0] = 0x14000000;
                c.g[T2][0] = 0x6c000000;
                c.g[V1][0] = c.g[A1][0] | c.g[V0][0];
                c.g[T7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = c.g[V1][0];
                c.g[S2][0] = c.g[V1][0] << 32;
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x48));
                c.g[S1][0] = c.g[V0][0] >> 32;
                c.g[T8][0] = 0x10000000;
                c.g[V0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S7][0] = 0x8000;
                c.g[S7][0] = c.g[S7][0] << 45;
                c.g[T2][0] = c.g[T2][0] | 0x11;
                c.g[S6][0] = 0x1000000;
                c.g[S6][0] = c.g[S6][0] | 0x101;
                c.g[T6][0] = c.g[V0][0] | c.g[V1][0];
                c.g[S5][0] = 0x60010000;
                c.g[S5][0] = c.g[S5][0] | 0x3ff;
                c.g[S3][0] = 0x50000000;
                pc = 0x0037b278; continue;
            }
            0x0037b278 => {
                c.cycles += 18;
                c.g[A2][0] = c.lw((c.g[T0][0] as u32));
                c.g[T3][0] = sx((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[T4][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[A2][0] as u32) << 18);
                c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.g[V1][0] | c.g[T8][0];
                c.g[V0][0] = c.g[V0][0] | c.g[S6][0];
                c.sd((c.g[A1][0] as u32), c.g[V1][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), 0u64);
                c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[S7][0]);
                let t = (c.g[A2][0] as i64) <= 0; 
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                if t { pc = 0x0037b308; continue; }
                pc = 0x0037b2c0; continue;
            }
            0x0037b2c0 => {
                c.cycles += 2;
                c.g[V1][0] = c.g[A2][0];
                pc = 0x0037b2c8; continue;
            }
            0x0037b2c8 => {
                c.cycles += 16;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                c.lqc2(1, (c.g[V0][0] as u32));
                c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[A1][0] as u32));
                c.sqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                let t = c.g[V1][0] != 0u64; 
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                if t { if back_edge_unwinds(c, 0x0037b2c8) { return Ok(()); } pc = 0x0037b2c8; continue; }
                pc = 0x0037b308; continue;
            }
            0x0037b308 => {
                c.cycles += 22;
                c.g[V0][0] = c.g[A2][0] << 32;
                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[T6][0]);
                c.g[V0][0] = c.g[V0][0] | c.g[S5][0];
                c.sd((c.g[A1][0] as u32), c.g[V0][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                c.sw((c.g[T7][0] as u32), c.g[A1][0]);
                c.g[A1][0] = c.lw((c.g[A3][0] as u32));
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lwu((c.g[A1][0] as u32));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[V0][0] = c.g[V0][0] | c.g[S3][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                c.sd((c.g[A0][0] as u32), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                let t = c.g[A1][0] == c.g[A2][0]; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                if t { pc = 0x0037b3b8; continue; }
                pc = 0x0037b360; continue;
            }
            0x0037b360 => {
                c.cycles += 6;
                c.g[T1][0] = 0x50000000;
                c.g[T1][0] = c.g[T1][0] | 0x1;
                c.g[T0][0] = 0x8800;
                c.g[T0][0] = c.g[T0][0] << 45;
                c.g[A3][0] = c.g[S1][0] << 32;
                pc = 0x0037b378; continue;
            }
            0x0037b378 => {
                c.cycles += 16;
                c.g[V1][0] = c.lwu((c.g[A1][0] as u32));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[V1][0] = c.g[V1][0] | c.g[T1][0];
                c.sd((c.g[A0][0] as u32), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                let t = c.g[A1][0] != c.g[A2][0]; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                if t { if back_edge_unwinds(c, 0x0037b378) { return Ok(()); } pc = 0x0037b378; continue; }
                pc = 0x0037b3b8; continue;
            }
            0x0037b3b8 => {
                c.cycles += 18;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = 0x10000000;
                c.g[V0][0] = c.g[V0][0] | 0x1;
                c.g[A1][0] = 0x8800;
                c.g[A1][0] = c.g[A1][0] << 45;
                c.g[T0][0] = c.g[T3][0];
                c.sd((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[A3][0] = c.g[T4][0];
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[S2][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                let t = c.g[A3][0] != c.g[T5][0]; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
                if t { if back_edge_unwinds(c, 0x0037b278) { return Ok(()); } pc = 0x0037b278; continue; }
                pc = 0x0037b400; continue;
            }
            0x0037b400 => {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
                pc = 0x0037b404; continue;
            }
            0x0037b404 => {
                c.cycles += 10;
                c.g[V0][0] = 0xfff0000;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18f0));
                c.g[T1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
                c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[A1][0] = 0x60000;
                if t { pc = 0x0037b4b0; continue; }
                pc = 0x0037b42c; continue;
            }
            0x0037b42c => {
                c.cycles += 31;
                c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[A1][0] | 0x8000;
                c.g[V1][0] = 0xffffffffffffffe0;
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
                c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                c.g[V1][0] = 0x30000000;
                c.g[V0][0] = c.g[V0][0] & 0x1f;
                c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
                c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
                c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.g[V1][0] = c.lw((c.g[A3][0] as u32));
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.ldr(5, (c.g[T0][0] as u32));
                c.ldl(6, (c.g[T0][0] as u32).wrapping_add(0xf));
                c.ldr(6, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.sdr(5, (c.g[V0][0] as u32));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[A1][0]);
                pc = 0x0037b4a8; continue;
            }
            0x0037b4a8 => {
                c.cycles += 2;
                c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
                pc = 0x0037b4b0; continue;
            }
            0x0037b4b0 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a00));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
                pc = 0x0037b4bc; continue;
            }
            0x0037b4bc => {
                c.cycles += 10;
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x48));
                c.g[S1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x7c));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[S4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A3][0]);
                if t { if back_edge_unwinds(c, 0x0037a948) { return Ok(()); } pc = 0x0037a948; continue; }
                pc = 0x0037b4e4; continue;
            }
            0x0037b4e4 => {
                c.cycles += 12;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[A1][0] = 0xfffffffffffffc3f;
                c.g[A2][0] = 0xffffffffc0000000;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x14));
                c.sw((c.g[S0][0] as u32).wrapping_add(0xe84), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_sub(0x14));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.sw((c.g[A0][0] as u32).wrapping_sub(0x14), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                pc = 0x0037b514; continue;
            }
            0x0037b514 => {
                c.cycles += 12;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x80));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x0037a610, 0x0037b544); return Ok(()); }
        }
    }
    });
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fakes, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes, in the machine's RAM.
    const OC1460_GATE: u32 = DATA + 0x7_f000;
    const OC1460_MAGIC: u32 = 0x1460_00de;

    macro_rules! oc1460_gated_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1460_GATE) != OC1460_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
            }
        };
    }

    oc1460_gated_fake!(
        oc1460_fake_37d968,
        OC1460_PREV_37D968,
        0x0037_d968,
        ee::s12::f_0037d968__generated
    );
    oc1460_gated_fake!(
        oc1460_fake_37cbc8,
        OC1460_PREV_37CBC8,
        0x0037_cbc8,
        ee::s12::f_0037cbc8__generated
    );

    fn oc1460_hook() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0037D968),
            oc1460_fake_37d968,
            &OC1460_PREV_37D968,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0037CBC8),
            oc1460_fake_37cbc8,
            &OC1460_PREV_37CBC8,
        );
    }

    const SELF: u32 = DATA;
    const DRAW_STATE: u32 = DATA + 0x7000;
    const VOBJ: u32 = DATA + 0x7200;
    const RING: u32 = DATA + 0x8000;
    const BONES: u32 = DATA + 0x1_0000;
    const BASE: u32 = DATA + 0x2_0000;
    const MDL: u32 = DATA + 0x2_1000;
    const CHUNKS: u32 = DATA + 0x2_2000;
    const LISTS: u32 = DATA + 0x2_3000;
    const INDICES: u32 = DATA + 0x2_3800;
    const LOD_W: u32 = DATA + 0x2_4000;
    const SUBS: u32 = DATA + 0x2_5000;
    const LOD_DATA: u32 = DATA + 0x2_8000;
    const REFS: u32 = DATA + 0x2_c000;
    const PACKETS: u32 = 0x00a0_0000;

    /// Fixture: a character model (0..3 chunks of 1..3 sub-meshes, 1..3 LODs
    /// with weights around the 0.01 threshold, chunk flags picking each of
    /// the three draw paths), the draw-state stack, the packet ring (full for
    /// some seeds) and the gated callee fakes.
    fn oc1460_setup(c: &mut Cpu, seed: u32) {
        oc1460_hook();
        let mut r = fakes::machine(c, seed, 0x1460, 6);
        c.pc = 0x0037_a610;
        c.bus.write32(OC1460_GATE, OC1460_MAGIC);
        let gp = GAME_GP;
        c.bus
            .write32(gp.wrapping_add(0x1420), (seed % 13 == 0) as u32);
        c.bus.write32(gp.wrapping_add(0x1440), seed & 1);
        for off in [0x276c, 0x2768, 0x2764] {
            c.bus.write32(gp.wrapping_sub(off), 0x3c23_d70a);
        }
        // Self.
        c.bus.write32(SELF + 0xe84, DRAW_STATE);
        c.bus.write32(SELF + 0x10d8, VOBJ);
        c.bus.write16(VOBJ + 0x3e0, (r.next_u32() & 0xfc) as u16);
        c.bus.write32(VOBJ + 0x3e4, fakes::FAKE);
        c.bus.write32(SELF + 0x6b90, (seed >> 1) & 1);
        c.bus.write32(SELF + 0x13e8, BONES);
        c.bus.write32(SELF + 0x5a00, PACKETS);
        c.bus.write32(SELF + 0x18f0, RING);
        for i in 0..4 {
            c.bus.write32(SELF + 0x10b0 + 4 * i, r.next_u32());
        }
        for i in 0..0x30 {
            let v = r.edge_f32();
            c.bus.write32(SELF + 0x58a0 + 4 * i, v);
        }
        for i in 0..0x60 {
            let v = r.edge_f32();
            c.bus.write32(BONES + 4 * i, v);
        }
        for i in 0..8 {
            let v = r.next_u32();
            c.bus.write32(DRAW_STATE + 4 * i, v);
        }
        let count = match (seed >> 2) % 5 {
            0 => 2600,
            n => n - 1,
        };
        c.bus.write32(RING, count);
        c.bus.write32(RING + 0x6_8000 + 0x1cc0, r.next_u32());
        c.bus.write32(RING + 0x6_8000 + 0x1cc4, r.next_u32());
        // Model.
        let chunks = (seed / 5) % 4;
        let lods = 1 + (seed / 3) % 3;
        c.bus.write32(MDL + 0x20, CHUNKS - BASE);
        c.bus.write32(MDL + 0x3c, LISTS);
        c.bus.write32(MDL + 0x40, LOD_W);
        c.bus.write16(MDL + 0x48, chunks as u16);
        c.bus.write16(MDL + 0x50, lods as u16);
        let weights = [0.0f32, -1.0, 0.5, 0.005, 2.0];
        for j in 0..3 {
            let w = weights[(r.next_u32() as usize + (seed / 7) as usize * (j + 1)) % 5];
            c.bus.write32(LOD_W + 4 * j as u32, w.to_bits());
        }
        if (seed / 7) % 3 == 0 {
            c.bus.write32(LOD_W, 0.5f32.to_bits());
        }
        for k in 0..10u32 {
            c.bus.write32(LISTS + 8 * k, r.next_u32() % 4);
            c.bus.write32(LISTS + 8 * k + 4, INDICES + 16 * k);
            for i in 0..4 {
                c.bus.write32(INDICES + 16 * k + 4 * i, r.next_u32() % 4);
            }
        }
        let flag_set = [0x8001u32, 1, 0x100, 0x8100, 0, 0x8000, 0x101];
        for i in 0..4u32 {
            let ch = CHUNKS + 0x14 * i;
            let flags = flag_set[((seed / 11 + 3 * i) % 7) as usize];
            c.bus.write32(ch, flags);
            c.bus.write32(ch + 4, r.next_u32());
            c.bus.write16(ch + 8, r.next_u32() as u16 & 3);
            c.bus.write8(ch + 0xa, r.next_u32() as u8);
            let subs = 1 + (seed / 2 + i) % 3;
            c.bus.write32(ch + 0xc, subs);
            let sub_base = SUBS + 0x100 * i;
            c.bus.write32(ch + 0x10, sub_base);
            for s in 0..3u32 {
                let refs = REFS + (i * 3 + s) * 0x80;
                c.bus.write32(sub_base + 8 * s, refs);
                c.bus.write32(sub_base + 8 * s + 4, 1 + (seed / 5 + i + s) % 4);
                for q in 0..8u32 {
                    c.bus.write32(refs + 8 * q, r.next_u32());
                    c.bus
                        .write32(refs + 8 * q + 4, LOD_DATA + 0x80 * (r.next_u32() % 30));
                }
            }
        }
        for q in 0..(0x2000 / 32) {
            c.bus.write16(LOD_DATA + 32 * q, 1);
        }
        if seed % 4 == 0 {
            c.next_event = (seed as u64 % 8) * 400;
        }
        // Arguments.
        scramble_args(c, &mut r, seed);
    }

    fn scramble_args(c: &mut Cpu, _r: &mut Rng, seed: u32) {
        c.g[A0][0] = sx(SELF);
        c.g[A1][0] = sx(MDL);
        c.g[A2][0] = sx(BASE);
        c.g[A3][0] = ((seed / 3) % 2) as u64;
    }

    /// `PS2GraphicsMan_v99` against the generated body: every draw path
    /// (shadow or not; flags 0x8000/1/0x100; weights found or not), the
    /// callees (view-matrix update, SetTexFunction, the vtable call) through
    /// gated fakes with unwinds.
    #[test]
    fn ps2_graphics_man_v99_matches() {
        fakes::check_reaching(
            ee::s12::f_0037a610__generated,
            ps2_graphics_man_v99,
            240,
            &[0x0037_a6a8, 0x0037_a958],
            &oc1460_setup,
        );
    }

    /// The fixtures reach many distinct paths of the body.
    #[test]
    fn ps2_graphics_man_v99_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..240 {
            let mut c = crate::difftest::fresh();
            oc1460_setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s12::f_0037a610__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 15, "only {} distinct paths", seen.len());
    }
}
