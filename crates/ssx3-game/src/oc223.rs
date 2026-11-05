//! Staging wave 203 (oc223): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::visualfx::boardwakefx as bw;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002e_1a80,
        dynamic_spray_fx_update_emitter7,
        "cDynamicSprayFX_UpdateEmitter7",
    ),
    (
        0x002e_df00,
        world_painter_compute_colour_factor,
        "WorldPainter_ComputeColourFactor",
    ),
    (0x003b_55f0, snd_tick, "SND_Tick"),
    (0x0012_20d8, rider_build_light_set, "Rider_BuildLightSet"),
    (0x0042_c078, d_intr, "DIntr"),
    (
        0x0023_0f40,
        game_module_process_state_change,
        "GameModule_ProcessStateChange",
    ),
];

// ---------------------------------------------------------------------------
// cDynamicSprayFX_UpdateEmitter7 (0x2e1a80).

/// `cDynamicSprayFX` fields read by the carve-spray emitter (slot 7).
pub(crate) mod spray7 {
    /// The rider (`+0x370` up vector, `+0x3b0` side axis, `+0x1e0` velocity).
    pub const OBJ: u32 = 0x00;
    /// Base of the dynamic emitter array (0x210 bytes per slot).
    pub const P28: u32 = 0x28;
    /// Curve table (`+0x730..0x738`: up/normal/side spawn scales).
    pub const P2C: u32 = 0x2c;
    /// The edge normal the spray is thrown along.
    pub const NORMAL: u32 = 0x50;
    /// Spawn colour.
    pub const COL: u32 = 0x90;
    /// Spray position.
    pub const POS: u32 = 0xa0;
    /// Non-zero while the board is carving.
    pub const ACTIVE: u32 = 0xb0;
    /// Side of the board (negative flips the throw direction).
    pub const SIDE: u32 = 0xb4;
    /// Ground speed.
    pub const SPEED: u32 = 0xb8;
    /// Scale of the up-vector offset.
    pub const UP_SCALE: u32 = 0xbc;
    /// Board direction (dotted with the rider's side axis).
    pub const DIR: u32 = 0xc0;
    /// Configuration (`+0x50` enabled, `+0x54` throw scale).
    pub const CFG: u32 = 0xd0;
    /// Turn rate: the spray only spawns while it is small.
    pub const TURN: u32 = 0x12c;
    /// The emitter slot this update drives (7 * 0x210).
    pub const SLOT: u32 = 0xe70;
}

/// `void cDynamicSprayFX_UpdateEmitter7(cDynamicSprayFX* fx)` (0x2e1a80): the
/// carve spray. The spawn chance grows with how sideways the board runs
/// (`2|dir . side|`, capped at 1), the rider's lean (`+0x1f0`) and the square
/// of the speed above 416.67 (over 1250). While carving, with the spray
/// enabled and a small turn rate, a random draw below that chance spawns one
/// particle in slot 7: thrown along the edge normal (flipped by the side) plus
/// the rider's velocity and up vector, at the trail point, scaled by the
/// curve table. Otherwise slot 7 is kept alive at the spray position.
pub fn dynamic_spray_fx_update_emitter7(c: &mut Cpu) {
    use spray7::*;
    guest(c, |c| {
        // chance = min(2 |dir . side|, 1).
        c.cycles += 26;
        let frame = Frame::open(c, 0x150);
        bw::fconst(c, 6, 0x3f80_0000); // 1
        frame.save(c, S1, 0x130);
        frame.save(c, S2, 0x120);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x140);
        c.mov_s(1, 6);
        frame.save_ra(c, 0x110);
        let fx = r32(c, S1);
        let sp0 = sp(c);
        c.g[T1][0] = c.lw(fx);
        c.lqc2(3, bw::p(r32(c, T1), 0x3b0));
        c.lqc2(5, bw::p(fx, DIR));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(V0, 4);
        c.mtc1(V0, 0);
        c.g[V1][0] = c.lw(bw::p(fx, P28));
        c.abs_s(0, 0);
        c.add_s(0, 0, 0);
        c.c_le_s(0, 6); // 2|d| <= 1 ?
        let small = c.fcc();
        c.g[S2][0] = sx(r32(c, V1).wrapping_add(SLOT));
        if small {
            c.cycles += 1;
            c.mov_s(1, 0);
        }
        // s = min((speed - 416.67) / 1250, 1); lean = |rider+0x1f0| + |chance| - 0.1.
        c.cycles += 15;
        c.abs_s(4, 1);
        c.lwc1(2, bw::gp(c, -0x3a90)); // 416.66669
        c.lwc1(1, bw::p(fx, SPEED));
        c.mov_s(5, 6);
        c.lwc1(3, bw::gp(c, -0x3a8c)); // 1250
        c.sub_s(1, 1, 2);
        c.lwc1(0, bw::p(r32(c, T1), 0x1f0));
        c.abs_s(0, 0);
        c.lwc1(2, bw::gp(c, -0x3a88)); // 0.1
        c.add_s(0, 0, 4);
        c.div_s(1, 1, 3);
        c.c_lt_s(1, 6);
        let under = c.fcc();
        c.sub_s(0, 0, 2);
        if under {
            c.cycles += 1;
            c.mov_s(5, 1);
        }
        'idle: {
            // A random draw in [0, 0.4) against lean * s^2.
            c.cycles += 21;
            c.g[V0][0] = c.lw(bw::gp(c, 0xa0c));
            c.g[A1][0] = 0x0001_8fcd;
            c.g[A0][0] = 0x00e9_507c;
            c.mult(0, 2, 2, 5);
            c.g[A2][0] = 0x007f_ffff;
            c.g[T2][0] = 0x3f80_0000;
            c.mul_s(5, 5, 5);
            c.g[V1][0] = c.lw(bw::p(fx, ACTIVE));
            c.g[V0][0] = bw::add32(c.g[V0][0], c.g[A0][0]);
            c.lwc1(1, bw::gp(c, -0x3a84)); // 0.4
            c.g[V0][0] &= c.g[A2][0];
            c.mul_s(2, 0, 5);
            c.g[T0][0] = c.g[V0][0] | c.g[T2][0];
            c.sw(bw::gp(c, 0xa0c), c.g[T0][0]);
            c.lwc1(0, bw::gp(c, 0xa0c));
            c.sub_s(0, 0, 6);
            let inactive = c.g[V1][0] == 0;
            c.mul_s(0, 0, 1);
            if inactive {
                break 'idle;
            }
            c.cycles += 3;
            c.c_le_s(0, 2);
            if !c.fcc() {
                break 'idle;
            }
            c.cycles += 4;
            c.g[A3][0] = c.lw(bw::p(fx, CFG));
            c.g[V0][0] = c.lw(bw::p(r32(c, A3), 0x50));
            if c.g[V0][0] == 0 {
                break 'idle;
            }
            c.cycles += 7;
            c.lwc1(0, bw::p(fx, TURN));
            c.lwc1(1, bw::gp(c, -0x3a80)); // 0.2
            c.abs_s(0, 0);
            c.c_lt_s(0, 1);
            let turning = !c.fcc();
            c.mult(0, 2, 8, 5);
            if turning {
                break 'idle;
            }
            // Spawn: k = (next random - 0.5) * cfg scale; the throw is
            // pos + up * k*3 * 1/60 ... (see the vector steps below).
            c.cycles += 69;
            bw::fconst(c, 4, 0x3f00_0000); // 0.5
            bw::fconst(c, 3, 0x4040_0000); // 3
            c.lwc1(2, bw::gp(c, -0x3a7c)); // 1/60
            c.mtc1(0, 5); // f5 = 0
            c.g[V1][0] = bw::add32(c.g[V0][0], c.g[A0][0]);
            c.lq(2, bw::p(fx, POS));
            c.g[V1][0] &= c.g[A2][0];
            c.g[V1][0] |= c.g[T2][0];
            c.sq(2, sp0);
            c.lwc1(1, bw::p(r32(c, A3), 0x54));
            c.sw(bw::gp(c, 0xa0c), c.g[V1][0]);
            c.lwc1(0, bw::gp(c, 0xa0c));
            c.sub_s(0, 0, 6);
            c.add_s(0, 0, 4);
            c.mul_s(1, 0, 1);
            // up * f1 -> sp+0x20.
            c.mfc1(3, 1);
            c.lqc2(4, bw::p(r32(c, T1), 0x370));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x30));
            c.lq(2, bw::p(sp0, 0x30));
            c.g[V1][0] = c.lw(fx);
            c.sq(2, bw::p(sp0, 0x20));
            // velocity * f3 * f2 * f0 -> sp+0x30.
            c.mfc1(4, 3);
            c.lqc2(4, bw::p(r32(c, V1), 0x1e0));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x60));
            c.lq(2, bw::p(sp0, 0x60));
            c.sq(2, bw::p(sp0, 0x50));
            c.mfc1(3, 2);
            c.lqc2(4, bw::p(sp0, 0x50));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x60));
            c.lq(2, bw::p(sp0, 0x60));
            c.sq(2, bw::p(sp0, 0x40));
            c.mfc1(3, 0);
            c.lqc2(4, bw::p(sp0, 0x40));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x60));
            c.lq(2, bw::p(sp0, 0x60));
            c.sq(2, bw::p(sp0, 0x30));
            // pos += up*f1 - that.
            c.lqc2(3, bw::p(sp0, 0x20));
            c.lqc2(4, bw::p(sp0, 0x30));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, bw::p(sp0, 0x60));
            c.lq(2, bw::p(sp0, 0x60));
            c.sq(2, bw::p(sp0, 0x10));
            c.lqc2(3, sp0);
            c.lqc2(4, bw::p(sp0, 0x10));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp0);
            // side = normal x up (x cleared), flipped when SIDE < 0.
            c.g[V0][0] = c.lw(fx);
            c.lqc2(4, bw::p(fx, NORMAL));
            c.lqc2(5, bw::p(r32(c, V0), 0x370));
            c.vu0().opmula(4, 5);
            c.vu0().opmsub(6, 5, 4);
            c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
            c.sqc2(6, bw::p(sp0, 0x20));
            c.lwc1(0, bw::p(fx, SIDE));
            c.lq(2, bw::p(sp0, 0x20));
            c.c_lt_s(0, 5);
            let flip = c.fcc();
            c.sq(2, bw::p(sp0, 0x10));
            if flip {
                c.cycles += 10;
                bw::fconst(c, 0, 0xbf80_0000); // -1
                c.mfc1(3, 0);
                c.lqc2(4, bw::p(sp0, 0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, bw::p(sp0, 0x30));
                c.lq(2, bw::p(sp0, 0x30));
                c.sq(2, bw::p(sp0, 0x10));
                c.sq(2, bw::p(sp0, 0x20));
            }
            // The edge normal, negated when the rider's +0x320 is set.
            c.cycles += 4;
            c.g[V0][0] = c.lw(fx);
            c.g[V1][0] = c.lw(bw::p(r32(c, V0), 0x320));
            if c.g[V1][0] == 0 {
                c.lq(2, bw::p(fx, NORMAL));
            } else {
                c.cycles += 8;
                bw::fconst(c, 0, 0xbf80_0000); // -1
                c.mfc1(3, 0);
                c.lqc2(4, bw::p(fx, NORMAL));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, bw::p(sp0, 0x30));
                c.lq(2, bw::p(sp0, 0x30));
            }
            // vel = (-normal * speed - velocity) * 0.3; then the trail point.
            c.cycles += 134;
            c.sq(2, bw::p(sp0, 0x20));
            bw::fconst(c, 1, 0xbf80_0000); // -1
            c.g[S0][0] = sx(bw::p(sp0, 0x60));
            c.mfc1(2, 1);
            c.lqc2(4, bw::p(sp0, 0x20));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x50));
            c.lwc1(2, bw::gp(c, -0x3a78)); // 0.3
            c.lq(2, bw::p(sp0, 0x50));
            c.lwc1(0, bw::p(fx, SPEED));
            c.sq(2, bw::p(sp0, 0x40));
            c.mfc1(3, 0);
            c.lqc2(4, bw::p(sp0, 0x40));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x50));
            c.lq(2, bw::p(sp0, 0x50));
            c.g[V1][0] = c.lw(fx);
            c.sq(2, bw::p(sp0, 0x30));
            c.mfc1(4, 1);
            c.lqc2(4, bw::p(r32(c, V1), 0x1e0));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x50));
            c.lq(2, bw::p(sp0, 0x50));
            c.sq(2, bw::p(sp0, 0x40));
            c.lqc2(3, bw::p(sp0, 0x30));
            c.lqc2(4, bw::p(sp0, 0x40));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, bw::p(sp0, 0x70));
            c.lq(2, bw::p(sp0, 0x70));
            c.sq(2, bw::p(sp0, 0x60));
            c.mfc1(3, 2);
            c.lqc2(4, bw::p(sp0, 0x60));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x70));
            c.lq(2, bw::p(sp0, 0x70));
            // The trail point through the rider's vtable (+0x6c0 slot 0x10).
            c.g[A0][0] = c.lw(fx);
            c.sq(2, bw::p(sp0, 0x50));
            c.g[A1][0] = sx(r32(c, A0).wrapping_add(0x6c0));
            c.g[V1][0] = c.lw(bw::p(r32(c, A0), 0x6c0));
            c.g[A0][0] = c.lh(bw::p(r32(c, V1), 0x10));
            c.g[V0][0] = c.lw(bw::p(r32(c, V1), 0x14));
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x002e_1de4)?;
            // Spawn offset = trail * curve[0x730] + side * speed * f0
            //   + up * curve[0x734] + up * curve[0x738] * UP_SCALE + vel.
            c.g[V1][0] = c.lw(bw::p(fx, P2C));
            c.g[A0][0] = c.g[S2][0];
            c.mtc1(0, 1); // f1 = 0
            c.g[A2][0] = c.g[S0][0];
            c.lwc1(0, bw::p(r32(c, V1), 0x730));
            c.g[A3][0] = sx(bw::p(fx, COL));
            c.mfc1(3, 0);
            c.lqc2(4, r32(c, V0));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0xb0));
            c.lwc1(12, bw::gp(c, -0x3a74)); // 1/60
            c.lq(2, bw::p(sp0, 0xb0));
            c.g[A1][0] = c.g[SP][0];
            c.lwc1(0, bw::p(fx, SPEED));
            c.g[T0][0] = 1;
            c.sq(2, bw::p(sp0, 0xa0));
            c.mfc1(3, 0);
            c.lqc2(4, bw::p(sp0, 0x10));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0xd0));
            c.lq(2, bw::p(sp0, 0xd0));
            c.sq(2, bw::p(sp0, 0xc0));
            c.mfc1(3, 1);
            c.lqc2(4, bw::p(sp0, 0xc0));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0xd0));
            c.lq(2, bw::p(sp0, 0xd0));
            c.sq(2, bw::p(sp0, 0xb0));
            c.lqc2(3, bw::p(sp0, 0xa0));
            c.lqc2(4, bw::p(sp0, 0xb0));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, bw::p(sp0, 0xd0));
            c.lq(2, bw::p(sp0, 0xd0));
            c.g[T1][0] = c.lw(bw::p(fx, P2C));
            c.sq(2, bw::p(sp0, 0x90));
            c.lwc1(0, bw::p(r32(c, T1), 0x734));
            c.g[V1][0] = c.lw(fx);
            c.mfc1(10, 0);
            c.lqc2(4, bw::p(r32(c, V1), 0x370));
            c.qmtc2(10, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0xe0));
            c.lq(2, bw::p(sp0, 0xe0));
            c.sq(2, bw::p(sp0, 0xd0));
            c.lqc2(3, bw::p(sp0, 0x90));
            c.lqc2(4, bw::p(sp0, 0xd0));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, bw::p(sp0, 0xe0));
            c.lq(2, bw::p(sp0, 0xe0));
            c.g[T1][0] = c.lw(bw::p(fx, P2C));
            c.sq(2, bw::p(sp0, 0x80));
            c.lwc1(0, bw::p(r32(c, T1), 0x738));
            c.g[V1][0] = c.lw(fx);
            c.mfc1(10, 0);
            c.lqc2(4, bw::p(r32(c, V1), 0x370));
            c.qmtc2(10, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x100));
            c.lq(2, bw::p(sp0, 0x100));
            c.lwc1(0, bw::p(fx, UP_SCALE));
            c.sq(2, bw::p(sp0, 0xf0));
            c.mfc1(3, 0);
            c.lqc2(4, bw::p(sp0, 0xf0));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, bw::p(sp0, 0x100));
            c.lq(2, bw::p(sp0, 0x100));
            c.sq(2, bw::p(sp0, 0xe0));
            c.lqc2(3, bw::p(sp0, 0x80));
            c.lqc2(4, bw::p(sp0, 0xe0));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, bw::p(sp0, 0x100));
            c.lq(2, bw::p(sp0, 0x100));
            c.sq(2, bw::p(sp0, 0x70));
            c.lqc2(3, bw::p(sp0, 0x70));
            c.lqc2(4, bw::p(sp0, 0x50));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, bw::p(sp0, 0x100));
            c.lq(2, bw::p(sp0, 0x100));
            c.sq(2, bw::p(sp0, 0x60));
            // DynamicEmitter_SpawnAt(slot 7, &pos, &vel, &colour, 1/60, 1).
            call_ptr(c, 0x0037_17c0, 0x002e_1f34)?;
            frame.restore(c, S0, 0x140);
            c.cycles += 5;
            frame.restore(c, S1, 0x130);
            frame.restore(c, S2, 0x120);
            frame.restore_ra(c, 0x110);
            frame.close(c);
            return Ok(());
        }
        // Idle: keep slot 7 alive at the spray position.
        c.cycles += 8;
        c.lwc1(12, bw::gp(c, -0x3a70)); // 1/60
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(bw::p(r32(c, S1), POS));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x0037_17c0, 0x002e_1f58)?;
        frame.restore(c, S0, 0x140);
        c.cycles += 5;
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// WorldPainter_ComputeColourFactor (0x2edf00).

/// `float WorldPainter_ComputeColourFactor(Colour* col)` (0x2edf00): the
/// luminance `0.299 r + 0.587 g + 0.114 b` of the colour at `+4..+0xc`,
/// mapped to a 0..1 factor: 0 at or below 0.1, 1 at or above 0.45, between
/// them `clamp(exp(log((lum - 0.1) / |0.35|) * 0.6001176), 0, 1)` (a gamma
/// curve).
pub fn world_painter_compute_colour_factor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        let col = r32(c, A0);
        c.lwc1(3, bw::gp(c, -0x395c)); // 0.299
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x10);
        c.lwc1(4, bw::gp(c, -0x3958)); // 0.587
        c.lwc1(1, bw::p(col, 0x4));
        c.lwc1(2, bw::p(col, 0x8));
        c.mul_s(1, 1, 3);
        c.lwc1(0, bw::p(col, 0xc));
        c.mul_s(2, 2, 4);
        c.lwc1(3, bw::gp(c, -0x3954)); // 0.114
        c.lwc1(4, bw::gp(c, -0x3950)); // 0.1
        c.mul_s(0, 0, 3);
        c.add_s(1, 1, 2);
        c.add_s(1, 1, 0);
        c.c_le_s(1, 4); // lum <= 0.1 ?
        c.mtc1(0, 0); // f0 = 0
        let dark = c.fcc();
        frame.restore_ra(c, 0);
        'restore: {
            if dark {
                break 'restore;
            }
            c.cycles += 6;
            c.lwc1(0, bw::gp(c, -0x394c)); // 0.45
            c.c_le_s(0, 1); // 0.45 <= lum ?
            bw::fconst(c, 0, 0x3f80_0000); // 1
            let bright = c.fcc();
            frame.restore_fpr(c, 20, 0x10);
            if bright {
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            // exp(log(t) * k), clamped to [0, 1].
            c.cycles += 17;
            c.sub_s(0, 1, 4);
            c.lwc1(12, bw::gp(c, -0x3948)); // 0.35
            c.abs_s(12, 12);
            c.lwc1(20, bw::gp(c, -0x3944)); // 0.6001176
            c.div_s(12, 0, 12);
            call_ptr(c, 0x0040_da10, 0x002e_df80)?; // logf
            c.g[RA][0] = sx(0x002e_df88);
            c.mul_s(12, 0, 20);
            call_ptr(c, 0x0040_d758, 0x002e_df88)?; // expf
            c.mov_s(1, 0);
            bw::fconst(c, 2, 0x3f80_0000); // 1
            c.mtc1(0, 0); // f0 = 0
            c.c_le_s(0, 1);
            if c.fcc() {
                c.min_s(0, 1, 2);
            }
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 1;
        frame.restore_fpr(c, 20, 0x10);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// SND_Tick (0x3b55f0).

/// The sound driver's state block and the per-channel fields `SND_Tick`
/// steps.
pub(crate) mod snd_tick {
    /// The driver state (`0x510000 - 0x5718`).
    pub const STATE: u32 = 0x0050_a8e8;
    /// `s8`: number of tick callbacks.
    pub const TICK_CALLBACKS: u32 = 0x177;
    /// `s16`: number of voice channels.
    pub const CHANNEL_COUNT: u32 = 0x17e;
    /// `u32`: driver ticks run so far.
    pub const TICKS: u32 = 0x180;
    /// `void (*)()[]`: the tick callbacks.
    pub const CALLBACKS: u32 = 0x184;
    /// `SndChannel*`: the channel array (0x8c bytes each).
    pub const CHANNELS: u32 = 0x1f0;
    pub const CHANNEL_SIZE: u64 = 0x8c;

    /// Channel: volume sweep step per tick (0: no sweep).
    pub const SWEEP_STEP: u32 = 0x30;
    /// Channel: sweep target.
    pub const SWEEP_TARGET: u32 = 0x34;
    /// Channel: current sweep value.
    pub const SWEEP: u32 = 0x38;
    /// Channel: envelope level step per tick.
    pub const ENV_STEP: u32 = 0x3c;
    /// Channel: envelope level (16.16).
    pub const ENV_LEVEL: u32 = 0x40;
    /// Channel: ticks left in the envelope segment.
    pub const ENV_TICKS: u32 = 0x44;
    /// Channel: number of envelope segments (`s8`).
    pub const ENV_SEGMENTS: u32 = 0x5c;
    /// Channel: current envelope segment (`u8`).
    pub const ENV_SEGMENT: u32 = 0x5d;
    /// Channel: volume LFO length / position (`u8`).
    pub const VOL_LFO_LEN: u32 = 0x65;
    /// Channel: pitch LFO length (`u8`).
    pub const PITCH_LFO_LEN: u32 = 0x66;
    pub const VOL_LFO_POS: u32 = 0x67;
    pub const PITCH_LFO_POS: u32 = 0x68;
    /// Channel: allocated flag (`s8`, 1 when allocated).
    pub const ALLOCATED: u32 = 0x69;
    /// Channel: envelope segment table (`{ticks, level}` pairs).
    pub const ENV_TABLE: u32 = 0x6c;
    /// Channel: volume LFO table, or null.
    pub const VOL_LFO: u32 = 0x78;
    /// Channel: pitch LFO (modulation) table, or null.
    pub const PITCH_LFO: u32 = 0x7c;
    /// Channel: cached pitch (`u16`), cleared to force a recompute.
    pub const PITCH_CACHE: u32 = 0x86;
}

/// The callees of `SND_Tick`.
mod snd_tick_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;
    pub const TICK_STUB: Callee = Callee {
        addr: 0x003c_2438,
        f: ee::s13::SND_TickStub,
    };
    pub const CALC_PITCH: Callee = Callee {
        addr: 0x003b_a938,
        f: ee::s13::SND_ChannelCalcPitch,
    };
    pub const APPLY_PITCH: Callee = Callee {
        addr: 0x003c_25e0,
        f: ee::s13::SND_ChannelApplyPitch,
    };
    pub const STOP: Callee = Callee {
        addr: 0x003b_89e0,
        f: ee::s13::SND_Stop,
    };
    pub const CALC_VOLUME: Callee = Callee {
        addr: 0x003b_a7b0,
        f: ee::s13::SND_ChannelCalcVolume,
    };
    pub const APPLY_VOLUME: Callee = Callee {
        addr: 0x003c_2a50,
        f: ee::s13::SND_ChannelApplyVolume,
    };
}

/// `void SND_Tick(void)` (0x3b55f0): the sound driver's tick. Counts the
/// tick, runs the registered tick callbacks, then steps every allocated,
/// playing channel: the pitch LFO (recomputing and applying the pitch), the
/// volume LFO, the volume sweep (toward its target), and the envelope (the
/// level ramps by its step; at the end of a segment the next one is loaded,
/// or the voice is stopped after the last one, or when the sweep goes
/// negative). A channel whose volume inputs moved gets its volume recomputed
/// and applied.
pub fn snd_tick(c: &mut Cpu) {
    use snd_tick::*;
    use snd_tick_callee as callee;
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x0051_0000;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx(STATE);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 0;
        frame.save(c, S2, 0x20);
        c.g[S3][0] = c.g[V0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = sx((c.lw(STATE + TICKS) as u32).wrapping_add(1));
        c.sw(STATE + TICKS, c.g[V1][0]);
        callee::TICK_STUB.call(c, 0x003b_562c)?;
        // The tick callbacks.
        c.g[V0][0] = c.lb(STATE + TICK_CALLBACKS);
        c.g[S2][0] = c.g[S0][0];
        if c.g[V0][0] as i64 > 0 {
            c.cycles += 2;
            c.g[S0][0] = sx(STATE + CALLBACKS);
            c.g[V1][0] = c.lw(r32(c, S0));
            loop {
                c.cycles += 7;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let callback = r32(c, V1);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                call_ptr(c, callback, 0x003b_564c)?;
                c.g[V0][0] = c.lb(STATE + TICK_CALLBACKS);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = c.lw(r32(c, S0));
                if event_due(c) {
                    deliver_event_here(c, 0x003b_5640);
                }
            }
        }
        // The channels.
        c.cycles += 4;
        c.g[V1][0] = sx(r32(c, S3).wrapping_sub(0x5718));
        c.g[V0][0] = c.lh(r32(c, V1) + CHANNEL_COUNT);
        c.g[S1][0] = 0;
        if c.g[V0][0] as i64 > 0 {
            c.cycles += 3;
            c.g[S2][0] = c.g[V1][0];
            c.g[V1][0] = CHANNEL_SIZE;
            loop {
                snd_tick_channel(c)?;
                c.cycles += 4;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = CHANNEL_SIZE;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003b_5678);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// One channel of [`snd_tick`]: `s1` is the index, `s2` the driver state;
/// leaves the channel count in `v0` (`lh`) for the loop test.
fn snd_tick_channel(c: &mut Cpu) -> crate::call::Flow {
    use snd_tick::*;
    use snd_tick_callee as callee;
    let state = r32(c, S2);
    let count = |c: &mut Cpu| c.g[V0][0] = c.lh(state + CHANNEL_COUNT);
    c.cycles += 7;
    c.g[A0][0] = c.lw(state + CHANNELS);
    c.mult(0, 3, 17, 3); // index * 0x8c
    c.g[A1][0] = 1;
    c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    let ch = r32(c, S0);
    c.g[V0][0] = c.lb(bw::p(ch, ALLOCATED));
    if c.g[V0][0] != c.g[A1][0] {
        count(c);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = c.lw(ch);
    if (c.g[V0][0] as i64) < 0 {
        count(c);
        return Ok(());
    }
    // Pitch LFO: step its position (wrapping at the length) and re-pitch.
    c.cycles += 3;
    c.g[V0][0] = c.lw(bw::p(ch, PITCH_LFO));
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        c.g[V0][0] = c.lbu(bw::p(ch, PITCH_LFO_POS));
        c.g[A0][0] = c.lbu(bw::p(ch, PITCH_LFO_LEN));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V1][0] = c.g[V0][0] & 0xff;
        c.g[V1][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
        let inside = c.g[V1][0] != 0;
        c.sb(bw::p(ch, PITCH_LFO_POS), c.g[V0][0]);
        if !inside {
            c.cycles += 1;
            c.sb(bw::p(ch, PITCH_LFO_POS), 0);
        }
        c.cycles += 6;
        c.sh(bw::p(ch, PITCH_CACHE), 0);
        c.g[A0][0] = c.g[S1][0];
        callee::CALC_PITCH.call(c, 0x003b_56d8)?;
        c.g[A0][0] = c.g[S1][0];
        callee::APPLY_PITCH.call(c, 0x003b_56e0)?;
    }
    c.g[V0][0] = c.lw(bw::p(ch, VOL_LFO));
    // Volume LFO: step its position; a1 = volume changed.
    c.cycles += 2;
    c.g[A1][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 8;
        c.g[V0][0] = c.lbu(bw::p(ch, VOL_LFO_POS));
        c.g[A1][0] = 1;
        c.g[A0][0] = c.lbu(bw::p(ch, VOL_LFO_LEN));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V1][0] = c.g[V0][0] & 0xff;
        c.g[V1][0] = (c.g[V1][0] < c.g[A0][0]) as u64;
        let inside = c.g[V1][0] != 0;
        c.sb(bw::p(ch, VOL_LFO_POS), c.g[V0][0]);
        if !inside {
            c.cycles += 1;
            c.sb(bw::p(ch, VOL_LFO_POS), 0);
        }
    }
    'volume: {
        'next_segment: {
            'stop: {
                'envelope: {
                    // Sweep toward the target; a negative sweep stops.
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(bw::p(ch, SWEEP_STEP));
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = c.lw(bw::p(ch, ENV_TICKS));
                        break 'envelope;
                    }
                    'reached: {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw(bw::p(ch, SWEEP));
                        c.g[A1][0] = 1;
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                        let rising = (c.g[V1][0] as i64) >= 0;
                        c.sw(bw::p(ch, SWEEP), c.g[V0][0]);
                        if !rising {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw(bw::p(ch, SWEEP_TARGET));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            break 'reached;
                        }
                        c.cycles += 2;
                        c.g[V1][0] = c.lw(bw::p(ch, SWEEP_TARGET));
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    }
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lw(bw::p(ch, SWEEP));
                    } else {
                        // Target reached: snap and end the sweep.
                        c.cycles += 3;
                        c.sw(bw::p(ch, SWEEP), c.g[V1][0]);
                        c.sw(bw::p(ch, SWEEP_STEP), 0);
                        c.g[V0][0] = c.lw(bw::p(ch, SWEEP));
                    }
                    c.cycles += 2;
                    if (c.g[V0][0] as i64) < 0 {
                        break 'stop;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(bw::p(ch, ENV_TICKS));
                }
                // Envelope: count the segment down and ramp the level.
                c.cycles += 4;
                c.g[V1][0] = c.lw(bw::p(ch, ENV_STEP));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                let flat = c.g[V1][0] == 0;
                c.sw(bw::p(ch, ENV_TICKS), c.g[V0][0]);
                if !flat {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(bw::p(ch, ENV_LEVEL));
                    c.g[A1][0] = 1;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.sw(bw::p(ch, ENV_LEVEL), c.g[V0][0]);
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(bw::p(ch, ENV_TICKS));
                if c.g[V0][0] != 0 {
                    break 'volume;
                }
                // Segment over: the next one, or stop after the last.
                c.cycles += 8;
                c.g[V0][0] = c.lbu(bw::p(ch, ENV_SEGMENT));
                c.g[V1][0] = c.lb(bw::p(ch, ENV_SEGMENTS));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                c.g[A0][0] = sx(r32(c, V0) << 24);
                c.g[A0][0] = sx(((r32(c, A0) as i32) >> 24) as u32);
                c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let more = c.g[V1][0] != 0;
                c.sb(bw::p(ch, ENV_SEGMENT), c.g[V0][0]);
                if more {
                    break 'next_segment;
                }
                break 'stop;
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw(ch);
            callee::STOP.call(c, 0x003b_57b8)?;
            count(c);
            return Ok(());
        }
        // Load segment a0: {ticks (negative: forever), level << 16}; the
        // step spreads the level change over the ticks.
        c.cycles += 6;
        c.g[V0][0] = c.lw(bw::p(ch, ENV_TABLE));
        c.g[V1][0] = sx(r32(c, A0) << 3);
        c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[A0][0] = c.lw(r32(c, V1));
        let finite = (c.g[A0][0] as i64) >= 0;
        c.sw(bw::p(ch, ENV_TICKS), c.g[A0][0]);
        if !finite {
            c.cycles += 3;
            c.g[V0][0] = 0x7fff_ffff;
            c.sw(bw::p(ch, ENV_TICKS), c.g[V0][0]);
        }
        c.cycles += 8;
        c.g[V0][0] = c.lw(bw::p(r32(c, V1), 4));
        c.g[A0][0] = c.lw(bw::p(ch, ENV_LEVEL));
        c.g[V1][0] = c.lw(bw::p(ch, ENV_TICKS));
        c.g[V0][0] = sx(r32(c, V0) << 16);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A0)));
        c.div(0, 2, 3);
        c.cycles += 2;
        c.g[V0][0] = c.lo[0];
        c.sw(bw::p(ch, ENV_STEP), c.g[V0][0]);
    }
    // Volume inputs moved: recompute and apply.
    c.cycles += 2;
    if c.g[A1][0] == 0 {
        count(c);
        return Ok(());
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S1][0];
    callee::CALC_VOLUME.call(c, 0x003b_581c)?;
    c.g[V0][0] = c.lw(ch);
    if (c.g[V0][0] as i64) < 0 {
        count(c);
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[S1][0];
    callee::APPLY_VOLUME.call(c, 0x003b_5830)?;
    count(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Rider_BuildLightSet (0x1220d8).

/// `cRider` fields used to build its light set.
pub(crate) mod rider_lights {
    /// The rider's vtable (`+0x38` slot: the AI/view index getter).
    pub const VTABLE: u32 = 0x6c0;
    pub const AI_INDEX_SLOT: u32 = 0x38;
    /// `RiderHandler*` (its `+0xd30` holds the extra directional lights).
    pub const HANDLER: u32 = 0x77c;
    /// Eight dynamic lights affecting the rider (null for none).
    pub const DYNAMIC_LIGHTS: u32 = 0x794;
    /// The rider's irradiance (light set).
    pub const LIGHTS: u32 = 0x7c0;
    /// Handler offset of the directional-light block: `{x, y, z, n}` and
    /// `n` lights (`+0x10` points, 12 bytes each; `+0x50` directions, 16
    /// bytes each).
    pub const HANDLER_LIGHTS: u32 = 0xd30;
}

/// `int rider->GetAIIndex()` through the rider's vtable, returning to `ret`.
fn rider_ai_index(c: &mut Cpu, rider: u32, ret: u32) -> crate::call::Flow {
    use rider_lights::*;
    c.g[V1][0] = c.lw(rider.wrapping_add(VTABLE));
    c.g[A1][0] = sx(rider.wrapping_add(VTABLE));
    c.g[A0][0] = c.lh(bw::p(r32(c, V1), AI_INDEX_SLOT));
    c.g[V0][0] = c.lw(bw::p(r32(c, V1), AI_INDEX_SLOT + 4));
    let method = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

/// `void Rider_BuildLightSet(cRider* r)` (0x1220d8): rebuild the rider's
/// irradiance at `+0x7c0`: clear it, copy the view's ambient set
/// (`WorldPainter_GetViewIrradiance(index)`, 0xa0 bytes), add the
/// camera-relative light at the hips (`WorldPainter_GetLighting30`), each of
/// the eight dynamic lights, and, if the rider handler has them, its
/// directional block: the offset vector, then one directional light per
/// entry.
pub fn rider_build_light_set(c: &mut Cpu) {
    use rider_lights::*;
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S0, 0x80);
        c.g[S2][0] = c.g[A0][0];
        let rider = r32(c, S2);
        c.g[S0][0] = sx(rider.wrapping_add(LIGHTS));
        frame.save(c, S1, 0x70);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0038_9260, 0x0012_2108)?; // Irradiance_Clear
        rider_ai_index(c, rider, 0x0012_2120)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002e_dfb8, 0x0012_2128)?; // WorldPainter_GetViewIrradiance
                                                // Copy the view's irradiance (0xa0 bytes, two quadwords a step).
        c.g[V1][0] = sx(r32(c, V0).wrapping_add(0xa0));
        loop {
            c.cycles += 7;
            c.lq(A0, r32(c, V0));
            c.lq(A1, bw::p(r32(c, V0), 0x10));
            c.sq(A0, r32(c, S0));
            c.sq(A1, bw::p(r32(c, S0), 0x10));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x20));
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0012_212c);
            }
        }
        // The camera-relative light at the hips.
        c.cycles += 20;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x0011_ff48,
            f: ee::s00::Rider_GetHipsPosition,
        }
        .call(c, 0x0012_2154)?;
        c.g[S0][0] = sx(rider.wrapping_add(LIGHTS));
        c.g[S4][0] = c.g[S0][0];
        c.g[S1][0] = sx(rider.wrapping_add(DYNAMIC_LIGHTS));
        c.g[S3][0] = 7;
        rider_ai_index(c, rider, 0x0012_217c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002e_efa8, 0x0012_2184)?; // WorldPainter_GetLighting30
        c.mov_s(12, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x0038_9cb8, 0x0012_2194)?; // Irradiance_AddCameraRelativeLight
                                                // The dynamic lights.
        loop {
            c.cycles += 3;
            c.g[A2][0] = c.lw(r32(c, S1));
            let none = c.g[A2][0] == 0;
            c.g[A0][0] = c.g[S4][0];
            if !none {
                c.cycles += 2;
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x0038_a6a8, 0x0012_21ac)?; // Irradiance_AddDynamicLight
            }
            c.cycles += 3;
            c.g[S3][0] = sx(r32(c, S3).wrapping_sub(1));
            let more = (c.g[S3][0] as i64) >= 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0012_2198);
            }
        }
        // The handler's directional block.
        c.cycles += 6;
        c.g[V1][0] = c.lw(rider.wrapping_add(HANDLER));
        c.g[S1][0] = 0;
        c.g[V0][0] = sx(r32(c, V1).wrapping_add(HANDLER_LIGHTS));
        if c.g[V1][0] != 0 {
            c.g[S1][0] = c.g[V0][0];
        }
        let block = r32(c, S1);
        c.g[A1][0] = sx(bw::p(sp(c), 0x10));
        if block != 0 {
            c.cycles += 13;
            c.lwc1(0, block);
            c.lwc1(1, bw::p(block, 0x4));
            c.g[S5][0] = c.g[A1][0];
            c.lwc1(2, bw::p(block, 0x8));
            c.g[A0][0] = c.g[S4][0];
            c.swc1(0, bw::p(sp(c), 0x10));
            c.g[S3][0] = 0;
            c.swc1(1, bw::p(sp(c), 0x14));
            c.swc1(2, bw::p(sp(c), 0x18));
            call_ptr(c, 0x0038_9558, 0x0012_21f8)?; // Vec3_AddTo
            c.g[V0][0] = c.lw(bw::p(block, 0xc));
            c.g[S0][0] = sx(bw::p(block, 0x18));
            if c.g[V0][0] as i64 > 0 {
                c.cycles += 3;
                c.g[S2][0] = sx(bw::p(block, 0x50));
                c.lwc1(0, r32(c, S0).wrapping_sub(0x8));
                loop {
                    c.cycles += 16;
                    c.g[A1][0] = c.g[S2][0];
                    c.lwc1(1, r32(c, S0).wrapping_sub(0x4));
                    c.g[A0][0] = c.g[S4][0];
                    c.lwc1(2, r32(c, S0));
                    c.g[A2][0] = c.g[S5][0];
                    c.swc1(0, bw::p(sp(c), 0x10));
                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                    c.swc1(1, bw::p(sp(c), 0x14));
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(0xc));
                    c.swc1(2, bw::p(sp(c), 0x18));
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x10));
                    call_ptr(c, 0x0038_9520, 0x0012_2240)?; // Irradiance_AddDirectional
                    c.g[V0][0] = c.lw(bw::p(block, 0xc));
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.lwc1(0, r32(c, S0).wrapping_sub(0x8));
                    if event_due(c) {
                        deliver_event_here(c, 0x0012_2210);
                    }
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// DIntr (0x42c078).

/// COP0 Status bit EIE (interrupts enabled).
const STATUS_EIE: u64 = 0x1_0000;

/// `int DIntr(void)` (0x42c078): disable interrupts; returns 1 if they were
/// enabled. The `di` is repeated until Status.EIE reads back clear.
pub fn d_intr(c: &mut Cpu) {
    c.cycles += 5;
    c.mfc0(V1, 12);
    c.g[V0][0] = STATUS_EIE;
    c.g[V1][0] &= c.g[V0][0];
    let was_on = c.g[V1][0] != 0;
    c.g[A0][0] = was_on as u64;
    if !was_on {
        c.cycles += 3;
        c.g[A0][0] = 0;
        c.g[V0][0] = 0;
        ret(c);
        return;
    }
    c.cycles += 1;
    loop {
        c.cycles += 7;
        c.di();
        c.mfc0(V0, 12);
        c.g[V1][0] = STATUS_EIE;
        c.g[V0][0] &= c.g[V1][0];
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0042_c090);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

// ---------------------------------------------------------------------------
// GameModule_ProcessStateChange (0x230f40).

/// The game module's state machine fields.
pub(crate) mod game_module {
    /// The loader (`GameViewMan_UpdateSplitScreen` argument).
    pub const LOADER: u32 = 0x84;
    /// The previous state object.
    pub const PREV: u32 = 0x1fc;
    /// The current state object (null for none).
    pub const CUR: u32 = 0x200;
    /// The pending (entered, still updating) state object.
    pub const PENDING: u32 = 0x204;
    /// Id (1..16) of the state to enter as pending; 0 for none.
    pub const PENDING_ID: u32 = 0x208;
    /// Id of the last entered pending state.
    pub const ENTERED_ID: u32 = 0x20c;
    /// Id (1..16) of the state to switch to; 0 for none.
    pub const NEXT_ID: u32 = 0x210;
    pub const CUR_ID: u32 = 0x214;
    pub const PREV_ID: u32 = 0x218;
    /// Argument for the next state's start.
    pub const NEXT_ARG: u32 = 0x21c;
    /// Non-zero holds the module while the global run flag is clear.
    pub const HOLD: u32 = 0x224;
    /// The state objects for ids 1..16 (each with its vtable at `+0xc`).
    pub const STATES: [u32; 16] = [
        0xb0, 0xd0, 0xe0, 0xf0, 0x100, 0x118, 0x128, 0x138, 0x148, 0x158, 0x178, 0x18c, 0x19c,
        0x1b8, 0x1dc, 0x1ec,
    ];
    /// State vtable slots.
    pub const VT_ENTER: u32 = 0x10;
    pub const VT_UPDATE: u32 = 0x18;
    pub const VT_START: u32 = 0x28;
    pub const VT_TICK: u32 = 0x30;
    pub const VT_EXIT: u32 = 0x38;
    /// The two `switch` jump tables (pending id, next id).
    pub const PENDING_TABLE: u32 = 0x0047_b7b0;
    pub const NEXT_TABLE: u32 = 0x0047_b7f0;
    /// The case labels of the two tables.
    pub const PENDING_CASE0: u32 = 0x0023_0ffc;
    pub const NEXT_CASE0: u32 = 0x0023_1124;
    pub const START: u32 = 0x0023_0f40;
    pub const END: u32 = 0x0023_124c;
}

/// `state->vt[slot](state)` (state vtable at `+0xc`), returning to `ret`.
fn game_state_call(c: &mut Cpu, obj: usize, slot: u32, ret: u32) -> crate::call::Flow {
    let state = r32(c, obj);
    c.g[V0][0] = c.lw(bw::p(state, 0xc));
    c.g[A0][0] = c.lh(bw::p(r32(c, V0), slot));
    c.g[V1][0] = c.lw(bw::p(r32(c, V0), slot + 4));
    let method = r32(c, V1);
    c.g[A0][0] = sx(state.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

/// `void GameModule_ProcessStateChange(GameModule* m)` (0x230f40): the
/// game's state machine step. Unless held (the global run flag at
/// 0x534b30 clear with `hold` set, or flag 0x20 at gp-0x6a0): tick the
/// current state (not while state 3 waits for no switch); enter a pending
/// state (`enter`) and keep updating it; then, on a requested switch, exit
/// the current state, remember it as previous, select the next (ids 1..16),
/// drop the pending state if it is the one now current, refresh the split
/// screen and start the new state with the request's argument.
pub fn game_module_process_state_change(c: &mut Cpu) {
    use game_module::*;
    if c.pc != START {
        let pc = c.pc;
        c.jump_out(pc, START, END);
        return;
    }
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw(bw::gp(c, -0x848));
        frame.save(c, S0, 0x30);
        c.g[A1][0] = 7;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let m = r32(c, S0);
        c.g[A0][0] = c.lw(bw::p(r32(c, V0), 0x78));
        call_ptr(c, 0x0014_dd58, 0x0023_0f68)?; // BELibrary_GetInterface(be, 7)
        'out: {
            c.g[V1][0] = 0x0053_0000;
            c.g[V0][0] = c.lw(0x0053_4b30);
            let running = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(bw::gp(c, -0x6a0));
            if !running {
                c.cycles += 3;
                c.g[V0][0] = c.lw(bw::p(m, HOLD));
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x30);
                    break 'out;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(bw::gp(c, -0x6a0));
            }
            c.cycles += 3;
            c.g[V0][0] &= 0x20;
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            // Tick the current state.
            c.cycles += 3;
            c.g[A1][0] = c.lw(bw::p(m, CUR));
            c.g[V0][0] = 3;
            'ticked: {
                if c.g[A1][0] != 0 {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(bw::p(m, CUR_ID));
                    if c.g[V1][0] == c.g[V0][0] {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(bw::p(m, NEXT_ID));
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = c.lw(bw::p(m, PENDING_ID));
                            break 'ticked;
                        }
                        c.cycles += 1;
                    }
                    c.g[V0][0] = c.lw(bw::p(r32(c, A1), 0xc));
                    c.cycles += 4;
                    c.g[A0][0] = c.lh(bw::p(r32(c, V0), VT_TICK));
                    c.g[V1][0] = c.lw(bw::p(r32(c, V0), VT_TICK + 4));
                    let method = r32(c, V1);
                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    call_ptr(c, method, 0x0023_0fcc)?;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(bw::p(m, PENDING_ID));
            }
            // Enter the pending state.
            c.cycles += 2;
            let none = c.g[V0][0] == 0;
            c.g[V1][0] = sx(r32(c, V0).wrapping_sub(1));
            if !none {
                c.cycles += 3;
                let in_range = c.g[V1][0] < 0x10;
                c.g[V0][0] = in_range as u64;
                c.g[V0][0] = 0x0048_0000;
                if in_range {
                    c.cycles += 6;
                    c.g[V1][0] = sx(r32(c, V1) << 2);
                    c.g[V0][0] = sx(PENDING_TABLE);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(PENDING_TABLE));
                    c.g[A0][0] = c.lw(r32(c, V1));
                    let target = r32(c, A0);
                    let k = target.wrapping_sub(PENDING_CASE0) / 8;
                    if target.wrapping_sub(PENDING_CASE0) % 8 != 0 || k >= 16 {
                        c.jump_out(target, START, END);
                        return Ok(());
                    }
                    c.cycles += if k == 15 { 1 } else { 2 };
                    c.g[V0][0] = sx(m.wrapping_add(STATES[k as usize]));
                    c.cycles += 1;
                    c.sw(bw::p(m, PENDING), c.g[V0][0]);
                }
                c.cycles += 9;
                c.g[V0][0] = c.lw(bw::p(m, PENDING_ID));
                c.sw(bw::p(m, PENDING_ID), 0);
                c.sw(bw::p(m, ENTERED_ID), c.g[V0][0]);
                c.g[A1][0] = c.lw(bw::p(m, PENDING));
                game_state_call(c, A1, VT_ENTER, 0x0023_10a0)?;
            }
            // Update the pending state.
            c.cycles += 3;
            c.g[A1][0] = c.lw(bw::p(m, PENDING));
            if c.g[A1][0] != 0 {
                c.cycles += 6;
                game_state_call(c, A1, VT_UPDATE, 0x0023_10c0)?;
            }
            c.g[S1][0] = c.lw(bw::p(m, NEXT_ID));
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            // Switch: exit the current state.
            c.cycles += 3;
            c.g[A1][0] = c.lw(bw::p(m, CUR));
            c.g[S2][0] = c.lw(bw::p(m, NEXT_ARG));
            if c.g[A1][0] != 0 {
                c.cycles += 5;
                game_state_call(c, A1, VT_EXIT, 0x0023_10ec)?;
            }
            c.cycles += 7;
            c.g[V0][0] = c.lw(bw::p(m, CUR_ID));
            c.g[A1][0] = sx(r32(c, S1).wrapping_sub(1));
            c.g[V1][0] = c.lw(bw::p(m, CUR));
            let in_range = c.g[A1][0] < 0x10;
            c.g[A0][0] = in_range as u64;
            c.sw(bw::p(m, PREV_ID), c.g[V0][0]);
            c.sw(bw::p(m, PREV), c.g[V1][0]);
            if in_range {
                c.cycles += 7;
                c.g[V0][0] = sx(NEXT_TABLE);
                c.g[V1][0] = sx(r32(c, A1) << 2);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(NEXT_TABLE));
                c.g[A0][0] = c.lw(r32(c, V1));
                let target = r32(c, A0);
                let k = target.wrapping_sub(NEXT_CASE0) / 0xc;
                if target.wrapping_sub(NEXT_CASE0) % 0xc != 0 || k >= 16 {
                    c.jump_out(target, START, END);
                    return Ok(());
                }
                c.cycles += if k == 15 { 2 } else { 3 };
                c.g[V0][0] = sx(m.wrapping_add(STATES[k as usize]));
                c.g[V1][0] = k as u64 + 1;
                c.cycles += 2;
                c.sw(bw::p(m, CUR), c.g[V0][0]);
                c.sw(bw::p(m, CUR_ID), c.g[V1][0]);
            }
            c.cycles += 5;
            c.g[V1][0] = c.lw(bw::p(m, ENTERED_ID));
            c.g[V0][0] = c.lw(bw::p(m, CUR_ID));
            c.sw(bw::p(m, NEXT_ID), 0);
            let differ = c.g[V1][0] != c.g[V0][0];
            c.sw(bw::p(m, NEXT_ARG), 0);
            if !differ {
                // The pending state is the new current one.
                c.cycles += 2;
                c.sw(bw::p(m, ENTERED_ID), 0);
                c.sw(bw::p(m, PENDING), 0);
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(bw::p(m, CUR));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'out;
            }
            // Start the new state.
            c.cycles += 10;
            c.g[A0][0] = c.lw(bw::p(m, LOADER));
            call_ptr(c, 0x0022_e550, 0x0023_1218)?; // GameViewMan_UpdateSplitScreen
            c.g[A2][0] = c.lw(bw::p(m, CUR));
            c.g[A1][0] = c.g[S2][0];
            game_state_call(c, A2, VT_START, 0x0023_1234)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
    use crate::difftest::{
        check, check_lohi, elf_image, fill, put_vec, scramble, Rng, DATA, GAME_GP,
    };
    use crate::game::visualfx::boardwakefx::tests::{f, gp_at, put_spray_slot};

    /// Emitter 7 (the carve spray): the chance inputs (side dot, lean,
    /// speed), the carving/enabled/turn gates, the side and +0x320 flips, the
    /// trail-point vtable call (a fake) and the real `DynamicEmitter_SpawnAt`
    /// on slot 7, with the idle keep-alive when any gate fails.
    #[test]
    fn dynamic_spray_fx_update_emitter7_matches() {
        use spray7::*;
        check_lohi(
            ee::s09::f_002e1a80__generated,
            dynamic_spray_fx_update_emitter7,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e1a, 2);
                c.pc = 0x002e_1a80;
                let (fx, obj, cfg, em, vt, curve, world, g, players) = (
                    DATA,
                    DATA + 0x5000,
                    DATA + 0x5200,
                    DATA + 0x3000,
                    DATA + 0x5400,
                    DATA + 0x5600,
                    DATA + 0x6000,
                    DATA + 0x6200,
                    DATA + 0x6400,
                );
                c.bus.write32(fx + OBJ, obj);
                c.bus.write32(fx + P28, em);
                c.bus.write32(fx + P2C, curve);
                c.bus.write32(fx + CFG, cfg);
                for k in 0..4u32 {
                    c.bus.write32(obj + 0x3b0 + 4 * k, r.f32(1.0));
                    c.bus.write32(fx + DIR + 4 * k, r.f32(1.0));
                    c.bus.write32(obj + 0x370 + 4 * k, r.f32(10.0));
                    c.bus.write32(obj + 0x1e0 + 4 * k, r.f32(10.0));
                    c.bus.write32(fx + NORMAL + 4 * k, r.f32(10.0));
                    c.bus.write32(fx + POS + 4 * k, r.f32(10.0));
                    c.bus.write32(fx + COL + 4 * k, r.f32(10.0));
                }
                c.bus
                    .write32(obj + 0x1f0, f([0.0f32, 0.05, -3.0][(seed % 3) as usize]));
                c.bus.write32(
                    fx + SPEED,
                    f([300.0f32, 900.0, 3000.0][((seed / 3) % 3) as usize]),
                );
                c.bus.write32(fx + ACTIVE, (seed % 5 != 0) as u32);
                c.bus.write32(cfg + 0x50, (seed % 7 != 0) as u32);
                c.bus.write32(cfg + 0x54, r.f32(4.0));
                c.bus.write32(
                    fx + TURN,
                    f([0.05f32, 0.5, -0.1][((seed / 2) % 3) as usize]),
                );
                c.bus.write32(fx + SIDE, r.f32(1.0));
                c.bus.write32(fx + UP_SCALE, r.f32(2.0));
                c.bus.write32(obj + 0x320, seed % 2);
                c.bus.write32(obj + 0x6c0, vt);
                c.bus.write16(vt + 0x10, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + 0x14, FAKE);
                for off in [0x730u32, 0x734, 0x738] {
                    c.bus.write32(curve + off, r.f32(3.0));
                }
                put_spray_slot(c, &mut r, em + SLOT, DATA + 0x8000);
                c.bus.write32(gp_at(-0x848), world);
                c.bus.write32(world + 0x84, g);
                c.bus.write32(g + 0x84, players);
                c.bus.write32(players + 0x10, 0);
                c.bus.write32(gp_at(0xa0c), r.next_u32());
                for (off, v) in [
                    (-0x3a90, 416.66669f32),
                    (-0x3a8c, 1250.0),
                    (-0x3a88, 0.1),
                    (-0x3a84, 0.4),
                    (-0x3a80, 0.2),
                    (-0x3a7c, 1.0 / 60.0),
                    (-0x3a78, 0.3),
                    (-0x3a74, 1.0 / 60.0),
                    (-0x3a70, 1.0 / 60.0),
                ] {
                    c.bus.write32(gp_at(off), f(v));
                }
                c.g[A0][0] = sx(fx);
            },
        );
    }

    /// The colour factor over the three luminance bands (dark, bright, and
    /// the gamma curve through the real `logf`/`expf`), with random colours
    /// around the thresholds.
    #[test]
    fn world_painter_compute_colour_factor_matches() {
        check(
            ee::s09::f_002edf00__generated,
            world_painter_compute_colour_factor,
            250,
            &|c, seed| {
                let mut r = Rng(0x2edf_0000 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                elf_image(c);
                let col = DATA;
                // Grey levels in the three bands, plus random colours.
                let grey = [0.05f32, 0.1, 0.2, 0.3, 0.44, 0.45, 0.8][(seed % 7) as usize];
                for k in 0..3u32 {
                    let v = if seed % 3 == 0 {
                        f32::from_bits(r.f32(1.0)).abs()
                    } else {
                        grey
                    };
                    c.bus.write32(col + 4 + 4 * k, v.to_bits());
                }
                c.g[A0][0] = sx(col);
                c.pc = 0x002e_df00;
            },
        );
    }

    /// `SND_Tick` over 0-3 tick callbacks (fakes) and 0-4 channels with
    /// random LFO, sweep and envelope states: unallocated / stopped
    /// channels, LFO wrap, sweep up/down to the target or below zero (stop),
    /// envelope ramps, the next segment (finite or forever) and the last
    /// segment (stop), with the real pitch/volume/stop callees (no voices).
    #[test]
    fn snd_tick_matches() {
        use snd_tick::*;
        check_lohi(ee::s13::f_003b55f0__generated, snd_tick, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x3b55, 3);
            let channels = DATA + 0x1_0000;
            let tables = DATA + 0x2_0000;
            fill(c, &mut r, tables, 0x400);
            c.bus.write32(STATE + TICKS, r.next_u32());
            let callbacks = r.next_u32() % 4;
            c.bus.write8(STATE + TICK_CALLBACKS, callbacks as u8);
            for i in 0..4 {
                c.bus.write32(STATE + CALLBACKS + 4 * i, FAKE + 0x20 * i);
            }
            let n = (seed / 4) % 5;
            c.bus.write16(STATE + CHANNEL_COUNT, n as u16);
            c.bus.write32(STATE + CHANNELS, channels);
            let small = |r: &mut Rng, m: u32| (r.next_u32() % (2 * m + 1)) as i32 - m as i32;
            for i in 0..n {
                let ch = channels + i * CHANNEL_SIZE as u32;
                fill(c, &mut r, ch, CHANNEL_SIZE as u32 / 4);
                let alloc = if r.next_u32() % 5 != 0 {
                    1
                } else {
                    r.next_u32() as u8 % 3
                };
                c.bus.write8(ch + ALLOCATED, alloc);
                let h = if r.next_u32() % 5 != 0 {
                    (r.next_u32() << 8 | i) & 0x7fff_ffff
                } else {
                    r.next_u32() | 0x8000_0000
                };
                c.bus.write32(ch, h);
                c.bus.write8(ch + 0x23, 0); // no voices
                c.bus.write32(
                    ch + 0x74,
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        tables + 0x80
                    },
                );
                let lfo = |r: &mut Rng| {
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        tables + 0x100
                    }
                };
                let (pl, vl) = (lfo(&mut r), lfo(&mut r));
                c.bus.write32(ch + PITCH_LFO, pl);
                c.bus.write32(ch + VOL_LFO, vl);
                for off in [PITCH_LFO_POS, PITCH_LFO_LEN, VOL_LFO_POS, VOL_LFO_LEN] {
                    c.bus.write8(ch + off, (r.next_u32() % 6) as u8);
                }
                let step = match r.next_u32() % 3 {
                    0 => 0,
                    1 => small(&mut r, 40),
                    _ => -(1 + r.next_u32() as i32 % 40).abs(),
                };
                c.bus.write32(ch + SWEEP_STEP, step as u32);
                c.bus.write32(ch + SWEEP, small(&mut r, 60) as u32);
                c.bus.write32(ch + SWEEP_TARGET, small(&mut r, 60) as u32);
                c.bus.write32(
                    ch + ENV_STEP,
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32()
                    },
                );
                c.bus.write32(
                    ch + ENV_TICKS,
                    [1u32, 1, 2, 30][(r.next_u32() % 4) as usize],
                );
                c.bus.write8(ch + ENV_SEGMENTS, (r.next_u32() % 4) as u8);
                c.bus.write8(ch + ENV_SEGMENT, (r.next_u32() % 4) as u8);
                c.bus.write32(ch + ENV_TABLE, tables + 0x200);
            }
            // Segment table: {ticks (some forever / zero), level}.
            for k in 0..8u32 {
                let ticks = match r.next_u32() % 4 {
                    0 => u32::MAX,
                    1 => 0,
                    _ => 1 + r.next_u32() % 100,
                };
                c.bus.write32(tables + 0x200 + 8 * k, ticks);
                c.bus.write32(tables + 0x204 + 8 * k, r.next_u32() % 0x8000);
            }
            c.pc = 0x003b_55f0;
        });
    }

    /// A dynamic light for `Irradiance_AddDynamicLight` (kind, position,
    /// colour, cone and falloff fields).
    fn dynamic_light(c: &mut Cpu, r: &mut Rng, light: u32, kind: u32) {
        c.bus.write32(light + 0x10, kind);
        for k in 0..3 {
            c.bus.write32(light + 4 * k, r.f32(20.0));
        }
        c.bus.write32(light + 0xc, f(1.0));
        c.bus
            .write32(light + 0x14, f(f32::from_bits(r.f32(1.5)) + 1.5));
        c.bus.write32(light + 0x18, r.f32(1.0));
        c.bus
            .write32(light + 0x1c, f(f32::from_bits(r.f32(1.0)) + 1.0));
        for k in 0..6 {
            c.bus.write32(light + 0x20 + 4 * k, r.f32(1.0));
        }
        fill(c, r, light + 0x38, 2);
        for k in 0..8 {
            c.bus.write32(light + 0x40 + 4 * k, r.f32(30.0));
        }
        let outer = f32::from_bits(r.f32(5.0)) + 5.0;
        c.bus.write32(light + 0x60, f(outer));
        c.bus.write32(light + 0x5c, f(outer + 10.0));
        c.bus.write8(light + 0x64, (r.next_u32() % 5) as u8);
        c.bus.write8(light + 0x65, (r.next_u32() % 6) as u8);
    }

    /// `Rider_BuildLightSet` with the real irradiance callees: the view
    /// irradiance copy, the camera-relative light (camera view matrix through
    /// a fake), 0-8 dynamic lights, and the handler's directional block
    /// (absent, empty, or 1-3 lights). The rider's AI-index getter and the
    /// lighting/camera methods are fakes (some seeds unwind at them).
    #[test]
    fn rider_build_light_set_matches() {
        use rider_lights::*;
        check_lohi(
            ee::s00::f_001220d8__generated,
            rider_build_light_set,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x1220, 4);
                let (rider, vt, hips, bones, handler) = (
                    DATA,
                    DATA + 0x2000,
                    DATA + 0x2100,
                    DATA + 0x2200,
                    DATA + 0x4000,
                );
                let (camera, cam_vt, view) = (DATA + 0x5000, DATA + 0x6000, DATA + 0x5100);
                fill(c, &mut r, rider + LIGHTS, 0x28);
                c.bus.write32(rider + VTABLE, vt);
                c.bus
                    .write16(vt + AI_INDEX_SLOT, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + AI_INDEX_SLOT + 4, FAKE);
                // The view index from both getter calls (forced fake results).
                let view_index = 1 + seed % 3;
                c.bus.write32(LOG_RESULTS, view_index);
                c.bus.write32(LOG_RESULTS + 4, 1 + (seed / 3) % 3);
                c.bus.write32(LOG_RESULTS + 12, view);
                for i in 1..4u32 {
                    fill(c, &mut r, 0x004f_a3c0 + i * 0xf0, 0x28);
                    let (cell, obj, ovt) = (
                        DATA + 0x7000 + i * 0x40,
                        DATA + 0x7400 + i * 0x40,
                        DATA + 0x7800 + i * 0x200,
                    );
                    c.bus.write32(0x004f_a370 + i * 0xf0 + 0x1c, cell);
                    c.bus.write32(cell, obj);
                    c.bus.write32(obj + 4, ovt);
                    c.bus.write16(ovt + 0x150, (r.next_u32() & 0x7c) as u16);
                    c.bus.write32(ovt + 0x154, FAKE + 0x10);
                }
                // Hips: rider+0x780 -> skeleton, +0x2c bones (32 bytes), +0x89c index.
                c.bus.write32(rider + 0x780, hips);
                c.bus.write32(hips + 0x2c, bones);
                c.bus.write32(rider + 0x89c, r.next_u32() % 4);
                for k in 0..32 {
                    c.bus.write32(bones + 4 * k, r.f32(50.0));
                }
                // The camera (gp+0x2a90) and its view matrix.
                c.bus.write32(GAME_GP + 0x2a90, camera);
                c.bus.write32(camera + 0x10d8, cam_vt);
                c.bus.write16(cam_vt + 0x118, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(cam_vt + 0x11c, FAKE + 0x20);
                for k in 0..16 {
                    c.bus.write32(view + 4 * k, r.f32(2.0));
                }
                // Dynamic lights: none, or one of two lights per slot.
                let (l1, l2) = (DATA + 0x3000, DATA + 0x3100);
                dynamic_light(c, &mut r, l1, [1u32, 2, 0, 3][(seed % 4) as usize]);
                dynamic_light(c, &mut r, l2, [2u32, 1, 3, 0][(seed / 4 % 4) as usize]);
                for k in 0..8u32 {
                    let l = match (r.next_u32() % 4, seed % 5) {
                        (_, 0) => 0,
                        (0, _) => l1,
                        (1, _) => l2,
                        _ => 0,
                    };
                    c.bus.write32(rider + DYNAMIC_LIGHTS + 4 * k, l);
                }
                // The handler's directional block.
                c.bus
                    .write32(rider + HANDLER, if seed % 4 == 1 { 0 } else { handler });
                let block = handler + HANDLER_LIGHTS;
                for k in 0..3 {
                    c.bus.write32(block + 4 * k, r.f32(5.0));
                }
                c.bus.write32(block + 0xc, (seed / 2) % 4);
                for k in 0..12 {
                    c.bus.write32(block + 0x10 + 4 * k, r.f32(5.0));
                    c.bus.write32(block + 0x50 + 4 * k, r.f32(1.0));
                }
                c.bus.write32(block + 0x80, r.f32(1.0));
                c.g[A0][0] = sx(rider);
                c.pc = 0x0012_20d8;
            },
        );
    }

    /// `DIntr` with interrupts on and off (random other Status bits).
    #[test]
    fn d_intr_matches() {
        check(ee::s15::f_0042c078__generated, d_intr, 64, &|c, seed| {
            let mut r = Rng(0x42c0_7800 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let status = r.next_u32() & !(STATUS_EIE as u32);
            c.cop0[12] = if seed % 2 == 0 {
                status
            } else {
                status | STATUS_EIE as u32
            };
            c.pc = 0x0042_c078;
        });
    }

    /// `GameModule_ProcessStateChange` over the hold conditions, the tick
    /// (state 3 waiting on a switch), pending ids 0..17 and next ids 0..17
    /// (17: outside the `switch`), with every state method a fake (some
    /// seeds unwind at one) and `GameViewMan_UpdateSplitScreen` on its short
    /// path.
    #[test]
    fn game_module_process_state_change_matches() {
        use game_module::*;
        check(
            ee::s06::f_00230f40__generated,
            game_module_process_state_change,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x230f, 6);
                let (m, world, g, be, be_vt, svt, loader) = (
                    DATA,
                    DATA + 0x6000,
                    DATA + 0x6200,
                    DATA + 0x6400,
                    DATA + 0x6500,
                    DATA + 0x7000,
                    DATA + 0x6800,
                );
                fill(c, &mut r, m, 0x90);
                // BELibrary_GetInterface(world->be, 7): a fake interface method.
                c.bus.write32(gp_at(-0x848), world);
                c.bus.write32(world + 0x78, r.next_u32());
                c.bus.write32(gp_at(-0x1efc), 0);
                c.bus.write32(0x0053_08a8, be);
                c.bus.write32(be + 4, be_vt);
                c.bus.write16(be_vt + 8, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(be_vt + 0xc, FAKE + 0x40);
                // GameViewMan_UpdateSplitScreen: the view count is unchanged.
                c.bus.write32(world + 0x84, g);
                let views = r.next_u32();
                c.bus.write32(g + 0x214, views);
                c.bus.write32(m + LOADER, loader);
                c.bus.write32(loader + 0xc, views);
                // The state objects share a vtable of fakes.
                for off in STATES {
                    c.bus.write32(m + off + 0xc, svt);
                }
                for (i, slot) in [VT_ENTER, VT_UPDATE, VT_START, VT_TICK, VT_EXIT]
                    .into_iter()
                    .enumerate()
                {
                    c.bus.write16(svt + slot, (r.next_u32() & 0x3c) as u16);
                    c.bus.write32(svt + slot + 4, FAKE + 0x50 + 0x10 * i as u32);
                }
                for k in 0..16u32 {
                    c.bus.write32(PENDING_TABLE + 4 * k, PENDING_CASE0 + 8 * k);
                    c.bus.write32(NEXT_TABLE + 4 * k, NEXT_CASE0 + 0xc * k);
                }
                // Hold: the run flag, the module's hold, and gp-0x6a0 bit 0x20.
                c.bus.write32(0x0053_4b30, (seed % 7 != 0) as u32);
                c.bus
                    .write32(m + HOLD, if seed % 2 == 0 { 0 } else { r.next_u32() });
                let flags = r.next_u32() & !0x20;
                c.bus.write32(
                    gp_at(-0x6a0),
                    if seed % 11 == 3 { flags | 0x20 } else { flags },
                );
                let state = |r: &mut Rng| {
                    if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        m + STATES[(r.next_u32() % 16) as usize]
                    }
                };
                let cur = state(&mut r);
                c.bus.write32(m + CUR, cur);
                let pending = state(&mut r);
                c.bus.write32(m + PENDING, pending);
                let id = |r: &mut Rng| match r.next_u32() % 5 {
                    0 => 0,
                    1 => 17 + r.next_u32() % 3,
                    _ => 1 + r.next_u32() % 16,
                };
                c.bus
                    .write32(m + CUR_ID, if seed % 3 == 0 { 3 } else { id(&mut r) });
                c.bus.write32(m + NEXT_ID, id(&mut r));
                c.bus.write32(m + PENDING_ID, id(&mut r));
                c.bus.write32(
                    m + ENTERED_ID,
                    if seed % 2 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 16
                    },
                );
                c.bus.write32(m + NEXT_ARG, r.next_u32());
                c.g[A0][0] = sx(m);
                c.pc = START;
            },
        );
    }
}
