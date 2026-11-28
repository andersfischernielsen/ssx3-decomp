//! Staging module oc1468 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1468`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0022_b008,
    game_module_render,
    "GameModule_Render",
)];

/// `GameModule_Render` (`cGame::render`, 0x0022b008): draws one game frame (the draw path: per view it
/// sets the camera and render state, gathers the visible world and calls the model, fog, flare and
/// bezier-patch passes; then the HUD and menu overlay and the frame-counter tick): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[allow(unused_assignments, unused_variables, unused_labels, unreachable_code)]
pub fn game_module_render(c: &mut Cpu) {
    if c.pc != 0x0022b008 {
        return ee::s05::f_0022b008__generated(c);
    }
    guest(c, |c| {
'b_22c02c: {
'b_22b078: {
c.cycles += 26;
c.g[T4][0] = 0x83e0;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub((c.g[T4][0] as u32)));
c.g[T5][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[SP][0] as u32)));
c.sq(18, (c.g[T5][0] as u32).wrapping_sub(0x50));
c.sq(16, (c.g[T5][0] as u32).wrapping_sub(0x30));
c.g[S2][0] = c.g[A0][0];
c.sq(17, (c.g[T5][0] as u32).wrapping_sub(0x40));
c.sq(19, (c.g[T5][0] as u32).wrapping_sub(0x60));
c.sq(20, (c.g[T5][0] as u32).wrapping_sub(0x70));
c.sq(21, (c.g[T5][0] as u32).wrapping_sub(0x80));
c.sq(22, (c.g[T5][0] as u32).wrapping_sub(0x90));
c.sq(23, (c.g[T5][0] as u32).wrapping_sub(0xa0));
c.sq(30, (c.g[T5][0] as u32).wrapping_sub(0xb0));
c.sd((c.g[T5][0] as u32).wrapping_sub(0xc0), c.g[RA][0]);
c.swc1(23, (c.g[T5][0] as u32).wrapping_sub(0x8));
c.swc1(22, (c.g[T5][0] as u32).wrapping_sub(0x10));
c.swc1(21, (c.g[T5][0] as u32).wrapping_sub(0x18));
c.swc1(20, (c.g[T5][0] as u32).wrapping_sub(0x20));
c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x0022b068);
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0022b068)?;
let t = c.g[V0][0] != 0u64;
c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
if t {
break 'b_22b078;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = 0u64;
if t {
break 'b_22c02c;
} else {
break 'b_22b078;
}
}
}
'b_22b0d8: {
c.cycles += 16;
c.g[A1][0] = 0xffffffffffffffff;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x3a0));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3a4));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0022b090);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0022b090)?;
c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x98));
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
c.g[V1][0] = c.lw((c.g[A1][0] as u32));
c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
c.g[RA][0] = sx(0x0022b0ac);
c.g[S0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
Callee { addr: 0x0022e8b8, f: ee::s05::GameViewMan_PrefetchViews }.call(c, 0x0022b0ac)?;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x9c));
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = 0u64;
if t {
break 'b_22b0d8;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x0022b0c0);
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
Callee { addr: 0x0022e920, f: ee::s05::GameViewMan_AllViewsReady }.call(c, 0x0022b0c0)?;
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = 0x1;
if t {
break 'b_22b0d8;
} else {
c.cycles += 4;
c.g[V0][0] = 0x1;
c.g[A1][0] = 0u64;
c.sw((c.g[S2][0] as u32).wrapping_add(0xa0), c.g[V0][0]);
c.sw((c.g[S2][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
break 'b_22b0d8;
}
}
}
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x0022b0e4);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, t, 0x0022b0e4)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_22c02c;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x0022b0f4);
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x00231d60, 0x0022b0f4)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_22c02c;
} else {
'b_22b11c: {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x504));
let t = c.g[V1][0] == 0u64;

if t {
break 'b_22b11c;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x0022b114);
c.g[A0][0] = c.g[S2][0];
Callee { addr: 0x0022c830, f: ee::s05::GameModule_RenderUI }.call(c, 0x0022b114)?;
let t = true;
c.g[V0][0] = 0x1;
if t {
break 'b_22c02c;
} else {
break 'b_22b11c;
}
}
}
'b_22bcbc: {
c.cycles += 25;
c.g[RA][0] = sx(0x0022b124);
c.g[S3][0] = 0u64;
call_ptr(c, 0x0036a1e8, 0x0022b124)?;
c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x24));
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0xf50));
c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x44));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[A1][0] = c.lw((c.g[V0][0] as u32));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.sw((c.g[A2][0] as u32).wrapping_add(0x10a8), c.g[A1][0]);
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.sw((c.g[A2][0] as u32).wrapping_add(0x10ac), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[A0][0] as u32).wrapping_add(0x6490), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[A2][0] as u32).wrapping_add(0x270), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x8310;
if t {
break 'b_22bcbc;
} else {
c.cycles += 24;
c.g[V1][0] = 0xfffffffffe7f0000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[SP][0] as u32)));
c.g[A2][0] = c.g[V1][0] | 0xffff;
c.sw((c.g[V0][0] as u32), c.g[A2][0]);
c.g[A1][0] = 0xfffffffffff00000;
c.g[A0][0] = 0x8314;
c.g[A3][0] = c.g[A1][0] | 0xfff;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[SP][0] as u32)));
c.g[V0][0] = 0x8318;
c.sw((c.g[A0][0] as u32), c.g[A3][0]);
c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[SP][0] as u32)));
c.g[FP][0] = 0xffffffffffcf0000;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 22);
c.g[FP][0] = c.g[FP][0] | 0xffff;
c.sw((c.g[V0][0] as u32), c.g[V1][0]);
c.g[S1][0] = c.g[SP][0];
c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
'l_22b1e0: loop {
'b_22b584: {
c.cycles += 232;
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10d8));
c.sw((c.g[A0][0] as u32).wrapping_add(0x274), c.g[S3][0]);
c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x210));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x214));
let t = c.g[A1][0] as u32;
c.g[RA][0] = sx(0x0022b1f8);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
call_ptr(c, t, 0x0022b1f8)?;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
c.g[A1][0] = sx((c.g[S3][0] as u32) << 4);
c.g[T1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x18));
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S3][0]);
c.g[A2][0] = 0x1;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
c.g[A3][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x10d8));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0xc0));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[T0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc4));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
let t = c.g[T0][0] as u32;
c.g[RA][0] = sx(0x0022b23c);
c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
call_ptr(c, t, 0x0022b23c)?;
c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x110));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x114));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0022b258);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0022b258)?;
c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.mov_s(13, 22);
c.lwc1(12, (c.g[S0][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
c.lwc1(14, (c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0022b27c);
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0022b27c)?;
c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A0][0] = 0x8310;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[SP][0] as u32)));
c.g[A2][0] = 0x800000;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
c.g[A3][0] = 0x300000;
c.g[A0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A0][0] = 0x8314;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[SP][0] as u32)));
c.g[A0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | 0xd000;
c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0022b2f4);
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0022b2f4)?;
c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.lwc1(12, (c.g[S0][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
c.lwc1(14, (c.g[S0][0] as u32).wrapping_add(0x8));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0022b318);
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0022b318)?;
c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A1][0] = c.g[S3][0];
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x3a0));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3a4));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0022b334);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0022b334)?;
c.g[A2][0] = 0x8318;
c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[SP][0] as u32)));
c.g[A3][0] = 0xfffffffffffffc1f;
c.g[A0][0] = c.lw((c.g[A2][0] as u32));
c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
c.g[T0][0] = 0xfffffffffffff07f;
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.g[V0][0] = c.g[V0][0] | 0x80;
c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x24));
c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xf50));
c.sh((c.g[A2][0] as u32).wrapping_add(0x12), c.g[V0][0]);
c.g[A2][0] = 0x800000;
c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
c.g[V0][0] = c.g[V0][0] | 0x380;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] | 0x30;
c.sw((c.g[V1][0] as u32), c.g[V0][0]);
c.g[V1][0] = 0x8310;
c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[SP][0] as u32)));
c.g[V1][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A2][0] = 0x8314;
c.g[A1][0] = 0x300000;
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[SP][0] as u32)));
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = 0xfffffffffffffc1f;
c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
c.g[A2][0] = c.lw((c.g[A2][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.g[V0][0] = c.g[V0][0] | 0xd000;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | 0x80;
c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x158));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x15c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x0022b428);
c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
call_ptr(c, t, 0x0022b428)?;
c.g[A1][0] = 0x8318;
c.lwc1(20, (c.g[S0][0] as u32).wrapping_add(0x4));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[SP][0] as u32)));
c.lwc1(21, (c.g[S0][0] as u32).wrapping_add(0x8));
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
c.g[A1][0] = c.lw((c.g[A1][0] as u32));
c.lqc2(1, (c.g[A1][0] as u32));
c.lqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[A0][0] as u32));
c.sqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
c.g[RA][0] = sx(0x0022b468);
call_ptr(c, 0x0031b6c8, 0x0022b468)?;
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(5, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sq(5, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
c.mfc1(3, 20);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(2, (c.g[SP][0] as u32));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
c.mfc1(3, 20);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lwc1(0, (c.g[SP][0] as u32));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.c_lt_s(1, 0);
let t = !c.fcc();
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
if t {
break 'b_22b584;
} else {
c.cycles += 1;
c.swc1(1, (c.g[SP][0] as u32));
break 'b_22b584;
}
}
'b_22b59c: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
break 'b_22b59c;
} else {
break 'b_22b59c;
}
}
'b_22b5b4: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
break 'b_22b5b4;
} else {
break 'b_22b5b4;
}
}
'b_22b5cc: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_22b5cc;
} else {
break 'b_22b5cc;
}
}
'b_22b5e4: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
break 'b_22b5e4;
} else {
break 'b_22b5e4;
}
}
'b_22b5fc: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
break 'b_22b5fc;
} else {
break 'b_22b5fc;
}
}
'b_22b680: {
c.cycles += 32;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
c.mfc1(3, 20);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lwc1(0, (c.g[SP][0] as u32));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.c_lt_s(1, 0);
let t = !c.fcc();
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
if t {
break 'b_22b680;
} else {
c.cycles += 1;
c.swc1(1, (c.g[SP][0] as u32));
break 'b_22b680;
}
}
'b_22b698: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
break 'b_22b698;
} else {
break 'b_22b698;
}
}
'b_22b6b0: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
break 'b_22b6b0;
} else {
break 'b_22b6b0;
}
}
'b_22b6c8: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_22b6c8;
} else {
break 'b_22b6c8;
}
}
'b_22b6e0: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
break 'b_22b6e0;
} else {
break 'b_22b6e0;
}
}
'b_22b6f8: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
break 'b_22b6f8;
} else {
break 'b_22b6f8;
}
}
'b_22b77c: {
c.cycles += 33;
c.lqc2(3, (c.g[S4][0] as u32));
c.lqc2(4, (c.g[S5][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(4, (c.g[S7][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(4, (c.g[S6][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
c.mfc1(3, 20);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[S1][0] as u32));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32));
break 'b_22b77c;
} else {
break 'b_22b77c;
}
}
'b_22b794: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
break 'b_22b794;
} else {
break 'b_22b794;
}
}
'b_22b7ac: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
break 'b_22b7ac;
} else {
break 'b_22b7ac;
}
}
'b_22b7c4: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x10));
break 'b_22b7c4;
} else {
break 'b_22b7c4;
}
}
'b_22b7dc: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
break 'b_22b7dc;
} else {
break 'b_22b7dc;
}
}
'b_22b7f4: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
break 'b_22b7f4;
} else {
break 'b_22b7f4;
}
}
'b_22b878: {
c.cycles += 33;
c.lqc2(3, (c.g[S4][0] as u32));
c.lqc2(4, (c.g[S5][0] as u32));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(4, (c.g[S7][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(4, (c.g[S6][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
c.mfc1(3, 21);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[S1][0] as u32));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32));
break 'b_22b878;
} else {
break 'b_22b878;
}
}
'b_22b890: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
break 'b_22b890;
} else {
break 'b_22b890;
}
}
'b_22b8a8: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
break 'b_22b8a8;
} else {
break 'b_22b8a8;
}
}
'b_22b8c0: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x10));
break 'b_22b8c0;
} else {
break 'b_22b8c0;
}
}
'b_22b8d8: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
break 'b_22b8d8;
} else {
break 'b_22b8d8;
}
}
'b_22b8f0: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
break 'b_22b8f0;
} else {
break 'b_22b8f0;
}
}
'b_22b974: {
c.cycles += 33;
c.lqc2(3, (c.g[S4][0] as u32));
c.lqc2(4, (c.g[S5][0] as u32));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(4, (c.g[S7][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(4, (c.g[S6][0] as u32));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
c.mfc1(3, 21);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[S1][0] as u32));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32));
break 'b_22b974;
} else {
break 'b_22b974;
}
}
'b_22b98c: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
break 'b_22b98c;
} else {
break 'b_22b98c;
}
}
'b_22b9a4: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
break 'b_22b9a4;
} else {
break 'b_22b9a4;
}
}
'b_22b9bc: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x10));
break 'b_22b9bc;
} else {
break 'b_22b9bc;
}
}
'b_22b9d4: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
break 'b_22b9d4;
} else {
break 'b_22b9d4;
}
}
'b_22b9ec: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
break 'b_22b9ec;
} else {
break 'b_22b9ec;
}
}
'b_22ba70: {
c.cycles += 33;
c.lqc2(3, (c.g[S4][0] as u32));
c.lqc2(4, (c.g[S5][0] as u32));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(4, (c.g[S7][0] as u32));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(4, (c.g[S6][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
c.mfc1(3, 21);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[S1][0] as u32));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32));
break 'b_22ba70;
} else {
break 'b_22ba70;
}
}
'b_22ba88: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
break 'b_22ba88;
} else {
break 'b_22ba88;
}
}
'b_22baa0: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
break 'b_22baa0;
} else {
break 'b_22baa0;
}
}
'b_22bab8: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x10));
break 'b_22bab8;
} else {
break 'b_22bab8;
}
}
'b_22bad0: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
break 'b_22bad0;
} else {
break 'b_22bad0;
}
}
'b_22bae8: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
break 'b_22bae8;
} else {
break 'b_22bae8;
}
}
'b_22bb6c: {
c.cycles += 33;
c.lqc2(3, (c.g[S4][0] as u32));
c.lqc2(4, (c.g[S5][0] as u32));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lqc2(4, (c.g[S7][0] as u32));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lqc2(4, (c.g[S6][0] as u32));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
c.mfc1(3, 21);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x100));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xec));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[S1][0] as u32));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32));
break 'b_22bb6c;
} else {
break 'b_22bb6c;
}
}
'b_22bb84: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
break 'b_22bb84;
} else {
break 'b_22bb84;
}
}
'b_22bb9c: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x8));
break 'b_22bb9c;
} else {
break 'b_22bb9c;
}
}
'b_22bbb4: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x10));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x10));
break 'b_22bbb4;
} else {
break 'b_22bbb4;
}
}
'b_22bbcc: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
break 'b_22bbcc;
} else {
break 'b_22bbcc;
}
}
'b_22bbe4: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x18));
c.c_lt_s(0, 1);
if c.fcc() {
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
break 'b_22bbe4;
} else {
break 'b_22bbe4;
}
}
c.cycles += 54;
c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8c));
c.g[A3][0] = c.g[SP][0];
c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
c.g[RA][0] = sx(0x0022bc00);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x0022add8, f: ee::s05::Nash_GatherVisible }.call(c, 0x0022bc00)?;
c.g[RA][0] = sx(0x0022bc08);
c.g[A0][0] = c.g[S2][0];
call_ptr(c, 0x00230640, 0x0022bc08)?;
c.g[RA][0] = sx(0x0022bc10);
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
Callee { addr: 0x00229ba8, f: ee::s05::CrowdMan2d_RenderSprites }.call(c, 0x0022bc10)?;
c.g[RA][0] = sx(0x0022bc18);
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00344138, 0x0022bc18)?;
c.g[RA][0] = sx(0x0022bc20);
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
call_ptr(c, 0x00357c50, 0x0022bc20)?;
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A2][0] = 0xfffffffffffffc1f;
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.g[V0][0] = c.g[V0][0] | 0x80;
c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xcc));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0022bc54);
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0022bc54)?;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x0022bc60);
c.g[A1][0] = c.g[S0][0];
Callee { addr: 0x0022c078, f: ee::s05::GameModule_RenderModels }.call(c, 0x0022bc60)?;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x0022bc6c);
c.g[A1][0] = c.g[S0][0];
Callee { addr: 0x0022c708, f: ee::s05::GameModule_RenderFogParticles }.call(c, 0x0022bc6c)?;
c.g[A2][0] = c.g[S3][0];
c.g[A0][0] = c.g[S2][0];
c.g[A1][0] = c.g[S0][0];
c.g[RA][0] = sx(0x0022bc80);
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
Callee { addr: 0x0022c790, f: ee::s05::GameModule_RenderLensFlares }.call(c, 0x0022bc80)?;
c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A3][0] = 0xfffffffffffffc1f;
c.g[A1][0] = c.g[S0][0];
c.g[A0][0] = c.g[S2][0];
c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.g[V0][0] = c.g[V0][0] | 0x40;
c.g[RA][0] = sx(0x0022bca8);
c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
Callee { addr: 0x0022c620, f: ee::s05::GameModule_RenderBezierPatches }.call(c, 0x0022bca8)?;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
if c.g[V0][0] != 0u64 {
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
if event_due(c) { deliver_event_here(c, 0x0022b1e0); } continue 'l_22b1e0;
} else {
break 'b_22bcbc;
}
}
}
}
'b_22bd14: {
'b_22bcf8: {
c.cycles += 11;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10d8));
c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xb8));
c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbc));
c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x50));
c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x0022bce0);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
call_ptr(c, t, 0x0022bce0)?;
let t = (c.g[V0][0] as i64) < 0;
c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
if t {
break 'b_22bcf8;
} else {
c.cycles += 4;
c.mtc1(2, 20);
c.cvt_s_w(20, 20);
let t = true;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
if t {
break 'b_22bd14;
} else {
break 'b_22bcf8;
}
}
}
c.cycles += 7;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 20);
c.cvt_s_w(20, 20);
c.add_s(20, 20, 20);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
break 'b_22bd14;
}
'b_22bd50: {
'b_22bd3c: {
c.cycles += 6;
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x0022bd24);
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0022bd24)?;
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_22bd3c;
} else {
c.cycles += 4;
c.mtc1(2, 15);
c.cvt_s_w(15, 15);
let t = true;

if t {
break 'b_22bd50;
} else {
break 'b_22bd3c;
}
}
}
c.cycles += 5;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 15);
c.cvt_s_w(15, 15);
c.add_s(15, 15, 15);
break 'b_22bd50;
}
'b_22bef0: {
c.cycles += 81;
c.mtc1(0, 12);
c.g[A0][0] = c.g[S0][0];
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
c.mov_s(14, 20);
c.mov_s(21, 12);
c.mov_s(13, 12);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 23);
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x0022bd78);
c.g[A1][0] = 0x1;
call_ptr(c, t, 0x0022bd78)?;
c.g[S0][0] = 0xfffffffffffffc1f;
c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A1][0] = 0u64;
c.g[AT][0] = 0x44200000;
c.mtc1(1, 14);
c.g[A2][0] = 0u64;
c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
c.mov_s(16, 21);
c.g[AT][0] = 0x43f00000;
c.mtc1(1, 15);
c.mov_s(17, 23);
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xd0));
c.mov_s(12, 21);
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd4));
c.mov_s(13, 21);
c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0022bdc0);
c.g[S1][0] = 0xffffffffffffff83;
call_ptr(c, t, 0x0022bdc0)?;
c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x51e4));
c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A1][0] = 0x500000;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe60));
c.g[AT][0] = 0x40000000;
c.mtc1(1, 22);
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x110));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x114));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0022bdec);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0022bdec)?;
c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
c.g[V0][0] = c.g[V0][0] | 0x300;
c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
c.g[V0][0] = c.g[V0][0] | 0x14;
c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x30));
c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0x34));
c.mul_s(0, 0, 20);
c.mul_s(1, 1, 20);
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x38));
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x3c));
c.swc1(22, (c.g[SP][0] as u32));
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x4));
c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[SP][0] as u32));
c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x2f));
c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x28));
c.swc1(23, (c.g[SP][0] as u32));
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc));
c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(7, (c.g[SP][0] as u32));
c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x47));
c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x40));
c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4f));
c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x48));
c.g[RA][0] = sx(0x0022be8c);
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa4));
call_ptr(c, 0x002cc098, 0x0022be8c)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
if t {
break 'b_22bef0;
} else {
c.cycles += 23;
c.g[RA][0] = sx(0x0022be9c);
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xa4));
call_ptr(c, 0x002cc350, 0x0022be9c)?;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
c.swc1(22, (c.g[SP][0] as u32));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x4));
c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(6, (c.g[SP][0] as u32));
c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x2f));
c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x28));
c.swc1(23, (c.g[SP][0] as u32));
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc));
c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(5, (c.g[SP][0] as u32));
c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xf));
c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x8));
c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x47));
c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x40));
c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x4f));
c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x48));
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
break 'b_22bef0;
}
}
'b_22bfd8: {
c.cycles += 47;
c.g[V1][0] = 0x500000;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1420));
c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x7));
c.ldr(5, (c.g[V0][0] as u32));
c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
c.sdr(5, (c.g[A0][0] as u32));
c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A3][0]);
c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
c.g[V0][0] = c.g[V0][0] | 0x300;
c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
c.g[V0][0] = c.g[V0][0] | 0x14;
c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
c.swc1(22, (c.g[SP][0] as u32));
c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[SP][0] as u32));
c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x2f));
c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x28));
c.swc1(23, (c.g[SP][0] as u32));
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc));
c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(7, (c.g[SP][0] as u32));
c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xf));
c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x8));
c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x47));
c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x40));
c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x4f));
c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x48));
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1450));
let t = c.g[V0][0] != 0u64;

if t {
break 'b_22bfd8;
} else {
c.cycles += 11;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x24));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x4));
c.swc1(21, (c.g[SP][0] as u32));
c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(6, (c.g[SP][0] as u32));
c.sdl(6, (c.g[V0][0] as u32).wrapping_add(0x2f));
c.sdr(6, (c.g[V0][0] as u32).wrapping_add(0x28));
c.g[RA][0] = sx(0x0022bfd0);
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x48));
call_ptr(c, 0x00397df8, 0x0022bfd0)?;
c.g[RA][0] = sx(0x0022bfd8);
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x30));
call_ptr(c, 0x0030b4b8, 0x0022bfd8)?;
break 'b_22bfd8;
}
}
c.cycles += 21;
c.g[RA][0] = sx(0x0022bfe0);
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
call_ptr(c, 0x002789c0, 0x0022bfe0)?;
c.g[RA][0] = sx(0x0022bfe8);
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x94));
call_ptr(c, 0x001f31e0, 0x0022bfe8)?;
c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0022c000);
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0022c000)?;
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0xac));
c.cvt_s_w(0, 0);
c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x51e0));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x410));
c.g[A1][0] = 0x1;
c.g[RA][0] = sx(0x0022c01c);
c.mul_s(12, 0, 12);
call_ptr(c, 0x00285bf8, 0x0022c01c)?;
c.g[RA][0] = sx(0x0022c024);
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x21e4));
call_ptr(c, 0x002b5d78, 0x0022c024)?;
c.sw((c.g[S2][0] as u32).wrapping_add(0xac), 0u64);
c.g[V0][0] = 0x1;
break 'b_22c02c;
}
}
}
c.cycles += 18;
c.g[T4][0] = 0x83e0;
c.g[T5][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[SP][0] as u32)));
c.lq(16, (c.g[T5][0] as u32).wrapping_sub(0x30));
c.lq(17, (c.g[T5][0] as u32).wrapping_sub(0x40));
c.lq(18, (c.g[T5][0] as u32).wrapping_sub(0x50));
c.lq(19, (c.g[T5][0] as u32).wrapping_sub(0x60));
c.lq(20, (c.g[T5][0] as u32).wrapping_sub(0x70));
c.lq(21, (c.g[T5][0] as u32).wrapping_sub(0x80));
c.lq(22, (c.g[T5][0] as u32).wrapping_sub(0x90));
c.lq(23, (c.g[T5][0] as u32).wrapping_sub(0xa0));
c.lq(30, (c.g[T5][0] as u32).wrapping_sub(0xb0));
c.g[RA][0] = c.ld((c.g[T5][0] as u32).wrapping_sub(0xc0));
c.lwc1(23, (c.g[T5][0] as u32).wrapping_sub(0x8));
c.lwc1(22, (c.g[T5][0] as u32).wrapping_sub(0x10));
c.lwc1(21, (c.g[T5][0] as u32).wrapping_sub(0x18));
c.lwc1(20, (c.g[T5][0] as u32).wrapping_sub(0x20));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[T4][0] as u32)));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{self, fakes, fresh, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this test's callee fakes and their mailbox (in the machine's own RAM,
    /// outside the random arena).
    const OC1468_CTRL: u32 = DATA + 0x0f_0000;
    const OC1468_MAGIC: u32 = 0x1468_0b00;
    /// The random arena: words that are pointers into it, floats, small integers or zero.
    const OC1468_ARENA: u32 = DATA;
    const OC1468_ARENA_WORDS: u32 = 0x1_0000;
    const OC1468_SELF: u32 = DATA + 0x1000;
    const OC1468_GFX: u32 = DATA + 0x4000;
    const OC1468_TOP: u32 = DATA + 0x6000;
    const OC1468_X: u32 = DATA + 0x6400;
    const OC1468_VIEWS: u32 = DATA + 0x7000;
    const OC1468_HUD: u32 = DATA + 0x7400;
    const OC1468_TARGET: u32 = DATA + 0x7800;
    const OC1468_CAMS: u32 = DATA + 0x8000;

    /// Every callee of `GameModule_Render` with an override slot (all are faked: the real
    /// draw passes run away on random data).
    const OC1468_ADDRS: [u32; 23] = [
        0x0022_e8b8,
        0x0022_e920,
        0x0023_1d60,
        0x0036_a1e8,
        0x0022_c830,
        0x0031_b6c8,
        0x0022_add8,
        0x0023_0640,
        0x0022_9ba8,
        0x0034_4138,
        0x0035_7c50,
        0x0022_c078,
        0x0022_c708,
        0x0022_c790,
        0x0022_c620,
        0x002c_c098,
        0x002c_c350,
        0x0039_7df8,
        0x0030_b4b8,
        0x0027_89c0,
        0x001f_31e0,
        0x0028_5bf8,
        0x002b_5d78,
    ];

    fn oc1468_idx(addr: u32) -> u32 {
        OC1468_ADDRS
            .iter()
            .position(|&a| a == addr)
            .expect("fake address") as u32
    }

    /// The result the fixture forces for callee `addr` (a word in the mailbox).
    fn oc1468_result_word(addr: u32) -> u32 {
        OC1468_CTRL + 0x100 + 4 * oc1468_idx(addr)
    }

    /// A slot fake: logs the call like `fakes::fake_call`, then returns the forced result;
    /// the matrix inverse also writes a random matrix to its output (so the view box grows).
    fn oc1468_slot_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let a0 = c.g[A0][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let res = c.bus.read32(oc1468_result_word(addr));
        c.g[V0][0] = res as u64;
        if addr == 0x0031_b6c8 {
            for i in 0..16u32 {
                let h = fakes::mix((a0 as u64) << 8 ^ (i as u64) ^ (n as u64) << 40);
                let f = ((h >> 8) as u32 % 2000) as f32 / 100.0 - 10.0;
                c.bus.write32(a0 + 4 * i, f.to_bits());
            }
        }
    }

    macro_rules! oc1468_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1468_CTRL) != OC1468_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1468_slot_call(c, $addr);
                }
            )*
            fn oc1468_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1468_fakes!(
        (oc1468_f_prefetch, OC1468_P_PREFETCH, ee::s05::OVERRIDE_0022E8B8, ee::s05::f_0022e8b8__generated, 0x0022_e8b8),
        (oc1468_f_ready, OC1468_P_READY, ee::s05::OVERRIDE_0022E920, ee::s05::f_0022e920__generated, 0x0022_e920),
        (oc1468_f_231d60, OC1468_P_231D60, ee::s06::OVERRIDE_00231D60, ee::s06::f_00231d60__generated, 0x0023_1d60),
        (oc1468_f_36a1e8, OC1468_P_36A1E8, ee::s12::OVERRIDE_0036A1E8, ee::s12::f_0036a1e8__generated, 0x0036_a1e8),
        (oc1468_f_ui, OC1468_P_UI, ee::s05::OVERRIDE_0022C830, ee::s05::f_0022c830__generated, 0x0022_c830),
        (oc1468_f_invert, OC1468_P_INVERT, ee::s10::OVERRIDE_0031B6C8, ee::s10::f_0031b6c8__generated, 0x0031_b6c8),
        (oc1468_f_gather, OC1468_P_GATHER, ee::s05::OVERRIDE_0022ADD8, ee::s05::f_0022add8__generated, 0x0022_add8),
        (oc1468_f_230640, OC1468_P_230640, ee::s06::OVERRIDE_00230640, ee::s06::f_00230640__generated, 0x0023_0640),
        (oc1468_f_crowd, OC1468_P_CROWD, ee::s05::OVERRIDE_00229BA8, ee::s05::f_00229ba8__generated, 0x0022_9ba8),
        (oc1468_f_344138, OC1468_P_344138, ee::s11::OVERRIDE_00344138, ee::s11::f_00344138__generated, 0x0034_4138),
        (oc1468_f_357c50, OC1468_P_357C50, ee::s11::OVERRIDE_00357C50, ee::s11::f_00357c50__generated, 0x0035_7c50),
        (oc1468_f_models, OC1468_P_MODELS, ee::s05::OVERRIDE_0022C078, ee::s05::f_0022c078__generated, 0x0022_c078),
        (oc1468_f_fog, OC1468_P_FOG, ee::s05::OVERRIDE_0022C708, ee::s05::f_0022c708__generated, 0x0022_c708),
        (oc1468_f_flares, OC1468_P_FLARES, ee::s05::OVERRIDE_0022C790, ee::s05::f_0022c790__generated, 0x0022_c790),
        (oc1468_f_patches, OC1468_P_PATCHES, ee::s05::OVERRIDE_0022C620, ee::s05::f_0022c620__generated, 0x0022_c620),
        (oc1468_f_2cc098, OC1468_P_2CC098, ee::s09::OVERRIDE_002CC098, ee::s09::f_002cc098__generated, 0x002c_c098),
        (oc1468_f_2cc350, OC1468_P_2CC350, ee::s09::OVERRIDE_002CC350, ee::s09::f_002cc350__generated, 0x002c_c350),
        (oc1468_f_397df8, OC1468_P_397DF8, ee::s13::OVERRIDE_00397DF8, ee::s13::f_00397df8__generated, 0x0039_7df8),
        (oc1468_f_30b4b8, OC1468_P_30B4B8, ee::s10::OVERRIDE_0030B4B8, ee::s10::f_0030b4b8__generated, 0x0030_b4b8),
        (oc1468_f_2789c0, OC1468_P_2789C0, ee::s07::OVERRIDE_002789C0, ee::s07::f_002789c0__generated, 0x0027_89c0),
        (oc1468_f_1f31e0, OC1468_P_1F31E0, ee::s04::OVERRIDE_001F31E0, ee::s04::f_001f31e0__generated, 0x001f_31e0),
        (oc1468_f_285bf8, OC1468_P_285BF8, ee::s07::OVERRIDE_00285BF8, ee::s07::f_00285bf8__generated, 0x0028_5bf8),
        (oc1468_f_2b5d78, OC1468_P_2B5D78, ee::s08::OVERRIDE_002B5D78, ee::s08::f_002b5d78__generated, 0x002b_5d78),
    );

    /// The miss hook (the vtable and function-pointer calls): logs like `fakes::fake_call`;
    /// the fixture's mask word zeroes the result of some of the first 32 calls.
    fn oc1468_miss(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc == ra && n < 32 && c.bus.read32(OC1468_CTRL + 4) >> n & 1 != 0 {
            c.g[V0][0] = 0;
        }
    }

    /// A game object `cGame` with its graphics object, view manager, camera array, HUD and
    /// target, all in a random arena (every pointer chain stays inside it); the seed picks
    /// the view count (0-3), the early exits (not ready, loader not ready, UI-only frame),
    /// the menu branch, the HUD-off branch and the unwinding callee.
    fn oc1468_setup(c: &mut Cpu, seed: u32) {
        oc1468_install_fakes();
        let mut r = fakes::machine(c, seed, 0x1468_0b00, 90);
        c.hooks.miss = oc1468_miss;
        difftest::elf_image(c);
        for i in 0..OC1468_ARENA_WORDS {
            let w = match r.next_u32() % 10 {
                0..=5 => OC1468_ARENA + (r.next_u32() & 0x3_fff0),
                6 | 7 => r.f32(300.0),
                8 => r.next_u32() % 9,
                _ => 0,
            };
            c.bus.write32(OC1468_ARENA + 4 * i, w);
        }
        for a in 0..0x200u32 {
            c.bus.write32(OC1468_CTRL + 4 * a, 0);
        }
        c.bus.write32(OC1468_CTRL, OC1468_MAGIC);
        // Zeroed results of the first calls: sparse, and the first (isReady) only now and then.
        let mut mask = r.next_u32() & r.next_u32() & r.next_u32();
        if seed % 17 != 16 {
            mask &= !1;
        }
        c.bus.write32(OC1468_CTRL + 4, mask);
        c.bus
            .write32(oc1468_result_word(0x0022_e920), seed >> 1 & 1);
        c.bus
            .write32(oc1468_result_word(0x0023_1d60), (seed % 13 == 12) as u32);
        c.bus
            .write32(oc1468_result_word(0x002c_c098), seed >> 3 & 1);
        // Globals: the graphics object, the 004a28a4 object and the three plain globals.
        c.bus.write32(GAME_GP - 0x854, OC1468_GFX);
        c.bus.write32(GAME_GP - 0x84c, OC1468_X);
        c.bus.write32(GAME_GP + 0x1450, seed >> 2 & 1);
        c.bus.write32(GAME_GP + 0x410, OC1468_ARENA + 0x1_2340);
        c.bus.write32(GAME_GP + 0x21e4, OC1468_ARENA + 0x2_3450);
        // 004a28a4 + 0x504: a UI-only frame for some seeds.
        c.bus
            .write32(OC1468_X + 0x504, ((seed / 4) % 3 == 2) as u32);
        // The graphics object: vtable, render-state block, layer indices.
        c.bus.write32(OC1468_GFX + 0x10d8, OC1468_ARENA + 0x2_0000);
        c.bus.write32(OC1468_GFX + 0xe84, OC1468_TOP);
        c.bus.write32(OC1468_GFX + 0x24, r.next_u32() % 4);
        c.bus.write32(OC1468_GFX + 0x44, r.next_u32() % 4);
        // The game object.
        let s = OC1468_SELF;
        c.bus.write32(s + 0x84, OC1468_VIEWS);
        c.bus.write32(s + 0x1c, OC1468_TARGET);
        c.bus.write32(s + 0x24, OC1468_HUD);
        c.bus.write32(s + 0x9c, (seed >> 2 & 1) ^ 1);
        c.bus.write32(OC1468_VIEWS + 0x10, seed % 4);
        // Three cameras at views+4..+0x10 (views+0x10 is the view count: a fourth
        // pointer there made the loop endless).
        for i in 0..3u32 {
            c.bus
                .write32(OC1468_VIEWS + 4 + 4 * i, OC1468_CAMS + 0x80 * i);
            // The camera: fov, near and far clip, then the view matrix at +0x40.
            for k in 0..0x20u32 {
                let f = match k {
                    0 => 0.9f32,
                    1 => 0.5 + (r.next_u32() % 8) as f32,
                    2 => 100.0 + (r.next_u32() % 900) as f32,
                    _ => ((r.next_u32() % 2000) as f32 - 1000.0) / 100.0,
                };
                c.bus.write32(OC1468_CAMS + 0x80 * i + 4 * k, f.to_bits());
            }
        }
        for k in 0..0x10u32 {
            let f = (r.next_u32() % 400) as f32 / 100.0;
            c.bus.write32(OC1468_HUD + 4 * k, f.to_bits());
        }
        c.g[A0][0] = s as u64;
        c.pc = 0x0022_b008;
    }

    /// `GameModule_Render` against the generated body: random arenas, every callee faked.
    #[test]
    fn game_module_render_matches() {
        crate::difftest::check(
            ee::s05::f_0022b008__generated,
            game_module_render,
            48,
            &|c, seed| oc1468_setup(c, seed),
        );
    }

    /// The seeds reach the early exits, the view loop (0-3 views) and the tail: several
    /// distinct cycle totals, some normal returns and some long runs.
    #[test]
    fn game_module_render_paths() {
        let mut seen = std::collections::BTreeSet::new();
        let (mut returned, mut long) = (0, 0);
        for seed in 0..48 {
            let mut c = fresh();
            oc1468_setup(&mut c, seed);
            let before = c.cycles;
            difftest::run_generated(&mut c, ee::s05::f_0022b008__generated);
            seen.insert(c.cycles - before);
            returned += (c.pc == difftest::RA) as u32;
            long += (c.cycles - before > 1500) as u32;
        }
        assert!(seen.len() >= 8, "paths {seen:?}");
        assert!(returned >= 8, "returned {returned}");
        assert!(long >= 4, "long runs {long}");
    }
}
