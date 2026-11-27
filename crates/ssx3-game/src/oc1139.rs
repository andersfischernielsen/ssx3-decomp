//! Staging module oc1139 (wave wave272): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1139`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002f_2598, c_splash_manager_init_drop, "cSplashManager_InitDrop"),
    (0x002f_2d70, c_splash_manager_init_crystal, "cSplashManager_InitCrystal"),
    (0x003b_bda0, snd_parse_pt_header, "SND_ParsePtHeader"),
    (0x003c_4b70, snd_spu_mem_alloc, "SND_SpuMemAlloc"),
];

/// `void cSplashManager_InitDrop(tDrop* drop, t2Vector pos, t2Vector* range)`
/// (0x2f2598): starts a lens drop at `pos`. Width is a random size between
/// the configured minimum and `range.x`; height is the width plus a random
/// aspect difference, clamped to `[min, range.y]`. Age 0, a random life
/// span, active, zero velocity, a random angle (0..360), a random spin and a
/// colour mixed between two tints, all from the float LCG at `gp+0xa0c`
/// (`seed = ((seed * 0x18fcd + 0xe9507c) & 0x7fffff) | 1.0f`).
pub fn c_splash_manager_init_drop(c: &mut Cpu) {
'b_2f264c: {
c.cycles += 44;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xa0c));
c.g[T3][0] = 0x10000;
c.g[T3][0] = c.g[T3][0] | 0x8fcd;
c.g[T1][0] = 0xe90000;
c.mult(0, 2, 2, 11);
c.g[T1][0] = c.g[T1][0] | 0x507c;
c.g[T2][0] = 0x7f0000;
c.g[T5][0] = 0x3f800000;
c.g[T2][0] = c.g[T2][0] | 0xffff;
c.g[T4][0] = c.g[A0][0];
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0x11ac));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
c.neg_s(3, 2);
c.g[V0][0] = c.g[V0][0] | c.g[T5][0];
c.lwc1(4, (c.g[A2][0] as u32));
c.mult(0, 4, 2, 11);
c.lwc1(7, (c.g[A2][0] as u32).wrapping_add(0x4));
c.sw((c.g[GP][0] as u32).wrapping_add(0xa0c), c.g[V0][0]);
c.sub_s(2, 2, 3);
c.lwc1(5, (c.g[GP][0] as u32).wrapping_add(0x11a4));
c.g[T6][0] = c.g[A1][0];
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0xa0c));
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 8);
c.g[V1][0] = c.g[V1][0] & c.g[T2][0];
c.sub_s(4, 4, 5);
c.g[V1][0] = c.g[V1][0] | c.g[T5][0];
c.sub_s(1, 1, 8);
c.sw((c.g[GP][0] as u32).wrapping_add(0xa0c), c.g[V1][0]);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0xa0c));
c.mul_s(4, 4, 1);
c.sub_s(0, 0, 8);
c.add_s(1, 5, 4);
c.mul_s(2, 2, 0);
c.add_s(3, 3, 2);
c.add_s(0, 1, 3);
c.c_le_s(5, 0);
let t = !c.fcc();
c.mov_s(6, 5);
if t {
break 'b_2f264c;
} else {
c.cycles += 1;
c.min_s(6, 0, 7);
break 'b_2f264c;
}
}
c.cycles += 112;
c.mult(0, 4, 3, 11);
c.sw((c.g[T4][0] as u32).wrapping_add(0x1c), 0u64);
c.swc1(1, (c.g[SP][0] as u32));
c.g[V1][0] = 0x1;
c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0x119c));
c.g[A2][0] = 0x4a0000;
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0x11a0));
c.g[T0][0] = 0x500000;
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
c.sub_s(1, 1, 2);
c.g[V0][0] = c.g[V0][0] | c.g[T5][0];
c.g[AT][0] = 0x43b40000;
c.mtc1(1, 3);
c.sw((c.g[GP][0] as u32).wrapping_add(0xa0c), c.g[V0][0]);
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x46e0));
c.g[A3][0] = 0x500000;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0xa0c));
c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x46f0));
c.sw((c.g[T4][0] as u32), c.g[V1][0]);
c.sub_s(0, 0, 8);
c.mul_s(1, 1, 0);
c.add_s(2, 2, 1);
c.swc1(2, (c.g[T4][0] as u32).wrapping_add(0x20));
c.ldl(15, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(15, (c.g[SP][0] as u32));
c.sdl(15, (c.g[T4][0] as u32).wrapping_add(0x13));
c.sdr(15, (c.g[T4][0] as u32).wrapping_add(0xc));
c.ldl(15, (c.g[T6][0] as u32).wrapping_add(0x7));
c.ldr(15, (c.g[T6][0] as u32));
c.sdl(15, (c.g[T4][0] as u32).wrapping_add(0xb));
c.sdr(15, (c.g[T4][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5960));
c.ldl(14, (c.g[V0][0] as u32).wrapping_add(0x7));
c.ldr(14, (c.g[V0][0] as u32));
c.sdl(14, (c.g[T4][0] as u32).wrapping_add(0x1b));
c.sdr(14, (c.g[T4][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xa0c));
c.mult(0, 3, 2, 11);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
c.g[V0][0] = c.g[V0][0] | c.g[T5][0];
c.sw((c.g[GP][0] as u32).wrapping_add(0xa0c), c.g[V0][0]);
c.mult(0, 3, 2, 11);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0xa0c));
c.sub_s(0, 0, 8);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
c.g[V0][0] = c.g[V0][0] | c.g[T5][0];
c.mul_s(0, 0, 3);
c.mult(0, 6, 2, 11);
c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x34));
c.sw((c.g[GP][0] as u32).wrapping_add(0xa0c), c.g[V0][0]);
c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x11dc));
c.g[V1][0] = c.g[V1][0] & c.g[T2][0];
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0xa0c));
c.g[V1][0] = c.g[V1][0] | c.g[T5][0];
c.neg_s(2, 0);
c.sub_s(1, 1, 8);
c.sub_s(0, 0, 2);
c.mul_s(0, 0, 1);
c.add_s(2, 2, 0);
c.swc1(2, (c.g[T4][0] as u32).wrapping_add(0x38));
c.sw((c.g[GP][0] as u32).wrapping_add(0xa0c), c.g[V1][0]);
c.lwc1(7, (c.g[A0][0] as u32).wrapping_add(0xc));
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0xa0c));
c.lwc1(5, (c.g[T0][0] as u32).wrapping_add(0x46e0));
c.sub_s(0, 0, 8);
c.lwc1(3, (c.g[A0][0] as u32).wrapping_add(0x4));
c.lwc1(2, (c.g[A0][0] as u32).wrapping_add(0x8));
c.lwc1(6, (c.g[A3][0] as u32).wrapping_add(0x46f0));
c.sub_s(1, 8, 0);
c.lwc1(9, (c.g[A1][0] as u32).wrapping_add(0xc));
c.lwc1(4, (c.g[A1][0] as u32).wrapping_add(0x4));
c.mul_s(6, 6, 0);
c.lwc1(8, (c.g[A1][0] as u32).wrapping_add(0x8));
c.mul_s(9, 9, 0);
c.mul_s(7, 7, 1);
c.mul_s(5, 5, 1);
c.mul_s(3, 3, 1);
c.mul_s(2, 2, 1);
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x2c));
c.mul_s(4, 4, 0);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.mul_s(8, 8, 0);
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x24));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
c.add_s(5, 5, 6);
c.add_s(3, 3, 4);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x30));
c.add_s(2, 2, 8);
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x34));
c.add_s(7, 7, 9);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x10));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x1c));
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x38));
c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x3c));
c.ldl(14, (c.g[SP][0] as u32).wrapping_add(0x17));
c.ldr(14, (c.g[SP][0] as u32).wrapping_add(0x10));
c.ldl(15, (c.g[SP][0] as u32).wrapping_add(0x1f));
c.ldr(15, (c.g[SP][0] as u32).wrapping_add(0x18));
c.sdl(14, (c.g[T4][0] as u32).wrapping_add(0x2b));
c.sdr(14, (c.g[T4][0] as u32).wrapping_add(0x24));
c.sdl(15, (c.g[T4][0] as u32).wrapping_add(0x33));
c.sdr(15, (c.g[T4][0] as u32).wrapping_add(0x2c));
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.pc = t; return;
}

/// `void cSplashManager_InitCrystal(tCrystal* crystal, t2Vector pos, int n)`
/// (0x2f2d70): starts a lens crystal at `pos`. Size is random in the
/// configured range (height from the width plus a random aspect difference,
/// clamped), shrunk by 0.7 per generation `n`; then age 0, random life span,
/// active, zero velocity, a random tint, angle and spin (the float LCG at
/// `gp+0xa0c`), and the +0x40..+0x48 and dead words cleared.
pub fn c_splash_manager_init_crystal(c: &mut Cpu) {
'b_2f2e1c: {
c.cycles += 42;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xa0c));
c.g[V1][0] = 0x10000;
c.g[V1][0] = c.g[V1][0] | 0x8fcd;
c.g[A3][0] = 0xe90000;
c.mult(0, 2, 2, 3);
c.g[A3][0] = c.g[A3][0] | 0x507c;
c.g[T0][0] = 0x7f0000;
c.g[T1][0] = 0x3f800000;
c.g[T0][0] = c.g[T0][0] | 0xffff;
c.g[T2][0] = c.g[A0][0];
c.lwc1(3, (c.g[GP][0] as u32).wrapping_add(0x11f8));
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.lwc1(5, (c.g[GP][0] as u32).wrapping_add(0x11f0));
c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
c.neg_s(4, 3);
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 6);
c.mult(0, 4, 2, 3);
c.sw((c.g[GP][0] as u32).wrapping_add(0xa0c), c.g[V0][0]);
c.lwc1(7, (c.g[GP][0] as u32).wrapping_add(0x11f4));
c.sub_s(3, 3, 4);
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0xa0c));
c.g[T5][0] = c.g[A1][0];
c.sub_s(2, 7, 5);
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.sub_s(1, 1, 6);
c.g[V1][0] = c.g[V1][0] & c.g[T0][0];
c.g[V1][0] = c.g[V1][0] | c.g[T1][0];
c.sw((c.g[GP][0] as u32).wrapping_add(0xa0c), c.g[V1][0]);
c.mul_s(2, 2, 1);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0xa0c));
c.sub_s(0, 0, 6);
c.add_s(6, 5, 2);
c.mul_s(3, 3, 0);
c.add_s(4, 4, 3);
c.add_s(0, 6, 4);
c.c_le_s(5, 0);
let t = !c.fcc();
c.mov_s(8, 5);
if t {
break 'b_2f2e1c;
} else {
c.cycles += 1;
c.min_s(8, 0, 7);
break 'b_2f2e1c;
}
}
'b_2f2e40: {
c.cycles += 4;
c.mov_s(5, 8);
c.g[T3][0] = 0x500000;
let t = (c.g[A2][0] as i64) <= 0;
c.g[T4][0] = 0x500000;
if t {
break 'b_2f2e40;
} else {
c.cycles += 1;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3910));
'l_2f2e30: loop {
c.cycles += 4;
c.mul_s(6, 6, 0);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
let t = c.g[A2][0] != 0u64;
c.mul_s(5, 5, 0);
if t {
if event_due(c) { deliver_event_here(c, 0x002f2e30); } continue 'l_2f2e30;
} else {
break 'b_2f2e40;
}
}
}
}
c.cycles += 124;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xa0c));
c.g[T0][0] = 0x10000;
c.g[T0][0] = c.g[T0][0] | 0x8fcd;
c.g[A2][0] = 0xe90000;
c.mult(0, 2, 2, 8);
c.g[A2][0] = c.g[A2][0] | 0x507c;
c.g[A3][0] = 0x7f0000;
c.g[T1][0] = 0x3f800000;
c.g[A3][0] = c.g[A3][0] | 0xffff;
c.sw((c.g[T2][0] as u32).wrapping_add(0x20), 0u64);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 10);
c.g[V1][0] = 0x1;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0x11e8));
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0x11ec));
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.swc1(6, (c.g[SP][0] as u32));
c.sw((c.g[GP][0] as u32).wrapping_add(0xa0c), c.g[V0][0]);
c.sub_s(1, 1, 2);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x4700));
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0xa0c));
c.g[A1][0] = sx((c.g[T4][0] as u32).wrapping_add(0x4710));
c.sw((c.g[T2][0] as u32).wrapping_add(0x4), c.g[V1][0]);
c.sub_s(0, 0, 10);
c.g[AT][0] = 0x43b40000;
c.mtc1(1, 11);
c.mul_s(1, 1, 0);
c.add_s(2, 2, 1);
c.swc1(2, (c.g[T2][0] as u32).wrapping_add(0x24));
c.ldl(14, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(14, (c.g[SP][0] as u32));
c.sdl(14, (c.g[T2][0] as u32).wrapping_add(0x17));
c.sdr(14, (c.g[T2][0] as u32).wrapping_add(0x10));
c.ldl(14, (c.g[T5][0] as u32).wrapping_add(0x7));
c.ldr(14, (c.g[T5][0] as u32));
c.sdl(14, (c.g[T2][0] as u32).wrapping_add(0xf));
c.sdr(14, (c.g[T2][0] as u32).wrapping_add(0x8));
c.g[V1][0] = 0x4a0000;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5960));
c.ldl(13, (c.g[V0][0] as u32).wrapping_add(0x7));
c.ldr(13, (c.g[V0][0] as u32));
c.sdl(13, (c.g[T2][0] as u32).wrapping_add(0x1f));
c.sdr(13, (c.g[T2][0] as u32).wrapping_add(0x18));
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xa0c));
c.lwc1(8, (c.g[A0][0] as u32).wrapping_add(0xc));
c.mult(0, 3, 2, 8);
c.lwc1(9, (c.g[A1][0] as u32).wrapping_add(0xc));
c.lwc1(5, (c.g[A0][0] as u32).wrapping_add(0x4));
c.lwc1(2, (c.g[A0][0] as u32).wrapping_add(0x8));
c.lwc1(3, (c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.lwc1(7, (c.g[T3][0] as u32).wrapping_add(0x4700));
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.lwc1(4, (c.g[A1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.lwc1(6, (c.g[T4][0] as u32).wrapping_add(0x4710));
c.sw((c.g[GP][0] as u32).wrapping_add(0xa0c), c.g[V0][0]);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0xa0c));
c.sub_s(0, 0, 10);
c.sub_s(1, 10, 0);
c.mul_s(9, 9, 0);
c.mul_s(6, 6, 0);
c.mul_s(8, 8, 1);
c.mul_s(7, 7, 1);
c.mul_s(5, 5, 1);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mul_s(2, 2, 1);
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x2c));
c.mul_s(3, 3, 0);
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x20));
c.mul_s(4, 4, 0);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x24));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
c.add_s(7, 7, 6);
c.add_s(5, 5, 3);
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
c.add_s(2, 2, 4);
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x38));
c.add_s(8, 8, 9);
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x10));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x14));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x1c));
c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x3c));
c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x17));
c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x10));
c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x1f));
c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x18));
c.sdl(4, (c.g[T2][0] as u32).wrapping_add(0x2f));
c.sdr(4, (c.g[T2][0] as u32).wrapping_add(0x28));
c.sdl(5, (c.g[T2][0] as u32).wrapping_add(0x37));
c.sdr(5, (c.g[T2][0] as u32).wrapping_add(0x30));
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xa0c));
c.mult(0, 3, 2, 8);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.sw((c.g[GP][0] as u32).wrapping_add(0xa0c), c.g[V0][0]);
c.mult(0, 3, 2, 8);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0xa0c));
c.sub_s(0, 0, 10);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.mul_s(0, 0, 11);
c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x38));
c.sw((c.g[GP][0] as u32).wrapping_add(0xa0c), c.g[V0][0]);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x1228));
c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0xa0c));
c.neg_s(1, 0);
c.sw((c.g[T2][0] as u32), 0u64);
c.sub_s(2, 2, 10);
c.sw((c.g[T2][0] as u32).wrapping_add(0x40), 0u64);
c.sw((c.g[T2][0] as u32).wrapping_add(0x44), 0u64);
c.sub_s(0, 0, 1);
c.sw((c.g[T2][0] as u32).wrapping_add(0x48), 0u64);
c.mul_s(0, 0, 2);
c.add_s(1, 1, 0);
c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0x3c));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.pc = t; return;
}

/// `void SNDMIDI_ProcessEvent(int seq)` (0x3b94b0): runs the MIDI sequence
/// `seq`'s next event record (`ev, status, data`): note on/off, control
/// change, pitch bend (through their handlers), program change (stored in the
/// channel), or the end-of-track meta event (stops the sequence once its notes
/// are finished, else rewinds and retries); loops while the delta `ev` is 0
/// and the sequence type (+0x28) is 3. Resumable dispatcher.
///
/// NOT REGISTERED: its test fixture never reached the handler calls in the
/// generated body (no seed made the NotesFinished call), so the port has no
/// passing coverage; kept for a later wave.
#[allow(dead_code)]
pub fn snd_midi_process_event(c: &mut Cpu) {
    if c.pc != 0x003b94b0 {
        return ee::s13::f_003b94b0__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003b94b0;
    loop {
        match pc {
            0x003b94b0 => {
                c.cycles += 26;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[V0][0] = 0x510000;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5ee0));
                c.g[S1][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = 0x70;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.mult(0, 4, 17, 4);
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[A2][0] = 0xf0;
                c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x27));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sb((c.g[S0][0] as u32).wrapping_add(0x27), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = c.lbu((c.g[V1][0] as u32));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.g[A0][0] = c.g[V0][0] & 0xf0;
                c.g[A3][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x1));
                let t = c.g[A0][0] != c.g[A2][0]; 
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                if t { pc = 0x003b9544; continue; }
                pc = 0x003b9518; continue;
            }
            0x003b9518 => {
                c.cycles += 3;
                c.g[V0][0] = 0x2f;
                
                if c.g[A3][0] != c.g[V0][0] { c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x27)); pc = 0x003b9548; continue; }
                pc = 0x003b9524; continue;
            }
            0x003b9524 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003b952c);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x003b8ee8, f: ee::s13::SNDMIDI_NotesFinished }.call(c, 0x003b952c)?;
                
                if c.g[V0][0] == 0u64 { c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30)); pc = 0x003b96a4; continue; }
                pc = 0x003b9534; continue;
            }
            0x003b9534 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003b953c);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                Callee { addr: 0x003b9920, f: ee::s13::SNDMIDI_Stop }.call(c, 0x003b953c)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t { pc = 0x003b96a4; continue; }
                pc = 0x003b9544; continue;
            }
            0x003b9544 => {
                c.cycles += 1;
                c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x27));
                pc = 0x003b9548; continue;
            }
            0x003b9548 => {
                c.cycles += 2;
                
                if c.g[V0][0] != 0u64 { c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30)); pc = 0x003b96a4; continue; }
                pc = 0x003b9550; continue;
            }
            0x003b9550 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                if t { pc = 0x003b9568; continue; }
                pc = 0x003b9558; continue;
            }
            0x003b9558 => {
                c.cycles += 3;
                c.g[V0][0] = 0x3;
                
                if c.g[V1][0] != c.g[V0][0] { c.sb((c.g[S0][0] as u32).wrapping_add(0x27), c.g[S2][0]); pc = 0x003b96a0; continue; }
                pc = 0x003b9564; continue;
            }
            0x003b9564 => {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                pc = 0x003b9568; continue;
            }
            0x003b9568 => {
                c.cycles += 16;
                c.g[A2][0] = 0xb0;
                c.g[S2][0] = c.lbu((c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                c.g[A0][0] = c.lbu((c.g[V0][0] as u32));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.g[T0][0] = c.g[A0][0] & 0xf;
                c.g[V1][0] = sx((c.g[T0][0] as u32) << 2);
                c.g[A3][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                c.g[A0][0] = c.g[A0][0] & 0xf0;
                let t = c.g[A0][0] == c.g[A2][0]; 
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                if t { pc = 0x003b962c; continue; }
                pc = 0x003b95a8; continue;
            }
            0x003b95a8 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 177i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0x80;
                if t { pc = 0x003b95cc; continue; }
                pc = 0x003b95b4; continue;
            }
            0x003b95b4 => {
                c.cycles += 2;
                let t = c.g[A0][0] == c.g[V0][0]; 
                c.g[V0][0] = 0x90;
                if t { pc = 0x003b9614; continue; }
                pc = 0x003b95bc; continue;
            }
            0x003b95bc => {
                c.cycles += 2;
                let t = c.g[A0][0] == c.g[V0][0]; 
                c.g[A2][0] = c.g[T0][0];
                if t { pc = 0x003b9604; continue; }
                pc = 0x003b95c4; continue;
            }
            0x003b95c4 => {
                c.cycles += 2;
                let t = true; 
                
                if t { pc = 0x003b9694; continue; }
                pc = 0x003b95cc; continue;
            }
            0x003b95cc => {
                c.cycles += 3;
                c.g[V0][0] = 0xe0;
                let t = c.g[A0][0] == c.g[V0][0]; 
                c.g[V0][0] = ((c.g[A0][0] as i64) < 225i64) as u64;
                if t { pc = 0x003b9640; continue; }
                pc = 0x003b95d8; continue;
            }
            0x003b95d8 => {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = 0xc0;
                if t { pc = 0x003b95f0; continue; }
                pc = 0x003b95e0; continue;
            }
            0x003b95e0 => {
                c.cycles += 2;
                
                if c.g[A0][0] == c.g[V0][0] { c.sb((c.g[A1][0] as u32).wrapping_add(0x3), c.g[A3][0]); pc = 0x003b9694; continue; }
                pc = 0x003b95e8; continue;
            }
            0x003b95e8 => {
                c.cycles += 2;
                let t = true; 
                
                if t { pc = 0x003b9694; continue; }
                pc = 0x003b95f0; continue;
            }
            0x003b95f0 => {
                c.cycles += 3;
                c.g[V0][0] = 0xf0;
                let t = c.g[A0][0] == c.g[V0][0]; 
                c.g[V0][0] = 0x2f;
                if t { pc = 0x003b9654; continue; }
                pc = 0x003b95fc; continue;
            }
            0x003b95fc => {
                c.cycles += 2;
                let t = true; 
                
                if t { pc = 0x003b9694; continue; }
                pc = 0x003b9604; continue;
            }
            0x003b9604 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003b960c);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x003b8f78, f: ee::s13::SNDMIDI_NoteOn }.call(c, 0x003b960c)?;
                let t = true; 
                
                if t { pc = 0x003b9694; continue; }
                pc = 0x003b9614; continue;
            }
            0x003b9614 => {
                c.cycles += 6;
                c.g[A1][0] = c.g[T0][0];
                c.g[A2][0] = c.g[A3][0];
                c.g[RA][0] = sx(0x003b9624);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x003b9140, f: ee::s13::SNDMIDI_NoteOff }.call(c, 0x003b9624)?;
                let t = true; 
                
                if t { pc = 0x003b9694; continue; }
                pc = 0x003b962c; continue;
            }
            0x003b962c => {
                c.cycles += 5;
                c.g[A2][0] = c.g[T0][0];
                c.g[RA][0] = sx(0x003b9638);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x003b91e0, f: ee::s13::SNDMIDI_ControlChange }.call(c, 0x003b9638)?;
                let t = true; 
                
                if t { pc = 0x003b9694; continue; }
                pc = 0x003b9640; continue;
            }
            0x003b9640 => {
                c.cycles += 5;
                c.g[A2][0] = c.g[T0][0];
                c.g[RA][0] = sx(0x003b964c);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x003b93c8, f: ee::s13::SNDMIDI_PitchBend }.call(c, 0x003b964c)?;
                let t = true; 
                
                if t { pc = 0x003b9694; continue; }
                pc = 0x003b9654; continue;
            }
            0x003b9654 => {
                c.cycles += 2;
                let t = c.g[A3][0] != c.g[V0][0]; 
                
                if t { pc = 0x003b9694; continue; }
                pc = 0x003b965c; continue;
            }
            0x003b965c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003b9664);
                c.g[A0][0] = c.g[S1][0];
                Callee { addr: 0x003b8ee8, f: ee::s13::SNDMIDI_NotesFinished }.call(c, 0x003b9664)?;
                
                if c.g[V0][0] == 0u64 { c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8)); pc = 0x003b967c; continue; }
                pc = 0x003b966c; continue;
            }
            0x003b966c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003b9674);
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                Callee { addr: 0x003b9920, f: ee::s13::SNDMIDI_Stop }.call(c, 0x003b9674)?;
                let t = true; 
                
                if t { pc = 0x003b9694; continue; }
                pc = 0x003b967c; continue;
            }
            0x003b967c => {
                c.cycles += 6;
                c.g[S2][0] = 0x1;
                c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x27));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sb((c.g[S0][0] as u32).wrapping_add(0x27), c.g[V0][0]);
                pc = 0x003b9694; continue;
            }
            0x003b9694 => {
                c.cycles += 2;
                
                if c.g[S2][0] == 0u64 { c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x28)); if back_edge_unwinds(c, 0x003b9558) { return Ok(()); } pc = 0x003b9558; continue; }
                pc = 0x003b969c; continue;
            }
            0x003b969c => {
                c.cycles += 1;
                c.sb((c.g[S0][0] as u32).wrapping_add(0x27), c.g[S2][0]);
                pc = 0x003b96a0; continue;
            }
            0x003b96a0 => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                pc = 0x003b96a4; continue;
            }
            0x003b96a4 => {
                c.cycles += 5;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003b94b0, 0x003b96b8); return Ok(()); }
        }
    }
    });
}

/// `void SND_ParsePtHeader(.., src, hdr, out, extra)` (0x3bbda0): parses a
/// patch (`pt`) header's tag list at `src + 8`: clears `hdr` (4 bytes), `out`
/// (0x68) and `extra` (0x1c), sets the default patch params, collects the
/// common (0..42) and platform (0x80..0xad) tags until the end tag 0xfe,
/// records the type-20 tag pointers/sizes into `out`, then fills `hdr` and
/// `extra` from the platform values (channel count, rate, flags) and copies
/// the channel table row. Resumable dispatcher.
pub fn snd_parse_pt_header(c: &mut Cpu) {
    if c.pc != 0x003bbda0 {
        return ee::s13::f_003bbda0__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003bbda0;
    loop {
        match pc {
            0x003bbda0 => {
                c.cycles += 38;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x3b0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x390));
                c.g[S0][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x380));
                c.g[S1][0] = c.g[A2][0];
                c.g[S2][0] = c.g[A3][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x300), c.g[T0][0]);
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x370));
                c.g[A0][0] = c.g[S1][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x360));
                c.g[A1][0] = 0u64;
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x350));
                c.g[A2][0] = 0x4;
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x340));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x330));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x320));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x310), c.g[RA][0]);
                c.g[RA][0] = sx(0x003bbdf0);
                c.g[S4][0] = 0u64;
                call_ptr(c, 0x00416210, 0x003bbdf0)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x003bbe00);
                c.g[A2][0] = 0x68;
                call_ptr(c, 0x00416210, 0x003bbe00)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x300));
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x003bbe10);
                c.g[A2][0] = 0x1c;
                call_ptr(c, 0x00416210, 0x003bbe10)?;
                c.g[RA][0] = sx(0x003bbe18);
                c.g[A0][0] = c.g[S2][0];
                Callee { addr: 0x003c7218, f: ee::s13::SND_InitPatchParams }.call(c, 0x003bbe18)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x2e0), c.g[S0][0]);
                c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
                c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2e0));
                c.g[T0][0] = 0x2a;
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x158));
                pc = 0x003bbe38; continue;
            }
            0x003bbe38 => {
                c.cycles += 7;
                c.sw((c.g[V0][0] as u32), 0u64);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                let t = (c.g[T0][0] as i64) >= 0; 
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                if t { if back_edge_unwinds(c, 0x003bbe38) { return Ok(()); } pc = 0x003bbe38; continue; }
                pc = 0x003bbe54; continue;
            }
            0x003bbe54 => {
                c.cycles += 3;
                c.g[T0][0] = 0x2d;
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(0xb4));
                pc = 0x003bbe60; continue;
            }
            0x003bbe60 => {
                c.cycles += 7;
                c.sw((c.g[V0][0] as u32), 0u64);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
                let t = (c.g[T0][0] as i64) >= 0; 
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                if t { if back_edge_unwinds(c, 0x003bbe60) { return Ok(()); } pc = 0x003bbe60; continue; }
                pc = 0x003bbe7c; continue;
            }
            0x003bbe7c => {
                c.cycles += 12;
                c.g[V0][0] = 0x1;
                c.g[V1][0] = 0xbb80;
                c.g[A0][0] = 0xa;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x168), c.g[V0][0]);
                c.g[FP][0] = 0x14;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x170), c.g[V1][0]);
                c.g[S7][0] = 0xfe;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x1e0), c.g[A0][0]);
                c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x174), 0u64);
                let t = true; 
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x58));
                if t { pc = 0x003bbf40; continue; }
                pc = 0x003bbeac; continue;
            }
            0x003bbeac => {
                c.cycles += 1;
                pc = 0x003bbeb0; continue;
            }
            0x003bbeb0 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 43i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                if t { pc = 0x003bbefc; continue; }
                pc = 0x003bbebc; continue;
            }
            0x003bbebc => {
                c.cycles += 11;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2e8));
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V1][0] as u32), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2e4));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2ec));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2e4));
                let t = c.g[V1][0] != c.g[FP][0]; 
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2ec));
                if t { pc = 0x003bbf40; continue; }
                pc = 0x003bbee8; continue;
            }
            0x003bbee8 => {
                c.cycles += 5;
                c.sw((c.g[S0][0] as u32).wrapping_sub(0x10), c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2f0));
                c.sw((c.g[S0][0] as u32), c.g[V1][0]);
                let t = true; 
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                if t { pc = 0x003bbf40; continue; }
                pc = 0x003bbefc; continue;
            }
            0x003bbefc => {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x80));
                c.g[V0][0] = (c.g[V1][0] < 0x2eu64) as u64;
                
                if c.g[V0][0] != 0u64 { c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2e8)); pc = 0x003bbf1c; continue; }
                pc = 0x003bbf0c; continue;
            }
            0x003bbf0c => {
                c.cycles += 2;
                let t = c.g[A0][0] != c.g[S7][0]; 
                c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x168));
                if t { pc = 0x003bbf40; continue; }
                pc = 0x003bbf14; continue;
            }
            0x003bbf14 => {
                c.cycles += 2;
                let t = true; 
                c.g[A3][0] = 0x4;
                if t { pc = 0x003bbf58; continue; }
                pc = 0x003bbf1c; continue;
            }
            0x003bbf1c => {
                c.cycles += 9;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sw((c.g[V1][0] as u32), c.g[A0][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2e4));
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2ec));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x80));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.sw((c.g[V0][0] as u32), c.g[V1][0]);
                pc = 0x003bbf40; continue;
            }
            0x003bbf40 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003bbf48);
                c.g[A0][0] = c.g[S5][0];
                Callee { addr: 0x003c7388, f: ee::s13::SND_ParseNextTag }.call(c, 0x003bbf48)?;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2e4));
                if t { if back_edge_unwinds(c, 0x003bbeb0) { return Ok(()); } pc = 0x003bbeb0; continue; }
                pc = 0x003bbf50; continue;
            }
            0x003bbf50 => {
                c.cycles += 2;
                c.g[A1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x168));
                c.g[A3][0] = 0x4;
                pc = 0x003bbf58; continue;
            }
            0x003bbf58 => {
                c.cycles += 14;
                c.g[A0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.g[A2][0] = 0x3;
                c.g[V1][0] = c.lhu((c.g[SP][0] as u32).wrapping_add(0x170));
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x174));
                c.sb((c.g[S1][0] as u32).wrapping_add(0x2), c.g[A1][0]);
                c.sb((c.g[S1][0] as u32).wrapping_add(0x3), c.g[A0][0]);
                c.sh((c.g[S1][0] as u32), c.g[V1][0]);
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x300));
                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                c.sh((c.g[S2][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                c.sb((c.g[S2][0] as u32).wrapping_add(0x7), c.g[A2][0]);
                c.g[V1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x2));
                let t = c.g[V1][0] == 0u64; 
                c.g[T0][0] = 0u64;
                if t { pc = 0x003bbfd0; continue; }
                pc = 0x003bbf90; continue;
            }
            0x003bbf90 => {
                c.cycles += 4;
                c.g[V0][0] = 0x510000;
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5438));
                c.g[A1][0] = 0xc;
                pc = 0x003bbfa0; continue;
            }
            0x003bbfa0 => {
                c.cycles += 12;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.mult(0, 2, 2, 5);
                c.g[V1][0] = sx((c.g[T0][0] as u32) << 1);
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                c.g[V0][0] = c.lhu((c.g[V1][0] as u32));
                c.sh((c.g[A0][0] as u32), c.g[V0][0]);
                c.g[V1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x2));
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
                if t { if back_edge_unwinds(c, 0x003bbfa0) { return Ok(()); } pc = 0x003bbfa0; continue; }
                pc = 0x003bbfd0; continue;
            }
            0x003bbfd0 => {
                c.cycles += 12;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x390));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x380));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x370));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x360));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x350));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x340));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x330));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x320));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x310));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3b0));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003bbda0, 0x003bc000); return Ok(()); }
        }
    }
    });
}

/// `int SND_SpuMemAlloc(int type, int size)` (0x3c4b70): allocates SPU memory
/// from the sorted block list. Type 0x100 goes to the IOP heap
/// (`sceSifAllocIopHeap(size + 0x40)`); otherwise the size is rounded up to 64-byte
/// units and the first gap (clamped by `SND_SpuMemClampBlock`) that fits it
/// gets a new block; returns its address (`start << 6`) or 0. Resumable
/// dispatcher.
pub fn snd_spu_mem_alloc(c: &mut Cpu) {
    if c.pc != 0x003c4b70 {
        return ee::s13::f_003c4b70__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003c4b70;
    loop {
        match pc {
            0x003c4b70 => {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.g[V0][0] = 0x100;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S1][0] = 0u64;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S2][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[A0][0] != c.g[V0][0]; 
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                if t { pc = 0x003c4bb4; continue; }
                pc = 0x003c4ba4; continue;
            }
            0x003c4ba4 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x003c4bac);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x40));
                call_ptr(c, 0x0042aba0, 0x003c4bac)?;
                let t = true; 
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                if t { pc = 0x003c4d50; continue; }
                pc = 0x003c4bb4; continue;
            }
            0x003c4bb4 => {
                c.cycles += 9;
                c.g[V0][0] = 0x510000;
                c.g[V1][0] = 0x510000;
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5300));
                c.g[S5][0] = c.g[V0][0];
                c.g[A0][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x1a98));
                c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_sub(0x56f0));
                c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3f));
                if t { pc = 0x003c4c18; continue; }
                pc = 0x003c4bd8; continue;
            }
            0x003c4bd8 => {
                c.cycles += 2;
                let t = c.g[A0][0] != 0u64; 
                c.g[S2][0] = sx(((c.g[S2][0] as u32) as i32 >> 6) as u32);
                if t { pc = 0x003c4c20; continue; }
                pc = 0x003c4be0; continue;
            }
            0x003c4be0 => {
                c.cycles += 8;
                c.g[A2][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x1a9a));
                c.g[A0][0] = c.g[SP][0];
                c.g[V0][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x1a9c));
                c.g[A1][0] = c.g[SP][0] | 0x4;
                c.g[V1][0] = c.g[A2][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[A2][0] as u32)));
                c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                pc = 0x003c4c00; continue;
            }
            0x003c4c00 => {
                c.cycles += 6;
                c.g[RA][0] = sx(0x003c4c08);
                Callee { addr: 0x003c4af8, f: ee::s13::SND_SpuMemClampBlock }.call(c, 0x003c4c08)?;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x5300));
                if t { pc = 0x003c4d20; continue; }
                pc = 0x003c4c18; continue;
            }
            0x003c4c18 => {
                c.cycles += 2;
                let t = true; 
                c.g[V0][0] = 0u64;
                if t { pc = 0x003c4d4c; continue; }
                pc = 0x003c4c20; continue;
            }
            0x003c4c20 => {
                c.cycles += 3;
                c.g[V0][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x1a98));
                let t = c.g[V0][0] == 0u64; 
                c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
                if t { pc = 0x003c4ce0; continue; }
                pc = 0x003c4c2c; continue;
            }
            0x003c4c2c => {
                c.cycles += 3;
                c.g[S0][0] = c.g[A3][0];
                c.g[S3][0] = 0xfffffffffffffffc;
                c.g[S6][0] = c.g[S5][0];
                pc = 0x003c4c38; continue;
            }
            0x003c4c38 => {
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1aa0));
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                let t = c.g[S1][0] != 0u64; 
                c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                if t { pc = 0x003c4c50; continue; }
                pc = 0x003c4c48; continue;
            }
            0x003c4c48 => {
                c.cycles += 2;
                let t = true; 
                c.g[V1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1a9a));
                if t { pc = 0x003c4c60; continue; }
                pc = 0x003c4c50; continue;
            }
            0x003c4c50 => {
                c.cycles += 4;
                c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                c.g[V0][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x2));
                c.g[V1][0] = c.lhu((c.g[A3][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                pc = 0x003c4c60; continue;
            }
            0x003c4c60 => {
                c.cycles += 11;
                c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                c.g[V0][0] = c.lhu((c.g[A2][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x003c4c7c);
                c.g[A1][0] = c.g[S4][0];
                Callee { addr: 0x003c4af8, f: ee::s13::SND_SpuMemClampBlock }.call(c, 0x003c4c7c)?;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                
                if c.g[V0][0] != 0u64 { c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1a98)); pc = 0x003c4cd0; continue; }
                pc = 0x003c4c8c; continue;
            }
            0x003c4c8c => {
                c.cycles += 4;
                c.g[A1][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x1a98));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5300));
                if t { pc = 0x003c4d1c; continue; }
                pc = 0x003c4c9c; continue;
            }
            0x003c4c9c => {
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1aa0));
                pc = 0x003c4ca0; continue;
            }
            0x003c4ca0 => {
                c.cycles += 10;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A0][0] = ((c.g[S1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                c.lwl(2, (c.g[V1][0] as u32).wrapping_sub(0x1));
                c.lwr(2, (c.g[V1][0] as u32).wrapping_sub(0x4));
                c.swl(2, (c.g[V1][0] as u32).wrapping_add(0x3));
                c.swr(2, (c.g[V1][0] as u32));
                
                if c.g[A0][0] != 0u64 { c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1aa0)); if back_edge_unwinds(c, 0x003c4ca0) { return Ok(()); } pc = 0x003c4ca0; continue; }
                pc = 0x003c4cc8; continue;
            }
            0x003c4cc8 => {
                c.cycles += 2;
                let t = true; 
                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x5300));
                if t { pc = 0x003c4d20; continue; }
                pc = 0x003c4cd0; continue;
            }
            0x003c4cd0 => {
                c.cycles += 4;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                if t { if back_edge_unwinds(c, 0x003c4c38) { return Ok(()); } pc = 0x003c4c38; continue; }
                pc = 0x003c4ce0; continue;
            }
            0x003c4ce0 => {
                c.cycles += 15;
                c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x5300));
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1aa0));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                c.g[A2][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x1a9c));
                c.g[A1][0] = c.g[S4][0];
                c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[V1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x2));
                c.g[A0][0] = c.g[SP][0];
                c.g[V0][0] = c.lhu((c.g[A3][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                let t = true; 
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                if t { if back_edge_unwinds(c, 0x003c4c00) { return Ok(()); } pc = 0x003c4c00; continue; }
                pc = 0x003c4d1c; continue;
            }
            0x003c4d1c => {
                c.cycles += 1;
                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x5300));
                pc = 0x003c4d20; continue;
            }
            0x003c4d20 => {
                c.cycles += 11;
                c.g[A1][0] = c.lhu((c.g[SP][0] as u32));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1aa0));
                c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.sh((c.g[A2][0] as u32).wrapping_add(0x2), c.g[S2][0]);
                c.sh((c.g[A2][0] as u32), c.g[A1][0]);
                c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x1a98));
                c.g[V0][0] = c.lw((c.g[SP][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.sh((c.g[A0][0] as u32).wrapping_add(0x1a98), c.g[V1][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                pc = 0x003c4d4c; continue;
            }
            0x003c4d4c => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                pc = 0x003c4d50; continue;
            }
            0x003c4d50 => {
                c.cycles += 9;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003c4b70, 0x003c4d74); return Ok(()); }
        }
    }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{
        check, check_paths_lohi, elf_image, fresh, run_generated, scramble, Rng, DATA, GAME_GP,
    };
    use std::collections::BTreeSet;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    fn oc1139_f(x: f32) -> u32 {
        x.to_bits()
    }

    /// A float in `[lo, hi)` as raw bits.
    fn oc1139_rnd(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        oc1139_f(lo + (hi - lo) * (r.next_u32() as f32 / 4_294_967_296.0))
    }

    // ---- cSplashManager_InitDrop / InitCrystal --------------------------

    /// The splash tuning floats (`gp + 0x1190..0x1230`: sizes, aspect
    /// differences, life spans, rotation changes) and the LCG seed, random.
    fn oc1139_splash_tuning(c: &mut Cpu, r: &mut Rng) {
        for off in (0x1190..0x1230u32).step_by(4) {
            let v = oc1139_rnd(r, -2.0, 30.0);
            c.bus.write32(GAME_GP + off, v);
        }
        let seed = r.next_u32();
        c.bus.write32(GAME_GP + 0xa0c, seed);
    }

    /// `cSplashManager_InitDrop`: both arms of the height clamp.
    #[test]
    fn c_splash_manager_init_drop_matches() {
        check_paths_lohi(
            ee::s09::f_002f2598__generated,
            c_splash_manager_init_drop,
            300,
            &[156, 157],
            &|c, seed| {
                let mut r = Rng(0x2f2598 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = GAME_GP as u64;
                oc1139_splash_tuning(c, &mut r);
                let (drop, pos, range) = (DATA + 0x100, DATA + 0x200, DATA + 0x300);
                for k in 0..16 {
                    let v = r.next_u32();
                    c.bus.write32(drop + 4 * k, v);
                }
                let (px, py) = (oc1139_rnd(&mut r, 0.0, 640.0), oc1139_rnd(&mut r, 0.0, 480.0));
                c.bus.write32(pos, px);
                c.bus.write32(pos + 4, py);
                let (rx, ry) = (oc1139_rnd(&mut r, 3.0, 40.0), oc1139_rnd(&mut r, 3.0, 40.0));
                c.bus.write32(range, rx);
                c.bus.write32(range + 4, ry);
                c.g[A0][0] = drop as u64;
                c.g[A1][0] = pos as u64;
                c.g[A2][0] = range as u64;
            },
        );
    }

    /// `cSplashManager_InitCrystal`: generation counts 0 (and negative: the
    /// shrink loop is skipped) up to 3.
    #[test]
    fn c_splash_manager_init_crystal_matches() {
        check_paths_lohi(
            ee::s09::f_002f2d70__generated,
            c_splash_manager_init_crystal,
            300,
            &[170, 175, 179, 183],
            &|c, seed| {
                let mut r = Rng(0x2f2d70 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = GAME_GP as u64;
                oc1139_splash_tuning(c, &mut r);
                let (crystal, pos) = (DATA + 0x100, DATA + 0x200);
                for k in 0..20 {
                    let v = r.next_u32();
                    c.bus.write32(crystal + 4 * k, v);
                }
                let (px, py) = (oc1139_rnd(&mut r, 0.0, 640.0), oc1139_rnd(&mut r, 0.0, 480.0));
                c.bus.write32(pos, px);
                c.bus.write32(pos + 4, py);
                c.g[A0][0] = crystal as u64;
                c.g[A1][0] = pos as u64;
                c.g[A2][0] = [0u64, 1, 2, 3, u64::MAX][seed as usize % 5];
            },
        );
    }

    // ---- gated callee fakes (own gate word and mailboxes in machine RAM) --

    /// Gate word: the fakes below only act when it holds [`OC1139_MAGIC`].
    const OC1139_CTRL: u32 = DATA + 0x20_0000;
    const OC1139_MAGIC: u32 = 0x1139_c0de;
    /// Result mailbox of the fake `SNDMIDI_NotesFinished`.
    const OC1139_NF_MAIL: u32 = 4;

    /// A fake for the callee at `$target` (override slot): logs the call with
    /// `fakes::fake`; with a non-zero `$mail` its result is the word at
    /// `OC1139_CTRL + $mail`.
    macro_rules! oc1139_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path, $mail:expr) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC1139_CTRL) != OC1139_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
                if $mail != 0 {
                    let w = c.bus.read32(OC1139_CTRL + $mail);
                    c.g[V0][0] = w as u64;
                }
            }
        };
    }

    oc1139_fake!(
        oc1139_fake_notes_finished,
        OC1139_PREV_NF,
        0x003b_8ee8,
        ee::s13::f_003b8ee8__generated,
        OC1139_NF_MAIL
    );
    oc1139_fake!(
        oc1139_fake_note_on,
        OC1139_PREV_NOTE_ON,
        0x003b_8f78,
        ee::s13::f_003b8f78__generated,
        0
    );
    oc1139_fake!(
        oc1139_fake_note_off,
        OC1139_PREV_NOTE_OFF,
        0x003b_9140,
        ee::s13::f_003b9140__generated,
        0
    );
    oc1139_fake!(
        oc1139_fake_control_change,
        OC1139_PREV_CC,
        0x003b_91e0,
        ee::s13::f_003b91e0__generated,
        0
    );
    oc1139_fake!(
        oc1139_fake_pitch_bend,
        OC1139_PREV_PB,
        0x003b_93c8,
        ee::s13::f_003b93c8__generated,
        0
    );
    oc1139_fake!(
        oc1139_fake_midi_stop,
        OC1139_PREV_STOP,
        0x003b_9920,
        ee::s13::f_003b9920__generated,
        0
    );
    oc1139_fake!(
        oc1139_fake_alloc_iop_heap,
        OC1139_PREV_IOP,
        0x0042_aba0,
        ee::s15::f_0042aba0__generated,
        0
    );

    /// Tag script of the fake `SND_ParseNextTag`: the count at `CTRL+0x100`,
    /// the next index at `+0x104`, then 16-byte records `{type, val, ptr,
    /// size}` from `+0x110`. The fake writes one into the caller's record
    /// (`a0 + 4..0x14`) and returns 1, or 0 once the script is used up.
    static OC1139_PREV_TAG: AtomicUsize = AtomicUsize::new(0);
    fn oc1139_fake_parse_next_tag(c: &mut Cpu) {
        if c.bus.read32(OC1139_CTRL) != OC1139_MAGIC {
            return fakes::fall_back(c, &OC1139_PREV_TAG, ee::s13::f_003c7388__generated);
        }
        let rec = c.g[A0][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x003c_7388, ra);
        let n = c.bus.read32(OC1139_CTRL + 0x100);
        let i = c.bus.read32(OC1139_CTRL + 0x104);
        if i < n {
            let t = OC1139_CTRL + 0x110 + 16 * i;
            for k in 0..4 {
                let w = c.bus.read32(t + 4 * k);
                c.bus.write32(rec + 4 + 4 * k, w);
            }
            c.bus.write32(OC1139_CTRL + 0x104, i + 1);
            c.g[V0][0] = 1;
        } else {
            c.g[V0][0] = 0;
        }
    }

    fn oc1139_hook() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B8EE8),
            oc1139_fake_notes_finished,
            &OC1139_PREV_NF,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B8F78),
            oc1139_fake_note_on,
            &OC1139_PREV_NOTE_ON,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B9140),
            oc1139_fake_note_off,
            &OC1139_PREV_NOTE_OFF,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B91E0),
            oc1139_fake_control_change,
            &OC1139_PREV_CC,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B93C8),
            oc1139_fake_pitch_bend,
            &OC1139_PREV_PB,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003B9920),
            oc1139_fake_midi_stop,
            &OC1139_PREV_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003C7388),
            oc1139_fake_parse_next_tag,
            &OC1139_PREV_TAG,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_0042ABA0),
            oc1139_fake_alloc_iop_heap,
            &OC1139_PREV_IOP,
        );
    }

    /// Cycle totals of the generated body over `seeds` fixtures.
    fn oc1139_paths(
        generated: fn(&mut Cpu),
        seeds: u32,
        fixture: &dyn Fn(&mut Cpu, u32),
    ) -> BTreeSet<u64> {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            fixture(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        seen
    }

    // ---- SND_ParsePtHeader ----------------------------------------------

    /// `src` is unused by the fake tag source (the header's tags come from
    /// the script); `hdr`, `out` and `extra` are separate buffers. Tags: common
    /// types (incl. 20, which records pointer and size into `out`), platform
    /// types 0x82 (channel count), 0x84 (rate), 0x85 (flags), 0xa0 (10),
    /// ignored types, and the end tag 0xfe.
    fn oc1139_pt_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3bbda0, 6);
        elf_image(c);
        c.bus.write32(OC1139_CTRL, OC1139_MAGIC);
        c.pc = 0x003b_bda0;
        let (src, hdr, out, extra) = (DATA + 0x3000, DATA + 0x4000, DATA + 0x4100, DATA + 0x4400);
        c.g[A1][0] = src as u64;
        c.g[A2][0] = hdr as u64;
        c.g[A3][0] = out as u64;
        c.g[T0][0] = extra as u64;
        for k in 0..0x10 {
            let v = r.next_u32();
            c.bus.write32(hdr + 4 * k, v);
            c.bus.write32(extra + 4 * k, v);
        }
        for k in 0..0x80 {
            let v = r.next_u32();
            c.bus.write32(out + 4 * k, v);
        }
        let n = 1 + seed % 10;
        c.bus.write32(OC1139_CTRL + 0x100, n);
        c.bus.write32(OC1139_CTRL + 0x104, 0);
        for j in 0..n {
            let kind = r.next_u32() % 10;
            let (ty, val) = match kind {
                0 | 1 => (20, r.next_u32()),
                2 => (r.next_u32() % 43, r.next_u32()),
                3 => (0x82, [1u32, 2, 3, 6, 13, 0, 200][(r.next_u32() % 7) as usize]),
                4 => (0x84, r.next_u32()),
                5 => (0x85, r.next_u32()),
                6 => (0xa0, r.next_u32()),
                7 => (0x80 + r.next_u32() % 46, r.next_u32()),
                8 => (0x2b + r.next_u32() % 0x50, r.next_u32()),
                _ => (0xfe, r.next_u32()),
            };
            let t = OC1139_CTRL + 0x110 + 16 * j;
            c.bus.write32(t, ty);
            c.bus.write32(t + 4, val);
            c.bus.write32(t + 8, r.next_u32());
            c.bus.write32(t + 12, r.next_u32() & 0xffff);
        }
    }

    /// `SND_ParsePtHeader` over random tag lists (with and without the end
    /// tag, unwinding tag reads) and the channel-table copy.
    #[test]
    fn snd_parse_pt_header_matches() {
        oc1139_hook();
        check(
            ee::s13::f_003bbda0__generated,
            snd_parse_pt_header,
            300,
            &|c, seed| oc1139_pt_fixture(c, seed),
        );
        let seen = oc1139_paths(ee::s13::f_003bbda0__generated, 300, &|c, seed| {
            oc1139_pt_fixture(c, seed)
        });
        assert!(seen.len() >= 5, "pt header arms not covered: {seen:?}");
    }

    // ---- SND_SpuMemAlloc ------------------------------------------------

    /// The SPU memory block list state (`0x50ad00`: count +0x1a98, base
    /// +0x1a9a, end +0x1a9c, limit +0x1a9e, list +0x1aa0) and the maximum
    /// block count (`0x50a910`).
    const OC1139_SPU: u32 = 0x0050_ad00;

    /// A sorted block list of `seed % 5` blocks with random gaps, a request
    /// of random size (type 0x100 for one seed in eight: the IOP heap), and a
    /// block-count limit that is sometimes already reached.
    fn oc1139_spu_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3c4b70, 1);
        elf_image(c);
        c.bus.write32(OC1139_CTRL, OC1139_MAGIC);
        c.pc = 0x003c_4b70;
        let list = DATA + 0x6000;
        let n = seed % 5;
        let base = 0x100 + r.next_u32() % 0x40;
        let mut at = base;
        for i in 0..n {
            let start = at + r.next_u32() % 0x300;
            let size = 1 + r.next_u32() % 0x200;
            c.bus.write16(list + 4 * i, start as u16);
            c.bus.write16(list + 4 * i + 2, size as u16);
            at = start + size;
        }
        let end = at + r.next_u32() % 0x400;
        let limit = if seed % 4 == 0 { at + (end - at) / 2 } else { end + 0x100 };
        c.bus.write16(OC1139_SPU + 0x1a98, n as u16);
        c.bus.write16(OC1139_SPU + 0x1a9a, base as u16);
        c.bus.write16(OC1139_SPU + 0x1a9c, end as u16);
        c.bus.write16(OC1139_SPU + 0x1a9e, limit as u16);
        c.bus.write32(OC1139_SPU + 0x1aa0, list);
        c.bus
            .write16(0x0050_a910, if seed % 7 == 0 { n as u16 } else { 8 });
        c.g[A0][0] = if seed % 8 == 3 { 0x100 } else { 0 };
        c.g[A1][0] = (r.next_u32() % (0x300 * 64)) as u64;
    }

    /// `SND_SpuMemAlloc`: the IOP-heap type, a full list, an empty list, a
    /// fit in a gap (the shift loop) and after the last block, and no fit.
    #[test]
    fn snd_spu_mem_alloc_matches() {
        oc1139_hook();
        check(
            ee::s13::f_003c4b70__generated,
            snd_spu_mem_alloc,
            300,
            &|c, seed| oc1139_spu_fixture(c, seed),
        );
        let seen = oc1139_paths(ee::s13::f_003c4b70__generated, 300, &|c, seed| {
            oc1139_spu_fixture(c, seed)
        });
        assert!(seen.len() >= 5, "spu alloc arms not covered: {seen:?}");
    }
}
