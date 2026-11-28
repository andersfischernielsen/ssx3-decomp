//! Staging module oc1470 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1470`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0038_2af0,
    renderer_set_display_regs,
    "Renderer_SetDisplayRegs",
)];

/// `void Renderer_SetDisplayRegs(cPS2GraphicsMan* self, int en1, int waitVsync)`
/// (0x00382af0): advance the frame counter and pick the draw/display buffer
/// pages, wait for the DMA/GIF/GS FIFOs to drain (and for vsync), program
/// PMODE/SMODE2/DISPFB1/DISPLAY1/BGCOLOR, build the GS packet at 0x4ffcc0 (the
/// filter copy pass, the draw context, the brightness strips, the scissor) and
/// send it with `Dma_SendGifNormal`. Converted from the generated body: every
/// register, cycle and call kept.
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[allow(unused_assignments, unused_labels, unused_variables, unreachable_code)]
pub fn renderer_set_display_regs(c: &mut Cpu) {
    if c.pc != 0x00382af0 {
        return ee::s12::f_00382af0__generated(c);
    }
    guest(c, |c| {
'b_382b6c: {
'b_382b3c: {
c.cycles += 17;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
c.g[T9][0] = c.g[A0][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a74));
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0xf44));
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
let t = c.g[V1][0] == 0u64;
c.sw((c.g[T9][0] as u32).wrapping_add(0x5a74), c.g[A0][0]);
if t {
break 'b_382b3c;
} else {
c.cycles += 2;
let t = true;
c.sw((c.g[T9][0] as u32).wrapping_add(0xf44), 0u64);
if t {
break 'b_382b6c;
} else {
break 'b_382b3c;
}
}
}
'b_382b64: {
'b_382b60: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x59e8));
if c.g[V0][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a78));
break 'b_382b60;
} else {
c.cycles += 3;
c.g[V0][0] = c.g[A0][0] & 0x1;
if c.g[V0][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a78));
break 'b_382b60;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a7c));
let t = true;
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a78));
if t {
break 'b_382b64;
} else {
break 'b_382b60;
}
}
}
}
c.cycles += 1;
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a7c));
break 'b_382b64;
}
c.cycles += 2;
c.sw((c.g[T9][0] as u32).wrapping_add(0x5a84), c.g[V0][0]);
c.sw((c.g[T9][0] as u32).wrapping_add(0x5a88), c.g[V1][0]);
break 'b_382b6c;
}
'b_382bac: {
c.cycles += 7;
c.g[V1][0] = 0x10000000;
c.g[A1][0] = c.g[A1][0] << 32;
c.g[V1][0] = c.g[V1][0] | 0x9000;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x10000000;
if t {
break 'b_382bac;
} else {
c.cycles += 2;
c.g[V1][0] = c.g[V1][0] | 0x9000;
'l_382b90: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x00382b90); } continue 'l_382b90;
} else {
break 'b_382bac;
}
}
}
}
c.cycles += 3;
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0xa000;
'l_382bb8: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x00382bb8); } continue 'l_382bb8;
} else {
c.cycles += 3;
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0xd000;
'l_382be0: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x100;
let t = c.g[V0][0] != 0u64;
c.g[A0][0] = 0x10000000;
if t {
if event_due(c) { deliver_event_here(c, 0x00382be0); } continue 'l_382be0;
} else {
c.cycles += 3;
c.g[V1][0] = 0x1f000000;
c.g[A0][0] = c.g[A0][0] | 0x3c00;
c.g[V1][0] = c.g[V1][0] | 0x3;
'l_382c08: loop {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x00382c08); } continue 'l_382c08;
} else {
c.cycles += 3;
c.g[V1][0] = 0x12000000;
c.g[A0][0] = 0x4000;
c.g[V1][0] = c.g[V1][0] | 0x1000;
'l_382c30: loop {
c.cycles += 7;
c.g[V0][0] = c.ld((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0xc000;
let t = c.g[V0][0] != c.g[A0][0];

if t {
if event_due(c) { deliver_event_here(c, 0x00382c30); } continue 'l_382c30;
} else {
'b_382c9c: {
c.cycles += 2;
let t = c.g[A2][0] == 0u64;
c.g[V0][0] = 0x12000000;
if t {
break 'b_382c9c;
} else {
'b_382c94: {
c.cycles += 7;
c.g[A0][0] = 0x8;
c.g[V0][0] = c.g[V0][0] | 0x1000;
c.sd((c.g[V0][0] as u32), c.g[A0][0]);
c.g[V1][0] = c.ld((c.g[V0][0] as u32));
c.g[V1][0] = c.g[V1][0] & 0x8;
let t = c.g[V1][0] != 0u64;
c.g[V0][0] = 0x12000000;
if t {
break 'b_382c94;
} else {
c.cycles += 2;
c.g[V1][0] = 0x12000000;
c.g[V1][0] = c.g[V1][0] | 0x1000;
'l_382c78: loop {
c.cycles += 7;
c.g[V0][0] = c.ld((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & 0x8;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x12000000;
if t {
if event_due(c) { deliver_event_here(c, 0x00382c78); } continue 'l_382c78;
} else {
break 'b_382c94;
}
}
}
}
c.cycles += 2;
c.g[V0][0] = c.g[V0][0] | 0x1000;
c.g[V1][0] = c.ld((c.g[V0][0] as u32));
break 'b_382c9c;
}
}
'b_382e64: {
'b_382da4: {
c.cycles += 25;
c.g[V0][0] = c.g[A1][0] >> 32;
c.g[V1][0] = 0xff20;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A0][0] = 0x12000000;
c.sd((c.g[A0][0] as u32), c.g[V0][0]);
c.g[A1][0] = 0x12000000;
c.g[A1][0] = c.g[A1][0] | 0x20;
c.g[A2][0] = 0x12000000;
c.g[V1][0] = c.lh((c.g[T9][0] as u32).wrapping_add(0x5a46));
c.g[A2][0] = c.g[A2][0] | 0x70;
c.g[V0][0] = c.lh((c.g[T9][0] as u32).wrapping_add(0x5a44));
c.g[V1][0] = c.g[V1][0] << 1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[A1][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a5c));
c.g[V0][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a88));
c.g[A0][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a38));
c.g[V1][0] = c.g[V1][0] << 9;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A0][0] = c.g[A0][0] << 15;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.sd((c.g[A2][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a4c));
let t = c.g[V1][0] == 0u64;
c.g[A1][0] = 0x60e;
if t {
break 'b_382da4;
} else {
'b_382d30: {
c.cycles += 12;
c.g[A0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a54));
c.lwc1(0, (c.g[T9][0] as u32).wrapping_add(0x4));
c.g[A3][0] = 0x12000000;
c.div(0, 5, 4);
c.lwc1(1, (c.g[T9][0] as u32));
c.g[AT][0] = 0x40800000;
c.mtc1(1, 3);
c.add_s(0, 0, 0);
c.g[AT][0] = 0x41400000;
c.mtc1(1, 2);
if c.g[A0][0] == 0u64 {
break 'b_382d30;
} else {
break 'b_382d30;
}
}
c.cycles += 29;
c.mul_s(1, 1, 3);
c.g[AT][0] = 0xffffffffc22c0000;
c.mtc1(1, 4);
c.sub_s(2, 2, 0);
c.g[A2][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a58));
c.g[A3][0] = c.g[A3][0] | 0x80;
c.add_s(1, 1, 4);
c.cvt_w_s(0, 2);
c.mfc1(3, 0);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
c.g[A2][0] = c.g[A2][0] << 44;
c.cvt_w_s(0, 1);
c.mfc1(2, 0);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x23));
c.g[V1][0] = c.g[V1][0] << 12;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x158));
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A1][0] = c.lo[0];
c.mult(0, 4, 4, 5);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
c.g[A1][0] = c.g[A1][0] << 23;
c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
c.g[A0][0] = c.g[A0][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
c.sd((c.g[A3][0] as u32), c.g[V0][0]);
let t = true;
c.g[V0][0] = 0x12000000;
if t {
break 'b_382e64;
} else {
break 'b_382da4;
}
}
}
'b_382dc0: {
c.cycles += 3;
c.g[V0][0] = c.lh((c.g[T9][0] as u32).wrapping_add(0x5a44));
let t = c.g[V0][0] == 0u64;
c.g[S2][0] = 0x1;
if t {
break 'b_382dc0;
} else {
c.cycles += 4;
c.g[V0][0] = c.lh((c.g[T9][0] as u32).wrapping_add(0x5a46));
c.g[V1][0] = 0x2;
c.g[V0][0] = c.g[V0][0] ^ 0x1;
if c.g[V0][0] == 0 { c.g[S2][0] = c.g[V1][0]; }
break 'b_382dc0;
}
}
c.cycles += 41;
c.lwc1(12, (c.g[T9][0] as u32));
c.g[S1][0] = 0x12000000;
c.g[AT][0] = 0x41000000;
c.mtc1(1, 1);
c.g[S1][0] = c.g[S1][0] | 0x80;
c.g[AT][0] = 0x40a00000;
c.mtc1(1, 2);
c.mul_s(12, 12, 1);
c.g[AT][0] = 0x441f0000;
c.mtc1(1, 0);
c.sq(25, (c.g[SP][0] as u32));
c.add_s(12, 12, 2);
c.g[RA][0] = sx(0x00382df8);
c.add_s(12, 12, 0);
call_ptr(c, 0x00411340, 0x00382df8)?;
c.lq(25, (c.g[SP][0] as u32));
c.g[S0][0] = c.g[V0][0];
c.g[AT][0] = 0x42480000;
c.mtc1(1, 0);
c.lwc1(12, (c.g[T9][0] as u32).wrapping_add(0x4));
c.add_s(12, 12, 12);
c.g[RA][0] = sx(0x00382e18);
c.sub_s(12, 0, 12);
call_ptr(c, 0x00411340, 0x00382e18)?;
c.lq(25, (c.g[SP][0] as u32));
c.g[V0][0] = c.g[V0][0] << 12;
c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a34));
c.g[A0][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a64));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
c.g[V0][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a60));
c.mult(0, 3, 3, 18);
c.g[A0][0] = c.g[A0][0] << 23;
c.g[S0][0] = c.g[S0][0] | c.g[A0][0];
c.g[V0][0] = c.g[V0][0] << 27;
c.g[A0][0] = 0xa000;
c.g[A0][0] = c.g[A0][0] << 28;
c.g[S0][0] = c.g[S0][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 44;
c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
c.g[S0][0] = c.g[S0][0] | c.g[V1][0];
c.sd((c.g[S1][0] as u32), c.g[S0][0]);
c.g[V0][0] = 0x12000000;
break 'b_382e64;
}
'b_3833a8: {
c.cycles += 8;
c.g[V1][0] = 0x500000;
c.g[V0][0] = c.g[V0][0] | 0xe0;
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x340));
c.sd((c.g[V0][0] as u32), 0u64);
c.g[V1][0] = 0x30000000;
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x59e8));
let t = c.g[V0][0] == 0u64;
c.g[FP][0] = c.g[A0][0] | c.g[V1][0];
if t {
break 'b_3833a8;
} else {
'b_383094: {
'b_38307c: {
c.cycles += 122;
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V0][0] = c.g[V0][0] | 0xb;
c.g[V1][0] = 0xe;
c.sd((c.g[FP][0] as u32), c.g[V0][0]);
c.g[T0][0] = 0xff00;
c.g[T0][0] = c.g[T0][0] << 48;
c.sd((c.g[FP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.g[A1][0] = 0x4c;
c.g[A2][0] = 0x61;
c.g[A3][0] = 0x14;
c.g[V1][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a5c));
c.g[T3][0] = 0x9a00;
c.g[T3][0] = c.g[T3][0] << 18;
c.g[V0][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a88));
c.g[T1][0] = 0x6;
c.g[A0][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a38));
c.g[V1][0] = c.g[V1][0] << 16;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x18), c.g[A1][0]);
c.g[A0][0] = c.g[A0][0] << 24;
c.sd((c.g[FP][0] as u32).wrapping_add(0x20), c.g[A2][0]);
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x28), c.g[A3][0]);
c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
c.g[A1][0] = 0x80;
c.g[A1][0] = c.g[A1][0] << 32;
c.g[A1][0] = c.g[A1][0] | 0x64;
c.sd((c.g[FP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
c.g[A2][0] = 0x42;
c.g[A3][0] = 0x5;
c.g[T0][0] = 0x8;
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a84));
c.g[T2][0] = 0x40;
c.g[A0][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a50));
c.g[T4][0] = 0x41;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
c.g[V1][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a3c));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[A0][0] = c.g[A0][0] << 14;
c.g[V0][0] = c.g[V0][0] >> 32;
c.g[V1][0] = c.g[V1][0] << 20;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x38), c.g[T1][0]);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x40), c.g[A1][0]);
c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x48), c.g[A2][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0x50), c.g[A3][0]);
c.g[A0][0] = 0x800;
c.sd((c.g[FP][0] as u32).wrapping_add(0x58), c.g[T0][0]);
c.g[A1][0] = 0x18;
c.sd((c.g[FP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
c.g[T3][0] = 0x8000;
c.g[T3][0] = c.g[T3][0] << 17;
c.g[T6][0] = 0x4e;
c.g[T7][0] = 0x30000;
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a54));
c.g[T5][0] = 0x47;
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a58));
c.g[T0][0] = 0x3f;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.sd((c.g[FP][0] as u32).wrapping_add(0x68), c.g[T2][0]);
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = c.g[V1][0] << 48;
c.g[V0][0] = c.g[V0][0] >> 16;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[T1][0] = 0x11e;
c.sd((c.g[FP][0] as u32).wrapping_add(0x60), c.g[V0][0]);
c.g[A2][0] = 0x24000000;
c.g[A2][0] = c.g[A2][0] << 32;
c.g[A2][0] = c.g[A2][0] | 0x1;
c.g[A3][0] = 0x10;
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a54));
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a58));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.sd((c.g[FP][0] as u32).wrapping_add(0x78), c.g[T4][0]);
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = c.g[V1][0] << 48;
c.g[V0][0] = c.g[V0][0] >> 16;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x70), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a54));
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a58));
c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
c.sd((c.g[FP][0] as u32).wrapping_add(0x88), c.g[A1][0]);
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V1][0] = c.g[V1][0] << 32;
c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
c.g[V1][0] = c.g[V1][0] >> 32;
c.g[A0][0] = c.g[A0][0] << 32;
c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x80), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a40));
c.g[V1][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a80));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V0][0] = c.g[V0][0] >> 8;
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] | c.g[T3][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x90), c.g[V1][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0x98), c.g[T6][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0xa0), c.g[T7][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0xa8), c.g[T5][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0xb8), c.g[T0][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0xb0), 0u64);
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x59f8));
c.sd((c.g[FP][0] as u32).wrapping_add(0xc0), c.g[A2][0]);
c.g[V0][0] = c.g[V0][0] ^ 0x80;
c.sd((c.g[FP][0] as u32).wrapping_add(0xc8), c.g[A3][0]);
c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
c.g[V0][0] = c.g[V0][0] << 6;
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0xd0), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x59ec));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_38307c;
} else {
c.cycles += 4;
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
let t = true;
c.lwc1(0, (c.g[T9][0] as u32).wrapping_add(0x59fc));
if t {
break 'b_383094;
} else {
break 'b_38307c;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.add_s(1, 1, 1);
c.lwc1(0, (c.g[T9][0] as u32).wrapping_add(0x59fc));
break 'b_383094;
}
'b_3830c4: {
'b_3830c0: {
c.cycles += 7;
c.mov_s(2, 0);
c.mul_s(0, 1, 0);
c.cvt_w_s(1, 0);
c.mfc1(2, 1);
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = 0xff;
if t {
break 'b_3830c0;
} else {
c.cycles += 4;
c.g[A2][0] = c.g[V0][0];
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
let t = true;
if c.g[V0][0] != 0 { c.g[A2][0] = c.g[V1][0]; }
if t {
break 'b_3830c4;
} else {
break 'b_3830c0;
}
}
}
c.cycles += 1;
c.g[A2][0] = 0u64;
break 'b_3830c4;
}
'b_3830f8: {
'b_3830e0: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x59f0));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_3830e0;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 0, 2);
if t {
break 'b_3830f8;
} else {
break 'b_3830e0;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 0, 2);
break 'b_3830f8;
}
'b_383120: {
'b_38311c: {
c.cycles += 5;
c.cvt_w_s(1, 0);
c.mfc1(2, 1);
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = 0xff;
if t {
break 'b_38311c;
} else {
c.cycles += 4;
c.g[A1][0] = c.g[V0][0];
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
let t = true;
if c.g[V0][0] != 0 { c.g[A1][0] = c.g[V1][0]; }
if t {
break 'b_383120;
} else {
break 'b_38311c;
}
}
}
c.cycles += 1;
c.g[A1][0] = 0u64;
break 'b_383120;
}
'b_383154: {
'b_38313c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x59f4));
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
if t {
break 'b_38313c;
} else {
c.cycles += 4;
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
let t = true;
c.mul_s(0, 0, 2);
if t {
break 'b_383154;
} else {
break 'b_38313c;
}
}
}
c.cycles += 6;
c.g[V0][0] = c.g[V0][0] & 0x1;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 0);
c.mul_s(0, 0, 2);
break 'b_383154;
}
'b_38317c: {
'b_383178: {
c.cycles += 5;
c.cvt_w_s(1, 0);
c.mfc1(2, 1);
let t = (c.g[V0][0] as i64) < 0;
c.g[V1][0] = 0xff;
if t {
break 'b_383178;
} else {
c.cycles += 4;
c.g[A0][0] = c.g[V0][0];
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
let t = true;
if c.g[V0][0] != 0 { c.g[A0][0] = c.g[V1][0]; }
if t {
break 'b_38317c;
} else {
break 'b_383178;
}
}
}
c.cycles += 1;
c.g[A0][0] = 0u64;
break 'b_38317c;
}
'b_3831c8: {
c.cycles += 19;
c.g[V1][0] = c.g[A1][0];
c.g[V1][0] = c.g[V1][0] << 8;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.g[V1][0] = c.g[A2][0] | c.g[V1][0];
c.g[A0][0] = c.g[A0][0] << 16;
c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
c.g[A1][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x59f8));
c.mfc1(4, 0);
c.g[A1][0] = c.g[A1][0] << 24;
c.g[V0][0] = c.g[A0][0];
c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
c.g[V0][0] = c.g[V0][0] << 32;
c.g[A0][0] = 0x20;
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[A1][0] = 0x8000;
c.sd((c.g[FP][0] as u32).wrapping_add(0xd8), c.g[V1][0]);
if c.g[A0][0] == 0u64 {
break 'b_3831c8;
} else {
break 'b_3831c8;
}
}
'b_383228: {
'b_38321c: {
c.cycles += 17;
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a54));
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a30));
c.divu(0, 2, 4);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A2][0] = c.lo[0];
c.divu(0, 3, 6);
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a58));
c.g[A0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a4c));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.g[V1][0] = c.lo[0];
let t = c.g[A0][0] == 0u64;
c.g[T3][0] = sx((c.g[V1][0] as u32) << 4);
if t {
break 'b_38321c;
} else {
c.cycles += 4;
c.g[T1][0] = 0x8;
c.g[T2][0] = 0u64;
let t = true;
c.g[A3][0] = 0u64;
if t {
break 'b_383228;
} else {
break 'b_38321c;
}
}
}
c.cycles += 3;
c.g[T1][0] = 0u64;
c.g[T2][0] = 0xfffffffffffffff8;
c.g[A3][0] = 0xfffffffffffffff8;
break 'b_383228;
}
'b_3832d4: {
c.cycles += 22;
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a34));
c.g[V0][0] = 0x8800;
c.g[V0][0] = c.g[V0][0] << 47;
c.g[A0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a58));
c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.g[T0][0] = c.g[T1][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
c.g[V0][0] = 0x5353;
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
c.sd((c.g[FP][0] as u32).wrapping_add(0xe0), c.g[A1][0]);
c.g[T1][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[T4][0] as u32)));
c.sd((c.g[FP][0] as u32).wrapping_add(0xe8), c.g[V0][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8));
c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[S0][0] = c.g[T0][0];
c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[T3][0] as u32)));
let t = (c.g[A2][0] as i64) <= 0;
c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0xf0));
if t {
break 'b_3832d4;
} else {
c.cycles += 6;
c.g[V0][0] = c.g[A3][0];
c.g[T6][0] = c.g[V0][0] << 16;
c.g[T5][0] = c.g[V1][0] << 16;
c.g[T4][0] = c.g[A0][0] << 16;
c.g[A3][0] = c.g[A2][0];
c.g[T7][0] = 0x80000;
'l_383298: loop {
c.cycles += 15;
c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x200));
c.g[V0][0] = c.g[T0][0] | c.g[T7][0];
c.g[V1][0] = c.g[T1][0] | c.g[T6][0];
c.g[A1][0] = c.g[T2][0] | c.g[T5][0];
c.g[A2][0] = c.g[A0][0] | c.g[T4][0];
c.sd((c.g[FP][0] as u32), c.g[V0][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.g[T1][0] = c.g[A0][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[T3][0] as u32)));
c.sd((c.g[FP][0] as u32).wrapping_add(0x18), c.g[A2][0]);
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[T3][0] as u32)));
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
let t = c.g[A3][0] != 0u64;
c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x20));
if t {
if event_due(c) { deliver_event_here(c, 0x00383298); } continue 'l_383298;
} else {
break 'b_3832d4;
}
}
}
}
c.cycles += 53;
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a34));
c.g[A1][0] = 0x8000;
c.g[A2][0] = 0x8000;
c.g[A2][0] = c.g[A2][0] << 32;
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x8));
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V1][0] = c.g[V1][0] >> 16;
c.g[V1][0] = c.g[S0][0] | c.g[V1][0];
c.sd((c.g[FP][0] as u32), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a54));
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a58));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = sx(0u32.wrapping_sub((c.g[V0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7fe8));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V1][0] = c.g[V1][0] >> 16;
c.g[V0][0] = c.g[V0][0] >> 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a34));
c.g[A0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a30));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x8));
c.g[A0][0] = c.g[A0][0] << 32;
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V1][0] = c.g[V1][0] >> 16;
c.g[A0][0] = c.g[A0][0] >> 32;
c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x10), c.g[A0][0]);
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a54));
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a58));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x7ff8));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] >> 32;
c.g[V1][0] = c.g[V1][0] >> 16;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x20));
break 'b_3833a8;
}
}
'b_38364c: {
c.cycles += 153;
c.g[S7][0] = 0xe;
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V0][0] = c.g[V0][0] | 0x8010;
c.sd((c.g[FP][0] as u32), c.g[V0][0]);
c.g[A1][0] = 0x4c;
c.sd((c.g[FP][0] as u32).wrapping_add(0x8), c.g[S7][0]);
c.g[T1][0] = 0xff00;
c.g[T1][0] = c.g[T1][0] << 48;
c.g[A3][0] = 0x4d;
c.g[T2][0] = 0x4e;
c.g[V0][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a50));
c.g[T3][0] = 0x4f;
c.g[A0][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a84));
c.g[A2][0] = 0x800;
c.g[V1][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a3c));
c.g[V0][0] = c.g[V0][0] << 16;
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x18), c.g[A1][0]);
c.g[V1][0] = c.g[V1][0] << 24;
c.g[A1][0] = 0x18;
c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
c.g[T4][0] = 0x19;
c.sd((c.g[FP][0] as u32).wrapping_add(0x10), c.g[A0][0]);
c.g[S1][0] = 0x4a;
c.g[S2][0] = 0x4b;
c.g[S3][0] = 0x80;
c.g[S3][0] = c.g[S3][0] << 32;
c.g[S3][0] = c.g[S3][0] | 0x80;
c.g[V1][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a50));
c.g[S6][0] = 0x3b;
c.g[V0][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a84));
c.g[T0][0] = 0x1;
c.g[A0][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a3c));
c.g[V1][0] = c.g[V1][0] << 16;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x28), c.g[A3][0]);
c.g[A0][0] = c.g[A0][0] << 24;
c.g[A3][0] = 0x46;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[T5][0] = 0x1a;
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.g[A0][0] = 0x45;
c.sd((c.g[FP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.g[T1][0] = 0x44;
c.g[T7][0] = 0x49;
c.g[S0][0] = 0x22;
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a40));
c.g[T6][0] = 0x3f;
c.g[V1][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a80));
c.g[T8][0] = 0x8000;
c.g[T8][0] = c.g[T8][0] << 45;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
c.sd((c.g[FP][0] as u32).wrapping_add(0x38), c.g[T2][0]);
c.g[V0][0] = c.g[V0][0] << 32;
c.g[T2][0] = 0x30000;
c.g[V0][0] = c.g[V0][0] >> 8;
c.g[S4][0] = 0x47;
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[S5][0] = 0x6;
c.sd((c.g[FP][0] as u32).wrapping_add(0x30), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a40));
c.g[V1][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x5a80));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
c.sd((c.g[FP][0] as u32).wrapping_add(0x48), c.g[T3][0]);
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V0][0] = c.g[V0][0] >> 8;
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x40), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a30));
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a34));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.sd((c.g[FP][0] as u32).wrapping_add(0x58), c.g[A1][0]);
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] >> 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x50), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a30));
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a34));
c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
c.sd((c.g[FP][0] as u32).wrapping_add(0x68), c.g[T4][0]);
c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V1][0] = c.g[V1][0] >> 32;
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x78), c.g[S1][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0x60), c.g[V1][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0x88), c.g[S2][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0x90), c.g[S3][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0x70), 0u64);
c.sd((c.g[FP][0] as u32).wrapping_add(0x80), 0u64);
c.sd((c.g[FP][0] as u32).wrapping_add(0x98), c.g[S6][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0xb8), c.g[A3][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0xa8), c.g[T5][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0xa0), c.g[T0][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0xb0), c.g[T0][0]);
c.g[V0][0] = c.lwu((c.g[T9][0] as u32).wrapping_add(0x59d0));
c.sd((c.g[FP][0] as u32).wrapping_add(0xc8), c.g[A0][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
c.g[V1][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x59e0));
c.sd((c.g[FP][0] as u32).wrapping_add(0xd8), c.g[T1][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0xd0), c.g[V1][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0xe8), c.g[T7][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0xf8), c.g[S0][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0x108), c.g[T6][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0xe0), 0u64);
c.sd((c.g[FP][0] as u32).wrapping_add(0xf0), 0u64);
c.sd((c.g[FP][0] as u32).wrapping_add(0x100), 0u64);
c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x110));
c.g[A0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a30));
c.g[A1][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x5a34));
c.g[A3][0] = sx((c.g[A0][0] as u32) >> 5);
c.sd((c.g[FP][0] as u32).wrapping_add(0x8), c.g[S7][0]);
c.g[V0][0] = sx((c.g[A3][0] as u32) << 1);
c.sd((c.g[FP][0] as u32).wrapping_add(0x10), c.g[T2][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
c.sd((c.g[FP][0] as u32).wrapping_add(0x18), c.g[S4][0]);
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.sd((c.g[FP][0] as u32).wrapping_add(0x20), c.g[S5][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0x28), 0u64);
c.g[V0][0] = c.g[V0][0] | c.g[T8][0];
c.g[V1][0] = sx((c.g[A1][0] as u32) >> 1);
c.sd((c.g[FP][0] as u32), c.g[V0][0]);
c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[T1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V0][0] = c.lw((c.g[T9][0] as u32).wrapping_add(0x6ae0));
c.g[V1][0] = 0xffffffff80000000;
c.g[A0][0] = sx((c.g[A0][0] as u32) >> 1);
c.sd((c.g[FP][0] as u32).wrapping_add(0x38), c.g[T0][0]);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[A0][0] = 0xfe00;
c.g[A0][0] = c.g[A0][0] << 46;
c.g[V0][0] = c.g[V0][0] >> 32;
c.g[V1][0] = sx((c.g[A1][0] as u32) << 4);
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[A2][0] = sx((c.g[A2][0] as u32) << 4);
c.sd((c.g[FP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
let t = c.g[A3][0] == 0u64;
c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x40));
if t {
break 'b_38364c;
} else {
c.cycles += 5;
c.g[V0][0] = c.g[T1][0];
c.g[T1][0] = c.g[V0][0] << 16;
c.g[A1][0] = c.g[V1][0] << 16;
c.g[T0][0] = 0x5;
'l_383620: loop {
c.cycles += 11;
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x200));
c.g[V1][0] = c.g[A2][0] | c.g[T1][0];
c.g[A0][0] = c.g[V0][0] | c.g[A1][0];
c.sd((c.g[FP][0] as u32), c.g[V1][0]);
c.sd((c.g[FP][0] as u32).wrapping_add(0x10), c.g[A0][0]);
c.g[A2][0] = c.g[V0][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x8), c.g[T0][0]);
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
c.sd((c.g[FP][0] as u32).wrapping_add(0x18), c.g[T0][0]);
let t = c.g[A3][0] != 0u64;
c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x20));
if t {
if event_due(c) { deliver_event_here(c, 0x00383620); } continue 'l_383620;
} else {
break 'b_38364c;
}
}
}
}
c.cycles += 73;
c.g[V0][0] = 0x50000;
c.g[V0][0] = c.g[V0][0] | 0xad;
c.g[V1][0] = 0x47;
c.sd((c.g[FP][0] as u32), c.g[V0][0]);
c.g[A0][0] = 0x10000000;
c.g[A0][0] = c.g[A0][0] << 32;
c.g[A0][0] = c.g[A0][0] | 0x8002;
c.sd((c.g[FP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.g[V0][0] = 0xe;
c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x10));
c.g[T0][0] = 0x500000;
c.sd((c.g[FP][0] as u32), c.g[A0][0]);
c.g[T1][0] = 0x40;
c.sd((c.g[FP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x340));
c.g[T2][0] = 0x41;
c.g[A1][0] = 0xfff0000;
c.g[V0][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x5900));
c.g[A1][0] = c.g[A1][0] | 0xffff;
c.g[A3][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x5904));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[A2][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x5902));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[T0][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x5906));
c.g[V1][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x5900));
c.g[V0][0] = c.g[V0][0] << 16;
c.g[A3][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x5902));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.g[A3][0] = c.g[A3][0] << 32;
c.g[A2][0] = c.g[A2][0] << 48;
c.g[V1][0] = c.g[V1][0] | c.g[A3][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[V0][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x5900));
c.g[V1][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x5904));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[A2][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x5902));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[T0][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x5906));
c.g[V1][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x5900));
c.g[V0][0] = c.g[V0][0] << 16;
c.g[A3][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x5902));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.g[A3][0] = c.g[A3][0] << 32;
c.g[A2][0] = c.g[A2][0] << 48;
c.g[V1][0] = c.g[V1][0] | c.g[A3][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x28), c.g[T2][0]);
c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
c.sd((c.g[FP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x30));
c.g[A1][0] = c.g[FP][0] & c.g[A1][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
c.g[A1][0] = sx((c.g[A1][0] as u32) >> 4);
c.g[RA][0] = sx(0x00383740);
c.g[A2][0] = 0x6;
Callee { addr: 0x00371940, f: ee::s12::Dma_SendGifNormal }.call(c, 0x00383740)?;
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
c.pc = t; return Ok(());
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
    use crate::difftest::fakes;
    use crate::difftest::{cycle_budget, fill, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the `Dma_SendGifNormal` fake (this test only).
    const OC1470_DMA_CTRL: u32 = DATA + 0x38_0000;
    const OC1470_DMA_MAGIC: u32 = 0x0c14_7001;
    static OC1470_PREV_DMA: AtomicUsize = AtomicUsize::new(0);
    /// Loop head of the vsync wait (waits for `CSR.VSINT`).
    const OC1470_VSYNC_WAIT: u32 = 0x0038_2c78;

    /// `Dma_SendGifNormal` as a logged fake (the kick would drive the DMA
    /// controller): the log records the packet address and quadword count.
    fn oc1470_fake_dma_send(c: &mut Cpu) {
        if c.bus.read32(OC1470_DMA_CTRL) != OC1470_DMA_MAGIC {
            return fakes::fall_back(c, &OC1470_PREV_DMA, ee::s12::f_00371940__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0037_1940, ra);
    }

    /// Event hook: no event, but the GS gets its vsync at the vsync wait's
    /// back edge so the wait ends; a runaway fails past the cycle budget.
    fn oc1470_vsync_event(c: &mut Cpu) -> bool {
        if c.cycles >= cycle_budget() {
            panic!("cycle budget exceeded ({} cycles, pc {:#x})", c.cycles, c.pc);
        }
        if c.pc & !1 == OC1470_VSYNC_WAIT {
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .hw
                .gs
                .vblank(0);
        }
        false
    }

    /// `Renderer_SetDisplayRegs`: one GS packet per path. The seed picks the
    /// frame-skip flag, the filter copy pass, progressive or interlaced
    /// display, the field/frame mode, the vsync wait, colour and brightness
    /// (clamped both ways) and sizes; `Dma_SendGifNormal` is a gated logged
    /// fake that unwinds at call 0 for a third of the seeds. The hardware
    /// registers are idle (waits do not loop) except the vsync wait.
    #[test]
    fn renderer_set_display_regs_matches() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00371940),
            oc1470_fake_dma_send,
            &OC1470_PREV_DMA,
        );
        fakes::covered(
            ee::s12::f_00382af0__generated,
            renderer_set_display_regs,
            384,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1470_0001, 1);
                c.hooks.event = oc1470_vsync_event;
                c.next_event = 0;
                c.bus.write32(OC1470_DMA_CTRL, OC1470_DMA_MAGIC);
                let obj = DATA;
                fill(c, &mut r, obj, 0x5b00 / 4);
                c.bus.write32(obj + 0xf44, seed % 2);
                c.bus.write32(obj + 0x59e8, seed / 2 % 2);
                c.bus.write32(obj + 0x5a4c, seed / 4 % 2);
                c.bus.write16(obj + 0x5a44, (seed / 8 % 2) as u16);
                c.bus.write16(obj + 0x5a46, (seed / 16 % 2) as u16);
                let w = [640u32, 512, 320, 448][(seed / 3 % 4) as usize];
                let h = [448u32, 224, 240][(seed / 5 % 3) as usize];
                c.bus.write32(obj + 0x5a54, w);
                c.bus.write32(obj + 0x5a58, h);
                // Frame buffer size (strip counts: width / 32 at most 20).
                c.bus.write32(obj + 0x5a30, [640u32, 512, 320, 448][(seed / 12 % 4) as usize]);
                c.bus.write32(obj + 0x5a34, [448u32, 224, 240][(seed / 4 % 3) as usize]);
                let (sx0, sy0) = if seed % 7 == 0 {
                    (r.edge_f32(), r.edge_f32())
                } else {
                    (r.f32(8.0), r.f32(8.0))
                };
                c.bus.write32(obj, sx0);
                c.bus.write32(obj + 4, sy0);
                // Brightness: mostly a plain scale (clamps high and low).
                let bright = if seed % 11 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(3.0)
                };
                c.bus.write32(obj + 0x59fc, bright);
                let colours = [0x00, 0x7f, 0xff, 0x8000_0000 | seed];
                c.bus.write32(obj + 0x59ec, colours[(seed % 4) as usize]);
                c.bus.write32(obj + 0x59f0, r.next_u32() % 256);
                c.bus.write32(obj + 0x59f4, r.next_u32() % 256);
                c.bus.write32(
                    obj + 0x59f8,
                    [0x80, 0x40, 0x80, 0xff][(seed / 7 % 4) as usize],
                );
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
                c.g[A2][0] = (seed / 32 % 2) as u64 * (1 + r.next_u32() as u64 % 3);
                c.pc = 0x0038_2af0;
            },
        );
    }
}
