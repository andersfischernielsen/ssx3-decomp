//! Staging module oc1477 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1477`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_0158, ui_object_set_anim_params, "UIObject_SetAnimParams"),
    (0x003a_0274, ui_object_anim_loop_tail, "UIObject_AnimLoopTail"),
    (0x003a_3808, ui_vector_set_anim_params, "UIVector_SetAnimParams"),
    (0x0037_e238, ps2_graphics_man_v95, "PS2GraphicsMan_DrawModel"),
];

/// `UIObject_SetAnimParams(this, count, shorts)` (0x003a0158): for each of `count`
/// (index, value) short pairs at `shorts`, dispatch through the jump table at
/// 0x494030 on the index (0x11 and up: next pair via `UIObject_AnimLoopTail`).
/// Every resume entry of the generated body is a `match pc` arm.
pub fn ui_object_set_anim_params(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003a0158 => {
                c.cycles += 3;
                c.g[A3][0] = c.g[A0][0];
                let t = c.g[A1][0] == 0u64; 
                c.g[T0][0] = 0u64;
                if t { c.pc = 0x003a0284; call_indirect(c, 0x003a0284); return Ok(()); }
                pc = 0x003a0164; continue;
            }
            0x003a0164 => {
                c.cycles += 1;
                pc = 0x003a0168; continue;
            }
            0x003a0168 => {
                c.cycles += 7;
                c.g[V1][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x2));
                c.g[A0][0] = c.lh((c.g[A2][0] as u32));
                c.mtc1(3, 1);
                c.cvt_s_w(1, 1);
                c.g[V0][0] = (c.g[A0][0] < 0x11u64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x490000;
                if t { c.pc = 0x003a0274; ee::s13::f_003a0274(c); return Ok(()); }
                pc = 0x003a0184; continue;
            }
            0x003a0184 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4030));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                let t = c.g[A0][0] as u32;
                
                pc = t; continue;
            }
            _ => { c.jump_out(pc, 0x003a0158, 0x003a019c); return Ok(()); }
        }
    }
    });
}

/// `UIObject_AnimLoopTail` (0x003a0274): the shared loop tail of the `setChannelDataKCP`
/// style loops: `t0 += 1`, `a2 += 4`, again from 0x3a0168 while `t0 < count`, else return.
pub fn ui_object_anim_loop_tail(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003a0274 => {
                c.cycles += 4;
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = (c.g[T0][0] < c.g[A1][0]) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                if t { c.pc = 0x003a0168; call_indirect(c, 0x003a0168); return Ok(()); }
                pc = 0x003a0284; continue;
            }
            0x003a0284 => {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003a0274, 0x003a028c); return Ok(()); }
        }
    }
    });
}

/// `UIVector_SetAnimParams(this, count, shorts)` (0x003a3808): like the `UIObject` one with
/// a divide by 9 of the index and a bound from `this+0x74`; jump table at 0x494140.
pub fn ui_vector_set_anim_params(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x003a3808 => {
                c.cycles += 4;
                c.g[T0][0] = c.g[A0][0];
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x80));
                let t = c.g[V0][0] == 0u64; 
                
                if t { c.pc = 0x003a39f0; call_indirect(c, 0x003a39f0); return Ok(()); }
                pc = 0x003a3818; continue;
            }
            0x003a3818 => {
                c.cycles += 2;
                let t = c.g[A1][0] == 0u64; 
                c.g[T2][0] = 0u64;
                if t { c.pc = 0x003a39f0; call_indirect(c, 0x003a39f0); return Ok(()); }
                pc = 0x003a3820; continue;
            }
            0x003a3820 => {
                c.cycles += 7;
                c.g[V1][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x2));
                c.g[A0][0] = c.lh((c.g[A2][0] as u32));
                c.mtc1(3, 1);
                c.cvt_s_w(1, 1);
                c.g[V0][0] = ((c.g[A0][0] as i64) < 21i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = (c.g[A0][0] < 0x11u64) as u64;
                if t { c.pc = 0x003a3918; call_indirect(c, 0x003a3918); return Ok(()); }
                pc = 0x003a383c; continue;
            }
            0x003a383c => {
                c.cycles += 5;
                c.g[V0][0] = 0x9;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x15));
                c.div(0, 4, 2);
                
                if c.g[V0][0] == 0u64 {  pc = 0x003a3850; continue; }
                pc = 0x003a3850; continue;
            }
            0x003a3850 => {
                c.cycles += 8;
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x74));
                c.g[V1][0] = sx((c.g[V1][0] as u32) >> 17);
                c.g[A0][0] = c.lo[0];
                c.g[T1][0] = c.hi[0];
                c.g[A3][0] = c.g[A0][0] & 0xffff;
                c.g[V1][0] = (c.g[A3][0] < c.g[V1][0]) as u64;
                let t = c.g[V1][0] == 0u64; 
                c.g[V1][0] = (c.g[T1][0] < 0x9u64) as u64;
                if t { c.pc = 0x003a39e0; ee::s13::f_003a39e0(c); return Ok(()); }
                pc = 0x003a3870; continue;
            }
            0x003a3870 => {
                c.cycles += 2;
                
                if c.g[V0][0] == 0u64 {  pc = 0x003a3878; continue; }
                pc = 0x003a3878; continue;
            }
            0x003a3878 => {
                c.cycles += 2;
                let t = c.g[V1][0] == 0u64; 
                c.g[V0][0] = 0x490000;
                if t { c.pc = 0x003a39e0; ee::s13::f_003a39e0(c); return Ok(()); }
                pc = 0x003a3880; continue;
            }
            0x003a3880 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[T1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4140));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                let t = c.g[A0][0] as u32;
                
                pc = t; continue;
            }
            _ => { c.jump_out(pc, 0x003a3808, 0x003a3898); return Ok(()); }
        }
    }
    });
}

/// `PS2GraphicsMan_v95` / DrawModel (0x0037e238), `(f12 alpha, gm, inst, texTable, world,
/// nodeMatrices, matrix2, flags)`: the generated body (every register, cycle and call
/// kept) as a native `match pc`; every resume entry is an arm.
pub fn ps2_graphics_man_v95(c: &mut Cpu) {
    guest(c, |c| {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x0037e238 => {
                c.cycles += 23;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x290));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1400));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x260));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x250));
                c.g[S1][0] = c.g[A0][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x220));
                c.g[S2][0] = c.g[A1][0];
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.g[S5][0] = c.g[A2][0];
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.g[S6][0] = c.g[T0][0];
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x288));
                c.g[S7][0] = c.g[A3][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x270));
                c.mov_s(21, 12);
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x240));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x230));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x1e0), c.g[RA][0]);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x280));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x148), c.g[T1][0]);
                let t = c.g[V0][0] != 0u64; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[T2][0]);
                if t { pc = 0x00380348; continue; }
                pc = 0x0037e294; continue;
            }
            0x0037e294 => {
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_eq_s(21, 0);
                let t = c.fcc(); 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x270));
                if t { pc = 0x0038034c; continue; }
                pc = 0x0037e2a8; continue;
            }
            0x0037e2a8 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6b90));
                let t = c.g[V0][0] != 0u64; 
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                if t { pc = 0x0037e2c0; continue; }
                pc = 0x0037e2b4; continue;
            }
            0x0037e2b4 => {
                c.cycles += 3;
                c.g[RA][0] = sx(0x0037e2bc);
                Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x0037e2bc)?;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                pc = 0x0037e2c0; continue;
            }
            0x0037e2c0 => {
                c.cycles += 3;
                c.g[V0][0] = c.g[V1][0] & 0x400;
                
                if c.g[V0][0] != 0u64 { c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8)); pc = 0x0037e3b8; continue; }
                pc = 0x0037e2cc; continue;
            }
            0x0037e2cc => {
                c.cycles += 29;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(5, (c.g[S2][0] as u32).wrapping_add(0x60));
                c.g[A1][0] = c.g[SP][0];
                c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x64));
                c.g[A2][0] = c.g[S0][0];
                c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x68));
                c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x58a0));
                c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x6c));
                c.lwc1(3, (c.g[S2][0] as u32).wrapping_add(0x70));
                c.lwc1(4, (c.g[S2][0] as u32).wrapping_add(0x74));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2e8));
                c.swc1(5, (c.g[SP][0] as u32));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2ec));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0037e330);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0037e330)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = 0x1;
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[V0][0] = 0x2;
                if t { pc = 0x00380348; continue; }
                pc = 0x0037e340; continue;
            }
            0x0037e340 => {
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                if t { pc = 0x0037e3a8; continue; }
                pc = 0x0037e348; continue;
            }
            0x0037e348 => {
                c.cycles += 24;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                c.g[A2][0] = c.g[S0][0];
                c.lwc1(5, (c.g[S2][0] as u32).wrapping_add(0x60));
                c.g[A1][0] = c.g[SP][0];
                c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x64));
                c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5800));
                c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x68));
                c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x6c));
                c.lwc1(3, (c.g[S2][0] as u32).wrapping_add(0x70));
                c.lwc1(4, (c.g[S2][0] as u32).wrapping_add(0x74));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2e8));
                c.swc1(5, (c.g[SP][0] as u32));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2ec));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0037e3a0);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0037e3a0)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                pc = 0x0037e3a8; continue;
            }
            0x0037e3a8 => {
                c.cycles += 4;
                c.g[V0][0] = c.g[A0][0] | 0x20;
                if c.g[V1][0] != 0 { c.g[A0][0] = c.g[V0][0]; }
                c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                pc = 0x0037e3b8; continue;
            }
            0x0037e3b8 => {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] & 0x4000;
                let t = c.g[V0][0] == 0u64; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x154), 0u64);
                if t { pc = 0x0037e3dc; continue; }
                pc = 0x0037e3c4; continue;
            }
            0x0037e3c4 => {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[A1][0] | 0x1;
                c.g[RA][0] = sx(0x0037e3d8);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x14c), c.g[A1][0]);
                call_ptr(c, 0x002f5148, 0x0037e3d8)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x154), c.g[V0][0]);
                pc = 0x0037e3dc; continue;
            }
            0x0037e3dc => {
                c.cycles += 11;
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a94));
                c.g[A3][0] = c.g[A3][0] & 0x1;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x158), c.g[A2][0]);
                c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1b8), c.g[A3][0]);
                c.g[S3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x80));
                c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x80));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x15c), c.g[T1][0]);
                let t = c.g[A3][0] == 0u64; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x160), c.g[T0][0]);
                if t { pc = 0x0037e414; continue; }
                pc = 0x0037e408; continue;
            }
            0x0037e408 => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x24));
                let t = true; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x164), c.g[V0][0]);
                if t { pc = 0x0037e41c; continue; }
                pc = 0x0037e414; continue;
            }
            0x0037e414 => {
                c.cycles += 2;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x98));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x164), c.g[V1][0]);
                pc = 0x0037e41c; continue;
            }
            0x0037e41c => {
                c.cycles += 49;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[S4][0] = c.g[S3][0];
                c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x600));
                c.g[V1][0] = 0xfffffffffffffc3f;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4));
                c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x170), c.g[A2][0]);
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x168), c.g[S5][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x174), c.g[A3][0]);
                c.g[V0][0] = c.g[V0][0] | 0xc0;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x16c), c.g[S6][0]);
                c.g[A1][0] = 0xffffffffc0000000;
                c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                c.g[S3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x80));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x84));
                c.lwc1(2, (c.g[T0][0] as u32).wrapping_add(0x18));
                c.lwc1(3, (c.g[T0][0] as u32).wrapping_add(0x1c));
                c.lwc1(1, (c.g[T0][0] as u32).wrapping_add(0x20));
                c.mul_s(2, 2, 0);
                c.mul_s(3, 3, 0);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                c.mul_s(1, 1, 0);
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x15c));
                c.swc1(2, (c.g[SP][0] as u32));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.lq(2, (c.g[SP][0] as u32));
                c.sq(2, (c.g[T1][0] as u32));
                c.lqc2(1, (c.g[S7][0] as u32));
                c.lqc2(2, (c.g[S7][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[S7][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[S7][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[A3][0] as u32));
                c.sqc2(2, (c.g[A3][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[A3][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[A3][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lhu((c.g[S2][0] as u32).wrapping_add(0x92));
                c.g[V0][0] = c.g[V0][0] & 0x1;
                let t = c.g[V0][0] == 0u64; 
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x174));
                if t { pc = 0x0037e4f4; continue; }
                pc = 0x0037e4e0; continue;
            }
            0x0037e4e0 => {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x2f0));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2f4));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0037e4f4);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0037e4f4)?;
                pc = 0x0037e4f4; continue;
            }
            0x0037e4f4 => {
                c.cycles += 16;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x178), c.g[S3][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x170));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x178));
                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc0));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x174));
                c.lqc2(1, (c.g[A0][0] as u32));
                c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[V1][0] as u32));
                c.sqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                
                if c.g[A1][0] != 0u64 { c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8)); pc = 0x0037e828; continue; }
                pc = 0x0037e534; continue;
            }
            0x0037e534 => {
                c.cycles += 40;
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x178));
                c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x15c));
                c.lqc2(1, (c.g[T0][0] as u32));
                c.lqc2(4, (c.g[T1][0] as u32));
                c.lqc2(5, (c.g[T1][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[T1][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[T1][0] as u32).wrapping_add(0x30));
                c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
                c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
                c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
                c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
                c.sqc2(8, (c.g[V0][0] as u32));
                c.sqc2(9, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.sqc2(10, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.sqc2(11, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.lqc2(1, (c.g[V0][0] as u32));
                c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[V1][0] as u32));
                c.sqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                c.lqc2(1, (c.g[V1][0] as u32));
                c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[V0][0] as u32));
                c.sqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x500000;
                if t { pc = 0x0037e5e4; continue; }
                pc = 0x0037e5d4; continue;
            }
            0x0037e5d4 => {
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1b8));
                if t { pc = 0x0037e7f8; continue; }
                pc = 0x0037e5e4; continue;
            }
            0x0037e5e4 => {
                c.cycles += 10;
                c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x84));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x16c), c.g[S4][0]);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[SP][0] as u32));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                let t = (c.g[V0][0] as i64) <= 0; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x0037e824; continue; }
                pc = 0x0037e60c; continue;
            }
            0x0037e60c => {
                c.cycles += 5;
                c.g[T3][0] = 0xffffffffffffffff;
                c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                c.g[A2][0] = c.g[S4][0];
                pc = 0x0037e620; continue;
            }
            0x0037e620 => {
                c.cycles += 3;
                c.g[A1][0] = c.lw((c.g[A3][0] as u32));
                let t = c.g[A1][0] == c.g[T3][0]; 
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 6);
                if t { pc = 0x0037e760; continue; }
                pc = 0x0037e62c; continue;
            }
            0x0037e62c => {
                c.cycles += 77;
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                c.lwc1(7, (c.g[SP][0] as u32));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                c.lq(2, (c.g[A0][0] as u32));
                c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0xc));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xf4));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0xf8));
                c.mul_s(1, 1, 7);
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xfc));
                c.mul_s(2, 2, 6);
                c.mul_s(3, 3, 5);
                c.mul_s(0, 0, 4);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xe4));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xe8));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xec));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.lqc2(4, (c.g[A1][0] as u32));
                c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[A1][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[A1][0] as u32).wrapping_add(0x30));
                c.lqc2(8, (c.g[T2][0] as u32));
                c.lqc2(9, (c.g[T2][0] as u32).wrapping_add(0x10));
                c.lqc2(10, (c.g[T2][0] as u32).wrapping_add(0x20));
                c.lqc2(11, (c.g[T2][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                c.sqc2(12, (c.g[T0][0] as u32));
                c.sqc2(13, (c.g[T0][0] as u32).wrapping_add(0x10));
                c.sqc2(14, (c.g[T0][0] as u32).wrapping_add(0x20));
                c.sqc2(15, (c.g[T0][0] as u32).wrapping_add(0x30));
                c.lqc2(1, (c.g[T0][0] as u32));
                c.lqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[T1][0] as u32));
                c.sqc2(2, (c.g[T1][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[T1][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[T1][0] as u32).wrapping_add(0x30));
                c.lqc2(1, (c.g[T1][0] as u32));
                c.lqc2(2, (c.g[T1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[T1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[T1][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[A2][0] as u32));
                c.sqc2(2, (c.g[A2][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[A2][0] as u32).wrapping_add(0x30));
                let t = true; 
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                if t { pc = 0x0037e7d8; continue; }
                pc = 0x0037e760; continue;
            }
            0x0037e760 => {
                c.cycles += 30;
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                c.lqc2(1, (c.g[V0][0] as u32));
                c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[A2][0] as u32));
                c.sqc2(2, (c.g[A2][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[A2][0] as u32).wrapping_add(0x30));
                c.lq(3, (c.g[A2][0] as u32).wrapping_add(0x30));
                c.lwc1(0, (c.g[SP][0] as u32));
                c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x24));
                c.mul_s(1, 1, 0);
                c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.mul_s(3, 3, 2);
                c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xc));
                c.mul_s(4, 4, 5);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.mul_s(0, 0, 2);
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x18));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(2, (c.g[A2][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                pc = 0x0037e7d8; continue;
            }
            0x0037e7d8 => {
                c.cycles += 6;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x10));
                if t { if back_edge_unwinds(c, 0x0037e620) { return Ok(()); } pc = 0x0037e620; continue; }
                pc = 0x0037e7f0; continue;
            }
            0x0037e7f0 => {
                c.cycles += 2;
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                if t { pc = 0x0037e828; continue; }
                pc = 0x0037e7f8; continue;
            }
            0x0037e7f8 => {
                c.cycles += 2;
                let t = c.g[A1][0] == 0u64; 
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x170));
                if t { pc = 0x0037e824; continue; }
                pc = 0x0037e800; continue;
            }
            0x0037e800 => {
                c.cycles += 9;
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x174));
                c.lqc2(1, (c.g[A3][0] as u32));
                c.lqc2(2, (c.g[A3][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[A3][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[A3][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[A2][0] as u32));
                c.sqc2(2, (c.g[A2][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[A2][0] as u32).wrapping_add(0x30));
                pc = 0x0037e824; continue;
            }
            0x0037e824 => {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                pc = 0x0037e828; continue;
            }
            0x0037e828 => {
                c.cycles += 23;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x17c), c.g[S3][0]);
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x118));
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x11c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0037e840);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0037e840)?;
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x17c));
                c.lqc2(1, (c.g[V0][0] as u32));
                c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[T0][0] as u32));
                c.sqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                c.g[T1][0] = sx((c.g[T0][0] as u32).wrapping_add(0x80));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x180), c.g[S3][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x184), c.g[T1][0]);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.g[V0][0] & 0x2;
                let t = c.g[V0][0] == 0u64; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x188), 0u64);
                if t { pc = 0x0037eca0; continue; }
                pc = 0x0037e884; continue;
            }
            0x0037e884 => {
                c.cycles += 263;
                c.lwc1(5, (c.g[S2][0] as u32).wrapping_add(0x74));
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.lwc1(4, (c.g[S2][0] as u32).wrapping_add(0x60));
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.lwc1(3, (c.g[S2][0] as u32).wrapping_add(0x64));
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x68));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x6c));
                c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x70));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 7);
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.sw((c.g[SP][0] as u32), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xc));
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x4c));
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x5c));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[AT][0] = 0x3f000000;
                c.mtc1(1, 0);
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.mfc1(7, 0);
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.qmtc2(7, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[T0][0] as u32));
                c.lqc2(5, (c.g[T0][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[T0][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[T0][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[SP][0] as u32));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x40), 0u64);
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), 0u64);
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x5c), 0u64);
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
                c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x38));
                c.neg_s(6, 3);
                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.mul_s(4, 3, 0);
                c.neg_s(5, 0);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.mul_s(3, 3, 1);
                c.mul_s(0, 2, 0);
                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.mul_s(1, 2, 1);
                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x64));
                c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x74));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x78));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x80), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x54), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x84), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x88), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x5c), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x6c), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x7c), 0u64);
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x8c));
                c.lqc2(4, (c.g[A2][0] as u32));
                c.lqc2(5, (c.g[A2][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[A2][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[A2][0] as u32).wrapping_add(0x30));
                c.lqc2(8, (c.g[T0][0] as u32));
                c.lqc2(9, (c.g[T0][0] as u32).wrapping_add(0x10));
                c.lqc2(10, (c.g[T0][0] as u32).wrapping_add(0x20));
                c.lqc2(11, (c.g[T0][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                c.sqc2(12, (c.g[V1][0] as u32));
                c.sqc2(13, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.sqc2(14, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.sqc2(15, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.lqc2(1, (c.g[V1][0] as u32));
                c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[A1][0] as u32));
                c.sqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
                c.lqc2(1, (c.g[A1][0] as u32));
                c.lqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[T0][0] as u32));
                c.sqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x178));
                c.lqc2(4, (c.g[T0][0] as u32));
                c.lqc2(5, (c.g[T0][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[T0][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[T0][0] as u32).wrapping_add(0x30));
                c.lqc2(8, (c.g[A1][0] as u32));
                c.lqc2(9, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.lqc2(10, (c.g[A1][0] as u32).wrapping_add(0x20));
                c.lqc2(11, (c.g[A1][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                c.sqc2(12, (c.g[A0][0] as u32));
                c.sqc2(13, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.sqc2(14, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.sqc2(15, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.lqc2(1, (c.g[A0][0] as u32));
                c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[SP][0] as u32));
                c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x180));
                c.lqc2(1, (c.g[SP][0] as u32));
                c.lqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[A2][0] as u32));
                c.sqc2(2, (c.g[A2][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[A2][0] as u32).wrapping_add(0x30));
                c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xc));
                c.g[V1][0] = 0x500000;
                c.sw((c.g[SP][0] as u32), 0u64);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4760));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                c.lq(2, (c.g[SP][0] as u32));
                c.sq(2, (c.g[T0][0] as u32).wrapping_add(0x70));
                c.lqc2(4, (c.g[V1][0] as u32));
                c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.lqc2(8, (c.g[A2][0] as u32));
                c.lqc2(9, (c.g[A2][0] as u32).wrapping_add(0x10));
                c.lqc2(10, (c.g[A2][0] as u32).wrapping_add(0x20));
                c.lqc2(11, (c.g[A2][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                c.sqc2(12, (c.g[A0][0] as u32));
                c.sqc2(13, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.sqc2(14, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.sqc2(15, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.lqc2(1, (c.g[A0][0] as u32));
                c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[SP][0] as u32));
                c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lqc2(1, (c.g[SP][0] as u32));
                c.lqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[T1][0] as u32));
                c.sqc2(2, (c.g[T1][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[T1][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[T1][0] as u32).wrapping_add(0x30));
                c.g[A3][0] = 0x100;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x188), c.g[A3][0]);
                pc = 0x0037eca0; continue;
            }
            0x0037eca0 => {
                c.cycles += 5;
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.g[V0][0] & 0x8;
                let t = c.g[V0][0] == 0u64; 
                c.g[A2][0] = 0xfffffffffffffc1f;
                if t { pc = 0x0037ed1c; continue; }
                pc = 0x0037ecb4; continue;
            }
            0x0037ecb4 => {
                c.cycles += 26;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A1][0] = 0xffffffffffbf0000;
                c.g[A3][0] = 0x400000;
                c.ldl(9, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.ldr(9, (c.g[V0][0] as u32));
                c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                c.sdl(9, (c.g[V0][0] as u32).wrapping_add(0x1b));
                c.sdr(9, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x23));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x1c));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[A0][0]);
                c.g[A1][0] = c.g[A1][0] | 0xffff;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                c.sw((c.g[S1][0] as u32).wrapping_add(0xe84), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | 0xe0;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                pc = 0x0037ed1c; continue;
            }
            0x0037ed1c => {
                c.cycles += 5;
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[S0][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
                let t = (c.g[V0][0] as i64) <= 0; 
                c.g[S2][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8));
                if t { pc = 0x0038032c; continue; }
                pc = 0x0037ed30; continue;
            }
            0x0037ed30 => {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 20);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x190), c.g[V0][0]);
                pc = 0x0037ed40; continue;
            }
            0x0037ed40 => {
                c.cycles += 7;
                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1bc), c.g[A0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x18c), c.g[V1][0]);
                let t = c.g[V1][0] == 0u64; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c0), c.g[A1][0]);
                if t { pc = 0x00380310; continue; }
                pc = 0x0037ed5c; continue;
            }
            0x0037ed5c => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                let t = c.g[V0][0] == 0u64; 
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                if t { pc = 0x00380310; continue; }
                pc = 0x0037ed68; continue;
            }
            0x0037ed68 => {
                c.cycles += 2;
                let t = c.g[A2][0] == 0u64; 
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 6);
                if t { pc = 0x0037f07c; continue; }
                pc = 0x0037ed70; continue;
            }
            0x0037ed70 => {
                c.cycles += 53;
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x174));
                c.lqc2(4, (c.g[A3][0] as u32));
                c.lqc2(5, (c.g[A3][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[A3][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[A3][0] as u32).wrapping_add(0x30));
                c.lqc2(8, (c.g[V0][0] as u32));
                c.lqc2(9, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.lqc2(10, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.lqc2(11, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                c.sqc2(12, (c.g[V1][0] as u32));
                c.sqc2(13, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.sqc2(14, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.sqc2(15, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.lqc2(1, (c.g[V1][0] as u32));
                c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[SP][0] as u32));
                c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x178));
                c.lqc2(1, (c.g[SP][0] as u32));
                c.lqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[T0][0] as u32));
                c.sqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.g[V0][0] & 0x2;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x17c));
                if t { pc = 0x0037efcc; continue; }
                pc = 0x0037ee44; continue;
            }
            0x0037ee44 => {
                c.cycles += 98;
                c.lqc2(4, (c.g[V0][0] as u32));
                c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.lqc2(8, (c.g[T0][0] as u32));
                c.lqc2(9, (c.g[T0][0] as u32).wrapping_add(0x10));
                c.lqc2(10, (c.g[T0][0] as u32).wrapping_add(0x20));
                c.lqc2(11, (c.g[T0][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                c.sqc2(12, (c.g[V1][0] as u32));
                c.sqc2(13, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.sqc2(14, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.sqc2(15, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.lqc2(1, (c.g[V1][0] as u32));
                c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[SP][0] as u32));
                c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x180));
                c.lqc2(1, (c.g[SP][0] as u32));
                c.lqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[A0][0] as u32));
                c.sqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.sw((c.g[SP][0] as u32), 0u64);
                c.g[A1][0] = 0x500000;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4760));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
                c.lq(2, (c.g[SP][0] as u32));
                c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.lqc2(4, (c.g[A1][0] as u32));
                c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[A1][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[A1][0] as u32).wrapping_add(0x30));
                c.lqc2(8, (c.g[A0][0] as u32));
                c.lqc2(9, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.lqc2(10, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.lqc2(11, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                c.sqc2(12, (c.g[V1][0] as u32));
                c.sqc2(13, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.sqc2(14, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.sqc2(15, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.lqc2(1, (c.g[V1][0] as u32));
                c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[SP][0] as u32));
                c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x184));
                c.lqc2(1, (c.g[SP][0] as u32));
                c.lqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[A2][0] as u32));
                c.sqc2(2, (c.g[A2][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[A2][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[A2][0] as u32).wrapping_add(0x30));
                pc = 0x0037efcc; continue;
            }
            0x0037efcc => {
                c.cycles += 3;
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1b8));
                let t = c.g[A3][0] == 0u64; 
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x170));
                if t { pc = 0x0037effc; continue; }
                pc = 0x0037efd8; continue;
            }
            0x0037efd8 => {
                c.cycles += 9;
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x178));
                c.lqc2(1, (c.g[T1][0] as u32));
                c.lqc2(2, (c.g[T1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[T1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[T1][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[T0][0] as u32));
                c.sqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                pc = 0x0037effc; continue;
            }
            0x0037effc => {
                c.cycles += 32;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x178));
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x15c));
                c.lqc2(1, (c.g[V0][0] as u32));
                c.lqc2(4, (c.g[A0][0] as u32));
                c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
                c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
                c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
                c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
                c.sqc2(8, (c.g[V1][0] as u32));
                c.sqc2(9, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.sqc2(10, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.sqc2(11, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.lqc2(1, (c.g[V1][0] as u32));
                c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[SP][0] as u32));
                c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                c.lqc2(1, (c.g[SP][0] as u32));
                c.lqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[A1][0] as u32));
                c.sqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
                pc = 0x0037f07c; continue;
            }
            0x0037f07c => {
                c.cycles += 5;
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18c));
                if t { pc = 0x0037f1b8; continue; }
                pc = 0x0037f090; continue;
            }
            0x0037f090 => {
                c.cycles += 5;
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                c.g[V0][0] = 0x1020;
                c.g[V1][0] = c.g[A3][0] & 0x1020;
                let t = c.g[V1][0] != c.g[V0][0]; 
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18c));
                if t { pc = 0x0037f1b8; continue; }
                pc = 0x0037f0a4; continue;
            }
            0x0037f0a4 => {
                c.cycles += 69;
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x58a0));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x190));
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x178));
                c.lqc2(4, (c.g[V0][0] as u32));
                c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.lqc2(8, (c.g[T1][0] as u32));
                c.lqc2(9, (c.g[T1][0] as u32).wrapping_add(0x10));
                c.lqc2(10, (c.g[T1][0] as u32).wrapping_add(0x20));
                c.lqc2(11, (c.g[T1][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.vu0().mula(0b1111, 4, Src::Bc(9, X));
                c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
                c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
                c.vu0().mula(0b1111, 4, Src::Bc(10, X));
                c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
                c.vu0().mula(0b1111, 4, Src::Bc(11, X));
                c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
                c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
                c.sqc2(12, (c.g[T0][0] as u32));
                c.sqc2(13, (c.g[T0][0] as u32).wrapping_add(0x10));
                c.sqc2(14, (c.g[T0][0] as u32).wrapping_add(0x20));
                c.sqc2(15, (c.g[T0][0] as u32).wrapping_add(0x30));
                c.lqc2(1, (c.g[T0][0] as u32));
                c.lqc2(2, (c.g[T0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[T0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[T0][0] as u32).wrapping_add(0x30));
                c.sqc2(1, (c.g[SP][0] as u32));
                c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18c));
                c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10d8));
                c.g[T1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                c.lwc1(0, (c.g[V0][0] as u32));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2e8));
                c.g[A3][0] = c.g[SP][0];
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1bc), c.g[T0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c0), c.g[T1][0]);
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x190));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4c));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x5c));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x58));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2ec));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0037f1ac);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0037f1ac)?;
                c.g[V1][0] = 0x1;
                let t = c.g[V0][0] == c.g[V1][0]; 
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18c));
                if t { pc = 0x00380310; continue; }
                pc = 0x0037f1b8; continue;
            }
            0x0037f1b8 => {
                c.cycles += 7;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1bc), c.g[S0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1c0), c.g[S2][0]);
                let t = (c.g[V0][0] as i64) <= 0; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x150), 0u64);
                if t { pc = 0x00380310; continue; }
                pc = 0x0037f1d4; continue;
            }
            0x0037f1d4 => {
                c.cycles += 27;
                c.g[A0][0] = 0x1fff0000;
                c.g[A1][0] = 0xffffffffe0000000;
                c.g[A0][0] = c.g[A0][0] | 0xfc00;
                c.g[A2][0] = 0xfffffffffe7f0000;
                c.g[A3][0] = 0xffffffffffcf0000;
                c.g[T0][0] = 0x10000;
                c.g[T1][0] = 0x60000;
                c.g[V0][0] = 0xf0000;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x198), c.g[A0][0]);
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x58a0));
                c.g[A1][0] = c.g[A1][0] | 0x3ff;
                c.g[A2][0] = c.g[A2][0] | 0xffff;
                c.g[A3][0] = c.g[A3][0] | 0xffff;
                c.g[T0][0] = c.g[T0][0] | 0x4000;
                c.g[T1][0] = c.g[T1][0] | 0x8000;
                c.g[V0][0] = c.g[V0][0] | 0xf000;
                c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x144));
                c.g[FP][0] = 0xfffffffffff00000;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x194), c.g[V1][0]);
                c.g[FP][0] = c.g[FP][0] | 0xfff;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x19c), c.g[A1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1a0), c.g[A2][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1a4), c.g[A3][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1a8), c.g[T0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1b0), c.g[T1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1b4), c.g[V0][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1ac), c.g[A0][0]);
                pc = 0x0037f240; continue;
            }
            0x0037f240 => {
                c.cycles += 15;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18c));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[S4][0] = c.lw((c.g[V0][0] as u32));
                c.g[V1][0] = c.lh((c.g[S4][0] as u32));
                c.g[V0][0] = c.lhu((c.g[S4][0] as u32).wrapping_add(0x2));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[V0][0] = c.g[V0][0] & 0x8;
                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                let t = c.g[V0][0] == 0u64; 
                c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                if t { pc = 0x0037f284; continue; }
                pc = 0x0037f27c; continue;
            }
            0x0037f27c => {
                c.cycles += 2;
                c.g[V0][0] = 0x40000;
                c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
                pc = 0x0037f284; continue;
            }
            0x0037f284 => {
                c.cycles += 11;
                c.g[V1][0] = c.lh((c.g[A0][0] as u32));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x148));
                c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.g[A0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.sh((c.g[V0][0] as u32).wrapping_add(0x12), c.g[A0][0]);
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1404));
                let t = c.g[V1][0] == 0u64; 
                c.g[T4][0] = 0u64;
                if t { pc = 0x0037f2bc; continue; }
                pc = 0x0037f2b0; continue;
            }
            0x0037f2b0 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffffffffff9f0000;
                c.g[V0][0] = c.g[V0][0] | 0xffff;
                c.g[A2][0] = c.g[A2][0] & c.g[V0][0];
                pc = 0x0037f2bc; continue;
            }
            0x0037f2bc => {
                c.cycles += 6;
                c.g[A0][0] = sx(((c.g[S0][0] as u32) as i32 >> 11) as u32);
                c.g[A1][0] = 0x180000;
                c.g[V1][0] = c.g[A2][0] & c.g[A1][0];
                c.g[V0][0] = 0x100000;
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[A0][0] = c.g[A0][0] & 0x1;
                if t { pc = 0x0037f318; continue; }
                pc = 0x0037f2d4; continue;
            }
            0x0037f2d4 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = 0x80000;
                if t { pc = 0x0037f2f0; continue; }
                pc = 0x0037f2e0; continue;
            }
            0x0037f2e0 => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                if t { pc = 0x0037f300; continue; }
                pc = 0x0037f2e8; continue;
            }
            0x0037f2e8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A3][0] = 0xfffffffffffffff3;
                if t { pc = 0x0037f34c; continue; }
                pc = 0x0037f2f0; continue;
            }
            0x0037f2f0 => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[A1][0]; 
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                if t { pc = 0x0037f334; continue; }
                pc = 0x0037f2f8; continue;
            }
            0x0037f2f8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A3][0] = 0xfffffffffffffff3;
                if t { pc = 0x0037f34c; continue; }
                pc = 0x0037f300; continue;
            }
            0x0037f300 => {
                c.cycles += 6;
                c.g[T0][0] = 0xfffffffffffffff3;
                c.g[A0][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                let t = true; 
                c.g[V0][0] = c.g[V0][0] | 0x8;
                if t { pc = 0x0037f354; continue; }
                pc = 0x0037f318; continue;
            }
            0x0037f318 => {
                c.cycles += 7;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T1][0] = 0xfffffffffffffff3;
                c.g[A0][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                let t = true; 
                c.g[V0][0] = c.g[V0][0] | 0x4;
                if t { pc = 0x0037f354; continue; }
                pc = 0x0037f334; continue;
            }
            0x0037f334 => {
                c.cycles += 6;
                c.g[A1][0] = 0xfffffffffffffff3;
                c.g[A0][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                let t = true; 
                c.g[V0][0] = c.g[V0][0] | 0xc;
                if t { pc = 0x0037f354; continue; }
                pc = 0x0037f34c; continue;
            }
            0x0037f34c => {
                c.cycles += 2;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                pc = 0x0037f354; continue;
            }
            0x0037f354 => {
                c.cycles += 6;
                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.g[V0][0] & 0x8;
                let t = c.g[V0][0] == 0u64; 
                c.g[T1][0] = 0xffffffffffffff83;
                if t { pc = 0x0037f494; continue; }
                pc = 0x0037f36c; continue;
            }
            0x0037f36c => {
                c.cycles += 16;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | 0x1c;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = 0xfffffffffffff07f;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | 0x180;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6b90));
                c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x30));
                let t = c.g[V0][0] != 0u64; 
                c.sq(3, (c.g[SP][0] as u32));
                if t { pc = 0x0037f3bc; continue; }
                pc = 0x0037f3ac; continue;
            }
            0x0037f3ac => {
                c.cycles += 4;
                c.sq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.g[RA][0] = sx(0x0037f3b8);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x0037f3b8)?;
                c.lq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                pc = 0x0037f3bc; continue;
            }
            0x0037f3bc => {
                c.cycles += 54;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x194));
                c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x5920));
                c.lqc2(10, (c.g[SP][0] as u32));
                c.lqc2(1, (c.g[A1][0] as u32));
                c.lqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 1, Src::Bc(10, X));
                c.vu0().madda(0b1111, 2, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 3, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 11, 4, Src::Bc(10, W));
                c.vu0().mul(0b0001, 11, 11, Src::Bc(5, W));
                c.vu0().max(0b0001, 11, 11, Src::Bc(0, X));
                c.vu0().mini(0b0001, 11, 11, Src::Bc(5, X));
                c.vu0().ftoi(0, 0b1111, 11, 11);
                c.vu0().mtir(1, 11, 3);
                c.cfc2(3, 1);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = 0xfffffffffffffffc;
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x198));
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 10);
                c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[A2][0] = 0x300000;
                c.g[V0][0] = c.g[V0][0] | 0x2;
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x19c));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = 0x800000;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a4));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                let t = true; 
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                if t { pc = 0x0037fd2c; continue; }
                pc = 0x0037f494; continue;
            }
            0x0037f494 => {
                c.cycles += 5;
                c.g[A1][0] = 0x660000;
                c.g[V0][0] = 0x200000;
                c.g[V1][0] = c.g[A2][0] & c.g[A1][0];
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if t { pc = 0x0037f7e4; continue; }
                pc = 0x0037f4a8; continue;
            }
            0x0037f4a8 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = 0x600000;
                if t { pc = 0x0037f4ec; continue; }
                pc = 0x0037f4b0; continue;
            }
            0x0037f4b0 => {
                c.cycles += 3;
                c.g[V0][0] = 0x20000;
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if t { pc = 0x0037f5ec; continue; }
                pc = 0x0037f4bc; continue;
            }
            0x0037f4bc => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = 0x40000;
                if t { pc = 0x0037f4d4; continue; }
                pc = 0x0037f4c4; continue;
            }
            0x0037f4c4 => {
                c.cycles += 2;
                let t = c.g[V1][0] == 0u64; 
                
                if t { pc = 0x0037f530; continue; }
                pc = 0x0037f4cc; continue;
            }
            0x0037f4cc => {
                c.cycles += 2;
                let t = true; 
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                if t { pc = 0x0037fbf8; continue; }
                pc = 0x0037f4d4; continue;
            }
            0x0037f4d4 => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[V0][0] = 0x60000;
                if t { pc = 0x0037f6b8; continue; }
                pc = 0x0037f4dc; continue;
            }
            0x0037f4dc => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                if t { pc = 0x0037f6bc; continue; }
                pc = 0x0037f4e4; continue;
            }
            0x0037f4e4 => {
                c.cycles += 2;
                let t = true; 
                c.g[T1][0] = 0xffffffffffffff83;
                if t { pc = 0x0037fbfc; continue; }
                pc = 0x0037f4ec; continue;
            }
            0x0037f4ec => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if t { pc = 0x0037f8b8; continue; }
                pc = 0x0037f4f4; continue;
            }
            0x0037f4f4 => {
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64; 
                c.g[V0][0] = 0x620000;
                if t { pc = 0x0037f518; continue; }
                pc = 0x0037f4fc; continue;
            }
            0x0037f4fc => {
                c.cycles += 3;
                c.g[V0][0] = 0x220000;
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[V0][0] = 0x260000;
                if t { pc = 0x0037fabc; continue; }
                pc = 0x0037f508; continue;
            }
            0x0037f508 => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[V0][0]; 
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                if t { pc = 0x0037fac0; continue; }
                pc = 0x0037f510; continue;
            }
            0x0037f510 => {
                c.cycles += 2;
                let t = true; 
                c.g[T1][0] = 0xffffffffffffff83;
                if t { pc = 0x0037fbfc; continue; }
                pc = 0x0037f518; continue;
            }
            0x0037f518 => {
                c.cycles += 2;
                
                if c.g[V1][0] == c.g[V0][0] { c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84)); pc = 0x0037f984; continue; }
                pc = 0x0037f520; continue;
            }
            0x0037f520 => {
                c.cycles += 2;
                let t = c.g[V1][0] == c.g[A1][0]; 
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                if t { pc = 0x0037f984; continue; }
                pc = 0x0037f528; continue;
            }
            0x0037f528 => {
                c.cycles += 2;
                let t = true; 
                c.g[T1][0] = 0xffffffffffffff83;
                if t { pc = 0x0037fbfc; continue; }
                pc = 0x0037f530; continue;
            }
            0x0037f530 => {
                c.cycles += 2;
                let t = c.g[A0][0] == 0u64; 
                c.g[T0][0] = 0x50000;
                if t { pc = 0x0037f548; continue; }
                pc = 0x0037f538; continue;
            }
            0x0037f538 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                let t = true; 
                c.g[T4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7488));
                if t { pc = 0x0037fd30; continue; }
                pc = 0x0037f548; continue;
            }
            0x0037f548 => {
                c.cycles += 41;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T1][0] = 0xffffffffffffff83;
                c.g[A3][0] = 0x800000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | 0x4;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = 0xfffffffffffff07f;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | 0x180;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = 0xfffffffffffffffc;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x19c));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a4));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                let t = true; 
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                if t { pc = 0x0037fd30; continue; }
                pc = 0x0037f5ec; continue;
            }
            0x0037f5ec => {
                c.cycles += 2;
                let t = c.g[A0][0] == 0u64; 
                c.g[V1][0] = 0x50000;
                if t { pc = 0x0037f604; continue; }
                pc = 0x0037f5f4; continue;
            }
            0x0037f5f4 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                let t = true; 
                c.g[T4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7490));
                if t { pc = 0x0037fd30; continue; }
                pc = 0x0037f604; continue;
            }
            0x0037f604 => {
                c.cycles += 45;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A2][0] = 0xffffffffffffff83;
                c.g[A3][0] = 0xfffffffffffff07f;
                c.g[T0][0] = 0xfffffffffffffffc;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = 0x50000;
                c.g[A1][0] = c.g[A1][0] | 0xc000;
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                c.g[A2][0] = 0x800000;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | 0x180;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.g[V0][0] = c.g[V0][0] | 0x1;
                c.g[T0][0] = 0x300000;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x19c));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a4));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                let t = true; 
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                if t { pc = 0x0037fd30; continue; }
                pc = 0x0037f6b8; continue;
            }
            0x0037f6b8 => {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                pc = 0x0037f6bc; continue;
            }
            0x0037f6bc => {
                c.cycles += 16;
                c.g[T1][0] = 0xffffffffffffff83;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = 0xfffffffffffff07f;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | 0x180;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6b90));
                c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x30));
                let t = c.g[V0][0] != 0u64; 
                c.sq(3, (c.g[SP][0] as u32));
                if t { pc = 0x0037f70c; continue; }
                pc = 0x0037f6fc; continue;
            }
            0x0037f6fc => {
                c.cycles += 4;
                c.sq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.g[RA][0] = sx(0x0037f708);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x0037f708)?;
                c.lq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                pc = 0x0037f70c; continue;
            }
            0x0037f70c => {
                c.cycles += 54;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x194));
                c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x5920));
                c.lqc2(10, (c.g[SP][0] as u32));
                c.lqc2(1, (c.g[A1][0] as u32));
                c.lqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 1, Src::Bc(10, X));
                c.vu0().madda(0b1111, 2, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 3, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 11, 4, Src::Bc(10, W));
                c.vu0().mul(0b0001, 11, 11, Src::Bc(5, W));
                c.vu0().max(0b0001, 11, 11, Src::Bc(0, X));
                c.vu0().mini(0b0001, 11, 11, Src::Bc(5, X));
                c.vu0().ftoi(0, 0b1111, 11, 11);
                c.vu0().mtir(1, 11, 3);
                c.cfc2(3, 1);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = 0xfffffffffffffffc;
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x198));
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 10);
                c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[A2][0] = 0x300000;
                c.g[V0][0] = c.g[V0][0] | 0x2;
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x19c));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = 0x800000;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a4));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                let t = true; 
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                if t { pc = 0x0037fd2c; continue; }
                pc = 0x0037f7e4; continue;
            }
            0x0037f7e4 => {
                c.cycles += 2;
                let t = c.g[A0][0] == 0u64; 
                c.g[T0][0] = 0x50000;
                if t { pc = 0x0037f7fc; continue; }
                pc = 0x0037f7ec; continue;
            }
            0x0037f7ec => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                let t = true; 
                c.g[T4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x748c));
                if t { pc = 0x0037f8a4; continue; }
                pc = 0x0037f7fc; continue;
            }
            0x0037f7fc => {
                c.cycles += 42;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T1][0] = 0xffffffffffffff83;
                c.g[A3][0] = 0x800000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | 0x4;
                c.g[T1][0] = 0x300000;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = 0xfffffffffffff07f;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | 0x100;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = 0xfffffffffffffffc;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x19c));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a4));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                pc = 0x0037f8a4; continue;
            }
            0x0037f8a4 => {
                c.cycles += 5;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x188));
                c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x184));
                let t = true; 
                c.g[S0][0] = c.g[A1][0] | c.g[A2][0];
                if t { pc = 0x0037fd30; continue; }
                pc = 0x0037f8b8; continue;
            }
            0x0037f8b8 => {
                c.cycles += 2;
                let t = c.g[A0][0] == 0u64; 
                c.g[A3][0] = 0x50000;
                if t { pc = 0x0037f8d0; continue; }
                pc = 0x0037f8c0; continue;
            }
            0x0037f8c0 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                let t = true; 
                c.g[T4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7488));
                if t { pc = 0x0037f970; continue; }
                pc = 0x0037f8d0; continue;
            }
            0x0037f8d0 => {
                c.cycles += 40;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T0][0] = 0xffffffffffffff83;
                c.g[T1][0] = 0xfffffffffffff07f;
                c.g[A3][0] = 0x800000;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.g[V0][0] = c.g[V0][0] | 0x4;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | 0x880;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = 0xfffffffffffffffc;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x19c));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a4));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                pc = 0x0037f970; continue;
            }
            0x0037f970 => {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x188));
                c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x184));
                let t = true; 
                c.g[S0][0] = c.g[V0][0] | c.g[V1][0];
                if t { pc = 0x0037fd30; continue; }
                pc = 0x0037f984; continue;
            }
            0x0037f984 => {
                c.cycles += 16;
                c.g[A0][0] = 0xffffffffffffff83;
                c.g[A1][0] = 0xfffffffffffff07f;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | 0x880;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6b90));
                c.lq(3, (c.g[A2][0] as u32).wrapping_add(0x30));
                let t = c.g[V0][0] != 0u64; 
                c.sq(3, (c.g[SP][0] as u32));
                if t { pc = 0x0037f9d4; continue; }
                pc = 0x0037f9c4; continue;
            }
            0x0037f9c4 => {
                c.cycles += 4;
                c.sq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.g[RA][0] = sx(0x0037f9d0);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x0037f9d0)?;
                c.lq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                pc = 0x0037f9d4; continue;
            }
            0x0037f9d4 => {
                c.cycles += 58;
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x194));
                c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x5920));
                c.lqc2(10, (c.g[SP][0] as u32));
                c.lqc2(1, (c.g[A3][0] as u32));
                c.lqc2(2, (c.g[A3][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[A3][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[A3][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 1, Src::Bc(10, X));
                c.vu0().madda(0b1111, 2, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 3, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 11, 4, Src::Bc(10, W));
                c.vu0().mul(0b0001, 11, 11, Src::Bc(5, W));
                c.vu0().max(0b0001, 11, 11, Src::Bc(0, X));
                c.vu0().mini(0b0001, 11, 11, Src::Bc(5, X));
                c.vu0().ftoi(0, 0b1111, 11, 11);
                c.vu0().mtir(1, 11, 3);
                c.cfc2(3, 1);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A2][0] = 0xfffffffffffffffc;
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 10);
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x188));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x198));
                c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x184));
                c.g[V0][0] = c.g[V0][0] | 0x2;
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                c.g[S0][0] = c.g[T0][0] | c.g[T1][0];
                c.g[A1][0] = 0x300000;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T1][0] = 0x800000;
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x19c));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a4));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                let t = true; 
                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                if t { pc = 0x0037fd2c; continue; }
                pc = 0x0037fabc; continue;
            }
            0x0037fabc => {
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                pc = 0x0037fac0; continue;
            }
            0x0037fac0 => {
                c.cycles += 16;
                c.g[A3][0] = 0xffffffffffffff83;
                c.g[T0][0] = 0xfffffffffffff07f;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.g[V0][0] = c.g[V0][0] | 0x100;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6b90));
                c.lq(3, (c.g[T1][0] as u32).wrapping_add(0x30));
                let t = c.g[V0][0] != 0u64; 
                c.sq(3, (c.g[SP][0] as u32));
                if t { pc = 0x0037fb10; continue; }
                pc = 0x0037fb00; continue;
            }
            0x0037fb00 => {
                c.cycles += 4;
                c.sq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.g[RA][0] = sx(0x0037fb0c);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x0037fb0c)?;
                c.lq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                pc = 0x0037fb10; continue;
            }
            0x0037fb10 => {
                c.cycles += 58;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x194));
                c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x5920));
                c.lqc2(10, (c.g[SP][0] as u32));
                c.lqc2(1, (c.g[V0][0] as u32));
                c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 1, Src::Bc(10, X));
                c.vu0().madda(0b1111, 2, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 3, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 11, 4, Src::Bc(10, W));
                c.vu0().mul(0b0001, 11, 11, Src::Bc(5, W));
                c.vu0().max(0b0001, 11, 11, Src::Bc(0, X));
                c.vu0().mini(0b0001, 11, 11, Src::Bc(5, X));
                c.vu0().ftoi(0, 0b1111, 11, 11);
                c.vu0().mtir(1, 11, 3);
                c.cfc2(3, 1);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T0][0] = 0xfffffffffffffffc;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 10);
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x188));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x198));
                c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x184));
                c.g[V0][0] = c.g[V0][0] | 0x2;
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[S0][0] = c.g[A0][0] | c.g[A2][0];
                c.g[V1][0] = c.g[V1][0] & c.g[A3][0];
                c.g[A1][0] = 0x800000;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = 0x300000;
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x19c));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a4));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                let t = true; 
                c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
                if t { pc = 0x0037fd2c; continue; }
                pc = 0x0037fbf8; continue;
            }
            0x0037fbf8 => {
                c.cycles += 1;
                c.g[T1][0] = 0xffffffffffffff83;
                pc = 0x0037fbfc; continue;
            }
            0x0037fbfc => {
                c.cycles += 15;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | 0x14;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = 0xfffffffffffff07f;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] | 0x180;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6b90));
                c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x30));
                let t = c.g[V0][0] != 0u64; 
                c.sq(3, (c.g[SP][0] as u32));
                if t { pc = 0x0037fc48; continue; }
                pc = 0x0037fc38; continue;
            }
            0x0037fc38 => {
                c.cycles += 4;
                c.sq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.g[RA][0] = sx(0x0037fc44);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x0037fc44)?;
                c.lq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                pc = 0x0037fc48; continue;
            }
            0x0037fc48 => {
                c.cycles += 57;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x194));
                c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x5920));
                c.lqc2(10, (c.g[SP][0] as u32));
                c.lqc2(1, (c.g[A1][0] as u32));
                c.lqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 1, Src::Bc(10, X));
                c.vu0().madda(0b1111, 2, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 3, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 11, 4, Src::Bc(10, W));
                c.vu0().mul(0b0001, 11, 11, Src::Bc(5, W));
                c.vu0().max(0b0001, 11, 11, Src::Bc(0, X));
                c.vu0().mini(0b0001, 11, 11, Src::Bc(5, X));
                c.vu0().ftoi(0, 0b1111, 11, 11);
                c.vu0().mtir(1, 11, 3);
                c.cfc2(3, 1);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T1][0] = 0xfffffffffffffffc;
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 10);
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x188));
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x198));
                c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x184));
                c.g[V0][0] = c.g[V0][0] | 0x2;
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.g[V1][0] & c.g[T0][0];
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x19c));
                c.g[S0][0] = c.g[A2][0] | c.g[A3][0];
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = 0x800000;
                c.g[T1][0] = 0x300000;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a4));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                pc = 0x0037fd2c; continue;
            }
            0x0037fd2c => {
                c.cycles += 1;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                pc = 0x0037fd30; continue;
            }
            0x0037fd30 => {
                c.cycles += 33;
                c.g[S7][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5a00));
                c.g[S3][0] = 0x30000000;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                c.g[A0][0] = c.g[S1][0];
                c.g[V0][0] = c.g[S7][0] | c.g[S3][0];
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x170));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x15c));
                c.g[T0][0] = c.g[S2][0];
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x154));
                c.g[T2][0] = c.g[S0][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                c.g[T3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                c.g[RA][0] = sx(0x0037fd68);
                c.sq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                Callee { addr: 0x0037e120, f: ee::s12::PS2GraphicsMan_EmitModelHeader }.call(c, 0x0037fd68)?;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = 0x50000000;
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x164));
                c.g[A0][0] = 0xfff0000;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x140));
                c.c_eq_s(21, 20);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[A0][0] = c.g[A0][0] | 0xffff;
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sd((c.g[A1][0] as u32), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x140));
                c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x140));
                c.lq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                c.g[S6][0] = c.g[V0][0] & c.g[A0][0];
                let t = !c.fcc(); 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x140), c.g[V0][0]);
                if t { pc = 0x0037ffdc; continue; }
                pc = 0x0037fdb4; continue;
            }
            0x0037fdb4 => {
                c.cycles += 2;
                let t = c.g[T4][0] == 0u64; 
                c.g[T1][0] = 0xffffffffffffffe0;
                if t { pc = 0x0037ff40; continue; }
                pc = 0x0037fdbc; continue;
            }
            0x0037fdbc => {
                c.cycles += 53;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
                c.g[A1][0] = c.g[T4][0];
                c.g[V1][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1ac));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x144), c.g[V1][0]);
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.g[V0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x12));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.sh((c.g[T0][0] as u32).wrapping_add(0x2), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1cc4));
                c.g[V0][0] = c.lw((c.g[T4][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.g[V1][0] & 0x1f;
                c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[T4][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[S4][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x144));
                c.g[T1][0] = c.lh((c.g[SP][0] as u32).wrapping_add(0x144));
                c.g[V0][0] = sx(((c.g[S4][0] as u32) as i32 >> 16) as u32);
                c.g[S2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                c.g[A2][0] = c.g[S4][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                c.g[T1][0] = c.g[T1][0] | c.g[V0][0];
                c.g[S5][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1cc0));
                c.g[T1][0] = c.g[T4][0] ^ c.g[T1][0];
                c.sq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.g[A3][0] = sx(((c.g[T1][0] as u32) as i32 >> 8) as u32);
                c.g[T2][0] = sx(((c.g[T1][0] as u32) as i32 >> 16) as u32);
                c.g[A3][0] = sx((c.g[A3][0] as u32) << 5);
                c.g[T2][0] = sx((c.g[T2][0] as u32) << 5);
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[T1][0] as u32)));
                c.g[V0][0] = sx((c.g[T1][0] as u32) << 5);
                c.g[V1][0] = sx(((c.g[T1][0] as u32) as i32 >> 24) as u32);
                c.g[A3][0] = sx(((c.g[A3][0] as u32) as i32 >> 8) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
                c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[T1][0] as u32)));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V0][0] = c.g[V0][0] ^ c.g[A3][0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T1][0] as u32)));
                c.g[T2][0] = sx(((c.g[T2][0] as u32) as i32 >> 16) as u32);
                c.g[V0][0] = c.g[V0][0] ^ c.g[T2][0];
                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[S3][0] = c.g[V0][0] & 0xff;
                c.g[T0][0] = c.g[S5][0];
                c.g[RA][0] = sx(0x0037fe84);
                c.g[A3][0] = c.g[S3][0];
                call_ptr(c, 0x00395000, 0x0037fe84)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] != 0u64; 
                c.lq(12, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                if t { pc = 0x0037ff28; continue; }
                pc = 0x0037fe90; continue;
            }
            0x0037fe90 => {
                c.cycles += 38;
                c.g[V0][0] = 0x50000;
                c.g[A2][0] = 0x50000;
                c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A2][0] = c.g[A2][0] | 0x7498;
                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x7494));
                c.g[A0][0] = c.g[T4][0];
                c.g[A1][0] = c.g[S4][0];
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 5);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.sw((c.g[A3][0] as u32).wrapping_add(0x7494), c.g[V1][0]);
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[S0][0] = c.g[V0][0];
                c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[S5][0]);
                c.sw((c.g[V0][0] as u32), c.g[T4][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S4][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0u64);
                c.g[RA][0] = sx(0x0037fee0);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
                Callee { addr: 0x00364240, f: ee::s12::DeferMan_ComputeSortKey }.call(c, 0x0037fee0)?;
                c.g[A1][0] = 0x60000;
                c.g[A2][0] = sx((c.g[S3][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x78a0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                c.sw((c.g[A0][0] as u32), c.g[S0][0]);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x7ca4));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x7ca8), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x7ca4));
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x7cac), c.g[S0][0]);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x7ca4), c.g[A0][0]);
                pc = 0x0037ff28; continue;
            }
            0x0037ff28 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S7][0];
                c.g[RA][0] = sx(0x0037ff38);
                c.g[A2][0] = c.g[S6][0];
                Callee { addr: 0x00362978, f: ee::s12::DeferMan_AppendPacket }.call(c, 0x0037ff38)?;
                let t = true; 
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                if t { pc = 0x0037ffd4; continue; }
                pc = 0x0037ff40; continue;
            }
            0x0037ff40 => {
                c.cycles += 5;
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
                c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                if t { pc = 0x0037ffd0; continue; }
                pc = 0x0037ff54; continue;
            }
            0x0037ff54 => {
                c.cycles += 31;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.g[A2][0] = 0xffffffffffffffe0;
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[A0][0] = c.g[A0][0] | c.g[S3][0];
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V1][0] = c.g[V1][0] & 0x1f;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[A3][0] as u32));
                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                c.ldl(7, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.ldr(7, (c.g[T0][0] as u32));
                c.ldl(9, (c.g[T0][0] as u32).wrapping_add(0xf));
                c.ldr(9, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.sdr(7, (c.g[V0][0] as u32));
                c.sdl(9, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.sdr(9, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S7][0]);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[S6][0]);
                pc = 0x0037ffd0; continue;
            }
            0x0037ffd0 => {
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                pc = 0x0037ffd4; continue;
            }
            0x0037ffd4 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S1][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
                if t { pc = 0x003802f4; continue; }
                pc = 0x0037ffdc; continue;
            }
            0x0037ffdc => {
                c.cycles += 19;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.ldr(5, (c.g[V0][0] as u32));
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x1b));
                c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x23));
                c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x1c));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[A3][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6b90));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
                c.sw((c.g[S1][0] as u32).wrapping_add(0xe84), c.g[V1][0]);
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x158));
                c.lq(2, (c.g[T1][0] as u32).wrapping_add(0x30));
                let t = c.g[A0][0] != 0u64; 
                c.sq(2, (c.g[SP][0] as u32));
                if t { pc = 0x00380030; continue; }
                pc = 0x00380028; continue;
            }
            0x00380028 => {
                c.cycles += 2;
                c.g[RA][0] = sx(0x00380030);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x00380030)?;
                pc = 0x00380030; continue;
            }
            0x00380030 => {
                c.cycles += 12;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x194));
                c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x5920));
                c.lqc2(10, (c.g[SP][0] as u32));
                c.lqc2(1, (c.g[V0][0] as u32));
                c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
                c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 1, Src::Bc(10, X));
                c.vu0().madda(0b1111, 2, Src::Bc(10, Y));
                c.vu0().madda(0b1111, 3, Src::Bc(10, Z));
                c.vu0().madd(0b1111, 11, 4, Src::Bc(10, W));
                c.vu0().mul(0b0001, 11, 11, Src::Bc(5, W));
                pc = 0x00380060; continue;
            }
            0x00380060 => {
                c.cycles += 39;
                c.vu0().max(0b0001, 11, 11, Src::Bc(0, X));
                c.vu0().mini(0b0001, 11, 11, Src::Bc(5, X));
                c.vu0().ftoi(0, 0b1111, 11, 11);
                c.vu0().mtir(1, 11, 3);
                c.cfc2(3, 1);
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A2][0] = 0xfffffffffffffffc;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x198));
                c.g[V1][0] = c.g[V1][0] << 32;
                c.g[V1][0] = ((c.g[V1][0] as i64) >> 32) as u64;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 10);
                c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                c.g[T2][0] = 0xffffffffffffff83;
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[T0][0] = 0xfffffffffffff07f;
                c.g[V0][0] = c.g[V0][0] | 0x2;
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x19c));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] | 0xc;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.g[V0][0] = c.g[V0][0] | 0x180;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
                c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                if t { pc = 0x00380178; continue; }
                pc = 0x003800fc; continue;
            }
            0x003800fc => {
                c.cycles += 31;
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.g[A2][0] = 0xffffffffffffffe0;
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[T1][0] as u32)));
                c.g[A0][0] = c.g[A0][0] | c.g[S3][0];
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V1][0] = c.g[V1][0] & 0x1f;
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[A3][0] as u32));
                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                c.ldl(7, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.ldr(7, (c.g[T0][0] as u32));
                c.ldl(9, (c.g[T0][0] as u32).wrapping_add(0xf));
                c.ldr(9, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.sdr(7, (c.g[V0][0] as u32));
                c.sdl(9, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.sdr(9, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S7][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[S6][0]);
                pc = 0x00380178; continue;
            }
            0x00380178 => {
                c.cycles += 60;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                c.g[AT][0] = 0x43000000;
                c.mtc1(1, 0);
                c.g[T3][0] = 0x1;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x14));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
                c.mul_s(0, 21, 0);
                c.ldl(5, (c.g[V1][0] as u32).wrapping_sub(0xd));
                c.ldr(5, (c.g[V1][0] as u32).wrapping_sub(0x14));
                c.ldl(6, (c.g[V1][0] as u32).wrapping_sub(0x5));
                c.ldr(6, (c.g[V1][0] as u32).wrapping_sub(0xc));
                c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x4));
                c.sdl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.sdr(5, (c.g[V1][0] as u32));
                c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.sdr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                c.g[A3][0] = 0x800000;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.cvt_w_s(1, 0);
                c.mfc1(5, 1);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x14));
                c.sw((c.g[S1][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
                c.g[A1][0] = c.g[A1][0] & 0xff;
                c.g[A1][0] = sx((c.g[A1][0] as u32) << 12);
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1b4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                c.g[A1][0] = c.g[A1][0] & c.g[V1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a4));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                c.g[V0][0] = c.g[V0][0] | 0x2c;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
                c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                if t { pc = 0x003802e4; continue; }
                pc = 0x00380268; continue;
            }
            0x00380268 => {
                c.cycles += 31;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.g[A2][0] = 0xffffffffffffffe0;
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
                c.g[V1][0] = c.g[V1][0] & 0x1f;
                c.g[A0][0] = c.g[A0][0] | c.g[S3][0];
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[A3][0] as u32));
                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                c.ldl(7, (c.g[T0][0] as u32).wrapping_add(0x7));
                c.ldr(7, (c.g[T0][0] as u32));
                c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0xf));
                c.ldr(3, (c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.sdr(7, (c.g[V0][0] as u32));
                c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), c.g[T3][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S7][0]);
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[S6][0]);
                c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
                pc = 0x003802e4; continue;
            }
            0x003802e4 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                c.sw((c.g[S1][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
                pc = 0x003802f4; continue;
            }
            0x003802f4 => {
                c.cycles += 7;
                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18c));
                c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x150), c.g[A3][0]);
                if t { if back_edge_unwinds(c, 0x0037f240) { return Ok(()); } pc = 0x0037f240; continue; }
                pc = 0x00380310; continue;
            }
            0x00380310 => {
                c.cycles += 6;
                c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1bc));
                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c0));
                if t { if back_edge_unwinds(c, 0x0037ed40) { return Ok(()); } pc = 0x0037ed40; continue; }
                pc = 0x00380328; continue;
            }
            0x00380328 => {
                c.cycles += 1;
                c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x160));
                pc = 0x0038032c; continue;
            }
            0x0038032c => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.g[V0][0] & 0x8;
                let t = c.g[V0][0] == 0u64; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x270));
                if t { pc = 0x0038034c; continue; }
                pc = 0x0038033c; continue;
            }
            0x0038033c => {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
                c.sw((c.g[S1][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
                pc = 0x00380348; continue;
            }
            0x00380348 => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x270));
                pc = 0x0038034c; continue;
            }
            0x0038034c => {
                c.cycles += 13;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x260));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x250));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x240));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x230));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x220));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x288));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x280));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x290));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x0037e238, 0x00380380); return Ok(()); }
        }
    }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, elf_image, fakes, fill, DATA, GAME_GP};

    /// The distinct cycle totals the generated body takes over the seeds.
    fn oc1477_paths(generated: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    // ------------------------------------------------------------------
    // The (index, value) short-pair loops: 0x3a0158, 0x3a0274, 0x3a3808.

    const OC1477_SHORTS: u32 = DATA + 0x1000;
    const OC1477_OBJ: u32 = DATA + 0x2000;

    /// A machine with the game image (the jump tables) and the pair array.
    fn oc1477_pairs_machine(c: &mut Cpu, seed: u32, salt: u64) -> crate::difftest::Rng {
        let mut r = fakes::machine(c, seed, salt, 6);
        elf_image(c);
        fill(c, &mut r, OC1477_SHORTS, 16);
        fill(c, &mut r, OC1477_OBJ, 64);
        r
    }

    /// `UIObject_SetAnimParams` fixture: 0 to 4 pairs, indices on both sides
    /// of the 0x11 table size, the entry or one of its resume entries.
    pub(crate) fn oc1477_object_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1477_pairs_machine(c, seed, 0x3a0158);
        let count = seed % 5;
        for i in 0..4u32 {
            let idx = if r.next_u32() % 10 < 7 {
                0x11 + r.next_u32() % 0x20
            } else {
                r.next_u32() % 0x11
            };
            c.bus.write16(OC1477_SHORTS + 4 * i, idx as u16);
            c.bus.write16(OC1477_SHORTS + 4 * i + 2, r.next_u32() as u16);
        }
        c.g[A0][0] = OC1477_OBJ as u64;
        c.g[A1][0] = count as u64;
        c.g[A2][0] = OC1477_SHORTS as u64;
        c.g[T0][0] = (seed / 5 % 3) as u64;
        c.pc = match seed % 4 {
            0 => 0x003a_0158,
            1 => 0x003a_0164,
            2 => 0x003a_0168,
            _ => {
                let idx = c.bus.read16(OC1477_SHORTS) as u64 % 0x11;
                c.g[A0][0] = idx;
                c.g[V0][0] = 0x49_0000;
                0x003a_0184
            }
        };
    }

    /// `UIObject_SetAnimParams`: no pairs, loop pairs (index 0x11 and up,
    /// through the tail) and table cases (a jump out), and every resume entry.
    #[test]
    fn ui_object_set_anim_params_matches() {
        check(
            ee::s13::f_003a0158__generated,
            ui_object_set_anim_params,
            160,
            &oc1477_object_setup,
        );
        assert!(
            oc1477_paths(ee::s13::f_003a0158__generated, 160, &oc1477_object_setup) >= 5,
            "fixture reaches too few paths"
        );
    }

    /// `UIObject_AnimLoopTail`: a counter below and at the count, and the return
    /// entry 0x3a0284.
    fn oc1477_tail_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1477_pairs_machine(c, seed, 0x3a0274);
        for i in 0..4u32 {
            let idx = 0x11 + r.next_u32() % 0x20;
            c.bus.write16(OC1477_SHORTS + 4 * i, idx as u16);
        }
        c.g[A0][0] = OC1477_OBJ as u64;
        c.g[A1][0] = (seed % 5) as u64;
        c.g[A2][0] = OC1477_SHORTS as u64;
        c.g[T0][0] = (seed / 5 % 4) as u64;
        c.pc = if seed % 7 == 6 { 0x003a_0284 } else { 0x003a_0274 };
    }

    #[test]
    fn ui_object_anim_loop_tail_matches() {
        check(
            ee::s13::f_003a0274__generated,
            ui_object_anim_loop_tail,
            140,
            &oc1477_tail_setup,
        );
        assert!(
            oc1477_paths(ee::s13::f_003a0274__generated, 140, &oc1477_tail_setup) >= 3,
            "fixture reaches too few paths"
        );
    }

    /// `UIVector_SetAnimParams` fixture: the object's channel word (zero or
    /// not), the bound in `this+0x74`, indices below 21 and `21 + 9 q + r`,
    /// every resume entry.
    fn oc1477_vector_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1477_pairs_machine(c, seed, 0x3a3808);
        c.bus
            .write32(OC1477_OBJ + 0x80, if seed % 7 == 0 { 0 } else { 0x1234 });
        c.bus.write32(OC1477_OBJ + 0x74, (r.next_u32() % 5) << 17 | 0x1_ffff & r.next_u32());
        for i in 0..4u32 {
            let idx = if r.next_u32() % 6 == 0 {
                r.next_u32() % 21
            } else {
                21 + 9 * (r.next_u32() % 5) + r.next_u32() % 9
            };
            c.bus.write16(OC1477_SHORTS + 4 * i, idx as u16);
            c.bus.write16(OC1477_SHORTS + 4 * i + 2, r.next_u32() as u16);
        }
        c.g[A0][0] = OC1477_OBJ as u64;
        c.g[A1][0] = (seed / 8 % 5) as u64;
        c.g[A2][0] = OC1477_SHORTS as u64;
        c.g[T0][0] = OC1477_OBJ as u64;
        c.g[T2][0] = (seed / 40 % 3) as u64;
        let idx21 = 21 + 9 * (r.next_u32() % 5) as u64 + (r.next_u32() % 9) as u64;
        c.pc = match seed % 8 {
            0 => 0x003a_3808,
            1 => 0x003a_3818,
            2 => 0x003a_3820,
            3 => {
                c.g[A0][0] = idx21;
                0x003a_383c
            }
            4 => {
                c.g[V0][0] = 9;
                c.lo[0] = r.next_u32() as u64 % 6;
                c.hi[0] = r.next_u32() as u64 % 9;
                0x003a_3850
            }
            5 => {
                c.g[V1][0] = (seed / 8 % 2) as u64;
                c.g[T1][0] = (r.next_u32() % 9) as u64;
                0x003a_3870
            }
            6 => {
                c.g[V1][0] = (seed / 8 % 2) as u64;
                c.g[T1][0] = (r.next_u32() % 9) as u64;
                0x003a_3878
            }
            _ => {
                c.g[T1][0] = (r.next_u32() % 9) as u64;
                0x003a_3880
            }
        };
    }

    #[test]
    fn ui_vector_set_anim_params_matches() {
        check_lohi(
            ee::s13::f_003a3808__generated,
            ui_vector_set_anim_params,
            320,
            &oc1477_vector_setup,
        );
        assert!(
            oc1477_paths(ee::s13::f_003a3808__generated, 320, &oc1477_vector_setup) >= 8,
            "fixture reaches too few paths"
        );
    }

    // ------------------------------------------------------------------
    // PS2GraphicsMan_v95 (DrawModel, 0x37e238).

    const OC1477_CTRL: u32 = DATA + 0xb_0000;
    const OC1477_MAGIC: u32 = 0x1477_0037;
    const GM: u32 = DATA + 0x1_0000;
    const VT: u32 = DATA + 0x1_7000;
    const ST: u32 = DATA + 0x1_8000;
    const WORK: u32 = DATA + 0x1_9000;
    const INST: u32 = DATA + 0x1_a000;
    const MODEL: u32 = DATA + 0x1_b000;
    const BOXES: u32 = DATA + 0x1_c000;
    const SUBS: u32 = DATA + 0x1_d000;
    const MESH_ARR: u32 = DATA + 0x1_e000;
    const MESHES: u32 = DATA + 0x1_f000;
    const TEX: u32 = DATA + 0x2_0000;
    const MATS: u32 = DATA + 0x2_1000;
    const WORLD: u32 = DATA + 0x2_2000;
    const NODEM: u32 = DATA + 0x2_3000;
    const MATRIX2: u32 = DATA + 0x2_4000;
    const BOXM: u32 = DATA + 0x2_5000;
    const BUCKET: u32 = DATA + 0x2_6000;
    const PK: u32 = DATA + 0x3_0000;
    const DEFER: u32 = DATA + 0x4_0000;

    /// A gated fake of a real callee in an override slot: with the gate
    /// word in the machine's RAM set it logs the call like `fakes::fake`.
    macro_rules! oc1477_fake {
        ($name:ident, $prev:ident, $generated:path, $addr:expr, $post:expr) => {
            static $prev: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1477_CTRL) != OC1477_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
                if c.pc == ra {
                    let post: fn(&mut Cpu) = $post;
                    post(c);
                }
            }
        };
    }

    fn oc1477_keep(_c: &mut Cpu) {}
    fn oc1477_bucket(c: &mut Cpu) {
        c.g[V0][0] = c.bus.read32(OC1477_CTRL + 8) as u64;
    }

    oc1477_fake!(
        oc1477_fake_update,
        OC1477_PREV_UPDATE,
        ee::s12::f_0037d968__generated,
        0x0037_d968,
        oc1477_keep
    );
    oc1477_fake!(
        oc1477_fake_emit,
        OC1477_PREV_EMIT,
        ee::s12::f_0037e120__generated,
        0x0037_e120,
        oc1477_keep
    );
    oc1477_fake!(
        oc1477_fake_sort_key,
        OC1477_PREV_SORT_KEY,
        ee::s12::f_00364240__generated,
        0x0036_4240,
        oc1477_keep
    );
    oc1477_fake!(
        oc1477_fake_append,
        OC1477_PREV_APPEND,
        ee::s12::f_00362978__generated,
        0x0036_2978,
        oc1477_keep
    );
    oc1477_fake!(
        oc1477_fake_find_bucket,
        OC1477_PREV_FIND_BUCKET,
        ee::s13::f_00395000__generated,
        0x0039_5000,
        oc1477_bucket
    );
    oc1477_fake!(
        oc1477_fake_light_cb,
        OC1477_PREV_LIGHT_CB,
        ee::s09::f_002f5148__generated,
        0x002f_5148,
        oc1477_keep
    );

    fn oc1477_install() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0037D968),
            oc1477_fake_update,
            &OC1477_PREV_UPDATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0037E120),
            oc1477_fake_emit,
            &OC1477_PREV_EMIT,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00364240),
            oc1477_fake_sort_key,
            &OC1477_PREV_SORT_KEY,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00362978),
            oc1477_fake_append,
            &OC1477_PREV_APPEND,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00395000),
            oc1477_fake_find_bucket,
            &OC1477_PREV_FIND_BUCKET,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002F5148),
            oc1477_fake_light_cb,
            &OC1477_PREV_LIGHT_CB,
        );
    }

    fn oc1477_floats(c: &mut Cpu, r: &mut crate::difftest::Rng, addr: u32, words: u32) {
        for i in 0..words {
            let v = r.f32(2.0);
            c.bus.write32(addr + 4 * i, v);
        }
    }

    /// One model drawn: the manager (`gm`), the model instance with its box
    /// tree, sub objects, strips and materials, and the draw state; the
    /// callees are fakes (vtable methods and gated slot fakes).
    pub(crate) fn oc1477_draw_setup(c: &mut Cpu, seed: u32) {
        oc1477_install();
        let mut r = fakes::machine(c, seed, 0x37e238, 36);
        let gp = GAME_GP;
        c.bus.write32(OC1477_CTRL, OC1477_MAGIC);
        c.bus.write32(OC1477_CTRL + 8, if seed % 3 == 0 { 0 } else { BUCKET });
        c.bus.write32(gp + 0x1400, 0);
        c.bus.write32(gp + 0x1404, (seed / 3) % 2);
        c.bus.write32(gp + 0x2a94, WORK);
        // Fake results: a clip test of 0 (hash pointer), 1, 2 or other.
        for i in 0..8u32 {
            let v = [0, 1, 2, 0, 0x100][(r.next_u32() % 5) as usize];
            c.bus.write32(fakes::LOG_RESULTS + 4 * i, v);
        }
        // gm: the view-matrices flag, the vtable, the draw state, the defer state.
        fill(c, &mut r, GM, 0x1b00);
        c.bus.write32(GM + 0x6b90, if seed % 5 == 0 { 0 } else { 1 });
        c.bus.write32(GM + 0x10d8, VT);
        c.bus.write32(GM + 0xe84, ST);
        c.bus.write32(GM + 0x18f0, DEFER);
        c.bus.write32(GM + 0x5a00, PK);
        for off in [0x2e8u32, 0x2f0, 0x118] {
            c.bus.write16(VT + off, (r.next_u32() % 8 * 4) as u16);
        }
        for off in [0x2ecu32, 0x2f4, 0x11c] {
            c.bus.write32(VT + off, fakes::FAKE);
        }
        fill(c, &mut r, ST, 0x40);
        fill(c, &mut r, WORK, 0x1d0);
        // The defer state: packet count, bucket pointers (or none), counters.
        c.bus.write32(DEFER, if seed % 4 == 0 { 0xa28 } else { r.next_u32() % 0x100 });
        for off in [0x57488u32, 0x5748c, 0x57490] {
            c.bus
                .write32(DEFER + off, if r.next_u32() % 4 == 0 { 0 } else { BUCKET });
        }
        c.bus.write32(DEFER + 0x57494, r.next_u32() % 4);
        c.bus.write32(DEFER + 0x67ca4, r.next_u32() % 4);
        c.bus.write32(DEFER + 0x69cc0, r.next_u32());
        c.bus.write32(DEFER + 0x69cc4, r.next_u32());
        fill(c, &mut r, BUCKET, 16);
        // The instance and its model.
        fill(c, &mut r, INST, 0x40);
        oc1477_floats(c, &mut r, INST + 0x60, 6);
        oc1477_floats(c, &mut r, INST + 0x84, 1);
        let flags = [0u32, 0x4000, 0, 0x8][(seed / 7 % 4) as usize];
        c.bus.write32(INST + 8, flags);
        c.bus.write32(INST + 0x80, MODEL);
        c.bus.write32(INST + 0x98, r.next_u32());
        c.bus.write16(INST + 0x92, (r.next_u32() & 1) as u16);
        let boxes = [0u32, 1, 2, 3, 1, 2][(r.next_u32() % 6) as usize];
        c.bus.write32(MODEL + 4, boxes);
        c.bus.write32(MODEL + 8, BOXES);
        let mflags = [0u32, 2, 8, 10][(seed / 11 % 4) as usize];
        c.bus.write32(MODEL + 0x10, mflags);
        c.bus.write32(MODEL + 0x24, r.next_u32());
        oc1477_floats(c, &mut r, MODEL + 0x18, 3);
        let fast = boxes == 1 && seed % 9 == 0;
        for i in 0..4u32 {
            let b = BOXES + 0x10 * i;
            let parent = if i == 0 || r.next_u32() % 2 == 0 {
                u32::MAX
            } else {
                r.next_u32() % i
            };
            c.bus.write32(b, parent);
            let sub = if r.next_u32() % 5 == 0 { 0 } else { SUBS + 0x40 * i };
            c.bus.write32(b + 4, sub);
            c.bus.write32(b + 8, r.next_u32());
            let m = if fast && i == 0 { 0x004f_f1a0 } else { BOXM + 0x40 * i };
            c.bus.write32(b + 0xc, m);
            oc1477_floats(c, &mut r, BOXM + 0x40 * i, 16);
            // The sub object: two vectors, the strip count and strips.
            let s = SUBS + 0x40 * i;
            oc1477_floats(c, &mut r, s, 6);
            c.bus.write32(s + 0x18, r.next_u32());
            c.bus.write32(s + 0x1c, r.next_u32() % 3);
            c.bus.write32(s + 0x20, MESH_ARR + 0x10 * i);
            for k in 0..3u32 {
                let mesh = MESHES + 0x40 * i + 0x10 * k;
                c.bus.write32(MESH_ARR + 0x10 * i + 4 * k, mesh);
                c.bus.write16(mesh, (r.next_u32() % 4) as u16);
                c.bus.write16(mesh + 2, (r.next_u32() & 0xc) as u16);
                c.bus.write32(mesh + 4, r.next_u32() % 0x100);
            }
        }
        // Texture table and materials.
        let blend = [0u32, 0x80000, 0x100000, 0x180000];
        let kind = [
            0u32, 0x20000, 0x40000, 0x60000, 0x200000, 0x220000, 0x260000, 0x600000, 0x620000,
            0x660000, 0x240000,
        ];
        for k in 0..4u32 {
            c.bus.write32(TEX + 4 + 4 * k, MATS + 0x20 * k);
            fill(c, &mut r, MATS + 0x20 * k, 8);
            let f = kind[(r.next_u32() % 11) as usize]
                | blend[(r.next_u32() % 4) as usize]
                | (r.next_u32() & 0x1_ffff & !0x1_8000);
            c.bus.write32(MATS + 0x20 * k + 0xc, f);
        }
        oc1477_floats(c, &mut r, WORLD, 16);
        oc1477_floats(c, &mut r, NODEM, 64);
        let alpha = [1.0f32.to_bits(), 1.0f32.to_bits(), 0.5f32.to_bits(), 0.0f32.to_bits()]
            [(seed / 2 % 4) as usize];
        c.f[12] = alpha;
        let t2 = [0u32, 0x400, 0x800, 0x1020, 1, 0x1400, 0x20, 0x1c00][(r.next_u32() % 8) as usize];
        c.g[A0][0] = GM as u64;
        c.g[A1][0] = INST as u64;
        c.g[A2][0] = TEX as u64;
        c.g[A3][0] = WORLD as u64;
        c.g[T0][0] = if seed % 2 == 0 { 0 } else { NODEM as u64 };
        c.g[T1][0] = MATRIX2 as u64;
        c.g[T2][0] = t2 as u64;
        c.pc = 0x0037_e238;
    }

    /// `PS2GraphicsMan_v95`: one to three boxes of strips, blend and material
    /// kinds, node matrices or not, light flag, the early outs, the 1.0 alpha
    /// defer path (found or new bucket) and the alpha-blended packets; calls
    /// unwinding at chosen indices.
    #[test]
    fn ps2_graphics_man_v95_matches() {
        check(
            ee::s12::f_0037e238__generated,
            ps2_graphics_man_v95,
            240,
            &oc1477_draw_setup,
        );
        assert!(
            oc1477_paths(ee::s12::f_0037e238__generated, 240, &oc1477_draw_setup) >= 20,
            "fixture reaches too few paths"
        );
    }
}
