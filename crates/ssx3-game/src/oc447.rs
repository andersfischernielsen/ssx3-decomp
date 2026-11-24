//! Staging module oc447 (wave wave224): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc447`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0012_0eb8, rider_update_fx, "Rider_UpdateFX"),
    (
        0x0012_2048,
        rider_apply_default_irradiance,
        "Rider_ApplyDefaultIrradiance",
    ),
    (0x0012_2618, rider_render_fx, "Rider_RenderFX"),
    (
        0x0012_2680,
        rider_draw_screen_overlay,
        "Rider_DrawScreenOverlay",
    ),
    (0x0012_34d0, rider_place_at_matrix, "Rider_PlaceAtMatrix"),
];

// ---------------------------------------------------------------------------
// Rider_UpdateFX (0x120eb8)

/// `void Rider_UpdateFX(cRider* self)` (0x120eb8): runs the base rider's FX
/// update on the base object at `self+0x77c`.
pub fn rider_update_fx(c: &mut Cpu) {
    c.cycles += 7;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[RA][0] = sx(0x00120ec8);
    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x77c));
    c.pc = 0x00111948;
    ee::s00::RiderBase_UpdateFX(c);
    if c.pc != 0x00120ec8 {
        return;
    }
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    oc447_epilogue(c, 0x10);
}

/// Epilogue shared by the leaf wrappers: restores `ra` from the saved slot
/// already in `ra`, pops `size` and returns to `ra`.
fn oc447_epilogue(c: &mut Cpu, size: u32) {
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(size));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// Rider_ApplyDefaultIrradiance (0x122048)

/// `void Rider_ApplyDefaultIrradiance(cRider* self)` (0x122048): builds the
/// default irradiance record on the stack (`Irradiance_BuildSH`, 0x389c38),
/// then hands it to the render manager's virtual slot at `+0x228` (`this`
/// adjusted by the halfword delta stored there).
pub fn rider_apply_default_irradiance(c: &mut Cpu) {
    c.cycles += 16;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
    c.g[A1][0] = 0u64;
    c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
    c.g[RA][0] = sx(0x0012205c);
    c.g[A0][0] = c.g[SP][0];
    c.pc = 0x00389c38;
    call_indirect(c, 0x00389c38);
    if c.pc != 0x0012205c {
        return;
    }
    c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
    c.g[A1][0] = c.g[SP][0];
    c.g[A2][0] = 0u64;
    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x228));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x22c));
    let t = c.g[V1][0] as u32;
    c.g[RA][0] = sx(0x0012207c);
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.pc = t;
    call_indirect(c, t);
    if c.pc != 0x0012207c {
        return;
    }
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
    oc447_epilogue(c, 0xb0);
}

// ---------------------------------------------------------------------------
// Rider_RenderFX (0x122618)

/// `void Rider_RenderFX(cRider* self)` (0x122618): the base rider's FX render
/// on the base object at `self+0x77c`.
pub fn rider_render_fx(c: &mut Cpu) {
    c.cycles += 7;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[RA][0] = sx(0x00122628);
    c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x77c));
    c.pc = 0x001119f8;
    ee::s00::RiderBase_RenderFX(c);
    if c.pc != 0x00122628 {
        return;
    }
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    oc447_epilogue(c, 0x10);
}

// ---------------------------------------------------------------------------
// Rider_DrawScreenOverlay (0x122680)

/// `void Rider_DrawScreenOverlay(void)` (0x122680): once per frame, when the
/// global at `gp+0x1450` is clear, copies the screen overlay's sprite state
/// (the 16-byte records at `world+0x84` and the render block `+0xe84`, with
/// the unaligned `ldl/ldr` moves), sets up the overlay's vertex transform
/// (`VU0` multiply by the matrix at `0x500000-0xec0`) and draws through the
/// render manager's virtual slots. Written register for register.
pub fn rider_draw_screen_overlay(c: &mut Cpu) {
    c.cycles += 6;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1450));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
    let skip = c.g[V0][0] != 0u64;
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    if !skip {
        c.cycles += 122;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.mtc1(0, 20);
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x37));
        c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x30));
        c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x3f));
        c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x38));
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xac));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001226d0);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        call_indirect(c, t);
        if c.pc != 0x001226d0 {
            return;
        }
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xf8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xfc));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001226e8);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        call_indirect(c, t);
        if c.pc != 0x001226e8 {
            return;
        }
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[T0][0] = 0x500000;
        c.g[T1][0] = 0xffffffffffffffff;
        c.g[AT][0] = 0x44200000;
        c.mtc1(1, 14);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
        c.g[A2][0] = 0u64;
        c.g[AT][0] = 0x43f00000;
        c.mtc1(1, 15);
        c.mov_s(12, 20);
        c.ldl(7, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.ldr(7, (c.g[V0][0] as u32));
        c.ldl(10, (c.g[V0][0] as u32).wrapping_add(0xf));
        c.ldr(10, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[T3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
        c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x1b));
        c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x14));
        c.sdl(10, (c.g[V0][0] as u32).wrapping_add(0x23));
        c.sdr(10, (c.g[V0][0] as u32).wrapping_add(0x1c));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[T3][0]);
        c.mov_s(13, 20);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 17);
        c.mov_s(16, 20);
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
        c.g[A1][0] = 0u64;
        c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
        c.sw((c.g[A0][0] as u32).wrapping_add(0xe84), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xe84));
        c.g[T3][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1420));
        c.ldl(4, (c.g[T3][0] as u32).wrapping_add(0x7));
        c.ldr(4, (c.g[T3][0] as u32));
        c.ldl(7, (c.g[T3][0] as u32).wrapping_add(0xf));
        c.ldr(7, (c.g[T3][0] as u32).wrapping_add(0x8));
        c.g[T2][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x10));
        c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x7));
        c.sdr(4, (c.g[V0][0] as u32));
        c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0xf));
        c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x8));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[T2][0]);
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
        c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[T1][0]);
        c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xd0));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd4));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001227b0);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        call_indirect(c, t);
        if c.pc != 0x001227b0 {
            return;
        }
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[A1][0] = 0x500000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe60));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x110));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x114));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001227d0);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        call_indirect(c, t);
        if c.pc != 0x001227d0 {
            return;
        }
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[A0][0] = 0xfffffffffffffc1f;
        c.g[A2][0] = 0xffffffffffffff83;
        c.g[AT][0] = 0x40000000;
        c.mtc1(1, 0);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | 0x300;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
        c.g[V0][0] = c.g[V0][0] | 0x14;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.swc1(0, (c.g[SP][0] as u32));
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(2, (c.g[SP][0] as u32));
        c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x2f));
        c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x28));
        c.swc1(20, (c.g[SP][0] as u32));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(7, (c.g[SP][0] as u32));
        c.sdl(7, (c.g[S0][0] as u32).wrapping_add(0x2f));
        c.sdr(7, (c.g[S0][0] as u32).wrapping_add(0x28));
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x100));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x104));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00122854);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        call_indirect(c, t);
        if c.pc != 0x00122854 {
            return;
        }
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xb0));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xb4));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0012286c);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        call_indirect(c, t);
        if c.pc != 0x0012286c {
            return;
        }
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x14));
        c.sw((c.g[A0][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
    }
    c.cycles += 5;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t;
}

// ---------------------------------------------------------------------------
// Rider_PlaceAtMatrix (0x1234d0)

/// `void Rider_PlaceAtMatrix(cRider* self, Matrix* m)` (0x1234d0): places the
/// rider at a matrix. Reads the world's matrix stack (`gp-0x848`, `+0x84`,
/// `+0x84`) through the rider's index `+0x86c`, multiplies the matrix `m` by
/// the VU0 transform at `0x500000-0xec0`, resets the rider (main state 3,
/// sub state 13 and 6, `Rider_Place` at the matrix, FX reset) and, when the
/// index at `+0x870` is below 2, calls the entry of the stack's record.
/// Written register for register.
pub fn rider_place_at_matrix(c: &mut Cpu) {
    c.cycles += 58;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S0][0] = c.g[A0][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
    c.g[S1][0] = c.g[A1][0];
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x86c));
    c.g[RA][0] = sx(0x001234fc);
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
    c.pc = 0x00270970;
    call_indirect(c, 0x00270970);
    if c.pc != 0x001234fc {
        return;
    }
    c.lq(2, (c.g[S1][0] as u32).wrapping_add(0x30));
    c.g[A1][0] = 0x1;
    c.sq(2, (c.g[SP][0] as u32));
    c.g[V0][0] = 0x500000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
    c.lqc2(8, (c.g[V0][0] as u32));
    c.lqc2(4, (c.g[S1][0] as u32));
    c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[S1][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[S1][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x790));
    c.g[RA][0] = sx(0x00123548);
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = 0x00119368;
    ee::s00::RiderMetrix_OnRiderReset(c);
    if c.pc != 0x00123548 {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    c.g[RA][0] = sx(0x00123554);
    c.g[A1][0] = 0x3;
    c.pc = 0x0011fe78;
    ee::s00::Rider_SetMainState(c);
    if c.pc != 0x00123554 {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    c.g[RA][0] = sx(0x00123560);
    c.g[A1][0] = 0xd;
    c.pc = 0x0011fec8;
    ee::s00::Rider_SetSubState(c);
    if c.pc != 0x00123560 {
        return;
    }
    c.mtc1(0, 12);
    c.g[A1][0] = c.g[SP][0];
    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A3][0] = 0x5;
    c.g[RA][0] = sx(0x00123578);
    c.g[A0][0] = c.g[S0][0];
    c.pc = 0x0011d660;
    ee::s00::Rider_Place(c);
    if c.pc != 0x00123578 {
        return;
    }
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x77c));
    c.g[V0][0] = 0x2;
    c.g[A1][0] = 0x6;
    c.g[A0][0] = c.g[S0][0];
    c.g[RA][0] = sx(0x00123590);
    c.sw((c.g[V1][0] as u32).wrapping_add(0x290), c.g[V0][0]);
    c.pc = 0x0011fec8;
    ee::s00::Rider_SetSubState(c);
    if c.pc != 0x00123590 {
        return;
    }
    c.g[RA][0] = sx(0x00123598);
    c.g[A0][0] = c.g[S0][0];
    c.pc = 0x00125038;
    ee::s00::Rider_StopTransition(c);
    if c.pc != 0x00123598 {
        return;
    }
    c.g[RA][0] = sx(0x001235a0);
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x77c));
    c.pc = 0x00111890;
    ee::s00::RiderBase_ResetFX(c);
    if c.pc != 0x001235a0 {
        return;
    }
    c.g[RA][0] = sx(0x001235a8);
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88c));
    c.pc = 0x002f1a00;
    call_indirect(c, 0x002f1a00);
    if c.pc != 0x001235a8 {
        return;
    }
    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x870));
    c.g[V0][0] = (c.g[A1][0] < 0x2u64) as u64;
    let below_two = c.g[V0][0] != 0u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    if below_two {
        c.cycles += 11;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001235e4);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.pc = t;
        call_indirect(c, t);
        if c.pc != 0x001235e4 {
            return;
        }
    }
    c.cycles += 5;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
    oc447_epilogue(c, 0x60);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate for the oc447 fakes: a magic word in the test machine's RAM.
    const GATE: u32 = DATA + 0x20_0000;
    const MAGIC: u32 = 0x0447_0447;

    static PREV_BASE_UPDATE_FX: AtomicUsize = AtomicUsize::new(0);
    static PREV_BASE_RENDER_FX: AtomicUsize = AtomicUsize::new(0);

    /// `RiderBase_UpdateFX` (0x111948) with the gate on: log the call only.
    fn oc447_fake_base_update_fx(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, &PREV_BASE_UPDATE_FX, ee::s00::f_00111948__generated);
        }
        fakes::fake_call(c, 0x0011_1948);
    }

    /// `RiderBase_RenderFX` (0x1119f8) with the gate on: log the call only.
    fn oc447_fake_base_render_fx(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, &PREV_BASE_RENDER_FX, ee::s00::f_001119f8__generated);
        }
        fakes::fake_call(c, 0x0011_19f8);
    }

    #[test]
    fn rider_update_fx_matches() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00111948),
            oc447_fake_base_update_fx,
            &PREV_BASE_UPDATE_FX,
        );
        check(ee::s00::f_00120eb8__generated, rider_update_fx, 200, &|c, seed| {
            let mut r = Rng(0x1234_5678 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = sx(DATA);
            c.bus.write32(GATE, MAGIC);
            c.bus.write32(DATA + 0x77c, r.next_u32());
        });
    }

    #[test]
    fn rider_render_fx_matches() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s00::OVERRIDE_001119F8),
            oc447_fake_base_render_fx,
            &PREV_BASE_RENDER_FX,
        );
        check(ee::s00::f_00122618__generated, rider_render_fx, 200, &|c, seed| {
            let mut r = Rng(0x2345_6789 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = sx(DATA);
            c.bus.write32(GATE, MAGIC);
            c.bus.write32(DATA + 0x77c, r.next_u32());
        });
    }

    #[test]
    fn rider_apply_default_irradiance_matches() {
        check(
            ee::s00::f_00122048__generated,
            rider_apply_default_irradiance,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x447_1, 1);
                c.g[GP][0] = GAME_GP as u64;
                // gp-0x854: the render manager; its vtable at +0x10d8.
                let grman = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0x854, grman);
                let vt = fakes::vtable(c, &mut r, &[0x228]);
                c.bus.write32(grman + 0x10d8, vt);
            },
        );
    }

    #[test]
    fn rider_draw_screen_overlay_matches() {
        check(
            ee::s00::f_00122680__generated,
            rider_draw_screen_overlay,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x447_2, 6);
                c.g[GP][0] = GAME_GP as u64;
                let world = DATA + 0x2000;
                let stage = DATA + 0x3000;
                let sprite = DATA + 0x4000;
                let grman = DATA + 0x5000;
                // The clear flag at gp+0x1450 decides whether the body runs.
                c.bus
                    .write32(GAME_GP + 0x1450, if seed % 5 == 0 { 1 } else { 0 });
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, stage);
                c.bus.write32(stage + 0x24, DATA + 0x6000);
                fill(c, &mut r, DATA + 0x6000, 0x20);
                c.bus.write32(GAME_GP - 0x854, grman);
                c.bus.write32(grman + 0xe84, sprite);
                fill(c, &mut r, sprite, 0x20);
                let vt = fakes::vtable(c, &mut r, &[0xa8, 0xb0, 0xd0, 0xf8, 0x100, 0x110]);
                c.bus.write32(grman + 0x10d8, vt);
                // The overlay's source block (0x500000 + 0x1420) and the
                // matrix the VU0 transform loads (0x500000 - 0xec0).
                fill(c, &mut r, 0x0050_1420, 0x20);
                fill(c, &mut r, 0x0050_0000 - 0xec0, 0x10);
            },
        );
    }

    #[test]
    fn rider_place_at_matrix_matches() {
        check(ee::s00::f_001234d0__generated, rider_place_at_matrix, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x447_3, 4);
            c.g[GP][0] = GAME_GP as u64;
            let world = DATA + 0x2000;
            let stage = DATA + 0x2100;
            let list = DATA + 0x2200;
            let entry = DATA + 0x2300;
            let rider = DATA + 0x3000;
            let matrix = DATA + 0x4000;
            let base = DATA + 0x5000;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, stage);
            c.bus.write32(stage + 0x28, DATA + 0x2400);
            c.bus.write32(stage + 0x84, list);
            c.bus.write32(list + 0x4, entry);
            c.bus.write32(entry + 0xa8, base);
            let rec = fakes::vtable(c, &mut r, &[0x20]);
            c.bus.write32(base + 0x14, rec);
            fill(c, &mut r, rider, 0x100);
            c.bus.write32(rider + 0x86c, DATA + 0x2500);
            c.bus.write32(rider + 0x870, r.next_u32() % 3);
            c.bus.write32(rider + 0x77c, base);
            c.bus.write32(rider + 0x790, base);
            c.bus.write32(rider + 0x88c, base);
            c.bus.write32(rider + 0x290, 0);
            c.g[A0][0] = sx(rider);
            c.g[A1][0] = sx(matrix);
            fill(c, &mut r, matrix, 0x20);
            fill(c, &mut r, 0x0050_0000 - 0xec0, 0x10);
        });
    }

}
