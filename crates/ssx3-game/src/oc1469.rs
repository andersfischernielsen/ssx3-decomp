//! Staging module oc1469 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1469`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0038_6e78, draw_shadow_volume, "PS2GraphicsMan_v89")];

/// `DrawShadowVolume` (0x00386e78): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn draw_shadow_volume(c: &mut Cpu) {
    if c.pc != 0x00386e78 {
        return ee::s12::f_00386e78__generated(c);
    }
    guest(c, |c| {
'b_386f4c: {
c.cycles += 48;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
c.g[T9][0] = c.g[A0][0];
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[V0][0] = 0x500000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[A0][0] = 0xfffffffffffffc1f;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.g[S6][0] = c.g[A1][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[A3][0] = 0xfffffffffffffc3f;
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x100));
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1420));
c.ldl(8, (c.g[T3][0] as u32).wrapping_add(0x7));
c.ldr(8, (c.g[T3][0] as u32));
c.ldl(9, (c.g[T3][0] as u32).wrapping_add(0xf));
c.ldr(9, (c.g[T3][0] as u32).wrapping_add(0x8));
c.g[T2][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x10));
c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
c.sdr(8, (c.g[V1][0] as u32));
c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
c.sdr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A2][0]);
c.g[T0][0] = 0xffffffffc0000000;
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | 0x60;
c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.g[V0][0] = c.g[V0][0] | 0xc0;
c.sw((c.g[A1][0] as u32), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x6b90));
if c.g[V1][0] != 0u64 {
c.g[T0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
break 'b_386f4c;
} else {
c.cycles += 5;
c.g[A0][0] = c.g[T9][0];
c.g[RA][0] = sx(0x00386f44);
c.sq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x00386f44)?;
c.lq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[T0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
break 'b_386f4c;
}
}
'b_387264: {
'b_38725c: {
c.cycles += 192;
c.g[V1][0] = 0xffffffffffbf0000;
c.g[V1][0] = c.g[V1][0] | 0xffff;
c.g[A0][0] = 0x400000;
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x4));
c.g[A1][0] = 0xfffffffffe7f0000;
c.g[A1][0] = c.g[A1][0] | 0xffff;
c.g[S0][0] = 0x1000000;
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.g[A2][0] = 0xffffffffffcf0000;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[A2][0] = c.g[A2][0] | 0xffff;
c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A3][0] = 0xfffffffffff00000;
c.g[A3][0] = c.g[A3][0] | 0xfff;
c.g[T0][0] = 0xffffffffffffff83;
c.g[A0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
c.g[V0][0] = c.g[V0][0] | c.g[S0][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
c.g[V0][0] = c.g[V0][0] | 0x14;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x14e4));
c.g[A0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V1][0] = sx((c.g[T9][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xf50));
c.sh((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
c.g[A0][0] = sx((c.g[T9][0] as u32).wrapping_add((c.g[A0][0] as u32)));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x00387000);
c.sq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
call_ptr(c, t, 0x00387000)?;
c.lq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S5][0] = c.g[V0][0];
c.g[FP][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a94));
c.g[A1][0] = 0u64;
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a84));
c.g[A0][0] = c.g[FP][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a80));
c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V1][0]);
c.g[A2][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a00));
c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[A2][0]);
c.g[RA][0] = sx(0x00387034);
c.g[A2][0] = 0x30;
call_ptr(c, 0x003e6448, 0x00387034)?;
c.g[T2][0] = 0xffffffffffffffff;
c.lq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[V1][0] = ((c.g[T2][0] as i64) < (c.g[S5][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x3f));
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[S5][0]; }
c.g[A0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
c.g[A1][0] = 0x30000000;
c.g[V1][0] = c.g[A3][0] << 32;
c.g[A0][0] = c.g[A0][0] | c.g[A1][0];
c.g[V0][0] = c.g[V0][0] << 16;
c.g[V1][0] = c.g[V1][0] >> 32;
c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[A2][0] = 0x6;
c.sw((c.g[FP][0] as u32), c.g[A1][0]);
c.g[V0][0] = 0xffffffffffffffff;
c.g[V0][0] = c.g[V0][0] << 40;
c.g[V0][0] = c.g[V0][0] >> 8;
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[A2][0]);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sw((c.g[FP][0] as u32).wrapping_add(0xc), c.g[A3][0]);
c.g[T3][0] = 0x1;
c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
c.g[T1][0] = 0xc;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[T4][0] = 0x4c;
c.sw((c.g[FP][0] as u32).wrapping_add(0x8), c.g[T0][0]);
c.g[A1][0] = 0x4e;
c.g[S0][0] = c.g[V0][0] | c.g[S0][0];
c.sw((c.g[FP][0] as u32).wrapping_add(0x20), c.g[T1][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
c.g[S0][0] = c.g[S0][0] << 32;
c.sw((c.g[FP][0] as u32).wrapping_add(0x18), c.g[T3][0]);
c.g[V0][0] = 0x8000;
c.g[V0][0] = c.g[V0][0] << 17;
c.sw((c.g[FP][0] as u32).wrapping_add(0x1c), c.g[T3][0]);
c.g[S0][0] = c.g[S0][0] >> 32;
c.sw((c.g[FP][0] as u32).wrapping_add(0x24), 0u64);
c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
c.sd((c.g[A0][0] as u32).wrapping_add(0x30), c.g[V1][0]);
c.g[A3][0] = 0x30000;
c.g[T0][0] = 0x47;
c.g[A2][0] = 0xffffffffffff0000;
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.g[A2][0] = c.g[A2][0] | 0xfffb;
c.g[T1][0] = 0x8000;
c.g[T1][0] = c.g[T1][0] << 45;
c.g[T5][0] = 0xe;
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T4][0]);
c.g[T6][0] = 0x50000000;
c.g[T7][0] = 0x4;
c.g[T4][0] = 0x1000000;
c.g[T4][0] = c.g[T4][0] | 0x101;
c.g[T4][0] = c.g[T4][0] << 32;
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[S0][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[A3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x2c));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A3][0] as u32)));
c.sw((c.g[FP][0] as u32), c.g[V1][0]);
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.g[A0][0] = c.g[A0][0] & c.g[A2][0];
c.g[V1][0] = ((c.g[T2][0] as i64) < (c.g[A1][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[A1][0]; }
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[A0][0]);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.sd((c.g[A3][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T5][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.sd((c.g[V1][0] as u32), 0u64);
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V1][0] = ((c.g[T2][0] as i64) < (c.g[A0][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[A0][0]; }
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[V0][0] = c.g[V0][0] & 0xff;
c.g[V0][0] = c.g[V0][0] | c.g[T6][0];
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[T3][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.sw((c.g[FP][0] as u32).wrapping_add(0x24), c.g[T7][0]);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x28), 0u64);
c.lq(2, (c.g[T9][0] as u32).wrapping_add(0x5840));
c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lw((c.g[FP][0] as u32));
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32), c.g[V1][0]);
c.lq(2, (c.g[T9][0] as u32).wrapping_add(0x5850));
c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), 0u64);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
c.sw((c.g[FP][0] as u32), c.g[V1][0]);
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.sd((c.g[A0][0] as u32), c.g[T4][0]);
c.g[T2][0] = ((c.g[T2][0] as i64) < (c.g[A1][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[T2][0] != 0 { c.g[V0][0] = c.g[A1][0]; }
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x28));
c.g[V0][0] = c.g[V0][0] & 0xff;
c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x24));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
let t = c.g[A0][0] == 0u64;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
if t {
break 'b_38725c;
} else {
c.cycles += 4;
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.g[V1][0] = 0x6c000000;
let t = true;
c.g[V0][0] = c.g[A2][0] | c.g[V0][0];
if t {
break 'b_387264;
} else {
break 'b_38725c;
}
}
}
c.cycles += 2;
c.g[V0][0] = c.g[A2][0] | c.g[V0][0];
c.g[V1][0] = 0x6c000000;
break 'b_387264;
}
'b_3876d8: {
'b_3876d0: {
c.cycles += 279;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[A1][0] as u32), c.g[V0][0]);
c.g[S1][0] = 0xfff0000;
c.g[S1][0] = c.g[S1][0] | 0xffff;
c.mtc1(0, 20);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.g[A0][0] = c.g[T9][0];
c.g[T2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
c.g[A1][0] = c.g[S6][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A3][0] = 0u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
c.g[T0][0] = 0x1;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
c.g[T1][0] = 0u64;
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
c.mov_s(12, 20);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.mov_s(13, 20);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.g[V1][0] = 0x10000000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[S3][0] = 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[S0][0] = 0xffffffffffffffff;
c.sd((c.g[T2][0] as u32), c.g[V0][0]);
c.g[S2][0] = 0xffffffffffff0000;
c.g[S2][0] = c.g[S2][0] | 0xfffb;
c.g[S4][0] = 0x8000;
c.g[S4][0] = c.g[S4][0] << 45;
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sw((c.g[FP][0] as u32), 0u64);
c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
c.sw((c.g[T9][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.g[RA][0] = sx(0x00387300);
c.sq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
Callee { addr: 0x00386d10, f: ee::s12::PS2GraphicsMan_DrawShadowMeshPass }.call(c, 0x00387300)?;
c.lq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A0][0] = 0x30000000;
c.g[A2][0] = 0xc;
c.g[T2][0] = 0x8800;
c.g[T2][0] = c.g[T2][0] << 45;
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a00));
c.g[T3][0] = 0x6;
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), 0u64);
c.g[T4][0] = 0x50000;
c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
c.sw((c.g[FP][0] as u32).wrapping_add(0x18), c.g[S3][0]);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.g[T5][0] = 0x1;
c.sw((c.g[FP][0] as u32).wrapping_add(0x1c), c.g[S3][0]);
c.g[A0][0] = c.g[T9][0];
c.sw((c.g[FP][0] as u32).wrapping_add(0x20), c.g[A2][0]);
c.g[A1][0] = c.g[S6][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A3][0] = 0u64;
c.sw((c.g[FP][0] as u32).wrapping_add(0x24), 0u64);
c.g[T0][0] = 0x1;
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
c.g[T1][0] = 0u64;
c.mov_s(12, 20);
c.mov_s(13, 20);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T2][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[T3][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[T3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
c.g[T2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
c.sw((c.g[FP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.sw((c.g[FP][0] as u32), c.g[T2][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x8), c.g[T3][0]);
c.g[T2][0] = 0x4a;
c.sd((c.g[V1][0] as u32).wrapping_add(0x30), c.g[T4][0]);
c.g[V1][0] = 0x47;
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T5][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T2][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[T4][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[T2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x2c));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[T4][0] as u32)));
c.sw((c.g[FP][0] as u32), c.g[V1][0]);
c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.g[T2][0] = c.g[T2][0] & c.g[S2][0];
c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[T3][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[T3][0]; }
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[T2][0]);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[T3][0] = 0xe;
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.g[V0][0] = c.g[V0][0] | c.g[S4][0];
c.sd((c.g[T4][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T3][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.sd((c.g[V1][0] as u32), 0u64);
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.g[T3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[T2][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[T2][0]; }
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[V1][0] = 0x50000000;
c.g[V0][0] = c.g[V0][0] & 0xff;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[T3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[T3][0] = 0x10000000;
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), 0u64);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.g[T2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
c.sd((c.g[T2][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sw((c.g[FP][0] as u32), 0u64);
c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
c.sw((c.g[T9][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.g[RA][0] = sx(0x00387498);
c.sq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
Callee { addr: 0x00386d10, f: ee::s12::PS2GraphicsMan_DrawShadowMeshPass }.call(c, 0x00387498)?;
c.lq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[V0][0] = 0xc;
c.g[A2][0] = 0x30000000;
c.g[T3][0] = 0x8800;
c.g[T3][0] = c.g[T3][0] << 45;
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a00));
c.g[A1][0] = 0x1000000;
c.sw((c.g[FP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.g[A3][0] = 0x4e;
c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
c.g[T0][0] = 0x30000;
c.g[T0][0] = c.g[T0][0] | 0xc000;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.g[T1][0] = 0x4;
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), 0u64);
c.g[A1][0] = c.g[A0][0] | c.g[A1][0];
c.sw((c.g[FP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
c.g[A1][0] = c.g[A1][0] << 32;
c.sw((c.g[FP][0] as u32).wrapping_add(0x18), c.g[S3][0]);
c.g[A1][0] = c.g[A1][0] >> 32;
c.sw((c.g[FP][0] as u32).wrapping_add(0x1c), c.g[S3][0]);
c.g[T2][0] = 0x1000000;
c.g[T2][0] = c.g[T2][0] | 0x101;
c.g[T2][0] = c.g[T2][0] << 32;
c.sw((c.g[FP][0] as u32).wrapping_add(0x24), 0u64);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T3][0]);
c.g[V0][0] = 0x6;
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
c.sw((c.g[FP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.sw((c.g[FP][0] as u32), c.g[A0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x8), c.g[A2][0]);
c.g[A0][0] = 0x4a;
c.sd((c.g[V1][0] as u32).wrapping_add(0x30), c.g[A1][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
c.g[V1][0] = 0x47;
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x2c));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A2][0] as u32)));
c.sw((c.g[FP][0] as u32), c.g[V1][0]);
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.g[A0][0] = c.g[A0][0] & c.g[S2][0];
c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[A1][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[A1][0]; }
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[A0][0]);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[A1][0] = 0xe;
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.g[S4][0] = c.g[V0][0] | c.g[S4][0];
c.sd((c.g[A2][0] as u32), c.g[S4][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
c.g[A2][0] = 0x50000000;
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V1][0] as u32), 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[A0][0]; }
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[V0][0] = c.g[V0][0] & 0xff;
c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[S3][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sw((c.g[FP][0] as u32).wrapping_add(0x24), c.g[T1][0]);
c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.sw((c.g[FP][0] as u32), c.g[V1][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x28), 0u64);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x18));
c.lwc1(0, (c.g[T9][0] as u32).wrapping_add(0x5840));
c.lwc1(1, (c.g[T9][0] as u32).wrapping_add(0x5844));
c.lwc1(2, (c.g[T9][0] as u32).wrapping_add(0x584c));
c.lwc1(3, (c.g[T9][0] as u32).wrapping_add(0x5850));
c.lwc1(4, (c.g[T9][0] as u32).wrapping_add(0x5854));
c.lwc1(5, (c.g[T9][0] as u32).wrapping_add(0x585c));
c.swc1(0, (c.g[SP][0] as u32));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xc));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x10));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x14));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x1c));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.lq(2, (c.g[SP][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[A0][0] = c.lw((c.g[FP][0] as u32));
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32), c.g[V1][0]);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), 0u64);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A1][0] as u32)));
c.sw((c.g[FP][0] as u32), c.g[V1][0]);
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.sd((c.g[A1][0] as u32), c.g[T2][0]);
c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[S0][0] != 0 { c.g[V0][0] = c.g[A0][0]; }
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x28));
c.g[V0][0] = c.g[V0][0] & 0xff;
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x24));
c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
let t = c.g[A0][0] == 0u64;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
if t {
break 'b_3876d0;
} else {
c.cycles += 4;
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.g[V1][0] = 0x6c000000;
let t = true;
c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
if t {
break 'b_3876d8;
} else {
break 'b_3876d0;
}
}
}
c.cycles += 2;
c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
c.g[V1][0] = 0x6c000000;
break 'b_3876d8;
}
'b_387854: {
c.cycles += 88;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[A2][0] as u32), c.g[V0][0]);
c.g[T4][0] = 0x10000000;
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[T2][0] = 0xfff0000;
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.g[T2][0] = c.g[T2][0] | 0xffff;
c.g[T3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[T9][0];
c.mtc1(0, 12);
c.g[A1][0] = c.g[S6][0];
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[T3][0] as u32)));
c.g[A3][0] = 0u64;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
c.g[T0][0] = 0x1;
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
c.g[T1][0] = 0u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.mov_s(13, 12);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
c.sd((c.g[T3][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sw((c.g[FP][0] as u32), 0u64);
c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
c.sw((c.g[T9][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.g[RA][0] = sx(0x00387758);
c.sq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
Callee { addr: 0x00386d10, f: ee::s12::PS2GraphicsMan_DrawShadowMeshPass }.call(c, 0x00387758)?;
c.lq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[V0][0] = 0x30000000;
c.g[T5][0] = 0x1;
c.g[V1][0] = 0xc;
c.g[A0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a00));
c.g[T4][0] = 0xffffffffffffffff;
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), 0u64);
c.g[A2][0] = 0x8800;
c.g[A2][0] = c.g[A2][0] << 45;
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sw((c.g[FP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32).wrapping_add(0x4), c.g[A0][0]);
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.g[V1][0] = ((c.g[T4][0] as i64) < (c.g[S5][0] as i64)) as u64;
c.sw((c.g[FP][0] as u32).wrapping_add(0x18), c.g[T5][0]);
c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x3f));
c.sw((c.g[FP][0] as u32).wrapping_add(0x1c), c.g[T5][0]);
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[S5][0]; }
c.sw((c.g[FP][0] as u32).wrapping_add(0x24), 0u64);
c.g[V1][0] = 0x6;
c.sd((c.g[A0][0] as u32).wrapping_add(0x10), 0u64);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
c.g[T0][0] = 0x50000;
c.g[A0][0] = c.lw((c.g[FP][0] as u32));
c.g[V0][0] = c.g[V0][0] << 16;
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[T1][0] = 0x47;
c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[V1][0]);
c.g[A1][0] = c.g[A3][0] << 32;
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[A1][0] = c.g[A1][0] >> 32;
c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
c.g[A3][0] = 0x4c;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0xc), c.g[A2][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x30), c.g[A1][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T1][0]);
c.g[A1][0] = c.lw((c.g[FP][0] as u32));
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x14dc));
let t = c.g[V1][0] == 0u64;
c.g[A0][0] = 0x4a;
if t {
break 'b_387854;
} else {
c.cycles += 7;
c.g[V0][0] = 0x1;
c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
break 'b_387854;
}
}
'b_387990: {
'b_387988: {
c.cycles += 73;
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.g[A1][0] = 0xffffffffffff0000;
c.g[A3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
c.g[A1][0] = c.g[A1][0] | 0xfffb;
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x2c));
c.g[T0][0] = 0x8000;
c.g[T0][0] = c.g[T0][0] << 45;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[A3][0] as u32)));
c.g[T1][0] = 0xe;
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.g[A0][0] = c.g[A0][0] & c.g[A1][0];
c.g[V1][0] = ((c.g[T4][0] as i64) < (c.g[A2][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[A2][0]; }
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[A0][0]);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[A2][0] = 0x50000000;
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.g[T2][0] = 0x4;
c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
c.g[T3][0] = 0x1000000;
c.g[T3][0] = c.g[T3][0] | 0x101;
c.g[T3][0] = c.g[T3][0] << 32;
c.sd((c.g[A3][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T1][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.sd((c.g[V1][0] as u32), 0u64);
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.g[V1][0] = ((c.g[T4][0] as i64) < (c.g[A0][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[A0][0]; }
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[V0][0] = c.g[V0][0] & 0xff;
c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[T5][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.sw((c.g[FP][0] as u32).wrapping_add(0x24), c.g[T2][0]);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x28), 0u64);
c.lq(2, (c.g[T9][0] as u32).wrapping_add(0x5840));
c.sq(2, (c.g[V1][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lw((c.g[FP][0] as u32));
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32), c.g[V1][0]);
c.lq(2, (c.g[T9][0] as u32).wrapping_add(0x5850));
c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), 0u64);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
c.sw((c.g[FP][0] as u32), c.g[V1][0]);
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.sd((c.g[A0][0] as u32), c.g[T3][0]);
c.g[V1][0] = ((c.g[T4][0] as i64) < (c.g[A1][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[A1][0]; }
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x28));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & 0xff;
c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x24));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
let t = c.g[A0][0] == 0u64;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
if t {
break 'b_387988;
} else {
c.cycles += 4;
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.g[V1][0] = 0x6c000000;
let t = true;
c.g[V0][0] = c.g[A2][0] | c.g[V0][0];
if t {
break 'b_387990;
} else {
break 'b_387988;
}
}
}
c.cycles += 2;
c.g[V0][0] = c.g[A2][0] | c.g[V0][0];
c.g[V1][0] = 0x6c000000;
break 'b_387990;
}
'b_387d84: {
c.cycles += 65;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[A1][0] as u32), c.g[V0][0]);
c.g[T3][0] = 0x10000000;
c.g[S7][0] = 0xfff0000;
c.g[A0][0] = c.g[T9][0];
c.g[T2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
c.g[S7][0] = c.g[S7][0] | 0xffff;
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.g[A1][0] = c.g[S6][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A3][0] = 0x2;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
c.g[T0][0] = 0u64;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
c.g[T1][0] = 0x1;
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
c.sd((c.g[T2][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sw((c.g[FP][0] as u32), 0u64);
c.g[V0][0] = c.g[V0][0] & c.g[S7][0];
c.sw((c.g[T9][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.g[RA][0] = sx(0x00387a08);
c.sq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
Callee { addr: 0x00386dd0, f: ee::s12::PS2GraphicsMan_DrawShadowEdgePass }.call(c, 0x00387a08)?;
c.lq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A1][0] = c.g[S6][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A3][0] = 0u64;
c.g[A0][0] = c.g[T9][0];
c.g[T0][0] = 0x3;
c.g[RA][0] = sx(0x00387a28);
c.g[T1][0] = 0x1;
Callee { addr: 0x00386dd0, f: ee::s12::PS2GraphicsMan_DrawShadowEdgePass }.call(c, 0x00387a28)?;
c.lq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A1][0] = c.g[S6][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A3][0] = 0x3;
c.g[A0][0] = c.g[T9][0];
c.g[T0][0] = 0x4;
c.g[RA][0] = sx(0x00387a48);
c.g[T1][0] = 0x1;
Callee { addr: 0x00386dd0, f: ee::s12::PS2GraphicsMan_DrawShadowEdgePass }.call(c, 0x00387a48)?;
c.lq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A1][0] = c.g[S6][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A3][0] = 0x4;
c.g[A0][0] = c.g[T9][0];
c.g[T0][0] = 0x1;
c.g[RA][0] = sx(0x00387a68);
c.g[T1][0] = 0x1;
Callee { addr: 0x00386dd0, f: ee::s12::PS2GraphicsMan_DrawShadowEdgePass }.call(c, 0x00387a68)?;
c.lq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A1][0] = c.g[S6][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A3][0] = 0x1;
c.g[A0][0] = c.g[T9][0];
c.g[T0][0] = 0x5;
c.g[RA][0] = sx(0x00387a88);
c.g[T1][0] = 0x1;
Callee { addr: 0x00386dd0, f: ee::s12::PS2GraphicsMan_DrawShadowEdgePass }.call(c, 0x00387a88)?;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x14dc));
let t = c.g[V0][0] == 0u64;
c.lq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
if t {
break 'b_387d84;
} else {
c.cycles += 188;
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = 0x30000000;
c.g[S1][0] = 0x1;
c.g[S4][0] = 0xc;
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), 0u64);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.g[A0][0] = 0x6;
c.sw((c.g[FP][0] as u32).wrapping_add(0x18), c.g[S1][0]);
c.g[T4][0] = 0x50000;
c.g[T4][0] = c.g[T4][0] | 0x4000;
c.sw((c.g[FP][0] as u32).wrapping_add(0x1c), c.g[S1][0]);
c.g[T5][0] = 0x4a;
c.sw((c.g[FP][0] as u32).wrapping_add(0x20), c.g[S4][0]);
c.g[S0][0] = 0xffffffffffffffff;
c.sw((c.g[FP][0] as u32).wrapping_add(0x24), 0u64);
c.g[S2][0] = 0xffffffffffff0000;
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
c.g[S2][0] = c.g[S2][0] | 0xfffb;
c.g[V1][0] = 0x8800;
c.g[V1][0] = c.g[V1][0] << 45;
c.g[S3][0] = 0x8000;
c.g[S3][0] = c.g[S3][0] << 45;
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.g[A1][0] = c.g[S6][0];
c.g[A3][0] = 0u64;
c.g[T0][0] = 0x1;
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.g[T1][0] = 0x1;
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[A0][0]);
c.g[T2][0] = c.lw((c.g[FP][0] as u32));
c.g[A0][0] = c.g[T9][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[T3][0] = sx((c.g[T2][0] as u32).wrapping_add(0x10));
c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add(0x30));
c.sw((c.g[FP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.sw((c.g[FP][0] as u32), c.g[V1][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x8), c.g[T3][0]);
c.sd((c.g[T2][0] as u32).wrapping_add(0x30), c.g[T4][0]);
c.g[T2][0] = 0x47;
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T2][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T5][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[T4][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[T2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x2c));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[T4][0] as u32)));
c.sw((c.g[FP][0] as u32), c.g[V1][0]);
c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.g[T2][0] = c.g[T2][0] & c.g[S2][0];
c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[T3][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[T3][0]; }
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[T2][0]);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[T3][0] = 0xe;
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.g[V0][0] = c.g[V0][0] | c.g[S3][0];
c.sd((c.g[T4][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[T3][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V1][0] as u32), 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[T3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[T2][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[T2][0]; }
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[V1][0] = 0x50000000;
c.g[V0][0] = c.g[V0][0] & 0xff;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[T3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[T3][0] = 0x10000000;
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), 0u64);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.g[T2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
c.sd((c.g[T2][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sw((c.g[FP][0] as u32), 0u64);
c.g[V0][0] = c.g[V0][0] & c.g[S7][0];
c.sw((c.g[T9][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.g[RA][0] = sx(0x00387c38);
c.sq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
Callee { addr: 0x00386dd0, f: ee::s12::PS2GraphicsMan_DrawShadowEdgePass }.call(c, 0x00387c38)?;
c.lq(25, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[V0][0] = 0x30000000;
c.g[A0][0] = 0x6;
c.g[A2][0] = 0x50000;
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a00));
c.g[A3][0] = 0x50000000;
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), 0u64);
c.g[T0][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sw((c.g[FP][0] as u32).wrapping_add(0x1c), c.g[S1][0]);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x20), c.g[S4][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x18), c.g[S1][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x24), 0u64);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
c.g[V1][0] = 0x8800;
c.g[V1][0] = c.g[V1][0] << 45;
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[A0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
c.sw((c.g[FP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
c.sw((c.g[FP][0] as u32), c.g[V0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0xc), c.g[A1][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x30), c.g[A2][0]);
c.g[A1][0] = 0x47;
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32));
c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x2c));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A2][0] as u32)));
c.sw((c.g[FP][0] as u32), c.g[V1][0]);
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.g[A0][0] = c.g[A0][0] & c.g[S2][0];
c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[A1][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[A1][0]; }
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), c.g[A0][0]);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.g[S3][0] = c.g[V0][0] | c.g[S3][0];
c.sd((c.g[A2][0] as u32), c.g[S3][0]);
c.g[A2][0] = 0xe;
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xc));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sd((c.g[V1][0] as u32), 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x8));
c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if c.g[S0][0] != 0 { c.g[V0][0] = c.g[A0][0]; }
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[V0][0] = c.g[V0][0] & 0xff;
c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0x2c), 0u64);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
c.sd((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x4));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.lw((c.g[FP][0] as u32));
c.sw((c.g[FP][0] as u32), 0u64);
c.g[V0][0] = c.g[V0][0] & c.g[S7][0];
c.sw((c.g[T9][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
break 'b_387d84;
}
}
'b_387e24: {
c.cycles += 6;
c.g[A3][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x18f0));
c.g[T1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
if t {
break 'b_387e24;
} else {
c.cycles += 34;
c.g[A1][0] = 0x60000;
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.g[A1][0] | 0x8000;
c.g[V1][0] = 0xffffffffffffffe0;
c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
c.g[V1][0] = 0x30000000;
c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
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
c.ldl(10, (c.g[T0][0] as u32).wrapping_add(0x7));
c.ldr(10, (c.g[T0][0] as u32));
c.ldl(11, (c.g[T0][0] as u32).wrapping_add(0xf));
c.ldr(11, (c.g[T0][0] as u32).wrapping_add(0x8));
c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
c.sdl(10, (c.g[V0][0] as u32).wrapping_add(0x7));
c.sdr(10, (c.g[V0][0] as u32));
c.sdl(11, (c.g[V0][0] as u32).wrapping_add(0xf));
c.sdr(11, (c.g[V0][0] as u32).wrapping_add(0x8));
c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[A2][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_387e24;
}
}
c.cycles += 39;
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a00));
c.g[A0][0] = 0xfffffffffffffc3f;
c.g[A1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
c.g[A2][0] = 0xffffffffc0000000;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[A3][0] = 0x500000;
c.sw((c.g[T9][0] as u32).wrapping_add(0x5a00), c.g[V1][0]);
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sw((c.g[A1][0] as u32), c.g[V0][0]);
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[A0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xe84));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[T3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1420));
c.ldl(8, (c.g[T3][0] as u32).wrapping_add(0x7));
c.ldr(8, (c.g[T3][0] as u32));
c.ldl(9, (c.g[T3][0] as u32).wrapping_add(0xf));
c.ldr(9, (c.g[T3][0] as u32).wrapping_add(0x8));
c.g[T2][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x10));
c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
c.sdr(8, (c.g[V1][0] as u32));
c.sdl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
c.sdr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x100));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
c.pc = t; return Ok(());
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the oc1469 fakes (its own word and magic).
    const OC1469_CTRL: u32 = DATA + 0xe_0000;
    const OC1469_MAGIC: u32 = 0x1469_0386;
    static OC1469_PREV_MESH: AtomicUsize = AtomicUsize::new(0);
    static OC1469_PREV_EDGE: AtomicUsize = AtomicUsize::new(0);
    static OC1469_PREV_VIEW: AtomicUsize = AtomicUsize::new(0);

    fn oc1469_fake_mesh(c: &mut Cpu) {
        if c.bus.read32(OC1469_CTRL) != OC1469_MAGIC {
            return fakes::fall_back(c, &OC1469_PREV_MESH, ee::s12::f_00386d10__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0038_6d10, ra);
    }

    fn oc1469_fake_edge(c: &mut Cpu) {
        if c.bus.read32(OC1469_CTRL) != OC1469_MAGIC {
            return fakes::fall_back(c, &OC1469_PREV_EDGE, ee::s12::f_00386dd0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0038_6dd0, ra);
    }

    fn oc1469_fake_view(c: &mut Cpu) {
        if c.bus.read32(OC1469_CTRL) != OC1469_MAGIC {
            return fakes::fall_back(c, &OC1469_PREV_VIEW, ee::s12::f_0037d968__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0037_d968, ra);
    }

    fn oc1469_hooks() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00386D10),
            oc1469_fake_mesh,
            &OC1469_PREV_MESH,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00386DD0),
            oc1469_fake_edge,
            &OC1469_PREV_EDGE,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0037D968),
            oc1469_fake_view,
            &OC1469_PREV_VIEW,
        );
    }

    /// Fixture: the graphics manager at `OBJ`, its GS-state block, the DMA
    /// state block, the packet buffer and the counter object. Seeds vary the
    /// view-matrix flag (+0x6b90), the gp flag 0x14dc and the counter (< 2600).
    fn oc1469_setup(c: &mut Cpu, seed: u32) {
        const OBJ: u32 = DATA + 0x1_0000;
        let mut r: Rng = fakes::machine(c, seed, 0x38_6e78, 10);
        elf_image(c);
        oc1469_hooks();
        c.bus.write32(OC1469_CTRL, OC1469_MAGIC);
        c.pc = 0x0038_6e78;
        c.g[A0][0] = sx(OBJ);
        let gs = DATA + 0x3_0000;
        let dma = DATA + 0x2_0000;
        let vt = DATA + 0x4_0000;
        let counter = DATA + 0x6_0000;
        c.bus.write32(OBJ + 0xe84, gs);
        c.bus.write32(OBJ + 0x6b90, (seed % 2) as u32);
        c.bus.write32(OBJ + 0x10d8, vt);
        c.bus.write16(vt + 0x50, 8);
        c.bus.write32(vt + 0x54, fakes::FAKE);
        c.bus.write32(OBJ + 0x5a00, DATA + 0x5_0000);
        c.bus.write32(OBJ + 0x5a80, r.next_u32());
        c.bus.write32(OBJ + 0x5a84, r.next_u32());
        for k in 0..8 {
            c.bus.write32(OBJ + 0x5840 + 4 * k, r.next_u32());
        }
        for k in 0..4 {
            c.bus.write32(OBJ + 0xf50 + 4 * k, r.next_u32());
        }
        c.bus.write32(OBJ + 0x18f0, counter);
        let count = if seed % 5 == 0 { 2600 + seed } else { seed % 40 };
        c.bus.write32(counter, count);
        c.bus.write32(GAME_GP + 0x14e4, seed % 4);
        c.bus.write32(GAME_GP + 0x14dc, (seed / 2 % 2) as u32);
        c.bus.write32(GAME_GP + 0x2a94, dma);
    }

    /// `PS2GraphicsMan_v89` (0x386e78, builds the shadow-volume GIF packets
    /// around the mesh and edge passes). The three pass callees are gated
    /// fakes that log their registers; the vtable call lands in the miss hook.
    #[test]
    fn draw_shadow_volume_matches() {
        fakes::covered(
            ee::s12::f_00386e78__generated,
            draw_shadow_volume,
            150,
            &|c, seed| oc1469_setup(c, seed),
        );
    }
}
