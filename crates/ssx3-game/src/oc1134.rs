//! Staging module oc1134 (wave wave271): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1134`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `void Render2D_DrawWipeQuad(Element* e, Rect* rect, Colour* col, int flags, float fill)`
/// (0x3a2220, `cTextureElement::render2`): draws the bottom `fill` fraction of
/// a texture element as one quad (rect.y/h and the v coordinate are cut by
/// `1 - fill`), tinted `col` (a * 128, rgb * 255 as ints per vertex); flag 4
/// draws it rotated by 90 degrees (`DrawQuadRot90`), else `DrawQuad`.
pub fn render2d_draw_wipe_quad(c: &mut Cpu) {
    guest(c, |c| {
c.cycles += 46;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 2);
c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
c.ldr(2, (c.g[A1][0] as u32));
c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(2, (c.g[SP][0] as u32));
c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
c.sub_s(12, 2, 12);
c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
c.ldr(2, (c.g[A2][0] as u32));
c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0xf));
c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x8));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x1f));
c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x18));
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.sub_s(2, 2, 12);
c.g[A1][0] = c.lw((c.g[A0][0] as u32));
c.g[A2][0] = c.g[SP][0];
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = 0x3;
c.g[T1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A1][0]);
c.g[V0][0] = 0xffffffffffffffff;
c.lwc1(5, (c.g[A0][0] as u32).wrapping_add(0xc));
c.lwc1(6, (c.g[A0][0] as u32).wrapping_add(0x18));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
c.sub_s(4, 6, 5);
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.mul_s(3, 12, 0);
c.lwc1(8, (c.g[A0][0] as u32).wrapping_add(0x14));
c.mul_s(0, 0, 2);
c.lwc1(7, (c.g[A0][0] as u32).wrapping_add(0x10));
c.mul_s(12, 12, 4);
c.add_s(1, 1, 3);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
c.add_s(5, 5, 12);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
'l_3a22d8: loop {
c.cycles += 7;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
let t = c.g[V1][0] != c.g[V0][0];
c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(0xc0));
if t {
if event_due(c) { deliver_event_here(c, 0x003a22d8); } continue 'l_3a22d8;
} else {
c.cycles += 9;
c.g[AT][0] = 0x43000000;
c.mtc1(1, 4);
c.g[A3][0] = c.g[A3][0] & 0x4;
c.g[AT][0] = 0x437f0000;
c.mtc1(1, 2);
c.g[V1][0] = c.g[T1][0];
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 3);
c.lwc1(0, (c.g[T0][0] as u32));
'l_3a2318: loop {
c.cycles += 20;
c.mul_s(0, 0, 4);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x1c));
c.lwc1(1, (c.g[T0][0] as u32).wrapping_add(0x4));
c.mul_s(1, 1, 2);
c.cvt_w_s(0, 1);
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x10));
c.lwc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
c.mul_s(0, 0, 2);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
c.lwc1(1, (c.g[T0][0] as u32).wrapping_add(0xc));
c.swc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
c.mul_s(1, 1, 2);
c.cvt_w_s(0, 1);
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x18));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.lwc1(0, (c.g[T0][0] as u32));
if event_due(c) { deliver_event_here(c, 0x003a2318); } continue 'l_3a2318;
} else {
'b_3a23b0: {
'b_3a23a0: {
c.cycles += 9;
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x54));
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x80));
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xb4));
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x20));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x24));
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x50));
let t = c.g[A3][0] == 0u64;
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x84));
if t {
break 'b_3a23a0;
} else {
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.g[A1][0] = c.g[T1][0];
call_ptr(c, 0x0038e200, 0x003a2398)?;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
break 'b_3a23b0;
}
}
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
c.g[A1][0] = c.g[T1][0];
call_ptr(c, 0x0038e118, 0x003a23ac)?;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
break 'b_3a23b0;
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.pc = t; return Ok(());
}
}
}
}
    })
}

/// `void Render2D_DrawSpriteXform(Element* e, Rect* rect, Colour* col, Xform* xf, int flags)`
/// (0x3a2a00, `cTextureElement::renderRot`): draws a texture element as one
/// quad through a transform; flags 1 and 2 swap the element's u and v pairs,
/// flag 4 draws the rotated quad (`DrawQuadRot90Xform`), else `DrawQuadXform`.
pub fn render2d_draw_sprite_xform(c: &mut Cpu) {
    guest(c, |c| {
'b_3a2ad0: {
c.cycles += 50;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x120));
c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
c.ldl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
c.ldr(2, (c.g[A1][0] as u32));
c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0xf));
c.ldr(3, (c.g[A1][0] as u32).wrapping_add(0x8));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(2, (c.g[SP][0] as u32));
c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[T1][0] = c.g[A0][0];
c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
c.ldr(2, (c.g[A2][0] as u32));
c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0xf));
c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x8));
c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0x17));
c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x10));
c.ldl(5, (c.g[A2][0] as u32).wrapping_add(0x1f));
c.ldr(5, (c.g[A2][0] as u32).wrapping_add(0x18));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x1f));
c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x18));
c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x27));
c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x2f));
c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x28));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
c.ldl(10, (c.g[A3][0] as u32).wrapping_add(0x7));
c.ldr(10, (c.g[A3][0] as u32));
c.ldl(2, (c.g[A3][0] as u32).wrapping_add(0xf));
c.ldr(2, (c.g[A3][0] as u32).wrapping_add(0x8));
c.sdl(10, (c.g[SP][0] as u32).wrapping_add(0x47));
c.sdr(10, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x4f));
c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x48));
c.g[A2][0] = c.g[SP][0];
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[T1][0] as u32));
c.g[T2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
c.g[A1][0] = c.g[T0][0] & 0x1;
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.lwc1(6, (c.g[T1][0] as u32).wrapping_add(0xc));
c.lwc1(4, (c.g[T1][0] as u32).wrapping_add(0x18));
c.lwc1(5, (c.g[T1][0] as u32).wrapping_add(0x10));
let t = c.g[A1][0] == 0u64;
c.lwc1(3, (c.g[T1][0] as u32).wrapping_add(0x14));
if t {
break 'b_3a2ad0;
} else {
c.cycles += 2;
c.mov_s(5, 3);
c.lwc1(3, (c.g[T1][0] as u32).wrapping_add(0x10));
break 'b_3a2ad0;
}
}
'b_3a2ae4: {
c.cycles += 3;
c.g[V0][0] = c.g[T0][0] & 0x2;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.g[T0][0] & 0x4;
if t {
break 'b_3a2ae4;
} else {
c.cycles += 2;
c.mov_s(6, 4);
c.lwc1(4, (c.g[T1][0] as u32).wrapping_add(0xc));
break 'b_3a2ae4;
}
}
c.cycles += 3;
c.g[V0][0] = 0x3;
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[V1][0] = 0xffffffffffffffff;
'l_3a2af0: loop {
c.cycles += 7;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
let t = c.g[V0][0] != c.g[V1][0];
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc0));
if t {
if event_due(c) { deliver_event_here(c, 0x003a2af0); } continue 'l_3a2af0;
} else {
c.cycles += 9;
c.g[AT][0] = 0x43000000;
c.mtc1(1, 8);
c.g[V1][0] = c.g[A1][0];
c.g[AT][0] = 0x437f0000;
c.mtc1(1, 2);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 7);
c.lwc1(0, (c.g[T2][0] as u32));
'l_3a2b30: loop {
c.cycles += 20;
c.mul_s(0, 0, 8);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x1c));
c.lwc1(1, (c.g[T2][0] as u32).wrapping_add(0x4));
c.mul_s(1, 1, 2);
c.cvt_w_s(0, 1);
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x10));
c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
c.mul_s(0, 0, 2);
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[V1][0] as u32).wrapping_add(0x14));
c.lwc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
c.swc1(7, (c.g[V1][0] as u32).wrapping_add(0x8));
c.mul_s(1, 1, 2);
c.cvt_w_s(0, 1);
c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x18));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.lwc1(0, (c.g[T2][0] as u32));
if event_due(c) { deliver_event_here(c, 0x003a2b30); } continue 'l_3a2b30;
} else {
'b_3a2bc0: {
'b_3a2bb4: {
c.cycles += 9;
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x84));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xe4));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x54));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x80));
let t = c.g[T0][0] == 0u64;
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xb4));
if t {
break 'b_3a2bb4;
} else {
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
call_ptr(c, 0x0038e468, 0x003a2bac)?;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
break 'b_3a2bc0;
}
}
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x844));
call_ptr(c, 0x0038e2e8, 0x003a2bbc)?;
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
break 'b_3a2bc0;
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
c.pc = t; return Ok(());
}
}
}
}
    })
}

/// `void SNDENC_PutBits(unsigned value, int n)` (0x3bcae8): appends the low `n`
/// bits of `value` to the encoder's bit buffer (`*0x50ac68`: `pos` bit
/// position as u16 at +6, byte buffer pointer at +8), MSB first within each
/// byte, bytes swapped pairwise (`byte ^ 1`); a fresh byte is cleared first.
pub fn snd_enc_put_bits(c: &mut Cpu) {
'b_3bcb94: {
c.cycles += 6;
c.g[T1][0] = c.g[A1][0];
c.g[V0][0] = 0x20;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[T1][0] as u32)));
c.g[T3][0] = sx((c.g[A0][0] as u32) << ((c.g[V0][0] as u32) & 31));
let t = (c.g[T1][0] as i64) <= 0;
c.g[T3][0] = sx((c.g[T3][0] as u32) >> ((c.g[V0][0] as u32) & 31));
if t {
break 'b_3bcb94;
} else {
c.cycles += 2;
c.g[T2][0] = 0x510000;
c.g[T4][0] = 0x8;
'l_3bcb08: loop {
'b_3bcb24: {
c.cycles += 6;
c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_sub(0x5398));
c.g[V1][0] = c.lhu((c.g[V0][0] as u32).wrapping_add(0x6));
c.g[T0][0] = sx((c.g[V1][0] as u32) >> 3);
c.g[V0][0] = c.g[T0][0] & 0x1;
if c.g[V0][0] == 0u64 {
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
break 'b_3bcb24;
} else {
c.cycles += 1;
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x1));
break 'b_3bcb24;
}
}
'b_3bcb4c: {
c.cycles += 6;
c.g[V1][0] = c.lw((c.g[T2][0] as u32).wrapping_sub(0x5398));
c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x6));
c.g[V0][0] = c.g[V0][0] & 0x7;
c.g[A3][0] = sx((c.g[T4][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
let t = c.g[A3][0] != c.g[T4][0];
c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_sub(0x5398));
if t {
break 'b_3bcb4c;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.sb((c.g[V0][0] as u32), 0u64);
c.g[V0][0] = c.lw((c.g[T2][0] as u32).wrapping_sub(0x5398));
break 'b_3bcb4c;
}
}
c.cycles += 18;
c.g[V1][0] = ((c.g[A3][0] as i64) < (c.g[T1][0] as i64)) as u64;
c.g[A2][0] = c.g[A3][0];
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
if c.g[V1][0] == 0 { c.g[A2][0] = c.g[T1][0]; }
c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_sub((c.g[A2][0] as u32)));
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub((c.g[A2][0] as u32)));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.g[V0][0] = sx((c.g[T3][0] as u32) >> ((c.g[A0][0] as u32) & 31));
c.g[V1][0] = c.lbu((c.g[A1][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A3][0] as u32) & 31));
c.g[T1][0] = c.g[A0][0];
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sb((c.g[A1][0] as u32), c.g[V1][0]);
c.g[A0][0] = c.lw((c.g[T2][0] as u32).wrapping_sub(0x5398));
c.g[V0][0] = c.lhu((c.g[A0][0] as u32).wrapping_add(0x6));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
let t = (c.g[T1][0] as i64) > 0;
c.sh((c.g[A0][0] as u32).wrapping_add(0x6), c.g[V0][0]);
if t {
if event_due(c) { deliver_event_here(c, 0x003bcb08); } continue 'l_3bcb08;
} else {
break 'b_3bcb94;
}
}
}
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.pc = t; return;
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_2220, render2d_draw_wipe_quad, "Render2D_DrawWipeQuad"),
    (0x003a_2a00, render2d_draw_sprite_xform, "Render2D_DrawSpriteXform"),
    (0x003b_cae8, snd_enc_put_bits, "SNDENC_PutBits"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes;
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the oc1134 quad fakes (own RAM word, both sides get their own).
    pub(crate) const OC1134_CTRL: u32 = DATA + 0x2e_0000;
    pub(crate) const OC1134_MAGIC: u32 = 0x1134_c0de;
    /// Number of logged draw calls, then the records.
    pub(crate) const OC1134_LOG_N: u32 = OC1134_CTRL + 4;
    pub(crate) const OC1134_LOG: u32 = OC1134_CTRL + 0x100;
    pub(crate) const OC1134_ELEM: u32 = DATA + 0x1000;
    pub(crate) const OC1134_RECT: u32 = DATA + 0x1100;
    pub(crate) const OC1134_COL: u32 = DATA + 0x1200;
    pub(crate) const OC1134_XF: u32 = DATA + 0x1300;
    pub(crate) const OC1134_GRMAN: u32 = DATA + 0x2000;
    pub(crate) const OC1134_DRAWSTATE: u32 = DATA + 0x3000;
    /// Encoder state of `SNDENC_PutBits`: the pointer at 0x50ac68, the struct
    /// (`pos` u16 at +6, buffer pointer at +8) and the byte buffer.
    pub(crate) const OC1134_ENC_PTR: u32 = 0x0050_ac68;
    pub(crate) const OC1134_ENC: u32 = DATA + 0x4000;
    pub(crate) const OC1134_BUF: u32 = DATA + 0x5000;

    pub(crate) static OC1134_PREV_QUAD: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1134_PREV_ROT: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1134_PREV_QUAD_XF: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static OC1134_PREV_ROT_XF: AtomicUsize = AtomicUsize::new(0);

    /// A gated fake draw call: logs the vertex array, rect, colour and
    /// transform the caller built (the real callees read them), then the usual
    /// fake (which logs the registers and may unwind).
    pub(crate) fn oc1134_fake_draw(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(OC1134_LOG_N);
        c.bus.write32(OC1134_LOG_N, n + 1);
        let p = OC1134_LOG + (n % 8) * 0x200;
        let (a1, a2, a3) = (
            c.g[A1][0] as u32,
            c.g[A2][0] as u32,
            c.g[A3][0] as u32,
        );
        for i in 0..48 {
            let w = c.bus.read32(a1 + 4 * i);
            c.bus.write32(p + 4 * i, w);
        }
        for i in 0..8 {
            let w = c.bus.read32(a2 + 4 * i);
            c.bus.write32(p + 0xc0 + 4 * i, w);
            let w = c.bus.read32(a3 + 4 * i);
            c.bus.write32(p + 0xe0 + 4 * i, w);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
    }

    pub(crate) fn oc1134_fake_quad(c: &mut Cpu) {
        if c.bus.read32(OC1134_CTRL) != OC1134_MAGIC {
            return fakes::fall_back(c, &OC1134_PREV_QUAD, ee::s12::f_0038e118__generated);
        }
        oc1134_fake_draw(c, 0x0038_e118)
    }

    pub(crate) fn oc1134_fake_rot(c: &mut Cpu) {
        if c.bus.read32(OC1134_CTRL) != OC1134_MAGIC {
            return fakes::fall_back(c, &OC1134_PREV_ROT, ee::s12::f_0038e200__generated);
        }
        oc1134_fake_draw(c, 0x0038_e200)
    }

    pub(crate) fn oc1134_fake_quad_xf(c: &mut Cpu) {
        if c.bus.read32(OC1134_CTRL) != OC1134_MAGIC {
            return fakes::fall_back(c, &OC1134_PREV_QUAD_XF, ee::s12::f_0038e2e8__generated);
        }
        oc1134_fake_draw(c, 0x0038_e2e8)
    }

    pub(crate) fn oc1134_fake_rot_xf(c: &mut Cpu) {
        if c.bus.read32(OC1134_CTRL) != OC1134_MAGIC {
            return fakes::fall_back(c, &OC1134_PREV_ROT_XF, ee::s12::f_0038e468__generated);
        }
        oc1134_fake_draw(c, 0x0038_e468)
    }

    pub(crate) fn oc1134_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038E118),
            oc1134_fake_quad,
            &OC1134_PREV_QUAD,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038E200),
            oc1134_fake_rot,
            &OC1134_PREV_ROT,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038E2E8),
            oc1134_fake_quad_xf,
            &OC1134_PREV_QUAD_XF,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038E468),
            oc1134_fake_rot_xf,
            &OC1134_PREV_ROT_XF,
        );
    }

    /// Element, rect, colour and transform data for the two sprite draws.
    fn oc1134_sprite_data(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let flt = |r: &mut Rng| {
            if seed % 5 == 0 {
                r.edge_f32()
            } else {
                r.f32(100.0)
            }
        };
        c.bus.write32(OC1134_CTRL, OC1134_MAGIC);
        c.bus.write32(OC1134_LOG_N, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x854), OC1134_GRMAN);
        c.bus.write32(OC1134_GRMAN + 0xe84, OC1134_DRAWSTATE);
        c.bus.write32(GAME_GP.wrapping_sub(0x844), r.next_u32());
        c.bus.write32(OC1134_ELEM, r.next_u32() & 0xffff);
        for off in [0xc, 0x10, 0x14, 0x18] {
            let v = flt(r);
            c.bus.write32(OC1134_ELEM + off, v);
        }
        for i in 0..4 {
            let v = flt(r);
            c.bus.write32(OC1134_RECT + 4 * i, v);
        }
        for i in 0..9 {
            let v = if i < 4 { r.f32(1.5) } else { r.next_u32() };
            c.bus.write32(OC1134_COL + 4 * i, v);
        }
        for i in 0..4 {
            let v = r.next_u32();
            c.bus.write32(OC1134_XF + 4 * i, v);
        }
        c.g[A0][0] = sx(OC1134_ELEM);
        c.g[A1][0] = sx(OC1134_RECT);
        c.g[A2][0] = sx(OC1134_COL);
    }

    fn oc1134_wipe_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = fakes::machine(c, seed, 0x3a_2220, 1);
        oc1134_install_fakes();
        oc1134_sprite_data(c, &mut r, seed);
        // Flags: bit 2 picks the rotated quad; the other bits and the upper
        // word are noise.
        let noise = if seed % 3 == 0 { 0 } else { r.next_u32() as u64 & !4 };
        c.g[A3][0] = noise | if seed % 2 == 0 { 4 } else { 0 };
        let fills = [0.0f32, 0.25, 0.5, 1.0, 1.5, -0.5];
        c.f[12] = if seed % 7 == 0 {
            r.edge_f32()
        } else {
            fills[(seed / 2 % 6) as usize].to_bits()
        };
        c.pc = 0x003a_2220;
        r
    }

    /// `Render2D_DrawWipeQuad`: both quad kinds (flag 4), fill fractions from
    /// below 0 to above 1 and float edge cases; the draw calls unwind at
    /// chosen indices.
    #[test]
    fn render2d_draw_wipe_quad_matches() {
        fakes::covered(
            ee::s13::f_003a2220__generated,
            render2d_draw_wipe_quad,
            300,
            &|c, seed| {
                oc1134_wipe_setup(c, seed);
            },
        );
    }

    fn oc1134_xform_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = fakes::machine(c, seed, 0x3a_2a00, 1);
        oc1134_install_fakes();
        oc1134_sprite_data(c, &mut r, seed);
        c.g[A3][0] = sx(OC1134_XF);
        // Flags (t0): 1 swaps the u pair, 2 swaps the v pair, 4 rotates.
        let noise = if seed % 3 == 0 { 0 } else { r.next_u32() as u64 & !7 };
        c.g[T0][0] = noise | (seed / 2 % 8) as u64;
        c.pc = 0x003a_2a00;
        r
    }

    /// `Render2D_DrawSpriteXform`: all eight flag combinations, plain and
    /// rotated quads, random rect, colour, transform; draw calls unwind.
    #[test]
    fn render2d_draw_sprite_xform_matches() {
        fakes::covered(
            ee::s13::f_003a2a00__generated,
            render2d_draw_sprite_xform,
            300,
            &|c, seed| {
                oc1134_xform_setup(c, seed);
            },
        );
    }

    fn oc1134_put_bits_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = fakes::machine(c, seed, 0x3b_cae8, 0);
        c.bus.write32(OC1134_ENC_PTR, OC1134_ENC);
        // Bit position: byte aligned and not, low and high; a stale buffer.
        let pos = match seed % 4 {
            0 => 0,
            1 => 8 * (r.next_u32() % 16),
            _ => r.next_u32() % 200,
        };
        c.bus.write16(OC1134_ENC + 6, pos as u16);
        c.bus.write32(OC1134_ENC + 8, OC1134_BUF);
        fill(c, &mut r, OC1134_BUF, 0x20);
        // Widths: none, negative, 1..32 bits and wider than a word.
        let n: i32 = match seed % 10 {
            0 => 0,
            1 => -(r.next_u32() as i32 % 9),
            2 => 32,
            3 => 33 + (r.next_u32() % 8) as i32,
            _ => 1 + (r.next_u32() % 31) as i32,
        };
        c.g[A0][0] = if seed % 5 == 0 { r.next_u32() as u64 } else { sx(r.next_u32()) };
        c.g[A1][0] = sx(n as u32);
        c.pc = 0x003b_cae8;
        r
    }

    /// `SNDENC_PutBits`: field widths from negative and zero to over 32, with
    /// the bit position byte aligned (fresh byte cleared) and not.
    #[test]
    fn snd_enc_put_bits_matches() {
        check(
            ee::s13::f_003bcae8__generated,
            snd_enc_put_bits,
            400,
            &|c, seed| {
                oc1134_put_bits_setup(c, seed);
            },
        );
    }

    #[test]
    fn snd_enc_put_bits_events() {
        driven!(gen_put, ee::s13::f_003bcae8__generated);
        driven!(hand_put, snd_enc_put_bits);
        check_events(gen_put, hand_put, 120, &|c, seed| {
            let mut r = oc1134_put_bits_setup(c, 270 + seed / 3 % 15);
            schedule(c, &mut r, seed, 0x003b_cae8)
        });
    }
}
