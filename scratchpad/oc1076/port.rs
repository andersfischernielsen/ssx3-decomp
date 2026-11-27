//! Staging module oc1076 (wave wave266): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1076`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `cDynamicSprayFX::~cDynamicSprayFX(int flags)` (0x2df1c8): destructor of the
/// board spray effect. `a0` this, `a1` flags (bit 0: delete this, bit 1: also run
/// the owning rider's destructor). Points the rider's three table pointers
/// (`+0x6e8`, `+0x6d0`, `+0x6c0`) at the effect's static tables first; with flags
/// 0 it also builds stack copies of the two tables patched with this object's
/// offset (the compiler's inlined base-class unwinding) and points the rider at
/// those. Then it destroys the 0x210-byte emitters of the array at `this+0x28`
/// (count at `-0x10`, back to front, through their vtable slot `+0x20/+0x24`),
/// frees that array, the buffer at `this+0x2c` and the one at `this+0x7c`
/// (`EA_MemFree`), calls `Rider_dtor(*this, 0)` for flag 2 and
/// `__builtin_delete(this)` for flag 1. Kept as the generated block structure
/// (labels are the generated block addresses) so every register and cycle
/// matches.
#[allow(unreachable_code, unused_assignments)]
pub fn cdynamic_spray_fx_dtor(c: &mut Cpu) {
    guest(c, |c| {
'b_2df2ec: {
c.cycles += 23;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
c.g[V0][0] = 0x490000;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7ca0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
c.g[S1][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
c.g[T0][0] = c.g[A1][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
c.g[A0][0] = 0x460000;
c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6470));
c.g[A3][0] = c.g[V0][0];
c.g[V1][0] = c.lw((c.g[S1][0] as u32));
c.sw((c.g[V1][0] as u32).wrapping_add(0x6e8), c.g[A2][0]);
c.g[V1][0] = 0x490000;
c.g[A1][0] = c.lw((c.g[S1][0] as u32));
c.g[T3][0] = c.g[V1][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7c58));
c.sw((c.g[A1][0] as u32).wrapping_add(0x6d0), c.g[A0][0]);
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
let t = c.g[T0][0] != 0u64;
c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), c.g[V1][0]);
if t {
break 'b_2df2ec;
} else {
c.cycles += 4;
c.g[A1][0] = c.g[A2][0];
c.g[V1][0] = c.g[SP][0];
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
'l_2df234: loop {
c.cycles += 11;
c.g[A0][0] = c.ld((c.g[A1][0] as u32));
c.g[A2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
c.g[T2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
c.g[T4][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
c.sd((c.g[V1][0] as u32), c.g[A0][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T2][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
let t = c.g[A1][0] != c.g[V0][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
if t {
if event_due(c) { deliver_event_here(c, 0x002df234); } continue 'l_2df234;
} else {
c.cycles += 14;
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7ca0));
c.g[A2][0] = c.ld((c.g[A1][0] as u32));
c.sd((c.g[V1][0] as u32), c.g[A2][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x6e8), c.g[SP][0]);
c.g[A2][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7c58));
c.g[V1][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_add(0xa0));
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
c.g[A1][0] = c.g[T1][0];
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x130));
c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
'l_2df298: loop {
c.cycles += 11;
c.g[V0][0] = c.ld((c.g[A2][0] as u32));
c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
c.g[T4][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
c.sd((c.g[A1][0] as u32), c.g[V0][0]);
c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T4][0]);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
let t = c.g[A2][0] != c.g[T2][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
if t {
if event_due(c) { deliver_event_here(c, 0x002df298); } continue 'l_2df298;
} else {
c.cycles += 10;
c.g[V0][0] = c.ld((c.g[A2][0] as u32));
c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
c.sd((c.g[A1][0] as u32), c.g[V0][0]);
c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.g[V0][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x7c58));
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x8));
c.sw((c.g[A0][0] as u32).wrapping_add(0x6c0), c.g[T1][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.sh((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
break 'b_2df2ec;
}
}
}
}
}
}
'b_2df348: {
c.cycles += 4;
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
c.g[S3][0] = c.g[T0][0] & 0x1;
let t = c.g[A1][0] == 0u64;
c.g[S2][0] = c.g[T0][0] & 0x2;
if t {
break 'b_2df348;
} else {
'b_2df340: {
c.cycles += 6;
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x10));
c.g[V0][0] = 0x210;
c.mult(0, 4, 3, 2);
c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
if c.g[A1][0] == c.g[S0][0] {
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
break 'b_2df340;
} else {
c.cycles += 1;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x210));
'l_2df318: loop {
c.cycles += 9;
c.g[A1][0] = 0u64;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x002df330);
c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x002df330)?;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
if c.g[V0][0] != c.g[S0][0] {
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x210));
if event_due(c) { deliver_event_here(c, 0x002df318); } continue 'l_2df318;
} else {
c.cycles += 1;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
break 'b_2df340;
}
}
}
}
c.cycles += 2;
c.g[RA][0] = sx(0x002df348);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
call_ptr(c, 0x00317e98, 0x002df348)?;
break 'b_2df348;
}
}
'b_2df360: {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
break 'b_2df360;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x002df35c);
call_ptr(c, 0x00317e98, 0x002df35c)?;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
break 'b_2df360;
}
}
'b_2df370: {
c.cycles += 2;
let t = c.g[A0][0] == 0u64;

if t {
break 'b_2df370;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x002df370);
call_ptr(c, 0x00317e98, 0x002df370)?;
break 'b_2df370;
}
}
'b_2df384: {
c.cycles += 2;
let t = c.g[S2][0] == 0u64;

if t {
break 'b_2df384;
} else {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
c.g[RA][0] = sx(0x002df384);
c.g[A1][0] = 0u64;
call_ptr(c, 0x0011b978, 0x002df384)?;
break 'b_2df384;
}
}
'b_2df398: {
c.cycles += 2;
let t = c.g[S3][0] == 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
if t {
break 'b_2df398;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x002df394);
c.g[A0][0] = c.g[S1][0];
call_ptr(c, 0x00317e50, 0x002df394)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
break 'b_2df398;
}
}
c.cycles += 6;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
c.pc = t; return Ok(());
    })
}

/// `void PS2BezierMan_EmitPatchProjectorPasses(self, int kind, void *tbl,
/// sPatch *patch, unsigned lights, unsigned *handle)` (0x38d448): for every light
/// `k < gm->numLights` whose bit is set in `lights` (`t0`), emits one projector
/// pass for the patch: sets the render state's texture (`gm+0xe84` top state,
/// halfword `+0x12` from the light's texture at `gm+0x13f0+0xa0k+0x94`) and
/// its type bits to 3, then writes a 0x70-byte DMA record at `*handle` (uncached
/// alias): the light's 4x4 projection matrix (VU0 `lqc2/sqc2` from
/// `gm+0x1440+0xa0k`) and the VU1 start address taken from the program table
/// `tbl + 8*patch[1] + 0x30`; the record is then also queued in the draw ring
/// (`gm+0x18f0`, up to 2600 entries) by copying the 0x14 bytes of the state's
/// top state. `a2` tbl, `a3` patch, `t0` lights, `t1` handle. A leaf: no calls.
/// Kept as the generated block structure so every register and cycle matches.
#[allow(unreachable_code, unused_assignments)]
pub fn ps2_bezier_man_emit_patch_projector_passes(c: &mut Cpu) {
'b_38d60c: {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
c.g[T3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[T5][0] = 0u64;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S2][0] = c.g[T0][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(30, (c.g[SP][0] as u32));
c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x13ec));
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
if t {
break 'b_38d60c;
} else {
c.cycles += 18;
c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
c.g[T0][0] = 0xfff0000;
c.g[T9][0] = 0x60000;
c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[FP][0] = 0xffffffffffffffcf;
c.g[S7][0] = 0x10000000;
c.g[S7][0] = c.g[S7][0] | 0x6;
c.g[S6][0] = 0x20000000;
c.g[S5][0] = 0x6c040000;
c.g[S5][0] = c.g[S5][0] | 0x8080;
c.g[S5][0] = c.g[S5][0] << 32;
c.g[S4][0] = 0x14000000;
c.g[T0][0] = c.g[T0][0] | 0xffff;
c.g[T9][0] = c.g[T9][0] | 0x8000;
c.g[S3][0] = 0xffffffffffffffe0;
c.g[T8][0] = sx((c.g[T3][0] as u32).wrapping_add(0x1440));
c.g[T7][0] = sx((c.g[T3][0] as u32).wrapping_add(0x13f0));
'l_38d4d0: loop {
'b_38d5f4: {
c.cycles += 4;
c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> ((c.g[T5][0] as u32) & 31)) as u32);
c.g[V0][0] = c.g[V0][0] & 0x1;
let t = c.g[V0][0] == 0u64;
c.g[A2][0] = 0x30000000;
if t {
break 'b_38d5f4;
} else {
'b_38d5e8: {
c.cycles += 37;
c.g[A0][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x94));
c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0xe84));
c.g[T6][0] = c.lw((c.g[T1][0] as u32));
c.sh((c.g[V0][0] as u32).wrapping_add(0x12), c.g[A0][0]);
c.g[V1][0] = c.g[T6][0] | c.g[A2][0];
c.g[A0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0xe84));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[FP][0];
c.g[V0][0] = c.g[V0][0] | 0x30;
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.sd((c.g[V1][0] as u32), c.g[S7][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), c.g[S6][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x18), c.g[S5][0]);
c.lqc2(1, (c.g[T8][0] as u32));
c.lqc2(2, (c.g[T8][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[T8][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[T8][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[A1][0] as u32));
c.sqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
c.sd((c.g[V1][0] as u32).wrapping_add(0x60), 0u64);
c.g[S0][0] = 0x1;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.g[V0][0] = c.g[V0][0] | c.g[S4][0];
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[V1][0] as u32).wrapping_add(0x68), c.g[V0][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x70));
c.g[T4][0] = c.g[V1][0] & c.g[T0][0];
c.sw((c.g[T1][0] as u32), c.g[T4][0]);
c.g[A3][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x18f0));
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T2][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0xe84));
if t {
break 'b_38d5e8;
} else {
c.cycles += 29;
c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[T9][0] as u32)));
c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x8));
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
c.g[A0][0] = c.g[A0][0] | c.g[A2][0];
c.g[V1][0] = c.g[V1][0] & 0x1f;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sw((c.g[T2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[A3][0] as u32));
c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
c.ldl(3, (c.g[T2][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[T2][0] as u32));
c.ldl(4, (c.g[T2][0] as u32).wrapping_add(0xf));
c.ldr(4, (c.g[T2][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10));
c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
c.sdr(3, (c.g[V0][0] as u32));
c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), c.g[S0][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T6][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T4][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
break 'b_38d5e8;
}
}
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[T1][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.sw((c.g[T1][0] as u32), c.g[V0][0]);
break 'b_38d5f4;
}
}
c.cycles += 6;
c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x13ec));
c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x1));
c.g[T8][0] = sx((c.g[T8][0] as u32).wrapping_add(0xa0));
c.g[V0][0] = (c.g[T5][0] < c.g[V0][0]) as u64;
let t = c.g[V0][0] != 0u64;
c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0xa0));
if t {
if event_due(c) { deliver_event_here(c, 0x0038d4d0); } continue 'l_38d4d0;
} else {
break 'b_38d60c;
}
}
}
}
c.cycles += 11;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lq(30, (c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
c.pc = t; return;
}
