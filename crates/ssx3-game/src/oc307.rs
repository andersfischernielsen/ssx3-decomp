//! Staging wave 212 (oc307): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003e_14c8,
        stream_file_get_channel_bytes,
        "StreamFile_GetChannelBytes",
    ),
    (
        0x0011_b1a8,
        trick_lookup_bonus_trick,
        "Trick_LookupBonusTrick",
    ),
    (
        0x0012_f620,
        rider_natural_air_sub_enter,
        "RiderNaturalAirSub_Enter",
    ),
    (
        0x0011_97d8,
        rider_metrix_complete_trick,
        "RiderMetrix_CompleteTrick",
    ),
    (0x0011_5168, rider_toggle_stance, "Rider_ToggleStance"),
    (
        0x0010_e228,
        rider_report_rider_hit0,
        "Rider_ReportRiderHit0",
    ),
    (0x0034_c848, c_flag_manager_save, "cFlagManager_Save"),
    (0x003e_1530, stream_file_is_idle, "StreamFile_IsIdle"),
    (0x0034_6300, halo_modifier_save, "HaloModifier_Save"),
];

/// `StreamFile_Validate(file, &header, &handle)` (0x3dfbd0): 0 when the
/// stream file is valid (and the out parameters are written), else 1.
const SFILE_VALIDATE: Callee = Callee {
    addr: 0x003d_fbd0,
    f: ee::s14::f_003dfbd0,
};

/// The stream handle's buffered byte count for its channel.
pub(crate) const SFILE_HANDLE_CHANNEL_BYTES: u32 = 0x8;

/// `int StreamFile_GetChannelBytes(StreamFile *file)` (0x3e14c8): the bytes
/// buffered for the stream file's channel (handle +0x8), or 0 when the file
/// does not validate.
pub fn stream_file_get_channel_bytes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        // Out parameters: header at 0(sp), handle at 4(sp).
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[SP][0] | 0x4;
        SFILE_VALIDATE.call(c, 0x003e_14dc)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 2;
            let handle = Addr(frame.at(c, 4)).lw(c);
            c.g[V1][0] = handle;
            c.g[V0][0] = Addr((handle as u32).wrapping_add(SFILE_HANDLE_CHANNEL_BYTES)).lw(c);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The bonus-trick table (24 entries of 16 bytes): +0 grade (stored into
/// the trick id's top 5 bits), +4 points, +8..+0xe the trick-id fields to
/// match.
pub(crate) const BONUS_TRICKS: u32 = 0x0043_d608;

pub(crate) const BONUS_TRICK_COUNT: i64 = 24;

pub(crate) const BONUS_TRICK_SIZE: u32 = 0x10;

pub(crate) const BONUS_TRICK_GRADE: u32 = 0x0;

pub(crate) const BONUS_TRICK_POINTS: u32 = 0x4;

/// `EA_MemSet(dst, value, n)` (0x3e6448, through the function table).
const BONUS_TRICK_MEM_SET: u32 = 0x003e_6448;

/// The bonus-table loop head (event checks resume there).
const BONUS_TRICK_LOOP: u32 = 0x0011_b1d8;

/// How one bonus-table entry compared with the trick id.
enum BonusTrickCmp {
    /// A field before the last one differed.
    Miss,
    /// Only the last field (word 1 bits 11..17) differed.
    LastMiss,
    /// Every field matched.
    Hit,
}

/// Compare the entry at s0 with the trick id at s1 (a1 counts entries).
fn bonus_trick_compare(c: &mut Cpu) -> BonusTrickCmp {
    c.cycles += 6;
    let entry = r32(c, S0);
    let w0 = Addr(r32(c, S1)).lw(c);
    c.g[A0][0] = w0;
    // Word 0: bits 10..11, 12..15, 16..18, 19..21 against bytes +8..+0xb.
    for (i, &(shift, mask, off)) in [
        (10, 0x3, 0x8),
        (12, 0xf, 0x9),
        (16, 0x7, 0xa),
        (19, 0x7, 0xb),
    ]
    .iter()
    .enumerate()
    {
        if i > 0 {
            c.cycles += 5;
        }
        c.g[V1][0] = Addr(entry.wrapping_add(off)).lbu(c);
        c.g[V0][0] = sx((w0 as u32) >> shift) & mask;
        if c.g[V0][0] != c.g[V1][0] {
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
            return BonusTrickCmp::Miss;
        }
    }
    // Bits 28..31 against +0xc (the counter steps either way).
    c.cycles += 4;
    c.g[V1][0] = Addr(entry.wrapping_add(0xc)).lbu(c);
    c.g[V0][0] = sx((w0 as u32) >> 28);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
    if c.g[V0][0] != c.g[V1][0] {
        return BonusTrickCmp::Miss;
    }
    // Word 1: bits 3..9 against +0xd, bits 11..17 against +0xe.
    c.cycles += 6;
    let w1 = Addr(r32(c, S1).wrapping_add(4)).lw(c);
    c.g[A0][0] = w1;
    c.g[V1][0] = Addr(entry.wrapping_add(0xd)).lbu(c);
    c.g[V0][0] = sx((w1 as u32) >> 3) & 0x7f;
    if c.g[V0][0] != c.g[V1][0] {
        return BonusTrickCmp::Miss;
    }
    c.cycles += 5;
    c.g[V0][0] = sx((w1 as u32) >> 11) & 0x7f;
    c.g[V1][0] = Addr(entry.wrapping_add(0xe)).lbu(c);
    if c.g[V0][0] != c.g[V1][0] {
        return BonusTrickCmp::LastMiss;
    }
    BonusTrickCmp::Hit
}

/// `int Trick_LookupBonusTrick(RiderMetrix *self, sTrickId *id)` (0x11b1a8):
/// look the trick id up in the 24-entry bonus-trick table; on a match clear
/// the id's first 8 bytes, store the entry's grade into the id's top 5 bits
/// and return the entry's points; 0 when no entry matches.
pub fn trick_lookup_bonus_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x44_0000;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(BONUS_TRICKS);
        c.g[A1][0] = 0;
        c.g[S2][0] = 0x07ff_ffff;
        loop {
            match bonus_trick_compare(c) {
                BonusTrickCmp::Hit => {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0x8;
                    call_ptr(c, BONUS_TRICK_MEM_SET, 0x0011_b278)?;
                    let entry = r32(c, S0);
                    let id = r32(c, S1);
                    let grade = Addr(entry.wrapping_add(BONUS_TRICK_GRADE)).lw(c);
                    let w1 = Addr(id.wrapping_add(4)).lw(c);
                    c.g[V1][0] = sx((grade as u32) << 27);
                    let w1 = (w1 & c.g[S2][0]) | c.g[V1][0];
                    c.g[V0][0] = w1;
                    Addr(id.wrapping_add(4)).sw(c, w1);
                    c.g[V0][0] = Addr(entry.wrapping_add(BONUS_TRICK_POINTS)).lw(c);
                    break;
                }
                cmp => {
                    if let BonusTrickCmp::Miss = cmp {
                        c.cycles += 1;
                    }
                    c.g[V0][0] = ((c.g[A1][0] as i64) < BONUS_TRICK_COUNT) as u64;
                }
            }
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(BONUS_TRICK_SIZE));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, BONUS_TRICK_LOOP);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderNaturalAirSub` fields: heading angle +0x0, speed +0x4, two
/// flags +0x8/+0xc (set to 1), a timer +0x10 (set to -1), the rider +0x14.
pub(crate) const NAT_AIR_ANGLE: u32 = 0x0;

pub(crate) const NAT_AIR_SPEED: u32 = 0x4;

pub(crate) const NAT_AIR_FLAG_A: u32 = 0x8;

pub(crate) const NAT_AIR_FLAG_B: u32 = 0xc;

pub(crate) const NAT_AIR_TIMER: u32 = 0x10;

pub(crate) const NAT_AIR_RIDER: u32 = 0x14;

/// Rider fields: the horizontal (x) and vertical (y) air velocity
/// components, and two (step, counter) pairs reset on entry.
pub(crate) const NAT_AIR_RIDER_VX: u32 = 0x1f0;

pub(crate) const NAT_AIR_RIDER_VY: u32 = 0x220;

pub(crate) const NAT_AIR_RIDER_STEP_A: u32 = 0x2a8;

pub(crate) const NAT_AIR_RIDER_COUNT_A: u32 = 0x2ac;

pub(crate) const NAT_AIR_RIDER_STEP_B: u32 = 0x2b4;

pub(crate) const NAT_AIR_RIDER_COUNT_B: u32 = 0x2b8;

/// gp-relative `.lit4` constants: -pi/2 (x = 0, y < 0), pi/2 (x = 0,
/// y >= 0), the +pi / -pi quadrant fixes for x < 0, and the step 1/30.
pub(crate) const NAT_AIR_GP_DOWN: u32 = 0x7718;

pub(crate) const NAT_AIR_GP_UP: u32 = 0x7714;

pub(crate) const NAT_AIR_GP_PLUS_PI: u32 = 0x7710;

pub(crate) const NAT_AIR_GP_MINUS_PI: u32 = 0x770c;

pub(crate) const NAT_AIR_GP_STEP: u32 = 0x7708;

/// `Math_Atan(x)` (0x31c228, through the function table).
const NAT_AIR_MATH_ATAN: u32 = 0x0031_c228;

/// A gp-relative float constant at gp - `off`.
fn nat_air_gp_const(c: &mut Cpu, off: u32) -> FpuF32 {
    Addr(r32(c, GP).wrapping_sub(off)).read_fpu(c)
}

/// `void RiderNaturalAirSub_Enter(RiderNaturalAirSub *self)` (0x12f620):
/// entering the natural-air sub-state. Speed = max(|vx|, vy), heading =
/// atan2(vy, vx) (computed with `Math_Atan` and quadrant fixes), both flags
/// 1, the timer -1, and the rider's two (step, counter) pairs reset to
/// (1/30, 0).
pub fn rider_natural_air_sub_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 22, 0x30);
        frame.save_fpr(c, 21, 0x28);
        frame.save_fpr(c, 20, 0x20);
        let this = r32(c, S0);
        let rider = Addr(this.wrapping_add(NAT_AIR_RIDER)).lw(c);
        c.g[V0][0] = rider;
        let rider = rider as u32;
        let vy = Addr(rider.wrapping_add(NAT_AIR_RIDER_VY)).read_fpu(c);
        c.f[1] = vy.0;
        let mut speed = Addr(rider.wrapping_add(NAT_AIR_RIDER_VX)).read_fpu(c).abs();
        if abi::c_lt_s(c, speed, vy) {
            speed = vy;
        }
        c.f[0] = speed.0;

        c.cycles += 7;
        Addr(this.wrapping_add(NAT_AIR_SPEED)).write_fpu(c, speed);
        let zero = FpuF32(0);
        c.f[22] = 0;
        let vx = Addr(rider.wrapping_add(NAT_AIR_RIDER_VX)).read_fpu(c);
        c.f[20] = vx.0;
        let vertical = vx.eq_s(zero);
        abi::set_fcc(c, vertical);
        let vy = Addr(rider.wrapping_add(NAT_AIR_RIDER_VY)).read_fpu(c);
        c.f[21] = vy.0;

        // Where the original rejoins: whether a0 (the rider) still has to be
        // reloaded, and whether v1 = 1 is still to be set.
        let (reload_rider, set_flag) = if vertical {
            // Straight up or down (or no velocity at all).
            c.cycles += 4;
            let still = vy.eq_s(zero);
            abi::set_fcc(c, still);
            c.f[12] = vy.0;
            if still {
                (true, true)
            } else {
                c.cycles += 4;
                let up = zero.le(vy);
                abi::set_fcc(c, up);
                c.f[12] = nat_air_gp_const(c, NAT_AIR_GP_DOWN).0;
                c.g[A0][0] = Addr(this.wrapping_add(NAT_AIR_RIDER)).lw(c);
                if !up {
                    (false, true)
                } else {
                    c.cycles += 3;
                    c.f[12] = nat_air_gp_const(c, NAT_AIR_GP_UP).0;
                    c.g[V1][0] = 1;
                    (false, false)
                }
            }
        } else {
            c.cycles += 7;
            c.f[12] = vy.div(vx).0;
            call_ptr(c, NAT_AIR_MATH_ATAN, 0x0012_f6b0)?;
            let (vx, zero, vy) = (FpuF32(c.f[20]), FpuF32(c.f[22]), FpuF32(c.f[21]));
            let backwards = abi::c_lt_s(c, vx, zero);
            c.f[12] = c.f[0];
            if backwards {
                c.cycles += 3;
                let fix = if abi::c_lt_s(c, zero, vy) {
                    c.cycles += 3;
                    let pi = nat_air_gp_const(c, NAT_AIR_GP_PLUS_PI);
                    c.f[0] = pi.0;
                    FpuF32(c.f[12]) + pi
                } else {
                    c.cycles += 2;
                    let pi = nat_air_gp_const(c, NAT_AIR_GP_MINUS_PI);
                    c.f[0] = pi.0;
                    FpuF32(c.f[12]) - pi
                };
                c.f[12] = fix.0;
            }
            (true, true)
        };
        if reload_rider {
            c.cycles += 1;
            c.g[A0][0] = Addr(this.wrapping_add(NAT_AIR_RIDER)).lw(c);
        }
        if set_flag {
            c.cycles += 1;
            c.g[V1][0] = 1;
        }

        c.cycles += 18;
        let this = r32(c, S0);
        let step = nat_air_gp_const(c, NAT_AIR_GP_STEP);
        c.f[0] = step.0;
        c.g[V0][0] = u64::MAX;
        Addr(this.wrapping_add(NAT_AIR_TIMER)).sw(c, u64::MAX);
        let angle = FpuF32(c.f[12]);
        Addr(this.wrapping_add(NAT_AIR_ANGLE)).write_fpu(c, angle);
        let one = c.g[V1][0];
        Addr(this.wrapping_add(NAT_AIR_FLAG_B)).sw(c, one);
        Addr(this.wrapping_add(NAT_AIR_FLAG_A)).sw(c, one);
        let rider = r32(c, A0);
        Addr(rider.wrapping_add(NAT_AIR_RIDER_STEP_A)).write_fpu(c, step);
        Addr(rider.wrapping_add(NAT_AIR_RIDER_COUNT_A)).sw(c, 0);
        frame.restore_ra(c, 0);
        let rider = Addr(this.wrapping_add(NAT_AIR_RIDER)).lw(c);
        c.g[V0][0] = rider;
        frame.restore(c, S0, 0x10);
        frame.restore_fpr(c, 22, 0x30);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        let rider = rider as u32;
        Addr(rider.wrapping_add(NAT_AIR_RIDER_STEP_B)).write_fpu(c, step);
        Addr(rider.wrapping_add(NAT_AIR_RIDER_COUNT_B)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `RiderMetrix` fields: the running trick's score +0x40 (-1 when none),
/// the score total +0x44, the best single score +0x48, completed-trick
/// counters (small +0x4c, medium +0x50, big +0x54, big while the rider's
/// level is >= 6 +0x58), a combo flag +0x5c, the rider +0x1ac.
pub(crate) const METRIX_TRICK_SCORE: u32 = 0x40;

pub(crate) const METRIX_SCORE_TOTAL: u32 = 0x44;

pub(crate) const METRIX_BEST_SCORE: u32 = 0x48;

pub(crate) const METRIX_SMALL_TRICKS: u32 = 0x4c;

pub(crate) const METRIX_MEDIUM_TRICKS: u32 = 0x50;

pub(crate) const METRIX_BIG_TRICKS: u32 = 0x54;

pub(crate) const METRIX_BIG_TRICKS_HIGH: u32 = 0x58;

pub(crate) const METRIX_COMBO_FLAG: u32 = 0x5c;

pub(crate) const METRIX_RIDER: u32 = 0x1ac;

/// The rider's level field and the level from which big tricks also
/// count in +0x58.
pub(crate) const METRIX_RIDER_LEVEL: u32 = 0x2f4;

pub(crate) const METRIX_HIGH_LEVEL: i64 = 6;

/// Trick ids from 19 are medium, from 35 big.
pub(crate) const METRIX_MEDIUM_ID: i64 = 19;

pub(crate) const METRIX_BIG_ID: i64 = 35;

/// `RiderMetrix_AddTrickHistory(metrix, id)` (0x119068).
const METRIX_ADD_TRICK_HISTORY: Callee = Callee {
    addr: 0x0011_9068,
    f: ee::s00::f_00119068,
};

/// `float RiderMetrix_CompleteTrick(RiderMetrix *m, int id)` (0x1197d8):
/// record the trick in the history, count it as small / medium / big (big
/// ones also in +0x58 when the rider's level is at least 6), then fold the
/// running score into the best score and the total, reset it to -1, clear
/// the combo flag; returns 0.0.
pub fn rider_metrix_complete_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        METRIX_ADD_TRICK_HISTORY.call(c, 0x0011_97f4)?;
        let m = r32(c, S0);
        let id = c.g[S1][0] as i64;
        c.g[V0][0] = (id < METRIX_MEDIUM_ID) as u64;
        // Bump the counter at m + off (v0 = the new count).
        let bump = |c: &mut Cpu, off: u32| {
            let n = sx((Addr(m.wrapping_add(off)).lw(c) as u32).wrapping_add(1));
            c.g[V0][0] = n;
            Addr(m.wrapping_add(off)).sw(c, n);
        };
        let mut joined = true;
        if id >= METRIX_BIG_ID {
            c.cycles += 8;
            let rider = Addr(m.wrapping_add(METRIX_RIDER)).lw(c);
            c.g[A0][0] = rider;
            bump(c, METRIX_BIG_TRICKS);
            let level = Addr((rider as u32).wrapping_add(METRIX_RIDER_LEVEL)).lw(c) as i64;
            c.g[V1][0] = (level < METRIX_HIGH_LEVEL) as u64;
            if level < METRIX_HIGH_LEVEL {
                joined = false;
            } else {
                c.cycles += 4;
                bump(c, METRIX_BIG_TRICKS_HIGH);
            }
        } else {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                bump(c, METRIX_SMALL_TRICKS);
            } else {
                c.cycles += 4;
                bump(c, METRIX_MEDIUM_TRICKS);
            }
        }
        if joined {
            c.cycles += 1;
        }
        let score = Addr(m.wrapping_add(METRIX_TRICK_SCORE)).read_fpu(c);

        c.cycles += 16;
        let total = Addr(m.wrapping_add(METRIX_SCORE_TOTAL)).read_fpu(c);
        let best = Addr(m.wrapping_add(METRIX_BEST_SCORE)).read_fpu(c);
        c.f[2] = best.0;
        let total = total + score;
        c.f[1] = total.0;
        // -1.0
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.f[3] = 0xbf80_0000;
        let best = score.max(best);
        Addr(m.wrapping_add(METRIX_COMBO_FLAG)).sw(c, 0);
        Addr(m.wrapping_add(METRIX_BEST_SCORE)).write_fpu(c, best);
        Addr(m.wrapping_add(METRIX_SCORE_TOTAL)).write_fpu(c, total);
        Addr(m.wrapping_add(METRIX_TRICK_SCORE)).write_u32(c, 0xbf80_0000);
        c.f[0] = 0;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Rider fields: the stance flag +0x320 (1 = switch), the animation
/// controller +0x784, two direction quadwords +0x3a0/+0x3b0 flipped on a
/// stance change, and the lateral velocity / orientation components negated
/// with them.
pub(crate) const STANCE_FLAG: u32 = 0x320;

pub(crate) const STANCE_ANIM: u32 = 0x784;

pub(crate) const STANCE_DIR_A: u32 = 0x3a0;

pub(crate) const STANCE_DIR_B: u32 = 0x3b0;

/// The negated floats in the order the original loads them (into f5, f7,
/// f8, f6, f4, f3, f2, f0, f1), then +0x288.
pub(crate) const STANCE_NEGATED: [(u32, usize); 9] = [
    (0x1f0, 5),
    (0x1f8, 7),
    (0x214, 8),
    (0x21c, 6),
    (0x208, 4),
    (0x210, 3),
    (0x1fc, 2),
    (0x204, 0),
    (0x280, 1),
];

pub(crate) const STANCE_NEGATED_LAST: u32 = 0x288;

/// Animation controller fields: root translation +0x30 and rotation
/// quaternion +0x40, and the stance copy +0x18.
pub(crate) const STANCE_ANIM_ROOT_POS: u32 = 0x30;

pub(crate) const STANCE_ANIM_ROOT_ROT: u32 = 0x40;

pub(crate) const STANCE_ANIM_STANCE: u32 = 0x18;

/// gp-relative `.lit4`: the half turn pi (rotate roots by it) and the
/// switch-stance root half angle pi/2.
pub(crate) const STANCE_GP_PI: u32 = 0x7b88;

pub(crate) const STANCE_GP_HALF_ANGLE: u32 = 0x7b84;

/// The zero root translation and the root rotation axis (0x4ff130,
/// 0x4ff160).
pub(crate) const STANCE_ROOT_TRANSLATION: u32 = 0x004f_f130;

pub(crate) const STANCE_ROOT_AXIS: u32 = 0x004f_f160;

/// `AnimController_RotateRoots(ctrl, angle)`, `Math_SinCos(angle, &sin,
/// &cos)` (through the function table).
const STANCE_ROTATE_ROOTS: u32 = 0x0031_1b48;

const STANCE_SIN_COS: u32 = 0x0031_be50;

/// Write the root quaternion `{axis * sin, cos}` (sin/cos at `sin_at`,
/// `sin_at + 4` on the stack, the axis read through `axis` (+0, +4, +8))
/// to `ctrl + 0x40`, through the stack at 0(sp) as the original does.
fn stance_store_root_rotation(c: &mut Cpu, sin_at: u32, axis: u32, axis_x: u32, ctrl: u32) {
    let axis_z = Addr(axis.wrapping_add(8)).read_fpu(c);
    let sin = Addr(sp(c).wrapping_add(sin_at)).read_fpu(c);
    let ax = Addr(axis_x).read_fpu(c);
    let ay = Addr(axis.wrapping_add(4)).read_fpu(c);
    c.f[4] = ay.0;
    let qz = sin * axis_z;
    let qx = sin * ax;
    let cos = Addr(sp(c).wrapping_add(sin_at + 4)).read_fpu(c);
    let qy = sin * ay;
    c.f[1] = qz.0;
    c.f[2] = qx.0;
    c.f[3] = cos.0;
    c.f[0] = qy.0;
    let q = sp(c);
    Addr(q + 0xc).write_fpu(c, cos);
    Addr(q + 8).write_fpu(c, qz);
    Addr(q).write_fpu(c, qx);
    Addr(q + 4).write_fpu(c, qy);
    c.lq(V0, q);
    c.sq(V0, ctrl.wrapping_add(STANCE_ANIM_ROOT_ROT));
}

/// `void Rider_ToggleStance(Rider *rider)` (0x115168): switch between
/// regular and switch stance. Flip the stance flag, rotate the animation
/// roots by a half turn, reset the root transform (zero translation; the
/// rotation about the root axis by the stance's angle, pi or -0), copy the
/// stance to the controller, and mirror the rider's direction quadwords
/// (scaled by -1 on VU0) and the lateral velocity / orientation floats.
pub fn rider_toggle_stance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x80);
        c.f[12] = Addr(r32(c, GP).wrapping_sub(STANCE_GP_PI)).read_u32(c);
        frame.save(c, S3, 0x40);
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0x30);
        let rider = r32(c, S3);
        let stance = Addr(rider.wrapping_add(STANCE_FLAG)).lw(c) ^ 1;
        c.g[A0][0] = Addr(rider.wrapping_add(STANCE_ANIM)).lw(c);
        c.g[V0][0] = stance;
        Addr(rider.wrapping_add(STANCE_FLAG)).sw(c, stance);
        call_ptr(c, STANCE_ROTATE_ROOTS, 0x0011_519c)?;

        let rider = r32(c, S3);
        let switch = Addr(rider.wrapping_add(STANCE_FLAG)).lw(c) != 0;
        c.g[V0][0] = 0x50_0000;
        c.cycles += 25;
        if switch {
            let ctrl = Addr(rider.wrapping_add(STANCE_ANIM)).lw(c);
            c.g[S2][0] = ctrl;
            c.g[V0][0] = sx(STANCE_ROOT_TRANSLATION);
            c.f[12] = Addr(r32(c, GP).wrapping_sub(STANCE_GP_HALF_ANGLE)).read_u32(c);
            c.lq(V1, STANCE_ROOT_TRANSLATION);
            c.g[S1][0] = 0x50_0000;
            c.g[A0][0] = sx(frame.at(c, 0x20));
            c.g[A1][0] = sx(frame.at(c, 0x24));
            c.sq(V1, (ctrl as u32).wrapping_add(STANCE_ANIM_ROOT_POS));
            c.g[S0][0] = sx(STANCE_ROOT_AXIS);
            call_ptr(c, STANCE_SIN_COS, 0x0011_51d0)?;
            let axis = r32(c, S0);
            let axis_x = r32(c, S1).wrapping_sub(0xea0);
            let ctrl = r32(c, S2);
            stance_store_root_rotation(c, 0x20, axis, axis_x, ctrl);
        } else {
            c.f[12] = 0;
            c.g[V0][0] = sx(STANCE_ROOT_TRANSLATION);
            let ctrl = Addr(rider.wrapping_add(STANCE_ANIM)).lw(c);
            c.g[S1][0] = ctrl;
            c.lq(V1, STANCE_ROOT_TRANSLATION);
            c.f[12] ^= 0x8000_0000;
            c.g[A0][0] = sx(frame.at(c, 0x28));
            c.g[A1][0] = sx(frame.at(c, 0x2c));
            c.sq(V1, (ctrl as u32).wrapping_add(STANCE_ANIM_ROOT_POS));
            c.g[S0][0] = 0x50_0000;
            call_ptr(c, STANCE_SIN_COS, 0x0011_5234)?;
            let axis = r32(c, S0).wrapping_sub(0xea0);
            c.g[V0][0] = sx(axis);
            let ctrl = r32(c, S1);
            stance_store_root_rotation(c, 0x28, axis, axis, ctrl);
        }

        c.cycles += 58;
        let rider = r32(c, S3);
        let stance = Addr(rider.wrapping_add(STANCE_FLAG)).lw(c);
        c.g[V0][0] = stance;
        let ctrl = Addr(rider.wrapping_add(STANCE_ANIM)).lw(c);
        c.g[V1][0] = ctrl;
        // -1.0
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.f[0] = 0xbf80_0000;
        Addr((ctrl as u32).wrapping_add(STANCE_ANIM_STANCE)).sw(c, stance);
        // Both direction quadwords scaled by -1 (VU0 mul by vf3.x).
        let scratch = frame.at(c, 0x10);
        c.mfc1(A0, 0);
        c.lqc2(4, rider.wrapping_add(STANCE_DIR_A));
        c.qmtc2(A0, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, scratch);
        c.lq(V0, scratch);
        c.sq(V0, rider.wrapping_add(STANCE_DIR_A));
        c.sq(V0, sp(c));
        c.mfc1(V1, 0);
        c.lqc2(4, rider.wrapping_add(STANCE_DIR_B));
        c.qmtc2(V1, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, scratch);
        for &(off, f) in STANCE_NEGATED.iter() {
            c.f[f] = Addr(rider.wrapping_add(off)).read_u32(c) ^ 0x8000_0000;
        }
        c.lq(V0, scratch);
        let store = |c: &mut Cpu, i: usize| {
            let (off, f) = STANCE_NEGATED[i];
            let v = c.f[f];
            Addr(rider.wrapping_add(off)).write_u32(c, v);
        };
        store(c, 0);
        c.sq(V0, rider.wrapping_add(STANCE_DIR_B));
        for i in 1..8 {
            store(c, i);
        }
        c.sq(V0, sp(c));
        store(c, 8);
        let last = Addr(rider.wrapping_add(STANCE_NEGATED_LAST)).read_u32(c);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x30);
        let last = last ^ 0x8000_0000;
        c.f[0] = last;
        Addr(rider.wrapping_add(STANCE_NEGATED_LAST)).write_u32(c, last);
        frame.restore(c, S3, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Table callees of `Rider_ReportRiderHit0`: `SSXAudioSystem_GetInstance`,
/// `SSXAudioSystem_PlayRiderCollisionSound(audio, a, b)`,
/// `SSXAudioSystem_SpeechRiderCollision(audio, b, a, 1, 0)`,
/// `BEAggressionInt_GetInstance`, `BEAggressionInt_AddAggression(agg,
/// rider, rival, 0)`.
const HIT_AUDIO_INSTANCE: u32 = 0x0028_b180;

const HIT_PLAY_COLLISION_SOUND: u32 = 0x0029_8488;

const HIT_SPEECH_COLLISION: u32 = 0x002a_0a30;

const HIT_AGGRESSION_INSTANCE: u32 = 0x0015_5a50;

const HIT_ADD_AGGRESSION: u32 = 0x0015_5bf0;

/// The rider's controller sub-object (+0x6c0) and its virtual
/// `GetRiderIndex` slot (vtable +0x38: this delta, +0x3c: function).
pub(crate) const HIT_CONTROLLER: u32 = 0x6c0;

pub(crate) const HIT_INDEX_SLOT: u32 = 0x38;

/// Call `(rider + 0x6c0)->GetRiderIndex()` for the rider in `reg`; `reg`
/// becomes the controller pointer, as in the original.
fn hit_rider_index(c: &mut Cpu, reg: usize, ret: u32) -> crate::call::Flow {
    let ctl = r32(c, reg).wrapping_add(HIT_CONTROLLER);
    let vt = Addr(ctl).lw(c);
    c.g[V1][0] = vt;
    c.g[reg][0] = sx(ctl);
    let delta = Addr((vt as u32).wrapping_add(HIT_INDEX_SLOT)).lh(c);
    let f = Addr((vt as u32).wrapping_add(HIT_INDEX_SLOT + 4)).lw(c);
    c.g[V0][0] = f;
    c.g[A0][0] = sx(ctl.wrapping_add(delta as u32));
    call_ptr(c, f as u32, ret)
}

/// `void Rider_ReportRiderHit0(Rider *a, Rider *b)` (0x10e228): rider `a`
/// hit rider `b`: play the collision sound and the collision speech (kind
/// 1, variant 0), and add aggression from `b`'s index towards `a`'s.
pub fn rider_report_rider_hit0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 47;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, HIT_AUDIO_INSTANCE, 0x0010_e248)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, HIT_PLAY_COLLISION_SOUND, 0x0010_e258)?;
        call_ptr(c, HIT_AUDIO_INSTANCE, 0x0010_e260)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[T0][0] = 0;
        c.g[A3][0] = 1;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, HIT_SPEECH_COLLISION, 0x0010_e278)?;
        call_ptr(c, HIT_AGGRESSION_INSTANCE, 0x0010_e280)?;
        c.g[S2][0] = c.g[V0][0];
        hit_rider_index(c, S0, 0x0010_e29c)?;
        c.g[S0][0] = c.g[V0][0];
        hit_rider_index(c, S1, 0x0010_e2b8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = 0;
        call_ptr(c, HIT_ADD_AGGRESSION, 0x0010_e2cc)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFlagManager`: 15 flag groups of 0x188 bytes from +0x20; a group's
/// item count is at +0x5c.
pub(crate) const FLAG_MANAGER_GROUPS: u32 = 0x20;

pub(crate) const FLAG_MANAGER_GROUP_SIZE: u32 = 0x188;

pub(crate) const FLAG_MANAGER_GROUP_COUNT: u32 = 0x5c;

/// `cFlagManager_GroupSave(group, stream)` (0x34c2e0).
const FLAG_MANAGER_GROUP_SAVE: Callee = Callee {
    addr: 0x0034_c2e0,
    f: ee::s11::f_0034c2e0,
};

/// `void cFlagManager_Save(cFlagManager *self, Stream *stream)` (0x34c848):
/// save the 15 flag groups that follow the header.
pub fn c_flag_manager_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(FLAG_MANAGER_GROUPS));
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0xe;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            FLAG_MANAGER_GROUP_SAVE.call(c, 0x0034_c874)?;
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(FLAG_MANAGER_GROUP_SIZE));
            if !more {
                c.cycles += 6;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_c868);
            }
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The stream header's read state (0 = idle).
pub(crate) const SFILE_HEADER_READ_STATE: u32 = 0x38;

/// `bool StreamFile_IsIdle(StreamFile *file)` (0x3e1530): whether a valid
/// stream file has no read in flight (header read state 0) and no bytes
/// buffered for its channel; false for an invalid file.
pub fn stream_file_is_idle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        // Out parameters: header at 0(sp), handle at 4(sp).
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[SP][0] | 0x4;
        SFILE_VALIDATE.call(c, 0x003e_1544)?;
        let invalid = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !invalid {
            c.cycles += 5;
            let header = Addr(frame.at(c, 0)).lw(c);
            c.g[V1][0] = header;
            c.g[A0][0] = 0;
            let state = Addr((header as u32).wrapping_add(SFILE_HEADER_READ_STATE)).lw(c);
            c.g[V0][0] = 0;
            if state == 0 {
                c.cycles += 4;
                let handle = Addr(frame.at(c, 4)).lw(c);
                let bytes = Addr((handle as u32).wrapping_add(SFILE_HANDLE_CHANNEL_BYTES)).lw(c);
                c.g[V1][0] = bytes;
                let idle = (bytes == 0) as u64;
                c.g[A0][0] = idle;
                c.g[V0][0] = idle;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `ParticleModifier_BaseSave(self, stream)` (0x345890).
const HALO_BASE_SAVE: Callee = Callee {
    addr: 0x0034_5890,
    f: ee::s11::f_00345890,
};

/// The halo modifier's saved block: 0x30 bytes at +0x10.
pub(crate) const HALO_SAVE_BLOCK: u32 = 0x10;

pub(crate) const HALO_SAVE_BLOCK_SIZE: u64 = 0x30;

/// Stream vtable slot `Write(this, data, size)` (+0x8 this delta, +0xc
/// function).
pub(crate) const HALO_STREAM_WRITE: u32 = 0x8;

/// `void HaloModifier_Save(HaloModifier *self, Stream *stream)` (0x346300):
/// the base particle modifier's save, then the 0x30-byte block at +0x10.
pub fn halo_modifier_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        HALO_BASE_SAVE.call(c, 0x0034_631c)?;
        let stream = r32(c, S0);
        let vt = Addr(stream).lw(c);
        c.g[V1][0] = vt;
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(HALO_SAVE_BLOCK));
        c.g[A2][0] = HALO_SAVE_BLOCK_SIZE;
        let delta = Addr((vt as u32).wrapping_add(HALO_STREAM_WRITE)).lh(c);
        let write = Addr((vt as u32).wrapping_add(HALO_STREAM_WRITE + 4)).lw(c);
        c.g[V0][0] = write;
        c.g[A0][0] = sx(stream.wrapping_add(delta as u32));
        call_ptr(c, write as u32, 0x0034_6338)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA};

    /// The stream header magic `StreamFile_Validate` checks.
    const SFILE_MAGIC: u32 = 0x4d52_5453;

    #[test]
    fn stream_file_get_channel_bytes_matches() {
        check(
            ee::s14::f_003e14c8__generated,
            stream_file_get_channel_bytes,
            300,
            &|c, seed| {
                let mut r = Rng(0x3e14_c800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003e_14c8;
                let (file, header) = (DATA + 0x100, DATA + 0x200);
                fill(c, &mut r, DATA, 0x400 / 4);
                fill(c, &mut r, crate::difftest::SP - 0x40, 0x10);
                c.bus.write32(file, header);
                // file = 0, bad magic, or valid.
                c.bus.write32(
                    header,
                    if seed % 3 == 0 {
                        r.next_u32()
                    } else {
                        SFILE_MAGIC
                    },
                );
                c.g[A0][0] = if seed % 5 == 0 { 0 } else { file as u64 };
            },
        );
    }

    #[test]
    fn trick_lookup_bonus_trick_matches() {
        check(
            ee::s00::f_0011b1a8__generated,
            trick_lookup_bonus_trick,
            300,
            &|c, seed| {
                let mut r = Rng(0x11b1_a800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0011_b1a8;
                fill(c, &mut r, crate::difftest::SP - 0x80, 0x20);
                let id = DATA + 0x40;
                let (w0, w1) = (r.next_u32(), r.next_u32());
                c.bus.write32(id, w0);
                c.bus.write32(id + 4, w1);
                let fields = [
                    (w0 >> 10) & 0x3,
                    (w0 >> 12) & 0xf,
                    (w0 >> 16) & 0x7,
                    (w0 >> 19) & 0x7,
                    w0 >> 28,
                    (w1 >> 3) & 0x7f,
                    (w1 >> 11) & 0x7f,
                ];
                // The matching entry (24: none); the others match a prefix
                // of the fields and differ at the next one.
                let target = seed % 25;
                for i in 0..24u32 {
                    let e = BONUS_TRICKS + i * BONUS_TRICK_SIZE;
                    c.bus.write32(e + BONUS_TRICK_GRADE, r.next_u32());
                    c.bus.write32(e + BONUS_TRICK_POINTS, r.next_u32());
                    let same = if i == target { 7 } else { r.next_u32() % 7 };
                    for (k, &f) in fields.iter().enumerate() {
                        let k = k as u32;
                        let b = if k < same {
                            f
                        } else if k == same {
                            f ^ 1
                        } else {
                            r.next_u32() & 0xff
                        };
                        c.bus.write8(e + 8 + k, b as u8);
                    }
                    c.bus.write8(e + 0xf, r.next_u32() as u8);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = id as u64;
            },
        );
    }

    #[test]
    fn rider_natural_air_sub_enter_matches() {
        check(
            ee::s00::f_0012f620__generated,
            rider_natural_air_sub_enter,
            300,
            &|c, seed| {
                let mut r = Rng(0x12f6_2000 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0012_f620;
                c.g[GP][0] = crate::difftest::GAME_GP as u64;
                fill(c, &mut r, crate::difftest::SP - 0x80, 0x20);
                let gp = crate::difftest::GAME_GP;
                for (off, v) in [
                    (NAT_AIR_GP_DOWN, -std::f32::consts::FRAC_PI_2),
                    (NAT_AIR_GP_UP, std::f32::consts::FRAC_PI_2),
                    (NAT_AIR_GP_PLUS_PI, std::f32::consts::PI),
                    (NAT_AIR_GP_MINUS_PI, std::f32::consts::PI),
                    (NAT_AIR_GP_STEP, 1.0 / 30.0),
                ] {
                    c.bus.write32(gp - off, v.to_bits());
                }
                let this = DATA;
                let rider = DATA + 0x1000;
                fill(c, &mut r, this, 0x8);
                fill(c, &mut r, rider + 0x1e0, 0x40);
                c.bus.write32(this + NAT_AIR_RIDER, rider);
                // Zero (both signs), edge cases, or ordinary velocities.
                let comp = |r: &mut Rng, k: u32| match k % 4 {
                    0 => (r.next_u32() & 1) << 31,
                    1 => r.edge_f32(),
                    _ => r.f32(50.0),
                };
                let vx = comp(&mut r, seed);
                let vy = comp(&mut r, seed / 4);
                c.bus.write32(rider + NAT_AIR_RIDER_VX, vx);
                c.bus.write32(rider + NAT_AIR_RIDER_VY, vy);
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn rider_metrix_complete_trick_matches() {
        check(
            ee::s00::f_001197d8__generated,
            rider_metrix_complete_trick,
            300,
            &|c, seed| {
                let mut r = Rng(0x1197_d800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0011_97d8;
                fill(c, &mut r, crate::difftest::SP - 0x80, 0x20);
                let m = DATA;
                let rider = DATA + 0x400;
                fill(c, &mut r, m, 0x80);
                // Trick history (+0x60..+0x6c): small ids, some empty slots.
                for i in 0..4 {
                    let v = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32() % 50
                    };
                    c.bus.write32(m + 0x60 + 4 * i, v);
                }
                for off in [METRIX_TRICK_SCORE, METRIX_SCORE_TOTAL, METRIX_BEST_SCORE] {
                    let v = if r.next_u32() % 4 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(5000.0)
                    };
                    c.bus.write32(m + off, v);
                }
                c.bus.write32(m + METRIX_RIDER, rider);
                c.bus.write32(
                    rider + METRIX_RIDER_LEVEL,
                    (r.next_u32() % 12).wrapping_sub(2),
                );
                let id = match seed % 5 {
                    0 => r.next_u32() % 19,
                    1 => 19 + r.next_u32() % 16,
                    2 => r.next_u32(),
                    _ => 35 + r.next_u32() % 20,
                };
                c.g[A0][0] = m as u64;
                c.g[A1][0] = sx(id);
            },
        );
    }

    /// Rider_ToggleStance: both stance directions; the real RotateRoots
    /// walks an anim controller with empty layer lists, SinCos is real.
    #[test]
    fn rider_toggle_stance_matches() {
        check(ee::s00::f_00115168__generated, rider_toggle_stance, 200, &|c, seed| {
            let mut r = Rng(0x1151_6800 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0011_5168;
            c.g[GP][0] = crate::difftest::GAME_GP as u64;
            fill(c, &mut r, crate::difftest::SP - 0x100, 0x40);
            let gp = crate::difftest::GAME_GP;
            c.bus
                .write32(gp - STANCE_GP_PI, std::f32::consts::PI.to_bits());
            c.bus.write32(
                gp - STANCE_GP_HALF_ANGLE,
                std::f32::consts::FRAC_PI_2.to_bits(),
            );
            // Root translation and rotation axis (0x4ff130..0x4ff170).
            for i in 0..16 {
                c.bus.write32(STANCE_ROOT_TRANSLATION + 4 * i, r.f32(2.0));
            }
            let rider = DATA;
            let ctrl = DATA + 0x2000;
            let lists = DATA + 0x2400;
            for i in 0..0x100 {
                c.bus.write32(rider + 0x1c0 + 4 * i, r.f32(100.0));
            }
            c.bus.write32(rider + STANCE_FLAG, seed % 2);
            c.bus.write32(rider + STANCE_ANIM, ctrl);
            fill(c, &mut r, ctrl, 0x20);
            c.bus.write32(ctrl + 0x50, lists);
            // Empty layer lists: TransformRoots walks nothing.
            for layer in 0..8 {
                c.bus.write32(lists + 8 * layer, 0);
                c.bus.write32(lists + 8 * layer + 4, 0);
            }
            c.g[A0][0] = rider as u64;
        });
    }

    /// Rider_ReportRiderHit0: the real callees take short paths (no views:
    /// FindViewInRange gives -1 and the collision sound stops; speech volume
    /// 0 stops the speech); the riders' GetRiderIndex methods are fakes
    /// returning rider 1..2 and rival 1..3, and AddAggression runs on
    /// records as in its own test.
    #[test]
    fn rider_report_rider_hit0_matches() {
        use crate::difftest::fakes::{covered, machine, vtable, LOG_RESULTS};
        use crate::game::be::beintaggression::{AGGRESSION_MESSAGES_OFF, AGGRESSION_RECORDS};
        covered(
            ee::s00::f_0010e228__generated,
            rider_report_rider_hit0,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x10_e228, 3);
                c.pc = 0x0010_e228;
                let gp = crate::difftest::GAME_GP;
                let world = DATA + 0x5_0000;
                let audio = DATA + 0x1_0000;
                let agg = DATA + 0x6_0000;
                let (a, b) = (DATA, DATA + 0x1000);
                // Audio: no views (world+0x84 -> +0x84 -> count +0x10 = 0);
                // speech volume (audio + 0x62fc) 0.
                c.bus.write32(gp + 0x410, audio);
                c.bus.write32(audio + 0x62fc, 0);
                c.bus.write32(gp - 0x848, world);
                c.bus.write32(world + 0x84, world + 0x100);
                c.bus.write32(world + 0x100 + 0x84, world + 0x300);
                c.bus.write32(world + 0x300 + 0x10, 0);
                c.bus.write32(world + 0x100 + 0x28, world + 0x200);
                c.bus.write32(world + 0x200, 1 + r.next_u32() % 9);
                // Aggression singleton exists.
                c.bus.write32(gp - 0x1ea4, agg);
                // Controllers: GetRiderIndex (and the sound's position
                // method at +0x28) are fakes.
                for rider in [a, b] {
                    let vt = vtable(c, &mut r, &[0x28, HIT_INDEX_SLOT]);
                    c.bus.write32(rider + HIT_CONTROLLER, vt);
                }
                let rider = 1 + r.next_u32() % 2;
                let rival = 1 + r.next_u32() % 3;
                c.bus.write32(LOG_RESULTS + 4, rider);
                c.bus.write32(LOG_RESULTS + 8, rival);
                // AddAggression's rider slots, character records and
                // aggression records (as in its own test).
                let pick = |r: &mut Rng, v: &[i32]| v[(r.next_u32() as usize) % v.len()];
                for idx in 0..4u32 {
                    c.bus.write32(0x0053_05b0 + 4 * idx, idx);
                    let rec = 0x0053_5b20 + 0x1c * idx;
                    let flag = pick(&mut r, &[-1, 0, 1]) as u32;
                    c.bus.write32(rec + 0xc, flag);
                    c.bus.write32(rec + 0x10, r.next_u32() % 4);
                    c.bus.write8(rec + 0x11, (r.next_u32() % 5) as u8);
                }
                for profile in 0..3u32 {
                    for ch in 0..5u32 {
                        for rv in 0..5u32 {
                            let e = AGGRESSION_RECORDS + profile * 0x9b50 + ch * 0xf88 + rv * 3;
                            c.bus
                                .write8(e + 0xbc1, pick(&mut r, &[-1, 0, 1, 2, 3, 4]) as u8);
                            c.bus
                                .write8(e + 0xbc2, pick(&mut r, &[-1, 0, 1, 2, 3, 4, 5]) as u8);
                            c.bus.write8(
                                e + 0xbc3,
                                pick(&mut r, &[-3, 0, 4, 9, 14, 15, 19, 24, 30]) as u8,
                            );
                        }
                    }
                }
                c.bus
                    .write8(AGGRESSION_MESSAGES_OFF, (r.next_u32() % 2) as u8);
                c.g[A0][0] = a as u64;
                c.g[A1][0] = b as u64;
            },
        );
    }

    /// cFlagManager_Save: the real GroupSave on 15 groups of 0..2 items;
    /// the stream's two writers (vtable +0x8 and +0x28) are fakes, some
    /// unwind.
    #[test]
    fn c_flag_manager_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        covered(ee::s11::f_0034c848__generated, c_flag_manager_save, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x34_c848, 30);
            c.pc = 0x0034_c848;
            let this = DATA;
            let stream = DATA + 0x2000;
            let vt = vtable(c, &mut r, &[0x8, 0x28]);
            c.bus.write32(stream, vt);
            for i in 0..15 {
                let group = this + FLAG_MANAGER_GROUPS + i * FLAG_MANAGER_GROUP_SIZE;
                let count = match r.next_u32() % 4 {
                    0 => 0u32.wrapping_sub(1),
                    1 => 0,
                    _ => 1 + r.next_u32() % 2,
                };
                c.bus.write32(group + FLAG_MANAGER_GROUP_COUNT, count);
                fill(c, &mut r, group + 0x94, 2);
            }
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
        });
    }

    #[test]
    fn stream_file_is_idle_matches() {
        check(ee::s14::f_003e1530__generated, stream_file_is_idle, 300, &|c, seed| {
            let mut r = Rng(0x3e15_3000 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x003e_1530;
            let (file, header) = (DATA + 0x100, DATA + 0x200);
            fill(c, &mut r, DATA, 0x400 / 4);
            fill(c, &mut r, crate::difftest::SP - 0x40, 0x10);
            c.bus.write32(file, header);
            c.bus.write32(
                header,
                if seed % 7 == 0 {
                    r.next_u32()
                } else {
                    SFILE_MAGIC
                },
            );
            if seed % 3 != 0 {
                c.bus.write32(header + SFILE_HEADER_READ_STATE, 0);
            }
            if seed % 2 == 0 {
                c.bus.write32(file + SFILE_HANDLE_CHANNEL_BYTES, 0);
            }
            c.g[A0][0] = if seed % 11 == 0 { 0 } else { file as u64 };
        });
    }

    /// HaloModifier_Save: the real BaseSave; the stream's writers (vtable
    /// +0x8 block write, +0x28 object write) are fakes, some unwind.
    #[test]
    fn halo_modifier_save_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        covered(ee::s11::f_00346300__generated, halo_modifier_save, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x34_6300, 2);
            c.pc = 0x0034_6300;
            let this = DATA;
            let stream = DATA + 0x400;
            fill(c, &mut r, this, 0x20);
            let vt = vtable(c, &mut r, &[HALO_STREAM_WRITE, 0x28]);
            c.bus.write32(stream, vt);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
        });
    }
}
