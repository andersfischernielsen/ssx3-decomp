//! Staging module oc1473 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1473`.

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
    0x0036_3490,
    ps2_graphics_man_render_frame,
    "PS2GraphicsMan_RenderFrame",
)];

/// `void PS2GraphicsMan_RenderFrame(cPS2DeferManager* self, int* pktAddr, int* out1, int* path, int* out2)`
/// (0x00363490): the frame's draw-packet builder. Sorts the deferred draw
/// buckets by pass, emits state changes, texture binds and DMA tags per
/// bucket, and runs the depth-fog, bloom and colour-filter passes at the
/// pass boundaries. Converted from the generated body (every register, cycle
/// and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ps2_graphics_man_render_frame(c: &mut Cpu) {
    if c.pc != 0x00363490 {
        return ee::s12::f_00363490__generated(c);
    }
    guest(c, |c| {
'b_363b08: {
c.cycles += 68;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[FP][0] = c.g[A0][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), c.g[A2][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T0][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A1][0]);
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
c.g[RA][0] = sx(0x003634d4);
c.g[S5][0] = c.g[A3][0];
Callee { addr: 0x00362de8, f: ee::s12::DeferMan_BuildBuckets }.call(c, 0x003634d4)?;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
c.g[A2][0] = 0x30000000;
c.g[V1][0] = 0x60000;
c.g[A3][0] = 0x500000;
c.sw((c.g[V0][0] as u32), 0u64);
c.g[V1][0] = c.g[V1][0] | 0x9ca8;
c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
c.sw((c.g[A0][0] as u32), 0u64);
c.g[A0][0] = c.g[FP][0];
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1420));
c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x7));
c.ldr(6, (c.g[V0][0] as u32));
c.ldl(8, (c.g[V0][0] as u32).wrapping_add(0xf));
c.ldr(8, (c.g[V0][0] as u32).wrapping_add(0x8));
c.g[T1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
c.sdl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
c.sdr(6, (c.g[V1][0] as u32));
c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
c.g[RA][0] = sx(0x00363540);
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
Callee { addr: 0x00364360, f: ee::s12::DeferMan_EmitDefaultState }.call(c, 0x00363540)?;
c.g[RA][0] = sx(0x00363548);
c.g[A0][0] = c.g[FP][0];
Callee { addr: 0x00364050, f: ee::s12::DeferMan_SortBuckets }.call(c, 0x00363548)?;
c.g[RA][0] = sx(0x00363550);
Callee { addr: 0x003666f8, f: ee::s12::TexMan_ResetUsageTable }.call(c, 0x00363550)?;
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[A1][0] = 0x5;
c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V1][0]);
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A0][0]);
c.g[RA][0] = sx(0x0036356c);
c.g[A0][0] = c.g[V1][0];
Callee { addr: 0x0038f300, f: ee::s12::SprStream_SetMode }.call(c, 0x0036356c)?;
c.g[S3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18f4));
c.g[A0][0] = c.g[A1][0];
c.g[RA][0] = sx(0x00363580);
c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[A1][0]);
call_ptr(c, 0x00396128, 0x00363580)?;
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
c.g[RA][0] = sx(0x0036358c);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1f60));
Callee { addr: 0x00365f68, f: ee::s12::VramHeap_NodesToScratchpad }.call(c, 0x0036358c)?;
c.g[V0][0] = 0x60000;
c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x7ca4));
let t = c.g[V0][0] == 0u64;
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
if t {
break 'b_363b08;
} else {
'b_363640: {
c.cycles += 4;
c.g[V1][0] = 0u64;
c.g[S2][0] = 0u64;
let t = (c.g[V0][0] as i64) <= 0;
c.sw((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
if t {
break 'b_363640;
} else {
c.cycles += 4;
c.g[T1][0] = c.g[A0][0];
c.g[T3][0] = 0x540000;
c.g[T2][0] = sx((c.g[T1][0] as u32).wrapping_add(0x7cac));
c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
'l_3635c0: loop {
'b_36362c: {
c.cycles += 4;
c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A1][0] = c.lw((c.g[V0][0] as u32));
let t = c.g[A1][0] == 0u64;
c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
if t {
break 'b_36362c;
} else {
c.cycles += 2;
c.g[A2][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x5480));
c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
'l_3635d8: loop {
'b_3635fc: {
c.cycles += 3;
c.g[V0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x4));
let t = (c.g[V0][0] as i64) < 0;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
if t {
break 'b_3635fc;
} else {
c.cycles += 6;
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.sw((c.g[A0][0] as u32), c.g[S2][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[V1][0] = c.lw((c.g[V0][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.sw((c.g[V0][0] as u32), c.g[V1][0]);
break 'b_3635fc;
}
}
'b_363620: {
c.cycles += 3;
c.g[V0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x6));
let t = (c.g[V0][0] as i64) < 0;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
if t {
break 'b_363620;
} else {
c.cycles += 6;
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.sw((c.g[A0][0] as u32), c.g[S2][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[V1][0] = c.lw((c.g[V0][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.sw((c.g[V0][0] as u32), c.g[V1][0]);
break 'b_363620;
}
}
c.cycles += 3;
c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
let t = c.g[A1][0] != 0u64;
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
if t {
if event_due(c) { deliver_event_here(c, 0x003635d8); } continue 'l_3635d8;
} else {
break 'b_36362c;
}
}
}
}
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x7ca4));
c.g[V1][0] = c.g[T0][0];
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
if t {
if event_due(c) { deliver_event_here(c, 0x003635c0); } continue 'l_3635c0;
} else {
break 'b_363640;
}
}
}
}
c.cycles += 12;
c.g[V0][0] = 0x4a0000;
c.g[V1][0] = 0x60000;
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4058));
c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0x7));
c.ldr(8, (c.g[A1][0] as u32));
c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(8, (c.g[SP][0] as u32));
c.g[T7][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = 0u64;
c.g[S1][0] = c.g[T7][0];
c.g[T5][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
c.g[S0][0] = c.g[T5][0];
'l_363670: loop {
'b_36368c: {
'b_363688: {
c.cycles += 2;
let t = c.g[A0][0] == 0u64;
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
if t {
break 'b_363688;
} else {
c.cycles += 4;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[T5][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = true;
c.g[A3][0] = c.lw((c.g[V0][0] as u32));
if t {
break 'b_36368c;
} else {
break 'b_363688;
}
}
}
c.cycles += 1;
c.g[A3][0] = 0u64;
break 'b_36368c;
}
'b_363708: {
c.cycles += 9;
c.g[T0][0] = c.lw((c.g[T7][0] as u32).wrapping_add(0x7ca4));
c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
c.g[T1][0] = c.g[V0][0];
c.g[T6][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[T5][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub((c.g[A3][0] as u32)));
c.g[V1][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
let t = c.g[V1][0] != 0u64;
c.sw((c.g[V0][0] as u32), 0u64);
if t {
break 'b_363708;
} else {
c.cycles += 4;
c.g[T4][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[T3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7cac));
c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[T0][0] as u32)));
'l_3636c0: loop {
'b_3636f8: {
c.cycles += 13;
c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.sw((c.g[T4][0] as u32), c.g[A2][0]);
c.g[V1][0] = sx((c.g[A2][0] as u32) << 3);
c.g[V1][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = c.lw((c.g[V1][0] as u32));
c.g[A1][0] = c.lw((c.g[T2][0] as u32));
c.g[V1][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & 0x3e0;
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[A3][0] = c.g[A2][0];
break 'b_3636f8;
} else {
c.cycles += 1;
c.g[T0][0] = c.g[A2][0];
break 'b_3636f8;
}
}
c.cycles += 4;
c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub((c.g[A3][0] as u32)));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[T0][0] as u32)));
if t {
if event_due(c) { deliver_event_here(c, 0x003636c0); } continue 'l_3636c0;
} else {
break 'b_363708;
}
}
}
}
c.cycles += 5;
c.g[V1][0] = sx((c.g[T5][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[A0][0] = c.g[T6][0];
c.g[V0][0] = ((c.g[A0][0] as i64) < 2i64) as u64;
let t = c.g[V0][0] != 0u64;
c.sw((c.g[V1][0] as u32), c.g[T0][0]);
if t {
if event_due(c) { deliver_event_here(c, 0x00363670); } continue 'l_363670;
} else {
c.cycles += 19;
c.g[V0][0] = 0x60000;
c.g[V1][0] = 0x60000;
c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = c.g[V1][0] | 0x8000;
c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A1][0] = 0xffffffffffffffff;
c.g[T2][0] = 0u64;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7ca4));
c.g[S2][0] = 0u64;
c.g[V0][0] = 0x60000;
c.sw((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V1][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
c.g[V0][0] = c.g[V0][0] | 0x9ca8;
c.sw((c.g[V1][0] as u32).wrapping_add(0x1cc4), c.g[A1][0]);
c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
c.g[A0][0] = 0u64;
'l_363768: loop {
'b_363788: {
'b_363784: {
c.cycles += 2;
let t = c.g[A0][0] == 0u64;
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
if t {
break 'b_363784;
} else {
c.cycles += 5;
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = true;
c.g[V1][0] = c.lw((c.g[V0][0] as u32));
if t {
break 'b_363788;
} else {
break 'b_363784;
}
}
}
c.cycles += 1;
c.g[V1][0] = 0u64;
break 'b_363788;
}
'b_363898: {
c.cycles += 9;
c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x58));
c.sw((c.g[SP][0] as u32).wrapping_add(0x54), c.g[V0][0]);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[S7][0] = c.lw((c.g[V0][0] as u32));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S7][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A0][0]);
if t {
break 'b_363898;
} else {
c.cycles += 5;
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x7cac));
c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[T0][0]);
c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
'l_3637c0: loop {
'b_363888: {
c.cycles += 5;
c.g[V0][0] = sx((c.g[V1][0] as u32) << 3);
c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[S1][0] = c.lw((c.g[V0][0] as u32));
let t = c.g[S1][0] == 0u64;
c.g[S6][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
if t {
break 'b_363888;
} else {
c.cycles += 3;
c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x24));
c.g[S0][0] = c.lw((c.g[S1][0] as u32));
'l_3637e0: loop {
'b_3637fc: {
c.cycles += 2;
let t = c.g[S0][0] == c.g[T2][0];
c.g[A0][0] = c.g[FP][0];
if t {
break 'b_3637fc;
} else {
c.cycles += 5;
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.g[S0][0];
c.g[RA][0] = sx(0x003637f8);
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
Callee { addr: 0x00363c20, f: ee::s12::DeferMan_EmitStateChange }.call(c, 0x003637f8)?;
c.g[T2][0] = c.g[S0][0];
break 'b_3637fc;
}
}
'b_36387c: {
c.cycles += 16;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[A3][0] = c.g[S2][0];
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
c.g[T1][0] = c.g[S5][0];
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
c.g[A2][0] = c.lhu((c.g[S4][0] as u32).wrapping_add(0x2));
c.g[A1][0] = c.lh((c.g[SP][0] as u32).wrapping_add(0x24));
c.g[A2][0] = sx((c.g[A2][0] as u32) << 16);
c.sq(10, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[RA][0] = sx(0x00363830);
c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 16) as u32);
Callee { addr: 0x00368660, f: ee::s12::TexMan_BindTextures }.call(c, 0x00363830)?;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
let t = c.g[V0][0] == 0u64;
c.lq(10, (c.g[SP][0] as u32).wrapping_add(0x60));
if t {
break 'b_36387c;
} else {
c.cycles += 16;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V1][0] = 0x50000000;
c.g[V0][0] = c.g[V0][0] << 32;
c.g[A1][0] = 0x60000000;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[A0][0] as u32), c.g[V0][0]);
c.g[A0][0] = 0x30000000;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.sd((c.g[V1][0] as u32), c.g[A1][0]);
break 'b_36387c;
}
}
c.cycles += 3;
c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
if c.g[S1][0] != 0u64 {
c.g[S0][0] = c.lw((c.g[S1][0] as u32));
if event_due(c) { deliver_event_here(c, 0x003637e0); } continue 'l_3637e0;
} else {
break 'b_363888;
}
}
}
}
c.cycles += 4;
c.g[V1][0] = c.g[S6][0];
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S7][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x44));
if t {
if event_due(c) { deliver_event_here(c, 0x003637c0); } continue 'l_3637c0;
} else {
break 'b_363898;
}
}
}
}
'b_363b00: {
'b_363af8: {
'b_36398c: {
'b_3638c0: {
c.cycles += 6;
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x54));
c.g[V1][0] = 0x6;
c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[A0][0] = c.lw((c.g[V0][0] as u32));
let t = c.g[A0][0] == c.g[V1][0];
c.g[V0][0] = 0x9;
if t {
break 'b_3638c0;
} else {
c.cycles += 2;
let t = c.g[A0][0] == c.g[V0][0];
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
if t {
break 'b_36398c;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
if t {
break 'b_363b00;
} else {
break 'b_3638c0;
}
}
}
}
'b_3638fc: {
c.cycles += 5;
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
c.g[S0][0] = 0u64;
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x59c0));
let t = (c.g[V0][0] as i64) <= 0;
c.g[S1][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1f4c));
if t {
break 'b_3638fc;
} else {
c.cycles += 1;
c.g[A1][0] = c.g[S0][0];
'l_3638d8: loop {
c.cycles += 9;
c.g[A0][0] = c.g[S3][0];
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = sx(0x003638e8);
c.g[A3][0] = c.g[S5][0];
Callee { addr: 0x0036ac00, f: ee::s12::PS2GraphicsMan_RenderDepthFog }.call(c, 0x003638e8)?;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x59c0));
c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = c.g[S0][0];
if t {
if event_due(c) { deliver_event_here(c, 0x003638d8); } continue 'l_3638d8;
} else {
break 'b_3638fc;
}
}
}
}
c.cycles += 25;
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
c.g[T1][0] = 0xffffffffffffffff;
c.g[V1][0] = 0x500000;
c.g[T2][0] = 0u64;
c.sw((c.g[T0][0] as u32).wrapping_add(0x1cc4), c.g[T1][0]);
c.g[A0][0] = c.g[FP][0];
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4c));
c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1420));
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
c.sq(10, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[RA][0] = sx(0x00363950);
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
Callee { addr: 0x00364360, f: ee::s12::DeferMan_EmitDefaultState }.call(c, 0x00363950)?;
c.lq(10, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[A0][0] = 0xffffffffffffffff;
c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[V1][0] = 0x1;
'l_363960: loop {
c.cycles += 7;
c.sw((c.g[V0][0] as u32), c.g[A0][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
let t = (c.g[V1][0] as i64) >= 0;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x00363960); } continue 'l_363960;
} else {
c.cycles += 4;
c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
c.g[V0][0] = 0xffffffffffffffff;
let t = true;
c.sw((c.g[T1][0] as u32).wrapping_add(0x1f54), c.g[V0][0]);
if t {
break 'b_363af8;
} else {
break 'b_36398c;
}
}
}
}
'b_363a34: {
c.cycles += 31;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V1][0] = 0x70000000;
c.g[A0][0] = 0x20000000;
c.g[A3][0] = 0x70000000;
c.sd((c.g[V0][0] as u32), c.g[V1][0]);
c.g[A1][0] = 0xfff0000;
c.g[A1][0] = c.g[A1][0] | 0xffff;
c.g[S0][0] = 0u64;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lw((c.g[S5][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
c.sd((c.g[V0][0] as u32), c.g[A3][0]);
c.g[V1][0] = c.lw((c.g[S5][0] as u32));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
c.sw((c.g[S5][0] as u32), c.g[V1][0]);
c.g[A0][0] = c.g[A0][0] & c.g[A1][0];
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
c.sw((c.g[T0][0] as u32), c.g[A0][0]);
c.g[V0][0] = c.lw((c.g[S5][0] as u32));
c.g[T1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
c.sw((c.g[T1][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x59c0));
let t = (c.g[V1][0] as i64) <= 0;
c.g[S1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1f4c));
if t {
break 'b_363a34;
} else {
c.cycles += 2;
c.g[A1][0] = c.g[S0][0];
'l_363a10: loop {
c.cycles += 9;
c.g[A0][0] = c.g[S3][0];
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = sx(0x00363a20);
c.g[A3][0] = c.g[S5][0];
Callee { addr: 0x0036c790, f: ee::s12::PS2GraphicsMan_RenderBloom }.call(c, 0x00363a20)?;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x59c0));
c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = c.g[S0][0];
if t {
if event_due(c) { deliver_event_here(c, 0x00363a10); } continue 'l_363a10;
} else {
break 'b_363a34;
}
}
}
}
'b_363a6c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x59c0));
let t = (c.g[V0][0] as i64) <= 0;
c.g[S0][0] = 0u64;
if t {
break 'b_363a6c;
} else {
c.cycles += 2;
c.g[A1][0] = c.g[S0][0];
'l_363a48: loop {
c.cycles += 9;
c.g[A0][0] = c.g[S3][0];
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = sx(0x00363a58);
c.g[A3][0] = c.g[S5][0];
Callee { addr: 0x003904a0, f: ee::s12::PS2GraphicsMan_RenderColorFilter }.call(c, 0x00363a58)?;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x59c0));
c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[A1][0] = c.g[S0][0];
if event_due(c) { deliver_event_here(c, 0x00363a48); } continue 'l_363a48;
} else {
break 'b_363a6c;
}
}
}
}
c.cycles += 25;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x48));
c.g[V1][0] = 0xffffffffffffffff;
c.g[A2][0] = 0x500000;
c.g[T2][0] = 0u64;
c.sw((c.g[V0][0] as u32).wrapping_add(0x1cc4), c.g[V1][0]);
c.g[A0][0] = c.g[FP][0];
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4c));
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1420));
c.ldl(7, (c.g[V0][0] as u32).wrapping_add(0x7));
c.ldr(7, (c.g[V0][0] as u32));
c.ldl(8, (c.g[V0][0] as u32).wrapping_add(0xf));
c.ldr(8, (c.g[V0][0] as u32).wrapping_add(0x8));
c.g[T1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0x7));
c.sdr(7, (c.g[A1][0] as u32));
c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0xf));
c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x8));
c.sw((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T1][0]);
c.sq(10, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[RA][0] = sx(0x00363ac0);
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
Callee { addr: 0x00364360, f: ee::s12::DeferMan_EmitDefaultState }.call(c, 0x00363ac0)?;
c.lq(10, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[A0][0] = 0xffffffffffffffff;
c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[V1][0] = 0x1;
'l_363ad0: loop {
c.cycles += 7;
c.sw((c.g[V0][0] as u32), c.g[A0][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
let t = (c.g[V1][0] as i64) >= 0;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x00363ad0); } continue 'l_363ad0;
} else {
c.cycles += 3;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
c.g[A0][0] = 0xffffffffffffffff;
c.sw((c.g[V1][0] as u32).wrapping_add(0x1f54), c.g[A0][0]);
break 'b_363af8;
}
}
}
c.cycles += 2;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[V0][0] = ((c.g[A0][0] as i64) < 3i64) as u64;
break 'b_363b00;
}
c.cycles += 2;
let t = c.g[V0][0] != 0u64;

if t {
if event_due(c) { deliver_event_here(c, 0x00363768); } continue 'l_363768;
} else {
break 'b_363b08;
}
}
}
}
}
}
'b_363b4c: {
c.cycles += 4;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x2c));
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
let t = c.g[V0][0] != 0u64;
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
if t {
break 'b_363b4c;
} else {
c.cycles += 13;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V0][0] = 0x70000000;
c.g[A0][0] = 0xfff0000;
c.sd((c.g[V1][0] as u32), c.g[V0][0]);
c.g[A0][0] = c.g[A0][0] | 0xffff;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.g[V0][0] & c.g[A0][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.sw((c.g[A1][0] as u32), c.g[A0][0]);
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
break 'b_363b4c;
}
}
'b_363b80: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[A2][0] as u32));
let t = c.g[V0][0] != 0u64;
c.g[V1][0] = c.lw((c.g[S5][0] as u32));
if t {
break 'b_363b80;
} else {
c.cycles += 10;
c.g[V0][0] = 0x20000000;
c.g[A0][0] = 0x70000000;
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[V1][0] as u32), c.g[A0][0]);
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.lw((c.g[S5][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.sw((c.g[S5][0] as u32), c.g[V0][0]);
c.sw((c.g[A2][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S5][0] as u32));
break 'b_363b80;
}
}
c.cycles += 39;
c.g[V0][0] = 0x20000000;
c.g[A2][0] = 0x70000000;
c.g[A0][0] = 0xfff0000;
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[A0][0] = c.g[A0][0] | 0xffff;
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.sd((c.g[V1][0] as u32), c.g[A2][0]);
c.g[V0][0] = c.lw((c.g[S5][0] as u32));
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.sw((c.g[S5][0] as u32), c.g[V0][0]);
c.sd((c.g[A1][0] as u32), c.g[A2][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.g[V0][0] & c.g[A0][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.sw((c.g[A3][0] as u32), c.g[A0][0]);
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3c));
c.g[RA][0] = sx(0x00363be0);
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1f60));
Callee { addr: 0x00365fa8, f: ee::s12::VramHeap_NodesFromScratchpad }.call(c, 0x00363be0)?;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
c.g[RA][0] = sx(0x00363bec);
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x38));
Callee { addr: 0x0038f300, f: ee::s12::SprStream_SetMode }.call(c, 0x00363bec)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
c.pc = t; return Ok(());
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fakes, Rng, DATA, GAME_GP, SP as OC1473_SP0};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const OC1473_CTRL: u32 = DATA + 0x7_f800;
    const OC1473_MAGIC: u32 = 0x1473_c0de;
    const OC1473_SELF: u32 = DATA;
    const OC1473_GS: u32 = DATA + 0x7_0000;
    const OC1473_TEXMAN: u32 = DATA + 0x7_8000;
    const OC1473_STATES: u32 = DATA + 0x7_c000;
    const OC1473_NODES: u32 = DATA + 0x7_d000;
    const OC1473_TAILS: u32 = DATA + 0x7_e000;
    const OC1473_CELLS: u32 = DATA + 0x7_f000;
    const OC1473_PKT: u32 = DATA + 0x8_0000;

    /// Gated fake of a callee slot: logs the call (all registers) like
    /// `fakes::fake`, only when this test's gate word is set.
    macro_rules! oc1473_fake {
        ($name:ident, $prev:ident, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC1473_CTRL) != OC1473_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                let target = c.pc;
                let ret = c.g[RA][0] as u32;
                fakes::fake(c, target, ret);
            }
        };
    }

    oc1473_fake!(oc1473_f_build, OC1473_P_BUILD, ee::s12::f_00362de8__generated);
    oc1473_fake!(oc1473_f_default, OC1473_P_DEFAULT, ee::s12::f_00364360__generated);
    oc1473_fake!(oc1473_f_sort, OC1473_P_SORT, ee::s12::f_00364050__generated);
    oc1473_fake!(oc1473_f_reset, OC1473_P_RESET, ee::s12::f_003666f8__generated);
    oc1473_fake!(oc1473_f_mode, OC1473_P_MODE, ee::s12::f_0038f300__generated);
    oc1473_fake!(oc1473_f_to_spr, OC1473_P_TO_SPR, ee::s12::f_00365f68__generated);
    oc1473_fake!(oc1473_f_state, OC1473_P_STATE, ee::s12::f_00363c20__generated);
    oc1473_fake!(oc1473_f_bind, OC1473_P_BIND, ee::s12::f_00368660__generated);
    oc1473_fake!(oc1473_f_fog, OC1473_P_FOG, ee::s12::f_0036ac00__generated);
    oc1473_fake!(oc1473_f_bloom, OC1473_P_BLOOM, ee::s12::f_0036c790__generated);
    oc1473_fake!(oc1473_f_filter, OC1473_P_FILTER, ee::s12::f_003904a0__generated);
    oc1473_fake!(oc1473_f_from_spr, OC1473_P_FROM_SPR, ee::s12::f_00365fa8__generated);
    oc1473_fake!(oc1473_f_invalidate, OC1473_P_INV, ee::s13::f_00396128__generated);

    fn oc1473_install() {
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_00362DE8), oc1473_f_build, &OC1473_P_BUILD);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_00364360), oc1473_f_default, &OC1473_P_DEFAULT);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_00364050), oc1473_f_sort, &OC1473_P_SORT);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_003666F8), oc1473_f_reset, &OC1473_P_RESET);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_0038F300), oc1473_f_mode, &OC1473_P_MODE);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_00365F68), oc1473_f_to_spr, &OC1473_P_TO_SPR);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_00363C20), oc1473_f_state, &OC1473_P_STATE);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_00368660), oc1473_f_bind, &OC1473_P_BIND);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_0036AC00), oc1473_f_fog, &OC1473_P_FOG);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_0036C790), oc1473_f_bloom, &OC1473_P_BLOOM);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_003904A0), oc1473_f_filter, &OC1473_P_FILTER);
        fakes::hook(addr_of_mut!(ee::s12::OVERRIDE_00365FA8), oc1473_f_from_spr, &OC1473_P_FROM_SPR);
        fakes::hook(addr_of_mut!(ee::s13::OVERRIDE_00396128), oc1473_f_invalidate, &OC1473_P_INV);
    }

    fn oc1473_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// 0..5 buckets of 0..3 nodes each (some empty), nodes sharing a few
    /// states and texture keys (with and without a packet), pass types from a
    /// table that includes the fog (6) and bloom/filter (9) passes, 0..2
    /// views; fakes for every callee, unwinding at call `calls` (0: never).
    fn oc1473_setup(c: &mut Cpu, seed: u32, calls: u32) {
        oc1473_install();
        let mut r: Rng = fakes::machine(c, seed, 0x1473, calls);
        c.pc = 0x0036_3490;
        oc1473_w(c, OC1473_CTRL, OC1473_MAGIC);
        let gp = GAME_GP;
        oc1473_w(c, gp + 0x2a90, OC1473_GS);
        oc1473_w(c, gp + 0x1384, OC1473_CELLS + 0x100);
        let ring = r.next_u32();
        oc1473_w(c, OC1473_CELLS + 0x100, ring);
        oc1473_w(c, OC1473_GS + 0x18f4, OC1473_TEXMAN);
        oc1473_w(c, OC1473_GS + 0x59c0, (seed / 3) % 3);
        let n = seed % 5;
        oc1473_w(c, OC1473_SELF + 0x6_7ca4, n);
        let pa = [3u32, 6, 6, 9, 12, 0][(r.next_u32() % 6) as usize];
        let pb = [6u32, 9, 9, 3, 31, 9][(r.next_u32() % 6) as usize];
        let pc = [0u32, 6, 9, 5][(r.next_u32() % 4) as usize];
        oc1473_w(c, 0x004a_4058, pa);
        oc1473_w(c, 0x004a_405c, pb);
        oc1473_w(c, OC1473_SP0 - 0x110 + 8, pc);
        for s in 0..4u32 {
            let bits = (r.next_u32() % 32) << 5 | (r.next_u32() & !0x3e0);
            oc1473_w(c, OC1473_STATES + 0x40 * s + 8, bits);
        }
        let tex = |r: &mut Rng| -> u32 {
            if r.next_u32() % 4 == 0 {
                0xffff
            } else {
                r.next_u32() % 16
            }
        };
        let mut k = 0u32;
        for i in 0..n {
            let len = if r.next_u32() % 6 == 0 { 0 } else { 1 + r.next_u32() % 3 };
            let head = if len == 0 { 0 } else { OC1473_NODES + 0x40 * k };
            oc1473_w(c, OC1473_SELF + 0x6_7cac + 8 * i, head);
            for j in 0..len {
                let node = OC1473_NODES + 0x40 * k;
                let state = OC1473_STATES + 0x40 * (r.next_u32() % 4);
                oc1473_w(c, node, state);
                let key = tex(&mut r) | tex(&mut r) << 16;
                oc1473_w(c, node + 4, key);
                oc1473_w(c, node + 8, r.next_u32());
                let packet = if r.next_u32() % 2 == 0 { 0 } else { r.next_u32() | 1 };
                oc1473_w(c, node + 0xc, packet);
                oc1473_w(c, node + 0x10, OC1473_TAILS + 0x10 * k);
                let link = if j + 1 < len { OC1473_NODES + 0x40 * (k + 1) } else { 0 };
                oc1473_w(c, node + 0x18, link);
                k += 1;
            }
        }
        oc1473_w(c, OC1473_CELLS, OC1473_PKT + 0x10 * (r.next_u32() % 4));
        c.g[A0][0] = sx(OC1473_SELF);
        c.g[A1][0] = sx(OC1473_CELLS);
        c.g[A2][0] = sx(OC1473_CELLS + 0x10);
        c.g[A3][0] = sx(OC1473_CELLS + 0x30);
        c.g[T0][0] = sx(OC1473_CELLS + 0x20);
    }

    /// `PS2GraphicsMan_RenderFrame` against the generated body, fakes
    /// unwinding after any call.
    #[test]
    fn ps2_graphics_man_render_frame_matches() {
        fakes::covered(
            ee::s12::f_00363490__generated,
            ps2_graphics_man_render_frame,
            300,
            &|c, seed| oc1473_setup(c, seed, 40),
        );
    }

    /// Every call arm of the body is reached (return addresses of the calls).
    #[test]
    fn ps2_graphics_man_render_frame_reaches_every_call() {
        fakes::check_reaching(
            ee::s12::f_00363490__generated,
            ps2_graphics_man_render_frame,
            300,
            &[
                0x0036_34d4, 0x0036_3540, 0x0036_3548, 0x0036_3550, 0x0036_356c,
                0x0036_3580, 0x0036_358c, 0x0036_37f8, 0x0036_3830, 0x0036_38e8,
                0x0036_3950, 0x0036_3a20, 0x0036_3a58, 0x0036_3ac0, 0x0036_3be0,
                0x0036_3bec,
            ],
            &|c, seed| oc1473_setup(c, seed, 0),
        );
    }

    /// The seeds make many different paths (distinct cycle totals).
    #[test]
    fn ps2_graphics_man_render_frame_path_count() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..300 {
            let mut c = crate::difftest::fresh();
            oc1473_setup(&mut c, seed, 0);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s12::f_00363490__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 30, "only {} distinct paths", seen.len());
    }
}
