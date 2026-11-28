//! Staging module oc1483 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1483`.

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
    (0x0036_ac00, ps2_graphics_man_render_depth_fog, "PS2GraphicsMan_RenderDepthFog"),
    (0x0038_16f0, ps2_graphics_man_v86, "PS2GraphicsMan_DrawTrailB"),
    (0x002d_c7b0, c_ps2_fog_particle_man_draw_particles, "FogParticleMan_DrawParticles"),
];

/// `PS2GraphicsMan_RenderDepthFog` (0x0036ac00): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ps2_graphics_man_render_depth_fog(c: &mut Cpu) {
    if c.pc != 0x0036ac00 {
        return ee::s12::f_0036ac00__generated(c);
    }
    guest(c, |c| {
'b_36adf4: {
'b_36adf0: {
c.cycles += 17;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1234));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S5][0] = c.g[A0][0];
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S6][0] = c.g[A1][0];
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S7][0] = c.g[A3][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[FP][0] = c.g[A2][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
let t = c.g[V0][0] == 0u64;
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
if t {
break 'b_36adf0;
} else {
'b_36ac5c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1238));
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = sx((c.g[S6][0] as u32) << 5);
if t {
break 'b_36ac5c;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x0036ac58);
Callee { addr: 0x0036aba0, f: ee::s12::PS2GraphicsMan_ResetDepthFogParams }.call(c, 0x0036ac58)?;
c.g[V0][0] = sx((c.g[S6][0] as u32) << 5);
break 'b_36ac5c;
}
}
'b_36ac8c: {
c.cycles += 8;
c.mtc1(0, 1);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6c64));
c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
c.c_eq_s(0, 1);
if !c.fcc() {
c.g[V0][0] = 0x500000;
break 'b_36ac8c;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
if c.g[V0][0] != 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
break 'b_36adf4;
} else {
c.cycles += 1;
c.g[V0][0] = 0x500000;
break 'b_36ac8c;
}
}
}
c.cycles += 89;
c.g[S0][0] = sx((c.g[S6][0] as u32) << 10);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7b80));
c.g[A0][0] = c.g[S5][0];
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A2][0] = c.g[S6][0];
c.g[RA][0] = sx(0x0036aca8);
c.g[A1][0] = c.g[S0][0];
Callee { addr: 0x0036a428, f: ee::s12::PS2GraphicsMan_BuildDepthFogClut }.call(c, 0x0036aca8)?;
c.g[S3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[A2][0] = 0xffffffffffffffff;
c.g[A1][0] = c.lw((c.g[S7][0] as u32));
c.g[A3][0] = 0u64;
c.g[RA][0] = sx(0x0036acc0);
c.g[A0][0] = c.g[S3][0];
Callee { addr: 0x0038f460, f: ee::s12::SprStream_BeginBlock }.call(c, 0x0036acc0)?;
c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x18f4));
c.g[A2][0] = 0x7fff0000;
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
c.g[T0][0] = 0u64;
c.g[T1][0] = 0x1;
c.g[A2][0] = c.g[A2][0] | 0xffff;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f60));
c.g[A1][0] = 0x400;
c.g[RA][0] = sx(0x0036ace8);
c.g[A3][0] = 0u64;
Callee { addr: 0x00366ce0, f: ee::s12::VramHeap_Alloc }.call(c, 0x0036ace8)?;
c.g[S4][0] = c.g[V0][0];
c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x18f4));
c.g[V0][0] = 0x1c;
c.g[A3][0] = c.g[SP][0];
c.mult(0, 2, 20, 2);
c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3f50));
c.g[A1][0] = c.g[S0][0];
c.g[A0][0] = c.g[S5][0];
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A2][0] = c.lw((c.g[S2][0] as u32));
c.g[RA][0] = sx(0x0036ad18);
c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 8) as u32);
Callee { addr: 0x0036aa60, f: ee::s12::PS2GraphicsMan_EmitDepthFogClutUpload }.call(c, 0x0036ad18)?;
c.g[A1][0] = c.lw((c.g[SP][0] as u32));
c.g[A0][0] = c.g[S3][0];
c.g[RA][0] = sx(0x0036ad28);
c.g[A2][0] = 0x4;
Callee { addr: 0x0038f668, f: ee::s12::SprStream_EndBlockAligned }.call(c, 0x0036ad28)?;
c.g[A2][0] = 0xffffffffffffffff;
c.g[A1][0] = c.g[V0][0];
c.g[A3][0] = 0u64;
c.g[A0][0] = c.g[S3][0];
c.g[RA][0] = sx(0x0036ad40);
c.sw((c.g[S7][0] as u32), c.g[V0][0]);
Callee { addr: 0x0038f460, f: ee::s12::SprStream_BeginBlock }.call(c, 0x0036ad40)?;
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10d8));
c.g[S1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5a80));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[A0][0] as u32)));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x0036ad60);
c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5a84));
call_ptr(c, t, 0x0036ad60)?;
c.g[T0][0] = sx((c.g[V0][0] as u32) << 1);
c.g[T1][0] = c.g[SP][0];
c.g[A3][0] = c.g[S0][0];
c.g[A2][0] = c.g[S1][0];
c.g[A0][0] = c.g[S5][0];
c.g[RA][0] = sx(0x0036ad7c);
c.g[A1][0] = c.g[S6][0];
Callee { addr: 0x0036ae20, f: ee::s12::PS2GraphicsMan_EmitDepthFogZCopy }.call(c, 0x0036ad7c)?;
c.g[A1][0] = c.lw((c.g[SP][0] as u32));
c.g[A0][0] = c.g[S3][0];
c.g[RA][0] = sx(0x0036ad8c);
c.g[A2][0] = 0x4;
Callee { addr: 0x0038f668, f: ee::s12::SprStream_EndBlockAligned }.call(c, 0x0036ad8c)?;
c.g[A2][0] = 0xffffffffffffffff;
c.g[A0][0] = c.g[S3][0];
c.g[A1][0] = c.g[V0][0];
c.g[A3][0] = 0u64;
c.g[RA][0] = sx(0x0036ada4);
c.sw((c.g[S7][0] as u32), c.g[V0][0]);
Callee { addr: 0x0038f460, f: ee::s12::SprStream_BeginBlock }.call(c, 0x0036ada4)?;
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
c.g[A3][0] = c.g[SP][0];
c.g[A1][0] = c.g[S6][0];
c.g[A0][0] = c.g[S5][0];
c.g[A2][0] = c.lw((c.g[S2][0] as u32));
c.g[RA][0] = sx(0x0036adc0);
c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 8) as u32);
Callee { addr: 0x0036b158, f: ee::s12::PS2GraphicsMan_EmitDepthFogBlend }.call(c, 0x0036adc0)?;
c.g[A1][0] = c.lw((c.g[SP][0] as u32));
c.g[A0][0] = c.g[S3][0];
c.g[RA][0] = sx(0x0036add0);
c.g[A2][0] = 0x4;
Callee { addr: 0x0038f668, f: ee::s12::SprStream_EndBlockAligned }.call(c, 0x0036add0)?;
c.sw((c.g[S7][0] as u32), c.g[V0][0]);
c.g[A1][0] = c.g[S4][0];
c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x18f4));
c.g[RA][0] = sx(0x0036ade4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f60));
Callee { addr: 0x003663d8, f: ee::s12::VramHeap_FreeBlock }.call(c, 0x0036ade4)?;
c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x18f4));
c.g[RA][0] = sx(0x0036adf0);
c.g[A1][0] = c.g[FP][0];
Callee { addr: 0x00368138, f: ee::s12::TexMan_EmitPath3Window }.call(c, 0x0036adf0)?;
break 'b_36adf0;
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
break 'b_36adf4;
}
c.cycles += 11;
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
    });
}

/// `PS2GraphicsMan_v86` (0x003816f0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ps2_graphics_man_v86(c: &mut Cpu) {
    if c.pc != 0x003816f0 {
        return ee::s12::f_003816f0__generated(c);
    }
    guest(c, |c| {
'b_381784: {
c.cycles += 34;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
c.g[V1][0] = 0xfffffffffffffc3f;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[A3][0] = 0xffffffffc0000000;
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[S1][0] = c.g[A0][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[S2][0] = c.g[A1][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[T0][0] = 0xfffffffffffffffc;
c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
c.g[A2][0] = 0xffffffffe0000000;
c.g[A2][0] = c.g[A2][0] | 0x3ff;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | 0x140;
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
c.g[V0][0] = c.g[V0][0] | 0x2;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6b90));
let t = c.g[V1][0] != 0u64;
c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
if t {
break 'b_381784;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x00381780);
c.g[A0][0] = c.g[S1][0];
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x00381780)?;
c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
break 'b_381784;
}
}
'b_3818d4: {
c.cycles += 76;
c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5860));
c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x50));
c.lqc2(4, (c.g[V1][0] as u32));
c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[V1][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[V1][0] as u32).wrapping_add(0x30));
c.lqc2(8, (c.g[V0][0] as u32));
c.lqc2(9, (c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(10, (c.g[V0][0] as u32).wrapping_add(0x20));
c.lqc2(11, (c.g[V0][0] as u32).wrapping_add(0x30));
c.vu0().mula(0b1111, 4, Src::Bc(8, X));
c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
c.vu0().mula(0b1111, 4, Src::Bc(9, X));
c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
c.vu0().mula(0b1111, 4, Src::Bc(10, X));
c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
c.vu0().mula(0b1111, 4, Src::Bc(11, X));
c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
c.sqc2(12, (c.g[S0][0] as u32));
c.sqc2(13, (c.g[S0][0] as u32).wrapping_add(0x10));
c.sqc2(14, (c.g[S0][0] as u32).wrapping_add(0x20));
c.sqc2(15, (c.g[S0][0] as u32).wrapping_add(0x30));
c.lqc2(1, (c.g[S0][0] as u32));
c.lqc2(2, (c.g[S0][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[S0][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[SP][0] as u32));
c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5a00));
c.g[A2][0] = 0xffffffffffffffff;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[A3][0] = 0u64;
c.g[RA][0] = sx(0x00381834);
c.g[A1][0] = c.g[S3][0];
Callee { addr: 0x0038f460, f: ee::s12::SprStream_BeginBlock }.call(c, 0x00381834)?;
c.sw((c.g[SP][0] as u32).wrapping_add(0x80), c.g[V0][0]);
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0xd;
c.sd((c.g[V0][0] as u32), c.g[V1][0]);
c.g[A0][0] = 0xd817;
c.g[A0][0] = c.g[A0][0] << 47;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[A3][0] = c.g[A2][0];
c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A1][0] = c.g[SP][0];
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.sd((c.g[V1][0] as u32).wrapping_add(0x10), 0u64);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x54));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.sw((c.g[SP][0] as u32).wrapping_add(0x40), 0u64);
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x60));
c.sw((c.g[SP][0] as u32).wrapping_add(0x44), 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0x48), 0u64);
c.g[V1][0] = c.g[V0][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x80), c.g[V0][0]);
c.sw((c.g[SP][0] as u32).wrapping_add(0x4c), 0u64);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sw((c.g[SP][0] as u32).wrapping_add(0x58), 0u64);
let t = c.g[V1][0] == c.g[A0][0];
c.sw((c.g[SP][0] as u32).wrapping_add(0x5c), 0u64);
if t {
break 'b_3818d4;
} else {
c.cycles += 1;
'l_3818b8: loop {
c.cycles += 7;
c.lq(2, (c.g[A1][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A0][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x003818b8); } continue 'l_3818b8;
} else {
break 'b_3818d4;
}
}
}
}
'b_381908: {
c.cycles += 5;
c.sw((c.g[A2][0] as u32), c.g[V1][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
if c.g[V1][0] == c.g[A0][0] {
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
break 'b_381908;
} else {
'l_3818e8: loop {
c.cycles += 7;
c.lq(2, (c.g[S0][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A0][0];
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x003818e8); } continue 'l_3818e8;
} else {
c.cycles += 1;
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
break 'b_381908;
}
}
}
}
'b_381934: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] == c.g[A1][0];
c.g[A0][0] = c.g[T0][0];
if t {
break 'b_381934;
} else {
'l_381918: loop {
c.cycles += 7;
c.lq(2, (c.g[A0][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A1][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00381918); } continue 'l_381918;
} else {
break 'b_381934;
}
}
}
}
'b_381964: {
c.cycles += 5;
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x50));
let t = c.g[V1][0] == c.g[A0][0];
c.g[A1][0] = c.g[S2][0];
if t {
break 'b_381964;
} else {
'l_381948: loop {
c.cycles += 7;
c.lq(2, (c.g[A1][0] as u32));
c.sq(2, (c.g[V1][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
let t = c.g[V1][0] != c.g[A0][0];
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x00381948); } continue 'l_381948;
} else {
break 'b_381964;
}
}
}
}
'b_381a54: {
c.cycles += 27;
c.g[V0][0] = 0x0;
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x408));
c.g[V1][0] = 0x14000000;
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A1][0] = 0x8800;
c.g[A1][0] = c.g[A1][0] << 45;
c.g[V0][0] = c.g[V0][0] << 32;
c.g[A2][0] = 0x4;
c.sd((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.sd((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[A1][0] = c.g[V0][0];
c.g[RA][0] = sx(0x003819b4);
c.sw((c.g[SP][0] as u32).wrapping_add(0x80), c.g[V0][0]);
Callee { addr: 0x0038f668, f: ee::s12::SprStream_EndBlockAligned }.call(c, 0x003819b4)?;
c.g[T1][0] = c.g[V0][0];
c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18f0));
c.sw((c.g[S1][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
if t {
break 'b_381a54;
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
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S3][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_381a54;
}
}
c.cycles += 31;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5a00));
c.g[V1][0] = 0xfffffffffffffffc;
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[A0][0] = 0xffffffffe0000000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.g[A0][0] = c.g[A0][0] | 0x3ff;
c.sw((c.g[S1][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.g[A2][0] = 0xfffffffffffffc3f;
c.g[A3][0] = 0xffffffffc0000000;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe84));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
c.pc = t; return Ok(());
    });
}

/// `cPS2FogParticleMan_DrawParticles` (0x002dc7b0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_ps2_fog_particle_man_draw_particles(c: &mut Cpu) {
    if c.pc != 0x002dc7b0 {
        return ee::s09::f_002dc7b0__generated(c);
    }
    guest(c, |c| {
'b_2dc7f8: {
c.cycles += 18;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
c.g[V1][0] = 0x5;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S2][0] = 0u64;
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S6][0] = c.g[A0][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
if c.g[V1][0] == 0u64 {
break 'b_2dc7f8;
} else {
break 'b_2dc7f8;
}
}
'b_2dcb50: {
c.cycles += 11;
c.g[S1][0] = c.lw((c.g[S6][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
c.g[S3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 4) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
c.div(0, 2, 3);
c.g[V0][0] = c.lo[0];
let t = (c.g[S1][0] as i64) <= 0;
c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if t {
break 'b_2dcb50;
} else {
c.cycles += 5;
c.g[V0][0] = 0x530000;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 20);
c.g[FP][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7ac0));
'l_2dc838: loop {
'b_2dc8a4: {
'b_2dc890: {
c.cycles += 16;
c.g[S0][0] = c.g[S7][0];
c.g[S5][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S0][0] as i64)) as u64;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
if c.g[V0][0] != 0 { c.g[S0][0] = c.g[S1][0]; }
c.g[A1][0] = c.g[S5][0];
c.g[A2][0] = 0xffffffffffffffff;
c.g[RA][0] = sx(0x002dc85c);
c.g[A3][0] = 0u64;
call_ptr(c, 0x0038f460, 0x002dc85c)?;
c.g[T1][0] = c.g[V0][0];
c.sw((c.g[SP][0] as u32), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
c.g[V0][0] = c.g[S0][0] & 0x1;
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[V0][0] != 0u64;
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x14));
if t {
break 'b_2dc890;
} else {
c.cycles += 6;
c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
c.g[T4][0] = 0u64;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
let t = true;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3));
if t {
break 'b_2dc8a4;
} else {
break 'b_2dc890;
}
}
}
c.cycles += 5;
c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
c.g[T4][0] = 0x1;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
break 'b_2dc8a4;
}
'b_2dca80: {
c.cycles += 32;
c.g[A0][0] = 0x50000000;
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
c.g[A0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V1][0] = c.g[S0][0] | 0x8000;
c.g[V0][0] = 0x10000000;
c.g[A2][0] = 0xa800;
c.g[A2][0] = c.g[A2][0] << 47;
c.g[A1][0] = c.g[A1][0] | c.g[V0][0];
c.g[A2][0] = c.g[V1][0] | c.g[A2][0];
c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
c.g[A0][0] = c.g[A0][0] << 32;
c.g[V1][0] = 0x14000000;
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V1][0] = c.g[V1][0] | 0x8001;
c.g[A3][0] = 0x56;
c.g[T0][0] = 0x50000;
c.g[T0][0] = c.g[T0][0] | 0x2512;
c.sd((c.g[T1][0] as u32), c.g[A1][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x18), c.g[A0][0]);
c.g[S4][0] = c.g[V0][0];
c.sd((c.g[T1][0] as u32).wrapping_add(0x20), c.g[V1][0]);
c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[S0][0] as u32)));
c.sd((c.g[T1][0] as u32).wrapping_add(0x30), c.g[A3][0]);
c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
c.sd((c.g[T1][0] as u32).wrapping_add(0x40), c.g[A2][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x48), c.g[T0][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x8), 0u64);
c.sd((c.g[T1][0] as u32).wrapping_add(0x10), 0u64);
c.sd((c.g[T1][0] as u32).wrapping_add(0x28), 0u64);
c.sd((c.g[T1][0] as u32).wrapping_add(0x38), 0u64);
let t = c.g[V0][0] == 0u64;
c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x50));
if t {
break 'b_2dca80;
} else {
c.cycles += 5;
c.g[V0][0] = sx((c.g[S2][0] as u32) << 3);
c.g[T2][0] = sx((c.g[S4][0] as u32).wrapping_sub((c.g[S2][0] as u32)));
c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[FP][0] as u32)));
c.g[T3][0] = 0x30;
'l_2dc938: loop {
c.cycles += 82;
c.g[A1][0] = c.lw((c.g[T0][0] as u32));
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
c.lwc1(0, (c.g[SP][0] as u32));
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x8));
c.mult(0, 5, 5, 11);
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.lwc1(4, (c.g[A1][0] as u32).wrapping_add(0xc));
c.mul_s(0, 0, 4);
c.mul_s(1, 1, 4);
c.mfc1(2, 4);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
c.mfc1(4, 1);
c.g[A3][0] = c.g[V0][0];
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
c.g[A3][0] = c.g[A3][0] << 32;
c.g[V0][0] = c.g[A0][0] << 32;
c.g[V0][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[T1][0] as u32), c.g[V0][0]);
c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x28));
c.lwc1(3, (c.g[A1][0] as u32).wrapping_add(0x24));
c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x2c));
c.cvt_w_s(1, 0);
c.mfc1(2, 1);
c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x20));
c.cvt_w_s(0, 3);
c.mfc1(6, 0);
c.cvt_w_s(0, 2);
c.mfc1(3, 0);
c.cvt_w_s(0, 1);
c.mfc1(4, 0);
c.g[V0][0] = c.g[V0][0] << 8;
c.g[V0][0] = c.g[A2][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 16;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A0][0] = c.g[A0][0] << 24;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
c.sd((c.g[T1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x4));
c.lwc1(1, (c.g[A1][0] as u32));
c.mul_s(0, 0, 4);
c.cvt_w_s(3, 2);
c.mfc1(2, 3);
c.cvt_w_s(2, 1);
c.mfc1(4, 2);
c.cvt_w_s(1, 0);
c.mfc1(3, 1);
c.g[V0][0] = c.g[V0][0] << 16;
c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[T1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
c.mul_s(1, 1, 4);
c.mul_s(0, 0, 4);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
c.mfc1(4, 0);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
c.g[V0][0] = c.g[A0][0] << 32;
c.g[V0][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[T1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x18));
c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x14));
c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x10));
c.mul_s(0, 0, 4);
c.cvt_w_s(3, 2);
c.mfc1(2, 3);
c.cvt_w_s(2, 1);
c.mfc1(4, 2);
c.cvt_w_s(1, 0);
c.mfc1(3, 1);
c.g[V0][0] = c.g[V0][0] << 16;
c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[T1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
let t = c.g[T2][0] != 0u64;
c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x28));
if t {
if event_due(c) { deliver_event_here(c, 0x002dc938); } continue 'l_2dc938;
} else {
break 'b_2dca80;
}
}
}
}
'b_2dca94: {
c.cycles += 2;
let t = c.g[T4][0] == 0u64;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
if t {
break 'b_2dca94;
} else {
c.cycles += 3;
c.sd((c.g[T1][0] as u32), 0u64);
c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x8));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
break 'b_2dca94;
}
}
'b_2dcb3c: {
c.cycles += 9;
c.g[A1][0] = c.g[T1][0];
c.g[RA][0] = sx(0x002dcaa0);
c.g[A2][0] = 0x4;
call_ptr(c, 0x0038f668, 0x002dcaa0)?;
c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18f0));
c.g[T1][0] = c.g[V0][0];
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xe84));
if t {
break 'b_2dcb3c;
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
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
break 'b_2dcb3c;
}
}
c.cycles += 5;
c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x10));
c.g[S1][0] = c.g[S0][0];
c.sw((c.g[S3][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
let t = (c.g[S1][0] as i64) > 0;
c.g[S2][0] = c.g[S4][0];
if t {
if event_due(c) { deliver_event_here(c, 0x002dc838); } continue 'l_2dc838;
} else {
break 'b_2dcb50;
}
}
}
}
c.cycles += 13;
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
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{fill, fakes, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes (own address and magic in the machine's
    /// RAM: other tests' machines run the real callee bodies).
    const OC1483_CTRL: u32 = DATA + 0x20_0000;
    const OC1483_MAGIC: u32 = 0x1483_f00d;
    /// Fixture layout (everything above the fakes' result pointers at
    /// `DATA..DATA+0x40100` and below the call log at `DATA+0x100000`).
    const OC1483_GM: u32 = DATA + 0x6_0000;
    const OC1483_STATE: u32 = DATA + 0x7_0000;
    const OC1483_CNT: u32 = DATA + 0x7_4000;
    const OC1483_STREAM: u32 = DATA + 0x7_8000;
    const OC1483_OBJ: u32 = DATA + 0x7_9000;
    const OC1483_MSRC: u32 = DATA + 0x7_c000;
    const OC1483_PATH: u32 = DATA + 0x7_d000;
    const OC1483_HM: u32 = DATA + 0x8_0000;
    const OC1483_ARR: u32 = DATA + 0x8_8000;
    const OC1483_THIS: u32 = DATA + 0x9_0000;
    /// The particle index table the draw reads (a global in the game image).
    const OC1483_TABLE: u32 = 0x0053_7ac0;

    macro_rules! oc1483_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1483_CTRL) != OC1483_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                    oc1483_after(c, $addr);
                }
            )*
            /// Puts every fake in its override slot (once per process).
            fn oc1483_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    /// `VramHeap_Alloc` answers a small block number (the depth-fog code
    /// multiplies it by 0x1c to index a table).
    fn oc1483_after(c: &mut Cpu, target: u32) {
        if target == 0x0036_6ce0 {
            c.g[V0][0] = 1 + c.g[V1][0] % 3;
        }
    }

    oc1483_fakes! {
        oc1483_fake_0036aba0, 0x0036_aba0, ee::s12::OVERRIDE_0036ABA0, ee::s12::f_0036aba0__generated;
        oc1483_fake_0036a428, 0x0036_a428, ee::s12::OVERRIDE_0036A428, ee::s12::f_0036a428__generated;
        oc1483_fake_0038f460, 0x0038_f460, ee::s12::OVERRIDE_0038F460, ee::s12::f_0038f460__generated;
        oc1483_fake_00366ce0, 0x0036_6ce0, ee::s12::OVERRIDE_00366CE0, ee::s12::f_00366ce0__generated;
        oc1483_fake_0036aa60, 0x0036_aa60, ee::s12::OVERRIDE_0036AA60, ee::s12::f_0036aa60__generated;
        oc1483_fake_0038f668, 0x0038_f668, ee::s12::OVERRIDE_0038F668, ee::s12::f_0038f668__generated;
        oc1483_fake_0036ae20, 0x0036_ae20, ee::s12::OVERRIDE_0036AE20, ee::s12::f_0036ae20__generated;
        oc1483_fake_0036b158, 0x0036_b158, ee::s12::OVERRIDE_0036B158, ee::s12::f_0036b158__generated;
        oc1483_fake_003663d8, 0x0036_63d8, ee::s12::OVERRIDE_003663D8, ee::s12::f_003663d8__generated;
        oc1483_fake_00368138, 0x0036_8138, ee::s12::OVERRIDE_00368138, ee::s12::f_00368138__generated;
        oc1483_fake_0037d968, 0x0037_d968, ee::s12::OVERRIDE_0037D968, ee::s12::f_0037d968__generated;
    }

    fn oc1483_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Random floats in [-range, range) over `words` words.
    fn oc1483_fill_f32(c: &mut Cpu, r: &mut crate::difftest::Rng, a: u32, words: u32, range: f32) {
        for i in 0..words {
            let v = r.f32(range);
            c.bus.write32(a + 4 * i, v);
        }
    }

    /// The display-list cursor, the packet counter block (below or above the
    /// 2600-packet limit) and the display state words the packet writers use.
    fn oc1483_common(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        oc1483_w(c, OC1483_CTRL, OC1483_MAGIC);
        oc1483_w(c, GAME_GP + 0x1384, OC1483_STREAM);
        oc1483_w(c, OC1483_GM + 0xe84, OC1483_STATE);
        oc1483_w(c, OC1483_GM + 0x18f0, OC1483_CNT);
        oc1483_w(c, OC1483_GM + 0x5a00, r.next_u32());
        fill(c, r, OC1483_STATE, 8);
        let n = if seed % 4 == 0 { 2600 + r.next_u32() % 8 } else { r.next_u32() % 8 };
        oc1483_w(c, OC1483_CNT, n);
        fill(c, r, OC1483_CNT + 0x6_8000 + 0x1cc0, 4);
    }

    /// `PS2GraphicsMan_RenderDepthFog(gm, i, fp, path)`: with the fog enabled
    /// (gp flags), rebuilds the depth-fog CLUT and queues the CLUT upload, the
    /// z copy and the blend packet around VRAM heap allocation; exits early
    /// when disabled or when the fog parameters of slot `i` are unset.
    fn oc1483_fog_setup(c: &mut Cpu, seed: u32) {
        oc1483_hook();
        let mut r = fakes::machine(c, seed, 0x1483_0001, 16);
        oc1483_common(c, &mut r, seed);
        oc1483_w(c, GAME_GP + 0x1234, (seed % 5 != 0) as u32);
        oc1483_w(c, GAME_GP + 0x1238, r.next_u32() & 1);
        let i = r.next_u32() % 3;
        oc1483_w(c, OC1483_GM + 0x10d8, OC1483_OBJ);
        c.bus.write16(OC1483_OBJ + 0x58, 0);
        oc1483_w(c, OC1483_OBJ + 0x5c, fakes::FAKE);
        oc1483_w(c, OC1483_GM + 0x5a80, r.next_u32());
        oc1483_w(c, OC1483_GM + 0x5a84, r.next_u32());
        oc1483_w(c, OC1483_GM + 0x18f4, OC1483_HM);
        oc1483_w(c, OC1483_HM + 0x3f50, OC1483_ARR);
        fill(c, &mut r, OC1483_ARR, 0x40);
        let rec = OC1483_GM + 0x6c64 + 32 * i;
        match seed % 3 {
            0 => {
                oc1483_w(c, rec + 0xc, 0);
                oc1483_w(c, rec + 8, 1);
            }
            1 => {
                oc1483_w(c, rec + 0xc, 0);
                oc1483_w(c, rec + 8, 0);
            }
            _ => {
                oc1483_w(c, rec + 0xc, 0x3f80_0000);
                oc1483_w(c, rec + 8, r.next_u32() & 1);
            }
        }
        oc1483_w(c, OC1483_PATH, r.next_u32());
        c.g[A0][0] = OC1483_GM as u64;
        c.g[A1][0] = i as u64;
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = OC1483_PATH as u64;
        c.pc = 0x0036_ac00;
    }


    /// Every call arm of the depth-fog pass is reached by some seed.
    #[test]
    fn ps2_graphics_man_render_depth_fog_matches() {
        fakes::check_reaching(
            ee::s12::f_0036ac00__generated,
            ps2_graphics_man_render_depth_fog,
            120,
            &[
                0x0036_ac58, 0x0036_aca8, 0x0036_acc0, 0x0036_ace8, 0x0036_ad18, 0x0036_ad28,
                0x0036_ad40, 0x0036_ad60, 0x0036_ad7c, 0x0036_ad8c, 0x0036_ada4, 0x0036_adc0,
                0x0036_add0, 0x0036_ade4, 0x0036_adf0,
            ],
            &|c, seed| oc1483_fog_setup(c, seed),
        );
    }

    /// The seeds take several distinct paths (disabled, unset, with and
    /// without the parameter reset, unwinds).
    #[test]
    fn ps2_graphics_man_render_depth_fog_paths() {
        use crate::difftest::{fresh, run_generated};
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..120 {
            let mut c = fresh();
            oc1483_fog_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s12::f_0036ac00__generated);
            totals.insert(c.cycles - before);
        }
        assert!(totals.len() >= 4, "only {} distinct paths", totals.len());
    }

    /// `PS2GraphicsMan_v86(gm, m)` (the trail variant with depth fade): sets
    /// the display-state mode bits, concatenates the view matrix with `m`
    /// (VU0), emits a 0x80-byte GIF packet of the matrices and, below 2600
    /// packets, links it into the DMA chain.
    fn oc1483_v86_setup(c: &mut Cpu, seed: u32) {
        oc1483_hook();
        let mut r = fakes::machine(c, seed, 0x1483_0002, 16);
        oc1483_common(c, &mut r, seed);
        oc1483_w(c, OC1483_GM + 0x6b90, r.next_u32() & 1);
        oc1483_fill_f32(c, &mut r, OC1483_GM + 0x5860, 16, 4.0);
        oc1483_fill_f32(c, &mut r, OC1483_MSRC + 0x50, 16, 4.0);
        c.g[A0][0] = OC1483_GM as u64;
        c.g[A1][0] = OC1483_MSRC as u64;
        c.pc = 0x0038_16f0;
    }

    /// Every call arm of the trail draw is reached by some seed.
    #[test]
    fn ps2_graphics_man_v86_matches() {
        fakes::check_reaching(
            ee::s12::f_003816f0__generated,
            ps2_graphics_man_v86,
            100,
            &[0x0038_1780, 0x0038_1834, 0x0038_19b4],
            &|c, seed| oc1483_v86_setup(c, seed),
        );
    }

    #[test]
    fn ps2_graphics_man_v86_paths() {
        use crate::difftest::{fresh, run_generated};
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..100 {
            let mut c = fresh();
            oc1483_v86_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s12::f_003816f0__generated);
            totals.insert(c.cycles - before);
        }
        assert!(totals.len() >= 3, "only {} distinct paths", totals.len());
    }

    /// `cPS2FogParticleMan_DrawParticles(this)`: draws `this[0]` fog particles
    /// in packets of at most `s7` sprites (the chunk size comes from the
    /// stream's free space divided by the graphics manager address, a quirk of
    /// the original); odd chunks add a terminating tag word.
    fn oc1483_particles_setup(c: &mut Cpu, seed: u32) {
        oc1483_hook();
        let mut r = fakes::machine(c, seed, 0x1483_0003, 16);
        oc1483_common(c, &mut r, seed);
        oc1483_w(c, GAME_GP + 0x2a90, OC1483_GM);
        // Free space word: about 2k..2k+1 chunks of the manager's address.
        let k = 1 + r.next_u32() % 4;
        oc1483_w(c, OC1483_STREAM + 0x10, OC1483_GM * k + r.next_u32() % OC1483_GM);
        oc1483_w(c, OC1483_THIS, r.next_u32() % 31);
        oc1483_fill_f32(c, &mut r, OC1483_THIS + 0x10, 0x80, 50.0);
        for i in 0..40u32 {
            let idx = (r.next_u32() % 6) * 0x30;
            oc1483_w(c, OC1483_TABLE + 8 * i, idx);
        }
        c.g[A0][0] = OC1483_THIS as u64;
        c.pc = 0x002d_c7b0;
    }

    /// Both stream calls of the particle draw are reached (and some unwind).
    #[test]
    fn c_ps2_fog_particle_man_draw_particles_matches() {
        fakes::check_reaching(
            ee::s09::f_002dc7b0__generated,
            c_ps2_fog_particle_man_draw_particles,
            120,
            &[0x002d_c85c, 0x002d_caa0],
            &|c, seed| oc1483_particles_setup(c, seed),
        );
    }

    #[test]
    fn c_ps2_fog_particle_man_draw_particles_paths() {
        use crate::difftest::{fresh, run_generated};
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..120 {
            let mut c = fresh();
            oc1483_particles_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s09::f_002dc7b0__generated);
            totals.insert(c.cycles - before);
        }
        assert!(totals.len() >= 6, "only {} distinct paths", totals.len());
    }
}
