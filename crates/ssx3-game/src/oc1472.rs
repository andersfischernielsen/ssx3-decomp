//! Staging module oc1472 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1472`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x002e_c478, lens_fx_v04, "LensFX_v04")];

/// `int LensFX_v04(PS2LensFxMan* self, float* rect, float* out)` (0x2ec478,
/// reference name `cPS2LensFxMan_sampleZBuffer`): measures how much of a
/// screen rectangle is not hidden by geometry. It builds two GIF packets in
/// `self + 0x64a0` / `+ 0x64c0` (BITBLTBUF from the current render target,
/// TRXPOS, TRXREG, TRXDIR = GS -> EE), reads the Z buffer rectangle back
/// through VIF1 DMA into `self + 0x6510` (24-bit pixels, `params[6]` x
/// `params[7]`), then counts the pixels inside the rectangle
/// (`params[0..4]`, shifted by `params[4..6]`) and those whose Z is at most
/// the threshold (`params[9]`). `*out` is the visible fraction, or an
/// over/under estimate normalised by half the rectangle when the rectangle is
/// partly off the buffer. Returns 1. The hardware waits poll the VIF1/GIF
/// CHCR, VIF1_STAT and GS CSR registers.
pub fn lens_fx_v04(c: &mut Cpu) {
    guest(c, |c| {
'b_2ec634: {
c.cycles += 102;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S2][0] = c.g[A1][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0x24));
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x10));
c.sw((c.g[SP][0] as u32), c.g[A2][0]);
c.cvt_w_s(3, 0);
c.mfc1(30, 3);
c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x14));
c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x18));
c.lwc1(3, (c.g[S2][0] as u32).wrapping_add(0x1c));
c.cvt_w_s(0, 1);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x8));
c.cvt_w_s(0, 2);
c.mfc1(19, 0);
c.cvt_w_s(0, 3);
c.mfc1(23, 0);
c.lwc1(4, (c.g[S2][0] as u32));
c.lwc1(5, (c.g[S2][0] as u32).wrapping_add(0x4));
c.cvt_w_s(0, 4);
c.mfc1(21, 0);
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0xc));
c.cvt_w_s(2, 1);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.cvt_w_s(2, 5);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xc));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
// __fixunssfsi(params[9]): the Z threshold.
c.g[RA][0] = sx(0x002ec520);
c.g[S1][0] = c.g[A0][0];
call_ptr(c, 0x004139f8, 0x002ec520)?;
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
c.g[A3][0] = 0xa4;
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[A0][0] = 0x10000000;
c.g[A0][0] = c.g[A0][0] << 32;
c.g[A0][0] = c.g[A0][0] | 0x8001;
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x59c8));
c.g[T2][0] = 0xe;
c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x59cc));
c.g[A1][0] = 0x61;
c.sd((c.g[S1][0] as u32).wrapping_add(0x64a0), c.g[A0][0]);
c.mult(0, 2, 2, 7);
c.g[A2][0] = 0x10000000;
c.g[A2][0] = c.g[A2][0] << 32;
c.g[A2][0] = c.g[A2][0] | 0x8004;
c.g[T1][0] = c.g[S7][0] << 32;
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.g[T1][0] = c.g[S3][0] | c.g[T1][0];
c.sd((c.g[S1][0] as u32).wrapping_add(0x64c0), c.g[A2][0]);
c.g[T3][0] = 0x52;
c.sd((c.g[S1][0] as u32).wrapping_add(0x64c8), c.g[T2][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sd((c.g[S1][0] as u32).wrapping_add(0x64a8), c.g[T2][0]);
c.g[A3][0] = c.g[A3][0] << 16;
c.sd((c.g[S1][0] as u32).wrapping_add(0x64b8), c.g[A1][0]);
c.g[A3][0] = c.g[FP][0] | c.g[A3][0];
c.sd((c.g[S1][0] as u32).wrapping_add(0x64b0), 0u64);
c.g[A2][0] = 0x50;
c.g[T2][0] = 0x51;
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x5a80));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x88));
c.g[T0][0] = 0x1;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
c.g[A1][0] = c.lwu((c.g[V1][0] as u32).wrapping_add(0x98));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[A0][0] = ((c.g[A0][0] as i64) >> 6) as u64;
c.g[A0][0] = c.g[A0][0] << 16;
c.g[V0][0] = c.g[V0][0] >> 32;
c.g[V1][0] = c.g[A1][0] << 24;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A1][0] = c.g[A1][0] << 56;
c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
c.g[V1][0] = 0x53;
c.sd((c.g[S1][0] as u32).wrapping_add(0x64d0), c.g[V0][0]);
c.g[A0][0] = 0u64;
c.sd((c.g[S1][0] as u32).wrapping_add(0x6508), c.g[V1][0]);
c.sd((c.g[S1][0] as u32).wrapping_add(0x64d8), c.g[A2][0]);
c.sd((c.g[S1][0] as u32).wrapping_add(0x64e0), c.g[A3][0]);
c.sd((c.g[S1][0] as u32).wrapping_add(0x64e8), c.g[T2][0]);
c.sd((c.g[S1][0] as u32).wrapping_add(0x64f0), c.g[T1][0]);
c.sd((c.g[S1][0] as u32).wrapping_add(0x64f8), c.g[T3][0]);
// FlushCache(0), then wait for VIF1/GIF DMA, VIF1 and the GS FIFO to go idle.
c.g[RA][0] = sx(0x002ec5f8);
c.sd((c.g[S1][0] as u32).wrapping_add(0x6500), c.g[T0][0]);
call_ptr(c, 0x00424020, 0x002ec5f8)?;
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] | 0x9000;
c.g[V1][0] = c.lw((c.g[V0][0] as u32));
c.g[V1][0] = c.g[V1][0] & 0x100;
let t = c.g[V1][0] == 0u64;
c.g[V1][0] = 0x10000000;
if t {
break 'b_2ec634;
} else {
c.cycles += 2;
c.g[V1][0] = c.g[V1][0] | 0x9000;
'l_2ec618: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x002ec618); } continue 'l_2ec618;
} else {
break 'b_2ec634;
}
}
}
}
c.cycles += 7;
c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6510));
c.g[V1][0] = 0x10000000;
c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x64a0));
c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x64c0));
c.g[V1][0] = c.g[V1][0] | 0xa000;
'l_2ec650: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] != 0u64;
c.g[A0][0] = 0x10000000;
if t {
if event_due(c) { deliver_event_here(c, 0x002ec650); } continue 'l_2ec650;
} else {
c.cycles += 3;
c.g[V1][0] = 0x1f000000;
c.g[A0][0] = c.g[A0][0] | 0x3c00;
c.g[V1][0] = c.g[V1][0] | 0x3;
'l_2ec678: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x002ec678); } continue 'l_2ec678;
} else {
c.cycles += 3;
c.g[V1][0] = 0x12000000;
c.g[A0][0] = 0x4000;
c.g[V1][0] = c.g[V1][0] | 0x1000;
'l_2ec6a0: loop {
c.cycles += 7;
c.g[V0][0] = c.ld((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0xc000;
let t = c.g[V0][0] != c.g[A0][0];
c.g[S0][0] = 0x12000000;
if t {
if event_due(c) { deliver_event_here(c, 0x002ec6a0); } continue 'l_2ec6a0;
} else {
'b_2ec71c: {
c.cycles += 15;
c.g[V0][0] = 0x2;
c.g[S0][0] = c.g[S0][0] | 0x1000;
c.g[A0][0] = 0u64;
c.g[A1][0] = 0u64;
c.sd((c.g[S0][0] as u32), c.g[V0][0]);
// Clear CSR.FINISH, sceGsSyncPath(0, 0), send the 2-qword setup packet.
c.g[RA][0] = sx(0x002ec6d8);
call_ptr(c, 0x003fdb18, 0x002ec6d8)?;
c.g[A0][0] = c.g[S4][0];
c.g[A1][0] = 0x2;
c.g[RA][0] = sx(0x002ec6e8);
c.g[A2][0] = 0x6;
call_ptr(c, 0x00371940, 0x002ec6e8)?;
c.g[V0][0] = c.ld((c.g[S0][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x2;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x12000000;
if t {
break 'b_2ec71c;
} else {
c.cycles += 2;
c.g[V1][0] = 0x12000000;
c.g[V1][0] = c.g[V1][0] | 0x1000;
'l_2ec700: loop {
c.cycles += 7;
c.g[V0][0] = c.ld((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x12000000;
if t {
if event_due(c) { deliver_event_here(c, 0x002ec700); } continue 'l_2ec700;
} else {
break 'b_2ec71c;
}
}
}
}
'b_2ec7dc: {
c.cycles += 39;
c.g[V1][0] = 0x2;
c.g[V0][0] = c.g[V0][0] | 0x1000;
c.g[A0][0] = c.g[S6][0];
c.sd((c.g[V0][0] as u32), c.g[V1][0]);
c.g[A1][0] = 0x5;
// Send the 5-qword packet, switch the bus to GS -> EE, DMA the Z rectangle into the buffer.
c.g[RA][0] = sx(0x002ec738);
c.g[A2][0] = 0x6;
call_ptr(c, 0x00371940, 0x002ec738)?;
c.g[V0][0] = 0x10000000;
c.g[A0][0] = 0x800000;
c.g[V0][0] = c.g[V0][0] | 0x3c00;
c.g[V1][0] = 0x12000000;
c.sw((c.g[V0][0] as u32), c.g[A0][0]);
c.g[V1][0] = c.g[V1][0] | 0x1040;
c.g[A0][0] = 0x1;
c.g[V0][0] = 0x10000000;
c.sd((c.g[V1][0] as u32), c.g[A0][0]);
c.g[V0][0] = c.g[V0][0] | 0x9010;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
c.g[A1][0] = 0x10000000;
c.g[A1][0] = c.g[A1][0] | 0x9020;
c.g[A0][0] = 0x10000000;
c.sw((c.g[V0][0] as u32), c.g[V1][0]);
c.g[A0][0] = c.g[A0][0] | 0x9000;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A2][0] = 0x100;
c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
c.mul_s(0, 0, 1);
c.cvt_w_s(1, 0);
c.mfc1(3, 1);
c.g[V0][0] = sx((c.g[V1][0] as u32) << 1);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.sw((c.g[A1][0] as u32), c.g[V0][0]);
c.sw((c.g[A0][0] as u32), c.g[A2][0]);
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x10000000;
if t {
break 'b_2ec7dc;
} else {
c.cycles += 2;
c.g[V1][0] = c.g[V1][0] | 0x9000;
'l_2ec7c0: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x002ec7c0); } continue 'l_2ec7c0;
} else {
break 'b_2ec7dc;
}
}
}
}
c.cycles += 3;
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0xa000;
'l_2ec7e8: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] != 0u64;
c.g[A0][0] = 0x10000000;
if t {
if event_due(c) { deliver_event_here(c, 0x002ec7e8); } continue 'l_2ec7e8;
} else {
c.cycles += 3;
c.g[V1][0] = 0x1f000000;
c.g[A0][0] = c.g[A0][0] | 0x3c00;
c.g[V1][0] = c.g[V1][0] | 0x3;
'l_2ec810: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x002ec810); } continue 'l_2ec810;
} else {
c.cycles += 3;
c.g[V1][0] = 0x12000000;
c.g[A0][0] = 0x4000;
c.g[V1][0] = c.g[V1][0] | 0x1000;
'l_2ec838: loop {
c.cycles += 7;
c.g[V0][0] = c.ld((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0xc000;
let t = c.g[V0][0] != c.g[A0][0];
c.g[V0][0] = 0x10000000;
if t {
if event_due(c) { deliver_event_here(c, 0x002ec838); } continue 'l_2ec838;
} else {
'b_2ec954: {
c.cycles += 12;
c.g[V1][0] = 0x12000000;
c.g[V0][0] = c.g[V0][0] | 0x3c00;
c.g[V1][0] = c.g[V1][0] | 0x1040;
c.sw((c.g[V0][0] as u32), 0u64);
c.g[A0][0] = 0u64;
c.sd((c.g[V1][0] as u32), 0u64);
// Restore the bus direction; sceGsSyncPath(0, 0); count the pixels inside the rectangle.
c.g[RA][0] = sx(0x002ec874);
c.g[A1][0] = 0u64;
call_ptr(c, 0x003fdb18, 0x002ec874)?;
c.mtc1(0, 3);
c.g[A0][0] = 0u64;
let t = (c.g[S7][0] as i64) <= 0;
c.mov_s(5, 3);
if t {
break 'b_2ec954;
} else {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
c.g[T3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6510));
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A2][0] = 0u64;
'l_2ec898: loop {
'b_2ec944: {
c.cycles += 2;
let t = (c.g[S3][0] as i64) <= 0;
c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
if t {
break 'b_2ec944;
} else {
c.cycles += 12;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.mult(0, 3, 4, 19);
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
c.g[T1][0] = ((c.g[V0][0] as i64) < (c.g[T4][0] as i64)) as u64;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[T0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
c.g[V0][0] = sx((c.g[V1][0] as u32) << 1);
c.g[A3][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T3][0] as u32)));
'l_2ec8d0: loop {
'b_2ec938: {
c.cycles += 4;
c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S5][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
break 'b_2ec938;
} else {
c.cycles += 3;
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
if c.g[V0][0] == 0u64 {
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
break 'b_2ec938;
} else {
c.cycles += 2;
if c.g[T0][0] != 0u64 {
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
break 'b_2ec938;
} else {
c.cycles += 2;
if c.g[T1][0] == 0u64 {
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
break 'b_2ec938;
} else {
'b_2ec934: {
c.cycles += 13;
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[V1][0] = c.lbu((c.g[A1][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x2));
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
c.g[V1][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
let t = c.g[V1][0] != 0u64;
c.add_s(3, 3, 0);
if t {
break 'b_2ec934;
} else {
c.cycles += 1;
c.add_s(5, 5, 0);
break 'b_2ec934;
}
}
c.cycles += 1;
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
break 'b_2ec938;
}
}
}
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S3][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3));
if t {
if event_due(c) { deliver_event_here(c, 0x002ec8d0); } continue 'l_2ec8d0;
} else {
break 'b_2ec944;
}
}
}
}
c.cycles += 4;
c.g[A0][0] = c.g[T2][0];
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S7][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[A2][0] = 0u64;
if event_due(c) { deliver_event_here(c, 0x002ec898); } continue 'l_2ec898;
} else {
break 'b_2ec954;
}
}
}
}
'b_2ec99c: {
'b_2ec97c: {
c.cycles += 7;
c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0xc));
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x8));
c.mul_s(4, 0, 1);
c.c_eq_s(3, 4);
let t = !c.fcc();
c.g[V1][0] = c.lw((c.g[SP][0] as u32));
if t {
break 'b_2ec97c;
} else {
c.cycles += 3;
c.div_s(1, 5, 3);
let t = true;
c.g[V0][0] = 0x1;
if t {
break 'b_2ec99c;
} else {
break 'b_2ec97c;
}
}
}
c.cycles += 8;
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 0);
c.mtc1(0, 2);
c.mul_s(3, 4, 0);
c.sub_s(1, 5, 3);
c.div_s(1, 1, 3);
c.max_s(1, 1, 2);
c.g[V0][0] = 0x1;
break 'b_2ec99c;
}
c.cycles += 13;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
c.swc1(1, (c.g[V1][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
c.pc = t; return Ok(());
}
}
}
}
}
}
}
}
}
}
}
}
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, check_reaching, machine};
    use crate::difftest::{fill, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The fakes' gate word and the fixture's option bits, in the machine's RAM.
    const OC1472_CTRL: u32 = DATA + 0xe_0000;
    const OC1472_MAGIC: u32 = 0x1e45_f1c4;
    /// `self` (the packets at +0x64a0, the Z pixels at +0x6510), the parameter
    /// block, the output float.
    const OC1472_SELF: u32 = DATA + 0x4000;
    const OC1472_PARAMS: u32 = DATA + 0x300;
    const OC1472_OUT: u32 = DATA + 0x200;
    /// The renderer object behind the global at `0x4a5b80`, and the table of
    /// 0xa4-byte render targets its `+0x59cc` points at.
    const OC1472_RENDERER: u32 = DATA + 0x8_0000;
    const OC1472_TARGETS: u32 = DATA + 0x9_0000;
    const OC1472_VIF1_FIFO: u32 = 0x1000_5000;

    /// Option bits (a random 5-bit word per seed).
    const OPT_VIF1_BUSY: u32 = 1;
    const OPT_GIF_BUSY: u32 = 2;
    const OPT_FIFO_BUSY: u32 = 4;
    const OPT_FINISH_EARLY: u32 = 8;
    const OPT_FIFO_BUSY_AFTER_DMA: u32 = 16;

    static OC1472_PREV_SYNC_PATH: AtomicUsize = AtomicUsize::new(0);
    static OC1472_PREV_SEND_GIF: AtomicUsize = AtomicUsize::new(0);

    fn oc1472_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC1472_CTRL) == OC1472_MAGIC
    }

    /// Make a VIF1 FIFO that stays busy (a `DIRECT` code short of its data).
    fn oc1472_fifo_busy(c: &mut Cpu) {
        let direct = 0x5000_0002u128 | 0x11u128 << 32 | 0x22u128 << 64 | 0x33u128 << 96;
        c.bus.write128(OC1472_VIF1_FIFO, direct);
        c.next_event = c.next_event.min(c.cycles + 30);
    }

    /// `sceGsSyncPath` for this fixture: a logged fake.
    fn oc1472_fake_sync_path(c: &mut Cpu) {
        if !oc1472_gated(c) {
            return fakes::fall_back(
                c,
                &OC1472_PREV_SYNC_PATH,
                ee::s15::f_003fdb18__generated,
            );
        }
        let ra = r32(c, RA);
        fakes::fake(c, 0x003f_db18, ra);
    }

    /// `Dma_SendGifNormal` for this fixture: a logged fake. After the first
    /// packet the GS FINISH event arrives at once or only at a later event;
    /// after the second one the VIF1 FIFO may stay busy.
    fn oc1472_fake_send_gif(c: &mut Cpu) {
        if !oc1472_gated(c) {
            return fakes::fall_back(c, &OC1472_PREV_SEND_GIF, ee::s12::f_00371940__generated);
        }
        let ra = r32(c, RA);
        fakes::fake(c, 0x0037_1940, ra);
        if c.pc != ra {
            return;
        }
        let options = c.bus.read32(OC1472_CTRL + 4);
        if ra == 0x002e_c6e8 {
            if options & OPT_FINISH_EARLY != 0 {
                ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw.gs.raise_event(2);
            } else {
                c.next_event = c.next_event.min(c.cycles + 30);
            }
        } else if ra == 0x002e_c738 && options & OPT_FIFO_BUSY_AFTER_DMA != 0 {
            oc1472_fifo_busy(c);
        }
    }

    /// The fixture's event hook: ends every busy state (CHCR.STR of VIF1 and
    /// GIF, the VIF1 FIFO by a VIF1 reset), raises GS FINISH, and re-arms the
    /// next event 20 cycles away so no wait can spin for long. Keeps the test
    /// machines' runaway budget.
    fn oc1472_event(c: &mut Cpu) -> bool {
        use ssx3_ps2::hw::ch;
        assert!(
            c.cycles < crate::difftest::cycle_budget(),
            "cycle budget exceeded ({} cycles, pc {:#x}): runaway on test data",
            c.cycles,
            c.pc
        );
        {
            let hw = &mut ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw;
            for n in [ch::VIF1, ch::GIF] {
                hw.ch[n].chcr &= !0x100u32;
            }
            if hw.vif1.pending_words() > 0 {
                hw.vif1.write_reg(0x10, 1);
            }
            hw.gs.raise_event(2);
        }
        c.next_event = c.cycles + 20;
        false
    }

    fn oc1472_hook() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_003FDB18),
            oc1472_fake_sync_path,
            &OC1472_PREV_SYNC_PATH,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00371940),
            oc1472_fake_send_gif,
            &OC1472_PREV_SEND_GIF,
        );
    }

    /// A fixture: a Z buffer of `bw` x `bh` random 24-bit pixels, a rectangle
    /// that is inside it (every third seed: the "all pixels counted" path) or
    /// random and partly outside, random Z threshold, a render target and
    /// the hardware busy states of the option word.
    fn oc1472_setup(c: &mut Cpu, seed: u32) {
        oc1472_hook();
        let mut r: Rng = machine(c, seed, 0x2e_c478, 5);
        c.hooks.event = oc1472_event;
        c.bus.write32(OC1472_CTRL, OC1472_MAGIC);
        let options = r.next_u32() & 0x1f;
        c.bus.write32(OC1472_CTRL + 4, options);

        c.bus.write32(0x004a_5b80, OC1472_RENDERER);
        c.bus.write32(OC1472_RENDERER + 0x59c8, r.next_u32() % 3);
        c.bus.write32(OC1472_RENDERER + 0x59cc, OC1472_TARGETS);
        c.bus.write32(OC1472_RENDERER + 0x5a80, r.next_u32() % 0x400);
        fill(c, &mut r, OC1472_TARGETS, 0x80);

        let inside = seed % 3 == 0;
        let (x0, y0, rw, rh, xo, yo, bw, bh);
        if inside {
            x0 = (r.next_u32() % 3) as f32;
            y0 = (r.next_u32() % 3) as f32;
            rw = (1 + r.next_u32() % 4) as f32;
            rh = (1 + r.next_u32() % 4) as f32;
            xo = 0.0;
            yo = 0.0;
            bw = (x0 + rw) as u32 + r.next_u32() % 3;
            bh = (y0 + rh) as u32 + r.next_u32() % 3;
        } else {
            let frac = |r: &mut Rng| if r.next_u32() % 4 == 0 { 0.5 } else { 0.0 };
            x0 = (r.next_u32() % 6) as f32 - 1.0 + frac(&mut r);
            y0 = (r.next_u32() % 6) as f32 - 1.0 + frac(&mut r);
            rw = (1 + r.next_u32() % 5) as f32 + frac(&mut r);
            rh = (1 + r.next_u32() % 5) as f32 + frac(&mut r);
            xo = (r.next_u32() % 4) as f32 + frac(&mut r);
            yo = (r.next_u32() % 4) as f32 + frac(&mut r);
            bw = if r.next_u32() % 10 == 0 { 0 } else { 1 + r.next_u32() % 7 };
            bh = r.next_u32() % 7;
        }
        let threshold = (r.next_u32() & 0x00ff_ffff) as f32;
        let params = [x0, y0, rw, rh, xo, yo, bw as f32, bh as f32, 0.0, threshold];
        for (i, v) in params.into_iter().enumerate() {
            c.bus.write32(OC1472_PARAMS + 4 * i as u32, v.to_bits());
        }
        fill(c, &mut r, OC1472_SELF + 0x6510, 0x40);

        {
            use ssx3_ps2::hw::ch;
            let hw = &mut ssx3_ps2::platform::machine_of(&mut c.bus.plat).hw;
            if options & OPT_VIF1_BUSY != 0 {
                hw.ch[ch::VIF1].chcr |= 0x100;
            }
            if options & OPT_GIF_BUSY != 0 {
                hw.ch[ch::GIF].chcr |= 0x100;
            }
        }
        if options & (OPT_VIF1_BUSY | OPT_GIF_BUSY) != 0 {
            c.next_event = c.next_event.min(30);
        }
        if options & OPT_FIFO_BUSY != 0 {
            oc1472_fifo_busy(c);
        }
        c.g[A0][0] = sx(OC1472_SELF);
        c.g[A1][0] = sx(OC1472_PARAMS);
        c.g[A2][0] = sx(OC1472_OUT);
    }

    /// `LensFX_v04` against the generated body: rectangles inside and partly
    /// outside the Z buffer, empty buffers, both Z comparisons, the hardware
    /// waits spinning until an event ends them, and an unwind at each call.
    #[test]
    fn lens_fx_v04_matches() {
        check_reaching(
            ee::s09::f_002ec478__generated,
            lens_fx_v04,
            300,
            &[0x002e_c6d8, 0x002e_c6e8, 0x002e_c738, 0x002e_c874],
            &oc1472_setup,
        );
    }

    /// The seeds reach many different paths of the generated body (the cycle
    /// totals differ per pixel count, branch and wait).
    #[test]
    fn lens_fx_v04_fixture_covers_paths() {
        let mut seen = std::collections::BTreeSet::new();
        let mut outs = std::collections::BTreeSet::new();
        for seed in 0..300 {
            let mut c = crate::difftest::fresh();
            oc1472_setup(&mut c, seed);
            let before = c.cycles;
            ee::s09::f_002ec478__generated(&mut c);
            if c.pc == crate::difftest::RA {
                seen.insert(c.cycles - before);
                outs.insert(c.bus.read32(OC1472_OUT));
            }
        }
        assert!(seen.len() >= 25, "only {} distinct paths: {seen:?}", seen.len());
        assert!(outs.len() >= 8, "only {} distinct results", outs.len());
    }
}
