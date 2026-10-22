//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! `cBoardWakeFX_Update` (0x002dd0b8) is the board-wake trail update: it ages
//! the ring of cross-sections the rider leaves in the snow, transforms the
//! newest section through the rider's matrix, and rebuilds the trail strip
//! mesh (`cBoardWakeFX_BuildTrail`) and its ring cursor
//! (`cBoardWakeFX_AdvanceRing`). It is a `match pc` dispatcher because the
//! generated body resumes at the loop heads after an event unwind.

use crate::call::{call_ptr, guest, Callee};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x002d_d0b8,
    c_board_wake_fx_update,
    "cBoardWakeFX_Update",
)];

/// The wake-trail object's fields (reference `sWakeDCF28` in
/// `src/visualfx/boardwakefx.cpp`).
#[allow(dead_code)]
mod wake {
    /// The rider this trail belongs to.
    pub const OBJ: u32 = 0x00;
    /// Number of live sections (the outer loop bound).
    pub const LIVE: u32 = 0x04;
    /// Section cursor: `(first + i) % max` picks the section slot.
    pub const CURSOR: u32 = 0x08;
    /// `sWakeStrip[5]`: the cross-sections, 0x70 bytes each.
    pub const STRIPS: u32 = 0x0c;
    /// Five element rings, one per point of a section (`sWakeElemBW`).
    pub const ROWS: u32 = 0x10;
    /// Five per-ring weights (the points fade out down the section).
    pub const VALS: u32 = 0x24;
    /// Ring time (advanced by `cBoardWakeFX_AdvanceRing`).
    pub const RING: u32 = 0x38;
    /// Set once the current trail mesh has been built.
    pub const BUILT: u32 = 0x3c;
    /// Scratch float.
    pub const F40: u32 = 0x40;
    /// A 0x40-byte sub-object.
    pub const SUB: u32 = 0x60;
    /// Trail colour (four floats).
    pub const COL: u32 = 0xa0;
    /// Non-zero while a previous point exists.
    pub const HAS_PREV: u32 = 0xb0;
    /// Points per section (`count` in the reference struct).
    pub const SPAN: u32 = 0xb4;
    /// Slots in a cross-section ring (`fB8`).
    pub const MAX: u32 = 0xb8;
    /// Section lifetime in seconds (`fBC`).
    pub const LIFE: u32 = 0xbc;
}

/// `int Rider_GetMainState(Rider*)` (0x11fe98), already hand-written.
const RIDER_GET_MAIN_STATE: u32 = 0x0011_fe98;
/// `void cBoardWakeFX_AdvanceRing(cBoardWakeFX*)` (0x2de058).
const ADVANCE_RING: Callee = Callee {
    addr: 0x002d_e058,
    f: ee::s09::cBoardWakeFX_AdvanceRing,
};
/// `void cBoardWakeFX_BuildTrail(cBoardWakeFX*, Vec4*, Vec4*, float)`
/// (0x2ddd30).
const BUILD_TRAIL: Callee = Callee {
    addr: 0x002d_dd30,
    f: ee::s09::cBoardWakeFX_BuildTrail,
};

/// `void cBoardWakeFX_Update(cBoardWakeFX* self)` (0x002dd0b8).
///
/// `self+0` is the rider, `self+0x38` the ring time, `self+0x3c` the
/// "current trail built" flag. The section array is `self+0x0c`; each slot is
/// 0x70 bytes: a 0x20-byte header, five 0x10-byte points (age at +0x64 of the
/// first) and the row of 0x10-byte element rings at `self+0x10` indexed by
/// section slot.
pub fn c_board_wake_fx_update(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                // Entry: frame, world/game/rider indices, then the age loop.
                0x002d_d0b8 => {
                    c.cycles += 21;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x170));
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[A2][0] = 0xb0;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.g[A3][0] = 0u64;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x158));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x150));
                    // a1 = self->obj; world->+0x84 = game; game->+0x44 + obj
                    // index*0xb0 = this rider's race data (s3).
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x438));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                    c.mult(0, 2, 2, 6);
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    let no_live = (c.g[V1][0] as i64) <= 0;
                    c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    pc = if no_live { 0x002d_d2a8 } else { 0x002d_d10c };
                }
                // Constant setup for the age loop (dt, 20, 1.0, 0, 0x70, 0x30).
                0x002d_d10c => {
                    c.cycles += 13;
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3c34));
                    c.g[T4][0] = 0u64;
                    c.g[AT][0] = 0x41a00000;
                    c.mtc1(1, 7);
                    c.g[T3][0] = 0x70;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 3);
                    c.g[T2][0] = 0x30;
                    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3c30));
                    c.g[T1][0] = 0x30;
                    c.mtc1(0, 5);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                    pc = 0x002d_d140;
                }
                // Section slot = (cursor + i) % max; age += dt.
                0x002d_d140 => {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xb8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                    c.div(0, 2, 3);
                    // The original compares the divisor with t4 (0) and both
                    // arms continue here.
                    pc = 0x002d_d154;
                }
                // Strip address = strips + slot*0x70; compare age with life.
                0x002d_d154 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
                    c.g[A2][0] = c.hi[0];
                    c.mult(0, 2, 6, 11);
                    c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x64));
                    c.add_s(1, 0, 4);
                    c.swc1(1, (c.g[A1][0] as u32).wrapping_add(0x64));
                    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0xbc));
                    c.c_lt_s(0, 1);
                    let expired = c.fcc();
                    c.mult(0, 4, 6, 10);
                    pc = if expired { 0x002d_d378 } else { 0x002d_d184 };
                }
                // The section is alive: fade the point and write the first
                // element ring of the section.
                0x002d_d184 => {
                    c.cycles += 36;
                    c.mul_s(0, 1, 6);
                    c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x60));
                    c.mul_s(1, 1, 7);
                    c.g[T0][0] = 0x1;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
                    c.sub_s(0, 3, 0);
                    c.min_s(1, 1, 3);
                    c.sub_s(1, 3, 1);
                    c.max_s(0, 0, 5);
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[A1][0] as u32).wrapping_add(0x50));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.mul_s(1, 1, 2);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.mfc1(3, 4);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.add_s(1, 1, 3);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.sq(2, (c.g[SP][0] as u32));
                    c.mul_s(2, 1, 4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xb4));
                    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let one_point = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                    pc = if one_point { 0x002d_d294 } else { 0x002d_d214 };
                }
                // Element stride (0x30) and the per-point ring array.
                0x002d_d214 => {
                    c.cycles += 3;
                    c.mult(0, 6, 6, 9);
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
                    pc = 0x002d_d220;
                }
                // Inner loop: add the offset of every further point of the
                // section to its element ring (0x10 bytes per element).
                0x002d_d220 => {
                    c.cycles += 29;
                    c.mfc1(2, 2);
                    c.lqc2(4, c.g[A0][0] as u32);
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x14));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[V1][0] as u32).wrapping_add(0x20));
                    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.lwc1(1, (c.g[A1][0] as u32));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                    c.add_s(0, 0, 1);
                    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xb4));
                    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more_points = c.g[V0][0] != 0u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                    if more_points {
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_d220);
                        }
                        pc = 0x002d_d220;
                    } else {
                        pc = 0x002d_d294;
                    }
                }
                // Next live section?
                0x002d_d294 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                        if event_due(c) {
                            deliver_event_here(c, 0x002d_d140);
                        }
                        pc = 0x002d_d140;
                    } else {
                        pc = 0x002d_d2a4;
                    }
                }
                0x002d_d2a4 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    pc = 0x002d_d2a8;
                }
                // Per-frame trail state: read the rider's transform, blend the
                // two trail vectors of the newest point.
                0x002d_d2a8 => {
                    c.cycles += 21;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 2);
                    c.lq(2, (c.g[A1][0] as u32).wrapping_add(0x370));
                    c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x1f0));
                    c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x214));
                    c.sq(2, (c.g[SP][0] as u32));
                    c.abs_s(0, 0);
                    c.sub_s(2, 2, 0);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                    c.neg_s(21, 1);
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                    c.abs_s(1, 21);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x34));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.lq(2, c.g[A0][0] as u32);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x320));
                    let no_grind = c.g[V1][0] == 0u64;
                    c.mul_s(22, 1, 2);
                    pc = if no_grind { 0x002d_d324 } else { 0x002d_d2fc };
                }
                0x002d_d2fc => {
                    c.cycles += 10;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 0);
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    pc = 0x002d_d324;
                }
                0x002d_d324 => {
                    c.cycles += 12;
                    c.mtc1(0, 0);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lqc2(5, (c.g[SP][0] as u32));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                    c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.c_lt_s(21, 0);
                    let positive = !c.fcc();
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                    pc = if positive { 0x002d_d380 } else { 0x002d_d354 };
                }
                0x002d_d354 => {
                    c.cycles += 9;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 0);
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                    pc = 0x002d_d380;
                }
                // The trail section expired: remember how many are live and
                // back to the per-frame state.
                0x002d_d378 => {
                    c.cycles += 2;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[A3][0]);
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_d2a4);
                    }
                    pc = 0x002d_d2a4;
                }
                // Length of the newest point's velocity in the XZ plane.
                0x002d_d380 => {
                    c.cycles += 18;
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3c2c));
                    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x1e0));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 1);
                    c.c_lt_s(0, 1);
                    let fast = c.fcc();
                    pc = if fast { 0x002d_d3dc } else { 0x002d_d3c8 };
                }
                0x002d_d3c8 => {
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_le_s(0, 1);
                    let slow = !c.fcc();
                    c.mov_s(6, 0);
                    pc = if slow { 0x002d_d420 } else { 0x002d_d3dc };
                }
                0x002d_d3dc => {
                    c.cycles += 16;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3c28));
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x1e0));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(1, 0);
                    let clamp = !c.fcc();
                    c.mov_s(6, 0);
                    pc = if clamp { 0x002d_d420 } else { 0x002d_d41c };
                }
                0x002d_d41c => {
                    c.cycles += 1;
                    c.mov_s(6, 1);
                    pc = 0x002d_d420;
                }
                // Trail intensity: distance to the game camera's focus.
                0x002d_d420 => {
                    c.cycles += 8;
                    c.lwc1(0, (c.g[S3][0] as u32).wrapping_add(0x48));
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 4);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x320));
                    let no_grind = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                    pc = if no_grind { 0x002d_d448 } else { 0x002d_d440 };
                }
                0x002d_d440 => {
                    c.cycles += 2;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 4);
                    pc = 0x002d_d448;
                }
                // Blur/fade amount: the second trail vector scaled by a speed
                // factor and the board's matrix.
                0x002d_d448 => {
                    c.cycles += 63;
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                    c.lwc1(0, (c.g[A1][0] as u32).wrapping_add(0x208));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[AT][0] = 0x42ec0000;
                    c.mtc1(1, 1);
                    c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x30));
                    c.abs_s(0, 0);
                    c.mul_s(0, 0, 1);
                    c.g[AT][0] = 0xffffffffc2200000;
                    c.mtc1(1, 2);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[AT][0] = 0x41700000;
                    c.mtc1(1, 1);
                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                    c.add_s(0, 0, 2);
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                    c.mtc1(0, 5);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 2);
                    c.lq(2, c.g[V1][0] as u32);
                    c.c_lt_s(21, 5);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.mfc1(3, 0);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.mfc1(3, 4);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A1][0] = c.lw(c.g[S2][0] as u32);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x780));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8a4));
                    c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x20));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.mfc1(3, 1);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    let keep = !c.fcc();
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    pc = if keep { 0x002d_d54c } else { 0x002d_d544 };
                }
                0x002d_d544 => {
                    c.cycles += 2;
                    c.g[AT][0] = 0xffffffffbf800000;
                    c.mtc1(1, 2);
                    pc = 0x002d_d54c;
                }
                // Combine the two vectors, the two trail colours and the
                // section's fade into the final colour and width.
                0x002d_d54c => {
                    c.cycles += 61;
                    c.mfc1(2, 2);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3c24));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.mov_s(7, 5);
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 1);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.mfc1(3, 4);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.mul_s(1, 22, 1);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 2);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[AT][0] = 0x3e800000;
                    c.mtc1(1, 4);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x1e0));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().rsqrt(0, 3, 4, 0);
                    c.vu0().waitq();
                    c.vu0().mul(0b1111, 5, 3, Src::Q);
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x3b0));
                    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(4, 4);
                    c.mtc1(4, 0);
                    c.abs_s(0, 0);
                    c.mul_s(0, 0, 3);
                    c.add_s(3, 0, 1);
                    c.min_s(3, 3, 2);
                    c.add_s(0, 3, 3);
                    c.c_le_s(5, 0);
                    if c.fcc() {
                        c.min_s(7, 0, 4);
                    }
                    pc = 0x002d_d640;
                }
                // Fade the colour with the trail brightness.
                0x002d_d640 => {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3c20));
                    c.c_lt_s(0, 6);
                    if !c.fcc() {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x84));
                        pc = 0x002d_d680;
                    } else {
                        pc = 0x002d_d654;
                    }
                }
                0x002d_d654 => {
                    c.cycles += 8;
                    c.sub_s(1, 3, 4);
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3c1c));
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3c18));
                    c.mul_s(1, 1, 0);
                    c.c_le_s(5, 1);
                    let below = !c.fcc();
                    c.mov_s(4, 5);
                    pc = if below { 0x002d_d678 } else { 0x002d_d674 };
                }
                0x002d_d674 => {
                    c.cycles += 1;
                    c.min_s(4, 1, 2);
                    pc = 0x002d_d678;
                }
                0x002d_d678 => {
                    c.cycles += 2;
                    c.mov_s(5, 4);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x84));
                    pc = 0x002d_d680;
                }
                0x002d_d680 => {
                    c.cycles += 3;
                    c.mtc1(0, 20);
                    if c.g[V0][0] == 0u64 {
                        c.add_s(0, 7, 5);
                        pc = 0x002d_d6d8;
                    } else {
                        pc = 0x002d_d68c;
                    }
                }
                0x002d_d68c => {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3c14));
                    c.c_lt_s(0, 6);
                    if !c.fcc() {
                        c.add_s(0, 7, 5);
                        pc = 0x002d_d6d8;
                    } else {
                        pc = 0x002d_d6a0;
                    }
                }
                0x002d_d6a0 => {
                    c.cycles += 11;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3c10));
                    c.g[AT][0] = 0x420c0000;
                    c.mtc1(1, 1);
                    c.sub_s(0, 3, 0);
                    c.g[AT][0] = 0x40900000;
                    c.mtc1(1, 2);
                    c.mul_s(0, 0, 1);
                    c.c_le_s(20, 0);
                    let below = !c.fcc();
                    c.mov_s(4, 20);
                    pc = if below { 0x002d_d6d0 } else { 0x002d_d6cc };
                }
                0x002d_d6cc => {
                    c.cycles += 1;
                    c.min_s(4, 0, 2);
                    pc = 0x002d_d6d0;
                }
                0x002d_d6d0 => {
                    c.cycles += 2;
                    c.mov_s(20, 4);
                    c.add_s(0, 7, 5);
                    pc = 0x002d_d6d8;
                }
                // Final width, then the "trail is dead" test.
                0x002d_d6d8 => {
                    c.cycles += 9;
                    c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x4c));
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[S0][0] = 0x1;
                    c.mul_s(0, 6, 0);
                    c.mul_s(0, 0, 1);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                    call_ptr(c, RIDER_GET_MAIN_STATE, 0x002d_d6f4)?;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        pc = 0x002d_d760;
                    } else {
                        pc = 0x002d_d6fc;
                    }
                }
                // Airborne check and speed gates: kill a stale trail.
                0x002d_d6fc => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x3c0c));
                    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x274));
                    c.abs_s(0, 0);
                    c.c_lt_s(2, 0);
                    if c.fcc() {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        pc = 0x002d_d760;
                    } else {
                        pc = 0x002d_d71c;
                    }
                }
                0x002d_d71c => {
                    c.cycles += 6;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3c08));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.c_lt_s(0, 1);
                    if c.fcc() {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        pc = 0x002d_d760;
                    } else {
                        pc = 0x002d_d734;
                    }
                }
                0x002d_d734 => {
                    c.cycles += 4;
                    c.c_lt_s(22, 2);
                    if c.fcc() {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        pc = 0x002d_d760;
                    } else {
                        pc = 0x002d_d744;
                    }
                }
                0x002d_d744 => {
                    c.cycles += 6;
                    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3c04));
                    c.c_lt_s(1, 0);
                    if !c.fcc() {
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3c));
                        pc = 0x002d_d768;
                    } else {
                        pc = 0x002d_d75c;
                    }
                }
                0x002d_d75c => {
                    c.cycles += 1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                    pc = 0x002d_d760;
                }
                0x002d_d760 => {
                    c.cycles += 2;
                    c.g[S0][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x3c));
                    pc = 0x002d_d768;
                }
                // Already built?
                0x002d_d768 => {
                    c.cycles += 2;
                    let built = c.g[V0][0] == 0u64;
                    if built { pc = 0x002d_d9e4 } else { pc = 0x002d_d770 };
                }
                0x002d_d770 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xb0));
                    let no_prev = c.g[V0][0] == 0u64;
                    if no_prev { pc = 0x002d_d798 } else { pc = 0x002d_d77c };
                }
                0x002d_d77c => {
                    c.cycles += 5;
                    c.mtc1(0, 0);
                    c.c_lt_s(21, 0);
                    if c.fcc() {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                        pc = 0x002d_d7ac;
                    } else {
                        pc = 0x002d_d790;
                    }
                }
                0x002d_d790 => {
                    c.cycles += 2;
                    pc = 0x002d_d7b0;
                }
                0x002d_d798 => {
                    c.cycles += 4;
                    c.mtc1(0, 0);
                    c.c_lt_s(0, 21);
                    let not_positive = !c.fcc();
                    if not_positive {
                        pc = 0x002d_d7b0;
                    } else {
                        pc = 0x002d_d7a8;
                    }
                }
                0x002d_d7a8 => {
                    c.cycles += 1;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                    pc = 0x002d_d7ac;
                }
                0x002d_d7ac => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x90), 0u64);
                    pc = 0x002d_d7b0;
                }
                // Second airborne check before building the strip.
                0x002d_d7b0 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    call_ptr(c, RIDER_GET_MAIN_STATE, 0x002d_d7b8)?;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x94), 0u64);
                        pc = 0x002d_d9c4;
                    } else {
                        pc = 0x002d_d7c0;
                    }
                }
                // Blend trail colour 1 with the section fade and advance the
                // ring; store the resulting position/width when it moved.
                0x002d_d7c0 => {
                    c.cycles += 83;
                    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3c00));
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.mfc1(2, 5);
                    c.lqc2(4, (c.g[S2][0] as u32).wrapping_add(0x60));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3bfc));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3bf8));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mfc1(3, 4);
                    c.lqc2(4, (c.g[SP][0] as u32));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3bf4));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3bf0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x60));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.mfc1(3, 5);
                    c.lqc2(4, (c.g[S2][0] as u32).wrapping_add(0x70));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mfc1(3, 4);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x70));
                    c.mfc1(3, 5);
                    c.lqc2(4, (c.g[S2][0] as u32).wrapping_add(0x80));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.mfc1(3, 4);
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x94));
                    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
                    c.mul_s(1, 1, 0);
                    c.lwc1(3, (c.g[S2][0] as u32).wrapping_add(0x90));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.mul_s(2, 2, 6);
                    c.mul_s(3, 3, 5);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.mul_s(0, 0, 4);
                    c.add_s(1, 1, 2);
                    c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x80));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.add_s(3, 3, 0);
                    c.c_lt_s(7, 1);
                    c.swc1(1, (c.g[S2][0] as u32).wrapping_add(0x94));
                    let small = !c.fcc();
                    c.swc1(3, (c.g[S2][0] as u32).wrapping_add(0x90));
                    pc = if small { 0x002d_d9ac } else { 0x002d_d90c };
                }
                0x002d_d90c => {
                    c.cycles += 5;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3bec));
                    c.c_lt_s(0, 3);
                    if !c.fcc() {
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x94), 0u64);
                        pc = 0x002d_d9b0;
                    } else {
                        pc = 0x002d_d920;
                    }
                }
                // The new segment is long enough: advance the ring and build.
                0x002d_d920 => {
                    c.cycles += 23;
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lqc2(4, (c.g[S2][0] as u32).wrapping_add(0x50));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[AT][0] = 0x42480000;
                    c.mtc1(1, 1);
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.vu0().sqrt(4, 0);
                    c.vu0().waitq();
                    c.cfc2(3, 22);
                    c.mtc1(3, 0);
                    c.c_lt_s(1, 0);
                    let short = !c.fcc();
                    c.g[A1][0] = c.g[S3][0];
                    if short { pc = 0x002d_d990 } else { pc = 0x002d_d97c };
                }
                0x002d_d97c => {
                    c.cycles += 5;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002d_d988);
                    c.g[A0][0] = c.g[S2][0];
                    c.pc = 0x002d_e058;
                    ADVANCE_RING.call(c, 0x002d_d988)?;
                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x50));
                    pc = 0x002d_d990;
                }
                0x002d_d990 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002d_d9a4);
                    c.mov_s(12, 20);
                    c.pc = 0x002d_dd30;
                    BUILD_TRAIL.call(c, 0x002d_d9a4)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                    pc = 0x002d_da90;
                }
                0x002d_d9ac => {
                    c.cycles += 1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x94), 0u64);
                    pc = 0x002d_d9b0;
                }
                0x002d_d9b0 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3c), 0u64);
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    pc = 0x002d_d9d4;
                }
                0x002d_d9c4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3c), 0u64);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    pc = 0x002d_d9d4;
                }
                0x002d_d9d4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002d_d9dc);
                    c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x94));
                    c.pc = 0x002d_dd30;
                    BUILD_TRAIL.call(c, 0x002d_d9dc)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                    pc = 0x002d_da90;
                }
                // Nothing built yet: start the trail from the stored point.
                0x002d_d9e4 => {
                    c.cycles += 2;
                    let have_point = c.g[S0][0] == 0u64;
                    c.g[A2][0] = 0x1;
                    if have_point { pc = 0x002d_da8c } else { pc = 0x002d_d9ec };
                }
                0x002d_d9ec => {
                    c.cycles += 13;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x90), 0u64);
                    c.lwc1(20, (c.g[S2][0] as u32).wrapping_add(0x90));
                    c.lq(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(4, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.c_lt_s(20, 21);
                    c.lq(2, c.g[SP][0] as u32);
                    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(5, (c.g[S2][0] as u32).wrapping_add(0x80));
                    c.sq(4, (c.g[S2][0] as u32).wrapping_add(0x70));
                    c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x60));
                    c.sq(3, (c.g[S2][0] as u32).wrapping_add(0x50));
                    let wide = c.fcc();
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x40), 0u64);
                    if wide { pc = 0x002d_da24 } else { pc = 0x002d_da20 };
                }
                0x002d_da20 => {
                    c.cycles += 1;
                    c.g[A2][0] = 0u64;
                    pc = 0x002d_da24;
                }
                // Two ring advances and two strip builds, then mark built.
                0x002d_da24 => {
                    c.cycles += 26;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0xb0), c.g[A2][0]);
                    c.swc1(20, (c.g[S2][0] as u32).wrapping_add(0x94));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002d_da40);
                    c.g[A2][0] = c.g[S0][0];
                    c.pc = 0x002d_e058;
                    ADVANCE_RING.call(c, 0x002d_da40)?;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.mov_s(12, 20);
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002d_da58);
                    c.g[A2][0] = c.g[S1][0];
                    c.pc = 0x002d_dd30;
                    BUILD_TRAIL.call(c, 0x002d_da58)?;
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002d_da70);
                    c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x94));
                    c.pc = 0x002d_e058;
                    ADVANCE_RING.call(c, 0x002d_da70)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.mov_s(12, 20);
                    c.g[RA][0] = sx(0x002d_da84);
                    c.g[A0][0] = c.g[S2][0];
                    c.pc = 0x002d_dd30;
                    BUILD_TRAIL.call(c, 0x002d_da84)?;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                    pc = 0x002d_da8c;
                }
                0x002d_da8c => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                    pc = 0x002d_da90;
                }
                // Epilogue: the caller's callee-saved registers and FPU
                // temporaries, then return.
                0x002d_da90 => {
                    c.cycles += 9;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x158));
                    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x150));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002d_d0b8, 0x002d_dab4);
                    return Ok(());
                }
            }
        }
    })
}

// FUNCS_END

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{compare, fill, put_words, DATA, GAME_GP};

    const SEEDS: u32 = 200;

    /// Every compared state must match: hard, caller-saved (`soft`) and LO/HI.
    fn strict(g: GuestFn, h: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) {
        for seed in 0..seeds {
            let r = compare(g, h, &|c| setup(c, seed));
            assert!(
                r.hard.is_empty() && r.soft.is_empty() && r.lohi.is_empty(),
                "seed {seed}: hard={:#?} soft={:#?} lohi={:#?}",
                r.hard,
                r.soft,
                r.lohi
            );
        }
    }

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    // Fixture layout (all inside RAM, away from the stack).
    const SELF: u32 = DATA;
    const OBJ: u32 = DATA + 0x1000;
    const OWNER: u32 = DATA + 0x2000;
    const MODEL: u32 = DATA + 0x3000;
    const MATS: u32 = DATA + 0x4000;
    const VT: u32 = DATA + 0x5000;
    const GAME: u32 = DATA + 0x6000;
    const S3: u32 = DATA + 0x7000;
    const STRIPS: u32 = DATA + 0x8000;
    const ROWS: u32 = DATA + 0x9000;
    const WORLD: u32 = DATA + 0xe000;
    const BE: u32 = DATA + 0xf000;

    /// The game's float constants at `gp-0x3c34..gp-0x3bd8` (dt, 20.0, 0.8,
    /// 1.0, the camera and speed thresholds, ...): the test machine's RAM
    /// starts empty, so restore them from the executable's image.
    const GP_CONSTS: [u32; 25] = [
        0x3c888889, 0x40555555, 0x44d05556, 0x44d05556, 0x3f4ccccd, 0x440ae38e, 0x402ccccd,
        0x3eb33333, 0x448ae38e, 0x3f0a3d71, 0x3e19999a, 0x43d05556, 0x3d4ccccd, 0x3f4ccccd,
        0x3e4ccccc, 0x3f333333, 0x3e99999a, 0x3d4ccccd, 0x425e38e4, 0x3f4ccccd, 0x3f59999a,
        0x3f666666, 0x3d088889, 0x3f19999a, 0x3ecccccd,
    ];

    /// A wake-trail object whose rider, model, matrix row, element rings and
    /// cross-section array all live in the fixture's scratch area. Counts and
    /// cursors stay small so the real `BuildTrail`/`AdvanceRing` callees finish
    /// in a bounded number of steps; the rider's `+0x6c0` vtable slot points at
    /// an unmapped address so `AdvanceRing`'s virtual call goes through the
    /// test machine's fake-call hook.
    fn setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x151_0000, 8);

        // Game-wide singletons and the float constants this function reads.
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        put_words(c, GAME_GP.wrapping_sub(0x3c34), &GP_CONSTS);

        // world -> game object -> this rider's race data (s3).
        w(c, WORLD + 0x84, GAME);
        w(c, GAME + 0x44, S3);

        // Random contents everywhere the function reads floats from...
        fill(c, &mut r, SELF, 0x40);
        fill(c, &mut r, OBJ, 0x400);
        fill(c, &mut r, MATS, 0x200);
        fill(c, &mut r, S3, 0x100);
        fill(c, &mut r, STRIPS, 0x100);
        for i in 0u32..5 {
            fill(c, &mut r, ROWS + 0x1000 * i, 0x100);
        }

        // ...then the pointers and counters that bound the loops.
        w(c, SELF + wake::OBJ, OBJ);
        w(c, SELF + wake::LIVE, seed % 3);
        w(c, SELF + wake::CURSOR, (seed / 3) % 3);
        w(c, SELF + wake::SPAN, 1 + (seed / 9) % 3);
        w(c, SELF + wake::MAX, 1 + (seed / 27) % 4);
        for i in 0u32..5 {
            w(c, SELF + wake::ROWS + 4 * i, ROWS + 0x1000 * i);
        }
        // Some seeds force the section to expire (life is tiny) and cover the
        // "no previous point"/"built" flags both ways.
        w(
            c,
            SELF + wake::LIFE,
            if seed % 8 == 0 { 0x3c800000 } else { r.edge_f32() },
        );
        w(c, SELF + wake::BUILT, (seed / 4) % 2);
        w(c, SELF + wake::HAS_PREV, (seed / 5) % 2);

        w(c, OBJ + 0x438, (seed / 8) % 3);
        w(c, OBJ + 0x77c, OWNER);
        w(c, OBJ + 0x780, MODEL);
        w(c, OBJ + 0x6c0, VT);
        w(c, OBJ + 0x8a4, (seed / 10) % 3);
        w(c, OBJ + 0x320, if seed % 2 == 0 { 0 } else { 0x10 + seed });
        w(c, OWNER + 0xde0, (seed / 6) % 2);
        w(c, MODEL + 0x30, MATS);
        w(c, MODEL + 0x34, MATS + 0x100);
        w(c, S3 + 0x84, (seed / 7) % 2);
        // AdvanceRing's virtual call: {short delta, void (*fn)()} at +0x38.
        c.bus.write16(VT + 0x38, 0);
        w(c, VT + 0x3c, 0x0000_4000);

        c.g[A0][0] = SELF as u64;
        c.pc = 0x002d_d0b8;
    }

    #[test]
    fn board_wake_fx_update_matches() {
        strict(
            ssx3_ee::s09::f_002dd0b8__generated,
            c_board_wake_fx_update,
            SEEDS,
            &setup,
        );
    }
}
