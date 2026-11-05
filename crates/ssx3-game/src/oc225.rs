//! Staging wave 203 (oc225): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002b_de30,
        twpigd_lighting_is_equal,
        "tWPIGD_Lighting_IsEqual",
    ),
    (
        0x0012_ee30,
        rider_prewind_sub_play_directional_anim,
        "RiderPrewindSub_PlayDirectionalAnim",
    ),
    (
        0x0010_da10,
        computer_actor_rider_should_attack_back,
        "ComputerActorRider_ShouldAttackBack",
    ),
    (
        0x002e_86f0,
        c_board_trail_fx_should_emit,
        "cBoardTrailFX_ShouldEmit",
    ),
    (0x003b_7b48, sndstrm_get_status, "SNDSTRM_GetStatus"),
    (0x0042_47d8, invalid_dcache, "InvalidDCache"),
];

// ---------------------------------------------------------------------------
// tWPIGD_Lighting_IsEqual (0x2bde30)

/// `tWPIGD_Lighting_PairDiffers(a, b)` (0x2bb100): whether two 2-word
/// colour pairs differ.
const LIGHTING_PAIR_DIFFERS: Callee = Callee {
    addr: 0x002b_b100,
    f: ee::s08::f_002bb100,
};

/// `bool tWPIGD_Lighting_IsEqual(const tWPIGD_Lighting* a, const Lighting* b)`
/// (0x2bde30): whether a world-painter lighting record matches a lighting
/// state: the four colour pairs at a+8/+0x18/+0x28/+0x38 against
/// b+4/+0xc/+0x14/+0x1c (through `PairDiffers`), then the floats a+0x48 ==
/// b+0x24 and a+0x50 == b+0x28. Returns 1 if all match, else 0.
pub fn twpigd_lighting_is_equal(c: &mut Cpu) {
    /// (offset in a, offset in b, return address) of each pair compare.
    const PAIRS: [(u32, u32, u32); 4] = [
        (0x08, 0x04, 0x002b_de54),
        (0x18, 0x0c, 0x002b_de68),
        (0x28, 0x14, 0x002b_de7c),
        (0x38, 0x1c, 0x002b_de90),
    ];
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let a = r32(c, A0);
        let b = r32(c, A1);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let mut pairs_equal = true;
        for (i, &(oa, ob, back)) in PAIRS.iter().enumerate() {
            if i > 0 {
                c.cycles += 5;
            }
            c.g[A0][0] = sx(a.wrapping_add(oa));
            c.g[A1][0] = sx(b.wrapping_add(ob));
            LIGHTING_PAIR_DIFFERS.call(c, back)?;
            let differs = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if differs {
                pairs_equal = false;
                break;
            }
        }
        if pairs_equal {
            c.cycles += 6;
            let x = Addr(a.wrapping_add(0x48)).read_fpu(c);
            let y = Addr(b.wrapping_add(0x24)).read_fpu(c);
            c.f[1] = x.0;
            c.f[0] = y.0;
            let eq = x.eq_s(y);
            abi::set_fcc(c, eq);
            if eq {
                c.cycles += 6;
                let x = Addr(a.wrapping_add(0x50)).read_fpu(c);
                let y = Addr(b.wrapping_add(0x28)).read_fpu(c);
                c.f[1] = x.0;
                c.f[0] = y.0;
                let eq = x.eq_s(y);
                abi::set_fcc(c, eq);
                if eq {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                }
            }
        } else {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderPrewindSub_PlayDirectionalAnim (0x12ee30)

/// Rider fields read by the prewind directional animation.
mod prewind {
    /// Stick x / y (floats).
    pub const STICK_X: u32 = 0x2a4;
    pub const STICK_Y: u32 = 0x2b0;
    /// Non-zero: the rider rides switch (x is mirrored).
    pub const SWITCH: u32 = 0x320;
    /// Column of the animation table (stance/variant).
    pub const ANIM_COLUMN: u32 = 0x328;
    /// The rider's animation controller.
    pub const ANIM_CTL: u32 = 0x784;
}

/// `Math_Atan(f12)` (0x31c228).
const MATH_ATAN: u32 = 0x0031_c228;
/// `AnimController_GetLayerAnimId(ctl, layer)` (0x312aa0).
const ANIM_GET_LAYER_ANIM_ID: u32 = 0x0031_2aa0;
/// `AnimController_PlayAnim(ctl, anim, flags, f12 speed)` (0x3128e8).
const ANIM_PLAY_ANIM: u32 = 0x0031_28e8;

/// `void RiderPrewindSub_PlayDirectionalAnim(RiderPrewindSub* self, int (*table)[5])`
/// (0x12ee30): pick the prewind animation from the stick direction and play
/// it on layer 2 if it is not already playing. The stick angle
/// `atan2(-y, ±x)` (x mirrored when riding switch) selects one of nine rows
/// of the table (row 0: stick centred; then by angle sector: back, three
/// left-ish, three right-ish, forward), the rider's column selects the
/// entry. Plays with flags 0 and speed -1.
pub fn rider_prewind_sub_play_directional_anim(c: &mut Cpu) {
    use prewind::*;
    guest(c, |c| {
        let gp = r32(c, GP);
        let lit = |c: &mut Cpu, off: u32| Addr(gp.wrapping_sub(off)).read_fpu(c);
        let zero = FpuF32::ZERO;

        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let sub = Addr(r32(c, A0));
        let table = r32(c, A1);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 23, 0x48);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        let rider = sub.lw(c);
        c.g[V0][0] = rider;
        let rider = Addr(rider as u32);
        let x = rider.off(STICK_X).read_fpu(c);
        let y = rider.off(STICK_Y).read_fpu(c);
        c.f[21] = x.0;
        c.f[12] = y.0;
        let (ax, ay) = (x.abs(), y.abs());
        c.f[1] = ax.0;
        c.f[0] = ay.0;
        let x_major = ay.lt(ax);
        abi::set_fcc(c, x_major);
        // Stick magnitude (max norm) in f23.
        let mag = if x_major {
            c.cycles += 1;
            ax
        } else {
            ay
        };
        c.f[23] = mag.0;

        // The angle into f20. `stage` is where the shared tail is entered:
        // 0: f20 = f12 (atan result), 1: v0 = *self, 2: a1 = 2, 3: done.
        c.cycles += 3;
        let switch = rider.off(SWITCH).lw(c);
        c.g[V0][0] = switch;
        c.f[22] = 0;
        // Some((num >= 0)) when the denominator is zero.
        let mut axis: Option<bool> = None;
        let mut stage = 0;
        let atan = |c: &mut Cpu, num: FpuF32, den: FpuF32, back: u32| -> Flow {
            c.cycles += 7;
            c.f[12] = num.div(den).0;
            call_ptr(c, MATH_ATAN, back)?;
            let den_neg = den.lt(zero);
            abi::set_fcc(c, den_neg);
            c.f[12] = c.f[0];
            Ok(())
        };
        if switch != 0 {
            c.cycles += 6;
            let (num, den) = (-y, -x);
            c.f[20] = den.0;
            let den_zero = den.eq_s(zero);
            abi::set_fcc(c, den_zero);
            c.f[21] = num.0;
            if den_zero {
                c.cycles += 4;
                let num_zero = num.eq_s(zero);
                abi::set_fcc(c, num_zero);
                c.f[20] = num.0;
                if num_zero {
                    stage = 1;
                } else {
                    c.cycles += 3;
                    axis = Some(zero.le(num));
                }
            } else {
                atan(c, num, den, 0x0012_eecc)?;
                if den.lt(zero) {
                    c.cycles += 3;
                    let num_pos = zero.lt(num);
                    abi::set_fcc(c, num_pos);
                    if num_pos {
                        rider_prewind_add_pi(c, gp);
                    } else {
                        c.cycles += 2;
                        rider_prewind_sub_pi(c, gp);
                    }
                }
            }
        } else {
            c.cycles += 5;
            let (num, den) = (-y, x);
            let den_zero = den.eq_s(zero);
            abi::set_fcc(c, den_zero);
            c.f[20] = num.0;
            if den_zero {
                c.cycles += 4;
                let num_zero = num.eq_s(zero);
                abi::set_fcc(c, num_zero);
                if num_zero {
                    c.g[V0][0] = sub.lw(c);
                    stage = 2;
                } else {
                    c.cycles += 1;
                    axis = Some(zero.le(num));
                }
            } else {
                atan(c, num, den, 0x0012_ef3c)?;
                if den.lt(zero) {
                    c.cycles += 3;
                    let num_pos = zero.lt(num);
                    abi::set_fcc(c, num_pos);
                    if num_pos {
                        rider_prewind_add_pi(c, gp);
                    } else {
                        rider_prewind_sub_pi(c, gp);
                    }
                }
            }
        }
        if let Some(num_nonneg) = axis {
            abi::set_fcc(c, num_nonneg);
            // Straight up or down: +-pi/2 from the literal pool.
            c.cycles += 3;
            c.f[20] = lit(c, 0x775c).0;
            c.g[V0][0] = sub.lw(c);
            if num_nonneg {
                c.cycles += 3;
                c.f[20] = lit(c, 0x7758).0;
                c.g[A1][0] = 2;
                stage = 3;
            } else {
                stage = 2;
            }
        }
        if stage <= 0 {
            c.cycles += 1;
            c.f[20] = c.f[12];
        }
        if stage <= 1 {
            c.cycles += 1;
            c.g[V0][0] = sub.lw(c);
        }
        if stage <= 2 {
            c.cycles += 1;
            c.g[A1][0] = 2;
        }

        // The animation playing on layer 2.
        c.cycles += 7;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(ANIM_CTL)).lw(c);
        call_ptr(c, ANIM_GET_LAYER_ANIM_ID, 0x0012_ef80)?;
        c.f[0] = 0;
        let centred = mag.eq_s(zero);
        abi::set_fcc(c, centred);
        c.g[A0][0] = c.g[V0][0];

        // table[row][rider->column] into a1 (v1 = rider, v0 = entry address).
        let lookup = |c: &mut Cpu, reload: bool, row_off: u32| {
            if reload {
                c.g[V1][0] = sub.lw(c);
            }
            let column = Addr((c.g[V1][0] as u32).wrapping_add(ANIM_COLUMN)).lw(c);
            c.g[V0][0] = sx((column as u32) << 2);
            c.g[V0][0] = sx(((column as u32) << 2).wrapping_add(table));
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(row_off)).lw(c);
        };
        let angle = FpuF32(c.f[20]);
        if centred {
            c.cycles += 6;
            lookup(c, true, 0);
        } else {
            c.cycles += 5;
            let back = lit(c, 0x774c);
            c.f[1] = back.0;
            let a = angle.abs();
            c.f[0] = a.0;
            let behind = back.lt(a);
            abi::set_fcc(c, behind);
            if behind {
                c.cycles += 6;
                lookup(c, true, 0x14);
            } else {
                // (literal, angle above it (else below), row offset)
                const SECTORS: [(u32, bool, u32); 5] = [
                    (0x7748, true, 0x28),
                    (0x7744, true, 0x3c),
                    (0x7740, true, 0x50),
                    (0x773c, false, 0x64),
                    (0x7738, false, 0x78),
                ];
                let mut found = false;
                for &(off, above, row) in SECTORS.iter() {
                    c.cycles += 4;
                    let edge = lit(c, off);
                    c.f[0] = edge.0;
                    let hit = if above {
                        edge.lt(angle)
                    } else {
                        angle.lt(edge)
                    };
                    abi::set_fcc(c, hit);
                    if hit {
                        c.cycles += 6;
                        lookup(c, true, row);
                        found = true;
                        break;
                    }
                }
                if !found {
                    c.cycles += 5;
                    let edge = lit(c, 0x7734);
                    c.f[0] = edge.0;
                    let hit = angle.lt(edge);
                    abi::set_fcc(c, hit);
                    c.g[V1][0] = sub.lw(c);
                    if hit {
                        c.cycles += 5;
                        lookup(c, false, 0x8c);
                    } else {
                        c.cycles += 4;
                        lookup(c, false, 0xa0);
                    }
                }
            }
        }

        c.cycles += 2;
        let differs = c.g[A1][0] != c.g[A0][0];
        frame.restore(c, S0, 0x20);
        if differs {
            c.cycles += 6;
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(ANIM_CTL)).lw(c);
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.f[12] = 0xbf80_0000;
            c.g[A2][0] = 0;
            call_ptr(c, ANIM_PLAY_ANIM, 0x0012_f0f4)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 23, 0x48);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// atan2 quadrant fix-up: `f12 += pi` (literal at gp-0x7754).
fn rider_prewind_add_pi(c: &mut Cpu, gp: u32) {
    c.cycles += 3;
    let pi = Addr(gp.wrapping_sub(0x7754)).read_fpu(c);
    c.f[0] = pi.0;
    c.f[12] = (FpuF32(c.f[12]) + pi).0;
}

/// atan2 quadrant fix-up: `f12 -= pi` (literal at gp-0x7750).
fn rider_prewind_sub_pi(c: &mut Cpu, gp: u32) {
    c.cycles += 2;
    let pi = Addr(gp.wrapping_sub(0x7750)).read_fpu(c);
    c.f[0] = pi.0;
    c.f[12] = (FpuF32(c.f[12]) - pi).0;
}

// ---------------------------------------------------------------------------
// ComputerActorRider_ShouldAttackBack (0x10da10)

/// ComputerActorRider fields.
mod attack_back {
    /// The AI rider's own rider (its `+0x6c0` interface, its distance table).
    pub const RIDER: u32 = 0x18;
    /// Seconds left of the current decision (float).
    pub const TIMER: u32 = 0xf30;
    /// The current decision: attack back (1) or not (0).
    pub const DECISION: u32 = 0xf34;
    /// AI manager fields: rider count and rider pointer array.
    pub const AI_COUNT: u32 = 0x78;
    pub const AI_RIDERS: u32 = 0x28;
    /// Per-rider distance table entry stride and distance field.
    pub const DIST_STRIDE: u32 = 0x24;
    pub const DIST: u32 = 0x8;
    /// Rider: the AI interface object (vtable at its start).
    pub const IFACE: u32 = 0x6c0;
    /// Rider: animation controller.
    pub const ANIM_CTL: u32 = 0x784;
}

/// `AI_GetFrameCounter()` (0x1298c8).
const AI_GET_FRAME_COUNTER: Callee = Callee {
    addr: 0x0012_98c8,
    f: ee::s00::f_001298c8,
};
/// `AnimController_GetLayerAnimEntry(ctl, layer)` (0x311ae8).
const ANIM_GET_LAYER_ANIM_ENTRY: u32 = 0x0031_1ae8;
/// `Rand_NextGame()` (0x317810).
const RAND_NEXT_GAME: u32 = 0x0031_7810;

/// `int ComputerActorRider_ShouldAttackBack(ComputerActorRider* self)`
/// (0x10da10): whether an AI rider hits back at a rider that is attacking
/// it. A running decision is kept for its timer (2 s), counted down by one
/// tick (`.lit4` gp-0x7ce4) per call; a negative timer is clamped to 0 and
/// answers no. With no decision, every 6th AI frame scans the riders: the
/// first other rider whose interface reports an attack (vtable slot 8),
/// closer than 200 and playing animation-entry kind 13 on layer 1 makes a
/// new decision: attack with a 25% chance (`Rand_NextGame() % 100 < 25`).
pub fn computer_actor_rider_should_attack_back(c: &mut Cpu) {
    use attack_back::*;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        c.f[0] = 0;
        frame.save(c, S2, 0x30);
        frame.save(c, S0, 0x50);
        let me = Addr(r32(c, A0));
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let timer = me.off(TIMER).read_fpu(c);
        c.f[1] = timer.0;
        let zero = FpuF32::ZERO;
        let negative = timer.lt(zero);
        abi::set_fcc(c, negative);
        // false: fall to the shared "return 0" (v0 = 0).
        let mut answered = false;
        if negative {
            c.cycles += 2;
            me.off(TIMER).write_fpu(c, zero);
        } else {
            c.cycles += 3;
            let running = zero.lt(timer);
            abi::set_fcc(c, running);
            if running {
                c.cycles += 5;
                let tick = Addr(r32(c, GP).wrapping_sub(0x7ce4)).read_fpu(c);
                c.g[V0][0] = me.off(DECISION).lw(c);
                let left = timer - tick;
                c.f[0] = left.0;
                me.off(TIMER).write_fpu(c, left);
                answered = true;
            } else {
                c.cycles += 6;
                AI_GET_FRAME_COUNTER.call(c, 0x0010_da74)?;
                c.g[V1][0] = 6;
                let (_, phase) = abi::div(c, r32(c, V0), 6);
                c.cycles += 3;
                c.g[A0][0] = sx(phase);
                c.g[V0][0] = 0;
                if phase != 0 {
                    answered = true;
                } else {
                    c.cycles += 6;
                    let world = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
                    c.g[V0][0] = world;
                    let ai_root = Addr((world as u32).wrapping_add(0x84)).lw(c);
                    c.g[V1][0] = ai_root;
                    let ai = Addr((ai_root as u32).wrapping_add(0xc)).lw(c);
                    c.g[S3][0] = ai;
                    let ai = Addr(ai as u32);
                    c.g[V0][0] = ai.off(AI_COUNT).lw(c);
                    c.g[S0][0] = 0;
                    if c.g[V0][0] as i64 > 0 {
                        c.cycles += 2;
                        c.g[S4][0] = 100;
                        c.g[A0][0] = me.off(RIDER).lw(c);
                        loop {
                            // Index of the rider this AI rides (slot 7).
                            c.cycles += 8;
                            let iface = (c.g[A0][0] as u32).wrapping_add(IFACE);
                            let vt = Addr(iface).lw(c);
                            c.g[V1][0] = vt;
                            c.g[A0][0] = sx(iface);
                            c.g[A1][0] = Addr((vt as u32).wrapping_add(0x38)).lh(c);
                            c.g[V0][0] = Addr((vt as u32).wrapping_add(0x3c)).lw(c);
                            let f = r32(c, V0);
                            c.g[A0][0] = sx(iface.wrapping_add(r32(c, A1)));
                            call_ptr(c, f, 0x0010_dac8)?;
                            let i = r32(c, S0);
                            let mine = c.g[S0][0] == c.g[V0][0];
                            c.g[V1][0] = sx(i << 2);
                            let mut count_loaded = false;
                            if !mine {
                                // Is rider i attacking (slot 8)?
                                c.cycles += 11;
                                c.g[V0][0] = sx(r32(c, S3).wrapping_add(AI_RIDERS));
                                let slot = r32(c, V0).wrapping_add(i << 2);
                                c.g[S1][0] = sx(slot);
                                let other = Addr(slot).lw(c);
                                let iface = (other as u32).wrapping_add(IFACE);
                                let vt = Addr(iface).lw(c);
                                c.g[V1][0] = vt;
                                c.g[A0][0] = sx(iface);
                                c.g[A1][0] = Addr((vt as u32).wrapping_add(0x40)).lh(c);
                                c.g[V0][0] = Addr((vt as u32).wrapping_add(0x44)).lw(c);
                                let f = r32(c, V0);
                                c.g[A0][0] = sx(iface.wrapping_add(r32(c, A1)));
                                call_ptr(c, f, 0x0010_daf4)?;
                                let attacking = c.g[V0][0] != 0;
                                c.g[V0][0] = 0x24;
                                if attacking {
                                    // Distance to rider i from my rider's table.
                                    c.cycles += 10;
                                    let table = me.off(RIDER).lw(c);
                                    let off = abi::mult(c, 0x24, i);
                                    c.g[V0][0] = sx(off);
                                    c.g[AT][0] = 0x4348_0000;
                                    let limit = FpuF32(0x4348_0000); // 200.0
                                    c.f[1] = limit.0;
                                    let entry = (table as u32).wrapping_add(off);
                                    c.g[V1][0] = sx(entry);
                                    let dist = Addr(entry.wrapping_add(DIST)).read_fpu(c);
                                    c.f[0] = dist.0;
                                    let near = dist.lt(limit);
                                    abi::set_fcc(c, near);
                                    c.g[A1][0] = 1;
                                    if near {
                                        c.cycles += 6;
                                        let other = Addr(r32(c, S1)).lw(c);
                                        c.g[V0][0] = other;
                                        c.g[A0][0] =
                                            Addr((other as u32).wrapping_add(ANIM_CTL)).lw(c);
                                        call_ptr(c, ANIM_GET_LAYER_ANIM_ENTRY, 0x0010_db30)?;
                                        c.g[V1][0] = 0xd;
                                        if c.g[V0][0] != 0xd {
                                            c.g[V0][0] = ai.off(AI_COUNT).lw(c);
                                            count_loaded = true; // (skips the +1 below)
                                        } else {
                                            // New decision: 25% attack, for 2 s.
                                            c.cycles += 6;
                                            call_ptr(c, RAND_NEXT_GAME, 0x0010_db44)?;
                                            let (_, roll) = abi::divu(c, r32(c, V0), 100);
                                            c.g[A0][0] = sx(me.off(DECISION).0);
                                            c.cycles += 3;
                                            c.g[V0][0] = sx(roll);
                                            c.g[V1][0] = sx(roll >> 1);
                                            let roll_f = if (roll as i32) < 0 {
                                                c.cycles += 5;
                                                let v = (roll & 1) | (roll >> 1);
                                                c.g[V0][0] = sx(v);
                                                let h = FpuF32::from_i32(v as i32);
                                                h + h
                                            } else {
                                                c.cycles += 4;
                                                FpuF32::from_i32(roll as i32)
                                            };
                                            c.f[1] = roll_f.0;
                                            c.cycles += 6;
                                            c.g[AT][0] = 0x41c8_0000;
                                            let chance = FpuF32(0x41c8_0000); // 25.0
                                            c.f[0] = chance.0;
                                            let attack = roll_f.lt(chance);
                                            abi::set_fcc(c, attack);
                                            c.g[V0][0] = 1;
                                            if !attack {
                                                c.cycles += 1;
                                                c.g[V0][0] = 0;
                                            }
                                            c.cycles += 6;
                                            me.off(DECISION).sw(c, c.g[V0][0]);
                                            c.g[AT][0] = 0x4000_0000;
                                            c.f[0] = 0x4000_0000; // 2.0
                                            c.g[V0][0] = me.off(DECISION).lw(c);
                                            me.off(TIMER).write_fpu(c, FpuF32(0x4000_0000));
                                            answered = true;
                                            break;
                                        }
                                    }
                                }
                            }
                            if !count_loaded {
                                c.cycles += 1;
                                c.g[V0][0] = ai.off(AI_COUNT).lw(c);
                            }
                            c.cycles += 4;
                            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                            let more = (c.g[S0][0] as i64) < (c.g[V0][0] as i64);
                            c.g[V0][0] = more as u64;
                            if !more {
                                break;
                            }
                            c.g[A0][0] = me.off(RIDER).lw(c);
                            if event_due(c) {
                                deliver_event_here(c, 0x0010_dab0);
                            }
                        }
                    }
                }
            }
        }
        if !answered {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cBoardTrailFX_ShouldEmit (0x2e86f0)

/// cBoardTrailFX fields read by `ShouldEmit`.
mod trail_emit {
    /// The owning rider.
    pub const OWNER: u32 = 0x00;
    /// Rider position at the last emitted point (quadword).
    pub const LAST_POS: u32 = 0xb0;
    /// Trail mode (1: emit every other tick).
    pub const MODE: u32 = 0x94;
    /// Ticks to wait before the next emit check.
    pub const COOLDOWN: u32 = 0xe0;
    /// Rider: world position (quadword).
    pub const RIDER_POS: u32 = 0x460;
    /// Rider flags: on the ground / not grinding / on a rail.
    pub const RIDER_GROUNDED: u32 = 0xac4;
    pub const RIDER_GRINDING: u32 = 0xad0;
    pub const RIDER_RAIL: u32 = 0xafc;
}

/// `bool cBoardTrailFX_ShouldEmit(cBoardTrailFX* self, float speed /* f12 */)`
/// (0x2e86f0): whether the wake trail adds a point this tick. While the
/// cooldown runs it is counted down and the answer is no. Otherwise the
/// rider's distance from the last point (VU0 length) is compared with 110
/// (30 when grounded, not grinding and on a rail): far enough, or mode 1,
/// or `speed` above the `.lit4` threshold at gp-0x39d0 emits and records the
/// rider position (mode 1 also sets the cooldown to 2).
pub fn c_board_trail_fx_should_emit(c: &mut Cpu) {
    use trail_emit::*;
    c.cycles += 4;
    let fx = Addr(r32(c, A0));
    c.g[A1][0] = c.g[A0][0];
    let cooldown = fx.off(COOLDOWN).lw(c);
    c.g[V1][0] = cooldown;
    let frame = Frame::open(c, 0x20);
    if cooldown as i64 > 0 {
        c.cycles += 4;
        c.g[V1][0] = sx((cooldown as u32).wrapping_sub(1));
        c.g[V0][0] = 0;
        fx.off(COOLDOWN).sw(c, c.g[V1][0]);
    } else {
        // Distance from the last point: sqrt(|rider pos - last pos|^2) into
        // f0 (through VU0 Q), threshold 110 into f1.
        c.cycles += 24;
        let rider = fx.off(OWNER).lw(c);
        c.g[V0][0] = rider;
        c.g[A2][0] = 0;
        c.lqc2(3, (rider as u32).wrapping_add(RIDER_POS));
        c.lqc2(4, fx.off(LAST_POS).0);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x10));
        c.g[AT][0] = 0x42dc_0000;
        c.f[1] = 0x42dc_0000; // 110.0
        c.lq(V0, frame.at(c, 0x10));
        let rider = fx.off(OWNER).lw(c);
        c.g[A0][0] = rider;
        c.sq(V0, frame.at(c, 0));
        c.lqc2(3, frame.at(c, 0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V1, 22);
        c.f[0] = c.g[V1][0] as u32;
        let rider = Addr(rider as u32);
        c.g[V0][0] = rider.off(RIDER_GROUNDED).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = rider.off(RIDER_GRINDING).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = rider.off(RIDER_RAIL).lw(c);
                c.g[A2][0] = (0 < c.g[V0][0]) as u64;
            }
        }
        c.cycles += 2;
        let on_rail = c.g[A2][0] != 0;
        c.g[V1][0] = fx.off(MODE).lw(c);
        if on_rail {
            c.cycles += 2;
            c.g[AT][0] = 0x41f0_0000;
            c.f[1] = 0x41f0_0000; // 30.0
        }
        c.cycles += 3;
        c.g[V0][0] = 1;
        let emit = if c.g[V1][0] == 1 {
            // Mode 1: emit and wait two ticks.
            c.g[V0][0] = 2;
            c.cycles += 3;
            fx.off(COOLDOWN).sw(c, 2);
            c.g[A0][0] = fx.off(OWNER).lw(c);
            c.g[V0][0] = 1;
            true
        } else {
            c.cycles += 4;
            let dist = FpuF32(c.f[0]);
            let far = FpuF32(c.f[1]).le(dist);
            abi::set_fcc(c, far);
            if far {
                c.g[A0][0] = fx.off(OWNER).lw(c);
                true
            } else {
                c.cycles += 5;
                let fast = Addr(r32(c, GP).wrapping_sub(0x39d0)).read_fpu(c);
                c.f[0] = fast.0;
                let quick = abi::farg(c, 0).lt(fast);
                abi::set_fcc(c, quick);
                if quick {
                    c.cycles += 2;
                    c.g[A0][0] = fx.off(OWNER).lw(c);
                    true
                } else {
                    c.g[V0][0] = 0;
                    false
                }
            }
        };
        if emit {
            // Remember where the rider is.
            c.cycles += 2;
            c.lq(V1, (c.g[A0][0] as u32).wrapping_add(RIDER_POS));
            c.sq(V1, fx.off(LAST_POS).0);
        }
    }
    c.cycles += 2;
    frame.close(c);
}

// ---------------------------------------------------------------------------
// SNDSTRM_GetStatus (0x3b7b48)

/// Sound stream fields read by `SNDSTRM_GetStatus`.
mod sndstrm {
    /// Stream: its stream file (`StreamFile_GetChannelBytes`).
    pub const FILE: u32 = 0x00;
    /// Stream: buffer slot index (`SNDSTRM_GetBufferedBytes`).
    pub const BUFFER: u32 = 0x08;
    /// Stream: playback rate in bytes per second (u16).
    pub const RATE: u32 = 0x14;
    /// Stream: the current request (list head).
    pub const REQUEST: u32 = 0x11c;
    /// Stream: state (0: idle).
    pub const STATE: u32 = 0x124;
    /// Request: its id, and its total byte count.
    pub const REQ_ID: u32 = 0x0c;
    pub const REQ_BYTES: u32 = 0x10;
    /// Status out: state, request id, buffered milliseconds.
    pub const OUT_STATE: u32 = 0x0;
    pub const OUT_REQUEST: u32 = 0x4;
    pub const OUT_MS: u32 = 0x8;
}

/// `SNDSTRM_GetStream(id)` (0x3b6a38).
const SNDSTRM_GET_STREAM: Callee = Callee {
    addr: 0x003b_6a38,
    f: ee::s13::f_003b6a38,
};
/// `SNDSTRM_GetBufferedBytes(slot)` (0x3c6c98).
const SNDSTRM_GET_BUFFERED_BYTES: Callee = Callee {
    addr: 0x003c_6c98,
    f: ee::s13::f_003c6c98,
};
/// `SNDSTRM_FindRequest(id)` (0x3b7a70).
const SNDSTRM_FIND_REQUEST: Callee = Callee {
    addr: 0x003b_7a70,
    f: ee::s13::f_003b7a70,
};
/// `StreamFile_GetChannelBytes(file)` (0x3e14c8).
const STREAM_FILE_GET_CHANNEL_BYTES: u32 = 0x003e_14c8;

/// `int SNDSTRM_GetStatus(int id, SNDSTRMStatus* out)` (0x3b7b48): fill
/// `out` with stream `id`'s state, current request id and how many
/// milliseconds of audio are buffered (`bytes * 1000 / rate`; when nothing
/// is buffered yet, the request's channel bytes, capped at 0x3d0900, times
/// 1000 over the request's total). Returns -8 for an unknown stream, else 0.
pub fn sndstrm_get_status(c: &mut Cpu) {
    use sndstrm::*;
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        let out = Addr(r32(c, A1));
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        out.off(OUT_MS).sw(c, 0);
        out.off(OUT_REQUEST).sw(c, 0);
        out.off(OUT_STATE).sw(c, 0);
        SNDSTRM_GET_STREAM.call(c, 0x003b_7b74)?;
        c.g[S1][0] = c.g[V0][0];
        let stream = Addr(r32(c, V0));
        let found = c.g[V0][0] != 0;
        c.g[V0][0] = (-8i64) as u64;
        // false: the shared "return 0" (one more cycle).
        let mut done = true;
        if found {
            done = false;
            c.cycles += 3;
            c.g[V0][0] = stream.off(STATE).lw(c);
            out.off(OUT_STATE).sw(c, c.g[V0][0]);
            if c.g[V0][0] != 0 {
                c.cycles += 6;
                let req = stream.off(REQUEST).lw(c);
                c.g[S0][0] = req;
                c.g[V0][0] = Addr((req as u32).wrapping_add(REQ_ID)).lw(c);
                out.off(OUT_REQUEST).sw(c, c.g[V0][0]);
                c.g[V1][0] = stream.off(RATE).lhu(c);
                c.g[V0][0] = 0;
                if c.g[V1][0] == 0 {
                    done = true;
                } else {
                    // Buffered bytes in milliseconds.
                    c.cycles += 7;
                    c.g[A0][0] = stream.off(BUFFER).lw(c);
                    c.g[S3][0] = 1000;
                    SNDSTRM_GET_BUFFERED_BYTES.call(c, 0x003b_7bb0)?;
                    let scaled = abi::mult(c, r32(c, V0), 1000);
                    c.g[V0][0] = sx(scaled);
                    c.g[V1][0] = stream.off(RATE).lhu(c);
                    c.cycles += 4;
                    let (ms, _) = abi::divu(c, scaled, r32(c, V1));
                    c.g[V0][0] = sx(ms);
                    out.off(OUT_MS).sw(c, c.g[V0][0]);
                    if ms == 0 {
                        // Nothing buffered: from the request's file instead.
                        c.cycles += 6;
                        c.g[A0][0] = out.off(OUT_REQUEST).lw(c);
                        SNDSTRM_FIND_REQUEST.call(c, 0x003b_7bd8)?;
                        c.g[S0][0] = c.g[V0][0];
                        let req = Addr(r32(c, V0));
                        let total = req.off(REQ_BYTES).lw(c);
                        c.g[V0][0] = 0;
                        if total == 0 {
                            done = true;
                        } else {
                            c.cycles += 8;
                            c.g[A0][0] = stream.off(FILE).lw(c);
                            call_ptr(c, STREAM_FILE_GET_CHANNEL_BYTES, 0x003b_7bf0)?;
                            const CAP: u64 = 0x003d_0900;
                            c.g[A1][0] = req.off(REQ_BYTES).lw(c);
                            c.cycles += 6;
                            let over = CAP < c.g[V0][0];
                            c.g[A0][0] = over as u64;
                            c.g[V1][0] = if over { CAP } else { c.g[V0][0] };
                            let scaled = abi::mult(c, r32(c, V1), 1000);
                            let (ms, _) = abi::divu(c, scaled, r32(c, A1));
                            c.g[V1][0] = sx(ms);
                            out.off(OUT_MS).sw(c, c.g[V1][0]);
                        }
                    }
                }
            }
        }
        if !done {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// InvalidDCache (0x4247d8)

/// `DIntr()` (0x42c078): disable interrupts, return whether they were on.
const DINTR: Callee = Callee {
    addr: 0x0042_c078,
    f: ee::s15::f_0042c078,
};
/// `_InvalidDCache(start, end)` (0x424730): the cache-line loop.
const INVALID_DCACHE_LINES: Callee = Callee {
    addr: 0x0042_4730,
    f: ee::s15::f_00424730,
};
/// `EIntr()` (0x42c0c0).
const EINTR: u32 = 0x0042_c0c0;

/// `void InvalidDCache(void* start, void* end)` (0x4247d8, libkernel):
/// invalidate the data cache lines covering [start, end] (both rounded down
/// to 64 bytes) with interrupts disabled; re-enables them (tail call to
/// `EIntr`) if they were enabled on entry (Status.EIE, COP0 r12 bit 16).
pub fn invalid_dcache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        let at = frame.at(c, 0x20);
        c.sd(at, c.g[S2][0]);
        let at = frame.at(c, 0x10);
        c.sd(at, c.g[S1][0]);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        let at = frame.at(c, 0);
        c.sd(at, c.g[S0][0]);
        c.mfc0(S0, 12);
        c.g[V0][0] = 0x10000;
        c.g[S0][0] &= 0x10000;
        let enabled = c.g[S0][0] != 0;
        if enabled {
            c.cycles += 2;
            DINTR.call(c, 0x0042_4810)?;
        }
        c.cycles += 7;
        const LINE_MASK: u64 = 0xffff_ffff_ffff_ffc0;
        c.g[A1][0] = c.g[S1][0] & LINE_MASK;
        c.g[A0][0] = c.g[S2][0] & LINE_MASK;
        INVALID_DCACHE_LINES.call(c, 0x0042_4824)?;
        frame.restore_ra(c, 0x30);
        c.cycles += 5;
        c.g[S2][0] = c.ld(frame.at(c, 0x20));
        c.g[S1][0] = c.ld(frame.at(c, 0x10));
        c.g[S0][0] = c.ld(frame.at(c, 0));
        if enabled {
            // Tail call: EIntr returns to our caller.
            c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
            c.pc = EINTR;
            ee::s15::f_0042c0c0(c);
        } else {
            frame.close(c);
        }
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// HaloModifier_DrawQuad (0x2d1d10)

/// Render context (gp+0x2a90) and world (gp-0x854) fields used by the halo.
mod halo {
    /// Render context: current packet write pointer.
    pub const PACKET: u32 = 0xe84;
    /// Render context / world: vtable.
    pub const VTABLE: u32 = 0x10d8;
    /// Render context: per-blend-mode halfwords (`(mode + 0x25) * 4 + 0xf50`).
    pub const MODE_WORDS: u32 = 0xf50;
    /// vtable slots {delta (i16), fn}: world "skip draw" test, quad draw
    /// with UV scale, quad draw.
    pub const SLOT_SKIP: u32 = 0x2d8;
    pub const SLOT_DRAW_SCALED: u32 = 0x278;
    pub const SLOT_DRAW: u32 = 0x270;
    /// The GS register-set template copied into the packet (0x14 bytes).
    pub const TEMPLATE: u32 = 0x0050_1420;
}

/// `void HaloModifier_DrawQuad(const Vec4* pos, const Vec4* colour, int mode,
/// float size /* f12 */, float stretch /* f13 */)` (0x2d1d10): draw one halo
/// sprite unless the world's skip test (vtable slot 0x2d8) says no. Pushes
/// a 0x14-byte register template onto the render packet, patches its
/// fields (alpha/blend setup, the mode's halfword), then draws: a plain
/// unit quad when `stretch` is 0, else a quad stretched by `stretch *
/// k` (k at gp-0x3f98), and for modes 1 and 4 a second, mirrored one
/// (`-stretch * k`). Pops the template again.
pub fn halo_modifier_draw_quad(c: &mut Cpu) {
    use halo::*;
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 28;
        let frame = Frame::open(c, 0x120);
        let world = Addr(gp.wrapping_sub(0x854)).lw(c);
        c.g[A3][0] = world;
        frame.save(c, S0, 0xe0);
        frame.save(c, S2, 0xc0);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0xb0);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 23, 0x108);
        c.g[S3][0] = c.g[A2][0];
        frame.save_fpr(c, 22, 0x100);
        let stretch = abi::farg(c, 1);
        c.f[23] = stretch.0;
        frame.save(c, S1, 0xd0);
        c.f[22] = c.f[12]; // size
        frame.save(c, S4, 0xa0);
        c.g[A1][0] = c.g[S2][0];
        frame.save(c, S5, 0x90);
        frame.save(c, S6, 0x80);
        frame.save_ra(c, 0x70);
        frame.save_fpr(c, 24, 0x110);
        frame.save_fpr(c, 21, 0xf8);
        frame.save_fpr(c, 20, 0xf0);
        // world->ShouldSkipHalo(pos)
        let vt = Addr((world as u32).wrapping_add(VTABLE)).lw(c);
        c.g[V1][0] = vt;
        let delta = Addr((vt as u32).wrapping_add(SLOT_SKIP)).lh(c);
        c.g[A0][0] = delta;
        let f = Addr((vt as u32).wrapping_add(SLOT_SKIP + 4)).lw(c);
        c.g[V0][0] = f;
        c.g[A0][0] = sx((world as u32).wrapping_add(delta as u32));
        call_ptr(c, f as u32, 0x002d_1d78)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0xe0);
        } else {
            halo_push_and_draw(c, frame, gp, stretch)?;
            // Pop the template.
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
            let ctx = r32(c, S1);
            Addr(ctx.wrapping_add(PACKET)).sw(c, c.g[V0][0]);
            frame.restore(c, S0, 0xe0);
        }
        c.cycles += 14;
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore_ra(c, 0x70);
        frame.restore_fpr(c, 24, 0x110);
        frame.restore_fpr(c, 23, 0x108);
        frame.restore_fpr(c, 22, 0x100);
        frame.restore_fpr(c, 21, 0xf8);
        frame.restore_fpr(c, 20, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// The body of `HaloModifier_DrawQuad` after the skip test: push and patch
/// the template, draw. Leaves the packet pointer in v0.
fn halo_push_and_draw(c: &mut Cpu, frame: Frame, gp: u32, stretch: FpuF32) -> Flow {
    use halo::*;
    c.cycles += 98;
    // The colour onto the stack (w, x, y, z order of the loads).
    let colour = Addr(r32(c, S0));
    c.f[3] = colour.off(8).read_fpu(c).0;
    c.g[A0][0] = 0x50_0000;
    c.f[0] = colour.off(0xc).read_fpu(c).0;
    c.g[A1][0] = 0xffff_ffff_ffbf_ffff;
    c.f[1] = colour.read_fpu(c).0;
    c.f[2] = colour.off(4).read_fpu(c).0;
    c.g[T4][0] = 0x40_0000;
    c.swc1(0, frame.at(c, 0));
    c.g[T5][0] = 0xffff_ffff_ffff_fffc;
    c.swc1(1, frame.at(c, 4));
    c.g[A3][0] = 0xffff_ffff_e000_03ff;
    c.swc1(2, frame.at(c, 8));
    c.swc1(3, frame.at(c, 0xc));
    c.g[T6][0] = 0xffff_ffff_ffff_ff83;
    let ctx = Addr(gp.wrapping_add(0x2a90)).lw(c);
    c.g[S1][0] = ctx;
    let ctx = Addr(ctx as u32);
    c.g[T7][0] = 0xffff_ffff_ffff_fc1f;
    c.g[S0][0] = 0x80_0000;
    let packet = ctx.off(PACKET).lw(c);
    c.g[V0][0] = packet;
    c.g[T0][0] = 0xffff_ffff_fe7f_ffff;
    // Duplicate the last 0x14 bytes of the packet.
    let p = Addr(packet as u32);
    c.g[V1][0] = p.read_u64_unaligned(c);
    c.g[A2][0] = p.off(8).read_u64_unaligned(c);
    c.g[T2][0] = p.off(0x10).lw(c);
    p.off(0x14).write_u64_unaligned(c, c.g[V1][0]);
    p.off(0x1c).write_u64_unaligned(c, c.g[A2][0]);
    p.off(0x24).sw(c, c.g[T2][0]);
    c.g[T1][0] = 0xffff_ffff_ffcf_ffff;
    c.g[T3][0] = 0xffff_ffff_fff0_0fff;
    c.g[T2][0] = 0x1_4000;
    // Advance the packet and copy the template over the new entry.
    let packet = ctx.off(PACKET).lw(c);
    c.g[V1][0] = packet;
    let p = Addr(packet as u32);
    c.f[21] = 0;
    c.g[V0][0] = sx(p.0.wrapping_add(0x14));
    let mode_word = ctx.0.wrapping_add(r32(c, S3).wrapping_add(0x25) << 2);
    ctx.off(PACKET).sw(c, c.g[V0][0]);
    c.g[A2][0] = sx(mode_word);
    let template = Addr(TEMPLATE);
    c.g[S4][0] = sx(TEMPLATE);
    c.g[S5][0] = template.read_u64_unaligned(c);
    c.g[S6][0] = template.off(8).read_u64_unaligned(c);
    c.g[V0][0] = template.off(0x10).lw(c);
    p.off(0x14).write_u64_unaligned(c, c.g[S5][0]);
    p.off(0x1c).write_u64_unaligned(c, c.g[S6][0]);
    p.off(0x24).sw(c, c.g[V0][0]);
    let plain = stretch.eq_s(FpuF32::ZERO);
    abi::set_fcc(c, plain);
    // Patch the new entry's words: (pointer register, offset, and, or).
    const EDITS: [(usize, u32, u64, u64); 8] = [
        (A0, 0x4, 0xffff_ffff_ffbf_ffff, 0x40_0000),
        (V1, 0x4, 0xffff_ffff_ffff_fffc, 0),
        (A0, 0x8, 0xffff_ffff_e000_03ff, 0),
        (V1, 0x4, 0xffff_ffff_ffff_ff83, 0x1c),
        (A0, 0x8, 0xffff_ffff_ffff_fc1f, 0x100),
        (A1, 0x4, 0xffff_ffff_fe7f_ffff, 0x80_0000),
        (V1, 0x4, 0xffff_ffff_ffcf_ffff, 0),
        (A0, 0x4, 0xffff_ffff_fff0_0fff, 0x1_4000),
    ];
    for &(reg, off, and, or) in EDITS.iter() {
        let entry = ctx.off(PACKET).lw(c);
        c.g[reg][0] = entry;
        let at = Addr((entry as u32).wrapping_add(off));
        c.g[V0][0] = (at.lw(c) & and) | or;
        at.sw(c, c.g[V0][0]);
    }
    c.g[V1][0] = Addr(mode_word.wrapping_add(MODE_WORDS)).lw(c);
    c.g[V0][0] = ctx.off(PACKET).lw(c);
    Addr(r32(c, V0).wrapping_add(0x10)).sh(c, c.g[V1][0]);

    if plain {
        // A unit quad: size x size, uv (0, 0)-(1, 1).
        c.cycles += 19;
        let vt = ctx.off(VTABLE).lw(c);
        c.g[V1][0] = vt;
        c.g[A1][0] = c.g[S2][0];
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = FpuF32::ONE.0;
        c.g[A2][0] = sx(frame.at(c, 0x10));
        let delta = Addr((vt as u32).wrapping_add(SLOT_DRAW)).lh(c);
        c.g[A3][0] = sx(frame.at(c, 0x20));
        c.swc1(22, frame.at(c, 0x14));
        c.g[T0][0] = sx(frame.at(c, 0x60));
        c.swc1(21, frame.at(c, 0x24));
        c.g[A0][0] = sx(ctx.0.wrapping_add(delta as u32));
        c.swc1(0, frame.at(c, 0x64));
        c.swc1(22, frame.at(c, 0x10));
        c.swc1(21, frame.at(c, 0x20));
        c.swc1(0, frame.at(c, 0x60));
        let f = Addr((vt as u32).wrapping_add(SLOT_DRAW + 4)).lw(c);
        c.g[V0][0] = f;
        c.g[T1][0] = c.g[SP][0];
        call_ptr(c, f as u32, 0x002d_2010)?;
        c.g[V0][0] = ctx.off(PACKET).lw(c);
        return Ok(());
    }
    // Stretched quad, scale stretch * k.
    c.cycles += 25;
    let vt = ctx.off(VTABLE).lw(c);
    c.g[V1][0] = vt;
    c.g[S0][0] = sx(frame.at(c, 0x10));
    c.g[AT][0] = 0x3f80_0000;
    let one = FpuF32::ONE;
    c.f[20] = one.0;
    c.g[A1][0] = c.g[S2][0];
    c.g[A0][0] = Addr((vt as u32).wrapping_add(SLOT_DRAW_SCALED)).lh(c);
    c.g[A2][0] = c.g[S0][0];
    let k = Addr(gp.wrapping_sub(0x3f98)).read_fpu(c);
    c.f[24] = k.0;
    c.g[A3][0] = sx(frame.at(c, 0x20));
    c.swc1(22, frame.at(c, 0x10));
    c.g[A0][0] = sx(ctx.0.wrapping_add(r32(c, A0)));
    c.swc1(22, frame.at(c, 0x14));
    c.f[12] = (stretch * k).0;
    c.swc1(21, frame.at(c, 0x20));
    c.g[T0][0] = sx(frame.at(c, 0x30));
    c.swc1(21, frame.at(c, 0x24));
    c.g[T1][0] = c.g[SP][0];
    c.swc1(20, frame.at(c, 0x30));
    c.swc1(20, frame.at(c, 0x34));
    let f = Addr((vt as u32).wrapping_add(SLOT_DRAW_SCALED + 4)).lw(c);
    c.g[V0][0] = f;
    c.g[T2][0] = 1;
    call_ptr(c, f as u32, 0x002d_1f60)?;
    c.g[V1][0] = 1;
    c.g[V0][0] = 4;
    let mut mirrored = c.g[S3][0] == 1;
    if !mirrored {
        c.cycles += 2;
        mirrored = c.g[S3][0] == 4;
    }
    if mirrored {
        // The mirrored half: scale -stretch * k.
        c.cycles += 21;
        let vt = ctx.off(VTABLE).lw(c);
        c.g[V1][0] = vt;
        let neg = -stretch;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = Addr((vt as u32).wrapping_add(SLOT_DRAW_SCALED)).lh(c);
        c.g[A3][0] = sx(frame.at(c, 0x40));
        c.swc1(22, frame.at(c, 0x14));
        c.f[12] = (neg * k).0;
        c.swc1(21, frame.at(c, 0x44));
        c.g[A0][0] = sx(ctx.0.wrapping_add(r32(c, A0)));
        c.swc1(20, frame.at(c, 0x54));
        c.g[T0][0] = sx(frame.at(c, 0x50));
        c.swc1(22, frame.at(c, 0x10));
        c.g[T1][0] = c.g[SP][0];
        c.swc1(21, frame.at(c, 0x40));
        c.swc1(20, frame.at(c, 0x50));
        let f = Addr((vt as u32).wrapping_add(SLOT_DRAW_SCALED + 4)).lw(c);
        c.g[V0][0] = f;
        c.g[T2][0] = 1;
        call_ptr(c, f as u32, 0x002d_1fc0)?;
    }
    c.g[V0][0] = ctx.off(PACKET).lw(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA};

    /// Lighting compare: random records where each field is either copied
    /// from the other side or left different, so every early-out (pair 1-4,
    /// float 1-2) and the full match are reached.
    #[test]
    fn twpigd_lighting_is_equal_matches() {
        check(
            ee::s08::f_002bde30__generated,
            twpigd_lighting_is_equal,
            300,
            &|c, seed| {
                let mut r = Rng(0xde30 ^ (seed as u64) << 4);
                scramble(c, &mut r);
                c.pc = 0x002b_de30;
                let a = DATA;
                let b = DATA + 0x100;
                fill(c, &mut r, a, 0x18);
                fill(c, &mut r, b, 0x10);
                // First differing field (0..=5), or 6: all equal.
                let first_diff = seed % 7;
                let fields: [(u32, u32, u32); 6] = [
                    (0x08, 0x04, 2),
                    (0x18, 0x0c, 2),
                    (0x28, 0x14, 2),
                    (0x38, 0x1c, 2),
                    (0x48, 0x24, 1),
                    (0x50, 0x28, 1),
                ];
                for (i, &(oa, ob, n)) in fields.iter().enumerate() {
                    if (i as u32) < first_diff {
                        for k in 0..n {
                            let v = c.bus.read32(a + oa + 4 * k);
                            c.bus.write32(b + ob + 4 * k, v);
                        }
                    }
                }
                // Signed zeros compare equal; vary the second word of a pair only.
                if seed % 11 == 3 {
                    c.bus.write32(a + 0x48, 0x8000_0000);
                    c.bus.write32(b + 0x24, 0);
                }
                if seed % 13 == 5 && first_diff < 4 {
                    let ob = fields[first_diff as usize].1;
                    let oa = fields[first_diff as usize].0;
                    let v = c.bus.read32(a + oa);
                    c.bus.write32(b + ob, v);
                }
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | a as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | b as u64;
            },
        );
    }

    /// Prewind animation pick: stick x/y from {+0, -0, random} (both
    /// axes zero, one axis zero, every angle sector), switch on/off, a
    /// random column; the playing animation equals the chosen one (whole
    /// column the same) or not, so PlayAnim is reached. PlayAnim's virtual
    /// call is a fake forced to return 0x207 (its early exit) and sometimes
    /// unwinds.
    #[test]
    fn rider_prewind_sub_play_directional_anim_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        covered(
            ee::s00::f_0012ee30__generated,
            rider_prewind_sub_play_directional_anim,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0xee30, 1);
                c.pc = 0x0012_ee30;
                let sub = DATA;
                let rider = DATA + 0x1000;
                let table = DATA + 0x2000;
                let ctl = DATA + 0x3000;
                let vt = DATA + 0x4000;
                c.bus.write32(sub, rider);
                let pick = |r: &mut Rng, k: u32| match k {
                    0 => 0,
                    1 => 0x8000_0000,
                    _ => r.f32(2.0),
                };
                let x = pick(&mut r, seed % 5);
                let y = pick(&mut r, (seed / 5) % 5);
                c.bus.write32(rider + prewind::STICK_X, x);
                c.bus.write32(rider + prewind::STICK_Y, y);
                let switch = if (seed / 25) % 2 == 0 {
                    0
                } else {
                    r.next_u32() | 1
                };
                c.bus.write32(rider + prewind::SWITCH, switch);
                let column = r.next_u32() % 5;
                c.bus.write32(rider + prewind::ANIM_COLUMN, column);
                c.bus.write32(rider + prewind::ANIM_CTL, ctl);
                fill(c, &mut r, table, 45);
                let playing = r.next_u32() % 0x300;
                if seed % 3 == 0 {
                    for row in 0..9 {
                        c.bus.write32(table + row * 0x14 + column * 4, playing);
                    }
                }
                // Layer 2's animation id (ctl + 2 * 4).
                c.bus.write32(ctl + 8, playing);
                // PlayAnim: vtable at +0x58, its slot {delta at +0x20, fn at +0x24}.
                c.bus.write32(ctl + 0x58, vt);
                c.bus.write16(vt + 0x20, 0x10);
                c.bus.write32(vt + 0x24, FAKE);
                c.bus.write32(LOG_RESULTS, 0x207);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | sub as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | table as u64;
            },
        );
    }

    /// Attack-back decision: timer negative / running / zero; AI frame a
    /// multiple of 6 or not; 0-4 riders, each one mine or not, attacking or
    /// not, nearer than 200 or not, playing entry kind 13 or not (or no
    /// animation, 0x1b6). The interface methods (slots 7 and 8) point at the
    /// real `AnimController_GetLayerAnimId` with delta 4, which returns the
    /// word at iface+0x14: the fixture's answer.
    #[test]
    fn computer_actor_rider_should_attack_back_matches() {
        use crate::difftest::GAME_GP;
        use attack_back::*;
        check(
            ee::s00::f_0010da10__generated,
            computer_actor_rider_should_attack_back,
            300,
            &|c, seed| {
                let mut r = Rng(0xda10 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0010_da10;
                let me = DATA;
                let world = DATA + 0x1000;
                let root = DATA + 0x1100;
                let ai = DATA + 0x1200;
                let my_rider = DATA + 0x2000;
                let vt_self = DATA + 0x9000;
                let vt_other = DATA + 0x9100;
                let getter = 0x0031_2aa0; // returns lw(a0 + 0x10) with delta 4
                let timer = match seed % 4 {
                    0 => r.f32(2.0),
                    1 => 0x8000_0000,
                    _ => 0,
                };
                c.bus.write32(me + TIMER, timer);
                c.bus.write32(me + DECISION, r.next_u32());
                c.bus.write32(me + RIDER, my_rider);
                c.bus.write32(GAME_GP - 0x7ce4, 0x3c88_8889); // 1/60
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, root);
                c.bus.write32(root + 0xc, ai);
                let frame = if seed % 3 == 0 {
                    r.next_u32()
                } else {
                    6 * (r.next_u32() % 1000)
                };
                c.bus.write32(ai + 8, frame);
                let count = r.next_u32() % 5;
                c.bus.write32(ai + AI_COUNT, count);
                c.bus.write32(my_rider + IFACE, vt_self);
                c.bus.write16(vt_self + 0x38, 4);
                c.bus.write32(vt_self + 0x3c, getter);
                c.bus.write32(my_rider + IFACE + 0x14, r.next_u32() % 4);
                c.bus.write16(vt_other + 0x40, 4);
                c.bus.write32(vt_other + 0x44, getter);
                for i in 0..4u32 {
                    let rider = DATA + 0x3000 + i * 0x1000;
                    let ctl = DATA + 0x8000 + i * 0x100;
                    c.bus.write32(ai + AI_RIDERS + 4 * i, rider);
                    c.bus.write32(rider + IFACE, vt_other);
                    let attacking = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    };
                    c.bus.write32(rider + IFACE + 0x14, attacking);
                    c.bus.write32(
                        my_rider + i * DIST_STRIDE + DIST,
                        r.f32(400.0) & 0x7fff_ffff,
                    );
                    c.bus.write32(rider + ANIM_CTL, ctl);
                    // Layer 1's animation id; its table entry's first word.
                    let id = if r.next_u32() % 8 == 0 { 0x1b6 } else { 1 + i };
                    c.bus.write32(ctl + 4, id);
                    let kind = if r.next_u32() % 2 == 0 {
                        13
                    } else {
                        r.next_u32() % 20
                    };
                    c.bus.write32(0x0044_6990 + (1 + i) * 0x1c, kind);
                }
                // Rand_NextGame's state.
                fill(c, &mut r, 0x004f_f030, 6);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | me as u64;
            },
        );
    }

    /// Trail emit check: cooldown running / zero / negative; rider and last
    /// position up to ~170 apart (both thresholds, 30 and 110, on either
    /// side); grounded / grinding / rail flags; mode 1 or not; speed below
    /// or above the threshold literal.
    #[test]
    fn c_board_trail_fx_should_emit_matches() {
        use crate::difftest::{put_vec, GAME_GP};
        use trail_emit::*;
        check(
            ee::s09::f_002e86f0__generated,
            c_board_trail_fx_should_emit,
            300,
            &|c, seed| {
                let mut r = Rng(0x86f0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x002e_86f0;
                let fx = DATA;
                let rider = DATA + 0x1000;
                c.bus.write32(fx + OWNER, rider);
                let cooldown = match seed % 5 {
                    0 => 1 + r.next_u32() % 3,
                    1 => r.next_u32() | 0x8000_0000,
                    _ => 0,
                };
                c.bus.write32(fx + COOLDOWN, cooldown);
                let mode = if r.next_u32() % 3 == 0 {
                    1
                } else {
                    r.next_u32() % 4
                };
                c.bus.write32(fx + MODE, mode);
                let pos = [r.f32(1000.0), r.f32(1000.0), r.f32(1000.0), r.next_u32()];
                put_vec(c, rider + RIDER_POS, pos);
                let spread = [30.0f32, 100.0, 1.0][(seed / 5 % 3) as usize];
                let mut last = [0u32; 4];
                for k in 0..3 {
                    let d = f32::from_bits(r.f32(spread));
                    last[k] = (f32::from_bits(pos[k]) + d).to_bits();
                }
                last[3] = r.next_u32();
                put_vec(c, fx + LAST_POS, last);
                let flag = |r: &mut Rng| {
                    if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        r.next_u32()
                    }
                };
                let g = flag(&mut r);
                c.bus.write32(rider + RIDER_GROUNDED, g);
                let gr = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    r.next_u32()
                };
                c.bus.write32(rider + RIDER_GRINDING, gr);
                let rail = flag(&mut r);
                c.bus.write32(rider + RIDER_RAIL, rail);
                c.bus.write32(GAME_GP - 0x39d0, r.f32(50.0));
                c.f[12] = r.f32(50.0);
                c.g[A0][0] = ((r.next_u32() as u64) << 32) | fx as u64;
            },
        );
    }

    /// Stream status: unknown / negative stream id; state idle or not;
    /// rate 0 or not; buffered bytes zero or not; the request found or not
    /// (negative request id), its total zero or not; the stream file valid
    /// ("MRTS") or not, channel bytes below or above the 0x3d0900 cap.
    #[test]
    fn sndstrm_get_status_matches() {
        use sndstrm::*;
        check(ee::s13::f_003b7b48__generated, sndstrm_get_status, 300, &|c, seed| {
            let mut r = Rng(0x7b48 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x003b_7b48;
            const STREAM_COUNT: u32 = 0x0050_a92e;
            const STREAM_TABLE: u32 = 0x0050_a0a0;
            const BUFFER_TABLE: u32 = 0x0051_57b8 + 0x304;
            let out = DATA;
            let stream = DATA + 0x1000;
            let req = DATA + 0x2000;
            let buf = DATA + 0x3000;
            let file = DATA + 0x4000;
            let header = DATA + 0x5000;
            fill(c, &mut r, out, 4);
            c.bus.write8(STREAM_COUNT, 4);
            let id: u32 = match seed % 10 {
                0 => 4 + r.next_u32() % 100,
                1 => r.next_u32() | 0x8000_0000,
                _ => r.next_u32() % 4,
            };
            for k in 0..4 {
                c.bus.write32(STREAM_TABLE + 4 * k, stream);
            }
            let state = if seed % 7 == 2 {
                0
            } else {
                1 + r.next_u32() % 5
            };
            c.bus.write32(stream + STATE, state);
            c.bus.write32(stream + REQUEST, req);
            // The request id names this stream in its low byte (or is negative).
            let req_id = if seed % 9 == 4 {
                r.next_u32() | 0x8000_0000
            } else {
                (id & 0xff) | (r.next_u32() % 16) << 8
            };
            c.bus.write32(req + REQ_ID, req_id);
            c.bus.write32(req, 0);
            let rate = if seed % 11 == 3 {
                0
            } else {
                1 + r.next_u32() % 48000
            };
            c.bus.write16(stream + RATE, rate as u16);
            let slot = r.next_u32() % 4;
            c.bus.write32(stream + BUFFER, slot);
            c.bus.write32(BUFFER_TABLE + 4 * slot, buf);
            let (b0, b1) = if seed % 2 == 0 {
                (0, 0)
            } else {
                (r.next_u32() % 0x10000, r.next_u32() % 0x10000)
            };
            c.bus.write32(buf + 0x48, b0);
            c.bus.write32(buf + 0x4c, b1);
            let total = if seed % 13 == 6 {
                0
            } else {
                1 + r.next_u32() % 0x80_0000
            };
            c.bus.write32(req + REQ_BYTES, total);
            c.bus.write32(stream + FILE, file);
            c.bus.write32(file, header);
            let magic = if seed % 5 == 1 {
                r.next_u32()
            } else {
                0x4d52_5453
            };
            c.bus.write32(header, magic);
            let channel = if r.next_u32() % 2 == 0 {
                r.next_u32() % 0x40_0000
            } else {
                r.next_u32()
            };
            c.bus.write32(file + 8, channel);
            c.g[A0][0] = sx(id);
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | out as u64;
        });
    }

    /// Data-cache invalidate: interrupts enabled or not on entry (Status
    /// bit 16, with other Status bits random), random ranges.
    #[test]
    fn invalid_dcache_matches() {
        check(ee::s15::f_004247d8__generated, invalid_dcache, 200, &|c, seed| {
            let mut r = Rng(0x47d8 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            c.pc = 0x0042_47d8;
            let status = r.next_u32() & !0x1_0000;
            c.cop0[12] = if seed % 2 == 0 {
                status | 0x1_0000
            } else {
                status
            };
            c.cop0[28] = r.next_u32();
            let start = r.next_u32() % 0x0200_0000;
            let end = start.wrapping_add(r.next_u32() % 0x2000);
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | start as u64;
            c.g[A1][0] = sx(end);
        });
    }
}
