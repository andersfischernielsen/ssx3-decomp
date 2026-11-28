//! Staging module oc1466 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1466`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_0ce0, ps2gm_draw_m_emitter, "PS2GraphicsMan_DrawMEmitter"),
];

/// `cPS2GraphicsMan::RenderDynamicParticleEmitter1(self, emitter)` (0x00380ce0,
/// `PS2GraphicsMan_v84`): draws a dynamic-colour particle emitter ring buffer.
/// `a0` = graphics manager (`self`), `a1` = emitter. Unless the dynamic
/// emitters are disabled (`gp+0x140c`), it sets the top render state's draw
/// mode, refreshes the view matrices if the texture is not ready, then emits
/// one GIF/VIF block per 43 particles: the header quads (`emitter+0x20`, 21
/// quads), the per-segment header (step, float step, time offset, segment
/// size), the ring's two position arrays and the colours (wrapping around the
/// ring buffer, the last entry repeated, padded to 43), and finally submits
/// the block to the defer ring (`self+0x18f0`, up to 2600 entries).
/// The generated structure is kept: every register, cycle and call preserved.
pub fn ps2gm_draw_m_emitter(c: &mut Cpu) {
    guest(c, |c| {
'b_3812dc: {
c.cycles += 15;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x180));
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x140c));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
c.g[S1][0] = c.g[A1][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
c.g[S3][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x110));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x100));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xf0));
let t = c.g[V0][0] != 0u64;
c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
if t {
break 'b_3812dc;
} else {
'b_380d60: {
c.cycles += 14;
c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = 0xfffffffffffffc3f;
c.g[A2][0] = 0xffffffffc0000000;
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | 0x100;
c.sw((c.g[A1][0] as u32), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6b90));
if c.g[V1][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x178));
break 'b_380d60;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x00380d5c);
c.g[A0][0] = c.g[S3][0];
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x00380d5c)?;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x178));
break 'b_380d60;
}
}
'b_380d78: {
c.cycles += 6;
c.g[A0][0] = 0x2b;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
c.g[S7][0] = 0u64;
c.div(0, 2, 4);
if c.g[V0][0] == 0u64 {
break 'b_380d78;
} else {
break 'b_380d78;
}
}
'b_380db8: {
'b_380db0: {
c.cycles += 12;
c.g[A1][0] = c.lo[0];
c.div(0, 3, 2);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), c.g[A1][0]);
c.g[V1][0] = c.lo[0];
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V1][0]);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x200));
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x0;
if t {
break 'b_380db0;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf10));
if t {
break 'b_380db8;
} else {
break 'b_380db0;
}
}
}
c.cycles += 2;
c.g[V0][0] = 0x0;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xa00));
break 'b_380db8;
}
'b_3812b4: {
c.cycles += 5;
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc4));
let t = (c.g[V0][0] as i64) <= 0;
c.g[S0][0] = 0u64;
if t {
break 'b_3812b4;
} else {
c.cycles += 3;
c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
'l_380dd8: loop {
'b_380ea4: {
c.cycles += 43;
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[A2][0] = 0xffffffffffffffff;
c.g[A1][0] = c.g[V1][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0xcc), c.g[V1][0]);
c.g[A3][0] = 0u64;
c.g[RA][0] = sx(0x00380dfc);
c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[V0][0]);
Callee { addr: 0x0038f460, f: ee::s12::SprStream_BeginBlock }.call(c, 0x00380dfc)?;
c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0x7c;
c.sd((c.g[V0][0] as u32), c.g[V1][0]);
c.g[A1][0] = 0xd8dd;
c.g[A1][0] = c.g[A1][0] << 47;
c.g[V1][0] = 0x2b;
c.g[A3][0] = 0x2b;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.mult(0, 4, 16, 3);
c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x20));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.mtc1(4, 2);
c.cvt_s_w(2, 2);
c.sd((c.g[V0][0] as u32).wrapping_add(0x10), 0u64);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A1][0]);
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x2c));
c.mul_s(0, 0, 2);
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x178));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[S7][0] as u32)));
c.mul_s(0, 0, 1);
c.g[V1][0] = ((c.g[V0][0] as i64) < 44i64) as u64;
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x170));
if c.g[V1][0] == 0 { c.g[V0][0] = c.g[A3][0]; }
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.g[A0][0];
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
let t = c.g[V1][0] == c.g[A1][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[A0][0]);
if t {
break 'b_380ea4;
} else {
c.cycles += 1;
'l_380e88: loop {
c.cycles += 7;
c.lq(2, (c.g[A2][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A1][0];
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00380e88); } continue 'l_380e88;
} else {
break 'b_380ea4;
}
}
}
}
'b_380ed4: {
c.cycles += 5;
c.sw((c.g[S2][0] as u32), c.g[V1][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] == c.g[A1][0];
c.g[A0][0] = c.g[SP][0];
if t {
break 'b_380ed4;
} else {
'l_380eb8: loop {
c.cycles += 7;
c.lq(2, (c.g[A0][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A1][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00380eb8); } continue 'l_380eb8;
} else {
break 'b_380ed4;
}
}
}
}
'b_380ef4: {
c.cycles += 8;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x17c));
c.g[A1][0] = 0xffffffffffffffff;
c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x178));
c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
c.sw((c.g[S2][0] as u32), c.g[V1][0]);
c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[T1][0] as u32)));
if c.g[A3][0] == 0u64 {
break 'b_380ef4;
} else {
break 'b_380ef4;
}
}
'b_380f68: {
'b_380f50: {
c.cycles += 17;
c.div(0, 2, 7);
c.g[V1][0] = c.lw((c.g[SP][0] as u32));
c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A2][0] = ((c.g[S7][0] as i64) < (c.g[A3][0] as i64)) as u64;
c.g[T2][0] = c.hi[0];
c.g[V1][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A3][0] as u32)));
c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[T0][0] as i64)) as u64;
c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[S0][0] as i64)) as u64;
if c.g[V1][0] != 0 { c.g[T0][0] = c.g[A0][0]; }
if c.g[A1][0] == 0 { c.g[S0][0] = 0u64; }
c.g[V1][0] = 0x2b;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[S0][0] as u32)));
let t = c.g[A2][0] == 0u64;
c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[T0][0] as u32)));
if t {
break 'b_380f50;
} else {
'b_380f44: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[T1][0] as u32)));
if c.g[A3][0] == 0u64 {
break 'b_380f44;
} else {
break 'b_380f44;
}
}
c.cycles += 3;
c.div(0, 2, 7);
let t = true;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1a4));
if t {
break 'b_380f68;
} else {
break 'b_380f50;
}
}
}
'b_380f5c: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x1));
if c.g[A3][0] == 0u64 {
break 'b_380f5c;
} else {
break 'b_380f5c;
}
}
c.cycles += 3;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1a4));
c.div(0, 2, 7);
break 'b_380f68;
}
'b_380fdc: {
c.cycles += 21;
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1a0));
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x204));
c.g[T1][0] = c.g[A2][0];
c.g[V1][0] = c.hi[0];
c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[S5][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[T5][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V1][0] = sx((c.g[T2][0] as u32) << 4);
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[A2][0] = c.g[V1][0];
c.g[V0][0] = sx((c.g[T0][0] as u32) << 4);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[T4][0] = c.g[V0][0];
c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[T1][0] = sx((c.g[T3][0] as u32) << 4);
c.g[T2][0] = sx((c.g[T2][0] as u32) << 2);
c.g[S6][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
let t = c.g[A1][0] == c.g[A3][0];
c.g[S4][0] = sx((c.g[T0][0] as u32) << 2);
if t {
break 'b_380fdc;
} else {
c.cycles += 1;
'l_380fc0: loop {
c.cycles += 7;
c.lq(2, (c.g[V1][0] as u32));
c.sq(2, (c.g[A1][0] as u32));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
let t = c.g[A1][0] != c.g[A3][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00380fc0); } continue 'l_380fc0;
} else {
break 'b_380fdc;
}
}
}
}
'b_381018: {
c.cycles += 2;
let t = (c.g[S0][0] as i64) <= 0;
c.sw((c.g[S2][0] as u32), c.g[A1][0]);
if t {
break 'b_381018;
} else {
'b_381014: {
c.cycles += 5;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[V0][0] = sx((c.g[S0][0] as u32) << 4);
c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = c.g[V1][0] == c.g[A3][0];
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1a0));
if t {
break 'b_381014;
} else {
'l_380ff8: loop {
c.cycles += 7;
c.lq(2, (c.g[A1][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A3][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00380ff8); } continue 'l_380ff8;
} else {
break 'b_381014;
}
}
}
}
c.cycles += 1;
c.sw((c.g[S2][0] as u32), c.g[V1][0]);
break 'b_381018;
}
}
'b_381048: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
if c.g[V1][0] == c.g[A1][0] {
c.sw((c.g[S2][0] as u32), c.g[V1][0]);
break 'b_381048;
} else {
'l_381028: loop {
c.cycles += 7;
c.lq(2, (c.g[A0][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A1][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00381028); } continue 'l_381028;
} else {
c.cycles += 1;
c.sw((c.g[S2][0] as u32), c.g[V1][0]);
break 'b_381048;
}
}
}
}
'b_381084: {
c.cycles += 8;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1a4));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[A1][0] = c.g[V0][0];
c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[T4][0] as u32)));
let t = c.g[A1][0] == c.g[A2][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
if t {
break 'b_381084;
} else {
'l_381068: loop {
c.cycles += 7;
c.lq(2, (c.g[V1][0] as u32));
c.sq(2, (c.g[A1][0] as u32));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
let t = c.g[A1][0] != c.g[A2][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00381068); } continue 'l_381068;
} else {
break 'b_381084;
}
}
}
}
'b_3810c0: {
c.cycles += 2;
let t = (c.g[S0][0] as i64) <= 0;
c.sw((c.g[S2][0] as u32), c.g[A1][0]);
if t {
break 'b_3810c0;
} else {
'b_3810bc: {
c.cycles += 5;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[V0][0] = sx((c.g[S0][0] as u32) << 4);
c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = c.g[V1][0] == c.g[A2][0];
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1a4));
if t {
break 'b_3810bc;
} else {
'l_3810a0: loop {
c.cycles += 7;
c.lq(2, (c.g[A1][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A2][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x003810a0); } continue 'l_3810a0;
} else {
break 'b_3810bc;
}
}
}
}
c.cycles += 1;
c.sw((c.g[S2][0] as u32), c.g[V1][0]);
break 'b_3810c0;
}
}
'b_3810ec: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] == c.g[A1][0];
c.g[A0][0] = c.g[T5][0];
if t {
break 'b_3810ec;
} else {
'l_3810d0: loop {
c.cycles += 7;
c.lq(2, (c.g[A0][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A1][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x003810d0); } continue 'l_3810d0;
} else {
break 'b_3810ec;
}
}
}
}
c.cycles += 15;
c.sw((c.g[S2][0] as u32), c.g[V1][0]);
c.g[A0][0] = 0x6e2c0000;
c.g[A0][0] = c.g[A0][0] | 0xc06e;
c.g[A0][0] = c.g[A0][0] << 32;
c.g[A1][0] = 0x2b;
c.g[A2][0] = 0xffffffffffffffff;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
c.sd((c.g[V0][0] as u32), 0u64);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
'l_381128: loop {
c.cycles += 7;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
let t = c.g[A1][0] != c.g[A2][0];
c.g[A0][0] = c.g[FP][0];
if t {
if event_due(c) { deliver_event_here(c, 0x00381128); } continue 'l_381128;
} else {
'b_381168: {
c.cycles += 6;
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x204));
c.g[A2][0] = c.g[S4][0];
c.g[RA][0] = sx(0x00381154);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[T2][0] as u32)));
call_ptr(c, 0x003e6574, 0x00381154)?;
let t = (c.g[S0][0] as i64) <= 0;
c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[S4][0] as u32)));
if t {
break 'b_381168;
} else {
c.cycles += 3;
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x204));
c.g[RA][0] = sx(0x00381168);
c.g[A2][0] = sx((c.g[S0][0] as u32) << 2);
call_ptr(c, 0x003e6574, 0x00381168)?;
break 'b_381168;
}
}
'b_3811ac: {
c.cycles += 10;
c.g[V0][0] = sx((c.g[S6][0] as u32) << 2);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[SP][0] as u32)));
c.lwl(3, (c.g[S5][0] as u32).wrapping_add(0x3));
c.lwr(3, (c.g[S5][0] as u32));
c.swl(3, (c.g[V0][0] as u32).wrapping_add(0x13));
c.swr(3, (c.g[V0][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xb0));
let t = c.g[V1][0] == c.g[A0][0];
c.g[A1][0] = c.g[FP][0];
if t {
break 'b_3811ac;
} else {
'l_381190: loop {
c.cycles += 7;
c.lq(2, (c.g[A1][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A0][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00381190); } continue 'l_381190;
} else {
break 'b_3811ac;
}
}
}
}
'b_381298: {
c.cycles += 25;
c.sw((c.g[S2][0] as u32), c.g[V1][0]);
c.g[A1][0] = 0x8800;
c.g[A1][0] = c.g[A1][0] << 45;
c.g[V1][0] = 0x14000000;
c.g[A2][0] = 0x4;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc8));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[V1][0] = c.g[V0][0] | c.g[V1][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.sd((c.g[A0][0] as u32), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[A1][0] = c.g[V1][0];
c.g[RA][0] = sx(0x003811f4);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
Callee { addr: 0x0038f668, f: ee::s12::SprStream_EndBlockAligned }.call(c, 0x003811f4)?;
c.g[T1][0] = c.g[V0][0];
c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18f0));
c.sw((c.g[S3][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
if t {
break 'b_381298;
} else {
c.cycles += 34;
c.g[A1][0] = 0x60000;
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
c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[T0][0] as u32));
c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
c.sdr(3, (c.g[V0][0] as u32));
c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xcc));
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_381298;
}
}
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a00));
c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc4));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
let t = c.g[V1][0] != 0u64;
c.sw((c.g[S3][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
if t {
if event_due(c) { deliver_event_here(c, 0x00380dd8); } continue 'l_380dd8;
} else {
break 'b_3812b4;
}
}
}
}
}
}
c.cycles += 10;
c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = 0xfffffffffffffc3f;
c.g[A1][0] = 0xffffffffc0000000;
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
break 'b_3812dc;
}
}
c.cycles += 12;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x100));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x180));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, check_reaching, machine};
    use crate::difftest::{elf_image, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word and counters of this module's fakes, in the machine's RAM.
    const OC1466_CTRL: u32 = DATA + 0x3_f000;
    const OC1466_MAGIC: u32 = 0x1466_0ce0;
    static OC1466_PREV_VIEW: AtomicUsize = AtomicUsize::new(0);
    static OC1466_PREV_BEGIN: AtomicUsize = AtomicUsize::new(0);
    static OC1466_PREV_END: AtomicUsize = AtomicUsize::new(0);

    const SELF_: u32 = DATA + 0x1_0000;
    const RS: u32 = DATA + 0x1_000;
    const EMITTER: u32 = DATA + 0x8000;
    const POS_A: u32 = DATA + 0x3_0000;
    const POS_B: u32 = DATA + 0x3_2000;
    const COLS: u32 = DATA + 0x3_4000;
    const RING: u32 = DATA + 0x8_0000;
    const OUT: u32 = DATA + 0x4_0000;

    /// `UpdateViewMatrices` stand-in: log the call, return (or unwind).
    fn oc1466_fake_view(c: &mut Cpu) {
        if c.bus.read32(OC1466_CTRL) != OC1466_MAGIC {
            return fakes::fall_back(c, &OC1466_PREV_VIEW, ee::s12::f_0037d968__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0037_d968, ra);
    }

    /// `SprStream_BeginBlock` stand-in: returns a fresh output buffer.
    fn oc1466_fake_begin(c: &mut Cpu) {
        if c.bus.read32(OC1466_CTRL) != OC1466_MAGIC {
            return fakes::fall_back(c, &OC1466_PREV_BEGIN, ee::s12::f_0038f460__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0038_f460, ra);
        if c.pc == ra {
            let n = c.bus.read32(OC1466_CTRL + 4);
            c.bus.write32(OC1466_CTRL + 4, n + 1);
            c.g[V0][0] = (OUT + 0x1000 * (n % 8)) as u64;
        }
    }

    /// `SprStream_EndBlockAligned` stand-in: logs, returns a pseudo address.
    fn oc1466_fake_end(c: &mut Cpu) {
        if c.bus.read32(OC1466_CTRL) != OC1466_MAGIC {
            return fakes::fall_back(c, &OC1466_PREV_END, ee::s12::f_0038f668__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0038_f668, ra);
    }

    fn oc1466_install() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0037D968),
            oc1466_fake_view,
            &OC1466_PREV_VIEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038F460),
            oc1466_fake_begin,
            &OC1466_PREV_BEGIN,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038F668),
            oc1466_fake_end,
            &OC1466_PREV_END,
        );
    }

    /// An emitter ring of 1..=100 particles; some seeds disable the emitters,
    /// have the texture not ready (view matrices call), take `alt`, make the
    /// defer ring full, wrap the ring head, and unwind in a fake.
    fn oc1466_setup(c: &mut Cpu, seed: u32) {
        let mut r: Rng = machine(c, seed, 0x1466, 9);
        c.bus.write32(OC1466_CTRL, OC1466_MAGIC);
        c.bus.write32(OC1466_CTRL + 4, 0);
        let counts = [1u32, 5, 43, 44, 50, 86, 87, 100];
        let count = counts[(seed % 8) as usize];
        let head = if seed % 5 == 0 { count - 1 } else { r.next_u32() % count };
        c.bus.write32(GAME_GP + 0x140c, (seed % 11 == 0) as u32);
        c.bus.write32(GAME_GP + 0x1384, r.next_u32());
        c.bus.write32(SELF_ + 0xe84, RS);
        c.bus.write32(SELF_ + 0x6b90, (seed % 3 != 0) as u32);
        c.bus.write32(SELF_ + 0x5a00, DATA + 0x5_0000 + (r.next_u32() & 0xff0));
        c.bus.write32(SELF_ + 0x18f0, RING);
        for i in 0..8 {
            c.bus.write32(RS + 4 * i, r.next_u32());
        }
        let n = if seed % 7 == 0 { 2600 + seed % 3 } else { seed % 40 };
        c.bus.write32(RING, n);
        c.bus.write32(RING + 0x6_9cc0, r.next_u32());
        c.bus.write32(RING + 0x6_9cc4, r.next_u32());
        for i in 0..(0x214 / 4) {
            c.bus.write32(EMITTER + 4 * i, r.next_u32());
        }
        c.bus.write32(EMITTER + 0x20, 1 + r.next_u32() % 1000);
        let step = r.edge_f32();
        c.bus.write32(EMITTER + 0x2c, step);
        c.bus.write32(EMITTER + 0x178, count);
        c.bus.write32(EMITTER + 0x17c, head);
        c.bus.write32(EMITTER + 0x1a0, POS_A);
        c.bus.write32(EMITTER + 0x1a4, POS_B);
        c.bus.write32(EMITTER + 0x200, (seed % 2) as u32);
        c.bus.write32(EMITTER + 0x204, COLS);
        for i in 0..(count * 4) {
            c.bus.write32(POS_A + 4 * i, r.next_u32());
            c.bus.write32(POS_B + 4 * i, r.next_u32());
            c.bus.write32(COLS + 4 * i, r.next_u32());
        }
        c.g[A0][0] = SELF_ as u64;
        c.g[A1][0] = EMITTER as u64;
    }

    /// The ring emitter draw matches the generated body over wrapped and
    /// unwrapped rings, one and several 43-particle blocks, a full defer ring,
    /// disabled emitters and unwinding callees.
    #[test]
    fn ps2gm_draw_m_emitter_matches() {
        oc1466_install();
        check_reaching(
            ee::s12::f_00380ce0__generated,
            ps2gm_draw_m_emitter,
            400,
            &[0x0038_0d5c, 0x0038_0dfc, 0x0038_11f4],
            &oc1466_setup,
        );
    }
}
