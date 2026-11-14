//! Staging module oc346 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc346`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002e_02b8,
    dynamic_spray_fx_update_emitter1,
    "cDynamicSprayFX_UpdateEmitter1",
)];

// ---------------------------------------------------------------------------
// cDynamicSprayFX_UpdateEmitter1 (0x2e02b8).

/// `cDynamicSprayFX` fields read by the side-spray emitter update.
/// (`+0x00` is the owning rider: `+0x370` up vector, `+0x3b0` heading,
/// `+0x6c0` the trail-point object, `+0x77c` the board trail. The emitter
/// indices `{2, 1}` come from 0x4a3b10, the zero velocity from 0x4ff120.)
pub(crate) mod spray_emitter1 {
    /// The emitter array (0x210-byte emitters).
    pub const EMITTERS: u32 = 0x28;
    /// Per-emitter curve records (0xe8 bytes; +0xd8..+0xe4 velocity scales).
    pub const CURVES: u32 = 0x2c;
    /// Side direction (crossed with the up vector for the spray direction).
    pub const SIDE: u32 = 0x50;
    /// Spray colour.
    pub const COLOUR: u32 = 0x90;
    /// Spray position.
    pub const POS: u32 = 0xa0;
    /// Non-zero: the spray may run.
    pub const ENABLED: u32 = 0xb0;
    /// Signed lean (sign flips the spray direction).
    pub const LEAN: u32 = 0xb4;
    /// Speed-like value scaling the spray.
    pub const SPEED: u32 = 0xb8;
    /// Velocity scale for the curve's +0xe0 term.
    pub const VEL_SCALE: u32 = 0xbc;
    /// Forward direction (dotted with the owner's heading).
    pub const FORWARD: u32 = 0xc0;
    /// The rider configuration (+0x64, +0x70/+0x74, +0x78, +0x7c/+0x80).
    pub const CFG: u32 = 0xd0;
    /// 2: jitter the burst position.
    pub const MODE: u32 = 0xd4;
    /// The landing impact strength (a burst above 55.6).
    pub const IMPACT: u32 = 0xe0;
    /// Burst position.
    pub const BURST_POS: u32 = 0xf0;
    /// Burst direction.
    pub const BURST_DIR: u32 = 0x100;
    /// Non-zero: no burst.
    pub const NO_BURST: u32 = 0x114;
}

/// `off(sp)`.
#[inline(always)]
fn ue1_sp(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `off(reg)`.
#[inline(always)]
fn ue1_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn ue1_gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lui at, hi; mtc1 at, f`.
#[inline(always)]
fn ue1_fconst(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.mtc1(AT, f);
}

/// `d = a + b` (`addu`).
#[inline(always)]
fn ue1_addu(c: &mut Cpu, d: usize, a: usize, b: usize) {
    c.g[d][0] = sx(r32(c, a).wrapping_add(r32(c, b)));
}

/// The random step's tail: `d = ((a + s2) & s0) | or` (the LCG's add, then
/// the mantissa mask and the exponent of 1.0).
#[inline(always)]
fn ue1_lcg(c: &mut Cpu, d: usize, a: usize, or: usize) {
    ue1_addu(c, d, a, S2);
    c.g[d][0] &= c.g[S0][0];
    c.g[d][0] |= c.g[or][0];
}

/// `mfc1 gpr, f; lqc2 vf4, src; qmtc2 gpr, vf3; vmulx.xyzw vf5, vf4, vf3x;
/// sqc2 vf5, dst(sp)`: `*dst = *src * f`.
#[inline(always)]
fn ue1_scale(c: &mut Cpu, gpr: usize, f: usize, src: u32, dst: u32) {
    c.mfc1(gpr, f);
    c.lqc2(4, src);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let dst = ue1_sp(c, dst);
    c.sqc2(5, dst);
}

/// [`ue1_scale`] with the source on the stack too.
#[inline(always)]
fn ue1_scale_sp(c: &mut Cpu, gpr: usize, f: usize, src: u32, dst: u32) {
    let src = ue1_sp(c, src);
    ue1_scale(c, gpr, f, src, dst);
}

/// `lqc2 vf3, a(sp); lqc2 vf4, b(sp); vadd.xyzw vf5, vf3, vf4;
/// sqc2 vf5, out(sp)`.
#[inline(always)]
fn ue1_vadd(c: &mut Cpu, a: u32, b: u32, out: u32) {
    let (a, b, out) = (ue1_sp(c, a), ue1_sp(c, b), ue1_sp(c, out));
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `lq gpr, from(sp); sq gpr, to(sp)`.
#[inline(always)]
fn ue1_copy(c: &mut Cpu, gpr: usize, from: u32, to: u32) {
    let (from, to) = (ue1_sp(c, from), ue1_sp(c, to));
    c.lq(gpr, from);
    c.sq(gpr, to);
}

/// The VU0 dot product into `vf4.w`: `vf4 = vf3 * vf<b>`, then the lanes
/// summed through the accumulator (`vf6.w = 2`).
#[inline(always)]
fn ue1_dot_w(c: &mut Cpu, b: u8) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `void cDynamicSprayFX_UpdateEmitter1(cDynamicSprayFX* fx)` (0x2e02b8): the
/// board's side spray. On a hard landing (`impact` above 55.6, no
/// `NO_BURST`) both emitters get a burst particle: the burst position
/// (jittered in mode 2) and the burst direction plus a random spread, scaled
/// by the squared speed and impact ratios, plus the owner's +0x1e0 velocity.
/// Otherwise, when the spray is enabled and the configuration's +0x70/+0x74
/// exceed 0.01, each emitter gets a particle along the side direction (up x
/// side, flipped with the lean) with a spawn chance from the turn and lean
/// strength, its velocity from the board trail (or the trail point object)
/// and the curve scales. Without spray both emitters idle at the spray
/// position.
pub fn dynamic_spray_fx_update_emitter1(c: &mut Cpu) {
    use spray_emitter1 as fx;
    guest(c, |c| {
        'epilogue: {
            'spray: {
                // Prologue.
                c.cycles += 43;
                let frame = Frame::open(c, 0x230);
                c.g[V0][0] = 0x004a_0000;
                frame.save(c, S1, 0x1b0);
                frame.save(c, S0, 0x1c0);
                c.g[S1][0] = c.g[A0][0];
                frame.save(c, S2, 0x1a0);
                frame.save(c, S3, 0x190);
                frame.save(c, S4, 0x180);
                frame.save(c, S5, 0x170);
                frame.save(c, S6, 0x160);
                frame.save(c, S7, 0x150);
                frame.save(c, FP, 0x140);
                frame.save_ra(c, 0x130);
                for (k, f) in (20..=30).rev().enumerate() {
                    frame.save_fpr(c, f, 0x220 - 8 * k as u32);
                }
                c.lwc1(2, ue1_gp(c, -0x3b60)); // 55.6: the burst threshold
                c.g[V1][0] = c.lw(ue1_at(c, S1, fx::CFG));
                // The emitter indices to the stack (sp+0), the config's
                // +0x70/+0x74 pair to sp+0x10 (via sp+0x20).
                c.g[A3][0] = sx(r32(c, V0).wrapping_add(0x3b10));
                c.ldl(4, ue1_at(c, A3, 7));
                c.ldr(4, ue1_at(c, A3, 0));
                c.sdl(4, ue1_sp(c, 7));
                c.sdr(4, ue1_sp(c, 0));
                c.lwc1(0, ue1_at(c, V1, 0x70));
                c.lwc1(1, ue1_at(c, S1, fx::IMPACT));
                c.swc1(0, ue1_sp(c, 0x20));
                c.c_lt_s(2, 1);
                c.lwc1(0, ue1_at(c, V1, 0x74));
                c.swc1(0, ue1_sp(c, 0x24));
                c.ldl(2, ue1_sp(c, 0x27));
                c.ldr(2, ue1_sp(c, 0x20));
                c.sdl(2, ue1_sp(c, 0x17));
                c.sdr(2, ue1_sp(c, 0x10));
                if !c.fcc() {
                    c.g[V0][0] = c.lw(ue1_at(c, S1, fx::CFG));
                    break 'spray;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(ue1_at(c, S1, fx::NO_BURST));
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.lw(ue1_at(c, S1, fx::CFG));
                    break 'spray;
                }
                // The landing burst: one particle per emitter.
                c.cycles += 32;
                c.g[A2][0] = sx(ue1_at(c, S1, fx::COLOUR));
                c.g[S3][0] = 0x0001_0000;
                c.g[S2][0] = 0x00e9_0000;
                c.g[S0][0] = 0x007f_0000;
                ue1_fconst(c, 26, 0xc25c_0000); // -55
                c.g[S7][0] = 0;
                ue1_fconst(c, 25, 0x42dc_0000); // 110
                c.g[FP][0] = sx(ue1_sp(c, 0x30));
                ue1_fconst(c, 20, 0x3f80_0000); // 1
                c.g[S6][0] = sx(ue1_sp(c, 0x40));
                c.lwc1(29, ue1_gp(c, -0x3b5c)); // 2.52: speed scale
                c.g[S3][0] |= 0x8fcd;
                c.lwc1(28, ue1_gp(c, -0x3b58)); // 833.3: impact scale
                c.g[S2][0] |= 0x507c;
                let colour = c.g[A2][0];
                c.sw(ue1_sp(c, 0x118), colour);
                c.g[S0][0] |= 0xffff;
                ue1_fconst(c, 24, 0x4496_0000); // 1200
                c.g[S4][0] = 0x3f80_0000;
                ue1_fconst(c, 23, 0xc3fa_0000); // -500
                c.g[S5][0] = c.g[SP][0];
                ue1_fconst(c, 22, 0x447a_0000); // 1000
                ue1_fconst(c, 21, 0x3f40_0000); // 0.75
                ue1_fconst(c, 27, 0x43fa_0000); // 500
                c.lq(2, ue1_at(c, S1, fx::BURST_POS));
                loop {
                    c.cycles += 5;
                    c.g[A0][0] = 2;
                    c.g[A1][0] = c.lw(r32(c, S5));
                    c.g[V1][0] = c.lw(ue1_at(c, S1, fx::MODE));
                    let jitter = c.g[V1][0] == c.g[A0][0];
                    c.sq(2, ue1_sp(c, 0x30));
                    if jitter {
                        // pos.xy += rand * 110 - 55.
                        c.cycles += 27;
                        c.g[V0][0] = c.lw(ue1_gp(c, 0xa0c));
                        c.sw(ue1_sp(c, 0x48), 0);
                        c.mult(0, 3, 2, 19);
                        c.sw(ue1_sp(c, 0x4c), 0);
                        ue1_lcg(c, V0, V1, S4);
                        c.mult(0, 4, 2, 19);
                        let v = c.g[V0][0];
                        c.sw(ue1_gp(c, 0xa0c), v);
                        c.lwc1(1, ue1_gp(c, 0xa0c));
                        ue1_lcg(c, V1, A0, S4);
                        c.sub_s(1, 1, 20);
                        let v = c.g[V1][0];
                        c.sw(ue1_gp(c, 0xa0c), v);
                        c.mul_s(1, 1, 25);
                        c.lwc1(0, ue1_gp(c, 0xa0c));
                        c.add_s(1, 1, 26);
                        c.sub_s(0, 0, 20);
                        c.swc1(1, ue1_sp(c, 0x40));
                        c.mul_s(0, 0, 25);
                        c.add_s(0, 0, 26);
                        c.swc1(0, ue1_sp(c, 0x44));
                        ue1_vadd(c, 0x30, 0x40, 0x30);
                    }
                    // f3 = min(speed / 2.52, 1)^2 (* 0.4 without cfg+0x64).
                    c.cycles += 8;
                    c.lwc1(0, ue1_at(c, S1, fx::SPEED));
                    ue1_fconst(c, 3, 0x3f80_0000);
                    c.div_s(0, 0, 29);
                    c.c_le_s(0, 20);
                    if c.fcc() {
                        c.mov_s(3, 0);
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.lw(ue1_at(c, S1, fx::CFG));
                    c.g[V0][0] = c.lw(ue1_at(c, V1, 0x64));
                    let full = c.g[V0][0] != 0;
                    c.mul_s(3, 3, 3);
                    if !full {
                        c.cycles += 2;
                        c.lwc1(0, ue1_gp(c, -0x3b54));
                        c.mul_s(3, 3, 0);
                    }
                    // f1 = min(impact / 833.3, 1).
                    c.cycles += 8;
                    c.lwc1(0, ue1_at(c, S1, fx::IMPACT));
                    ue1_fconst(c, 1, 0x3f80_0000);
                    c.div_s(0, 0, 28);
                    c.c_le_s(0, 20);
                    if c.fcc() {
                        c.mov_s(1, 0);
                    }
                    // vel = (dir * 1200 + spread) * f3 * f1 + owner[0x1e0] * 0.75.
                    c.cycles += 77;
                    let dir = ue1_at(c, S1, fx::BURST_DIR);
                    ue1_scale(c, V0, 24, dir, 0x60);
                    c.mul_s(3, 3, 1);
                    c.g[V1][0] = c.lw(ue1_gp(c, 0xa0c));
                    c.g[T1][0] = 0x210;
                    c.lq(2, ue1_sp(c, 0x60));
                    c.mult(0, 9, 5, 9);
                    c.mult(0, 4, 3, 19);
                    c.swc1(20, ue1_sp(c, 0x6c));
                    c.sq(2, ue1_sp(c, 0x50));
                    c.g[A1][0] = c.g[FP][0];
                    c.g[A3][0] = c.lw(ue1_sp(c, 0x118));
                    c.g[A2][0] = c.g[S6][0];
                    c.lwc1(12, ue1_gp(c, -0x3b50)); // 1/60
                    c.g[T0][0] = 1;
                    ue1_addu(c, V1, A0, S2);
                    c.g[S7][0] = sx(r32(c, S7).wrapping_add(1));
                    c.g[V1][0] &= c.g[S0][0];
                    c.g[S5][0] = sx(r32(c, S5).wrapping_add(4));
                    c.g[V1][0] |= c.g[S4][0];
                    c.mult(0, 4, 3, 19);
                    let v = c.g[V1][0];
                    c.sw(ue1_gp(c, 0xa0c), v);
                    c.lwc1(2, ue1_gp(c, 0xa0c));
                    ue1_addu(c, V0, A0, S2);
                    c.sub_s(2, 2, 20);
                    c.g[V0][0] &= c.g[S0][0];
                    c.g[V0][0] |= c.g[S4][0];
                    c.mult(0, 4, 2, 19);
                    let v = c.g[V0][0];
                    c.sw(ue1_gp(c, 0xa0c), v);
                    c.mul_s(2, 2, 22);
                    c.lwc1(1, ue1_gp(c, 0xa0c));
                    ue1_addu(c, V1, A0, S2);
                    c.sub_s(1, 1, 20);
                    c.g[V1][0] &= c.g[S0][0];
                    c.add_s(2, 2, 23);
                    c.g[V1][0] |= c.g[S4][0];
                    let v = c.g[V1][0];
                    c.sw(ue1_gp(c, 0xa0c), v);
                    c.mul_s(1, 1, 22);
                    c.swc1(2, ue1_sp(c, 0x60));
                    c.lwc1(0, ue1_gp(c, 0xa0c));
                    c.add_s(1, 1, 23);
                    c.sub_s(0, 0, 20);
                    c.swc1(1, ue1_sp(c, 0x64));
                    c.mul_s(0, 0, 27);
                    c.swc1(0, ue1_sp(c, 0x68));
                    ue1_vadd(c, 0x50, 0x60, 0x70);
                    ue1_copy(c, 2, 0x70, 0x40);
                    ue1_scale_sp(c, A0, 3, 0x40, 0x40);
                    c.g[V1][0] = c.lw(r32(c, S1));
                    let src = ue1_at(c, V1, 0x1e0);
                    ue1_scale(c, A0, 21, src, 0x60);
                    ue1_copy(c, 2, 0x60, 0x50);
                    ue1_vadd(c, 0x40, 0x50, 0x40);
                    // DynamicEmitter_SpawnAt(&emitters[index], pos, vel, colour, 1).
                    c.g[A0][0] = c.lw(ue1_at(c, S1, fx::EMITTERS));
                    ue1_addu(c, A0, A0, T1);
                    call_ptr(c, 0x0037_17c0, 0x002e_05f0)?;
                    c.g[V0][0] = ((c.g[S7][0] as i64) < 2) as u64;
                    if c.g[V0][0] != 0 {
                        c.lq(2, ue1_at(c, S1, fx::BURST_POS));
                        if event_due(c) {
                            deliver_event_here(c, 0x002e_03f0);
                        }
                        continue;
                    }
                    c.cycles += 2;
                    frame.restore(c, S0, 0x1c0);
                    break 'epilogue;
                }
            }
            // v0 = fx->cfg: is the rider carving (cfg +0x70 or +0x74 > 0.01)?
            c.cycles += 6;
            c.lwc1(1, ue1_gp(c, -0x3b4c));
            c.lwc1(0, ue1_at(c, V0, 0x70));
            c.c_lt_s(1, 0);
            let carve_x = c.fcc();
            c.g[V1][0] = 0;
            let carving = if carve_x {
                true
            } else {
                c.cycles += 5;
                c.lwc1(0, ue1_at(c, V0, 0x74));
                c.c_lt_s(1, 0);
                c.fcc()
            };
            if carving {
                c.cycles += 2;
                c.g[V1][0] = 1;
            }
            c.g[V0][0] = c.lw(ue1_at(c, S1, fx::ENABLED));
            'idle: {
                c.cycles += 2;
                let disabled = c.g[V0][0] == 0;
                c.g[S5][0] = 0x0050_0000;
                if disabled {
                    c.cycles += 1;
                    c.g[S3][0] = sx(ue1_at(c, S1, fx::POS));
                    break 'idle;
                }
                c.cycles += 2;
                let still = c.g[V1][0] == 0;
                c.g[S3][0] = sx(ue1_at(c, S1, fx::POS));
                if still {
                    break 'idle;
                }
                spray_emitter1_carve(c, UE1_FRAME)?;
                break 'epilogue;
            }
            // Idle: both emitters spawn nothing at the spray position.
            c.cycles += 5;
            c.g[S4][0] = 0x210;
            c.g[S0][0] = c.g[SP][0];
            c.g[S2][0] = 1;
            c.g[A0][0] = c.lw(r32(c, S0));
            loop {
                c.cycles += 13;
                c.g[A1][0] = c.g[S3][0];
                c.g[V0][0] = c.lw(ue1_at(c, S1, fx::EMITTERS));
                c.g[A2][0] = sx(r32(c, S5).wrapping_sub(0xee0)); // ZERO_VEC
                c.mult(0, 4, 4, 20);
                c.lwc1(12, ue1_gp(c, -0x3b00)); // 1/60
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
                ue1_addu(c, A0, V0, A0);
                call_ptr(c, 0x0037_17c0, 0x002e_0e84)?;
                if (c.g[S2][0] as i64) >= 0 {
                    c.g[A0][0] = c.lw(r32(c, S0));
                    if event_due(c) {
                        deliver_event_here(c, 0x002e_0e58);
                    }
                    continue;
                }
                c.cycles += 1;
                let a = ue1_sp(c, 0x1c0);
                c.lq(S0, a);
                break 'epilogue;
            }
        }
        // Epilogue (s0 is already back).
        c.cycles += 22;
        let frame = UE1_FRAME;
        frame.restore(c, S1, 0x1b0);
        frame.restore(c, S2, 0x1a0);
        frame.restore(c, S3, 0x190);
        frame.restore(c, S4, 0x180);
        frame.restore(c, S5, 0x170);
        frame.restore(c, S6, 0x160);
        frame.restore(c, S7, 0x150);
        frame.restore(c, FP, 0x140);
        frame.restore_ra(c, 0x130);
        for (k, f) in (20..=30).rev().enumerate() {
            frame.restore_fpr(c, f, 0x220 - 8 * k as u32);
        }
        frame.close(c);
        Ok(())
    });
}

/// The emitter update's frame.
const UE1_FRAME: Frame = Frame { size: 0x230 };

/// The carving spray of `cDynamicSprayFX_UpdateEmitter1` (from 0x2e0690,
/// `s3 = &fx->pos`): the turn strength `f28` (heading . forward and the
/// lean), the edge strength `f22` (the owner's +0x214/+0x274), a spawn
/// chance `f23`, then per emitter a random velocity, a spawn roll, and a
/// velocity from the board trail (when the turn wins and the trail has
/// points), from the trail point object (turn wins, no trail) or a random
/// spread along the up vector (edge wins). Returns at the epilogue with s0
/// restored.
fn spray_emitter1_carve(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    use spray_emitter1 as fx;
    // f5 = min(|heading . forward| * 1.5, 1).
    c.cycles += 20;
    c.g[V1][0] = c.lw(r32(c, S1));
    c.lqc2(3, ue1_at(c, V1, 0x3b0));
    c.lqc2(5, ue1_at(c, S1, fx::FORWARD));
    ue1_dot_w(c, 5);
    c.qmfc2(2, 4);
    c.mtc1(2, 0);
    ue1_fconst(c, 1, 0x3fc0_0000); // 1.5
    c.abs_s(0, 0);
    ue1_fconst(c, 3, 0x3f80_0000); // 1
    c.mul_s(0, 0, 1);
    c.c_lt_s(0, 3);
    let below = c.fcc();
    c.mov_s(5, 3);
    if below {
        c.cycles += 1;
        c.mov_s(5, 0);
    }
    // f28 = clamp((|lean| + 0.2) * 0.8 + f5, 0, 1): the turn strength.
    c.cycles += 12;
    c.lwc1(0, ue1_at(c, S1, fx::LEAN));
    c.lwc1(1, ue1_gp(c, -0x3b48));
    c.abs_s(0, 0);
    c.lwc1(2, ue1_gp(c, -0x3b44));
    c.sub_s(0, 0, 1);
    c.mtc1(0, 4); // f4 = 0
    c.mul_s(0, 0, 2);
    c.add_s(0, 0, 5);
    c.c_le_s(4, 0);
    let positive = c.fcc();
    c.mov_s(28, 4);
    if positive {
        c.cycles += 1;
        c.min_s(28, 0, 3);
    }
    // f22 = max((|owner[0x214]| - 0.2) * 3, |owner[0x274]| - 0.4): the edge.
    c.cycles += 15;
    c.lwc1(2, ue1_gp(c, -0x3b40));
    c.lwc1(0, ue1_at(c, V1, 0x214));
    c.abs_s(0, 0);
    c.lwc1(1, ue1_at(c, V1, 0x274));
    c.sub_s(0, 0, 2);
    ue1_fconst(c, 3, 0x4040_0000); // 3
    c.lwc1(2, ue1_gp(c, -0x3b3c));
    c.abs_s(1, 1);
    c.mul_s(0, 0, 3);
    c.sub_s(1, 1, 2);
    c.c_le_s(0, 1);
    let edge_y = c.fcc();
    c.mov_s(22, 0);
    if edge_y {
        c.cycles += 1;
        c.mov_s(22, 1);
    }
    // f23 = strength * clamp((speed - a) / b, 0, 1.2) (turn: 555.6 and
    // 1944.4, edge: 416.7 and 833.3): the spawn chance.
    c.cycles += 4;
    c.c_lt_s(22, 28);
    let turn = c.fcc();
    c.lwc1(1, ue1_at(c, S1, fx::SPEED));
    let (lo, span, top) = if turn {
        (-0x3b38, -0x3b34, -0x3b30)
    } else {
        (-0x3b2c, -0x3b28, -0x3b24)
    };
    c.cycles += 9;
    c.lwc1(0, ue1_gp(c, lo));
    c.lwc1(2, ue1_gp(c, span));
    c.sub_s(1, 1, 0);
    c.lwc1(0, ue1_gp(c, top));
    c.div_s(1, 1, 2);
    c.c_le_s(4, 1);
    let positive = c.fcc();
    c.mov_s(3, 4);
    if positive {
        c.cycles += 1;
        c.min_s(3, 1, 0);
    }
    if turn {
        c.cycles += 2;
        c.mul_s(23, 28, 3);
    } else {
        c.cycles += 1;
        c.mul_s(23, 22, 3);
    }
    // dir = up x side (flipped for a negative lean) at sp+0x30/0x40; the
    // default velocity (the zero vector) at sp+0x20, the cfg pair at sp+0x10.
    c.cycles += 18;
    c.g[V1][0] = 0x0050_0000;
    c.g[A0][0] = c.lw(r32(c, S1));
    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0xee0));
    c.mtc1(0, 1);
    c.lq(2, r32(c, V1));
    c.sq(2, ue1_sp(c, 0x20));
    c.lqc2(4, ue1_at(c, S1, fx::SIDE));
    c.lqc2(5, ue1_at(c, A0, 0x370));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, ue1_sp(c, 0x40));
    c.lwc1(0, ue1_at(c, S1, fx::LEAN));
    c.lq(2, ue1_sp(c, 0x40));
    c.c_lt_s(0, 1);
    let flip = c.fcc();
    c.sq(2, ue1_sp(c, 0x30));
    if flip {
        c.cycles += 10;
        ue1_fconst(c, 0, 0xbf80_0000); // -1
        ue1_scale_sp(c, V1, 0, 0x30, 0x50);
        c.lq(2, ue1_sp(c, 0x50));
        c.sq(2, ue1_sp(c, 0x30));
        c.sq(2, ue1_sp(c, 0x40));
    }
    c.cycles += 31;
    c.g[A1][0] = sx(ue1_at(c, S1, fx::COLOUR));
    c.g[A2][0] = sx(ue1_sp(c, 0x10));
    c.g[A3][0] = sx(ue1_sp(c, 0x20));
    c.g[V0][0] = 1;
    c.g[S3][0] = 0x0001_0000;
    c.g[S2][0] = 0x00e9_0000;
    c.g[S0][0] = 0x007f_0000;
    ue1_fconst(c, 21, 0x3f80_0000); // 1
    c.lwc1(30, ue1_gp(c, -0x3b20));
    c.g[S6][0] = sx(ue1_sp(c, 0x40));
    c.lwc1(24, ue1_gp(c, -0x3b1c));
    c.g[S3][0] |= 0x8fcd;
    let v = c.g[A1][0];
    c.sw(ue1_sp(c, 0x118), v); // colour
    c.g[S2][0] |= 0x507c;
    c.lwc1(29, ue1_gp(c, -0x3b18));
    c.g[S0][0] |= 0xffff;
    let v = c.g[A2][0];
    c.sw(ue1_sp(c, 0x11c), v); // the chance table (the cfg pair)
    c.g[S5][0] = 0x3f80_0000;
    ue1_fconst(c, 27, 0xc348_0000); // -200
    c.g[S7][0] = 0xe8;
    let v = c.g[A3][0];
    c.sw(ue1_sp(c, 0x120), v); // the velocity
    c.g[S4][0] = c.g[SP][0];
    ue1_fconst(c, 26, 0x43c8_0000); // 400
    c.lwc1(25, ue1_gp(c, -0x3b14));
    c.sw(ue1_sp(c, 0x124), 0); // chance-table offset
    let v = c.g[V0][0];
    c.sw(ue1_sp(c, 0x110), v); // emitters left - 1
    c.lq(2, ue1_at(c, S1, fx::POS));
    loop {
        // pos = fx->pos + random velocity (sp+0x40); roll the spawn flag.
        c.cycles += 23;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S6][0];
        c.sw(ue1_sp(c, 0x114), 0);
        c.sq(2, ue1_sp(c, 0x40));
        crate::game::visualfx::boardwakefx::callee::ADD_RANDOM_VELOCITY.call(c, 0x002e_087c)?;
        c.g[A0][0] = c.lw(ue1_sp(c, 0x11c));
        c.g[A1][0] = c.lw(ue1_sp(c, 0x124));
        c.g[V0][0] = c.lw(ue1_gp(c, 0xa0c));
        ue1_addu(c, V1, A0, A1);
        c.g[FP][0] = c.lw(r32(c, S4));
        c.lwc1(1, r32(c, V1));
        c.mult(0, 3, 2, 19);
        c.mul_s(1, 23, 1);
        ue1_addu(c, V0, V1, S2);
        c.g[V0][0] &= c.g[S0][0];
        c.g[T0][0] = c.g[V0][0] | c.g[S5][0];
        let v = c.g[T0][0];
        c.sw(ue1_gp(c, 0xa0c), v);
        c.lwc1(0, ue1_gp(c, 0xa0c));
        c.sub_s(0, 0, 21);
        c.c_lt_s(0, 1);
        let spawn = c.fcc();
        c.g[A0][0] = 1;
        if spawn {
            c.cycles += 1;
            let v = c.g[A0][0];
            c.sw(ue1_sp(c, 0x114), v);
        }
        c.cycles += 4;
        c.c_lt_s(22, 28);
        let turn = c.fcc();
        c.mult(0, 2, 8, 19);
        let mut trail_done = false;
        if turn {
            c.cycles += 4;
            c.g[T1][0] = c.lw(ue1_at(c, S1, fx::CFG));
            c.g[V0][0] = c.lw(ue1_at(c, T1, 0x78));
            let no_trail = c.g[V0][0] == 0;
            c.g[A1][0] = c.lw(r32(c, S1));
            if !no_trail {
                c.cycles += 4;
                c.g[V1][0] = c.lw(ue1_at(c, A1, 0x77c));
                c.g[V0][0] = c.lw(ue1_at(c, V1, 0x3b4));
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[A3][0] = sx(ue1_at(c, V1, 0x3b0));
                if !empty {
                    spray_emitter1_trail_velocity(c);
                    trail_done = true;
                }
            }
            if !trail_done {
                spray_emitter1_point_velocity(c)?;
            }
        } else {
            spray_emitter1_edge_velocity(c)?;
        }
        if !trail_done {
            c.cycles += 1;
            c.g[A0][0] = 0x210;
        }
        // DynamicEmitter_SpawnAt(&emitters[index], pos, vel, colour, spawn).
        c.cycles += 19;
        c.g[V0][0] = c.lw(ue1_at(c, S1, fx::EMITTERS));
        c.mult(0, 4, 30, 4);
        c.g[V1][0] = c.lw(ue1_sp(c, 0x124));
        c.lwc1(12, ue1_gp(c, -0x3b04)); // 1/60
        c.g[A1][0] = c.g[S6][0];
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
        c.g[T0][0] = c.lw(ue1_sp(c, 0x114));
        c.g[A2][0] = c.lw(ue1_sp(c, 0x120));
        c.g[S4][0] = sx(r32(c, S4).wrapping_add(4));
        ue1_addu(c, A0, V0, A0);
        c.g[A3][0] = c.lw(ue1_sp(c, 0x118));
        c.g[V0][0] = c.lw(ue1_sp(c, 0x110));
        let v = c.g[V1][0];
        c.sw(ue1_sp(c, 0x124), v);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let v = c.g[V0][0];
        c.sw(ue1_sp(c, 0x110), v);
        call_ptr(c, 0x0037_17c0, 0x002e_0e2c)?;
        c.g[V1][0] = c.lw(ue1_sp(c, 0x110));
        if (c.g[V1][0] as i64) >= 0 {
            c.lq(2, ue1_at(c, S1, fx::POS));
            if event_due(c) {
                deliver_event_here(c, 0x002e_0868);
            }
            continue;
        }
        c.cycles += 2;
        frame.restore(c, S0, 0x1c0);
        return Ok(());
    }
}

/// The trail velocity (0x2e0a10): the newest board-trail point scaled to a
/// random length between cfg+0x7c and cfg+0x80, plus the spray direction
/// scaled by the point's length and a random factor, plus the up vector
/// scaled by the length; added to the default velocity at sp+0x20.
/// Leaves `a0 = 0x210`.
fn spray_emitter1_trail_velocity(c: &mut Cpu) {
    use spray_emitter1 as fx;
    c.cycles += 127;
    c.mult(0, 3, 8, 19);
    c.g[V0][0] = c.lw(ue1_at(c, A3, 0x8));
    c.g[A0][0] = 0x70;
    c.g[A2][0] = c.lw(ue1_at(c, A3, 0xc));
    c.mult(0, 2, 2, 4);
    c.lwc1(5, ue1_gp(c, -0x3b10));
    c.lwc1(6, ue1_gp(c, -0x3b0c));
    ue1_addu(c, A1, V1, S2);
    c.g[V1][0] = c.lw(ue1_at(c, A3, 0xb4));
    c.g[A1][0] &= c.g[S0][0];
    ue1_addu(c, V0, V0, A2);
    c.g[A1][0] |= c.g[S5][0];
    c.g[V1][0] = sx(r32(c, V1) << 4);
    ue1_addu(c, V1, V1, V0);
    c.mult(0, 2, 5, 19);
    ue1_addu(c, A0, V0, S2);
    c.lq(2, r32(c, V1).wrapping_sub(0x10));
    c.g[A0][0] &= c.g[S0][0];
    c.g[A0][0] |= c.g[S5][0];
    c.sq(2, ue1_sp(c, 0x50));
    // f4 = |point|.
    c.lqc2(3, ue1_sp(c, 0x50));
    ue1_dot_w(c, 3);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(2, 22);
    c.mtc1(2, 4);
    c.lwc1(2, ue1_at(c, T1, 0x7c));
    c.lwc1(3, ue1_at(c, T1, 0x80));
    let v = c.g[A1][0];
    c.sw(ue1_gp(c, 0xa0c), v);
    c.sub_s(3, 3, 2);
    c.lwc1(0, ue1_gp(c, 0xa0c));
    let v = c.g[A0][0];
    c.sw(ue1_gp(c, 0xa0c), v);
    c.sub_s(0, 0, 21);
    c.lwc1(1, ue1_gp(c, 0xa0c));
    c.mul_s(3, 3, 0);
    c.sub_s(1, 1, 21);
    c.add_s(2, 2, 3);
    ue1_scale_sp(c, V0, 2, 0x50, 0x90);
    c.mul_s(1, 1, 24);
    ue1_copy(c, 2, 0x90, 0x80);
    c.add_s(1, 1, 30);
    ue1_scale_sp(c, V1, 4, 0x30, 0xc0);
    ue1_copy(c, 2, 0xc0, 0xb0);
    ue1_scale_sp(c, V1, 2, 0xb0, 0xc0);
    ue1_copy(c, 2, 0xc0, 0xa0);
    ue1_scale_sp(c, V1, 1, 0xa0, 0xc0);
    ue1_copy(c, 2, 0xc0, 0x90);
    ue1_vadd(c, 0x80, 0x90, 0xc0);
    c.g[V0][0] = c.lw(ue1_gp(c, 0xa0c));
    c.lq(3, ue1_sp(c, 0xc0));
    c.mult(0, 4, 2, 19);
    c.sq(3, ue1_sp(c, 0x20));
    c.sq(3, ue1_sp(c, 0x60));
    ue1_lcg(c, V0, A0, S5);
    let v = c.g[V0][0];
    c.sw(ue1_gp(c, 0xa0c), v);
    c.lwc1(0, ue1_gp(c, 0xa0c));
    c.sub_s(0, 0, 21);
    c.mul_s(0, 0, 5);
    c.add_s(0, 0, 29);
    c.mul_s(0, 0, 4);
    let src = ue1_at(c, S1, fx::SIDE);
    ue1_scale(c, V1, 0, src, 0x80);
    c.lq(2, ue1_sp(c, 0x80));
    c.g[A0][0] = c.lw(r32(c, S1));
    c.sq(2, ue1_sp(c, 0x70));
    c.lqc2(3, ue1_sp(c, 0x50));
    ue1_dot_w(c, 3);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
    c.mtc1(3, 0);
    c.mul_s(0, 0, 6);
    let src = ue1_at(c, A0, 0x370);
    ue1_scale(c, V1, 0, src, 0x90);
    ue1_copy(c, 2, 0x90, 0x80);
    ue1_vadd(c, 0x70, 0x80, 0x90);
    ue1_copy(c, 2, 0x90, 0x60);
    ue1_vadd(c, 0x20, 0x60, 0x20);
    c.g[A0][0] = 0x210;
}

/// The trail-point velocity (0x2e0af4): the owner's trail point (virtual
/// call on `owner + 0x6c0`) and up vector scaled by the curve record's
/// +0xd8/+0xdc/+0xe0 (the last also by `VEL_SCALE`), plus the spray
/// direction scaled by |lean| * curve +0xe4 * `VEL_SCALE`; stored as the
/// velocity at sp+0x20 (and sp+0x50).
fn spray_emitter1_point_velocity(c: &mut Cpu) -> crate::call::Flow {
    use spray_emitter1 as fx;
    c.cycles += 101;
    c.g[V1][0] = c.lw(ue1_at(c, A1, 0x6c0));
    c.g[A1][0] = sx(ue1_at(c, A1, 0x6c0));
    c.g[A0][0] = c.lh(ue1_at(c, V1, 0x10));
    c.g[V0][0] = c.lw(ue1_at(c, V1, 0x14));
    let target = r32(c, V0);
    ue1_addu(c, A0, A1, A0);
    call_ptr(c, target, 0x002e_0b0c)?;
    // The curve record: curves + index * 0xe8.
    c.g[V1][0] = c.lw(r32(c, S4));
    c.g[A0][0] = c.lw(ue1_at(c, S1, fx::CURVES));
    c.mult(0, 3, 3, 23);
    ue1_addu(c, V1, V1, A0);
    c.lwc1(0, ue1_at(c, V1, 0xd8));
    let src = r32(c, V0);
    ue1_scale(c, A0, 0, src, 0x90);
    c.lq(2, ue1_sp(c, 0x90));
    c.g[V1][0] = c.lw(ue1_at(c, S1, fx::CURVES));
    c.sq(2, ue1_sp(c, 0x80));
    c.g[V0][0] = c.lw(r32(c, S4));
    c.g[A0][0] = c.lw(r32(c, S1));
    c.mult(0, 2, 2, 23);
    ue1_addu(c, V0, V0, V1);
    c.lwc1(0, ue1_at(c, V0, 0xdc));
    let src = ue1_at(c, A0, 0x370);
    ue1_scale(c, V1, 0, src, 0xa0);
    ue1_copy(c, 2, 0xa0, 0x90);
    ue1_vadd(c, 0x80, 0x90, 0xa0);
    c.lq(2, ue1_sp(c, 0xa0));
    c.g[V1][0] = c.lw(ue1_at(c, S1, fx::CURVES));
    c.sq(2, ue1_sp(c, 0x70));
    c.g[V0][0] = c.lw(r32(c, S4));
    c.g[A0][0] = c.lw(r32(c, S1));
    c.mult(0, 2, 2, 23);
    ue1_addu(c, V0, V0, V1);
    c.lwc1(0, ue1_at(c, V0, 0xe0));
    let src = ue1_at(c, A0, 0x370);
    ue1_scale(c, V1, 0, src, 0xd0);
    ue1_copy(c, 2, 0xd0, 0xb0);
    c.lwc1(0, ue1_at(c, S1, fx::VEL_SCALE));
    ue1_scale_sp(c, V1, 0, 0xb0, 0xd0);
    ue1_copy(c, 2, 0xd0, 0xa0);
    ue1_vadd(c, 0x70, 0xa0, 0xd0);
    c.lq(2, ue1_sp(c, 0xd0));
    c.lwc1(0, ue1_at(c, S1, fx::LEAN));
    c.sq(2, ue1_sp(c, 0x60));
    c.abs_s(0, 0);
    ue1_scale_sp(c, V0, 0, 0x30, 0x100);
    c.lq(3, ue1_sp(c, 0x100));
    c.g[A0][0] = c.lw(ue1_at(c, S1, fx::CURVES));
    c.sq(3, ue1_sp(c, 0xf0));
    c.g[V0][0] = c.lw(r32(c, S4));
    c.mult(0, 2, 2, 23);
    ue1_addu(c, V0, V0, A0);
    c.lwc1(0, ue1_at(c, V0, 0xe4));
    ue1_scale_sp(c, V1, 0, 0xf0, 0x100);
    c.lq(2, ue1_sp(c, 0x100));
    c.lwc1(0, ue1_at(c, S1, fx::VEL_SCALE));
    c.sq(2, ue1_sp(c, 0xe0));
    ue1_scale_sp(c, V1, 0, 0xe0, 0x100);
    ue1_copy(c, 2, 0x100, 0xd0);
    ue1_vadd(c, 0x60, 0xd0, 0x100);
    c.lq(2, ue1_sp(c, 0x100));
    c.sq(2, ue1_sp(c, 0x20));
    c.sq(2, ue1_sp(c, 0x50));
    Ok(())
}

/// The edge velocity (0x2e0c88): a random spread `(rand*400 - 200,
/// rand*400 - 200, rand*200, 1)` at sp+0x50, the trail point scaled by
/// 1100 (`f24`) plus the up vector scaled by `f25` and by
/// `f20 = min(speed / 1250, 1)`, plus the spread times `f20`; stored as the
/// velocity at sp+0x20 (and sp+0x60). `v0` holds the next random product.
fn spray_emitter1_edge_velocity(c: &mut Cpu) -> crate::call::Flow {
    use spray_emitter1 as fx;
    c.cycles += 38;
    c.lwc1(0, ue1_at(c, S1, fx::SPEED));
    c.lwc1(1, ue1_gp(c, -0x3b08));
    ue1_fconst(c, 3, 0x4348_0000); // 200
    c.div_s(4, 0, 1);
    c.swc1(21, ue1_sp(c, 0x5c));
    ue1_addu(c, V1, V0, S2);
    ue1_fconst(c, 20, 0x3f80_0000); // 1
    c.g[V1][0] &= c.g[S0][0];
    c.g[V1][0] |= c.g[S5][0];
    c.mult(0, 4, 3, 19);
    let v = c.g[V1][0];
    c.sw(ue1_gp(c, 0xa0c), v);
    c.lwc1(2, ue1_gp(c, 0xa0c));
    c.c_le_s(4, 21);
    ue1_addu(c, V0, A0, S2);
    c.sub_s(2, 2, 21);
    c.g[V0][0] &= c.g[S0][0];
    c.g[V0][0] |= c.g[S5][0];
    c.mult(0, 4, 2, 19);
    let v = c.g[V0][0];
    c.sw(ue1_gp(c, 0xa0c), v);
    c.mul_s(2, 2, 26);
    c.lwc1(1, ue1_gp(c, 0xa0c));
    ue1_addu(c, V1, A0, S2);
    c.sub_s(1, 1, 21);
    c.g[V1][0] &= c.g[S0][0];
    c.add_s(2, 2, 27);
    c.g[V1][0] |= c.g[S5][0];
    let v = c.g[V1][0];
    c.sw(ue1_gp(c, 0xa0c), v);
    c.mul_s(1, 1, 26);
    c.swc1(2, ue1_sp(c, 0x50));
    c.lwc1(0, ue1_gp(c, 0xa0c));
    c.add_s(1, 1, 27);
    c.sub_s(0, 0, 21);
    c.swc1(1, ue1_sp(c, 0x54));
    c.mul_s(0, 0, 3);
    let slow = c.fcc();
    c.swc1(0, ue1_sp(c, 0x58));
    if slow {
        c.cycles += 1;
        c.mov_s(20, 4);
    }
    c.cycles += 49;
    c.g[A0][0] = c.lw(r32(c, S1));
    c.g[V1][0] = c.lw(ue1_at(c, A0, 0x6c0));
    c.g[A0][0] = sx(ue1_at(c, A0, 0x6c0));
    c.g[A1][0] = c.lh(ue1_at(c, V1, 0x10));
    c.g[V0][0] = c.lw(ue1_at(c, V1, 0x14));
    let target = r32(c, V0);
    ue1_addu(c, A0, A0, A1);
    call_ptr(c, target, 0x002e_0d40)?;
    let src = r32(c, V0);
    ue1_scale(c, V1, 24, src, 0x90);
    c.lq(2, ue1_sp(c, 0x90));
    c.g[V1][0] = c.lw(r32(c, S1));
    c.sq(2, ue1_sp(c, 0x80));
    let src = ue1_at(c, V1, 0x370);
    ue1_scale(c, A0, 25, src, 0xb0);
    ue1_copy(c, 2, 0xb0, 0xa0);
    ue1_scale_sp(c, V1, 20, 0xa0, 0xb0);
    ue1_copy(c, 2, 0xb0, 0x90);
    ue1_vadd(c, 0x80, 0x90, 0xb0);
    ue1_copy(c, 2, 0xb0, 0x70);
    ue1_scale_sp(c, V1, 20, 0x50, 0xc0);
    ue1_copy(c, 2, 0xc0, 0xb0);
    ue1_vadd(c, 0x70, 0xb0, 0xc0);
    c.lq(2, ue1_sp(c, 0xc0));
    c.sq(2, ue1_sp(c, 0x60));
    c.sq(2, ue1_sp(c, 0x20));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, FAKE};
    use crate::difftest::{elf_image, Rng, DATA};

    /// Random float bits in `lo..hi`.
    fn ue1_range(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        let u = r.next_u32() as f32 / u32::MAX as f32;
        (lo + (hi - lo) * u).to_bits()
    }

    #[test]
    fn dynamic_spray_fx_update_emitter1_matches() {
        covered(
            ee::s09::f_002e02b8__generated,
            dynamic_spray_fx_update_emitter1,
            240,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x2e02b8, 2);
                c.pc = 0x002e_02b8;
                let (fx, owner, cfg, curves, em, trail, ring, vt) = (
                    DATA,
                    DATA + 0x2000,
                    DATA + 0x4000,
                    DATA + 0x5000,
                    DATA + 0x8000,
                    DATA + 0x9000,
                    DATA + 0xa000,
                    DATA + 0xc000,
                );
                // The emitters at the indices {2, 1} of the ELF's table.
                for k in 1..=2u32 {
                    crate::game::visualfx::boardwakefx::tests::put_spray_emitter(
                        c,
                        &mut r,
                        em + k * 0x210,
                    );
                }
                c.bus.write32(fx + 0x00, owner);
                c.bus.write32(fx + 0x28, em);
                c.bus.write32(fx + 0x2c, curves);
                for off in [0x40u32, 0x50, 0x90, 0xa0, 0xc0, 0xf0, 0x100] {
                    for k in 0..4u32 {
                        let v = ue1_range(&mut r, -2.0, 2.0);
                        c.bus.write32(fx + off + 4 * k, v);
                    }
                }
                for off in [0x1e0u32, 0x370, 0x3b0] {
                    for k in 0..4u32 {
                        let v = ue1_range(&mut r, -1.0, 1.0);
                        c.bus.write32(owner + off + 4 * k, v);
                    }
                }
                c.bus.write32(fx + 0x12c, ue1_range(&mut r, -1.0, 1.0));
                c.bus.write32(fx + 0xb4, ue1_range(&mut r, -1.5, 1.5));
                let speed_top = if r.next_u32() % 4 == 0 { 4.0 } else { 2500.0 };
                c.bus.write32(fx + 0xb8, ue1_range(&mut r, 0.0, speed_top));
                c.bus.write32(fx + 0xbc, ue1_range(&mut r, 0.0, 2.0));
                c.bus.write32(owner + 0x214, ue1_range(&mut r, -1.0, 1.0));
                c.bus.write32(owner + 0x274, ue1_range(&mut r, -1.0, 1.0));
                for k in 0..2u32 {
                    for off in [0xd8u32, 0xdc, 0xe0, 0xe4] {
                        let v = ue1_range(&mut r, -2.0, 2.0);
                        c.bus.write32(curves + 0xe8 * (k + 1) + off, v);
                    }
                }
                // The configuration.
                c.bus.write32(fx + 0xd0, cfg);
                c.bus.write32(cfg + 0x64, r.next_u32() & 1);
                let carve = |r: &mut Rng| {
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        ue1_range(r, 0.0, 1.0)
                    }
                };
                let (cx, cy) = (carve(&mut r), carve(&mut r));
                c.bus.write32(cfg + 0x70, cx);
                c.bus.write32(cfg + 0x74, cy);
                c.bus.write32(cfg + 0x78, r.next_u32() & 1);
                c.bus.write32(cfg + 0x7c, ue1_range(&mut r, 0.0, 2.0));
                c.bus.write32(cfg + 0x80, ue1_range(&mut r, 0.0, 2.0));
                // The board trail: newest point at ring + idx*0x70 + n*0x10 - 0x10.
                c.bus.write32(owner + 0x77c, trail);
                let count = if seed % 4 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 3
                };
                c.bus.write32(trail + 0x3b4, count);
                c.bus.write32(trail + 0x3b8, r.next_u32() % 4);
                c.bus.write32(trail + 0x3bc, ring);
                c.bus.write32(trail + 0x464, 1 + r.next_u32() % 6);
                for k in 0..0x80u32 {
                    let v = ue1_range(&mut r, -3.0, 3.0);
                    c.bus.write32(ring + 4 * k, v);
                }
                // The trail-point object's virtual call (a fake).
                c.bus.write32(owner + 0x6c0, vt);
                c.bus.write16(vt + 0x10, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + 0x14, FAKE);
                // The burst or the spray.
                match seed % 5 {
                    0 | 1 => {
                        c.bus.write32(fx + 0xe0, ue1_range(&mut r, 56.0, 1200.0));
                        c.bus.write32(fx + 0x114, (seed % 10 == 1) as u32);
                        c.bus.write32(fx + 0xd4, 1 + r.next_u32() % 2);
                    }
                    _ => {
                        c.bus.write32(fx + 0xe0, ue1_range(&mut r, 0.0, 50.0));
                        c.bus.write32(fx + 0x114, 0);
                        c.bus.write32(fx + 0xd4, 2);
                    }
                }
                c.bus.write32(fx + 0xb0, (seed % 7 != 0) as u32);
                c.g[A0][0] = sx(fx);
            },
        );
    }
}
