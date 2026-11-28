//! Staging module oc1481 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1481`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0038_1f10, ps2_graphics_man_v75, "DrawLineShape"),
    (0x0037_7cf0, ps2_graphics_man_v78, "DrawSprite3D"),
    (0x0038_5c10, cps2_grid_mesh_draw, "cPS2GridMesh_Draw"),
    (0x0037_d450, restore_render_target, "RestoreRenderTarget"),
    (0x0038_1ad0, ps2_graphics_man_v73, "DrawLineStrip"),
];

/// `PS2GraphicsMan_DrawLineShape(self, mat4x4* m, float* rgba, u32 tag)` (0x00381f10,
/// slot `DrawLineShape`, the reference's `RenderSpline`): set the GS register
/// block's prim/tex bits, make sure the view matrices are valid, then write one GIF
/// packet (header, colour, the 4x4 matrix) into the VIF stream and queue it in the
/// deferred-draw ring. Converted from the generated body; every register, cycle and
/// call kept.
#[allow(unused_labels, unused_assignments, unreachable_code, unused_variables)]
pub fn ps2_graphics_man_v75(c: &mut Cpu) {
    guest(c, |c| {
'b_381f74: {
c.cycles += 23;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
c.g[T0][0] = 0xffffffffc0000000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A0][0] = 0xfffffffffffffc3f;
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S1][0] = c.g[A2][0];
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[S3][0] = c.g[A1][0];
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | 0xc0;
c.sw((c.g[V1][0] as u32), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6b90));
let t = c.g[V1][0] != 0u64;
c.g[S2][0] = c.g[A3][0];
if t {
break 'b_381f74;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x00381f74);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x00381f74)?;
break 'b_381f74;
}
}
'b_382144: {
c.cycles += 84;
c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a00));
c.g[T3][0] = 0x30000000;
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] | 0xa;
c.g[V1][0] = 0xd811;
c.g[V1][0] = c.g[V1][0] << 47;
c.g[A1][0] = c.g[T2][0] | c.g[T3][0];
c.g[A0][0] = 0x10010000;
c.g[A0][0] = c.g[A0][0] | 0x4000;
c.g[A0][0] = c.g[A0][0] << 32;
c.g[A0][0] = c.g[A0][0] | 0x1;
c.g[A2][0] = 0xe;
c.sd((c.g[A1][0] as u32), c.g[V0][0]);
c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.g[V0][0] = c.g[S2][0] | 0x8000;
c.sd((c.g[A1][0] as u32).wrapping_add(0x20), c.g[A0][0]);
c.g[V1][0] = 0x800a;
c.g[V1][0] = c.g[V1][0] << 45;
c.sd((c.g[A1][0] as u32).wrapping_add(0x28), c.g[A2][0]);
c.g[V1][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[A1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = 0x1;
c.sd((c.g[A1][0] as u32).wrapping_add(0x10), 0u64);
c.g[A0][0] = 0x4;
c.g[AT][0] = 0x43000000;
c.mtc1(1, 0);
c.g[T0][0] = 0xfe00;
c.g[T0][0] = c.g[T0][0] << 46;
c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x8));
c.g[A3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x50));
c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0xc));
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x4));
c.mul_s(2, 2, 0);
c.lwc1(4, (c.g[S1][0] as u32));
c.mul_s(3, 3, 0);
c.mul_s(1, 1, 0);
c.sd((c.g[A1][0] as u32).wrapping_add(0x40), c.g[V1][0]);
c.mul_s(4, 4, 0);
c.sd((c.g[A1][0] as u32).wrapping_add(0x38), c.g[V0][0]);
c.sd((c.g[A1][0] as u32).wrapping_add(0x48), c.g[A0][0]);
c.cvt_w_s(0, 2);
c.mfc1(2, 0);
c.cvt_w_s(0, 3);
c.mfc1(3, 0);
c.cvt_w_s(0, 1);
c.mfc1(6, 0);
c.cvt_w_s(0, 4);
c.mfc1(4, 0);
c.g[V0][0] = c.g[V0][0] << 8;
c.g[V0][0] = c.g[A2][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 16;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A0][0] = c.g[A0][0] << 24;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
c.sd((c.g[A1][0] as u32).wrapping_add(0x30), c.g[V0][0]);
c.lqc2(1, (c.g[S3][0] as u32));
c.lqc2(2, (c.g[S3][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[S3][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[S3][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[A3][0] as u32));
c.sqc2(2, (c.g[A3][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[A3][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[A3][0] as u32).wrapping_add(0x30));
c.g[V0][0] = 0x0;
c.g[V1][0] = 0x14000000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3bf0));
c.sd((c.g[A1][0] as u32).wrapping_add(0x90), c.g[S2][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
c.sd((c.g[A1][0] as u32).wrapping_add(0x98), 0u64);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[A1][0] as u32).wrapping_add(0xa0), 0u64);
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V1][0] = 0xfff0000;
c.sd((c.g[A1][0] as u32).wrapping_add(0xa8), c.g[V0][0]);
c.g[V1][0] = c.g[V1][0] | 0xffff;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xb0));
c.g[T1][0] = c.g[A1][0] & c.g[V1][0];
c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18f0));
c.sw((c.g[S0][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
if t {
break 'b_382144;
} else {
c.cycles += 32;
c.g[A1][0] = 0x60000;
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.g[A1][0] | 0x8000;
c.g[V1][0] = 0xffffffffffffffe0;
c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
c.g[A2][0] = c.g[A2][0] | c.g[T3][0];
c.g[V0][0] = c.g[V0][0] & 0x1f;
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
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T2][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_382144;
}
}
c.cycles += 10;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a00));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[S0][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.pc = t; return Ok(());
    });
}

/// `PS2GraphicsMan_DrawSprite3D(self, float* pos, float* size, float* col, float* uv0,
/// float* uv1)` (0x00377cf0, slot `DrawSprite3D`): project a world position with the
/// screen-world matrix, build the screen-space corner pair from the size and the
/// screen scale, reject it with the clip codes, else write a two-corner sprite GIF
/// packet and queue it in the deferred-draw ring. Converted from the generated
/// body; every register, cycle and call kept.
#[allow(unused_labels, unused_assignments, unreachable_code, unused_variables)]
pub fn ps2_graphics_man_v78(c: &mut Cpu) {
    guest(c, |c| {
'b_377d38: {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S1][0] = c.g[A2][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S2][0] = c.g[A1][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S3][0] = c.g[T1][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S4][0] = c.g[A3][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6b90));
let t = c.g[V0][0] != 0u64;
c.g[S5][0] = c.g[T0][0];
if t {
break 'b_377d38;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x00377d38);
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x00377d38)?;
break 'b_377d38;
}
}
'b_377e14: {
c.cycles += 54;
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6b30));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.lqc2(8, (c.g[S2][0] as u32));
c.lqc2(4, (c.g[V0][0] as u32));
c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[V0][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[V0][0] as u32).wrapping_add(0x30));
c.vu0().mula(0b1111, 4, Src::Bc(8, X));
c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = 0u64;
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lwc1(6, (c.g[S1][0] as u32).wrapping_add(0x4));
c.sq(2, (c.g[SP][0] as u32));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
c.lwc1(5, (c.g[S1][0] as u32));
c.div_s(0, 0, 1);
c.lwc1(4, (c.g[S0][0] as u32).wrapping_add(0x6af0));
c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x6b04));
c.mul_s(4, 4, 5);
c.lwc1(3, (c.g[SP][0] as u32));
c.mul_s(2, 2, 6);
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
c.mul_s(1, 1, 0);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
c.mul_s(2, 2, 0);
c.mul_s(3, 3, 0);
c.mul_s(4, 4, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
c.swc1(3, (c.g[SP][0] as u32));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[SP][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(3, (c.g[SP][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6b70));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.c_lt_s(1, 0);
let t = !c.fcc();
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
if t {
break 'b_377e14;
} else {
c.cycles += 1;
c.g[V1][0] = 0x2;
break 'b_377e14;
}
}
'b_377e2c: {
c.cycles += 5;
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6b80));
c.c_lt_s(0, 1);
let t = !c.fcc();
c.mov_s(4, 0);
if t {
break 'b_377e2c;
} else {
c.cycles += 1;
c.g[V1][0] = c.g[V1][0] | 0x1;
break 'b_377e2c;
}
}
'b_377e48: {
c.cycles += 6;
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6b74));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
c.c_lt_s(1, 0);
let t = !c.fcc();
c.mov_s(3, 0);
if t {
break 'b_377e48;
} else {
c.cycles += 1;
c.g[V1][0] = c.g[V1][0] | 0x8;
break 'b_377e48;
}
}
'b_377e60: {
c.cycles += 5;
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6b84));
c.c_lt_s(0, 1);
let t = !c.fcc();
c.mov_s(2, 0);
if t {
break 'b_377e60;
} else {
c.cycles += 1;
c.g[V1][0] = c.g[V1][0] | 0x4;
break 'b_377e60;
}
}
'b_377ea8: {
'b_377e80: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x2c));
c.mtc1(0, 0);
c.c_lt_s(1, 0);
let t = !c.fcc();
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x28));
if t {
break 'b_377e80;
} else {
c.cycles += 2;
let t = true;
c.g[V1][0] = c.g[V1][0] ^ 0x2f;
if t {
break 'b_377ea8;
} else {
break 'b_377e80;
}
}
}
'b_377e94: {
c.cycles += 5;
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6b78));
c.c_lt_s(0, 1);
if c.fcc() {
c.g[V1][0] = c.g[V1][0] | 0x20;
break 'b_377e94;
} else {
break 'b_377e94;
}
}
c.cycles += 5;
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6b88));
c.c_lt_s(1, 0);
if c.fcc() {
c.g[V1][0] = c.g[V1][0] | 0x10;
break 'b_377ea8;
} else {
break 'b_377ea8;
}
}
'b_377ec4: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6b70));
c.c_lt_s(1, 0);
let t = !c.fcc();
c.g[V0][0] = 0u64;
if t {
break 'b_377ec4;
} else {
c.cycles += 1;
c.g[V0][0] = 0x2;
break 'b_377ec4;
}
}
'b_377ed4: {
c.cycles += 4;
c.c_lt_s(4, 1);
if c.fcc() {
c.g[V0][0] = c.g[V0][0] | 0x1;
break 'b_377ed4;
} else {
break 'b_377ed4;
}
}
'b_377ee8: {
c.cycles += 5;
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
c.c_lt_s(0, 3);
if c.fcc() {
c.g[V0][0] = c.g[V0][0] | 0x8;
break 'b_377ee8;
} else {
break 'b_377ee8;
}
}
'b_377ef8: {
c.cycles += 4;
c.c_lt_s(2, 0);
if c.fcc() {
c.g[V0][0] = c.g[V0][0] | 0x4;
break 'b_377ef8;
} else {
break 'b_377ef8;
}
}
'b_377f40: {
'b_377f18: {
c.cycles += 6;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x3c));
c.mtc1(0, 0);
c.c_lt_s(1, 0);
let t = !c.fcc();
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x38));
if t {
break 'b_377f18;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = c.g[V0][0] ^ 0x2f;
if t {
break 'b_377f40;
} else {
break 'b_377f18;
}
}
}
'b_377f2c: {
c.cycles += 5;
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6b78));
c.c_lt_s(0, 1);
if c.fcc() {
c.g[V0][0] = c.g[V0][0] | 0x20;
break 'b_377f2c;
} else {
break 'b_377f2c;
}
}
c.cycles += 5;
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x6b88));
c.c_lt_s(1, 0);
if c.fcc() {
c.g[V0][0] = c.g[V0][0] | 0x10;
break 'b_377f40;
} else {
break 'b_377f40;
}
}
'b_37817c: {
c.cycles += 3;
c.g[V0][0] = c.g[V1][0] & c.g[V0][0];
if c.g[V0][0] != 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
break 'b_37817c;
} else {
'b_37816c: {
c.cycles += 104;
c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a00));
c.g[T3][0] = 0x30000000;
c.g[A0][0] = 0x64000000;
c.g[A0][0] = c.g[A0][0] << 32;
c.g[A0][0] = c.g[A0][0] | 0x8001;
c.por(2, 0, 0);
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] | 0x5;
c.g[A1][0] = c.g[T2][0] | c.g[T3][0];
c.g[V1][0] = 0x50000000;
c.g[V1][0] = c.g[V1][0] | 0x4;
c.g[V1][0] = c.g[V1][0] << 32;
c.g[A2][0] = 0x520000;
c.g[A2][0] = c.g[A2][0] | 0x5120;
c.g[A3][0] = 0x5e;
c.sd((c.g[A1][0] as u32).wrapping_add(0x20), c.g[A0][0]);
c.g[T0][0] = 0xfff0000;
c.sd((c.g[A1][0] as u32).wrapping_add(0x28), c.g[A2][0]);
c.g[T0][0] = c.g[T0][0] | 0xffff;
c.sd((c.g[A1][0] as u32).wrapping_add(0x30), c.g[A3][0]);
c.sq(2, (c.g[A1][0] as u32));
c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.sd((c.g[A1][0] as u32).wrapping_add(0x10), 0u64);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[S4][0] as u32));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[AT][0] = 0x43000000;
c.mtc1(1, 4);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[A1][0] as u32).wrapping_add(0x38), c.g[V1][0]);
c.mfc1(2, 0);
c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x8));
c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0xc));
c.g[A3][0] = c.g[V0][0];
c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x4));
c.mul_s(1, 1, 4);
c.lwc1(3, (c.g[S3][0] as u32));
c.mul_s(2, 2, 4);
c.mul_s(0, 0, 4);
c.g[A3][0] = c.g[A3][0] << 32;
c.mul_s(3, 3, 4);
c.cvt_w_s(4, 1);
c.mfc1(2, 4);
c.cvt_w_s(1, 0);
c.mfc1(6, 1);
c.cvt_w_s(1, 2);
c.mfc1(3, 1);
c.cvt_w_s(0, 3);
c.mfc1(4, 0);
c.g[V0][0] = c.g[V0][0] << 8;
c.g[V0][0] = c.g[A2][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 16;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A0][0] = c.g[A0][0] << 24;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
c.sd((c.g[A1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x2c));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x28));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x24));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.mul_s(0, 0, 1);
c.cvt_w_s(1, 3);
c.mfc1(2, 1);
c.cvt_w_s(1, 2);
c.mfc1(4, 1);
c.cvt_w_s(1, 0);
c.mfc1(3, 1);
c.g[V0][0] = c.g[V0][0] << 16;
c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[A1][0] as u32).wrapping_add(0x48), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lw((c.g[S5][0] as u32));
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[A1][0] as u32).wrapping_add(0x50), c.g[V0][0]);
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x3c));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x38));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
c.mul_s(0, 0, 1);
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.cvt_w_s(1, 3);
c.mfc1(2, 1);
c.cvt_w_s(1, 2);
c.mfc1(4, 1);
c.cvt_w_s(1, 0);
c.mfc1(3, 1);
c.g[V0][0] = c.g[V0][0] << 16;
c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[A1][0] as u32).wrapping_add(0x58), c.g[V0][0]);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x60));
c.g[T1][0] = c.g[A1][0] & c.g[T0][0];
c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18f0));
c.sw((c.g[S0][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
if t {
break 'b_37816c;
} else {
c.cycles += 32;
c.g[A1][0] = 0x60000;
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.g[A1][0] | 0x8000;
c.g[V1][0] = 0xffffffffffffffe0;
c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
c.g[A2][0] = c.g[A2][0] | c.g[T3][0];
c.g[V0][0] = c.g[V0][0] & 0x1f;
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
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T2][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_37816c;
}
}
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.sw((c.g[S0][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
break 'b_37817c;
}
}
c.cycles += 8;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
c.pc = t; return Ok(());
    });
}

/// `cPS2GridMesh_Draw(self, mat4x4* m, u16 tex, int flag)` (0x00385c10): flip the
/// double buffer when requested, update the view matrices when invalid, scale the
/// matrix by the mesh range, begin a sprite-stream block, emit the model header,
/// close the block with this buffer's DMA tag and queue the draw in the
/// deferred-draw ring. Converted from the generated body; every register, cycle and
/// call kept.
#[allow(unused_labels, unused_assignments, unreachable_code, unused_variables)]
pub fn cps2_grid_mesh_draw(c: &mut Cpu) {
    guest(c, |c| {
'b_385c58: {
c.cycles += 13;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
c.g[S0][0] = c.g[A2][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
c.g[S2][0] = c.g[A0][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[S3][0] = c.g[A1][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1a4));
let t = c.g[V0][0] == 0u64;
c.g[S4][0] = c.g[A3][0];
if t {
break 'b_385c58;
} else {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1a0));
c.sw((c.g[S2][0] as u32).wrapping_add(0x1a4), 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
c.g[V0][0] = c.g[V0][0] & 0x1;
c.sw((c.g[S2][0] as u32).wrapping_add(0x1a0), c.g[V0][0]);
break 'b_385c58;
}
}
'b_385c74: {
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c4));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6b90));
if c.g[V0][0] != 0u64 {
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c4));
break 'b_385c74;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x00385c70);
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x00385c70)?;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c4));
break 'b_385c74;
}
}
'b_385e88: {
c.cycles += 99;
c.g[A1][0] = 0xfffffffffffffc3f;
c.g[A2][0] = 0xffffffffc0000000;
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
c.g[V0][0] = c.g[V0][0] | 0xc0;
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c4));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c4));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[S0][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c4));
c.g[T1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5a00));
c.lqc2(1, (c.g[S3][0] as u32));
c.lqc2(2, (c.g[S3][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[S3][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[S3][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[SP][0] as u32));
c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x194));
c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 1);
c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x48));
c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4c));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x40));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
c.lqc2(1, (c.g[S0][0] as u32));
c.lqc2(4, (c.g[S3][0] as u32));
c.lqc2(5, (c.g[S3][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[S3][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[S3][0] as u32).wrapping_add(0x30));
c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
c.sqc2(8, (c.g[V0][0] as u32));
c.sqc2(9, (c.g[V0][0] as u32).wrapping_add(0x10));
c.sqc2(10, (c.g[V0][0] as u32).wrapping_add(0x20));
c.sqc2(11, (c.g[V0][0] as u32).wrapping_add(0x30));
c.lqc2(1, (c.g[V0][0] as u32));
c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[S1][0] as u32));
c.sqc2(2, (c.g[S1][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[S1][0] as u32).wrapping_add(0x30));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[A1][0] = c.g[T1][0];
c.g[A2][0] = 0xffffffffffffffff;
c.g[RA][0] = sx(0x00385d68);
c.g[A3][0] = 0u64;
Callee { addr: 0x0038f460, f: ee::s12::SprStream_BeginBlock }.call(c, 0x00385d68)?;
c.g[T0][0] = c.g[S4][0];
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c4));
c.g[V1][0] = 0x20;
c.g[T2][0] = 0x22;
if c.g[T0][0] == 0 { c.g[T2][0] = c.g[V1][0]; }
c.g[T1][0] = 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[V0][0]);
c.g[A1][0] = c.g[S1][0];
c.g[A3][0] = c.g[S0][0];
c.g[A2][0] = c.g[SP][0];
c.g[RA][0] = sx(0x00385d98);
c.g[T3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
Callee { addr: 0x0037e120, f: ee::s12::PS2GraphicsMan_EmitModelHeader }.call(c, 0x00385d98)?;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1a0));
c.g[A0][0] = 0x50000000;
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[A2][0] = 0x4;
c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = c.lwu((c.g[V1][0] as u32).wrapping_add(0x1ac));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.sd((c.g[A1][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[A1][0] = c.g[V0][0];
c.g[RA][0] = sx(0x00385de0);
c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[V0][0]);
Callee { addr: 0x0038f668, f: ee::s12::SprStream_EndBlockAligned }.call(c, 0x00385de0)?;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c4));
c.g[T1][0] = c.g[V0][0];
c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18f0));
c.g[T2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
if t {
break 'b_385e88;
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
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T2][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c4));
break 'b_385e88;
}
}
c.cycles += 10;
c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x10));
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
c.sw((c.g[V1][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
c.pc = t; return Ok(());
    });
}

/// `PS2GraphicsMan_RestoreRenderTarget(self)` (0x0037d450, slot `RestoreRenderTarget`):
/// write the GIF packet that sets the frame/zbuf/scissor/test registers back to the
/// saved render-target values, then queue it in the deferred-draw ring. Leaf;
/// returns 0. Converted from the generated body; every register and cycle kept.
#[allow(unused_labels, unused_assignments, unreachable_code, unused_variables)]
pub fn restore_render_target(c: &mut Cpu) {
    guest(c, |c| {
'b_37d650: {
c.cycles += 96;
c.g[T6][0] = c.g[A0][0];
c.g[T8][0] = 0x30000000;
c.g[T7][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] | 0x6;
c.g[V1][0] = 0x8800;
c.g[V1][0] = c.g[V1][0] << 45;
c.g[A0][0] = 0x50000000;
c.g[A0][0] = c.g[A0][0] | 0x5;
c.g[A0][0] = c.g[A0][0] << 32;
c.g[A2][0] = c.g[T7][0] | c.g[T8][0];
c.g[A1][0] = 0x10000000;
c.g[A1][0] = c.g[A1][0] << 32;
c.g[A1][0] = c.g[A1][0] | 0x8004;
c.g[A3][0] = 0xe;
c.sd((c.g[A2][0] as u32), c.g[V0][0]);
c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[T2][0] = 0x4c;
c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[A0][0]);
c.g[T3][0] = 0x40;
c.sd((c.g[A2][0] as u32).wrapping_add(0x20), c.g[A1][0]);
c.g[T4][0] = 0x41;
c.sd((c.g[A2][0] as u32).wrapping_add(0x28), c.g[A3][0]);
c.g[T0][0] = 0x800;
c.sd((c.g[A2][0] as u32).wrapping_add(0x8), 0u64);
c.g[T5][0] = 0x18;
c.g[T1][0] = 0xfff0000;
c.g[V0][0] = c.lwu((c.g[T6][0] as u32).wrapping_add(0x5a50));
c.g[T1][0] = c.g[T1][0] | 0xffff;
c.g[V1][0] = c.lwu((c.g[T6][0] as u32).wrapping_add(0x5a84));
c.g[A0][0] = c.lwu((c.g[T6][0] as u32).wrapping_add(0x5a3c));
c.g[V0][0] = c.g[V0][0] << 16;
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[A2][0] as u32).wrapping_add(0x38), c.g[T2][0]);
c.g[A0][0] = c.g[A0][0] << 24;
c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
c.sd((c.g[A2][0] as u32).wrapping_add(0x30), c.g[V1][0]);
c.g[V0][0] = c.lhu((c.g[T6][0] as u32).wrapping_add(0x5900));
c.g[A1][0] = c.lhu((c.g[T6][0] as u32).wrapping_add(0x5904));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[A0][0] = c.lhu((c.g[T6][0] as u32).wrapping_add(0x5902));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A3][0] = c.lhu((c.g[T6][0] as u32).wrapping_add(0x5906));
c.g[V1][0] = c.lhu((c.g[T6][0] as u32).wrapping_add(0x5900));
c.g[V0][0] = c.g[V0][0] << 16;
c.g[A1][0] = c.lhu((c.g[T6][0] as u32).wrapping_add(0x5902));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[A1][0] = c.g[A1][0] << 32;
c.g[A0][0] = c.g[A0][0] << 48;
c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
c.sd((c.g[A2][0] as u32).wrapping_add(0x48), c.g[T3][0]);
c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
c.sd((c.g[A2][0] as u32).wrapping_add(0x40), c.g[V1][0]);
c.g[V0][0] = c.lhu((c.g[T6][0] as u32).wrapping_add(0x5900));
c.g[A1][0] = c.lhu((c.g[T6][0] as u32).wrapping_add(0x5904));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[A0][0] = c.lhu((c.g[T6][0] as u32).wrapping_add(0x5902));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A3][0] = c.lhu((c.g[T6][0] as u32).wrapping_add(0x5906));
c.g[V1][0] = c.lhu((c.g[T6][0] as u32).wrapping_add(0x5900));
c.g[V0][0] = c.g[V0][0] << 16;
c.g[A1][0] = c.lhu((c.g[T6][0] as u32).wrapping_add(0x5902));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[A1][0] = c.g[A1][0] << 32;
c.g[A0][0] = c.g[A0][0] << 48;
c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
c.sd((c.g[A2][0] as u32).wrapping_add(0x58), c.g[T4][0]);
c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
c.sd((c.g[A2][0] as u32).wrapping_add(0x50), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x5a30));
c.g[V1][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x5a34));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.sd((c.g[A2][0] as u32).wrapping_add(0x68), c.g[T5][0]);
c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[V0][0] = c.g[V0][0] << 32;
c.g[T0][0] = sx((c.g[T0][0] as u32) << 4);
c.g[T0][0] = c.g[T0][0] << 32;
c.g[V0][0] = c.g[V0][0] >> 32;
c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
c.sd((c.g[A2][0] as u32).wrapping_add(0x60), c.g[V0][0]);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x70));
c.g[A3][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x18f0));
c.g[T1][0] = c.g[A2][0] & c.g[T1][0];
c.g[T0][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0xe84));
c.sw((c.g[T6][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = 0x60000;
if t {
break 'b_37d650;
} else {
c.cycles += 32;
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.g[A1][0] | 0x8000;
c.g[V1][0] = 0xffffffffffffffe0;
c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
c.g[A2][0] = c.g[A2][0] | c.g[T8][0];
c.g[V0][0] = c.g[V0][0] & 0x1f;
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
c.g[V1][0] = 0x1;
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T7][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), c.g[V1][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
break 'b_37d650;
}
}
c.cycles += 5;
c.g[V1][0] = c.lw((c.g[T6][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = 0u64;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.sw((c.g[T6][0] as u32).wrapping_add(0x5a00), c.g[V1][0]);
c.pc = t; return Ok(());
    });
}

/// `PS2GraphicsMan_DrawLineStrip(self, int n, vtx* v)` (0x00381ad0, slot
/// `DrawLineStrip`, the reference's `RenderLineStrip`): project each vertex, compute
/// its clip code, and feed the strip builder (`LineStrip_Init/Begin/AddVertex/
/// Finish`), clipping segments that cross the viewport with `ClipLineSegment`.
/// Converted from the generated body; every register, cycle and call kept.
#[allow(unused_labels, unused_assignments, unreachable_code, unused_variables)]
pub fn ps2_graphics_man_v73(c: &mut Cpu) {
    guest(c, |c| {
'b_381b18: {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x190));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
c.g[S4][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
c.sd((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[RA][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[A1][0]);
c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x6b90));
let t = c.g[V0][0] != 0u64;
c.g[S0][0] = c.g[A2][0];
if t {
break 'b_381b18;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x00381b18);
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x00381b18)?;
break 'b_381b18;
}
}
'b_381ed0: {
c.cycles += 7;
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = sx(0x00381b24);
c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5a00));
Callee { addr: 0x00384d98, f: ee::s12::LineStrip_Init }.call(c, 0x00381b24)?;
c.g[S1][0] = 0u64;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
let t = (c.g[V0][0] as i64) <= 0;
c.g[T1][0] = 0xffffffffffffffff;
if t {
break 'b_381ed0;
} else {
c.cycles += 3;
c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6b30));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd4), c.g[V1][0]);
'l_381b40: loop {
'b_381bb4: {
c.cycles += 28;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd4));
c.g[S3][0] = 0u64;
c.lqc2(8, (c.g[S0][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[V0][0] as u32));
c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[V0][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[V0][0] as u32).wrapping_add(0x30));
c.vu0().mula(0b1111, 4, Src::Bc(8, X));
c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 3);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lwc1(4, (c.g[S4][0] as u32).wrapping_add(0x6b70));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x3c));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
c.div_s(3, 3, 0);
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
c.mul_s(1, 1, 3);
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x3c));
c.mul_s(2, 2, 3);
c.c_lt_s(1, 4);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
let t = !c.fcc();
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
if t {
break 'b_381bb4;
} else {
c.cycles += 1;
c.g[S3][0] = 0x2;
break 'b_381bb4;
}
}
'b_381bc8: {
c.cycles += 5;
c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x6b80));
c.c_lt_s(0, 1);
if c.fcc() {
c.g[S3][0] = c.g[S3][0] | 0x1;
break 'b_381bc8;
} else {
break 'b_381bc8;
}
}
'b_381bdc: {
c.cycles += 5;
c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x6b74));
c.c_lt_s(2, 0);
if c.fcc() {
c.g[S3][0] = c.g[S3][0] | 0x8;
break 'b_381bdc;
} else {
break 'b_381bdc;
}
}
'b_381bf0: {
c.cycles += 5;
c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x6b84));
c.c_lt_s(0, 2);
if c.fcc() {
c.g[S3][0] = c.g[S3][0] | 0x4;
break 'b_381bf0;
} else {
break 'b_381bf0;
}
}
'b_381c34: {
'b_381c0c: {
c.cycles += 5;
c.mtc1(0, 0);
c.c_lt_s(3, 0);
let t = !c.fcc();
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x38));
if t {
break 'b_381c0c;
} else {
c.cycles += 2;
let t = true;
c.g[S3][0] = c.g[S3][0] ^ 0x2f;
if t {
break 'b_381c34;
} else {
break 'b_381c0c;
}
}
}
'b_381c20: {
c.cycles += 5;
c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x6b78));
c.c_lt_s(0, 1);
if c.fcc() {
c.g[S3][0] = c.g[S3][0] | 0x20;
break 'b_381c20;
} else {
break 'b_381c20;
}
}
c.cycles += 5;
c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x6b88));
c.c_lt_s(1, 0);
if c.fcc() {
c.g[S3][0] = c.g[S3][0] | 0x10;
break 'b_381c34;
} else {
break 'b_381c34;
}
}
'b_381eb4: {
'b_381eb0: {
'b_381df4: {
'b_381de4: {
'b_381d38: {
c.cycles += 2;
let t = c.g[S3][0] != 0u64;

if t {
break 'b_381d38;
} else {
'b_381c5c: {
c.cycles += 2;
let t = c.g[T1][0] != 0u64;
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
if t {
break 'b_381c5c;
} else {
c.cycles += 6;
c.g[A1][0] = c.g[S0][0];
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = sx(0x00381c54);
c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
Callee { addr: 0x00384fd0, f: ee::s12::LineStrip_AddVertex }.call(c, 0x00381c54)?;
let t = true;
c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
if t {
break 'b_381eb0;
} else {
break 'b_381c5c;
}
}
}
'b_381d1c: {
c.cycles += 2;
let t = c.g[S1][0] == 0u64;
c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
if t {
break 'b_381d1c;
} else {
c.cycles += 11;
c.lq(3, (c.g[SP][0] as u32));
c.g[V0][0] = 0x1;
c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
c.g[A0][0] = 0xffffffffffffffff;
c.g[V0][0] = 0x1;
c.g[A0][0] = 0xffffffffffffffff;
'l_381c90: loop {
c.cycles += 7;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
let t = c.g[V0][0] != c.g[A0][0];
c.g[A3][0] = c.g[S1][0];
if t {
if event_due(c) { deliver_event_here(c, 0x00381c90); } continue 'l_381c90;
} else {
c.cycles += 28;
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(8, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A0][0] = c.g[S4][0];
c.lq(3, (c.g[S0][0] as u32).wrapping_sub(0x30));
c.g[A1][0] = c.g[S2][0];
c.g[A2][0] = c.g[S5][0];
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[S0][0] as u32).wrapping_sub(0x20));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(3, (c.g[S0][0] as u32).wrapping_sub(0x10));
c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[T1][0]);
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(8, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.lq(2, (c.g[S0][0] as u32));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(3, (c.g[S0][0] as u32).wrapping_add(0x10));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[S3][0]);
c.g[RA][0] = sx(0x00381d00);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
Callee { addr: 0x00383a10, f: ee::s12::PS2GraphicsMan_ClipLineSegment }.call(c, 0x00381d00)?;
c.g[A1][0] = c.g[S2][0];
c.g[A2][0] = c.g[S5][0];
c.g[RA][0] = sx(0x00381d10);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
Callee { addr: 0x003850a8, f: ee::s12::LineStrip_Begin }.call(c, 0x00381d10)?;
c.g[A1][0] = c.g[S0][0];
let t = true;
c.g[A2][0] = c.g[FP][0];
if t {
break 'b_381de4;
} else {
break 'b_381d1c;
}
}
}
}
}
c.cycles += 7;
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A1][0] = c.g[S0][0];
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[RA][0] = sx(0x00381d30);
c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
Callee { addr: 0x003850a8, f: ee::s12::LineStrip_Begin }.call(c, 0x00381d30)?;
let t = true;
c.g[S7][0] = 0x1;
if t {
break 'b_381eb0;
} else {
break 'b_381d38;
}
}
}
c.cycles += 2;
let t = c.g[T1][0] != 0u64;
c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
if t {
break 'b_381df4;
} else {
c.cycles += 12;
c.lq(3, (c.g[SP][0] as u32));
c.g[V0][0] = 0x1;
c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
c.g[A0][0] = 0xffffffffffffffff;
c.g[V0][0] = 0x1;
c.g[A0][0] = 0xffffffffffffffff;
'l_381d70: loop {
c.cycles += 7;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
let t = c.g[V0][0] != c.g[A0][0];
c.g[A1][0] = c.g[S1][0];
if t {
if event_due(c) { deliver_event_here(c, 0x00381d70); } continue 'l_381d70;
} else {
c.cycles += 22;
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(8, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A2][0] = c.g[S2][0];
c.lq(3, (c.g[S0][0] as u32).wrapping_sub(0x30));
c.g[A0][0] = c.g[S4][0];
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[S0][0] as u32).wrapping_sub(0x20));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(3, (c.g[S0][0] as u32).wrapping_sub(0x10));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[T1][0]);
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(8, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(2, (c.g[S0][0] as u32));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(3, (c.g[S0][0] as u32).wrapping_add(0x10));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), c.g[S3][0]);
c.g[RA][0] = sx(0x00381ddc);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
Callee { addr: 0x00383a10, f: ee::s12::PS2GraphicsMan_ClipLineSegment }.call(c, 0x00381ddc)?;
c.g[A1][0] = c.g[FP][0];
c.g[A2][0] = c.g[S5][0];
break 'b_381de4;
}
}
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00381dec);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
Callee { addr: 0x00384fd0, f: ee::s12::LineStrip_AddVertex }.call(c, 0x00381dec)?;
let t = true;
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
if t {
break 'b_381eb4;
} else {
break 'b_381df4;
}
}
c.cycles += 3;
c.g[V0][0] = c.g[T1][0] & c.g[S3][0];
let t = c.g[V0][0] != 0u64;
c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
if t {
break 'b_381eb0;
} else {
c.cycles += 8;
c.lq(3, (c.g[SP][0] as u32));
c.g[V0][0] = 0x1;
c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[A0][0] = 0xffffffffffffffff;
c.g[V0][0] = 0x1;
c.g[A0][0] = 0xffffffffffffffff;
'l_381e20: loop {
c.cycles += 7;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
let t = c.g[V0][0] != c.g[A0][0];
c.g[A1][0] = c.g[S1][0];
if t {
if event_due(c) { deliver_event_here(c, 0x00381e20); } continue 'l_381e20;
} else {
c.cycles += 22;
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[A0][0] = c.g[S4][0];
c.lq(8, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(3, (c.g[S0][0] as u32).wrapping_sub(0x30));
c.g[A2][0] = c.g[S2][0];
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[S0][0] as u32).wrapping_sub(0x20));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(3, (c.g[S0][0] as u32).wrapping_sub(0x10));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), c.g[T1][0]);
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(8, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(2, (c.g[S0][0] as u32));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(3, (c.g[S0][0] as u32).wrapping_add(0x10));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(2, (c.g[S0][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), c.g[S3][0]);
c.g[RA][0] = sx(0x00381e8c);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
Callee { addr: 0x00383a10, f: ee::s12::PS2GraphicsMan_ClipLineSegment }.call(c, 0x00381e8c)?;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = c.g[S1][0];
if t {
break 'b_381eb0;
} else {
c.cycles += 7;
c.g[A2][0] = c.g[S2][0];
c.g[RA][0] = sx(0x00381ea0);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
Callee { addr: 0x003850a8, f: ee::s12::LineStrip_Begin }.call(c, 0x00381ea0)?;
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
c.g[RA][0] = sx(0x00381eb0);
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
Callee { addr: 0x00384fd0, f: ee::s12::LineStrip_AddVertex }.call(c, 0x00381eb0)?;
break 'b_381eb0;
}
}
}
}
}
c.cycles += 1;
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
break 'b_381eb4;
}
c.cycles += 7;
c.g[S1][0] = c.g[S7][0];
c.g[T1][0] = c.g[S3][0];
c.sq(3, (c.g[SP][0] as u32));
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S0][0] = c.g[S6][0];
if t {
if event_due(c) { deliver_event_here(c, 0x00381b40); } continue 'l_381b40;
} else {
break 'b_381ed0;
}
}
}
}
c.cycles += 15;
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = sx(0x00381edc);
c.g[A1][0] = 0x2;
Callee { addr: 0x00384e50, f: ee::s12::LineStrip_Finish }.call(c, 0x00381edc)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xf0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, run_generated, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The graphics manager (`a0`), the GS register block, the output packet
    /// stream, the deferred-draw ring, the projection matrix, the argument
    /// blocks and the vertex array, all inside `DATA`.
    const OC1481_WORLD: u32 = DATA + 0x1_0000;
    const OC1481_REGS: u32 = DATA + 0x2_0000;
    const OC1481_OUT: u32 = DATA + 0x3_0000;
    const OC1481_RING: u32 = DATA + 0x4_0000;
    const OC1481_PROJ: u32 = DATA + 0xb_0000;
    const OC1481_MESH: u32 = DATA + 0xc_0000;
    const OC1481_ARGS: u32 = DATA + 0xd_0000;
    const OC1481_MAT: u32 = OC1481_ARGS;
    const OC1481_POS: u32 = OC1481_ARGS + 0x100;
    const OC1481_SIZE: u32 = OC1481_ARGS + 0x110;
    const OC1481_COL: u32 = OC1481_ARGS + 0x120;
    const OC1481_UV0: u32 = OC1481_ARGS + 0x140;
    const OC1481_UV1: u32 = OC1481_ARGS + 0x150;
    const OC1481_VTX: u32 = OC1481_ARGS + 0x200;
    /// Gate word of this module's callee fakes, the stream object and the
    /// addresses the fakes return.
    const OC1481_CTRL: u32 = DATA + 0xe_0000;
    const OC1481_STREAM: u32 = DATA + 0xe_1000;
    const OC1481_BEGIN: u32 = DATA + 0xf_0000;
    const OC1481_END: u32 = DATA + 0xf_8000;
    const OC1481_MAGIC: u32 = 0x1481_f00d;

    fn oc1481_f(r: &mut Rng, range: f32) -> f32 {
        f32::from_bits(r.f32(range))
    }

    fn oc1481_put_f32s(c: &mut Cpu, addr: u32, v: &[f32]) {
        for (i, x) in v.iter().enumerate() {
            c.bus.write32(addr + 4 * i as u32, x.to_bits());
        }
    }

    /// A row-major 4x4 matrix close to the identity (w stays near 1, or goes
    /// negative when `behind`), so projected points land around the clip box.
    fn oc1481_put_matrix(c: &mut Cpu, r: &mut Rng, addr: u32, behind: bool) {
        let mut m = [0f32; 16];
        for (i, x) in m.iter_mut().enumerate() {
            *x = oc1481_f(r, 0.25) + if i % 5 == 0 { 1.0 } else { 0.0 };
        }
        if behind {
            m[15] = -1.0;
        }
        oc1481_put_f32s(c, addr, &m);
    }

    /// Fixture of the draw-path functions: the manager (view-matrices-valid
    /// flag, clip box, framebuffer rectangle, stream cursor, deferred ring that
    /// is free or full, GS register block) and the projection matrix.
    fn oc1481_world(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let w = OC1481_WORLD;
        let have_view = seed & 1 == 1;
        let full = (seed / 2) % 5 == 0;
        for off in (0x5780..0x6c00).step_by(4) {
            let v = r.f32(4.0);
            c.bus.write32(w + off, v);
        }
        oc1481_put_matrix(c, r, w + 0x6b30, seed % 7 == 0);
        // Clip box: min x/y/z at +0x6b70.., max x/y/z at +0x6b80...
        let k = (r.next_u32() % 4) as f32 * 0.5;
        oc1481_put_f32s(c, w + 0x6b70, &[-1.0 - k, -1.0 - k, -10.0]);
        oc1481_put_f32s(c, w + 0x6b80, &[1.0 + k, 1.0 + k, 10.0]);
        // Framebuffer rectangle and render-state words.
        for (off, m) in [
            (0x5900, 0x280),
            (0x5902, 0x1c0),
            (0x5904, 0x280),
            (0x5906, 0x1c0),
        ] {
            let v = r.next_u32() % m;
            c.bus.write16(w + off, v as u16);
        }
        for off in [0x5a30, 0x5a34, 0x5a3c, 0x5a50, 0x5a84] {
            let v = r.next_u32() & 0xfff;
            c.bus.write32(w + off, v);
        }
        c.bus.write32(w + 0xe84, OC1481_REGS);
        c.bus.write32(w + 0x5a00, OC1481_OUT + (r.next_u32() & 0xf0));
        c.bus.write32(w + 0x18f0, OC1481_RING);
        c.bus.write32(w + 0x6b90, have_view as u32);
        c.bus.write32(w + 0x13e4, OC1481_PROJ);
        for k in 0..5 {
            let v = r.next_u32();
            c.bus.write32(OC1481_REGS + 4 * k, v);
        }
        c.bus.write16(OC1481_REGS + 0x10, r.next_u32() as u16);
        // Deferred ring: entry count, and a short matrix-record list so the
        // matrix interning in the view update stays bounded.
        let n = if full { 2700 } else { r.next_u32() % 4 };
        c.bus.write32(OC1481_RING, n);
        for k in 0..2 {
            let v = r.next_u32();
            c.bus.write32(OC1481_RING + 0x6_9cc0 + 4 * k, v);
        }
        c.bus.write32(OC1481_RING + 0x6_9cc8, r.next_u32() % 3);
        let proj: Vec<f32> = (0..16).map(|_| oc1481_f(r, 3.0)).collect();
        oc1481_put_f32s(c, OC1481_PROJ, &proj);
        c.bus.write32(GAME_GP + 0x1384, OC1481_STREAM);
    }

    /// A machine with the world, `a0` = the world and the entry `pc` set.
    fn oc1481_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        let mut r = fakes::machine(c, seed, salt, 0);
        oc1481_world(c, &mut r, seed);
        c.g[A0][0] = OC1481_WORLD as u64;
        c.pc = entry;
        r
    }

    fn oc1481_paths(
        generated: fn(&mut Cpu),
        seeds: u32,
        min: usize,
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, generated);
            totals.insert(c.cycles - before);
        }
        assert!(totals.len() >= min, "only {} distinct paths", totals.len());
    }

    fn oc1481_line_shape_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1481_setup(c, seed, 0x1481_75, 0x0038_1f10);
        oc1481_put_matrix(c, &mut r, OC1481_MAT, false);
        let col: Vec<f32> = (0..4).map(|_| oc1481_f(&mut r, 1.5)).collect();
        oc1481_put_f32s(c, OC1481_COL, &col);
        c.g[A1][0] = OC1481_MAT as u64;
        c.g[A2][0] = OC1481_COL as u64;
        c.g[A3][0] = (r.next_u32() & 0x7fff) as u64;
    }

    /// `PS2GraphicsMan_DrawLineShape` (the GIF packet with a 4x4 matrix, view
    /// update when invalid, ring entry or full ring) against the generated body.
    #[test]
    fn ps2_graphics_man_v75_matches() {
        check(
            ee::s12::f_00381f10__generated,
            ps2_graphics_man_v75,
            240,
            &oc1481_line_shape_setup,
        );
    }

    /// The fixture reaches the valid/invalid view and free/full ring paths.
    #[test]
    fn ps2_graphics_man_v75_paths() {
        oc1481_paths(ee::s12::f_00381f10__generated, 240, 4, &oc1481_line_shape_setup);
    }

    fn oc1481_restore_setup(c: &mut Cpu, seed: u32) {
        let _ = oc1481_setup(c, seed, 0x1481_37, 0x0037_d450);
    }

    /// `PS2GraphicsMan_RestoreRenderTarget` (restore packet and ring entry)
    /// against the generated body.
    #[test]
    fn restore_render_target_matches() {
        check(
            ee::s12::f_0037d450__generated,
            restore_render_target,
            240,
            &oc1481_restore_setup,
        );
    }

    /// The fixture reaches the free-ring and full-ring paths.
    #[test]
    fn restore_render_target_paths() {
        oc1481_paths(ee::s12::f_0037d450__generated, 240, 2, &oc1481_restore_setup);
    }

    fn oc1481_sprite_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1481_setup(c, seed, 0x1481_78, 0x0037_7cf0);
        let pos = [
            oc1481_f(&mut r, 3.0),
            oc1481_f(&mut r, 3.0),
            oc1481_f(&mut r, 3.0),
            1.0,
        ];
        oc1481_put_f32s(c, OC1481_POS, &pos);
        let size = [oc1481_f(&mut r, 0.5), oc1481_f(&mut r, 0.5)];
        oc1481_put_f32s(c, OC1481_SIZE, &size);
        let col: Vec<f32> = (0..4).map(|_| oc1481_f(&mut r, 1.5)).collect();
        oc1481_put_f32s(c, OC1481_COL, &col);
        for k in 0..2 {
            let a = r.next_u32();
            let b = r.next_u32();
            c.bus.write32(OC1481_UV0 + 4 * k, a);
            c.bus.write32(OC1481_UV1 + 4 * k, b);
        }
        c.g[A1][0] = OC1481_POS as u64;
        c.g[A2][0] = OC1481_SIZE as u64;
        c.g[A3][0] = OC1481_COL as u64;
        c.g[T0][0] = OC1481_UV0 as u64;
        c.g[T1][0] = OC1481_UV1 as u64;
    }

    /// `PS2GraphicsMan_DrawSprite3D` (project, clip-reject or write the sprite
    /// packet and ring entry) against the generated body.
    #[test]
    fn ps2_graphics_man_v78_matches() {
        check(
            ee::s12::f_00377cf0__generated,
            ps2_graphics_man_v78,
            300,
            &oc1481_sprite_setup,
        );
    }

    /// The fixture reaches the clip-reject exit as well as the packet paths.
    #[test]
    fn ps2_graphics_man_v78_paths() {
        oc1481_paths(ee::s12::f_00377cf0__generated, 300, 5, &oc1481_sprite_setup);
    }

    fn oc1481_strip_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc1481_setup(c, seed, 0x1481_73, 0x0038_1ad0);
        let n = (seed / 4) % 6;
        for i in 0..6u32 {
            let v = OC1481_VTX + 0x30 * i;
            for k in 0..8 {
                let x = r.next_u32();
                c.bus.write32(v + 4 * k, x);
            }
            for k in 0..3 {
                let x = oc1481_f(&mut r, 3.0);
                c.bus.write32(v + 0x20 + 4 * k, x.to_bits());
            }
            c.bus.write32(v + 0x2c, 1.0f32.to_bits());
        }
        c.g[A1][0] = n as u64;
        c.g[A2][0] = OC1481_VTX as u64;
    }

    /// `PS2GraphicsMan_DrawLineStrip` (project and clip-code each vertex, feed
    /// the real strip builder, clip crossing segments) against the generated body.
    #[test]
    fn ps2_graphics_man_v73_matches() {
        check(
            ee::s12::f_00381ad0__generated,
            ps2_graphics_man_v73,
            300,
            &oc1481_strip_setup,
        );
    }

    /// The fixture reaches many distinct vertex-code sequences (inside,
    /// outside, entering, leaving, crossing, behind the camera).
    #[test]
    fn ps2_graphics_man_v73_paths() {
        oc1481_paths(ee::s12::f_00381ad0__generated, 300, 12, &oc1481_strip_setup);
    }

    // Gated fakes of the sprite-stream callees of the grid mesh draw: with the
    // gate word off they run what the slot held before.
    static OC1481_PREV_BEGIN: AtomicUsize = AtomicUsize::new(0);
    static OC1481_PREV_HEADER: AtomicUsize = AtomicUsize::new(0);
    static OC1481_PREV_END: AtomicUsize = AtomicUsize::new(0);

    fn oc1481_fake_begin(c: &mut Cpu) {
        if c.bus.read32(OC1481_CTRL) != OC1481_MAGIC {
            return fakes::fall_back(c, &OC1481_PREV_BEGIN, ee::s12::f_0038f460__generated);
        }
        fakes::fake_call(c, 0x0038_f460);
    }

    fn oc1481_fake_header(c: &mut Cpu) {
        if c.bus.read32(OC1481_CTRL) != OC1481_MAGIC {
            return fakes::fall_back(c, &OC1481_PREV_HEADER, ee::s12::f_0037e120__generated);
        }
        fakes::fake_call(c, 0x0037_e120);
    }

    fn oc1481_fake_end(c: &mut Cpu) {
        if c.bus.read32(OC1481_CTRL) != OC1481_MAGIC {
            return fakes::fall_back(c, &OC1481_PREV_END, ee::s12::f_0038f668__generated);
        }
        fakes::fake_call(c, 0x0038_f668);
    }

    fn oc1481_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038F460),
            oc1481_fake_begin,
            &OC1481_PREV_BEGIN,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0037E120),
            oc1481_fake_header,
            &OC1481_PREV_HEADER,
        );
        fakes::hook(
            addr_of_mut!(ee::s12::OVERRIDE_0038F668),
            oc1481_fake_end,
            &OC1481_PREV_END,
        );
    }

    fn oc1481_grid_setup(c: &mut Cpu, seed: u32) {
        oc1481_install_fakes();
        let mut r = fakes::machine(c, seed, 0x1481_85, 3);
        oc1481_world(c, &mut r, seed);
        let m = OC1481_MESH;
        c.bus.write32(OC1481_CTRL, OC1481_MAGIC);
        // Call 0 (begin block) returns the packet cursor, call 2 (end block)
        // the aligned end address.
        c.bus.write32(fakes::LOG_RESULTS, OC1481_BEGIN);
        c.bus.write32(fakes::LOG_RESULTS + 8, OC1481_END);
        c.bus.write32(m + 0x1a4, (seed / 4) & 1);
        c.bus.write32(m + 0x1a0, r.next_u32() % 2);
        c.bus.write32(m + 0x1c4, OC1481_WORLD);
        let scale = oc1481_f(&mut r, 3.0);
        c.bus.write32(m + 0x194, scale.to_bits());
        for k in 0..2 {
            let t = r.next_u32();
            c.bus.write32(m + 0x1ac + 4 * k, t);
        }
        oc1481_put_matrix(c, &mut r, OC1481_MAT, false);
        c.g[A0][0] = m as u64;
        c.g[A1][0] = OC1481_MAT as u64;
        c.g[A2][0] = (r.next_u32() & 0xffff) as u64;
        c.g[A3][0] = ((seed / 8) & 1) as u64;
        c.pc = 0x0038_5c10;
    }

    /// `cPS2GridMesh_Draw` (buffer flip, view update, scaled matrix, stream
    /// block with its header and DMA tag, ring entry) against the generated
    /// body, with the three sprite-stream callees faked (call logs compare all
    /// registers; some seeds unwind inside a call).
    #[test]
    fn cps2_grid_mesh_draw_matches() {
        fakes::covered(
            ee::s12::f_00385c10__generated,
            cps2_grid_mesh_draw,
            240,
            &oc1481_grid_setup,
        );
    }

    /// The fixture reaches flip, view update, free/full ring and the flag
    /// variants.
    #[test]
    fn cps2_grid_mesh_draw_paths() {
        oc1481_paths(ee::s12::f_00385c10__generated, 240, 6, &oc1481_grid_setup);
    }
}
