//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! This module holds functions whose only readable source is the generated
//! body (`generated/ee`, a literal MIR→Rust translation of the EE machine
//! code). They are transcribed block for block: the same basic blocks, the
//! same block cycle charges, the same register writes, stack traffic and VU0
//! macro operations, in the same order relative to calls.
//!
//! - [`rider_anim_update_sequence`] (`RiderAnim_UpdateSequence`): advance one
//!   rider animation sequence through its type-specific update, then fire the
//!   queued sound events.
//! - [`rail_find_nearest_point`] (`Rail_FindNearestPoint`): the rail bounding
//!   box test plus the two VU0 passes that find the closest point on the rail.
//! - [`snd_dsp_resample_process`] (`SND_DspResample_Process`): the hand-written
//!   EE DSP resampler (pass-through and the MMI/VU0 resampling block).

use crate::call::Callee;
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_36a0, rider_anim_update_sequence, "RiderAnim_UpdateSequence"),
    (0x0033_5128, rail_find_nearest_point, "Rail_FindNearestPoint"),
    (0x003c_b540, snd_dsp_resample_process, "SND_DspResample_Process"),
];

// ---------------------------------------------------------------------------
// Direct-call targets (the generated bodies, which dispatch through their own
// override slot when one exists).

const ANIM_TABLE_GET_ENTRY: Callee = Callee { addr: 0x0031_15c8, f: ee::s10::AnimTable_GetEntry };
const FIRE_SOUND_EVENTS: Callee = Callee { addr: 0x0010_3aa0, f: ee::s00::RiderAnim_FireSoundEvents };

/// The sixteen per-type sequence updates, in sequence-type order.
const SEQ_TYPES: [Callee; 16] = [
    Callee { addr: 0x0010_48c0, f: ee::s00::RiderAnim_UpdateSeqType00 },
    Callee { addr: 0x0010_4940, f: ee::s00::RiderAnim_UpdateSeqType01 },
    Callee { addr: 0x0010_49c0, f: ee::s00::RiderAnim_UpdateSeqType02 },
    Callee { addr: 0x0010_4110, f: ee::s00::RiderAnim_UpdateSeqType03 },
    Callee { addr: 0x0010_4178, f: ee::s00::RiderAnim_UpdateSeqType04 },
    Callee { addr: 0x0010_4238, f: ee::s00::RiderAnim_UpdateSeqType05 },
    Callee { addr: 0x0010_42a8, f: ee::s00::RiderAnim_UpdateSeqType06 },
    Callee { addr: 0x0010_4358, f: ee::s00::RiderAnim_UpdateSeqType07 },
    Callee { addr: 0x0010_45d8, f: ee::s00::RiderAnim_UpdateSeqType08 },
    Callee { addr: 0x0010_4660, f: ee::s00::RiderAnim_UpdateSeqType09 },
    Callee { addr: 0x0010_42e0, f: ee::s00::RiderAnim_UpdateSeqType0A },
    Callee { addr: 0x0010_43f8, f: ee::s00::RiderAnim_UpdateSeqType0B },
    Callee { addr: 0x0010_45b8, f: ee::s00::RiderAnim_UpdateSeqType0C },
    Callee { addr: 0x0010_47f0, f: ee::s00::RiderAnim_UpdateSeqType0D },
    Callee { addr: 0x0010_4728, f: ee::s00::RiderAnim_UpdateSeqType0E },
    Callee { addr: 0x0010_46b0, f: ee::s00::RiderAnim_UpdateSeqType0F },
];

/// Return addresses of the sixteen type calls (they differ by two
/// instructions, and all land on the shared tail at `0x1038bc`).
const SEQ_RETS: [u32; 16] = [
    0x0010_3714, 0x0010_3730, 0x0010_374c, 0x0010_3768, 0x0010_3784, 0x0010_37a0, 0x0010_37bc,
    0x0010_37d8, 0x0010_37f4, 0x0010_3810, 0x0010_382c, 0x0010_3848, 0x0010_3864, 0x0010_3880,
    0x0010_389c, 0x0010_38b8,
];

/// `int RiderAnim_UpdateSequence(RiderAnim *anim, float frame, AnimSequence *seq)`
/// (0x1036a0): look the sequence's animation type up in the animation table
/// and run the matching per-type update; the returned flag (non-zero when the
/// sequence finished) is kept in `s1` and the tail fires the sequence's sound
/// events when it is clear and the sequence's owner has sounds pending.
///
/// Also accepts every resume entry of the generated `__entries` twin: the
/// trampoline can re-enter this function at each type call's return address
/// (and at the shared tail blocks) after an event unwinds a callee.
pub fn rider_anim_update_sequence(c: &mut Cpu) {
    let mut pc = c.pc as u32;
    loop {
        match pc {
            // Entry: frame, then the animation-table lookup.
            0x0010_36a0 => {
                c.cycles += 18;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                let sp = c.g[SP][0] as u32;
                c.sq(16, sp.wrapping_add(0x40));
                c.sq(17, sp.wrapping_add(0x30));
                c.g[S0][0] = c.g[A1][0];
                c.sq(18, sp.wrapping_add(0x20));
                c.g[S1][0] = 0;
                c.sq(19, sp.wrapping_add(0x10));
                c.g[S2][0] = c.g[A0][0];
                c.swc1(20, sp.wrapping_add(0x50));
                c.g[S3][0] = c.g[A2][0];
                c.sd(sp, c.g[RA][0]);
                c.mov_s(20, 12);
                c.g[A0][0] = Addr(c.g[S3][0] as u32).lw(c);
                if ANIM_TABLE_GET_ENTRY.call(c, 0x0010_36d8).is_err() {
                    return;
                }
                let ty = Addr(c.g[V0][0] as u32).off(4).lw(c);
                c.g[V1][0] = ty;
                c.g[V0][0] = (ty < 0x10) as u64;
                let known = ty < 0x10;
                c.g[V0][0] = 0x45_0000;
                pc = if known { 0x0010_36e8 } else { 0x0010_38bc };
            }
            // Animation-type dispatch through the jump table at 0x456950.
            0x0010_36e8 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6950));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let target = c.g[A0][0] as u32;
                pc = match target {
                    0x0010_36a0 => 0x0010_36a0,
                    0x0010_3700
                    | 0x0010_371c
                    | 0x0010_3738
                    | 0x0010_3754
                    | 0x0010_3770
                    | 0x0010_378c
                    | 0x0010_37a8
                    | 0x0010_37c4
                    | 0x0010_37e0
                    | 0x0010_37fc
                    | 0x0010_3818
                    | 0x0010_3834
                    | 0x0010_3850
                    | 0x0010_386c
                    | 0x0010_3888
                    | 0x0010_38a4 => target,
                    _ => {
                        c.jump_out(target, 0x0010_36a0, 0x0010_3914);
                        return;
                    }
                };
            }
            // The sixteen per-type updates; only the charge and return address
            // differ (the last one is a six-cycle block).
            0x0010_3700 | 0x0010_371c | 0x0010_3738 | 0x0010_3754 | 0x0010_3770
            | 0x0010_378c | 0x0010_37a8 | 0x0010_37c4 | 0x0010_37e0 | 0x0010_37fc
            | 0x0010_3818 | 0x0010_3834 | 0x0010_3850 | 0x0010_386c | 0x0010_3888 => {
                let kind = ((pc - 0x0010_3700) / 0x1c) as usize;
                c.cycles += 7;
                c.g[A1][0] = c.g[S0][0];
                c.mov_s(12, 20);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                if SEQ_TYPES[kind].call(c, SEQ_RETS[kind]).is_err() {
                    return;
                }
                c.g[S1][0] = c.g[V0][0];
                pc = 0x0010_38bc;
            }
            0x0010_38a4 => {
                c.cycles += 6;
                c.g[A1][0] = c.g[S0][0];
                c.mov_s(12, 20);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S3][0];
                if SEQ_TYPES[15].call(c, SEQ_RETS[15]).is_err() {
                    return;
                }
                c.g[S1][0] = c.g[V0][0];
                pc = 0x0010_38bc;
            }
            // Shared tail: fire sound events if the update asked for them.
            0x0010_38bc => {
                c.cycles += 2;
                let finished = c.g[S1][0] != 0;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                pc = if finished { 0x0010_38f8 } else { 0x0010_38c4 };
            }
            0x0010_38c4 => {
                c.cycles += 3;
                let owner = Addr(c.g[S2][0] as u32).off(0x60).lw(c);
                c.g[V1][0] = owner;
                if owner == 0 {
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    pc = 0x0010_38fc;
                } else {
                    pc = 0x0010_38d0;
                }
            }
            0x0010_38d0 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x874).lw(c);
                let no_pending = c.g[V0][0] == 0;
                c.g[A0][0] = 0;
                pc = if no_pending { 0x0010_38e4 } else { 0x0010_38dc };
            }
            0x0010_38dc => {
                c.cycles += 2;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x87c).lw(c);
                c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
                pc = 0x0010_38e4;
            }
            0x0010_38e4 => {
                c.cycles += 2;
                let silent = c.g[A0][0] == 0;
                c.g[A0][0] = c.g[S2][0];
                pc = if silent { 0x0010_38f4 } else { 0x0010_38ec };
            }
            0x0010_38ec => {
                c.cycles += 2;
                c.g[A1][0] = c.g[S3][0];
                if FIRE_SOUND_EVENTS.call(c, 0x0010_38f4).is_err() {
                    return;
                }
                pc = 0x0010_38f4;
            }
            0x0010_38f4 => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                pc = 0x0010_38f8;
            }
            0x0010_38f8 => {
                c.cycles += 1;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                pc = 0x0010_38fc;
            }
            // Epilogue.
            0x0010_38fc => {
                c.cycles += 6;
                let sp = c.g[SP][0] as u32;
                c.lq(18, sp.wrapping_add(0x20));
                c.lq(19, sp.wrapping_add(0x10));
                c.g[RA][0] = c.ld(sp);
                c.lwc1(20, sp.wrapping_add(0x50));
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx(sp.wrapping_add(0x60));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x0010_36a0, 0x0010_3914);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Rail_FindNearestPoint: the rail's bounding box test, then two VU0 passes
// (five control points, then 24 samples along the segment) tracking the
// closest point on the rail and its normal/tangent frame.

/// `int Rail_FindNearestPoint(Rail *rail, Vec4 *box, Vec4 *out, ...)`?
/// (0x335128). `a0` is the rail object (bounds at +0x6c..+0x80 and a 4x4
/// matrix at +0x10), `a1` the query box (min at +0, max at +0x10), `a2` a
/// vector; the result is written to `a3` (0x335828..0x335840) and the frame to
/// the caller's structures.
///
/// The port keeps the original's stack scratch: the VU0 code exchanges
/// vectors through `sp+0x00..0xc0`, and those stores are part of what a
/// caller (or an interrupt handler) can observe.
pub fn rail_find_nearest_point(c: &mut Cpu) {
    let mut pc = c.pc as u32;
    loop {
        match pc {
            // Bounding-box reject: six compares against the rail's bounds.
            0x0033_5128 => {
                c.cycles += 7;
                c.g[T2][0] = c.g[A0][0];
                let a1 = c.g[A1][0] as u32;
                let t2 = c.g[T2][0] as u32;
                c.lwc1(1, a1.wrapping_add(0x10));
                c.lwc1(0, t2.wrapping_add(0x6c));
                c.c_lt_s(1, 0);
                let out = c.fcc();
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                pc = if out { 0x0033_5954 } else { 0x0033_5144 };
            }
            0x0033_5144 => {
                c.cycles += 5;
                c.lwc1(1, c.g[A1][0] as u32);
                c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x78));
                c.c_lt_s(0, 1);
                let out = c.fcc();
                pc = if out { 0x0033_5954 } else { 0x0033_5158 };
            }
            0x0033_5158 => {
                c.cycles += 5;
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x14));
                c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x70));
                c.c_lt_s(1, 0);
                let out = c.fcc();
                pc = if out { 0x0033_5954 } else { 0x0033_516c };
            }
            0x0033_516c => {
                c.cycles += 5;
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x4));
                c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x7c));
                c.c_lt_s(0, 1);
                let out = c.fcc();
                pc = if out { 0x0033_5954 } else { 0x0033_5180 };
            }
            0x0033_5180 => {
                c.cycles += 5;
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x18));
                c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x74));
                c.c_lt_s(1, 0);
                let out = c.fcc();
                pc = if out { 0x0033_5954 } else { 0x0033_5194 };
            }
            0x0033_5194 => {
                c.cycles += 6;
                c.lwc1(1, (c.g[A1][0] as u32).wrapping_add(0x8));
                c.lwc1(0, (c.g[T2][0] as u32).wrapping_add(0x80));
                c.c_lt_s(0, 1);
                let out = c.fcc();
                c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x10));
                pc = if out { 0x0033_5954 } else { 0x0033_51ac };
            }
            // Pass 1 setup: transform the rail origin, seed the running
            // minimum (f2) with 1.0 and the best point with the origin.
            0x0033_51ac => {
                c.cycles += 47;
                let sp = c.g[SP][0] as u32;
                let v0 = c.g[V0][0] as u32;
                c.mtc1(0, 10);
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 12);
                c.g[T3][0] = c.g[V0][0];
                c.swc1(10, sp.wrapping_add(0x48));
                c.mov_s(5, 10);
                c.swc1(12, sp.wrapping_add(0x4c));
                c.mov_s(4, 12);
                c.swc1(10, sp.wrapping_add(0x44));
                c.g[T5][0] = c.g[T3][0];
                c.swc1(10, sp.wrapping_add(0x40));
                c.g[T4][0] = 0;
                c.lqc2(8, sp.wrapping_add(0x40));
                c.lqc2(4, v0);
                c.lqc2(5, v0.wrapping_add(0x10));
                c.lqc2(6, v0.wrapping_add(0x20));
                c.lqc2(7, v0.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x50));
                c.g[AT][0] = 0x3e80_0000;
                c.mtc1(1, 6);
                c.lq(2, sp.wrapping_add(0x50));
                c.g[A1][0] = 1;
                c.g[T6][0] = sx(sp.wrapping_add(0x30));
                c.g[A0][0] = sx(sp.wrapping_add(0x60));
                c.sq(2, sp.wrapping_add(0x30));
                c.sq(2, sp);
                c.lqc2(3, c.g[A2][0] as u32);
                c.lqc2(4, sp);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x40));
                c.lq(2, sp.wrapping_add(0x40));
                c.sq(2, sp.wrapping_add(0x30));
                c.lqc2(3, sp.wrapping_add(0x30));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 3);
                pc = 0x0033_5268;
            }
            // Pass 1 body: distance from the query point to the rail origin,
            // clamped against the running minimum (f2), five times.
            0x0033_5268 => {
                c.cycles += 59;
                let sp = c.g[SP][0] as u32;
                let t5 = c.g[T5][0] as u32;
                c.mtc1(5, 0);
                c.cvt_s_w(0, 0);
                c.swc1(4, sp.wrapping_add(0x4c));
                c.mov_s(2, 5);
                c.mul_s(0, 0, 6);
                c.mul_s(1, 0, 0);
                c.swc1(0, sp.wrapping_add(0x48));
                c.mul_s(0, 0, 1);
                c.swc1(1, sp.wrapping_add(0x44));
                c.swc1(0, sp.wrapping_add(0x40));
                c.lqc2(8, sp.wrapping_add(0x40));
                c.lqc2(4, t5);
                c.lqc2(5, t5.wrapping_add(0x10));
                c.lqc2(6, t5.wrapping_add(0x20));
                c.lqc2(7, t5.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x50));
                c.lq(2, sp.wrapping_add(0x50));
                c.sq(2, sp.wrapping_add(0x30));
                c.sq(2, sp.wrapping_add(0x10));
                c.lqc2(3, sp.wrapping_add(0x10));
                c.lqc2(4, sp);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x40));
                c.lq(2, sp.wrapping_add(0x40));
                c.sq(2, sp.wrapping_add(0x30));
                c.sq(2, sp.wrapping_add(0x20));
                c.lqc2(3, c.g[A2][0] as u32);
                c.lqc2(4, sp);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x40));
                c.lq(2, sp.wrapping_add(0x40));
                c.sq(2, sp.wrapping_add(0x30));
                c.lqc2(3, sp.wrapping_add(0x30));
                c.lqc2(5, sp.wrapping_add(0x20));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(3, 4);
                c.mtc1(3, 1);
                c.lqc2(3, sp.wrapping_add(0x20));
                c.lqc2(5, sp.wrapping_add(0x20));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.div_s(1, 1, 0);
                c.c_le_s(5, 1);
                if c.fcc() {
                    c.min_s(2, 1, 4);
                }
                pc = 0x0033_5354;
            }
            // Pass 1 step: move the sample point, recompute the closest point.
            0x0033_5354 => {
                c.cycles += 34;
                let sp = c.g[SP][0] as u32;
                c.mfc1(3, 2);
                c.lqc2(4, sp.wrapping_add(0x20));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, sp.wrapping_add(0x50));
                c.lq(2, sp.wrapping_add(0x50));
                c.sq(2, sp.wrapping_add(0x40));
                c.lqc2(3, sp);
                c.lqc2(4, sp.wrapping_add(0x40));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x50));
                c.lq(2, sp.wrapping_add(0x50));
                c.sq(2, sp.wrapping_add(0x30));
                c.sq(2, sp.wrapping_add(0x20));
                c.lqc2(3, c.g[A2][0] as u32);
                c.lqc2(4, sp.wrapping_add(0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x40));
                c.lq(2, sp.wrapping_add(0x40));
                c.sq(2, sp.wrapping_add(0x30));
                c.lqc2(3, sp.wrapping_add(0x30));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 0);
                c.c_lt_s(0, 3);
                let closer = c.fcc();
                c.lq(2, sp.wrapping_add(0x10));
                pc = if closer { 0x0033_53dc } else { 0x0033_53e4 };
            }
            0x0033_53dc => {
                c.cycles += 2;
                c.mov_s(3, 0);
                c.g[T4][0] = c.g[A1][0];
                pc = 0x0033_53e4;
            }
            // Pass 1 back edge (five samples).
            0x0033_53e4 => {
                c.cycles += 4;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = ((c.g[A1][0] as i64) < 5i64) as u64;
                let more = c.g[V1][0] != 0;
                c.sq(2, c.g[SP][0] as u32);
                if more {
                    if crate::events::back_edge_unwinds(c, 0x0033_5268) {
                        return;
                    }
                    pc = 0x0033_5268;
                } else {
                    pc = 0x0033_53f4;
                }
            }
            // Pass 2 setup: pick the best sample's two value pairs out of the
            // stack scratch, rebuild the query frame.
            0x0033_53f4 => {
                c.cycles += 167;
                let sp = c.g[SP][0] as u32;
                let t3 = c.g[T3][0] as u32;
                let t4 = c.g[T4][0] as u32;
                let t6 = c.g[T6][0] as u32;
                c.g[V0][0] = 0x49_0000;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2d68));
                c.g[T7][0] = sx(0x49_0000u32.wrapping_sub(0x1aa0));
                let t7 = 0x48_e560u32;
                c.ldl(3, t7.wrapping_add(0x7));
                c.ldr(3, t7);
                c.ldl(5, t7.wrapping_add(0xf));
                c.ldr(5, t7.wrapping_add(0x8));
                c.g[T5][0] = c.lw(t7.wrapping_add(0x10));
                c.sdl(3, sp.wrapping_add(0x37));
                c.sdr(3, sp.wrapping_add(0x30));
                c.sdl(5, sp.wrapping_add(0x3f));
                c.sdr(5, sp.wrapping_add(0x38));
                c.sw(sp.wrapping_add(0x40), c.g[T5][0]);
                c.g[V1][0] = sx(t4 << 2);
                c.g[V0][0] = 0x49_0000;
                c.g[T7][0] = sx(0x49_0000u32.wrapping_sub(0x1a88));
                let t7b = 0x48_e578u32;
                c.ldl(5, t7b.wrapping_add(0x7));
                c.ldr(5, t7b);
                c.ldl(12, t7b.wrapping_add(0xf));
                c.ldr(12, t7b.wrapping_add(0x8));
                c.g[T5][0] = c.lw(t7b.wrapping_add(0x10));
                c.sdl(5, sp.wrapping_add(0x67));
                c.sdr(5, sp.wrapping_add(0x60));
                c.sdl(12, sp.wrapping_add(0x6f));
                c.sdr(12, sp.wrapping_add(0x68));
                c.sw(sp.wrapping_add(0x70), c.g[T5][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V1][0] = sx(t6.wrapping_add(c.g[V1][0] as u32));
                c.mov_s(15, 0);
                c.lwc1(10, c.g[V1][0] as u32);
                c.g[A1][0] = 0;
                c.lwc1(12, c.g[A0][0] as u32);
                c.mul_s(2, 10, 10);
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 4);
                c.sub_s(1, 12, 10);
                c.swc1(10, sp.wrapping_add(0x88));
                c.swc1(4, sp.wrapping_add(0x8c));
                c.mul_s(5, 12, 12);
                c.mul_s(3, 10, 2);
                c.swc1(2, sp.wrapping_add(0x84));
                c.mul_s(0, 1, 0);
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x2d64));
                c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x2d60));
                c.mov_s(14, 4);
                c.swc1(3, sp.wrapping_add(0x80));
                c.mul_s(1, 1, 2);
                c.add_s(7, 10, 0);
                c.lqc2(8, sp.wrapping_add(0x80));
                c.lqc2(4, t3);
                c.lqc2(5, t3.wrapping_add(0x10));
                c.lqc2(6, t3.wrapping_add(0x20));
                c.lqc2(7, t3.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x90));
                c.mov_s(13, 2);
                c.lq(2, sp.wrapping_add(0x90));
                c.add_s(6, 10, 1);
                c.g[A0][0] = c.g[T3][0];
                c.mul_s(2, 7, 7);
                c.sq(2, sp.wrapping_add(0x50));
                c.sq(2, sp);
                c.mul_s(3, 12, 5);
                c.lqc2(3, sp);
                c.lqc2(4, c.g[A2][0] as u32);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x80));
                c.mul_s(1, 6, 6);
                c.mul_s(0, 7, 2);
                c.lq(2, sp.wrapping_add(0x80));
                c.swc1(4, sp.wrapping_add(0x8c));
                c.mov_s(11, 6);
                c.sq(2, sp.wrapping_add(0x50));
                c.swc1(0, sp.wrapping_add(0x80));
                c.swc1(7, sp.wrapping_add(0x88));
                c.mul_s(0, 6, 1);
                c.swc1(2, sp.wrapping_add(0x84));
                c.lqc2(8, sp.wrapping_add(0x80));
                c.lqc2(4, t3);
                c.lqc2(5, t3.wrapping_add(0x10));
                c.lqc2(6, t3.wrapping_add(0x20));
                c.lqc2(7, t3.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x90));
                c.lq(2, sp.wrapping_add(0x90));
                c.sq(2, sp);
                c.sq(2, sp.wrapping_add(0x50));
                c.lqc2(3, sp);
                c.lqc2(4, c.g[A2][0] as u32);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x80));
                c.lq(2, sp.wrapping_add(0x80));
                c.sq(2, sp.wrapping_add(0x50));
                c.lqc2(3, sp.wrapping_add(0x50));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 9);
                c.swc1(1, sp.wrapping_add(0x84));
                c.swc1(0, sp.wrapping_add(0x80));
                c.swc1(4, sp.wrapping_add(0x8c));
                c.swc1(6, sp.wrapping_add(0x88));
                c.lqc2(8, sp.wrapping_add(0x80));
                c.lqc2(4, t3);
                c.lqc2(5, t3.wrapping_add(0x10));
                c.lqc2(6, t3.wrapping_add(0x20));
                c.lqc2(7, t3.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x90));
                c.lq(2, sp.wrapping_add(0x90));
                c.sq(2, sp.wrapping_add(0x50));
                c.sq(2, sp);
                c.lqc2(3, sp);
                c.lqc2(4, c.g[A2][0] as u32);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x80));
                c.lq(2, sp.wrapping_add(0x80));
                c.sq(2, sp.wrapping_add(0x50));
                c.lqc2(3, sp.wrapping_add(0x50));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 8);
                c.swc1(3, sp.wrapping_add(0x80));
                c.mov_s(1, 8);
                c.swc1(4, sp.wrapping_add(0x8c));
                c.swc1(12, sp.wrapping_add(0x88));
                c.swc1(5, sp.wrapping_add(0x84));
                c.lqc2(8, sp.wrapping_add(0x80));
                c.lqc2(4, t3);
                c.lqc2(5, t3.wrapping_add(0x10));
                c.lqc2(6, t3.wrapping_add(0x20));
                c.lqc2(7, t3.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x90));
                c.lq(2, sp.wrapping_add(0x90));
                c.sq(2, sp.wrapping_add(0x50));
                c.sq(2, sp);
                c.lqc2(3, sp);
                c.lqc2(4, c.g[A2][0] as u32);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x80));
                c.lq(2, sp.wrapping_add(0x80));
                c.sq(2, sp.wrapping_add(0x50));
                pc = 0x0033_5804;
            }
            // Pass 2 body: refine the closest point over 24 samples.
            0x0033_5690 => {
                c.cycles += 4;
                c.c_lt_s(8, 9);
                if !c.fcc() {
                    c.mov_s(12, 6);
                    pc = 0x0033_5754;
                } else {
                    pc = 0x0033_56a0;
                }
            }
            0x0033_56a0 => {
                c.cycles += 45;
                let sp = c.g[SP][0] as u32;
                c.mov_s(10, 7);
                c.mul_s(2, 12, 15);
                c.mov_s(7, 6);
                c.swc1(14, sp.wrapping_add(0x8c));
                c.mul_s(0, 7, 13);
                c.mov_s(9, 8);
                c.add_s(6, 0, 2);
                c.mul_s(1, 6, 6);
                c.swc1(6, sp.wrapping_add(0x88));
                c.mov_s(11, 6);
                c.mul_s(0, 6, 1);
                c.swc1(1, sp.wrapping_add(0x84));
                c.swc1(0, sp.wrapping_add(0x80));
                c.lqc2(8, sp.wrapping_add(0x80));
                c.lqc2(4, c.g[A0][0] as u32);
                c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x90));
                c.lq(2, sp.wrapping_add(0x90));
                c.sq(2, sp.wrapping_add(0x50));
                c.sq(2, sp);
                c.lqc2(3, sp);
                c.lqc2(4, c.g[A2][0] as u32);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x80));
                c.lq(2, sp.wrapping_add(0x80));
                c.sq(2, sp.wrapping_add(0x50));
                c.lqc2(3, sp.wrapping_add(0x50));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 0);
                c.mov_s(8, 0);
                c.mov_s(1, 8);
                pc = 0x0033_5800;
            }
            0x0033_5754 => {
                c.cycles += 43;
                let sp = c.g[SP][0] as u32;
                c.mul_s(2, 10, 15);
                c.mov_s(6, 7);
                c.swc1(14, sp.wrapping_add(0x8c));
                c.mul_s(0, 6, 13);
                c.mov_s(8, 9);
                c.add_s(7, 0, 2);
                c.mul_s(1, 7, 7);
                c.swc1(7, sp.wrapping_add(0x88));
                c.mov_s(11, 7);
                c.mul_s(0, 7, 1);
                c.swc1(1, sp.wrapping_add(0x84));
                c.swc1(0, sp.wrapping_add(0x80));
                c.lqc2(8, sp.wrapping_add(0x80));
                c.lqc2(4, c.g[A0][0] as u32);
                c.lqc2(5, (c.g[A0][0] as u32).wrapping_add(0x10));
                c.lqc2(6, (c.g[A0][0] as u32).wrapping_add(0x20));
                c.lqc2(7, (c.g[A0][0] as u32).wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0x90));
                c.lq(2, sp.wrapping_add(0x90));
                c.sq(2, sp.wrapping_add(0x50));
                c.sq(2, sp);
                c.lqc2(3, sp);
                c.lqc2(4, c.g[A2][0] as u32);
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp.wrapping_add(0x80));
                c.lq(2, sp.wrapping_add(0x80));
                c.sq(2, sp.wrapping_add(0x50));
                c.lqc2(3, sp.wrapping_add(0x50));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(3, 22);
                c.mtc1(3, 0);
                c.mov_s(9, 0);
                c.mov_s(1, 9);
                pc = 0x0033_5800;
            }
            0x0033_5800 => {
                c.cycles += 1;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                pc = 0x0033_5804;
            }
            0x0033_5804 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < 24i64) as u64;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
                    pc = 0x0033_5828;
                } else {
                    pc = 0x0033_5810;
                }
            }
            0x0033_5810 => {
                c.cycles += 5;
                c.sub_s(0, 12, 10);
                c.abs_s(0, 0);
                c.c_le_s(0, 16);
                let keep = !c.fcc();
                if keep {
                    if crate::events::back_edge_unwinds(c, 0x0033_5690) {
                        return;
                    }
                    pc = 0x0033_5690;
                } else {
                    pc = 0x0033_5824;
                }
            }
            0x0033_5824 => {
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[A3][0] as u32).lw(c);
                pc = 0x0033_5828;
            }
            0x0033_5828 => {
                c.cycles += 2;
                let no_hit = c.g[V0][0] == 0;
                c.g[V0][0] = 1;
                pc = if no_hit { 0x0033_5840 } else { 0x0033_5830 };
            }
            0x0033_5830 => {
                c.cycles += 4;
                c.lwc1(0, c.g[T0][0] as u32);
                c.c_lt_s(1, 0);
                let keep = !c.fcc();
                pc = if keep { 0x0033_5954 } else { 0x0033_5840 };
            }
            // Success: write the result flag, the frame and the rail output.
            0x0033_5840 => {
                c.cycles += 69;
                let sp = c.g[SP][0] as u32;
                let t3 = c.g[T3][0] as u32;
                c.g[A1][0] = u64::MAX;
                c.sw(c.g[A3][0] as u32, c.g[V0][0]);
                c.g[V1][0] = 0x2;
                c.swc1(1, c.g[T0][0] as u32);
                c.g[A0][0] = 0x50_0000;
                c.sw((c.g[T1][0] as u32).wrapping_add(0x44), c.g[V1][0]);
                c.g[A0][0] = sx(0x50_0000u32.wrapping_sub(0xee0));
                c.sw((c.g[T1][0] as u32).wrapping_add(0x54), 0);
                c.add_s(2, 11, 11);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x50), 0);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x58), 0);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x5c), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x60), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x64), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x4c), c.g[A1][0]);
                c.g[AT][0] = 0x4040_0000;
                c.mtc1(1, 0);
                c.lq(2, c.g[A0][0] as u32);
                c.lq(3, sp);
                c.mul_s(0, 11, 0);
                c.sq(2, (c.g[T1][0] as u32).wrapping_add(0x20));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 1);
                c.lq(2, c.g[A0][0] as u32);
                c.mul_s(0, 11, 0);
                c.sq(3, c.g[T1][0] as u32);
                c.sq(2, (c.g[T1][0] as u32).wrapping_add(0x30));
                c.swc1(1, sp.wrapping_add(0xa8));
                c.swc1(2, sp.wrapping_add(0xa4));
                c.swc1(0, sp.wrapping_add(0xa0));
                c.sw((c.g[T1][0] as u32).wrapping_add(0x68), 0);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x6c), 0);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x70), 0);
                c.sw(sp.wrapping_add(0xac), 0);
                c.lqc2(8, sp.wrapping_add(0xa0));
                c.lqc2(4, t3);
                c.lqc2(5, t3.wrapping_add(0x10));
                c.lqc2(6, t3.wrapping_add(0x20));
                c.lqc2(7, t3.wrapping_add(0x30));
                c.vu0().mula(0b1111, 4, Src::Bc(8, X));
                c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
                c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
                c.sqc2(12, sp.wrapping_add(0xb0));
                c.lq(2, sp.wrapping_add(0xb0));
                c.sq(2, sp.wrapping_add(0x80));
                c.lqc2(3, sp.wrapping_add(0x80));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, sp.wrapping_add(0x90));
                c.lq(2, sp.wrapping_add(0x90));
                c.sq(2, sp.wrapping_add(0x50));
                c.sq(2, (c.g[T1][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x68)).lw(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x28)).lw(c);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x50), 0);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x5c), c.g[A1][0]);
                c.g[V0][0] = Addr((c.g[T2][0] as u32).wrapping_add(0x68)).lw(c);
                c.swc1(11, (c.g[T1][0] as u32).wrapping_add(0x68));
                c.sw((c.g[T1][0] as u32).wrapping_add(0x58), c.g[V0][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x64), c.g[A1][0]);
                c.sw((c.g[T1][0] as u32).wrapping_add(0x54), 0);
                pc = 0x0033_5954;
            }
            // Return.
            0x0033_5954 => {
                c.cycles += 2;
                let ra = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                c.pc = ra;
                return;
            }
            _ => {
                c.jump_out(pc, 0x0033_5128, 0x0033_595c);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// SND_DspResample_Process: the hand-written EE DSP resampler.
//
// The caller sets up more than the standard ABI arguments: `s0`, `s1`, `s2`
// and `t7` are read as input pointers before the function ever writes them
// (this is hand-written assembly, and the generated body does not initialise
// them). The port therefore reads the same registers at the same points. The
// object (`a0`) holds the format at +0x1c, a "done" flag at +0x24, a sample
// count at +0x26 and a resample callback pointer at +0x8.

/// `int SND_DspResample_Process(DspObj *o, void *in, s32 *out, s32 end)`
/// (0x3cb540): copy `o->count` samples into the caller's output, or run the
/// MMI/VU0 block that resamples `o->count` samples into `[out, end)` at the
/// rate in `o->+0x20`.
pub fn snd_dsp_resample_process(c: &mut Cpu) {
    crate::call::guest(c, |c| {
        // Prologue (object at 0x30, input at 0x00, output cursor at 0x08;
        // 0x20 tracks the cursor for the fast path). `t2` keeps a2 while a2
        // receives a3.
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
        let sp = c.g[SP][0] as u32;
        c.g[T2][0] = c.g[A2][0];
        c.sw(sp.wrapping_add(0x30), c.g[A0][0]);
        c.g[A2][0] = c.g[A3][0];
        c.sw(sp, c.g[A1][0]);
        c.g[V0][0] = 0x1_0000;
        c.sq(16, sp.wrapping_add(0xd0));
        c.sq(17, sp.wrapping_add(0xc0));
        c.g[T0][0] = c.g[A1][0];
        c.sq(18, sp.wrapping_add(0xb0));
        c.sq(19, sp.wrapping_add(0xa0));
        c.sq(20, sp.wrapping_add(0x90));
        c.sq(21, sp.wrapping_add(0x80));
        c.sq(22, sp.wrapping_add(0x70));
        c.sq(23, sp.wrapping_add(0x60));
        c.sq(30, sp.wrapping_add(0x50));
        c.sd(sp.wrapping_add(0x40), c.g[RA][0]);
        c.sw(sp.wrapping_add(0x4), c.g[A0][0]);
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
        c.sw(sp.wrapping_add(0x20), c.g[T2][0]);
        let other_format = c.g[V1][0] != c.g[V0][0];
        c.sw(sp.wrapping_add(0x8), c.g[A2][0]);

        if !other_format {
            // Fast path (format 0x10000): pass the input through.
            c.cycles += 3;
            let active = Addr(c.g[A0][0] as u32).off(0x24).lh(c);
            c.g[V0][0] = active;
            let nonzero = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(sp.wrapping_add(0x30));
            if nonzero {
                // Copy lh(o->+0x26) samples from the object to the output.
                c.cycles += 7;
                c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x26).lh(c);
                c.g[V0][0] = 0x4;
                c.g[A3][0] = Addr(c.g[A0][0] as u32).off(0x26).lhu(c);
                c.g[V0][0] = sx(4u32.wrapping_sub(c.g[V1][0] as u32));
                let empty = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = 0;
                if !empty {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw(sp.wrapping_add(0x30));
                    c.g[T1][0] = 0x4;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28));
                    loop {
                        c.cycles += 17;
                        c.g[V0][0] = sx((c.g[A3][0] as u32) << 16);
                        c.g[T9][0] = c.lw(sp.wrapping_add(0x8));
                        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.lwc1(0, c.g[V0][0] as u32);
                        c.swc1(0, c.g[T9][0] as u32);
                        c.g[T9][0] = sx((c.g[T9][0] as u32).wrapping_add(0x4));
                        c.sw(sp.wrapping_add(0x8), c.g[T9][0]);
                        c.g[FP][0] = c.lw(sp.wrapping_add(0x30));
                        c.g[V0][0] = Addr(c.g[FP][0] as u32).off(0x26).lh(c);
                        c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A3][0] = Addr(c.g[FP][0] as u32).off(0x26).lhu(c);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_b5e0);
                        }
                    }
                }
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x26).lh(c);
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            }

            // Done check: no resample callback means the object is finished.
            c.cycles += 4;
            c.g[V0][0] = c.lw(sp.wrapping_add(0x30));
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
            let no_callback = c.g[A0][0] == 0;
            c.g[V1][0] = c.lw(sp.wrapping_add(0x30));
            if !no_callback {
                c.cycles += 6;
                c.g[A2][0] = c.g[T2][0];
                let target = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V0][0] = target;
                c.g[T0][0] = Addr(c.g[V1][0] as u32).off(0x1a).lbu(c);
                c.g[A3][0] = c.lw(sp.wrapping_add(0x8));
                crate::call::call_ptr(c, target as u32, 0x003c_b648)?;
                c.g[T0][0] = c.g[V0][0];
            }

            c.cycles += 2;
            let done = (c.g[T0][0] as i64) > 0;
            c.g[A2][0] = c.lw(sp.wrapping_add(0x30));
            if !done {
                c.cycles += 4;
                c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
                c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x24).lh(c);
                let active = c.g[V0][0] != 0;
                c.g[A1][0] = c.lw(sp.wrapping_add(0x30));
                if !active {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    c.sh(Addr(c.g[A0][0] as u32).off(0x24).0, c.g[V0][0]);
                    c.g[A1][0] = c.lw(sp.wrapping_add(0x30));
                }
                c.cycles += 6;
                c.g[V0][0] = c.g[T0][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x34), 0);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x28), 0);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x2c), 0);
                c.sw((c.g[A1][0] as u32).wrapping_add(0x30), 0);
                return epilogue(c);
            }

            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x24).lh(c);
            let finished = c.g[V0][0] != 0;
            c.g[V1][0] = c.lw(sp);
            if finished {
                c.cycles += 4;
                c.g[A0][0] = c.lw(sp.wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            } else {
                c.cycles += 9;
                c.g[V1][0] = 0x1;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x26).lh(c);
                c.sh(Addr(c.g[A2][0] as u32).off(0x24).0, c.g[V1][0]);
                c.g[T9][0] = c.lw(sp);
                c.g[FP][0] = c.lw(sp.wrapping_add(0x8));
                c.g[V0][0] = sx((c.g[T9][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[A3][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[V0][0] as u32));
            }
            c.cycles += 11;
            c.lwc1(1, c.g[A3][0] as u32);
            c.g[A1][0] = c.lw(sp.wrapping_add(0x30));
            c.g[V0][0] = c.lw(sp);
            c.swc1(1, (c.g[A1][0] as u32).wrapping_add(0x28));
            c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x2c));
            c.lwc1(1, (c.g[A3][0] as u32).wrapping_add(0x8));
            c.swc1(1, (c.g[A1][0] as u32).wrapping_add(0x30));
            c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0xc));
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x34));
            return epilogue(c);
        }

        // Second half: only reached when the format is not 0x10000.
        c.cycles += 8;
        c.g[T9][0] = c.lw(sp);
        c.g[FP][0] = c.lw(sp.wrapping_add(0x30));
        c.mult(0, 2, 3, 25);
        c.g[V1][0] = Addr(c.g[FP][0] as u32).off(0x20).lw(c);
        c.g[A0][0] = Addr(c.g[FP][0] as u32).off(0x24).lh(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        let active = c.g[A0][0] != 0;
        c.g[A1][0] = sx((c.g[V0][0] as u32) >> 16);
        if active {
            // Already running: copy the object's current 4-float frame out.
            c.cycles += 12;
            c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
            c.g[T9][0] = c.lw(sp.wrapping_add(0x20));
            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x28));
            c.swc1(0, c.g[T9][0] as u32);
            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x2c));
            c.swc1(1, (c.g[T9][0] as u32).wrapping_add(0x4));
            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x30));
            c.swc1(0, (c.g[T9][0] as u32).wrapping_add(0x8));
            c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x34));
            c.swc1(1, (c.g[T9][0] as u32).wrapping_add(0xc));
            c.g[T9][0] = sx((c.g[T9][0] as u32).wrapping_add(0x10));
            c.sw(sp.wrapping_add(0x20), c.g[T9][0]);
        } else {
            // Starting: `a1` (already `len + 4 - count`) samples to emit.
            c.cycles += 5;
            c.g[V1][0] = Addr(c.g[FP][0] as u32).off(0x26).lh(c);
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0;
            let empty = (c.g[V1][0] as i64) <= 0;
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            if !empty {
                c.cycles += 2;
                c.g[V0][0] = c.lw(sp.wrapping_add(0x20));
                loop {
                    c.cycles += 9;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    c.sw(c.g[V0][0] as u32, 0);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[V1][0] = c.lw(sp.wrapping_add(0x30));
                    c.sw(sp.wrapping_add(0x20), c.g[V0][0]);
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x26).lh(c);
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = c.lw(sp.wrapping_add(0x20));
                    if !more {
                        c.cycles += 2;
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_b730);
                    }
                }
            }
        }

        c.cycles += 2;
        let empty = (c.g[A1][0] as i64) <= 0;
        c.g[FP][0] = c.lw(sp.wrapping_add(0x30));
        if empty {
            c.cycles += 1;
            c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(c.g[FP][0] as u32).off(0x8).lw(c);
            if c.g[A0][0] == 0 {
                c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
            } else {
                c.cycles += 7;
                c.g[T0][0] = Addr(c.g[FP][0] as u32).off(0x1a).lbu(c);
                let target = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V0][0] = target;
                c.g[A3][0] = c.lw(sp.wrapping_add(0x20));
                crate::call::call_ptr(c, target as u32, 0x003c_b7b0)?;
                c.g[T0][0] = c.g[V0][0];
                let done = (c.g[T0][0] as i64) > 0;
                c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
                if !done {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[FP][0] as u32).off(0x24).lh(c);
                    let active = c.g[V0][0] != 0;
                    c.g[V0][0] = c.lw(sp.wrapping_add(0x30));
                    if !active {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        c.sh(Addr(c.g[FP][0] as u32).off(0x24).0, c.g[V0][0]);
                        c.g[V0][0] = c.lw(sp.wrapping_add(0x30));
                    }
                    c.cycles += 7;
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x34), 0);
                    c.g[V0][0] = c.g[T0][0];
                    c.g[V1][0] = c.lw(sp.wrapping_add(0x30));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x28), 0);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x2c), 0);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x30), 0);
                    return epilogue(c);
                }
            }
        }

        // Emit the remaining samples.
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x24).lh(c);
        let active = c.g[V0][0] == 0;
        c.g[V0][0] = 0x4;
        if active {
            c.cycles += 3;
            c.g[A2][0] = c.lw(sp.wrapping_add(0x30));
            c.g[V0][0] = 0x1;
            c.sh(Addr(c.g[A2][0] as u32).off(0x24).0, c.g[V0][0]);
        } else {
            c.cycles += 7;
            c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x26).lh(c);
            c.g[A1][0] = c.lw(sp.wrapping_add(0x20));
            c.g[V0][0] = sx(4u32.wrapping_sub(c.g[V1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.sw(sp.wrapping_add(0x20), c.g[A1][0]);
        }

        // MMI/VU0 resample: 16-bit packed samples in, 4-float frames out.
        c.cycles += 29;
        c.g[T9][0] = c.lw(sp.wrapping_add(0x4));
        c.g[FP][0] = c.lw(sp);
        c.g[V0][0] = c.lw(sp.wrapping_add(0x20));
        c.g[V1][0] = c.lw(sp.wrapping_add(0x8));
        c.g[A0][0] = c.g[T9][0];
        c.g[A1][0] = c.g[FP][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = c.g[V1][0];
        c.g[T2][0] = Addr(c.g[A0][0] as u32).off(0x1c).lw(c);
        c.g[T0][0] = sx((c.g[T2][0] as u32) << 2);
        c.pextlw(8, 8, 8);
        c.pextlw(8, 8, 8);
        c.g[T3][0] = sx((c.g[T2][0] as u32) << 1);
        c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_add(c.g[T2][0] as u32));
        c.pextlw(9, 11, 0);
        c.pextlw(14, 12, 10);
        c.pextlw(9, 14, 9);
        c.paddw(10, 9, 8);
        c.paddw(11, 10, 8);
        c.paddw(12, 11, 8);
        c.psllw(8, 8, 2);
        c.g[T5][0] = Addr(c.g[A0][0] as u32).off(0x20).lw(c);
        c.pextlw(13, 13, 13);
        c.pextlw(13, 13, 13);
        c.g[T6][0] = 0x3780_0000;
        c.qmtc2(14, 1);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.mtsab(0, 4);
        loop {
            c.cycles += 129;
            c.paddw(14, 13, 9);
            c.paddw(15, 13, 10);
            c.paddw(16, 13, 11);
            c.paddw(17, 13, 12);
            c.paddw(13, 13, 8);
            c.pinteh(18, 0, 14);
            c.pinteh(19, 0, 15);
            c.pinteh(20, 0, 16);
            c.pinteh(21, 0, 17);
            c.qmtc2(18, 2);
            c.qmtc2(19, 3);
            c.qmtc2(20, 4);
            c.qmtc2(21, 5);
            c.vu0().itof(0, 0b1111, 2, 2);
            c.vu0().itof(0, 0b1111, 3, 3);
            c.pextlw(18, 6, 6);
            c.vu0().itof(0, 0b1111, 4, 4);
            c.vu0().itof(0, 0b1111, 5, 5);
            c.vu0().mul(0b1111, 2, 2, Src::Bc(1, X));
            c.vu0().mul(0b1111, 3, 3, Src::Bc(1, X));
            c.pextlw(18, 18, 18);
            c.vu0().mul(0b1111, 4, 4, Src::Bc(1, X));
            c.vu0().mul(0b1111, 5, 5, Src::Bc(1, X));
            c.psrlw(14, 14, 16);
            c.psrlw(15, 15, 16);
            c.psrlw(16, 16, 16);
            c.psrlw(17, 17, 16);
            c.psllw(14, 14, 2);
            c.psllw(15, 15, 2);
            c.psllw(16, 16, 2);
            c.psllw(17, 17, 2);
            c.paddw(14, 14, 18);
            c.paddw(15, 15, 18);
            c.paddw(16, 16, 18);
            c.paddw(17, 17, 18);
            c.qfsrv(18, 0, 14);
            c.g[S3][0] = Addr(c.g[T6][0] as u32).lw(c);
            c.g[S4][0] = Addr(c.g[T6][0] as u32).off(0x4).lw(c);
            c.qfsrv(14, 0, 18);
            c.g[S5][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[S6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.g[S2][0] = c.g[T6][0] >> 32;
            c.g[S7][0] = Addr(c.g[T6][0] as u32).lw(c);
            c.g[T8][0] = Addr(c.g[T6][0] as u32).off(0x4).lw(c);
            c.pextlw(19, 23, 19);
            c.g[S7][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[T6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.pextlw(20, 24, 20);
            c.pextlw(21, 23, 21);
            c.pextlw(22, 14, 22);
            c.pextlw(19, 21, 19);
            c.pextlw(20, 22, 20);
            c.qmtc2(19, 6);
            c.qmtc2(20, 10);
            c.qfsrv(18, 0, 15);
            c.g[S3][0] = Addr(c.g[T7][0] as u32).lw(c);
            c.g[S4][0] = Addr(c.g[T7][0] as u32).off(0x4).lw(c);
            c.qfsrv(15, 0, 18);
            c.g[S5][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[S6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.g[S2][0] = c.g[T7][0] >> 32;
            c.g[S7][0] = Addr(c.g[T7][0] as u32).lw(c);
            c.g[T8][0] = Addr(c.g[T7][0] as u32).off(0x4).lw(c);
            c.pextlw(19, 23, 19);
            c.g[S7][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[T6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.pextlw(20, 24, 20);
            c.pextlw(21, 23, 21);
            c.pextlw(22, 14, 22);
            c.pextlw(19, 21, 19);
            c.pextlw(20, 22, 20);
            c.qmtc2(19, 7);
            c.qmtc2(20, 11);
            c.qfsrv(18, 0, 16);
            c.g[S3][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[S4][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            c.qfsrv(16, 0, 18);
            c.g[S5][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[S6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.g[S2][0] = c.g[S0][0] >> 32;
            c.g[S7][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[T8][0] = Addr(c.g[S0][0] as u32).off(0x4).lw(c);
            c.pextlw(19, 23, 19);
            c.g[S7][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[T6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.pextlw(20, 24, 20);
            c.pextlw(21, 23, 21);
            c.pextlw(22, 14, 22);
            c.pextlw(19, 21, 19);
            c.pextlw(20, 22, 20);
            c.qmtc2(19, 8);
            c.qmtc2(20, 12);
            c.qfsrv(18, 0, 17);
            c.g[S3][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[S4][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
            c.qfsrv(17, 0, 18);
            c.g[S5][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[S6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.g[S2][0] = c.g[S1][0] >> 32;
            c.g[S7][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[T8][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
            c.pextlw(19, 23, 19);
            c.g[S7][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[T6][0] = Addr(c.g[S2][0] as u32).off(0x4).lw(c);
            c.pextlw(20, 24, 20);
            c.pextlw(21, 23, 21);
            c.pextlw(22, 14, 22);
            c.pextlw(19, 21, 19);
            c.pextlw(20, 22, 20);
            c.qmtc2(19, 9);
            c.qmtc2(20, 13);
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x40));
            c.vu0().sub(0b1111, 10, 10, Src::Vec(6));
            c.vu0().sub(0b1111, 11, 11, Src::Vec(7));
            c.vu0().sub(0b1111, 12, 12, Src::Vec(8));
            c.vu0().sub(0b1111, 13, 13, Src::Vec(9));
            c.vu0().mul(0b1111, 10, 10, Src::Vec(2));
            c.vu0().mul(0b1111, 11, 11, Src::Vec(3));
            c.vu0().mul(0b1111, 12, 12, Src::Vec(4));
            c.vu0().mul(0b1111, 13, 13, Src::Vec(5));
            c.vu0().add(0b1111, 10, 10, Src::Vec(6));
            c.vu0().add(0b1111, 11, 11, Src::Vec(7));
            c.vu0().add(0b1111, 12, 12, Src::Vec(8));
            c.vu0().add(0b1111, 13, 13, Src::Vec(9));
            c.sqc2(10, (c.g[A3][0] as u32).wrapping_sub(0x40));
            c.sqc2(11, (c.g[A3][0] as u32).wrapping_sub(0x30));
            c.sqc2(12, (c.g[A3][0] as u32).wrapping_sub(0x20));
            let more = c.g[A1][0] != c.g[A3][0];
            c.sqc2(13, (c.g[A3][0] as u32).wrapping_sub(0x10));
            if !more {
                c.cycles += 21;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x20), c.g[T5][0]);
                c.g[A0][0] = c.lw(sp.wrapping_add(0x30));
                c.g[V0][0] = c.g[FP][0];
                c.g[A2][0] = c.lw(sp.wrapping_add(0x20));
                c.g[A1][0] = Addr(c.g[A0][0] as u32).off(0x20).lw(c);
                c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x26).lh(c);
                c.g[V1][0] = sx((c.g[A1][0] as u32) >> 16);
                c.g[T9][0] = c.lw(sp.wrapping_add(0x30));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.lwc1(0, c.g[A3][0] as u32);
                c.swc1(0, (c.g[T9][0] as u32).wrapping_add(0x28));
                c.lwc1(1, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.swc1(1, (c.g[T9][0] as u32).wrapping_add(0x2c));
                c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.swc1(0, (c.g[T9][0] as u32).wrapping_add(0x30));
                c.lwc1(1, (c.g[A3][0] as u32).wrapping_add(0xc));
                c.sw((c.g[T9][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                c.swc1(1, (c.g[T9][0] as u32).wrapping_add(0x34));
                return epilogue(c);
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_b89c);
            }
        }
    })
}

/// The shared return: restore the callee-saved registers, drop the frame, go
/// back to `ra`. (The `v0` the caller reads is whatever the path left there.)
fn epilogue(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 12;
    let sp = c.g[SP][0] as u32;
    c.lq(16, sp.wrapping_add(0xd0));
    c.lq(17, sp.wrapping_add(0xc0));
    c.lq(18, sp.wrapping_add(0xb0));
    c.lq(19, sp.wrapping_add(0xa0));
    c.lq(20, sp.wrapping_add(0x90));
    c.lq(21, sp.wrapping_add(0x80));
    c.lq(22, sp.wrapping_add(0x70));
    c.lq(23, sp.wrapping_add(0x60));
    c.lq(30, sp.wrapping_add(0x50));
    c.g[RA][0] = c.ld(sp.wrapping_add(0x40));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx(sp.wrapping_add(0xe0));
    c.pc = ra;
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, put_vec, put_words, scramble, Rng, DATA, GAME_GP};

    /// The static animation table (`AnimTable_GetEntry`'s base; 28-byte
    /// entries, the sequence type at +4).
    const ANIM_TABLE: u32 = 0x0044_6990;

    #[test]
    fn rider_anim_update_sequence_matches() {
        check(
            ssx3_ee::s00::f_001036a0__generated,
            rider_anim_update_sequence,
            24,
            &|c, seed| {
                let mut r = Rng(0x1036_a0d1 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let anim = DATA;
                let seq = DATA + 0x200;
                c.g[A0][0] = anim as u64;
                c.g[A1][0] = (DATA + 0x1800) as u64;
                c.g[A2][0] = seq as u64;
                c.f[12] = r.edge_f32();
                // The animation-type jump table (0x456950) selects the
                // per-type update; the test machine's RAM starts empty.
                put_words(
                    c,
                    0x0045_6950,
                    &[
                        0x0010_3700, 0x0010_371c, 0x0010_3738, 0x0010_3754, 0x0010_3770,
                        0x0010_378c, 0x0010_37a8, 0x0010_37c4, 0x0010_37e0, 0x0010_37fc,
                        0x0010_3818, 0x0010_3834, 0x0010_3850, 0x0010_386c, 0x0010_3888,
                        0x0010_38a4,
                    ],
                );
                // The sequence's animation-table index (seed covers all 20
                // entries: 0..15 run a per-type update, 16..19 skip it).
                let index = seed % 20;
                c.bus.write32(seq, index);
                c.bus.write32(ANIM_TABLE + index * 0x1c + 4, seed % 20);
                // The optional sound-event owner at anim+0x60.
                let owner = anim + 0x300;
                c.bus.write32(anim + 0x60, if seed % 2 == 0 { owner } else { 0 });
                c.bus.write32(owner + 0x874, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(owner + 0x87c, if seed % 4 == 0 { 0 } else { 1 });
                // `RiderAnim_FireSoundEvents` indexes a table of sound owners
                // through gp+0xd08; give it a table whose first entry is an
                // object with no events (it returns after the first load).
                c.bus.write32(GAME_GP + 0xd08, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, DATA + 0x1100);
                c.bus.write32(DATA + 0x1108, 0);
                c.bus.write16(DATA + 0x12, 0);
            },
        );
    }

    #[test]
    fn rail_find_nearest_point_matches() {
        check(
            ssx3_ee::s11::f_00335128__generated,
            rail_find_nearest_point,
            40,
            &|c, seed| {
                let mut r = Rng(0x3351_28a7 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0033_5128;
                c.g[GP][0] = GAME_GP as u64;
                let rail = DATA;
                let query = DATA + 0x400;
                let vec = DATA + 0x500;
                let out = DATA + 0x800;
                c.g[A0][0] = rail as u64;
                c.g[A1][0] = query as u64;
                c.g[A2][0] = vec as u64;
                c.g[A3][0] = (DATA + 0x600) as u64;
                c.g[T0][0] = (DATA + 0x700) as u64;
                c.g[T1][0] = out as u64;
                // Rail bounds: min 1.0, max 3.0. The query box is either
                // inside them (main path) or below the minimum (reject).
                for off in [0x6c, 0x70, 0x74] {
                    c.bus.write32(rail + off, 0x3f80_0000);
                }
                for off in [0x78, 0x7c, 0x80] {
                    c.bus.write32(rail + off, 0x4040_0000);
                }
                let below = seed % 4 == 0;
                for off in [0x0, 0x4, 0x8] {
                    c.bus.write32(query + off, 0);
                }
                let max = if below { 0x3f00_0000 } else { 0x4000_0000 };
                for off in [0x10, 0x14, 0x18] {
                    c.bus.write32(query + off, max);
                }
                // The rail's 4x4 matrix and the query vector.
                for i in 0..4 {
                    put_vec(
                        c,
                        rail + 0x10 + i * 0x10,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.next_u32()],
                    );
                }
                put_vec(c, vec, [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.next_u32()]);
                // The result word read at 0x335804 / 0x335824, and the float
                // at t0 compared on the way to the success block.
                c.bus.write32(DATA + 0x600, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x700, r.edge_f32());
                // The gp-relative constants the pass-2 setup scales by.
                c.bus.write32(GAME_GP - 0x2d68, r.edge_f32());
                c.bus.write32(GAME_GP - 0x2d64, r.edge_f32());
                c.bus.write32(GAME_GP - 0x2d60, r.edge_f32());
                // The rail pointer read in the success block.
                c.bus.write32(rail + 0x68, DATA + 0x900);
                c.bus.write32(DATA + 0x928, r.next_u32());
            },
        );
    }

    #[test]
    fn snd_dsp_resample_process_matches() {
        use crate::difftest::fakes;
        check(
            ssx3_ee::s14::f_003cb540__generated,
            snd_dsp_resample_process,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x3cb5_40, 2);
                let o = DATA;
                // Format at +0x1c selects the pass-through half (0x10000) or
                // the resampling half (anything else).
                let fast = seed % 2 == 0;
                c.bus.write32(o + 0x1c, if fast { 0x1_0000 } else { 0x2_0000 });
                c.bus.write16(o + 0x24, if seed % 4 == 0 { 1 } else { 0 });
                c.bus.write16(o + 0x26, (seed % 3) as u16);
                c.bus.write32(o + 0x20, 0x1_0000);
                c.bus.write8(o + 0x1a, 0);
                // The resample callback: `o+0x8` points at a one-word slot
                // holding a fake target (outside the function table, so the
                // call lands in the `miss` hook).
                c.bus.write32(o + 0x8, DATA + 0x300);
                c.bus.write32(DATA + 0x300, fakes::FAKE);
                for i in 0..4u32 {
                    c.bus.write32(o + 0x28 + 4 * i, 0x3f80_0000 + i);
                }
                // The hand-written routine's extra input pointers (s0, s1,
                // s2, t7) and the mult operands used for the sample count.
                for (i, reg) in [S0, S1, S2, T7].into_iter().enumerate() {
                    c.g[reg][0] = (DATA + 0x2000 + 0x100 * i as u32) as u64;
                    c.bus.write32(DATA + 0x2000 + 0x100 * i as u32, 0);
                    c.bus.write32(DATA + 0x2004 + 0x100 * i as u32, 0);
                }
                // Arguments: the object, the number of floats to emit (16, so
                // the MMI loop is one 0x40-byte step), the cursor buffer and
                // the output start.
                c.g[A0][0] = o as u64;
                c.g[A1][0] = 16;
                c.g[A2][0] = (DATA + 0x2600) as u64;
                c.g[A3][0] = (DATA + 0x2400) as u64;
                // Unwind through the callback on the first two seeds.
                if seed < 2 {
                    c.bus.write32(fakes::LOG_UNWIND_AT, 0);
                }
            },
        );
    }
}
