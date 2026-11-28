//! Staging module oc1474 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1474`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `DeferMan_BuildBuckets` (0x00362de8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn deferman_build_buckets(c: &mut Cpu) {
    if c.pc != 0x00362de8 {
        return ee::s12::f_00362de8__generated(c);
    }
    guest(c, |c| {
'b_362e90: {
c.cycles += 42;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
c.g[V0][0] = 0x60000;
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S6][0] = c.g[A0][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x80));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[A0][0] = c.g[S0][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x7ca0), 0u64);
c.g[A1][0] = c.lw((c.g[S6][0] as u32));
c.g[A1][0] = sx((c.g[A1][0] as u32) << 7);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S6][0] as u32)));
c.g[RA][0] = sx(0x00362e40);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7f));
call_ptr(c, 0x00424698, 0x00362e40)?;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[A1][0] = c.g[S0][0];
c.g[A3][0] = 0x4;
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[S6][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 7);
c.div(0, 3, 2);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 7);
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.g[A2][0] = c.g[V1][0];
c.g[V0][0] = c.lo[0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
if c.g[V0][0] == 0u64 {
break 'b_362e90;
} else {
break 'b_362e90;
}
}
'b_363190: {
c.cycles += 7;
c.g[RA][0] = sx(0x00362e98);
Callee { addr: 0x0038f4f8, f: ee::s12::SprStream_BeginBlockUpload }.call(c, 0x00362e98)?;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
c.g[S0][0] = c.g[V0][0];
c.g[A1][0] = 0u64;
let t = (c.g[V1][0] as i64) <= 0;
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
if t {
break 'b_363190;
} else {
c.cycles += 1;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
'l_362eb0: loop {
'b_363168: {
c.cycles += 13;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[A1][0]);
c.g[A3][0] = 0x4;
c.mult(0, 5, 5, 2);
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
c.g[A1][0] = sx((c.g[A1][0] as u32) << 7);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x80));
c.g[RA][0] = sx(0x00362ed8);
c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A1][0] as u32)));
Callee { addr: 0x0038f4f8, f: ee::s12::SprStream_BeginBlockUpload }.call(c, 0x00362ed8)?;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
let t = c.g[V1][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
if t {
break 'b_363168;
} else {
c.cycles += 7;
c.g[V0][0] = 0x50000;
c.g[V1][0] = 0x60000;
c.g[S7][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
c.g[FP][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[S1][0] = c.g[S0][0];
'l_362f00: loop {
'b_363014: {
'b_362f98: {
c.cycles += 36;
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S6][0];
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
c.g[A1][0] = c.g[S1][0];
c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] ^ c.g[A2][0];
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] ^ c.g[A3][0];
c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 8) as u32);
c.g[T0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
c.g[A3][0] = sx((c.g[A3][0] as u32) << 5);
c.g[T0][0] = sx((c.g[T0][0] as u32) << 5);
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A2][0] = sx((c.g[V0][0] as u32) << 5);
c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A3][0] = sx(((c.g[A3][0] as u32) as i32 >> 8) as u32);
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
c.g[A2][0] = c.g[A2][0] ^ c.g[A3][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[T0][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
c.g[A2][0] = c.g[A2][0] ^ c.g[T0][0];
c.g[A2][0] = c.g[A2][0] ^ c.g[V1][0];
c.g[RA][0] = sx(0x00362f70);
c.g[A2][0] = c.g[A2][0] & 0xff;
call_ptr(c, 0x00394ed0, 0x00362f70)?;
c.g[S3][0] = c.g[V0][0];
c.g[V1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x12));
c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x10));
c.sh((c.g[SP][0] as u32).wrapping_add(0x2), c.g[V1][0]);
c.sh((c.g[SP][0] as u32), c.g[V0][0]);
c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x1e));
let t = c.g[V0][0] == 0u64;
c.g[S2][0] = c.lw((c.g[SP][0] as u32));
if t {
break 'b_362f98;
} else {
c.cycles += 2;
let t = true;
c.g[S5][0] = 0u64;
if t {
break 'b_363014;
} else {
break 'b_362f98;
}
}
}
c.cycles += 31;
c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> 16) as u32);
c.g[V1][0] = sx((c.g[S2][0] as u32) << 16);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
c.g[T0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x1c));
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[A0][0] = c.g[S6][0];
c.g[V1][0] = c.g[S3][0] ^ c.g[V1][0];
c.g[A1][0] = c.g[S3][0];
c.g[A2][0] = sx(((c.g[V1][0] as u32) as i32 >> 8) as u32);
c.g[T1][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
c.g[A2][0] = sx((c.g[A2][0] as u32) << 5);
c.g[T1][0] = sx((c.g[T1][0] as u32) << 5);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A3][0] = sx((c.g[V1][0] as u32) << 5);
c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 8) as u32);
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
c.g[A3][0] = c.g[A3][0] ^ c.g[A2][0];
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[T1][0] = sx(((c.g[T1][0] as u32) as i32 >> 16) as u32);
c.g[A3][0] = c.g[A3][0] ^ c.g[T1][0];
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
c.g[A3][0] = c.g[A3][0] ^ c.g[V0][0];
c.g[A2][0] = c.g[S2][0];
c.g[RA][0] = sx(0x00363010);
c.g[A3][0] = c.g[A3][0] & 0xff;
call_ptr(c, 0x00395000, 0x00363010)?;
c.g[S5][0] = c.g[V0][0];
break 'b_363014;
}
'b_36312c: {
c.cycles += 2;
if c.g[S5][0] != 0u64 {
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
break 'b_36312c;
} else {
'b_363128: {
'b_363078: {
c.cycles += 18;
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x7494));
c.g[V1][0] = 0x50000;
c.g[V1][0] = c.g[V1][0] | 0x7498;
c.g[A1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = sx((c.g[A0][0] as u32) << 5);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sw((c.g[FP][0] as u32).wrapping_add(0x7494), c.g[A0][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
c.sw((c.g[S0][0] as u32), c.g[S3][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S2][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x1e));
let t = c.g[V0][0] == 0u64;
c.g[S5][0] = c.g[S0][0];
if t {
break 'b_363078;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x7ca0));
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = 0x1;
if t {
break 'b_363078;
} else {
c.cycles += 2;
let t = true;
c.sw((c.g[S5][0] as u32).wrapping_add(0x14), c.g[A0][0]);
if t {
break 'b_363128;
} else {
break 'b_363078;
}
}
}
}
c.cycles += 44;
c.g[A0][0] = c.g[S3][0];
c.g[RA][0] = sx(0x00363084);
c.g[A1][0] = c.g[S2][0];
Callee { addr: 0x00364240, f: ee::s12::DeferMan_ComputeSortKey }.call(c, 0x00363084)?;
c.g[V1][0] = sx(((c.g[S2][0] as u32) as i32 >> 16) as u32);
c.g[A0][0] = sx((c.g[S2][0] as u32) << 16);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 16) as u32);
c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
c.g[T0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x78a0));
c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
c.g[A0][0] = c.g[S3][0] ^ c.g[A0][0];
c.g[A2][0] = sx(((c.g[A0][0] as u32) as i32 >> 8) as u32);
c.g[A3][0] = sx(((c.g[A0][0] as u32) as i32 >> 16) as u32);
c.g[A2][0] = sx((c.g[A2][0] as u32) << 5);
c.g[V1][0] = sx((c.g[A0][0] as u32) << 5);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[A1][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
c.g[A3][0] = sx((c.g[A3][0] as u32) << 5);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 8) as u32);
c.g[A1][0] = sx((c.g[A1][0] as u32) << 5);
c.g[V1][0] = c.g[V1][0] ^ c.g[A2][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[A3][0] = sx(((c.g[A3][0] as u32) as i32 >> 16) as u32);
c.g[V1][0] = c.g[V1][0] ^ c.g[A3][0];
c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
c.g[V1][0] = c.g[V1][0] ^ c.g[A1][0];
c.g[V1][0] = c.g[V1][0] & 0xff;
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = c.lw((c.g[T0][0] as u32));
c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sw((c.g[T0][0] as u32), c.g[S0][0]);
c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x7ca4));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.sw((c.g[V1][0] as u32).wrapping_add(0x7ca8), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x7ca4));
c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.sw((c.g[V0][0] as u32).wrapping_add(0x7cac), c.g[S0][0]);
c.sw((c.g[S4][0] as u32).wrapping_add(0x7ca4), c.g[A0][0]);
break 'b_363128;
}
c.cycles += 1;
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
break 'b_36312c;
}
}
'b_36315c: {
c.cycles += 6;
c.g[A0][0] = c.g[S5][0];
c.g[RA][0] = sx(0x00363138);
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
Callee { addr: 0x00362978, f: ee::s12::DeferMan_AppendPacket }.call(c, 0x00363138)?;
c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x1e));
if c.g[V0][0] == 0u64 {
c.sw((c.g[S4][0] as u32).wrapping_add(0x7ca0), c.g[S5][0]);
break 'b_36315c;
} else {
'b_363158: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x7ca0));
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x1;
if t {
break 'b_363158;
} else {
c.cycles += 2;
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[S5][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
break 'b_363158;
}
}
c.cycles += 1;
c.sw((c.g[S4][0] as u32).wrapping_add(0x7ca0), c.g[S5][0]);
break 'b_36315c;
}
}
c.cycles += 3;
c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_sub(0x1));
let t = c.g[S7][0] != 0u64;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x80));
if t {
if event_due(c) { deliver_event_here(c, 0x00362f00); } continue 'l_362f00;
} else {
break 'b_363168;
}
}
}
}
c.cycles += 10;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
c.g[RA][0] = sx(0x00363174);
c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
Callee { addr: 0x0038f6a8, f: ee::s12::SprStream_NextIndex }.call(c, 0x00363174)?;
c.g[RA][0] = sx(0x0036317c);
Callee { addr: 0x0038f7b0, f: ee::s12::Dma_WaitToScratchpad }.call(c, 0x0036317c)?;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
if t {
if event_due(c) { deliver_event_here(c, 0x00362eb0); } continue 'l_362eb0;
} else {
break 'b_363190;
}
}
}
}
'b_363454: {
c.cycles += 9;
c.g[RA][0] = sx(0x00363198);
c.g[S7][0] = 0u64;
Callee { addr: 0x0038f7b0, f: ee::s12::Dma_WaitToScratchpad }.call(c, 0x00363198)?;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.mult(0, 3, 2, 4);
c.g[V0][0] = c.lw((c.g[S6][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
let t = (c.g[V0][0] as i64) <= 0;
c.g[V0][0] = 0x50000;
if t {
break 'b_363454;
} else {
c.cycles += 7;
c.g[V1][0] = 0x60000;
c.g[FP][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[S1][0] = c.g[S0][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
'l_3631d0: loop {
'b_3632e8: {
'b_36326c: {
c.cycles += 37;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[S6][0];
c.g[A3][0] = c.lw((c.g[S1][0] as u32));
c.g[A1][0] = c.g[S1][0];
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
c.g[A3][0] = c.g[A3][0] ^ c.g[V1][0];
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
c.g[A3][0] = c.g[A3][0] ^ c.g[A2][0];
c.g[A3][0] = c.g[A3][0] ^ c.g[V0][0];
c.g[V1][0] = sx(((c.g[A3][0] as u32) as i32 >> 8) as u32);
c.g[T0][0] = sx(((c.g[A3][0] as u32) as i32 >> 16) as u32);
c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
c.g[T0][0] = sx((c.g[T0][0] as u32) << 5);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[A2][0] = sx((c.g[A3][0] as u32) << 5);
c.g[V0][0] = sx(((c.g[A3][0] as u32) as i32 >> 24) as u32);
c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 8) as u32);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
c.g[A2][0] = c.g[A2][0] ^ c.g[V1][0];
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[T0][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
c.g[A2][0] = c.g[A2][0] ^ c.g[T0][0];
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
c.g[A2][0] = c.g[A2][0] ^ c.g[V0][0];
c.g[RA][0] = sx(0x00363240);
c.g[A2][0] = c.g[A2][0] & 0xff;
call_ptr(c, 0x00394ed0, 0x00363240)?;
c.g[S3][0] = c.g[V0][0];
c.g[V1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x12));
c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.sh((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.sh((c.g[A0][0] as u32).wrapping_add(0x2), c.g[V1][0]);
c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x1e));
let t = c.g[V0][0] == 0u64;
c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
if t {
break 'b_36326c;
} else {
c.cycles += 2;
let t = true;
c.g[S5][0] = 0u64;
if t {
break 'b_3632e8;
} else {
break 'b_36326c;
}
}
}
c.cycles += 31;
c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> 16) as u32);
c.g[A2][0] = sx((c.g[S2][0] as u32) << 16);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 16) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
c.g[T0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x1c));
c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
c.g[A0][0] = c.g[S6][0];
c.g[A2][0] = c.g[S3][0] ^ c.g[A2][0];
c.g[A1][0] = c.g[S3][0];
c.g[V1][0] = sx(((c.g[A2][0] as u32) as i32 >> 8) as u32);
c.g[T1][0] = sx(((c.g[A2][0] as u32) as i32 >> 16) as u32);
c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
c.g[A3][0] = sx((c.g[A2][0] as u32) << 5);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V0][0] = sx(((c.g[A2][0] as u32) as i32 >> 24) as u32);
c.g[T1][0] = sx((c.g[T1][0] as u32) << 5);
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 8) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
c.g[A3][0] = c.g[A3][0] ^ c.g[V1][0];
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[T1][0] = sx(((c.g[T1][0] as u32) as i32 >> 16) as u32);
c.g[A3][0] = c.g[A3][0] ^ c.g[T1][0];
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
c.g[A3][0] = c.g[A3][0] ^ c.g[V0][0];
c.g[A2][0] = c.g[S2][0];
c.g[RA][0] = sx(0x003632e4);
c.g[A3][0] = c.g[A3][0] & 0xff;
call_ptr(c, 0x00395000, 0x003632e4)?;
c.g[S5][0] = c.g[V0][0];
break 'b_3632e8;
}
'b_363400: {
c.cycles += 2;
if c.g[S5][0] != 0u64 {
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
break 'b_363400;
} else {
'b_3633fc: {
'b_36334c: {
c.cycles += 18;
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x7494));
c.g[V1][0] = 0x50000;
c.g[V1][0] = c.g[V1][0] | 0x7498;
c.g[A1][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = sx((c.g[A0][0] as u32) << 5);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sw((c.g[FP][0] as u32).wrapping_add(0x7494), c.g[A0][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
c.sw((c.g[S0][0] as u32), c.g[S3][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[S2][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x1e));
let t = c.g[V0][0] == 0u64;
c.g[S5][0] = c.g[S0][0];
if t {
break 'b_36334c;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x7ca0));
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x1;
if t {
break 'b_36334c;
} else {
c.cycles += 2;
let t = true;
c.sw((c.g[S5][0] as u32).wrapping_add(0x14), c.g[V0][0]);
if t {
break 'b_3633fc;
} else {
break 'b_36334c;
}
}
}
}
c.cycles += 44;
c.g[A0][0] = c.g[S3][0];
c.g[RA][0] = sx(0x00363358);
c.g[A1][0] = c.g[S2][0];
Callee { addr: 0x00364240, f: ee::s12::DeferMan_ComputeSortKey }.call(c, 0x00363358)?;
c.g[V1][0] = sx(((c.g[S2][0] as u32) as i32 >> 16) as u32);
c.g[A1][0] = sx((c.g[S2][0] as u32) << 16);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 16) as u32);
c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
c.g[T0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x78a0));
c.g[A1][0] = c.g[A1][0] | c.g[V1][0];
c.g[A1][0] = c.g[S3][0] ^ c.g[A1][0];
c.g[A2][0] = sx(((c.g[A1][0] as u32) as i32 >> 8) as u32);
c.g[A3][0] = sx(((c.g[A1][0] as u32) as i32 >> 16) as u32);
c.g[A2][0] = sx((c.g[A2][0] as u32) << 5);
c.g[A3][0] = sx((c.g[A3][0] as u32) << 5);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[V1][0] = sx((c.g[A1][0] as u32) << 5);
c.g[A0][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 8) as u32);
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
c.g[V1][0] = c.g[V1][0] ^ c.g[A2][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A3][0] = sx(((c.g[A3][0] as u32) as i32 >> 16) as u32);
c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
c.g[V1][0] = c.g[V1][0] ^ c.g[A3][0];
c.g[V1][0] = c.g[V1][0] ^ c.g[A0][0];
c.g[V1][0] = c.g[V1][0] & 0xff;
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = c.lw((c.g[T0][0] as u32));
c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sw((c.g[T0][0] as u32), c.g[S0][0]);
c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x7ca4));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.sw((c.g[V1][0] as u32).wrapping_add(0x7ca8), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x7ca4));
c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.sw((c.g[V0][0] as u32).wrapping_add(0x7cac), c.g[S0][0]);
c.sw((c.g[S4][0] as u32).wrapping_add(0x7ca4), c.g[A0][0]);
break 'b_3633fc;
}
c.cycles += 1;
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
break 'b_363400;
}
}
'b_363430: {
c.cycles += 6;
c.g[A0][0] = c.g[S5][0];
c.g[RA][0] = sx(0x0036340c);
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
Callee { addr: 0x00362978, f: ee::s12::DeferMan_AppendPacket }.call(c, 0x0036340c)?;
c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x1e));
if c.g[V0][0] == 0u64 {
c.sw((c.g[S4][0] as u32).wrapping_add(0x7ca0), c.g[S5][0]);
break 'b_363430;
} else {
'b_36342c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x7ca0));
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = 0x1;
if t {
break 'b_36342c;
} else {
c.cycles += 2;
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[S5][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
break 'b_36342c;
}
}
c.cycles += 1;
c.sw((c.g[S4][0] as u32).wrapping_add(0x7ca0), c.g[S5][0]);
break 'b_363430;
}
}
c.cycles += 9;
c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x1));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
c.mult(0, 3, 4, 2);
c.g[V0][0] = c.lw((c.g[S6][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x80));
if t {
if event_due(c) { deliver_event_here(c, 0x003631d0); } continue 'l_3631d0;
} else {
break 'b_363454;
}
}
}
}
c.cycles += 14;
c.g[RA][0] = sx(0x0036345c);
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
Callee { addr: 0x0038f6a8, f: ee::s12::SprStream_NextIndex }.call(c, 0x0036345c)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
c.pc = t; return Ok(());
    });
}


/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0036_2de8,
    deferman_build_buckets,
    "DeferMan_BuildBuckets",
)];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, check_reaching};
    use crate::difftest::{fill, Rng, DATA, GAME_GP, SP as STACK_TOP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the callee fakes (they answer only while it holds the magic).
    const OC1474_CTRL: u32 = DATA + 0x000e_0000;
    const OC1474_MAGIC: u32 = 0x4244_4d31;
    /// FindBucket results, one word per call index.
    const OC1474_FIND1: u32 = DATA + 0x000e_0100;
    /// The defer manager (`count` at +0, records at +0x80, batch state at +0x60000).
    const OC1474_MAN: u32 = DATA;
    /// Scratchpad stream ring (word at +0x10 sizes the chunks) and the buffer it hands out.
    const OC1474_RING: u32 = DATA + 0x0008_0000;
    const OC1474_BUF: u32 = DATA + 0x0007_0000;

    static OC1474_PREV_SYNC: AtomicUsize = AtomicUsize::new(0);
    static OC1474_PREV_BEGIN: AtomicUsize = AtomicUsize::new(0);
    static OC1474_PREV_NEXT: AtomicUsize = AtomicUsize::new(0);
    static OC1474_PREV_WAIT: AtomicUsize = AtomicUsize::new(0);
    static OC1474_PREV_INTERN: AtomicUsize = AtomicUsize::new(0);
    static OC1474_PREV_FIND1: AtomicUsize = AtomicUsize::new(0);
    static OC1474_PREV_SORT: AtomicUsize = AtomicUsize::new(0);
    static OC1474_PREV_APPEND: AtomicUsize = AtomicUsize::new(0);

    fn oc1474_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC1474_CTRL) == OC1474_MAGIC
    }

    fn oc1474_fake_sync(c: &mut Cpu) {
        if !oc1474_gated(c) {
            return fakes::fall_back(c, &OC1474_PREV_SYNC, ee::s15::f_00424698__generated);
        }
        fakes::fake_call(c, 0x0042_4698);
    }

    /// `SprStream_BeginBlockUpload`: answers the fixed record buffer.
    fn oc1474_fake_begin(c: &mut Cpu) {
        if !oc1474_gated(c) {
            return fakes::fall_back(c, &OC1474_PREV_BEGIN, ee::s12::f_0038f4f8__generated);
        }
        fakes::fake_call(c, 0x0038_f4f8);
        c.g[V0][0] = sx(OC1474_BUF);
    }

    fn oc1474_fake_next(c: &mut Cpu) {
        if !oc1474_gated(c) {
            return fakes::fall_back(c, &OC1474_PREV_NEXT, ee::s12::f_0038f6a8__generated);
        }
        fakes::fake_call(c, 0x0038_f6a8);
    }

    fn oc1474_fake_wait(c: &mut Cpu) {
        if !oc1474_gated(c) {
            return fakes::fall_back(c, &OC1474_PREV_WAIT, ee::s12::f_0038f7b0__generated);
        }
        fakes::fake_call(c, 0x0038_f7b0);
    }

    fn oc1474_fake_intern(c: &mut Cpu) {
        if !oc1474_gated(c) {
            return fakes::fall_back(c, &OC1474_PREV_INTERN, ee::s13::f_00394ed0__generated);
        }
        fakes::fake_call(c, 0x0039_4ed0);
    }

    /// `DeferMan_FindBucket`: the seeded result, 0 (no bucket yet) or a node pointer.
    fn oc1474_fake_find1(c: &mut Cpu) {
        if !oc1474_gated(c) {
            return fakes::fall_back(c, &OC1474_PREV_FIND1, ee::s13::f_00395000__generated);
        }
        let n = c.bus.read32(fakes::LOG_COUNT);
        let result = c.bus.read32(OC1474_FIND1 + 4 * (n % 64));
        fakes::fake_call(c, 0x0039_5000);
        c.g[V0][0] = result as u64;
    }

    fn oc1474_fake_sort(c: &mut Cpu) {
        if !oc1474_gated(c) {
            return fakes::fall_back(c, &OC1474_PREV_SORT, ee::s12::f_00364240__generated);
        }
        fakes::fake_call(c, 0x0036_4240);
    }

    fn oc1474_fake_append(c: &mut Cpu) {
        if !oc1474_gated(c) {
            return fakes::fall_back(c, &OC1474_PREV_APPEND, ee::s12::f_00362978__generated);
        }
        fakes::fake_call(c, 0x0036_2978);
    }

    fn oc1474_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00424698),
            oc1474_fake_sync,
            &OC1474_PREV_SYNC,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038F4F8),
            oc1474_fake_begin,
            &OC1474_PREV_BEGIN,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038F6A8),
            oc1474_fake_next,
            &OC1474_PREV_NEXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038F7B0),
            oc1474_fake_wait,
            &OC1474_PREV_WAIT,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00394ED0),
            oc1474_fake_intern,
            &OC1474_PREV_INTERN,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00395000),
            oc1474_fake_find1,
            &OC1474_PREV_FIND1,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00364240),
            oc1474_fake_sort,
            &OC1474_PREV_SORT,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_00362978),
            oc1474_fake_append,
            &OC1474_PREV_APPEND,
        );
    }

    /// `DeferMan_BuildBuckets`: files the frame's 0x80-byte draw records into
    /// buckets, streaming them through the scratchpad ring in chunks (a
    /// chunk loop and a tail loop). Random chunk sizes (including 0, which
    /// divides by zero), record counts with and without a tail, records with
    /// and without a continuation flag, `FindBucket` answering "none" and a
    /// node, and an unwind in any callee.
    #[test]
    fn deferman_build_buckets_matches() {
        oc1474_install_fakes();
        check_reaching(
            ee::s12::f_00362de8__generated,
            deferman_build_buckets,
            400,
            &[
                0x0036_2e40,
                0x0036_2e98,
                0x0036_2ed8,
                0x0036_2f70,
                0x0036_3010,
                0x0036_3084,
                0x0036_3138,
                0x0036_3174,
                0x0036_317c,
                0x0036_3198,
                0x0036_3240,
                0x0036_32e4,
                0x0036_3358,
                0x0036_340c,
                0x0036_345c,
            ],
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1474_0000, 30);
                c.pc = 0x0036_2de8;
                c.g[SP][0] = sx(STACK_TOP);
                c.bus.write32(OC1474_CTRL, OC1474_MAGIC);
                c.g[A0][0] = sx(OC1474_MAN);
                // Ring: chunk = word(+0x10) / 8 records (0 now and then).
                c.bus.write32(GAME_GP + 0x1384, OC1474_RING);
                let chunk = if seed % 13 == 0 { 0 } else { 1 + r.next_u32() % 6 };
                c.bus
                    .write32(OC1474_RING + 0x10, chunk * 8 + r.next_u32() % 8);
                let count = r.next_u32() % 22;
                c.bus.write32(OC1474_MAN, count);
                // The record buffer the stream hands out: random states and
                // textures, continuation flag (+0x1e) set in about 40%.
                fill(c, &mut r, OC1474_BUF, 0x80 * 8 / 4);
                for k in 0..8u32 {
                    let cont = if r.next_u32() % 5 < 2 {
                        1 + (r.next_u32() & 0xff) as u16
                    } else {
                        0
                    };
                    c.bus.write16(OC1474_BUF + 0x80 * k + 0x1e, cont);
                }
                // FindBucket: no bucket (0) or an existing node.
                for n in 0..64u32 {
                    let v = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        OC1474_MAN + 0x5_7498 + 0x20 * (r.next_u32() % 8)
                    };
                    c.bus.write32(OC1474_FIND1 + 4 * n, v);
                }
            },
        );
    }
}
