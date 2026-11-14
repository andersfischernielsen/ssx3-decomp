//! Staging module oc362 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc362`.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0011_a8c8,
    rider_metrix_build_trick_desc,
    "RiderMetrix_BuildTrickDesc",
)];

/// `RiderMetrix` trick-state fields `RiderMetrix_BuildTrickDesc` reads:
/// riding switch +0x00, use the alternate table +0x04, +0x08/+0x0c/+0x10
/// (board-state flags behind the descriptor's 3-bit "k2" field), the trick
/// kind +0x20, its speed +0x24, the rail/air class +0x28 with its amount
/// +0x2c, spin and flip angles in radians +0x34/+0x38, up to three grabs
/// +0x60.., an override trick +0x70 (its low 3 bits), extra bits +0x7c, the
/// rider +0x1ac (whose +0x324 swaps the two kind codes).
pub(crate) const BTD_SWITCH: u32 = 0x00;
pub(crate) const BTD_ALT_TABLE: u32 = 0x04;
pub(crate) const BTD_FLAG_8: u32 = 0x08;
pub(crate) const BTD_FLAG_C: u32 = 0x0c;
pub(crate) const BTD_FLAG_10: u32 = 0x10;
pub(crate) const BTD_KIND: u32 = 0x20;
pub(crate) const BTD_SPEED: u32 = 0x24;
pub(crate) const BTD_CLASS: u32 = 0x28;
pub(crate) const BTD_AMOUNT: u32 = 0x2c;
pub(crate) const BTD_SPIN: u32 = 0x34;
pub(crate) const BTD_FLIP: u32 = 0x38;
pub(crate) const BTD_GRABS: u32 = 0x60;
pub(crate) const BTD_OVERRIDE: u32 = 0x70;
pub(crate) const BTD_EXTRA: u32 = 0x7c;
pub(crate) const BTD_RIDER: u32 = 0x1ac;
/// The rider's "mirrored kinds" flag.
pub(crate) const BTD_RIDER_SWAP: u32 = 0x324;
/// `.lit4` radians -> degrees for the spin and the flip angle.
pub(crate) const BTD_SPIN_DEG_LIT: u32 = 0x79e4;
pub(crate) const BTD_FLIP_DEG_LIT: u32 = 0x79e0;
/// The two 29 x 11 byte trick tables (by spin and flip index): the normal
/// one and the alternate one.
pub(crate) const BTD_TABLE: u32 = 0x0043_d388;
pub(crate) const BTD_TABLE_ALT: u32 = 0x0043_d4c8;
/// `MEM_fill(dst, byte, n)` (0x3e6448), through the function table.
const BTD_MEM_FILL: u32 = 0x003e_6448;

#[inline]
fn btd_lw(c: &mut Cpu, base: u32, off: u32) -> u64 {
    Addr(base.wrapping_add(off)).lw(c)
}

#[inline]
fn btd_sw(c: &mut Cpu, base: u32, off: u32, reg: usize) {
    let v = c.g[reg][0];
    Addr(base.wrapping_add(off)).sw(c, v);
}

/// The inlined `func_roundDegrees((int)(angle * 57.29578f))`: the angle at
/// `field` in whole degrees, rounded to the nearest multiple of 180, into
/// `out`. `raw` receives the truncated degrees, `rem` the remainder mod 180
/// (the registers the original uses for this angle).
fn btd_round_degrees(c: &mut Cpu, field: u32, lit: u32, raw: usize, rem: usize, out: usize) {
    c.cycles += 5;
    let scale = Addr(r32(c, GP).wrapping_sub(lit)).read_fpu(c);
    c.g[V1][0] = 0xb4;
    let angle = Addr(r32(c, S1).wrapping_add(field)).read_fpu(c);
    c.cycles += 8;
    let degrees = angle * scale;
    abi::set_fpr(c, 0, degrees);
    let whole = degrees.to_i32() as u32;
    c.f[1] = whole;
    c.g[raw][0] = sx(whole);
    let (_, r) = abi::div(c, whole, 0xb4);
    c.g[rem][0] = sx(r);
    let base = sx(whole.wrapping_sub(r));
    let r = r as i32;
    if r < -90 {
        c.cycles += 2;
        c.g[V0][0] = base;
        c.g[out][0] = sx((base as u32).wrapping_sub(0xb4));
    } else {
        c.cycles += 3;
        c.g[out][0] = base;
        if r < 91 {
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = base;
            c.g[out][0] = sx((base as u32).wrapping_add(0xb4));
        }
    }
}

/// The air-trick grade from the amount at +0x2c (>= 3): thresholds 12, 9
/// and 6 give `top`, `mid`, `low` or `base` (the C++'s 11/9/7/5 for the
/// class that matches the single spin, 12/10/8/6 otherwise).
fn btd_amount_grade(c: &mut Cpu, grades: [u64; 4], last_cycles: u64) {
    let amount = crate::ps2f::FpuF32(c.f[1]);
    for (limit, grade) in [(0x4140_0000u32, grades[0]), (0x4110_0000, grades[1])] {
        c.cycles += 6;
        c.g[AT][0] = limit as u64;
        c.f[0] = limit;
        let below = amount.lt(crate::ps2f::FpuF32(limit));
        abi::set_fcc(c, below);
        c.g[A0][0] = grade;
        if !below {
            return;
        }
    }
    c.cycles += 6;
    c.g[AT][0] = 0x40c0_0000;
    c.f[0] = 0x40c0_0000;
    let at_least = crate::ps2f::FpuF32(0x40c0_0000).le(amount);
    abi::set_fcc(c, at_least);
    c.g[A0][0] = grades[2];
    if !at_least {
        return;
    }
    c.cycles += last_cycles;
    c.g[A0][0] = grades[3];
}

/// `int RiderMetrix_BuildTrickDesc(TrickState *self, sTrickId *id, int
/// stance, int alternate, int style, int flag)` (0x11a8c8): clear the 8-byte
/// trick descriptor and pack the trick the rider just did into it. An
/// override trick (+0x70) is copied as is; rail/air classes (+0x28) and
/// special kinds (+0x20) get a grade from the spin count or the amount;
/// otherwise the spin and flip angles, rounded to half and whole turns,
/// pick an entry of the 29 x 11 trick table (out of range: the "unknown
/// trick" descriptor), and the spin direction, flip count and direction,
/// grabs, style/flag quality and extra bits are filled in. Returns whether
/// anything was done (1 on the early paths).
pub fn rider_metrix_build_trick_desc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S5, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[T1][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[T0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x8;
        call_ptr(c, BTD_MEM_FILL, 0x0011_a910)?;
        let me = r32(c, S1);
        let id = r32(c, S0);
        c.g[V0][0] = btd_lw(c, me, BTD_OVERRIDE);
        c.g[A1][0] = 0xffff_ffff_ffff_fff8;
        'done: {
            if c.g[V0][0] != 0 {
                // Override trick: kind and trick from +0x0c / +0x70.
                c.cycles += 13;
                c.g[V0][0] = btd_lw(c, me, BTD_FLAG_C);
                c.g[V1][0] = btd_lw(c, id, 0);
                c.g[V0][0] &= 0x7;
                c.g[A0][0] = btd_lw(c, id, 4);
                c.g[V1][0] = (c.g[V1][0] & c.g[A1][0]) | c.g[V0][0];
                c.g[A0][0] &= c.g[A1][0];
                btd_sw(c, id, 0, V1);
                c.g[V0][0] = btd_lw(c, me, BTD_OVERRIDE) & 0x7;
                c.g[A0][0] |= c.g[V0][0];
                btd_sw(c, id, 4, A0);
                if c.g[S4][0] != 0 {
                    c.cycles += 7;
                    c.g[V1][0] = 0x300_0000;
                    c.g[V0][0] = (c.g[A0][0] & 0xffff_ffff_f8ff_ffff) | 0x300_0000;
                    btd_sw(c, id, 4, V0);
                }
                c.cycles += 2;
                c.g[V0][0] = 1;
                break 'done;
            }

            // Riding switch mirrors the spin, kind 4 the flip.
            c.cycles += 3;
            c.g[V0][0] = btd_lw(c, me, BTD_SWITCH);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                let a = Addr(me.wrapping_add(BTD_SPIN));
                let v = -a.read_fpu(c);
                abi::set_fpr(c, 0, v);
                a.write_fpu(c, v);
            }
            c.g[V1][0] = btd_lw(c, me, BTD_KIND);
            c.cycles += 3;
            c.g[V0][0] = 0x4;
            if c.g[V1][0] == 0x4 {
                c.cycles += 3;
                let a = Addr(me.wrapping_add(BTD_FLIP));
                let v = -a.read_fpu(c);
                abi::set_fpr(c, 0, v);
                a.write_fpu(c, v);
            }
            // Spin to half turns (t2), flip to whole turns (t0).
            btd_round_degrees(c, BTD_SPIN, BTD_SPIN_DEG_LIT, A0, A1, T2);
            btd_round_degrees(c, BTD_FLIP, BTD_FLIP_DEG_LIT, A1, A0, T0);
            c.cycles += 5;
            c.g[V0][0] = 0x168;
            c.g[V1][0] = 0xb4;
            let flip_deg = r32(c, T0);
            let (flip_turns, _) = abi::div(c, flip_deg, 0x168);
            c.cycles += 5;
            c.g[A2][0] = btd_lw(c, me, BTD_CLASS);
            c.g[A0][0] = sx(flip_turns);
            let spin_deg = r32(c, T2);
            let (spin_halves, _) = abi::div(c, spin_deg, 0xb4);
            // flips (t1) and spins (a3): absolute counts.
            c.g[T1][0] = c.g[A0][0];
            if (flip_turns as i32) < 0 {
                c.cycles += 1;
                c.g[T1][0] = sx(flip_turns.wrapping_neg());
            }
            c.cycles += 3;
            c.g[A1][0] = sx(spin_halves);
            c.g[A3][0] = c.g[A1][0];
            if (spin_halves as i32) < 0 {
                c.cycles += 1;
                c.g[A3][0] = sx(spin_halves.wrapping_neg());
            }
            c.cycles += 2;
            let spins = c.g[A3][0] as i64;
            c.g[V0][0] = (spins < 2) as u64;

            if c.g[A2][0] != 0 {
                // Rail/air class: a grade from the spin count or the amount.
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                let graded = if spins >= 2 {
                    c.cycles += 3;
                    c.g[V0][0] = (spins < 12) as u64;
                    c.g[V1][0] = (spins < 6) as u64;
                    if spins < 12 {
                        c.cycles += 4;
                        c.g[A0][0] = 0xd;
                        c.g[V0][0] = 0xe;
                        if c.g[V1][0] == 0 {
                            c.g[A0][0] = 0xe;
                        }
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = 0xf;
                    }
                    true
                } else {
                    c.cycles += 2;
                    let mut big = false;
                    if spins == 1 {
                        c.cycles += 7;
                        let amount = Addr(me.wrapping_add(BTD_AMOUNT)).read_fpu(c);
                        abi::set_fpr(c, 1, amount);
                        c.g[AT][0] = 0x4040_0000;
                        c.f[0] = 0x4040_0000;
                        big = crate::ps2f::FpuF32(0x4040_0000).le(amount);
                        abi::set_fcc(c, big);
                    }
                    if big {
                        c.cycles += 2;
                        if c.g[A2][0] != c.g[A3][0] {
                            btd_amount_grade(c, [0xc, 0xa, 0x6, 0x8], 1);
                        } else {
                            btd_amount_grade(c, [0xb, 0x9, 0x5, 0x7], 2);
                        }
                    }
                    big
                };
                if graded {
                    c.cycles += 8;
                    c.g[V0][0] = btd_lw(c, id, 4);
                    c.g[V1][0] = 0xffff_ffff_ff0f_ffff;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 20);
                    c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | c.g[A0][0];
                    btd_sw(c, id, 4, V0);
                } else {
                    // Small spin: kind by class and +0x20, spin count.
                    c.cycles += 2;
                    if c.g[A2][0] != 1 {
                        c.g[A2][0] = btd_lw(c, me, BTD_KIND);
                        c.cycles += 8;
                        c.g[A1][0] = 0x4;
                        c.g[A0][0] = btd_lw(c, id, 4);
                        c.g[V1][0] = 0xffff_ffff_ff0f_ffff;
                        c.g[V0][0] = if c.g[A2][0] != 0 { 4 } else { 2 };
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
                        c.g[A0][0] &= c.g[V1][0];
                    } else {
                        c.cycles += 10;
                        c.g[A1][0] = btd_lw(c, me, BTD_KIND);
                        c.g[A0][0] = btd_lw(c, id, 4);
                        c.g[V1][0] = 0xffff_ffff_ff0f_ffff;
                        c.g[V0][0] = if c.g[A1][0] == 0 { c.g[A2][0] } else { 3 };
                        c.g[V0][0] &= 0xf;
                        c.g[A0][0] &= c.g[V1][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 20);
                    }
                    c.cycles += 13;
                    c.g[A0][0] |= c.g[V0][0];
                    btd_sw(c, id, 4, A0);
                    c.g[A0][0] = (spins < 11) as u64;
                    c.g[V0][0] = if c.g[A0][0] != 0 { c.g[A3][0] } else { 0xa };
                    c.g[V1][0] = btd_lw(c, id, 0);
                    c.g[V0][0] &= 0xf;
                    c.g[A0][0] = 0xffff_ffff_ffff_ff87;
                    c.g[V1][0] &= c.g[A0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[V1][0] |= c.g[V0][0];
                    btd_sw(c, id, 0, V1);
                }
                c.cycles += 2;
                c.g[V0][0] = 1;
                break 'done;
            }

            c.cycles += 3;
            c.g[A1][0] = btd_lw(c, me, BTD_KIND);
            c.g[A2][0] = c.g[A3][0];
            if c.g[A1][0] != 0 {
                // Special kind: slow ones are kind 5, else the kind (swapped
                // 1 <-> 2 for a mirrored rider) and the spin count.
                c.cycles += 7;
                let speed = Addr(me.wrapping_add(BTD_SPEED)).read_fpu(c);
                abi::set_fpr(c, 1, speed);
                c.g[AT][0] = 0x4248_0000;
                c.f[0] = 0x4248_0000;
                let slow = speed.le(crate::ps2f::FpuF32(0x4248_0000));
                abi::set_fcc(c, slow);
                c.g[V1][0] = 0xffff_ffff_ffff_fff8;
                if slow {
                    c.cycles += 4;
                    c.g[V0][0] = (btd_lw(c, id, 0) & c.g[V1][0]) | 0x5;
                } else {
                    c.cycles += 18;
                    c.g[A0][0] = btd_lw(c, id, 0);
                    c.g[V1][0] = (spins < 11) as u64;
                    c.g[A2][0] = 0xffff_ffff_ffff_fff8;
                    c.g[V0][0] = if c.g[V1][0] != 0 { c.g[A3][0] } else { 0xa };
                    c.g[A1][0] &= 0x7;
                    c.g[A0][0] &= c.g[A2][0];
                    c.g[V1][0] = 0xffff_ffff_ffff_ff87;
                    c.g[V0][0] &= 0xf;
                    c.g[A0][0] |= c.g[A1][0];
                    c.g[A0][0] &= c.g[V1][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[A0][0] |= c.g[V0][0];
                    btd_sw(c, id, 0, A0);
                    let rider = btd_lw(c, me, BTD_RIDER) as u32;
                    c.g[V0][0] = sx(rider);
                    c.g[V1][0] = btd_lw(c, rider, BTD_RIDER_SWAP);
                    c.g[V0][0] = 1;
                    if c.g[V1][0] == 0 {
                        c.cycles += 2;
                        c.g[V0][0] = 1;
                        break 'done;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = btd_lw(c, me, BTD_KIND);
                    if c.g[V1][0] == 1 {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[A0][0] & c.g[A2][0]) | 0x2;
                    } else {
                        c.g[V0][0] = 0x2;
                        c.cycles += 2;
                        if c.g[V1][0] != 2 {
                            c.g[V0][0] = 1;
                            break 'done;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = (c.g[A0][0] & c.g[A2][0]) | 0x1;
                    }
                }
                c.cycles += 1;
                btd_sw(c, id, 0, V0);
                c.cycles += 2;
                c.g[V0][0] = 1;
                break 'done;
            }

            // A plain trick. cnt (t3): how many flips came with three
            // half spins each.
            c.cycles += 5;
            c.g[A0][0] = c.g[T1][0];
            c.g[T3][0] = 0;
            c.g[V1][0] = btd_lw(c, me, BTD_GRABS);
            c.g[A1][0] = btd_lw(c, me, BTD_FLAG_10);
            loop {
                c.cycles += 2;
                c.g[V0][0] = ((c.g[A3][0] as i64) < 3) as u64;
                if (c.g[T1][0] as i64) <= 0 {
                    break;
                }
                c.cycles += 2;
                let few_spins = c.g[V0][0] != 0;
                c.g[T1][0] = sx(r32(c, T1).wrapping_sub(1));
                if few_spins {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0011_ace8);
                }
                c.cycles += 2;
                c.g[A3][0] = sx(r32(c, A3).wrapping_sub(3));
                c.g[T3][0] = sx(r32(c, T3).wrapping_add(1));
            }
            // Table indices: spin idx (a2), flip idx (a0); restore the
            // counts.
            c.cycles += 3;
            c.g[T1][0] = c.g[A0][0];
            c.g[A3][0] = c.g[A2][0];
            let spin_right = (c.g[T2][0] as i64) > 0;
            if !spin_right {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A3][0] as i64) < 15) as u64;
            }
            if spin_right || c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[A2][0] = sx(r32(c, A3).wrapping_add(0xe));
            }
            c.cycles += 2;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 6) as u64;
            let flip_back = (c.g[T0][0] as i64) < 0;
            if !flip_back {
                c.cycles += 2;
                c.g[T0][0] = 0;
            }
            if flip_back || c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5));
                c.g[T0][0] = 0;
            }
            // Grab count (t0).
            c.cycles += 2;
            c.g[T4][0] = ((c.g[A2][0] as i64) < 29) as u64;
            if c.g[V1][0] != 0 {
                c.cycles += 3;
                c.g[V1][0] = sx(me.wrapping_add(BTD_GRABS));
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                loop {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[T0][0] as i64) < 3) as u64;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.cycles += 3;
                    let grab = r32(c, V1);
                    c.g[V0][0] = btd_lw(c, grab, 0);
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0011_ad48);
                    }
                }
            }
            // k2 (bits 7..9) from the board-state flags.
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            let k2_bits = if c.g[A1][0] == 1 {
                c.cycles += 5;
                Some(0x280)
            } else {
                c.g[V0][0] = 0x2;
                c.cycles += 2;
                if c.g[A1][0] == 2 {
                    c.cycles += 5;
                    Some(0x300)
                } else {
                    c.g[V0][0] = btd_lw(c, me, BTD_FLAG_C);
                    c.cycles += 2;
                    if c.g[V0][0] != 0 {
                        c.cycles += 3;
                        c.g[V0][0] = btd_lw(c, me, BTD_SWITCH);
                        c.g[V1][0] = 0xffff_ffff_ffff_fc7f;
                        c.cycles += 4;
                        Some(if c.g[V0][0] != 0 { 0x180 } else { 0x100 })
                    } else {
                        c.g[V0][0] = btd_lw(c, me, BTD_FLAG_8);
                        c.cycles += 2;
                        c.g[V1][0] = 0xffff_ffff_ffff_fc7f;
                        if c.g[V0][0] != 0 {
                            c.cycles += 4;
                            Some(0x200)
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = btd_lw(c, me, BTD_SWITCH);
                            if c.g[V0][0] != 0 {
                                c.cycles += 3;
                                Some(0x80)
                            } else {
                                None
                            }
                        }
                    }
                }
            };
            if let Some(bits) = k2_bits {
                c.g[V1][0] = 0xffff_ffff_ffff_fc7f;
                c.g[V0][0] = (btd_lw(c, id, 0) & c.g[V1][0]) | bits;
                c.cycles += 1;
                btd_sw(c, id, 0, V0);
            }

            // The table entry (bits 22..27), or the "unknown trick".
            c.cycles += 2;
            c.g[V0][0] = ((c.g[A0][0] as i64) < 11) as u64;
            let in_range = c.g[T4][0] != 0 && {
                c.cycles += 2;
                c.g[V0][0] != 0
            };
            if !in_range {
                if c.g[T4][0] == 0 {
                    c.cycles += 1;
                }
                c.g[A0][0] = c.g[S0][0];
                c.cycles += 12;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x8;
                call_ptr(c, BTD_MEM_FILL, 0x0011_ae80)?;
                let id = r32(c, S0);
                c.g[V1][0] = btd_lw(c, id, 0);
                c.g[V0][0] = 0xffff_ffff_f03f_ffff;
                c.g[A0][0] = 0x40_0000;
                c.g[V1][0] = (c.g[V1][0] & c.g[V0][0]) | c.g[A0][0];
                c.g[V0][0] = 0x1;
                btd_sw(c, id, 0, V1);
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = btd_lw(c, me, BTD_ALT_TABLE);
            c.g[V1][0] = 0xb;
            let mut table = BTD_TABLE;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[A1][0] = 0x44_0000;
                if c.g[S2][0] != 0 {
                    table = BTD_TABLE_ALT;
                }
            }
            c.cycles += 3;
            c.g[A1][0] = sx(table);
            let spin_idx = r32(c, A2);
            c.g[V0][0] = sx(abi::mult(c, spin_idx, 0xb));
            c.cycles += 12;
            c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[A0][0] = btd_lw(c, id, 0);
            c.g[V0][0] = 0xffff_ffff_f03f_ffff;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(table));
            c.g[A0][0] &= c.g[V0][0];
            c.g[V0][0] = Addr(r32(c, V1)).lbu(c) & 0x3f;
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 22);
            c.g[A0][0] |= c.g[V0][0];
            btd_sw(c, id, 0, A0);

            // No table entry: spin direction (bits 10..11), spin count
            // (12..15) when no flips, flip count (16..18) and direction
            // (19..21).
            c.cycles += 5;
            c.g[V1][0] = btd_lw(c, id, 0);
            c.g[V0][0] = c.g[V1][0] & 0xfc0_0000;
            let mut extra_spins = true;
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                if (c.g[A3][0] as i64) > 0 {
                    c.cycles += 2;
                    c.g[V0][0] = 0xffff_ffff_ffff_f3ff;
                    let dir = if (c.g[T2][0] as i64) > 0 {
                        c.cycles += 3;
                        0x800
                    } else {
                        c.cycles += 2;
                        0x400
                    };
                    c.g[V0][0] = (c.g[V1][0] & c.g[V0][0]) | dir;
                    c.cycles += 1;
                    btd_sw(c, id, 0, V0);
                }
                c.cycles += 2;
                c.g[V0][0] = btd_lw(c, id, 0);
                if c.g[T1][0] == 0 {
                    c.cycles += 8;
                    c.g[V1][0] = 0xffff_ffff_ffff_0fff;
                    c.g[A0][0] = c.g[A3][0] & 0xf;
                    c.g[V0][0] &= c.g[V1][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 12);
                    c.g[V0][0] |= c.g[A0][0];
                    btd_sw(c, id, 0, V0);
                    c.g[V0][0] = btd_lw(c, id, 0);
                }
                c.cycles += 8;
                c.g[V1][0] = 0xffff_ffff_fff8_ffff;
                c.g[A0][0] = c.g[T1][0] & 0x7;
                c.g[A1][0] = c.g[V0][0] & c.g[V1][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
                c.g[A1][0] |= c.g[A0][0];
                btd_sw(c, id, 0, A1);
                if (c.g[T1][0] as i64) <= 0 {
                    extra_spins = false;
                } else {
                    c.cycles += 2;
                    let flip = Addr(me.wrapping_add(BTD_FLIP)).read_fpu(c);
                    abi::set_fpr(c, 1, flip);
                    let all_combined = (c.g[T3][0] as i64) > 0 && {
                        c.cycles += 2;
                        c.g[T3][0] == c.g[T1][0]
                    };
                    c.cycles += 5;
                    c.f[0] = 0;
                    let forward = crate::ps2f::FpuF32::ZERO.lt(flip);
                    abi::set_fcc(c, forward);
                    if all_combined {
                        c.g[V1][0] = 0x20_0000;
                        if forward {
                            c.cycles += 5;
                        } else {
                            c.cycles += 6;
                            c.g[V1][0] = 0x18_0000;
                        }
                        c.g[V0][0] = (c.g[A1][0] & 0xffff_ffff_ffc7_ffff) | c.g[V1][0];
                    } else {
                        c.g[A0][0] = 0x8_0000;
                        c.cycles += 4;
                        c.g[V0][0] = btd_lw(c, id, 0);
                        c.g[V1][0] = 0xffff_ffff_ffc7_ffff;
                        if !forward {
                            c.g[A0][0] = 0x10_0000;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | c.g[A0][0];
                    }
                    c.cycles += 1;
                    btd_sw(c, id, 0, V0);
                }
            }
            // Leftover half spins (bits 28..31) next to the flips.
            if extra_spins {
                c.cycles += 2;
                c.g[V0][0] = sx(r32(c, T1) << 1);
                if (c.g[T1][0] as i64) > 0 {
                    c.cycles += 3;
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T1)));
                    let three_each = c.g[A3][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[A3][0] as i64) < 15) as u64;
                    if !three_each {
                        c.cycles += 2;
                        c.g[V1][0] = 0xfff_0000;
                        if c.g[V0][0] != 0 {
                            c.cycles += 6;
                            c.g[V0][0] = btd_lw(c, id, 0);
                            c.g[V1][0] = 0xfff_ffff;
                            c.g[A0][0] = sx(r32(c, A3) << 28);
                            c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | c.g[A0][0];
                            btd_sw(c, id, 0, V0);
                        }
                    }
                }
            }

            // Quality (bits 24..26 of the second word).
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            let grabs = c.g[T0][0] as i64;
            let mut skip_grab_check = false;
            'quality: {
                if c.g[S3][0] == 1 {
                    c.cycles += 5;
                    c.g[V0][0] = btd_lw(c, id, 4);
                    c.g[V1][0] = 0xffff_ffff_f8ff_ffff;
                    c.g[A0][0] = 0x400_0000;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = 0x500_0000;
                    if c.g[S3][0] == 2 {
                        c.cycles += 4;
                        c.g[V0][0] = btd_lw(c, id, 4);
                        c.g[V1][0] = 0xffff_ffff_f8ff_ffff;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = 0x300_0000;
                        if c.g[S4][0] != 0 {
                            c.cycles += 4;
                            c.g[V0][0] = btd_lw(c, id, 4);
                            c.g[V1][0] = 0xffff_ffff_f8ff_ffff;
                        } else {
                            c.cycles += 5;
                            c.g[V1][0] = btd_lw(c, id, 0);
                            c.g[V0][0] = c.g[V1][0] & 0xfc0_0000;
                            if c.g[V0][0] != 0 {
                                break 'quality;
                            }
                            c.cycles += 2;
                            if c.g[T3][0] != 0 {
                                c.cycles += 4;
                                c.g[V0][0] = c.g[V1][0] & 0xffff_ffff_f000_0000;
                                if c.g[V0][0] == 0 {
                                    break 'quality;
                                }
                            }
                            c.cycles += 2;
                            let mut stance_q = false;
                            if c.g[S5][0] != 0 {
                                c.cycles += 3;
                                c.g[V0][0] = btd_lw(c, me, BTD_SWITCH);
                                c.g[A0][0] = 0x200_0000;
                                if c.g[V0][0] == 0 {
                                    c.cycles += 4;
                                    c.g[V0][0] = btd_lw(c, id, 4);
                                    c.g[V1][0] = 0xffff_ffff_f8ff_ffff;
                                    stance_q = true;
                                }
                            }
                            if !stance_q {
                                c.cycles += 2;
                                if c.g[A3][0] != 0 {
                                    break 'quality;
                                }
                                c.cycles += 2;
                                if c.g[T1][0] != 0 {
                                    break 'quality;
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 0x100_0000;
                                if grabs <= 0 {
                                    skip_grab_check = true;
                                    break 'quality;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = btd_lw(c, id, 4);
                                c.g[V1][0] = 0xffff_ffff_f8ff_ffff;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = (c.g[V0][0] & c.g[V1][0]) | c.g[A0][0];
                btd_sw(c, id, 4, V0);
            }

            // Grabs: the first (bits 3..9), a second one (bit 10, and its
            // id in bits 11..17, or bit 11 for three).
            if !skip_grab_check {
                c.cycles += 2;
                c.g[A0][0] = 0xffff_ffff_ffff_fc07;
                if grabs > 0 {
                    c.cycles += 9;
                    c.g[V0][0] = btd_lw(c, me, BTD_GRABS);
                    c.g[V1][0] = btd_lw(c, id, 4);
                    c.g[A1][0] = (grabs < 2) as u64;
                    c.g[V0][0] &= 0x7f;
                    c.g[V1][0] &= c.g[A0][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                    c.g[V1][0] |= c.g[V0][0];
                    btd_sw(c, id, 4, V1);
                    if c.g[A1][0] == 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[V1][0] | 0x400;
                        c.g[V0][0] = 0x2;
                        btd_sw(c, id, 4, A0);
                        if grabs == 2 {
                            c.cycles += 8;
                            c.g[V1][0] = btd_lw(c, me, BTD_GRABS + 4) & 0x7f;
                            c.g[V0][0] = c.g[A0][0] & 0xffff_ffff_fffc_07ff;
                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 11);
                            c.g[V0][0] |= c.g[V1][0];
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = (c.g[A0][0] & 0xffff_ffff_fffc_07ff) | 0x800;
                        }
                        c.cycles += 1;
                        btd_sw(c, id, 4, V0);
                    }
                }
            }

            // Extra bits (18..19 of the second word).
            c.cycles += 3;
            let extra = btd_lw(c, me, BTD_EXTRA);
            c.g[A0][0] = extra & 0x3;
            if extra != 0 {
                c.cycles += 7;
                c.g[V0][0] = btd_lw(c, id, 4);
                c.g[V1][0] = 0xffff_ffff_fff3_ffff;
                c.g[V0][0] &= c.g[V1][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 18);
                c.g[V0][0] |= c.g[A0][0];
                btd_sw(c, id, 4, V0);
            }

            // Anything at all?
            c.cycles += 2;
            c.g[V1][0] = 0;
            if (c.g[T1][0] as i64) > 0 {
                c.cycles += 1;
                c.g[V1][0] = 1;
            } else {
                c.cycles += 2;
                if (c.g[A3][0] as i64) > 0 {
                    c.g[V1][0] = 1;
                } else {
                    c.cycles += 2;
                    if grabs > 0 {
                        c.g[V1][0] = 1;
                    } else {
                        c.cycles += 3;
                        let extra = btd_lw(c, me, BTD_EXTRA);
                        c.g[V0][0] = c.g[V1][0];
                        if extra == 0 {
                            break 'done;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = 1;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[V1][0];
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_lohi, fill, scramble, Rng, DATA, GAME_GP};

    /// `RiderMetrix_BuildTrickDesc` over its four shapes: an override trick
    /// (+0x70), a rail/air class (+0x28: spin grades, amount grades, small
    /// spins by kind), a special kind (+0x20: slow, mirrored rider) and the
    /// plain trick (table lookup in and out of range, spin/flip fields,
    /// leftover spins, quality, grabs, extra bits, the "nothing done"
    /// return). Angles are built from whole half/full turns plus up to 100
    /// degrees, so all three rounding cases occur.
    #[test]
    fn rider_metrix_build_trick_desc_matches() {
        const ME: u32 = DATA + 0x100;
        const ID: u32 = DATA + 0x800;
        const RIDER: u32 = DATA + 0x1000;
        check_lohi(
            ee::s00::f_0011a8c8__generated,
            rider_metrix_build_trick_desc,
            800,
            &|c, seed| {
                let mut r = Rng(0x11a8_c800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0011_a8c8;
                c.g[GP][0] = sx(GAME_GP);
                fill(c, &mut r, crate::difftest::SP - 0x80, 0x20);
                fill(c, &mut r, ID, 2);
                fill(c, &mut r, ME, 0x80);
                c.bus
                    .write32(GAME_GP.wrapping_sub(BTD_SPIN_DEG_LIT), 0x4265_2ee0);
                c.bus
                    .write32(GAME_GP.wrapping_sub(BTD_FLIP_DEG_LIT), 0x4265_2ee0);
                // Both trick tables; half the entries 0 (no named trick).
                for i in 0..0x280u32 {
                    let b = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32() as u8
                    };
                    c.bus.write8(BTD_TABLE + i, b);
                }
                let mode = seed % 8;
                let some = |r: &mut Rng| {
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32() | 1
                    }
                };
                let pick = |r: &mut Rng, xs: &[u32]| xs[(r.next_u32() as usize) % xs.len()];

                let switch = some(&mut r);
                c.bus.write32(ME + BTD_SWITCH, switch);
                let alt = some(&mut r);
                c.bus.write32(ME + BTD_ALT_TABLE, alt);
                let f8 = some(&mut r);
                c.bus.write32(ME + BTD_FLAG_8, f8);
                let fc = some(&mut r);
                c.bus.write32(ME + BTD_FLAG_C, fc);
                let f10 = pick(&mut r, &[0, 1, 2, 3]);
                c.bus.write32(ME + BTD_FLAG_10, f10);
                let kind = match mode {
                    0 => r.next_u32(),
                    1 | 2 => pick(&mut r, &[0, 1, 4, 7]),
                    3 => pick(&mut r, &[1, 2, 3, 4]),
                    _ => 0,
                };
                c.bus.write32(ME + BTD_KIND, kind);
                let speed = r.f32(100.0);
                c.bus.write32(ME + BTD_SPEED, speed);
                let class = if mode == 1 || mode == 2 {
                    pick(&mut r, &[1, 1, 2, 3])
                } else {
                    0
                };
                c.bus.write32(ME + BTD_CLASS, class);
                let amount = ((r.next_u32() % 32) as f32 * 0.5).to_bits();
                c.bus.write32(ME + BTD_AMOUNT, amount);

                // Spin in half turns, flip in whole turns.
                let halves = if mode == 1 || mode == 2 {
                    pick(&mut r, &[0, 1, 1, 1, 2, 3, 5, 6, 8, 11, 12, 14])
                } else if r.next_u32() % 4 == 0 {
                    0
                } else {
                    r.next_u32() % 34
                };
                let turns = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    r.next_u32() % 13
                };
                let jitter = |r: &mut Rng| (r.next_u32() % 201) as f32 - 100.0;
                let sign = |r: &mut Rng| if r.next_u32() % 2 == 0 { 1.0 } else { -1.0f32 };
                let spin_deg = sign(&mut r) * (halves as f32 * 180.0 + jitter(&mut r));
                let flip_deg = sign(&mut r) * (turns as f32 * 360.0 + jitter(&mut r));
                c.bus
                    .write32(ME + BTD_SPIN, (spin_deg / 57.295_78).to_bits());
                c.bus
                    .write32(ME + BTD_FLIP, (flip_deg / 57.295_78).to_bits());

                // Grabs: the first `n` non-zero, then a 0.
                let n = r.next_u32() % 4;
                for k in 0..3u32 {
                    let v = if k < n {
                        r.next_u32() | 1
                    } else if k == n {
                        0
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(ME + BTD_GRABS + 4 * k, v);
                }
                let over = if mode == 0 { r.next_u32() | 1 } else { 0 };
                c.bus.write32(ME + BTD_OVERRIDE, over);
                let extra = some(&mut r);
                c.bus.write32(ME + BTD_EXTRA, extra);
                c.bus.write32(ME + BTD_RIDER, RIDER);
                let swap = some(&mut r);
                c.bus.write32(RIDER + BTD_RIDER_SWAP, swap);

                c.g[A0][0] = sx(ME);
                c.g[A1][0] = sx(ID);
                c.g[A2][0] = some(&mut r) as u64;
                c.g[A3][0] = some(&mut r) as u64;
                c.g[T0][0] = if r.next_u32() % 4 == 0 {
                    r.next_u32() as u64 | 1
                } else {
                    0
                };
                c.g[T1][0] = pick(&mut r, &[0, 0, 0, 1, 2, 3]) as u64;
            },
        );
    }
}
