//! Staging module oc1480 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1480`.

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
    (0x0037_9860, ps2_graphics_man_draw_tri_strip, "PS2GraphicsMan_DrawTriStrip"),
    (0x0037_b548, ps2_graphics_man_draw_skinned_model_shadow, "PS2GraphicsMan_DrawSkinnedModelShadow"),
    (0x0038_d690, ps2_bezier_man_draw_patch_fx, "PS2BezierMan_DrawPatchFX"),
    (0x0038_0518, ps2_graphics_man_draw_emitter, "PS2GraphicsMan_DrawEmitter"),
    (0x0014_4368, focus_man_collect_octree_node, "FocusMan_CollectOctreeNode"),
];

/// `void PS2GraphicsMan_DrawTriStrip(PS2GraphicsMan* self, int count, Vertex* verts)`
/// (0x00379860, `PS2GraphicsMan_v71`): packs a triangle strip of `count`
/// vertices into `SprStream` blocks of up to 72 vertices (renewing the block
/// when it is full), closes the dma chain, and records the draw in the deferred
/// draw ring. Ported from the generated body (labelled blocks kept).
pub fn ps2_graphics_man_draw_tri_strip(c: &mut Cpu) {
    guest(c, |c| {
'b_3798b4: {
'b_3798ac: {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1290));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S1][0] = c.g[A2][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S3][0] = c.g[A1][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S4][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
let t = c.g[V0][0] == 0u64;
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
if t {
break 'b_3798ac;
} else {
c.cycles += 3;
c.g[V0][0] = 0x0;
let t = true;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
if t {
break 'b_3798b4;
} else {
break 'b_3798ac;
}
}
}
c.cycles += 2;
c.g[V0][0] = 0x0;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b8));
break 'b_3798b4;
}
'b_379900: {
c.cycles += 16;
c.g[S2][0] = sx((c.g[V0][0] as u32) >> 3);
c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = 0xfffffffffffffc3f;
c.sw((c.g[GP][0] as u32).wrapping_add(0x1290), 0u64);
c.g[A2][0] = 0xffffffffc0000000;
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | 0xc0;
c.sw((c.g[A1][0] as u32), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6b90));
if c.g[V1][0] != 0u64 {
c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xe84));
break 'b_379900;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x003798fc);
c.g[A0][0] = c.g[S4][0];
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x003798fc)?;
c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xe84));
break 'b_379900;
}
}
'b_37991c: {
'b_379914: {
c.cycles += 3;
c.g[V1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
let t = (c.g[V1][0] as i64) < 0;
c.g[S0][0] = 0x5c;
if t {
break 'b_379914;
} else {
c.cycles += 2;
let t = true;
c.g[S6][0] = 0x412;
if t {
break 'b_37991c;
} else {
break 'b_379914;
}
}
}
c.cycles += 2;
c.g[S0][0] = 0x4c;
c.g[S6][0] = 0x41f;
break 'b_37991c;
}
'b_379ab8: {
c.cycles += 14;
c.g[S5][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[A2][0] = 0xffffffffffffffff;
c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5a00));
c.g[A3][0] = 0u64;
c.g[A0][0] = c.g[S5][0];
c.g[A1][0] = c.g[V0][0];
c.g[RA][0] = sx(0x0037993c);
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
Callee { addr: 0x0038f460, f: ee::s12::SprStream_BeginBlock }.call(c, 0x0037993c)?;
c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
c.g[T0][0] = c.g[V0][0];
c.g[V0][0] = ((c.g[S3][0] as i64) < 3i64) as u64;
c.g[A1][0] = sx((c.g[V1][0] as u32) << 4);
let t = c.g[V0][0] != 0u64;
c.g[T1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
if t {
break 'b_379ab8;
} else {
c.cycles += 9;
c.g[A0][0] = c.g[S0][0] << 47;
c.g[V1][0] = 0x8000;
c.g[V1][0] = c.g[V1][0] << 31;
c.g[V0][0] = 0x14000000;
c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
c.g[V0][0] = c.g[S2][0] | c.g[V0][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
c.g[FP][0] = c.g[V0][0] << 32;
c.g[S7][0] = c.g[A1][0];
'l_379978: loop {
'b_3799d4: {
c.cycles += 11;
c.g[V0][0] = ((c.g[S3][0] as i64) < 73i64) as u64;
c.g[S2][0] = 0x48;
if c.g[V0][0] != 0 { c.g[S2][0] = c.g[S3][0]; }
c.g[V1][0] = sx((c.g[S2][0] as u32) << 1);
c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V0][0] = (c.g[V0][0] < c.g[T1][0]) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
if t {
break 'b_3799d4;
} else {
c.cycles += 12;
c.g[A1][0] = c.g[T0][0];
c.g[A0][0] = c.g[S5][0];
c.g[RA][0] = sx(0x003799b4);
c.g[A2][0] = 0x4;
Callee { addr: 0x0038f668, f: ee::s12::SprStream_EndBlockAligned }.call(c, 0x003799b4)?;
c.g[A0][0] = c.g[S5][0];
c.g[A1][0] = c.g[V0][0];
c.sw((c.g[S4][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.g[A2][0] = 0xffffffffffffffff;
c.g[RA][0] = sx(0x003799cc);
c.g[A3][0] = 0u64;
Callee { addr: 0x0038f460, f: ee::s12::SprStream_BeginBlock }.call(c, 0x003799cc)?;
c.g[T0][0] = c.g[V0][0];
c.g[T1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[S7][0] as u32)));
break 'b_3799d4;
}
}
'b_379a90: {
c.cycles += 37;
c.g[A3][0] = 0x3;
c.g[A1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x8));
c.mult(0, 0, 18, 7);
c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
c.g[A0][0] = c.g[S2][0] | 0x8000;
c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
c.g[A2][0] = c.g[A0][0] | c.g[A1][0];
c.sd((c.g[T0][0] as u32).wrapping_add(0x8), 0u64);
c.g[A0][0] = 0x10000000;
c.g[A1][0] = 0xc000;
c.g[A1][0] = c.g[A1][0] << 46;
c.g[V0][0] = c.lo[0];
c.g[A3][0] = c.hi[0];
c.g[V0][0] = c.g[V0][0] << 32;
c.g[A3][0] = c.g[A3][0] << 32;
c.g[V0][0] = c.g[V0][0] >> 32;
c.g[A2][0] = c.g[A2][0] | c.g[A1][0];
c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
c.sd((c.g[T0][0] as u32).wrapping_add(0x20), c.g[A2][0]);
c.g[V0][0] = 0x6c000000;
c.sd((c.g[T0][0] as u32).wrapping_add(0x30), c.g[A3][0]);
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.sd((c.g[T0][0] as u32).wrapping_add(0x10), 0u64);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[T0][0] as u32).wrapping_add(0x28), c.g[S6][0]);
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.sd((c.g[T0][0] as u32), c.g[V0][0]);
c.sd((c.g[T0][0] as u32).wrapping_add(0x38), 0u64);
c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2));
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
c.g[A3][0] = 0xffffffffffffffff;
let t = c.g[A2][0] == c.g[A3][0];
c.g[A0][0] = c.g[T0][0];
if t {
break 'b_379a90;
} else {
'l_379a68: loop {
c.cycles += 10;
c.lq(2, (c.g[S1][0] as u32));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
c.sq(2, (c.g[A0][0] as u32));
c.lq(3, (c.g[S1][0] as u32).wrapping_add(0x10));
c.sq(3, (c.g[A0][0] as u32).wrapping_add(0x10));
c.lq(2, (c.g[S1][0] as u32).wrapping_add(0x20));
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x30));
c.sq(2, (c.g[A0][0] as u32).wrapping_add(0x20));
let t = c.g[A2][0] != c.g[A3][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
if t {
if event_due(c) { deliver_event_here(c, 0x00379a68); } continue 'l_379a68;
} else {
break 'b_379a90;
}
}
}
}
c.cycles += 10;
c.g[V0][0] = 0x30;
c.g[S3][0] = c.g[A1][0];
c.mult(0, 3, 18, 2);
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x60));
c.g[V0][0] = ((c.g[S3][0] as i64) < 3i64) as u64;
c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.sd((c.g[T0][0] as u32), 0u64);
c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[FP][0]);
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00379978); } continue 'l_379978;
} else {
break 'b_379ab8;
}
}
}
}
'b_379b6c: {
c.cycles += 11;
c.g[A1][0] = c.g[T0][0];
c.g[A0][0] = c.g[S5][0];
c.g[RA][0] = sx(0x00379ac8);
c.g[A2][0] = 0x4;
Callee { addr: 0x0038f668, f: ee::s12::SprStream_EndBlockAligned }.call(c, 0x00379ac8)?;
c.g[T1][0] = c.g[V0][0];
c.g[A3][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18f0));
c.sw((c.g[S4][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xe84));
if t {
break 'b_379b6c;
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
c.g[A1][0] = c.lw((c.g[SP][0] as u32));
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[A1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_379b6c;
}
}
c.cycles += 25;
c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x5a00));
c.g[A1][0] = 0xfffffffffffffc3f;
c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xe84));
c.g[A2][0] = 0xffffffffc0000000;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sw((c.g[S4][0] as u32).wrapping_add(0x5a00), c.g[V1][0]);
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xe84));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
c.pc = t; return Ok(());
    });
}

/// `void PS2GraphicsMan_DrawSkinnedModelShadow(self, Model* model, base, float* colour)`
/// (0x0037b548, `PS2GraphicsMan_v100`): sets the shadow draw state, then for
/// every flagged bone writes a dma packet per track (header with the colour,
/// the bone matrices copied from the scratchpad, and the reference to the
/// bone's vertex list), and records the draw in the deferred draw ring. No
/// callees. Ported from the generated body (labelled blocks kept).
pub fn ps2_graphics_man_draw_skinned_model_shadow(c: &mut Cpu) {
    guest(c, |c| {
'b_37b938: {
c.cycles += 78;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
c.g[T8][0] = c.g[A0][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[V1][0] = 0x30000000;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[T5][0] = 0xffffffffffffff83;
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[T7][0] = 0xfffffffffffff07f;
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[T2][0] = 0xfffffffff00f0000;
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[T2][0] = c.g[T2][0] | 0xffff;
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S0][0] = 0x6400000;
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[T0][0] = 0xfffffffffe7f0000;
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[T0][0] = c.g[T0][0] | 0xffff;
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S1][0] = 0x1000000;
c.g[T3][0] = 0xffffffffffcf0000;
c.g[S2][0] = 0x300000;
c.g[V0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x5a00));
c.g[T3][0] = c.g[T3][0] | 0xffff;
c.g[S6][0] = c.g[A1][0];
c.g[T4][0] = 0xfffffffffff00000;
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.g[V0][0] | c.g[V1][0];
c.g[T4][0] = c.g[T4][0] | 0xfff;
c.g[T1][0] = 0x10000;
c.g[A0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0xe84));
c.g[T1][0] = c.g[T1][0] | 0x4000;
c.g[T6][0] = 0xffffffffffffffff;
c.g[T9][0] = c.g[A3][0];
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[A3][0] = 0u64;
c.sw((c.g[SP][0] as u32), c.g[V1][0]);
c.g[V0][0] = c.g[V0][0] | 0x30;
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
c.g[V0][0] = c.g[V0][0] | 0x4;
c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
c.g[V1][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T7][0];
c.g[V0][0] = c.g[V0][0] | 0x580;
c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
c.g[V0][0] = c.g[V0][0] | c.g[S0][0];
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
c.g[V0][0] = c.g[V0][0] | c.g[S1][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0xe84));
c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T6][0]);
c.g[V0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0xe84));
c.sh((c.g[V0][0] as u32).wrapping_add(0x12), c.g[T6][0]);
c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lh((c.g[S6][0] as u32).wrapping_add(0x48));
let t = (c.g[V0][0] as i64) <= 0;
c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
if t {
break 'b_37b938;
} else {
c.cycles += 22;
c.g[V0][0] = 0x0;
c.g[A0][0] = 0x0;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x80));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x70));
c.g[A1][0] = 0x14000000;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
c.g[A0][0] = sx((c.g[A0][0] as u32) >> 3);
c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
c.g[A0][0] = c.g[A0][0] | c.g[A1][0];
c.mfc1(5, 0);
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V0][0] = c.g[V0][0] >> 32;
c.g[A0][0] = c.g[A0][0] << 32;
c.g[V1][0] = c.g[A1][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[V1][0] = c.g[V1][0] << 32;
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A0][0]);
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[AT][0] = 0x43000000;
c.mtc1(1, 4);
'l_37b6d8: loop {
'b_37b914: {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[T2][0] as u32));
c.g[S7][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
c.g[V0][0] = c.g[V0][0] & 0x1;
let t = c.g[V0][0] == 0u64;
c.g[FP][0] = sx((c.g[T2][0] as u32).wrapping_add(0x14));
if t {
break 'b_37b914;
} else {
c.cycles += 62;
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
c.g[A3][0] = 0x10000000;
c.g[A3][0] = c.g[A3][0] | 0x3;
c.g[A0][0] = 0x50000000;
c.g[A0][0] = c.g[A0][0] | 0x2;
c.g[A0][0] = c.g[A0][0] << 16;
c.g[A0][0] = c.g[A0][0] | 0x1100;
c.g[A0][0] = c.g[A0][0] << 16;
c.g[A1][0] = 0x10000000;
c.g[A1][0] = c.g[A1][0] << 32;
c.g[A1][0] = c.g[A1][0] | 0x8001;
c.sd((c.g[V0][0] as u32), c.g[A3][0]);
c.g[A2][0] = 0xe;
c.g[A3][0] = 0x1;
c.g[T1][0] = 0u64;
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
c.g[T0][0] = sx((c.g[T3][0] as u32) << 3);
c.g[V1][0] = c.lw((c.g[SP][0] as u32));
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32));
c.sd((c.g[V1][0] as u32).wrapping_add(0x20), c.g[A1][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x28), c.g[A2][0]);
c.lwc1(2, (c.g[T9][0] as u32).wrapping_add(0x8));
c.lwc1(3, (c.g[T9][0] as u32).wrapping_add(0xc));
c.lwc1(0, (c.g[T9][0] as u32).wrapping_add(0x4));
c.mul_s(2, 2, 4);
c.lwc1(1, (c.g[T9][0] as u32));
c.mul_s(3, 3, 4);
c.mul_s(0, 0, 4);
c.g[A2][0] = c.lw((c.g[SP][0] as u32));
c.cvt_w_s(5, 2);
c.mfc1(2, 5);
c.mul_s(1, 1, 4);
c.cvt_w_s(2, 3);
c.mfc1(3, 2);
c.cvt_w_s(2, 0);
c.mfc1(5, 2);
c.cvt_w_s(0, 1);
c.mfc1(4, 0);
c.g[V0][0] = c.g[V0][0] << 8;
c.g[V1][0] = c.g[V1][0] << 16;
c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[V1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.g[A0][0] << 24;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[A2][0] as u32).wrapping_add(0x30), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32));
c.sd((c.g[V1][0] as u32).wrapping_add(0x38), c.g[A3][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x3c));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0xc));
c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
let t = (c.g[A0][0] as i64) <= 0;
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
if t {
break 'b_37b914;
} else {
c.cycles += 15;
c.g[A0][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x18));
c.g[T3][0] = 0x6c000000;
c.g[A1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = c.g[SP][0];
c.g[S5][0] = 0x10000000;
c.g[S4][0] = 0x8000;
c.g[S4][0] = c.g[S4][0] << 45;
c.g[T3][0] = c.g[T3][0] | 0x11;
c.g[S3][0] = 0x1000000;
c.g[S3][0] = c.g[S3][0] | 0x101;
c.g[S2][0] = 0x60010000;
c.g[S2][0] = c.g[S2][0] | 0x3ff;
c.g[T7][0] = c.g[A0][0] | c.g[A1][0];
c.g[S1][0] = 0x50000000;
'l_37b820: loop {
'b_37b8b4: {
c.cycles += 20;
c.g[A2][0] = c.lw((c.g[A3][0] as u32));
c.g[T5][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
c.g[A0][0] = c.lw((c.g[SP][0] as u32));
c.g[T6][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx((c.g[A2][0] as u32) << 18);
c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
c.g[V0][0] = c.g[V0][0] | c.g[T3][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[T0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x13e8));
c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.g[V1][0] | c.g[S5][0];
c.g[V0][0] = c.g[V0][0] | c.g[S3][0];
c.sd((c.g[A0][0] as u32), c.g[V1][0]);
c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[T4][0] = sx((c.g[T1][0] as u32) << 3);
c.sd((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[S4][0]);
let t = (c.g[A2][0] as i64) <= 0;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
if t {
break 'b_37b8b4;
} else {
c.cycles += 2;
c.g[V1][0] = c.g[A1][0];
c.g[A1][0] = c.g[A2][0];
'l_37b878: loop {
c.cycles += 15;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.lqc2(1, (c.g[V0][0] as u32));
c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[A0][0] as u32));
c.sqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
let t = c.g[A1][0] != 0u64;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x0037b878); } continue 'l_37b878;
} else {
break 'b_37b8b4;
}
}
}
}
c.cycles += 24;
c.g[V0][0] = c.g[A2][0] << 32;
c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T7][0]);
c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
c.g[T1][0] = c.g[T5][0];
c.sd((c.g[A0][0] as u32), c.g[V0][0]);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
c.sw((c.g[S0][0] as u32), c.g[A0][0]);
c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0x10));
c.g[A1][0] = c.lw((c.g[SP][0] as u32));
c.g[V1][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.lwu((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[S1][0];
c.sd((c.g[A1][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0xc));
c.g[V1][0] = ((c.g[T1][0] as i64) < (c.g[V1][0] as i64)) as u64;
let t = c.g[V1][0] != 0u64;
c.g[A3][0] = c.g[T6][0];
if t {
if event_due(c) { deliver_event_here(c, 0x0037b820); } continue 'l_37b820;
} else {
break 'b_37b914;
}
}
}
}
}
c.cycles += 9;
c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_add(0xc));
c.g[A3][0] = c.g[S7][0];
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.g[T2][0] = c.g[FP][0];
c.g[V0][0] = c.lh((c.g[S6][0] as u32).wrapping_add(0x48));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A2][0]);
if t {
if event_due(c) { deliver_event_here(c, 0x0037b6d8); } continue 'l_37b6d8;
} else {
break 'b_37b938;
}
}
}
}
'b_37b9ec: {
c.cycles += 11;
c.g[V1][0] = c.lw((c.g[SP][0] as u32));
c.g[V0][0] = 0xfff0000;
c.g[V0][0] = c.g[V0][0] | 0xffff;
c.g[A3][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x18f0));
c.g[T1][0] = c.g[V1][0] & c.g[V0][0];
c.g[T0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0xe84));
c.sw((c.g[T8][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = 0x60000;
if t {
break 'b_37b9ec;
} else {
c.cycles += 34;
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
c.ldl(7, (c.g[T0][0] as u32).wrapping_add(0x7));
c.ldr(7, (c.g[T0][0] as u32));
c.ldl(11, (c.g[T0][0] as u32).wrapping_add(0xf));
c.ldr(11, (c.g[T0][0] as u32).wrapping_add(0x8));
c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x7));
c.sdr(7, (c.g[V0][0] as u32));
c.sdl(11, (c.g[V0][0] as u32).wrapping_add(0xf));
c.sdr(11, (c.g[V0][0] as u32).wrapping_add(0x8));
c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[V1][0] = 0x1;
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), c.g[V1][0]);
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[A2][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
break 'b_37b9ec;
}
}
c.cycles += 24;
c.g[V1][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0x5a00));
c.g[A1][0] = 0xfffffffffffffc3f;
c.g[A0][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0xe84));
c.g[A2][0] = 0xffffffffc0000000;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sw((c.g[T8][0] as u32).wrapping_add(0x5a00), c.g[V1][0]);
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[V1][0] = c.lw((c.g[T8][0] as u32).wrapping_add(0xe84));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
c.pc = t; return Ok(());
    });
}

/// `void PS2BezierMan_DrawPatchFX(PS2BezierMan* man, PatchRecord* rec, Patch* patch, uint* handle)`
/// (0x0038d690): the snow sparkle effect of a patch: from the patch's box
/// diagonal, the sparkle count, the snow density and the lod (low lods halve
/// it) it derives a sparkle count `n`; when `n > 0` it writes a dma packet
/// (the patch matrices and the manager's two constant blocks) through
/// `SprStream_BeginBlock` / `EndBlockAligned` and records the draw in the
/// deferred draw ring. Ported from the generated body.
pub fn ps2_bezier_man_draw_patch_fx(c: &mut Cpu) {
    guest(c, |c| {
'b_38d940: {
c.cycles += 15;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
c.g[V1][0] = 0x800000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S0][0] = c.g[A1][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S2][0] = c.g[A0][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
let t = c.g[V0][0] == 0u64;
c.g[S4][0] = c.g[A3][0];
if t {
break 'b_38d940;
} else {
'b_38d74c: {
c.cycles += 30;
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x164));
c.lwc1(4, (c.g[S0][0] as u32).wrapping_add(0x168));
c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x158));
c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0x15c));
c.lwc1(5, (c.g[S0][0] as u32).wrapping_add(0x16c));
c.sub_s(2, 2, 0);
c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x160));
c.sub_s(3, 3, 4);
c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x26b4));
c.sub_s(1, 1, 5);
c.lwc1(6, (c.g[GP][0] as u32).wrapping_add(0x1258));
c.mul_s(0, 2, 2);
c.swc1(2, (c.g[SP][0] as u32));
c.mul_s(5, 3, 3);
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
c.mul_s(6, 6, 4);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
c.mul_s(4, 1, 1);
c.g[S3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
c.add_s(0, 0, 5);
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0xc4));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
c.add_s(0, 0, 4);
c.mul_s(0, 0, 6);
c.mul_s(0, 0, 1);
c.cvt_w_s(1, 0);
c.mfc1(17, 1);
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = sx((c.g[S1][0] as u32) >> 31);
if t {
break 'b_38d74c;
} else {
c.cycles += 2;
c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[S1][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
break 'b_38d74c;
}
}
c.cycles += 2;
let t = (c.g[S1][0] as i64) <= 0;
c.g[A2][0] = 0xffffffffffffffff;
if t {
break 'b_38d940;
} else {
'b_38d7e4: {
c.cycles += 29;
c.g[S5][0] = c.lw((c.g[S4][0] as u32));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[A3][0] = 0u64;
c.g[RA][0] = sx(0x0038d768);
c.g[A1][0] = c.g[S5][0];
Callee { addr: 0x0038f460, f: ee::s12::SprStream_BeginBlock }.call(c, 0x0038d768)?;
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0x18;
c.sd((c.g[V0][0] as u32), c.g[V1][0]);
c.g[A0][0] = 0xd82d;
c.g[A0][0] = c.g[A0][0] << 47;
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x420));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[T1][0] = c.g[A2][0];
c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3e0));
c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
c.sw((c.g[S2][0] as u32).wrapping_add(0x43c), c.g[S1][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x120));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.g[V0][0];
let t = c.g[V1][0] == c.g[A0][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
if t {
break 'b_38d7e4;
} else {
'l_38d7c8: loop {
c.cycles += 7;
c.lq(2, (c.g[A1][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A0][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x0038d7c8); } continue 'l_38d7c8;
} else {
break 'b_38d7e4;
}
}
}
}
'b_38d814: {
c.cycles += 5;
c.sw((c.g[A2][0] as u32), c.g[V1][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x40));
let t = c.g[V1][0] == c.g[A1][0];
c.g[A0][0] = c.g[A3][0];
if t {
break 'b_38d814;
} else {
'l_38d7f8: loop {
c.cycles += 7;
c.lq(2, (c.g[A0][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A1][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x0038d7f8); } continue 'l_38d7f8;
} else {
break 'b_38d814;
}
}
}
}
'b_38d844: {
c.cycles += 5;
c.sw((c.g[T1][0] as u32), c.g[V1][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
let t = c.g[V1][0] == c.g[A1][0];
c.g[A0][0] = c.g[T0][0];
if t {
break 'b_38d844;
} else {
'l_38d828: loop {
c.cycles += 7;
c.lq(2, (c.g[A0][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A1][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x0038d828); } continue 'l_38d828;
} else {
break 'b_38d844;
}
}
}
}
'b_38d934: {
c.cycles += 27;
c.g[V0][0] = 0x0;
c.sw((c.g[T1][0] as u32), c.g[V1][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1460));
c.g[V1][0] = 0x14000000;
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A1][0] = 0x8800;
c.g[A1][0] = c.g[A1][0] << 45;
c.g[V0][0] = c.g[V0][0] << 32;
c.g[A2][0] = 0x4;
c.sd((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[A1][0] = c.g[V0][0];
c.g[RA][0] = sx(0x0038d894);
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
Callee { addr: 0x0038f668, f: ee::s12::SprStream_EndBlockAligned }.call(c, 0x0038d894)?;
c.g[T1][0] = c.g[V0][0];
c.sw((c.g[S4][0] as u32), c.g[T1][0]);
c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18f0));
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
if t {
break 'b_38d934;
} else {
c.cycles += 33;
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
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S5][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_38d934;
}
}
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S4][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.sw((c.g[S4][0] as u32), c.g[V0][0]);
break 'b_38d940;
}
}
}
c.cycles += 9;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
c.pc = t; return Ok(());
    });
}

/// `void PS2GraphicsMan_DrawEmitter(PS2GraphicsMan* self, Emitter* obj)`
/// (0x00380518, `PS2GraphicsMan_v82`): unless static emitters are disabled,
/// pushes a draw state, writes a dma packet holding the emitter's 21 quads
/// (`SprStream_BeginBlock` / `EndBlockAligned`), updates the view matrices
/// when the texture is not ready, records the draw in the deferred draw ring
/// and pops the state. Ported from the generated body.
pub fn ps2_graphics_man_draw_emitter(c: &mut Cpu) {
    guest(c, |c| {
'b_380784: {
c.cycles += 9;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1408));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S1][0] = c.g[A1][0];
let t = c.g[V0][0] != 0u64;
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
if t {
break 'b_380784;
} else {
'b_38066c: {
c.cycles += 68;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
c.g[T1][0] = 0xfffffffffffffc3f;
c.g[T2][0] = 0xffffffffc0000000;
c.g[T3][0] = 0xfffffffffffffffc;
c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[V0][0] as u32));
c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x1b));
c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x14));
c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x23));
c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x1c));
c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[A1][0]);
c.g[T0][0] = 0xffffffffe0000000;
c.g[T0][0] = c.g[T0][0] | 0x3ff;
c.g[T4][0] = 0xfffffffffffffc1f;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
c.g[A2][0] = 0xffffffffffffffff;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[A3][0] = 0u64;
c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14));
c.sw((c.g[S0][0] as u32).wrapping_add(0xe84), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
c.g[V0][0] = c.g[V0][0] | 0x100;
c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
c.g[V0][0] = c.g[V0][0] | 0x2;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
c.g[V0][0] = c.g[V0][0] | 0xe0;
c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a00));
c.g[RA][0] = sx(0x003805fc);
c.g[A1][0] = c.g[S2][0];
Callee { addr: 0x0038f460, f: ee::s12::SprStream_BeginBlock }.call(c, 0x003805fc)?;
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0x17;
c.sd((c.g[V0][0] as u32), c.g[V1][0]);
c.g[A1][0] = 0xd82b;
c.g[A1][0] = c.g[A1][0] << 47;
c.g[A3][0] = c.g[SP][0];
c.g[A2][0] = c.g[S1][0];
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
c.g[V1][0] = c.lw((c.g[SP][0] as u32));
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
c.g[A0][0] = c.lw((c.g[SP][0] as u32));
c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x170));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.g[V0][0];
let t = c.g[V1][0] == c.g[A0][0];
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
if t {
break 'b_38066c;
} else {
c.cycles += 1;
'l_380650: loop {
c.cycles += 7;
c.lq(2, (c.g[A2][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A0][0];
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00380650); } continue 'l_380650;
} else {
break 'b_38066c;
}
}
}
}
'b_3806d0: {
c.cycles += 23;
c.g[V0][0] = 0x0;
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x0));
c.g[V1][0] = 0x14000000;
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
c.g[A0][0] = c.lw((c.g[SP][0] as u32));
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A1][0] = 0x8800;
c.g[A1][0] = c.g[A1][0] << 45;
c.g[V0][0] = c.g[V0][0] << 32;
c.g[A2][0] = 0x4;
c.sd((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[A1][0] = c.g[V0][0];
c.g[RA][0] = sx(0x003806bc);
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
Callee { addr: 0x0038f668, f: ee::s12::SprStream_EndBlockAligned }.call(c, 0x003806bc)?;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6b90));
let t = c.g[V1][0] != 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
if t {
break 'b_3806d0;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x003806d0);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x003806d0)?;
break 'b_3806d0;
}
}
'b_38076c: {
c.cycles += 6;
c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18f0));
c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
if t {
break 'b_38076c;
} else {
c.cycles += 33;
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
c.ldl(7, (c.g[T0][0] as u32).wrapping_add(0x7));
c.ldr(7, (c.g[T0][0] as u32));
c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0xf));
c.ldr(3, (c.g[T0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x7));
c.sdr(7, (c.g[V0][0] as u32));
c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0xf));
c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x8));
c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S2][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_38076c;
}
}
c.cycles += 6;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a00));
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x14));
c.sw((c.g[S0][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0xe84), c.g[V1][0]);
break 'b_380784;
}
}
c.cycles += 6;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.pc = t; return Ok(());
    });
}

/// `void FocusMan_CollectOctreeNode(FocusMan* scan, OctreeNode* node, CellKey* key)`
/// (0x00144368): for every focus box that overlaps the octree cell `key`
/// (`level, x, y, z`; cell size `2^level`, grown by 0.2 of a cell), hand the
/// node to `FocusMan_CollectNodeObjects` (leaf when `level < 14`) and, above
/// the leaf levels, recurse into the eight children with the doubled key
/// (children visited in the original's order: +x, +y, -x, +z, +x, -y, -x, then
/// the first child). Ported from the generated body (labelled blocks kept).
pub fn focus_man_collect_octree_node(c: &mut Cpu) {
    guest(c, |c| {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
c.g[A3][0] = 0u64;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S1][0] = c.g[A2][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S2][0] = c.g[A0][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S3][0] = c.g[A1][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
c.g[T1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
'l_144398: loop {
'b_1445f8: {
'b_1445f4: {
c.cycles += 2;
let t = c.g[A3][0] == c.g[T1][0];
c.g[A0][0] = sx((c.g[A3][0] as u32) << 5);
if t {
break 'b_1445f4;
} else {
'b_1444a0: {
c.cycles += 18;
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
c.mtc1(6, 1);
c.cvt_s_w(1, 1);
c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x6e1c));
c.g[S0][0] = c.g[V0][0];
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7f));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
c.sub_s(1, 1, 3);
c.mtc1(2, 2);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x10));
c.mul_s(1, 1, 2);
c.c_le_s(1, 0);
let t = !c.fcc();
c.g[T0][0] = 0u64;
if t {
break 'b_1444a0;
} else {
c.cycles += 9;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x14));
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.sub_s(0, 0, 3);
c.mul_s(0, 0, 2);
c.c_le_s(0, 1);
let t = !c.fcc();

if t {
break 'b_1444a0;
} else {
c.cycles += 10;
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x18));
c.mtc1(5, 0);
c.cvt_s_w(0, 0);
c.sub_s(0, 0, 3);
c.mul_s(0, 0, 2);
c.c_le_s(0, 1);
let t = !c.fcc();
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
if t {
break 'b_1444a0;
} else {
c.cycles += 9;
c.lwc1(1, (c.g[A0][0] as u32));
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 3);
c.mul_s(0, 0, 2);
c.c_le_s(1, 0);
let t = !c.fcc();
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
if t {
break 'b_1444a0;
} else {
c.cycles += 9;
c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x4));
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 3);
c.mul_s(0, 0, 2);
c.c_le_s(1, 0);
let t = !c.fcc();
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
if t {
break 'b_1444a0;
} else {
c.cycles += 9;
c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 3);
c.mul_s(0, 0, 2);
c.c_le_s(1, 0);
if c.fcc() {
c.g[T0][0] = 0x1;
break 'b_1444a0;
} else {
break 'b_1444a0;
}
}
}
}
}
}
}
c.cycles += 2;
let t = c.g[T0][0] == 0u64;
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
if t {
if event_due(c) { deliver_event_here(c, 0x00144398); } continue 'l_144398;
} else {
c.cycles += 7;
c.g[S0][0] = ((c.g[S0][0] as i64) < 14i64) as u64;
c.g[A0][0] = c.lw((c.g[S2][0] as u32));
c.g[A1][0] = c.g[S3][0];
c.g[RA][0] = sx(0x001444bc);
c.g[A2][0] = c.g[S0][0];
call_ptr(c, 0x00101a28, 0x001444bc)?;
if c.g[S0][0] != 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
break 'b_1445f8;
} else {
c.cycles += 4;
c.g[A1][0] = c.lw((c.g[S1][0] as u32));
c.g[V0][0] = 0xb;
let t = c.g[A1][0] == c.g[V0][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
if t {
break 'b_1445f4;
} else {
'b_144514: {
c.cycles += 12;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
c.g[A2][0] = c.lw((c.g[S3][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
c.sw((c.g[SP][0] as u32), c.g[A1][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
let t = c.g[A2][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[A0][0]);
if t {
break 'b_144514;
} else {
c.cycles += 4;
c.g[A1][0] = c.g[A2][0];
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x00144514);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x00144368, f: ee::s01::FocusMan_CollectOctreeNode }.call(c, 0x00144514)?;
break 'b_144514;
}
}
'b_144534: {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
let t = c.g[A1][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
if t {
break 'b_144534;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x00144534);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x00144368, f: ee::s01::FocusMan_CollectOctreeNode }.call(c, 0x00144534)?;
break 'b_144534;
}
}
'b_144554: {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
let t = c.g[A1][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
if t {
break 'b_144554;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x00144554);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x00144368, f: ee::s01::FocusMan_CollectOctreeNode }.call(c, 0x00144554)?;
break 'b_144554;
}
}
'b_144574: {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
let t = c.g[A1][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
if t {
break 'b_144574;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x00144574);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x00144368, f: ee::s01::FocusMan_CollectOctreeNode }.call(c, 0x00144574)?;
break 'b_144574;
}
}
'b_144594: {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
let t = c.g[A1][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
if t {
break 'b_144594;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x00144594);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x00144368, f: ee::s01::FocusMan_CollectOctreeNode }.call(c, 0x00144594)?;
break 'b_144594;
}
}
'b_1445b4: {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
let t = c.g[A1][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
if t {
break 'b_1445b4;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x001445b4);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x00144368, f: ee::s01::FocusMan_CollectOctreeNode }.call(c, 0x001445b4)?;
break 'b_1445b4;
}
}
'b_1445d4: {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
let t = c.g[A1][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
if t {
break 'b_1445d4;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x001445d4);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x00144368, f: ee::s01::FocusMan_CollectOctreeNode }.call(c, 0x001445d4)?;
break 'b_1445d4;
}
}
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
let t = c.g[A1][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
if t {
break 'b_1445f4;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[S2][0];
c.g[RA][0] = sx(0x001445f4);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x00144368, f: ee::s01::FocusMan_CollectOctreeNode }.call(c, 0x001445f4)?;
break 'b_1445f4;
}
}
}
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
break 'b_1445f8;
}
c.cycles += 6;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
c.pc = t; return Ok(());
}
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, check_reaching, machine};
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word and block counter of this module's callee fakes, in the
    /// machine's RAM (the fakes only act when the gate holds the magic).
    const OC1480_CTRL: u32 = DATA + 0xf_c000;
    const OC1480_MAGIC: u32 = 0x1480_d2a5;
    static OC1480_PREV_VIEW: AtomicUsize = AtomicUsize::new(0);
    static OC1480_PREV_BEGIN: AtomicUsize = AtomicUsize::new(0);
    static OC1480_PREV_END: AtomicUsize = AtomicUsize::new(0);

    /// Output blocks `SprStream_BeginBlock` hands out (0x4000 bytes each).
    const OC1480_OUT: u32 = DATA + 0x4_0000;
    /// The graphics manager under test (draw state stack, cursor, ring).
    const OC1480_SELF: u32 = DATA + 0x1_0000;
    /// The draw state (`m_pCurrentDrawState`).
    const OC1480_RS: u32 = DATA + 0x2_0000;
    /// The `SprStream` the gp slot `0x1384` points at.
    const OC1480_STREAM: u32 = DATA + 0x2_8000;
    /// Scratchpad bone matrices (64 bytes each).
    const OC1480_SCR: u32 = DATA + 0x3_0000;
    /// Vertex data of the tri strip.
    const OC1480_VERTS: u32 = DATA + 0x6_0000;
    /// The deferred draw ring (`ring + 0x69cc0` holds the key words).
    const OC1480_RING: u32 = DATA + 0x8_0000;
    /// Where the direct-write functions put their packets.
    const OC1480_PKT: u32 = DATA + 0xa_0000;

    /// `UpdateViewMatrices` stand-in: logs the call and returns (or unwinds).
    fn oc1480_fake_view(c: &mut Cpu) {
        if c.bus.read32(OC1480_CTRL) != OC1480_MAGIC {
            return fakes::fall_back(c, &OC1480_PREV_VIEW, ee::s12::f_0037d968__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0037_d968, ra);
    }

    /// `SprStream_BeginBlock` stand-in: hands out a fresh output block.
    fn oc1480_fake_begin(c: &mut Cpu) {
        if c.bus.read32(OC1480_CTRL) != OC1480_MAGIC {
            return fakes::fall_back(c, &OC1480_PREV_BEGIN, ee::s12::f_0038f460__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0038_f460, ra);
        if c.pc == ra {
            let n = c.bus.read32(OC1480_CTRL + 4);
            c.bus.write32(OC1480_CTRL + 4, n + 1);
            c.g[V0][0] = (OC1480_OUT + 0x4000 * (n % 8)) as u64;
        }
    }

    /// `SprStream_EndBlockAligned` stand-in: logs, returns a pseudo address.
    fn oc1480_fake_end(c: &mut Cpu) {
        if c.bus.read32(OC1480_CTRL) != OC1480_MAGIC {
            return fakes::fall_back(c, &OC1480_PREV_END, ee::s12::f_0038f668__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0038_f668, ra);
    }

    fn oc1480_install() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0037D968),
            oc1480_fake_view,
            &OC1480_PREV_VIEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038F460),
            oc1480_fake_begin,
            &OC1480_PREV_BEGIN,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038F668),
            oc1480_fake_end,
            &OC1480_PREV_END,
        );
    }

    /// A machine with the fakes armed (gate on, block counter 0).
    fn oc1480_env(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC1480_CTRL, OC1480_MAGIC);
        c.bus.write32(OC1480_CTRL + 4, 0);
        r
    }

    /// The deferred-draw ring: the entry count (full on every fifth seed) and
    /// the two key words.
    fn oc1480_ring(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let n = if seed % 5 == 0 { 2600 + seed % 3 } else { seed % 30 };
        c.bus.write32(OC1480_RING, n);
        c.bus.write32(OC1480_RING + 0x6_9cc0, r.next_u32());
        c.bus.write32(OC1480_RING + 0x6_9cc4, r.next_u32());
    }

    // ------------------------------------------------------------ tri strip

    /// A strip of 0..=200 vertices into blocks of several capacities (so the
    /// block is renewed in the loop), with the view matrices stale or not, the
    /// flag that picks the dma size, either render-state mode, a full ring and
    /// unwinding callees.
    fn oc1480_tri_strip_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1480_env(c, seed, 0x0037_9860, 12);
        let gp = GAME_GP;
        c.bus.write32(gp + 0x1290, r.next_u32() % 2);
        c.bus.write32(gp + 0x1384, OC1480_STREAM);
        let caps = [0x10u32, 0x40, 0xe0, 0x1000];
        c.bus.write32(OC1480_STREAM + 0x10, caps[((seed / 12) % 4) as usize]);
        c.bus.write32(OC1480_SELF + 0xe84, OC1480_RS);
        c.bus.write32(OC1480_SELF + 0x6b90, r.next_u32() % 2);
        c.bus.write32(OC1480_SELF + 0x5a00, OC1480_PKT + (r.next_u32() & 0xff0));
        c.bus.write32(OC1480_SELF + 0x18f0, OC1480_RING);
        for i in 0..8 {
            c.bus.write32(OC1480_RS + 4 * i, r.next_u32());
        }
        let low = if r.next_u32() % 2 == 0 { 0x8000 } else { 0x1234 };
        c.bus.write32(OC1480_RS + 0x10, (r.next_u32() & 0xffff_0000) | low);
        fill(c, &mut r, OC1480_VERTS, 0x1000);
        oc1480_ring(c, &mut r, seed);
        let counts = [0u32, 1, 2, 3, 4, 10, 72, 73, 100, 145, 150, 200];
        c.g[A0][0] = OC1480_SELF as u64;
        c.g[A1][0] = counts[(seed % 12) as usize] as u64;
        c.g[A2][0] = OC1480_VERTS as u64;
    }

    /// `PS2GraphicsMan_DrawTriStrip` matches the generated body.
    #[test]
    fn ps2_graphics_man_draw_tri_strip_matches() {
        oc1480_install();
        check_reaching(
            ee::s12::f_00379860__generated,
            ps2_graphics_man_draw_tri_strip,
            400,
            &[0x0037_98fc, 0x0037_993c, 0x0037_99b4, 0x0037_99cc, 0x0037_9ac8],
            &oc1480_tri_strip_setup,
        );
    }

    // ------------------------------------------------------- skinned shadow

    const OC1480_MODEL: u32 = DATA + 0x7_0000;
    const OC1480_BASE: u32 = DATA + 0x7_1000;
    const OC1480_REFS: u32 = DATA + 0x7_2000;
    const OC1480_PTRS: u32 = DATA + 0x7_3000;
    const OC1480_TRACKS: u32 = DATA + 0x7_4000;
    const OC1480_IDXS: u32 = DATA + 0x7_5000;
    const OC1480_COL: u32 = DATA + 0x7_6000;

    /// A model with -1..=5 bones (flag bit 0 decides which are drawn), 0..=3
    /// tracks per bone with 0..=4 matrix indices each, a random colour and a
    /// full or empty ring. A leaf function: no callees.
    fn oc1480_shadow_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x37b5_4800 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        c.bus.write32(OC1480_SELF + 0x5a00, OC1480_PKT + (r.next_u32() & 0xff0));
        c.bus.write32(OC1480_SELF + 0xe84, OC1480_RS);
        c.bus.write32(OC1480_SELF + 0x13e8, OC1480_SCR);
        c.bus.write32(OC1480_SELF + 0x18f0, OC1480_RING);
        for i in 0..8 {
            c.bus.write32(OC1480_RS + 4 * i, r.next_u32());
        }
        fill(c, &mut r, OC1480_SCR, 0x100);
        oc1480_ring(c, &mut r, seed);
        let nbones = [1u16, 0, 3, 0xffff, 2, 5, 4, 1][(seed % 8) as usize];
        c.bus.write32(OC1480_MODEL + 0x20, 0x100);
        c.bus.write16(OC1480_MODEL + 0x48, nbones);
        c.bus.write32(OC1480_MODEL + 0x3c, OC1480_TRACKS);
        let bones = OC1480_BASE + 0x100;
        let mut cnt = 0u32;
        for k in 0..5u32 {
            let b = bones + 0x14 * k;
            let ntracks = r.next_u32() % 4;
            c.bus.write32(b, if (seed >> (3 + k)) & 1 == 0 { 1 } else { r.next_u32() & !1 });
            c.bus.write32(b + 0xc, ntracks);
            let refs = OC1480_REFS + 0x40 * k;
            c.bus.write32(b + 0x10, refs);
            for j in 0..4u32 {
                let p = OC1480_PTRS + 4 * (4 * k + j);
                c.bus.write32(refs + 8 * j, p);
                c.bus.write32(p, r.next_u32());
            }
            for _ in 0..ntracks {
                let n = r.next_u32() % 5;
                let t = OC1480_TRACKS + 8 * cnt;
                let idx = OC1480_IDXS + 0x20 * cnt;
                c.bus.write32(t, n);
                c.bus.write32(t + 4, idx);
                for q in 0..4u32 {
                    c.bus.write32(idx + 4 * q, r.next_u32() % 8);
                }
                cnt += 1;
            }
        }
        for q in 0..4u32 {
            let v = r.f32(2.0);
            c.bus.write32(OC1480_COL + 4 * q, v);
        }
        c.g[A0][0] = OC1480_SELF as u64;
        c.g[A1][0] = OC1480_MODEL as u64;
        c.g[A2][0] = OC1480_BASE as u64;
        c.g[A3][0] = OC1480_COL as u64;
    }

    /// `PS2GraphicsMan_DrawSkinnedModelShadow` matches the generated body.
    #[test]
    fn ps2_graphics_man_draw_skinned_model_shadow_matches() {
        check(
            ee::s12::f_0037b548__generated,
            ps2_graphics_man_draw_skinned_model_shadow,
            300,
            &oc1480_shadow_setup,
        );
    }

    // ----------------------------------------------------------- patch fx

    const OC1480_MAN: u32 = DATA + 0x2_4000;
    const OC1480_REC: u32 = DATA + 0x3_8000;
    const OC1480_PATCH: u32 = DATA + 0x3_9000;
    const OC1480_HANDLE: u32 = DATA + 0x3_a000;

    /// Patch records with and without the sparkle flag, a box diagonal and
    /// density that give n <= 0, small n and halved n (low lod), a full ring
    /// and unwinding callees.
    fn oc1480_patch_fx_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1480_env(c, seed, 0x0038_d690, 3);
        let gp = GAME_GP;
        c.bus.write32(gp.wrapping_sub(0x26b4), 3.333_333_3e-7f32.to_bits());
        let counts = [3000.0f32, 1000.0, 300.0];
        c.bus.write32(gp + 0x1258, counts[((seed / 6) % 3) as usize].to_bits());
        c.bus.write32(gp + 0x2a90, OC1480_SELF);
        c.bus.write32(gp + 0x1384, OC1480_STREAM);
        let densities = [0.0f32, 1.0, -1.0, 0.5, 3.0];
        c.bus.write32(OC1480_SELF + 0xc4, densities[((seed / 3) % 5) as usize].to_bits());
        c.bus.write32(OC1480_SELF + 0x18f0, OC1480_RING);
        c.bus.write32(OC1480_SELF + 0xe84, OC1480_RS);
        for i in 0..8 {
            c.bus.write32(OC1480_RS + 4 * i, r.next_u32());
        }
        let flags = if seed % 6 == 5 {
            r.next_u32() & !0x0080_0000
        } else {
            r.next_u32() | 0x0080_0000
        };
        c.bus.write32(OC1480_REC + 0xc, flags);
        fill(c, &mut r, OC1480_REC + 0x40, 0x40);
        for k in 0..6u32 {
            let v = r.f32(100.0);
            c.bus.write32(OC1480_REC + 0x158 + 4 * k, v);
        }
        fill(c, &mut r, OC1480_MAN + 0x3e0, 0x18);
        c.bus.write32(OC1480_PATCH + 8, (seed / 5) % 4);
        c.bus.write32(OC1480_HANDLE, r.next_u32());
        oc1480_ring(c, &mut r, seed);
        c.g[A0][0] = OC1480_MAN as u64;
        c.g[A1][0] = OC1480_REC as u64;
        c.g[A2][0] = OC1480_PATCH as u64;
        c.g[A3][0] = OC1480_HANDLE as u64;
    }

    /// `PS2BezierMan_DrawPatchFX` matches the generated body.
    #[test]
    fn ps2_bezier_man_draw_patch_fx_matches() {
        oc1480_install();
        check_reaching(
            ee::s12::f_0038d690__generated,
            ps2_bezier_man_draw_patch_fx,
            400,
            &[0x0038_d768, 0x0038_d894],
            &oc1480_patch_fx_setup,
        );
    }

    // ------------------------------------------------------------ emitter

    const OC1480_OBJ: u32 = DATA + 0x3_0800;

    /// Emitters disabled or not, the view matrices stale or not, a full ring
    /// and unwinding callees.
    fn oc1480_emitter_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1480_env(c, seed, 0x0038_0518, 4);
        let gp = GAME_GP;
        c.bus.write32(gp + 0x1408, (seed % 7 == 0) as u32);
        c.bus.write32(gp + 0x1384, OC1480_STREAM);
        c.bus.write32(OC1480_SELF + 0xe84, OC1480_RS);
        c.bus.write32(OC1480_SELF + 0x5a00, OC1480_PKT + (r.next_u32() & 0xff0));
        c.bus.write32(OC1480_SELF + 0x6b90, r.next_u32() % 2);
        c.bus.write32(OC1480_SELF + 0x18f0, OC1480_RING);
        fill(c, &mut r, OC1480_RS, 0x20);
        fill(c, &mut r, OC1480_OBJ, 0x60);
        oc1480_ring(c, &mut r, seed);
        c.g[A0][0] = OC1480_SELF as u64;
        c.g[A1][0] = OC1480_OBJ as u64;
    }

    /// `PS2GraphicsMan_DrawEmitter` matches the generated body.
    #[test]
    fn ps2_graphics_man_draw_emitter_matches() {
        oc1480_install();
        check_reaching(
            ee::s12::f_00380518__generated,
            ps2_graphics_man_draw_emitter,
            400,
            &[0x0038_05fc, 0x0038_06bc, 0x0038_06d0],
            &oc1480_emitter_setup,
        );
    }

    // -------------------------------------------------- focus octree node

    const OC1480_NODES: u32 = DATA + 0x2_0000;
    const OC1480_KEY: u32 = DATA + 0x200;
    const OC1480_BOXES: u32 = DATA + 0x400;

    /// A scan of 0..=3 boxes against a key at levels 10..=15 (collect leaf
    /// cells, or descend into the eight children of a cell) and a four-level
    /// octree (children present with probability 1/2, 1/3 or 1/4); the real
    /// `FocusMan_CollectNodeObjects` runs on nodes with empty object lists.
    fn oc1480_focus_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x1443_6800 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP - 0x6e1c, 0.2f32.to_bits());
        let scan = DATA;
        let boxes = seed % 4;
        c.bus.write32(scan, DATA + 0x100);
        c.bus.write32(scan + 4, OC1480_BOXES);
        c.bus.write32(scan + 8, boxes);
        let level = [15u32, 14, 15, 14, 13, 12, 11, 10][((seed / 4) % 8) as usize];
        let size = (1u32 << level) as f32;
        c.bus.write32(OC1480_KEY, level);
        for a in 1..4u32 {
            c.bus
                .write32(OC1480_KEY + 4 * a, (r.next_u32() % 3).wrapping_sub(1));
        }
        for i in 0..boxes {
            let b = OC1480_BOXES + 0x20 * i;
            for a in 0..3u32 {
                let (lo, hi) = match (seed / 32 + i) % 3 {
                    0 => (-4.0 * size, 4.0 * size),
                    1 => {
                        let lo = ((r.next_u32() % 6000) as f32 / 1000.0 - 3.0) * size;
                        (lo, lo + (r.next_u32() % 3000) as f32 / 1000.0 * size)
                    }
                    _ => {
                        let lo = ((r.next_u32() % 4000) as f32 / 1000.0 - 2.0) * size;
                        (lo, lo + size * 0.1)
                    }
                };
                c.bus.write32(b + 4 * a, lo.to_bits());
                c.bus.write32(b + 0x10 + 4 * a, hi.to_bits());
            }
        }
        let p = [2u32, 3, 4][(seed % 3) as usize];
        // Root 0; depth 1: 1..=8; depth 2: 9..=16; depth 3: 17..=24 (no children).
        for k in 0..25u32 {
            let node = OC1480_NODES + 0x40 * k;
            c.bus.write32(node + 0x20, 0);
            for ch in 0..8u32 {
                let child = match k {
                    0 => 1 + ch,
                    1..=8 => 9 + ch,
                    9..=16 => 17 + ch,
                    _ => 0,
                };
                let present = child != 0 && r.next_u32() % p == 0;
                let target = if present { OC1480_NODES + 0x40 * child } else { 0 };
                c.bus.write32(node + 4 * ch, target);
            }
        }
        c.g[A0][0] = scan as u64;
        c.g[A1][0] = OC1480_NODES as u64;
        c.g[A2][0] = OC1480_KEY as u64;
    }

    /// `FocusMan_CollectOctreeNode` matches the generated body.
    #[test]
    fn focus_man_collect_octree_node_matches() {
        check(
            ee::s01::f_00144368__generated,
            focus_man_collect_octree_node,
            400,
            &oc1480_focus_setup,
        );
    }
}
