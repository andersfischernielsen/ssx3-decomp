//! Staging module oc1152 (wave wave273): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1152`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Efu, Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_96f0, snd_dec_utk_decode_stream, "SND_DecUtk_DecodeStream"),
    (0x003c_a0b8, snd_dec_eaxa_decode_loop, "SND_DecEaxa_DecodeLoop"),
    (0x003c_e7c0, snd_fir_design, "SND_FirDesign"),
    (0x003c_ebe8, snd_fir_process, "SND_FirProcess"),
];

/// `int SND_DecUtk_DecodeStream(UtkDec* self, int count, ?, short* out)` (0x003c96f0):
/// decodes `count` samples of a UTK stream into `*out`, pulling more input
/// blocks (`SNDSTRM_PopBlock`) and restarting the decoder state
/// (`SND_UtkGetState`/`SetState`/`SetMode`/`SetInput`) when the decoder runs
/// dry; pads the rest with `memset` on end of stream. Returns the number of
/// samples decoded (v0 = s2). Out pointer `a3` lives at `sp+0x20` meanwhile.
pub fn snd_dec_utk_decode_stream(c: &mut Cpu) {
    guest(c, |c| {
'b_3c9730: {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S1][0] = c.g[A1][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S2][0] = 0u64;
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
let t = c.g[A2][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A3][0]);
if t {
break 'b_3c9730;
} else {
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
c.g[RA][0] = sx(0x003c972c);
c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x38));
call_ptr(c, 0x003c7010, 0x003c972c)?;
c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0u64);
break 'b_3c9730;
}
}
'b_3c9918: {
c.cycles += 2;
let t = (c.g[S1][0] as i64) <= 0;
c.g[S3][0] = 0x1;
if t {
break 'b_3c9918;
} else {
c.cycles += 2;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
'l_3c9740: loop {
'b_3c9910: {
'b_3c990c: {
c.cycles += 17;
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = sx(0x003c974c);
c.g[A2][0] = c.g[S1][0];
Callee { addr: 0x003cde68, f: ee::s14::SND_UtkDecode }.call(c, 0x003c974c)?;
c.g[A2][0] = c.g[V0][0];
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x2c));
c.g[A1][0] = sx((c.g[A2][0] as u32) << 2);
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.sh((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A0][0]);
c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S1][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V1][0]);
if t {
break 'b_3c990c;
} else {
'b_3c98e0: {
c.cycles += 8;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[A2][0] as u32)));
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
c.g[A1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x38));
c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[RA][0] = sx(0x003c979c);
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x24));
call_ptr(c, 0x003c6de0, 0x003c979c)?;
let t = c.g[V0][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
if t {
break 'b_3c98e0;
} else {
'b_3c98c0: {
'b_3c9844: {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
let t = c.g[V0][0] == 0u64;
c.sh((c.g[S0][0] as u32).wrapping_add(0x2c), 0u64);
if t {
break 'b_3c9844;
} else {
'b_3c9834: {
'b_3c97fc: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
if t {
break 'b_3c97fc;
} else {
c.cycles += 15;
c.g[RA][0] = sx(0x003c97c8);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
Callee { addr: 0x003ce078, f: ee::s14::SND_UtkGetState }.call(c, 0x003c97c8)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
c.g[A1][0] = c.g[SP][0];
c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(2, (c.g[SP][0] as u32));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.g[A2][0] = 0u64;
c.g[RA][0] = sx(0x003c97f4);
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S3][0]);
Callee { addr: 0x003ce0b8, f: ee::s14::SND_UtkSetState }.call(c, 0x003c97f4)?;
let t = true;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
if t {
break 'b_3c9834;
} else {
break 'b_3c97fc;
}
}
}
c.cycles += 14;
c.g[RA][0] = sx(0x003c9804);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
Callee { addr: 0x003ce078, f: ee::s14::SND_UtkGetState }.call(c, 0x003c9804)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
c.g[A1][0] = c.g[SP][0];
c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(2, (c.g[SP][0] as u32));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.g[A2][0] = 0u64;
c.g[RA][0] = sx(0x003c9830);
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
Callee { addr: 0x003ce0b8, f: ee::s14::SND_UtkSetState }.call(c, 0x003c9830)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
break 'b_3c9834;
}
c.cycles += 4;
c.g[RA][0] = sx(0x003c983c);
c.g[A1][0] = 0u64;
Callee { addr: 0x003ce0d8, f: ee::s14::SND_UtkSetMode }.call(c, 0x003c983c)?;
let t = true;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
if t {
break 'b_3c98c0;
} else {
break 'b_3c9844;
}
}
}
'b_3c98a0: {
'b_3c987c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
if t {
break 'b_3c987c;
} else {
c.cycles += 11;
c.g[RA][0] = sx(0x003c9858);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
Callee { addr: 0x003ce078, f: ee::s14::SND_UtkGetState }.call(c, 0x003c9858)?;
c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(2, (c.g[SP][0] as u32));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
let t = true;
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S3][0]);
if t {
break 'b_3c98a0;
} else {
break 'b_3c987c;
}
}
}
c.cycles += 9;
c.g[RA][0] = sx(0x003c9884);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
Callee { addr: 0x003ce078, f: ee::s14::SND_UtkGetState }.call(c, 0x003c9884)?;
c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0x17));
c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(6, (c.g[SP][0] as u32));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
break 'b_3c98a0;
}
c.cycles += 8;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
c.g[A1][0] = c.g[SP][0];
c.g[RA][0] = sx(0x003c98b0);
c.g[A2][0] = 0u64;
Callee { addr: 0x003ce0b8, f: ee::s14::SND_UtkSetState }.call(c, 0x003c98b0)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
c.g[RA][0] = sx(0x003c98bc);
c.g[A1][0] = 0x1;
Callee { addr: 0x003ce0d8, f: ee::s14::SND_UtkSetMode }.call(c, 0x003c98bc)?;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
break 'b_3c98c0;
}
c.cycles += 8;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
c.g[A3][0] = c.g[A2][0];
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[RA][0] = sx(0x003c98d8);
c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
Callee { addr: 0x003cdde8, f: ee::s14::SND_UtkSetInput }.call(c, 0x003c98d8)?;
let t = true;

if t {
break 'b_3c9910;
} else {
break 'b_3c98e0;
}
}
}
'b_3c9904: {
c.cycles += 2;
if (c.g[S1][0] as i64) <= 0 {
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
break 'b_3c9904;
} else {
'b_3c9900: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
if t {
break 'b_3c9900;
} else {
c.cycles += 3;
c.g[A2][0] = sx((c.g[S1][0] as u32) << 2);
c.g[RA][0] = sx(0x003c9900);
c.g[A1][0] = 0u64;
call_ptr(c, 0x00416210, 0x003c9900)?;
break 'b_3c9900;
}
}
c.cycles += 1;
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
break 'b_3c9904;
}
}
c.cycles += 2;
let t = true;
c.g[S1][0] = 0u64;
if t {
break 'b_3c9910;
} else {
break 'b_3c990c;
}
}
}
c.cycles += 1;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[A2][0] as u32)));
break 'b_3c9910;
}
c.cycles += 2;
if (c.g[S1][0] as i64) > 0 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
if event_due(c) { deliver_event_here(c, 0x003c9740); } continue 'l_3c9740;
} else {
break 'b_3c9918;
}
}
}
}
c.cycles += 8;
c.g[V0][0] = c.g[S2][0];
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
c.pc = t; return Ok(());
    });
}
/// `int SND_DecEaxa_DecodeLoop(EaxaDec* self, int count, ?, short* out)` (0x003ca0b8):
/// the EA-XA ADPCM decode loop: decodes `count` samples through
/// `SND_EaxaDecode`, re-feeding the input (`SND_EaxaSetInput`) per 28-sample
/// frame and saving/restoring the two-sample history
/// (`SND_EaxaGetHistory`/`SetHistory`) at frame boundaries. Returns the
/// number of samples decoded (v0 = s4).
pub fn snd_dec_eaxa_decode_loop(c: &mut Cpu) {
    guest(c, |c| {
'b_3ca330: {
'b_3ca32c: {
'b_3ca2b4: {
c.cycles += 15;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x120));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
c.g[S0][0] = c.g[A0][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[S1][0] = c.g[A1][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[S4][0] = 0u64;
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sd((c.g[SP][0] as u32).wrapping_add(0xb0), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
c.g[V0][0] = ((c.g[V0][0] as i64) < 3i64) as u64;
let t = c.g[V0][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[A3][0]);
if t {
break 'b_3ca2b4;
} else {
'b_3ca134: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x1c;
if t {
break 'b_3ca134;
} else {
'b_3ca10c: {
c.cycles += 3;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x44));
if c.g[V0][0] == 0u64 {
break 'b_3ca10c;
} else {
break 'b_3ca10c;
}
}
c.cycles += 10;
c.div(0, 6, 2);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
c.g[A2][0] = c.lo[0];
c.mult(0, 6, 6, 2);
c.g[A3][0] = c.g[A2][0];
c.g[RA][0] = sx(0x003ca12c);
c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
Callee { addr: 0x003ccf60, f: ee::s14::SND_EaxaSetInput }.call(c, 0x003ca12c)?;
c.g[V1][0] = 0x1;
c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[V1][0]);
break 'b_3ca134;
}
}
c.cycles += 2;
let t = c.g[S1][0] == 0u64;
c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
if t {
break 'b_3ca32c;
} else {
c.cycles += 3;
c.g[S3][0] = 0x1c;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
'l_3ca148: loop {
'b_3ca2a4: {
'b_3ca2a0: {
c.cycles += 14;
c.g[A1][0] = c.g[S5][0];
c.g[RA][0] = sx(0x003ca154);
c.g[A2][0] = c.g[S1][0];
Callee { addr: 0x003ccf90, f: ee::s14::SND_EaxaDecode }.call(c, 0x003ca154)?;
c.g[A3][0] = c.g[V0][0];
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S1][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[A0][0]);
if t {
break 'b_3ca2a0;
} else {
'b_3ca18c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x44));
if c.g[S3][0] == 0u64 {
break 'b_3ca18c;
} else {
break 'b_3ca18c;
}
}
'b_3ca260: {
c.cycles += 21;
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[A3][0] as u32)));
c.div(0, 2, 19);
c.g[S2][0] = c.hi[0];
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
c.div(0, 2, 19);
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x34));
c.g[A3][0] = c.g[A2][0];
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
c.g[V1][0] = c.lo[0];
c.g[A1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[RA][0] = sx(0x003ca1d4);
c.g[A1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
Callee { addr: 0x003ccf60, f: ee::s14::SND_EaxaSetInput }.call(c, 0x003ca1d4)?;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
if c.g[V1][0] != 0u64 {
c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4c));
break 'b_3ca260;
} else {
c.cycles += 32;
c.g[RA][0] = sx(0x003ca1e8);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
Callee { addr: 0x003cd138, f: ee::s14::SND_EaxaGetHistory }.call(c, 0x003ca1e8)?;
c.g[V1][0] = c.g[V0][0] << 32;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
c.g[V1][0] = c.g[V1][0] >> 32;
c.g[V0][0] = c.g[V0][0] >> 32;
c.sw((c.g[SP][0] as u32).wrapping_add(0x90), c.g[V1][0]);
c.g[A2][0] = c.g[S1][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x94), c.g[V0][0]);
c.g[A1][0] = c.g[S5][0];
c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x97));
c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(2, (c.g[SP][0] as u32));
c.lwc1(0, (c.g[SP][0] as u32));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4c));
c.g[RA][0] = sx(0x003ca22c);
c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x50));
Callee { addr: 0x003ccf90, f: ee::s14::SND_EaxaDecode }.call(c, 0x003ca22c)?;
c.g[A3][0] = c.g[V0][0];
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A1][0] = 0x1;
c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[A0][0]);
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[A3][0] as u32)));
c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[A1][0]);
let t = true;
c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A3][0] as u32)));
if t {
break 'b_3ca2a4;
} else {
break 'b_3ca260;
}
}
}
c.cycles += 16;
c.g[A1][0] = c.g[SP][0];
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x50));
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
c.swc1(1, (c.g[SP][0] as u32));
c.g[RA][0] = sx(0x003ca278);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
Callee { addr: 0x003cd190, f: ee::s14::SND_EaxaSetHistory }.call(c, 0x003ca278)?;
c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x18));
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
c.sw((c.g[SP][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
c.g[A2][0] = c.g[S2][0];
c.g[RA][0] = sx(0x003ca290);
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa4));
Callee { addr: 0x003ccf90, f: ee::s14::SND_EaxaDecode }.call(c, 0x003ca290)?;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = true;
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V1][0]);
if t {
break 'b_3ca2a4;
} else {
break 'b_3ca2a0;
}
}
}
c.cycles += 1;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[A3][0] as u32)));
break 'b_3ca2a4;
}
c.cycles += 2;
if c.g[S1][0] != 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
if event_due(c) { deliver_event_here(c, 0x003ca148); } continue 'l_3ca148;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = c.g[S4][0];
if t {
break 'b_3ca330;
} else {
break 'b_3ca2b4;
}
}
}
}
}
}
c.cycles += 2;
let t = (c.g[S1][0] as i64) <= 0;
c.g[V0][0] = c.g[S4][0];
if t {
break 'b_3ca330;
} else {
c.cycles += 1;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
'l_3ca2c0: loop {
'b_3ca324: {
c.cycles += 15;
c.g[A2][0] = c.g[S1][0];
c.g[RA][0] = sx(0x003ca2cc);
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
Callee { addr: 0x003ccf90, f: ee::s14::SND_EaxaDecode }.call(c, 0x003ca2cc)?;
c.g[A3][0] = c.g[V0][0];
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[V0][0] = sx((c.g[A3][0] as u32) << 2);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[A3][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.sw((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[V1][0]);
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[A0][0]);
c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[S1][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[A3][0] as u32)));
if t {
break 'b_3ca324;
} else {
c.cycles += 10;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.g[A3][0] = c.g[A2][0];
c.g[RA][0] = sx(0x003ca324);
c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
Callee { addr: 0x003ccf60, f: ee::s14::SND_EaxaSetInput }.call(c, 0x003ca324)?;
break 'b_3ca324;
}
}
c.cycles += 2;
if (c.g[S1][0] as i64) > 0 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
if event_due(c) { deliver_event_here(c, 0x003ca2c0); } continue 'l_3ca2c0;
} else {
break 'b_3ca32c;
}
}
}
}
c.cycles += 1;
c.g[V0][0] = c.g[S4][0];
break 'b_3ca330;
}
c.cycles += 9;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x100));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xb0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
c.pc = t; return Ok(());
    });
}
/// `void SND_FirDesign(Fir* f, int type)` (0x003ce7c0): designs a 4-tap FIR
/// (cut-off at `+0xf4`/`+0xf8`, taps at `+0xe0`) of type 2 (low/high pass),
/// 3 or 4 (band pass/stop) with a windowed sinc (`SND_Sinf`/`SND_Cosf`),
/// then normalises the taps by their sum.
pub fn snd_fir_design(c: &mut Cpu) {
    guest(c, |c| {
'b_3ce83c: {
c.cycles += 23;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
c.g[V1][0] = 0x1;
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S4][0] = c.g[A1][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S2][0] = c.g[A0][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x3));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S3][0] = 0x4;
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0xb8));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xa8));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
let t = c.g[V0][0] == 0u64;
c.sb((c.g[S2][0] as u32).wrapping_add(0xfc), c.g[V1][0]);
if t {
break 'b_3ce83c;
} else {
c.cycles += 8;
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0xf4));
c.g[AT][0] = 0x3f4c0000;
c.g[AT][0] = c.g[AT][0] | 0xcccd;
c.mtc1(1, 1);
c.c_lt_s(1, 0);
if c.fcc() {
c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0xf4));
break 'b_3ce83c;
} else {
break 'b_3ce83c;
}
}
}
'b_3ce9e0: {
'b_3ce940: {
'b_3ce8d8: {
c.cycles += 3;
c.g[V0][0] = 0x3;
let t = c.g[S4][0] == c.g[V0][0];
c.g[V0][0] = ((c.g[S4][0] as i64) < 4i64) as u64;
if t {
break 'b_3ce8d8;
} else {
'b_3ce874: {
'b_3ce860: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = 0x2;
if t {
break 'b_3ce860;
} else {
c.cycles += 2;
let t = c.g[S4][0] == c.g[V0][0];
c.g[S5][0] = ((c.g[S3][0] as i64) < 0i64) as u64;
if t {
break 'b_3ce874;
} else {
c.cycles += 2;
let t = true;

if t {
break 'b_3ce9e0;
} else {
break 'b_3ce860;
}
}
}
}
c.cycles += 3;
c.g[V0][0] = 0x4;
let t = c.g[S4][0] == c.g[V0][0];
c.g[S5][0] = ((c.g[S3][0] as i64) < 0i64) as u64;
if t {
break 'b_3ce940;
} else {
c.cycles += 2;
let t = true;

if t {
break 'b_3ce9e0;
} else {
break 'b_3ce874;
}
}
}
c.cycles += 7;
c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0xf8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[S1][0] = 0x1;
let t = c.g[S3][0] == 0u64;
c.swc1(0, (c.g[V1][0] as u32));
if t {
break 'b_3ce9e0;
} else {
c.cycles += 4;
c.g[AT][0] = 0x40490000;
c.g[AT][0] = c.g[AT][0] | 0xfdb;
c.mtc1(1, 20);
c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4));
'l_3ce8a0: loop {
c.cycles += 12;
c.mtc1(17, 0);
c.cvt_s_w(0, 0);
c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0xf8));
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
c.mul_s(21, 0, 20);
c.g[RA][0] = sx(0x003ce8bc);
c.mul_s(12, 12, 21);
call_ptr(c, 0x003bc770, 0x003ce8bc)?;
c.div_s(0, 0, 21);
c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S1][0] as i64)) as u64;
c.swc1(0, (c.g[S0][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x003ce8a0); } continue 'l_3ce8a0;
} else {
c.cycles += 2;
let t = true;

if t {
break 'b_3ce9e0;
} else {
break 'b_3ce8d8;
}
}
}
}
}
}
c.cycles += 8;
c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xe0));
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0xf4));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[S1][0] = 0x1;
c.g[S5][0] = ((c.g[S3][0] as i64) < 0i64) as u64;
let t = c.g[S3][0] == 0u64;
c.swc1(0, (c.g[V1][0] as u32));
if t {
break 'b_3ce9e0;
} else {
c.cycles += 4;
c.g[AT][0] = 0x40490000;
c.g[AT][0] = c.g[AT][0] | 0xfdb;
c.mtc1(1, 20);
c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4));
'l_3ce908: loop {
c.cycles += 12;
c.mtc1(17, 0);
c.cvt_s_w(0, 0);
c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0xf4));
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
c.mul_s(21, 0, 20);
c.g[RA][0] = sx(0x003ce924);
c.mul_s(12, 12, 21);
call_ptr(c, 0x003bc770, 0x003ce924)?;
c.div_s(0, 0, 21);
c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S1][0] as i64)) as u64;
c.swc1(0, (c.g[S0][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x003ce908); } continue 'l_3ce908;
} else {
c.cycles += 2;
let t = true;

if t {
break 'b_3ce9e0;
} else {
break 'b_3ce940;
}
}
}
}
}
c.cycles += 9;
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0xf8));
c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0xf4));
c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xe0));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[S1][0] = 0x1;
c.sub_s(0, 0, 1);
let t = c.g[S3][0] == 0u64;
c.swc1(0, (c.g[V0][0] as u32));
if t {
break 'b_3ce9e0;
} else {
c.cycles += 7;
c.g[AT][0] = 0x40490000;
c.g[AT][0] = c.g[AT][0] | 0xfdb;
c.mtc1(1, 23);
c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 22);
'l_3ce980: loop {
c.cycles += 24;
c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0xf8));
c.mtc1(17, 1);
c.cvt_s_w(1, 1);
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0xf4));
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
c.sub_s(12, 12, 0);
c.mul_s(21, 1, 23);
c.mul_s(12, 12, 22);
c.g[RA][0] = sx(0x003ce9a8);
c.mul_s(12, 12, 21);
call_ptr(c, 0x003bc770, 0x003ce9a8)?;
c.mov_s(20, 0);
c.lwc1(12, (c.g[S2][0] as u32).wrapping_add(0xf8));
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0xf4));
c.add_s(12, 12, 0);
c.mul_s(12, 12, 22);
c.g[RA][0] = sx(0x003ce9c4);
c.mul_s(12, 12, 21);
call_ptr(c, 0x003bc840, 0x003ce9c4)?;
c.add_s(20, 20, 20);
c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S1][0] as i64)) as u64;
c.mul_s(20, 20, 0);
c.div_s(20, 20, 21);
c.swc1(20, (c.g[S0][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x003ce980); } continue 'l_3ce980;
} else {
break 'b_3ce9e0;
}
}
}
}
'b_3ceab0: {
c.cycles += 6;
c.g[AT][0] = 0x3f490000;
c.g[AT][0] = c.g[AT][0] | 0xfdb;
c.mtc1(1, 21);
c.mtc1(0, 20);
let t = c.g[S5][0] != 0u64;
c.g[S1][0] = 0u64;
if t {
break 'b_3ceab0;
} else {
c.cycles += 10;
c.g[AT][0] = 0x3eeb0000;
c.g[AT][0] = c.g[AT][0] | 0x851f;
c.mtc1(1, 23);
c.g[FP][0] = 0x2;
c.g[AT][0] = 0x3f0a0000;
c.g[AT][0] = c.g[AT][0] | 0x3d71;
c.mtc1(1, 22);
c.g[S7][0] = 0x4;
c.g[S6][0] = 0x3;
c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xe0));
'l_3cea20: loop {
'b_3ceaa4: {
'b_3ceaa0: {
'b_3cea5c: {
'b_3cea30: {
c.cycles += 2;
let t = c.g[S4][0] == c.g[FP][0];

if t {
break 'b_3cea30;
} else {
c.cycles += 2;
let t = c.g[S4][0] != c.g[S7][0];

if t {
break 'b_3cea5c;
} else {
break 'b_3cea30;
}
}
}
c.cycles += 11;
c.mtc1(17, 12);
c.cvt_s_w(12, 12);
c.g[RA][0] = sx(0x003cea40);
c.mul_s(12, 21, 12);
call_ptr(c, 0x003bc840, 0x003cea40)?;
c.mul_s(0, 0, 23);
c.lwc1(1, (c.g[S0][0] as u32));
c.sub_s(0, 22, 0);
c.mul_s(1, 1, 0);
c.add_s(20, 20, 1);
let t = true;
c.swc1(1, (c.g[S0][0] as u32));
if t {
break 'b_3ceaa0;
} else {
break 'b_3cea5c;
}
}
c.cycles += 2;
if c.g[S4][0] != c.g[S6][0] {
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
break 'b_3ceaa4;
} else {
'b_3cea9c: {
c.cycles += 12;
c.mtc1(17, 12);
c.cvt_s_w(12, 12);
c.g[RA][0] = sx(0x003cea74);
c.mul_s(12, 21, 12);
call_ptr(c, 0x003bc840, 0x003cea74)?;
c.mul_s(0, 0, 23);
c.lwc1(1, (c.g[S0][0] as u32));
c.g[V0][0] = c.g[S1][0] & 0x1;
c.sub_s(0, 22, 0);
c.neg_s(0, 0);
c.mul_s(1, 1, 0);
let t = c.g[V0][0] != 0u64;
c.swc1(1, (c.g[S0][0] as u32));
if t {
break 'b_3cea9c;
} else {
c.cycles += 2;
let t = true;
c.add_s(20, 20, 1);
if t {
break 'b_3ceaa0;
} else {
break 'b_3cea9c;
}
}
}
c.cycles += 1;
c.sub_s(20, 20, 1);
break 'b_3ceaa0;
}
}
c.cycles += 1;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
break 'b_3ceaa4;
}
c.cycles += 3;
c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S1][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x003cea20); } continue 'l_3cea20;
} else {
break 'b_3ceab0;
}
}
}
}
'b_3ceae4: {
c.cycles += 3;
c.g[V0][0] = 0x3;
let t = c.g[S4][0] != c.g[V0][0];
c.g[V0][0] = 0u64;
if t {
break 'b_3ceae4;
} else {
c.cycles += 10;
c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xe0));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 1);
c.lwc1(0, (c.g[V0][0] as u32));
c.add_s(20, 20, 1);
c.add_s(0, 0, 1);
c.swc1(0, (c.g[V0][0] as u32));
c.g[V0][0] = 0u64;
break 'b_3ceae4;
}
}
'b_3ceb08: {
'b_3ceaf4: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = sx((c.g[S3][0] as u32) << 2);
if t {
break 'b_3ceaf4;
} else {
c.cycles += 2;
let t = true;
c.add_s(20, 20, 20);
if t {
break 'b_3ceb08;
} else {
break 'b_3ceaf4;
}
}
}
c.cycles += 5;
c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xe0));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.add_s(1, 20, 20);
c.lwc1(0, (c.g[V0][0] as u32));
c.sub_s(20, 1, 0);
break 'b_3ceb08;
}
'b_3ceb1c: {
c.cycles += 5;
c.mtc1(0, 0);
c.c_lt_s(20, 0);
if c.fcc() {
c.neg_s(20, 20);
break 'b_3ceb1c;
} else {
break 'b_3ceb1c;
}
}
'b_3ceb50: {
c.cycles += 2;
let t = c.g[S5][0] != 0u64;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
if t {
break 'b_3ceb50;
} else {
c.cycles += 3;
c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xe0));
c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
'l_3ceb30: loop {
c.cycles += 7;
c.lwc1(0, (c.g[V0][0] as u32));
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
c.div_s(0, 0, 20);
c.swc1(0, (c.g[V0][0] as u32));
let t = c.g[S1][0] != 0u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
if t {
if event_due(c) { deliver_event_here(c, 0x003ceb30); } continue 'l_3ceb30;
} else {
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
break 'b_3ceb50;
}
}
}
}
c.cycles += 15;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0xb8));
c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xa8));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
c.pc = t; return Ok(());
    });
}
/// `void SND_FirProcess(Fir* f, int count, float* in, float* out)` (0x003cebe8):
/// runs the FIR filter over `count` samples (a multiple of 16, four-float
/// vectors) with the VU0 matrix-multiply form: the filter state at `+0xc0`
/// is applied to each block of 16 floats, the carry kept in the state.
pub fn snd_fir_process(c: &mut Cpu) {
    guest(c, |c| {
'b_3ceca8: {
c.cycles += 6;
c.g[V1][0] = c.g[A0][0];
c.g[T3][0] = c.g[A1][0];
c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0xfc));
c.g[T2][0] = c.g[A2][0];
let t = c.g[V0][0] == 0u64;
c.g[T1][0] = c.g[A3][0];
if t {
break 'b_3ceca8;
} else {
c.cycles += 42;
c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0xe0));
c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0xe8));
c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0xec));
c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0xe4));
c.lwc1(4, (c.g[V1][0] as u32).wrapping_add(0xf0));
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8c));
c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x7c));
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x9c));
c.swc1(3, (c.g[V1][0] as u32));
c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x14));
c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x28));
c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x3c));
c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x10));
c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x24));
c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x38));
c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x4c));
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x20));
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x34));
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x48));
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x5c));
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x30));
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x44));
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x58));
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x6c));
c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x40));
c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x54));
c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x68));
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x50));
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x64));
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x78));
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x60));
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x74));
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x88));
c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x70));
c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0xac));
c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0xbc));
c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x84));
c.swc1(2, (c.g[V1][0] as u32).wrapping_add(0x98));
c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x80));
c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x94));
c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0xa8));
c.sb((c.g[V1][0] as u32).wrapping_add(0xfc), 0u64);
break 'b_3ceca8;
}
}
c.cycles += 22;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc0));
c.g[A2][0] = c.g[T3][0];
c.g[A0][0] = c.g[V1][0];
c.g[A1][0] = c.g[V0][0];
c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
c.g[A3][0] = c.g[T2][0];
c.g[T0][0] = c.g[T1][0];
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.lqc2(1, (c.g[A0][0] as u32));
c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x40));
c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x50));
c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x60));
c.lqc2(8, (c.g[A0][0] as u32).wrapping_add(0x70));
c.lqc2(9, (c.g[A0][0] as u32).wrapping_add(0x80));
c.lqc2(10, (c.g[A0][0] as u32).wrapping_add(0x90));
c.lqc2(11, (c.g[A0][0] as u32).wrapping_add(0xa0));
c.lqc2(12, (c.g[A0][0] as u32).wrapping_add(0xb0));
c.lqc2(20, (c.g[A1][0] as u32));
c.lqc2(21, (c.g[A1][0] as u32).wrapping_add(0x10));
'l_3ced00: loop {
c.cycles += 61;
c.lqc2(22, (c.g[A3][0] as u32));
c.lqc2(23, (c.g[A3][0] as u32).wrapping_add(0x10));
c.lqc2(24, (c.g[A3][0] as u32).wrapping_add(0x20));
c.lqc2(25, (c.g[A3][0] as u32).wrapping_add(0x30));
c.vu0().mula(0b1111, 1, Src::Bc(20, X));
c.vu0().madda(0b1111, 2, Src::Bc(20, Y));
c.vu0().madda(0b1111, 3, Src::Bc(20, Z));
c.vu0().madda(0b1111, 4, Src::Bc(20, W));
c.vu0().madda(0b1111, 5, Src::Bc(21, X));
c.vu0().madda(0b1111, 6, Src::Bc(21, Y));
c.vu0().madda(0b1111, 7, Src::Bc(21, Z));
c.vu0().madda(0b1111, 8, Src::Bc(21, W));
c.vu0().madda(0b1111, 9, Src::Bc(22, X));
c.vu0().madda(0b1111, 10, Src::Bc(22, Y));
c.vu0().madda(0b1111, 11, Src::Bc(22, Z));
c.vu0().madd(0b1111, 28, 12, Src::Bc(22, W));
c.vu0().mula(0b1111, 1, Src::Bc(21, X));
c.vu0().madda(0b1111, 2, Src::Bc(21, Y));
c.vu0().madda(0b1111, 3, Src::Bc(21, Z));
c.vu0().madda(0b1111, 4, Src::Bc(21, W));
c.vu0().madda(0b1111, 5, Src::Bc(22, X));
c.vu0().madda(0b1111, 6, Src::Bc(22, Y));
c.vu0().madda(0b1111, 7, Src::Bc(22, Z));
c.vu0().madda(0b1111, 8, Src::Bc(22, W));
c.vu0().madda(0b1111, 9, Src::Bc(23, X));
c.vu0().madda(0b1111, 10, Src::Bc(23, Y));
c.vu0().madda(0b1111, 11, Src::Bc(23, Z));
c.vu0().madd(0b1111, 29, 12, Src::Bc(23, W));
c.vu0().mula(0b1111, 1, Src::Bc(22, X));
c.vu0().madda(0b1111, 2, Src::Bc(22, Y));
c.vu0().madda(0b1111, 3, Src::Bc(22, Z));
c.vu0().madda(0b1111, 4, Src::Bc(22, W));
c.vu0().madda(0b1111, 5, Src::Bc(23, X));
c.vu0().madda(0b1111, 6, Src::Bc(23, Y));
c.vu0().madda(0b1111, 7, Src::Bc(23, Z));
c.vu0().madda(0b1111, 8, Src::Bc(23, W));
c.vu0().madda(0b1111, 9, Src::Bc(24, X));
c.vu0().madda(0b1111, 10, Src::Bc(24, Y));
c.vu0().madda(0b1111, 11, Src::Bc(24, Z));
c.vu0().madd(0b1111, 30, 12, Src::Bc(24, W));
c.vu0().mula(0b1111, 1, Src::Bc(23, X));
c.vu0().madda(0b1111, 2, Src::Bc(23, Y));
c.vu0().madda(0b1111, 3, Src::Bc(23, Z));
c.vu0().madda(0b1111, 4, Src::Bc(23, W));
c.vu0().madda(0b1111, 5, Src::Bc(24, X));
c.vu0().madda(0b1111, 6, Src::Bc(24, Y));
c.vu0().madda(0b1111, 7, Src::Bc(24, Z));
c.vu0().madda(0b1111, 8, Src::Bc(24, W));
c.vu0().madda(0b1111, 9, Src::Bc(25, X));
c.vu0().madda(0b1111, 10, Src::Bc(25, Y));
c.vu0().madda(0b1111, 11, Src::Bc(25, Z));
c.vu0().madd(0b1111, 31, 12, Src::Bc(25, W));
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
c.sqc2(28, (c.g[T0][0] as u32));
c.sqc2(29, (c.g[T0][0] as u32).wrapping_add(0x10));
c.sqc2(30, (c.g[T0][0] as u32).wrapping_add(0x20));
c.sqc2(31, (c.g[T0][0] as u32).wrapping_add(0x30));
c.vu0().vmove(0b1111, 20, 24);
c.vu0().vmove(0b1111, 21, 25);
let t = c.g[A2][0] != c.g[A3][0];
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x40));
if t {
if event_due(c) { deliver_event_here(c, 0x003ced00); } continue 'l_3ced00;
} else {
c.cycles += 4;
c.sqc2(20, (c.g[A1][0] as u32));
c.sqc2(21, (c.g[A1][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.pc = t; return Ok(());
}
}
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered};
    use crate::difftest::{check, Rng, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Control words of the oc1152 fixtures (per machine): the gate for the
    /// callee fakes in the shared override slots.
    pub(crate) const OC1152_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC1152_MAGIC: u32 = 0x0c11_5252;

    /// A fake callee that logs like `fakes::fake_call` and shapes the
    /// results the callers branch on: the decoders return a count no larger
    /// than their count argument (full, half or full), `SNDSTRM_PopBlock`
    /// writes a size through `a3` and returns a pointer or 0, and the
    /// sine/cosine return a float in [-1, 1].
    pub(crate) fn oc1152_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let (a0, a2, a3) = (c.g[A0][0] as u32, c.g[A2][0] as u32, c.g[A3][0] as u32);
        let h = fakes::mix(c.cycles ^ (addr as u64) << 20 ^ a0 as u64);
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        let n = (a2 as i32).max(0) as u32;
        match addr {
            0x003c_de68 | 0x003c_cf90 => {
                let v = if h % 4 == 1 { n.div_ceil(2) } else { n };
                c.g[V0][0] = sx(v);
            }
            0x003c_6de0 => {
                let size = if h % 3 == 0 { 0 } else { ((h >> 8) as u32 & 0xff) + 1 };
                c.bus.write32(a3, size);
                c.g[V0][0] = if (h >> 16) % 3 == 0 {
                    0
                } else {
                    sx(DATA + 0x4000 + ((h >> 24) as u32 & 0xf0))
                };
            }
            0x003b_c770 | 0x003b_c840 => {
                let f = ((h >> 4) % 2001) as f32 / 1000.0 - 1.0;
                c.f[0] = f.to_bits();
            }
            _ => {}
        }
    }

    /// The `miss` hook of the fixtures (indirect calls into the table).
    pub(crate) fn oc1152_miss(c: &mut Cpu, target: u32) {
        oc1152_fake_call(c, target);
    }

    macro_rules! oc1152_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1152_CTRL) != OC1152_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1152_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc1152_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1152_fakes!(
        (oc1152_fake_3cde68, OC1152_PREV_3CDE68, ee::s14::OVERRIDE_003CDE68, ee::s14::f_003cde68__generated, 0x003c_de68),
        (oc1152_fake_3ce078, OC1152_PREV_3CE078, ee::s14::OVERRIDE_003CE078, ee::s14::f_003ce078__generated, 0x003c_e078),
        (oc1152_fake_3ce0b8, OC1152_PREV_3CE0B8, ee::s14::OVERRIDE_003CE0B8, ee::s14::f_003ce0b8__generated, 0x003c_e0b8),
        (oc1152_fake_3ce0d8, OC1152_PREV_3CE0D8, ee::s14::OVERRIDE_003CE0D8, ee::s14::f_003ce0d8__generated, 0x003c_e0d8),
        (oc1152_fake_3cdde8, OC1152_PREV_3CDDE8, ee::s14::OVERRIDE_003CDDE8, ee::s14::f_003cdde8__generated, 0x003c_dde8),
        (oc1152_fake_3ccf60, OC1152_PREV_3CCF60, ee::s14::OVERRIDE_003CCF60, ee::s14::f_003ccf60__generated, 0x003c_cf60),
        (oc1152_fake_3ccf90, OC1152_PREV_3CCF90, ee::s14::OVERRIDE_003CCF90, ee::s14::f_003ccf90__generated, 0x003c_cf90),
        (oc1152_fake_3cd138, OC1152_PREV_3CD138, ee::s14::OVERRIDE_003CD138, ee::s14::f_003cd138__generated, 0x003c_d138),
        (oc1152_fake_3bc770, OC1152_PREV_3BC770, ee::s13::OVERRIDE_003BC770, ee::s13::f_003bc770__generated, 0x003b_c770),
        (oc1152_fake_3bc840, OC1152_PREV_3BC840, ee::s13::OVERRIDE_003BC840, ee::s13::f_003bc840__generated, 0x003b_c840),
        (oc1152_fake_3cd190, OC1152_PREV_3CD190, ee::s14::OVERRIDE_003CD190, ee::s14::f_003cd190__generated, 0x003c_d190),
    );

    /// A machine with the gated fakes armed and the indirect-call hook set.
    fn oc1152_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc1152_install_fakes();
        let r = fakes::machine(c, seed, salt, calls);
        c.hooks.miss = oc1152_miss;
        c.bus.write32(OC1152_CTRL, OC1152_MAGIC);
        r
    }

    /// `SND_DecUtk_DecodeStream`: count 0 / negative / positive, a pending
    /// byte count at `+0x28` (releases bytes first), the mode word at `+0x30`
    /// below and above 3, and PopBlock sizes of 0 or more (both state
    /// restore arms), plus unwinds in the callees.
    #[test]
    fn snd_dec_utk_decode_stream_matches() {
        covered(
            ee::s14::f_003c96f0__generated,
            snd_dec_utk_decode_stream,
            300,
            &|c, seed| {
                let mut r = oc1152_machine(c, seed, 0x3c96f0, 6);
                let s = DATA + 0x100;
                for i in 0..0x10 {
                    c.bus.write32(s + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(s);
                c.g[A1][0] = match seed % 6 {
                    0 => 0,
                    1 => sx((-3i32) as u32),
                    _ => 1 + (r.next_u32() % 40) as u64,
                };
                c.g[A3][0] = sx(DATA + 0x1000);
                c.bus
                    .write32(s + 0x28, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
                c.bus
                    .write32(s + 0x30, if (seed / 2) % 2 == 0 { 1 } else { 5 });
                c.bus.write32(s + 0x34, DATA + 0x2000);
                c.bus.write32(s + 0x1c, r.next_u32() & 0xffff);
                c.bus.write16(s + 0x2c, r.next_u32() as u16 & 0x3ff);
            },
        );
    }

    /// `SND_DecEaxa_DecodeLoop`: count 0 / positive, the mode at `+0x30`
    /// below and above 3, the input-primed flag at `+0x40`, the history flag
    /// at `+0x3c`, divisors at `+0x44`, and partial decodes (full and half).
    #[test]
    fn snd_dec_eaxa_decode_loop_matches() {
        covered(
            ee::s14::f_003ca0b8__generated,
            snd_dec_eaxa_decode_loop,
            300,
            &|c, seed| {
                let mut r = oc1152_machine(c, seed, 0x3ca0b8, 8);
                let s = DATA + 0x100;
                for i in 0..0x14 {
                    c.bus.write32(s + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(s);
                c.g[A1][0] = match seed % 7 {
                    0 => 0,
                    _ => 1 + (r.next_u32() % 90) as u64,
                };
                c.g[A3][0] = sx(DATA + 0x1000);
                c.bus
                    .write32(s + 0x30, if seed % 2 == 0 { 1 } else { 5 });
                c.bus
                    .write32(s + 0x40, if (seed / 2) % 2 == 0 { 0 } else { 1 });
                c.bus
                    .write32(s + 0x3c, if (seed / 4) % 2 == 0 { 0 } else { 1 });
                c.bus
                    .write32(s + 0x44, if seed % 11 == 0 { 0 } else { 1 + r.next_u32() % 9 });
                c.bus.write32(s + 0x48, r.next_u32() % 200);
                c.bus.write32(s + 0x34, DATA + 0x2000);
                c.bus.write32(s + 0x20, r.next_u32() % 100);
                c.bus.write32(s + 0x24, r.next_u32() % 50);
                c.bus.write32(s + 0x28, r.next_u32() % 100);
            },
        );
    }

    /// `SND_FirDesign`: filter types 2, 3, 4 and an unknown one (1, 5), with
    /// cut-offs `+0xf4`/`+0xf8` in the usual range; the sine/cosine fakes
    /// return floats in [-1, 1].
    #[test]
    fn snd_fir_design_matches() {
        covered(
            ee::s14::f_003ce7c0__generated,
            snd_fir_design,
            250,
            &|c, seed| {
                let mut r = oc1152_machine(c, seed, 0x3ce7c0, 12);
                let f = DATA + 0x100;
                for i in 0..0x44 {
                    c.bus.write32(f + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(f);
                c.g[A1][0] = [3u64, 4, 2, 3, 4, 5, 1, 2][(seed % 8) as usize];
                c.bus.write32(f + 0xf4, f32::from_bits(r.f32(0.5)).abs().to_bits());
                c.bus.write32(f + 0xf8, (f32::from_bits(r.f32(0.5)).abs() + 0.5).to_bits());
            },
        );
    }

    /// `SND_FirProcess`: 16, 32 or 48 samples (the loop steps 0x40 bytes),
    /// the dirty flag at `+0xfc` set or clear, random filter state.
    #[test]
    fn snd_fir_process_matches() {
        check(
            ee::s14::f_003cebe8__generated,
            snd_fir_process,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3cebe8, 0);
                let f = DATA + 0x100;
                for i in 0..0x40 {
                    c.bus.write32(f + 4 * i, r.f32(2.0));
                }
                c.bus.write8(f + 0xfc, (seed % 2) as u8);
                let input = DATA + 0x1000;
                for i in 0..48 {
                    c.bus.write32(input + 4 * i, r.f32(1.0));
                }
                for i in 0..48 {
                    c.bus.write32(DATA + 0x2000 + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(f);
                c.g[A1][0] = 16 * (1 + (seed % 3) as u64);
                c.g[A2][0] = sx(input);
                c.g[A3][0] = sx(DATA + 0x2000);
            },
        );
    }
}
