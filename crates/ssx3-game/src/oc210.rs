//! Staging wave 201 (oc210): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size),
//! one function per writer. Each reproduces the generated body's registers,
//! cycles, memory and call order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0041_1fd8, muldf3, "__muldf3"),
];

// ---------------------------------------------------------------- __muldf3

const UNPACK_D: Callee = Callee { addr: 0x0041_1c38, f: ee::s15::f_00411c38 };
const MULDI3: Callee = Callee { addr: 0x0041_14d0, f: ee::s15::f_004114d0 };
const PACK_D: Callee = Callee { addr: 0x0041_1b08, f: ee::s15::f_00411b08 };

/// `double __muldf3(double a, double b)` (0x411fd8, libgcc soft-float
/// multiply; doubles travel as raw bits in `a0`/`a1`, the result in `v0`).
///
/// Unpacks both operands into `fp_number_type`s on the stack (a at sp+0,
/// b at sp+0x20) and handles the special classes like fp-bit.c's `_fpmul`:
/// a NaN operand (a checked first) is returned with the product's sign,
/// infinity times zero gives the canonical NaN `__thenan_df` (0x52adc0),
/// infinity or zero times a number returns that operand with the product's
/// sign. Otherwise the 61-bit
/// fractions are multiplied into a 128-bit product with four `__muldi3`
/// calls (32x32 halves), the product is normalised (down while the high
/// word exceeds `u64::MAX >> 3`, carrying the shifted-out bit into the low
/// word only when set; up while it is at most `u64::MAX >> 4`), rounded to
/// even on the 8 guard bits and packed from the temporary at sp+0x40.
pub fn muldf3(c: &mut Cpu) {
    use crate::game::lib::gcc::fp_number as fp;
    use crate::guest::Addr;

    /// The unpacked operands and the result in the 0x100 frame.
    const A: u32 = 0x00;
    const B: u32 = 0x20;
    const TMP: u32 = 0x40;
    /// The arguments are spilled here for `__unpack_d`.
    const ARG_A: u32 = 0x60;
    const ARG_B: u32 = 0x68;
    /// `__thenan_df`: the canonical NaN operand.
    const THE_NAN: u64 = 0x0052_adc0;
    const HIGH_MAX: u64 = u64::MAX >> 3;
    const NORMAL_MIN: u64 = u64::MAX >> 4;
    const TOP_BIT: u64 = 1 << 63;

    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x100);
        c.sd(frame.at(c, ARG_A), c.g[A0][0]);
        c.sd(frame.at(c, ARG_B), c.g[A1][0]);
        c.g[A0][0] = sx(frame.at(c, ARG_A));
        c.sd(frame.at(c, 0xe0), c.g[S7][0]);
        c.g[A1][0] = c.g[SP][0];
        c.sd(frame.at(c, 0x70), c.g[S0][0]);
        frame.save_ra(c, 0xf0);
        c.sd(frame.at(c, 0xd0), c.g[S6][0]);
        c.sd(frame.at(c, 0xc0), c.g[S5][0]);
        c.sd(frame.at(c, 0xb0), c.g[S4][0]);
        c.sd(frame.at(c, 0xa0), c.g[S3][0]);
        c.sd(frame.at(c, 0x90), c.g[S2][0]);
        c.sd(frame.at(c, 0x80), c.g[S1][0]);
        UNPACK_D.call(c, 0x0041_2014)?;
        c.g[S0][0] = sx(frame.at(c, B));
        c.g[A0][0] = sx(frame.at(c, ARG_B));
        c.g[A1][0] = c.g[S0][0];
        UNPACK_D.call(c, 0x0041_2024)?;

        let a = Addr(frame.at(c, A));
        let b = Addr(frame.at(c, B));
        let class_a = a.off(fp::CLASS).lw(c);
        c.g[A0][0] = class_a;
        c.g[S7][0] = sx(frame.at(c, TMP));

        // What gets packed when an operand is special.
        enum Special {
            /// a with sign a ^ b (sp+0x04 rewritten); entered at 0x412088
            /// (loads sign a first).
            A,
            /// The same, entered at 0x412090 (a infinite: sign a in `v0`).
            AInfinite,
            /// b with sign a ^ b (sp+0x24 rewritten).
            B,
            /// Infinity times zero.
            TheNan,
        }
        let special = if class_a < 2 {
            Some(Special::A)
        } else {
            c.cycles += 4;
            let class_b = b.off(fp::CLASS).lw(c);
            c.g[V1][0] = class_b;
            if class_b < 2 {
                Some(Special::B)
            } else {
                c.cycles += 3;
                if class_a == fp::CLASS_INFINITY {
                    c.cycles += 3;
                    c.g[V0][0] = a.off(fp::SIGN).lw(c);
                    if class_b == fp::CLASS_ZERO {
                        Some(Special::TheNan)
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[SP][0];
                        Some(Special::AInfinite)
                    }
                } else {
                    c.cycles += 2;
                    if class_b == fp::CLASS_INFINITY {
                        c.cycles += 2;
                        c.g[V1][0] = b.off(fp::SIGN).lw(c);
                        if class_a != fp::CLASS_ZERO {
                            Some(Special::B)
                        } else {
                            Some(Special::TheNan)
                        }
                    } else {
                        c.cycles += 2;
                        if class_a == fp::CLASS_ZERO {
                            Some(Special::A)
                        } else {
                            c.cycles += 2;
                            c.g[S3][0] = a.off(fp::FRACTION).ld(c);
                            if class_b == fp::CLASS_ZERO {
                                c.cycles += 1;
                                Some(Special::B)
                            } else {
                                c.g[V0][0] = class_b ^ fp::CLASS_ZERO;
                                None
                            }
                        }
                    }
                }
            }
        };

        match special {
            Some(sp_a @ (Special::A | Special::AInfinite)) => {
                if matches!(sp_a, Special::A) {
                    c.cycles += 2;
                    c.g[V0][0] = a.off(fp::SIGN).lw(c);
                    c.g[A0][0] = c.g[SP][0];
                }
                c.cycles += 5;
                let sign_b = b.off(fp::SIGN).lw(c);
                c.g[V1][0] = sign_b;
                let sign = ((c.g[V0][0] ^ sign_b) != 0) as u64;
                c.g[V0][0] = sign;
                a.off(fp::SIGN).sw(c, sign);
            }
            Some(Special::B) => {
                c.cycles += 6;
                let sign_b = b.off(fp::SIGN).lw(c);
                c.g[V1][0] = sign_b;
                c.g[A0][0] = c.g[S0][0];
                let sign = ((a.off(fp::SIGN).lw(c) ^ sign_b) != 0) as u64;
                c.g[V0][0] = sign;
                b.off(fp::SIGN).sw(c, sign);
            }
            Some(Special::TheNan) => {
                c.cycles += 3;
                c.g[V0][0] = 0x0053_0000;
                c.g[A0][0] = THE_NAN;
            }
            None => multiply(c, a, b)?,
        }

        c.cycles += 13;
        PACK_D.call(c, 0x0041_2254)?;
        frame.restore_ra(c, 0xf0);
        c.g[S7][0] = c.ld(frame.at(c, 0xe0));
        c.g[S6][0] = c.ld(frame.at(c, 0xd0));
        c.g[S5][0] = c.ld(frame.at(c, 0xc0));
        c.g[S4][0] = c.ld(frame.at(c, 0xb0));
        c.g[S3][0] = c.ld(frame.at(c, 0xa0));
        c.g[S2][0] = c.ld(frame.at(c, 0x90));
        c.g[S1][0] = c.ld(frame.at(c, 0x80));
        c.g[S0][0] = c.ld(frame.at(c, 0x70));
        frame.close(c);
        Ok(())
    });

    /// Both operands are numbers (class 3): multiply the fractions
    /// (fraction a already in `s3`) into a 128-bit product, normalise,
    /// round, and leave the result in the temporary at sp+0x40 (`a0`, also
    /// `s7`) with class 3.
    fn multiply(c: &mut Cpu, a: Addr, b: Addr) -> Flow {
        let tmp = Addr(c.g[S7][0] as u32);
        c.cycles += 47;
        // 32x32 partial products: s3/s0 = fraction a high/low, s2/s5 = b.
        c.g[S6][0] = 0xffff_ffff;
        c.g[S2][0] = b.off(fp::FRACTION).ld(c);
        c.g[S0][0] = c.g[S3][0] & c.g[S6][0];
        c.g[S5][0] = c.g[S2][0] & c.g[S6][0];
        c.g[S3][0] >>= 32;
        c.g[S2][0] >>= 32;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S5][0];
        MULDI3.call(c, 0x0041_20f0)?;
        c.g[S4][0] = c.g[V0][0]; // lo b * lo a
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        MULDI3.call(c, 0x0041_2100)?;
        c.g[S1][0] = c.g[V0][0]; // hi b * lo a
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S3][0];
        MULDI3.call(c, 0x0041_2110)?;
        c.g[S0][0] = c.g[V0][0]; // lo b * hi a
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S3][0];
        MULDI3.call(c, 0x0041_2120)?;
        let hi_hi = c.g[V0][0];

        let lo_lo = c.g[S4][0];
        let cross = c.g[S1][0].wrapping_add(c.g[S0][0]);
        let cross_carry = (cross < c.g[S1][0]) as u64;
        let mut low = lo_lo.wrapping_add(cross << 32);
        let low_carry = (low < lo_lo) as u64;
        let exp_a = a.off(fp::NORMAL_EXP).lw(c);
        let exp_b = b.off(fp::NORMAL_EXP).lw(c);
        let sign_a = a.off(fp::SIGN).lw(c);
        let sign_b = b.off(fp::SIGN).lw(c);
        let cross_high = ((cross >> 32) & c.g[S6][0]).wrapping_add(hi_hi);
        let mut high = ((cross_carry << 32) | low_carry).wrapping_add(cross_high);
        let sign = ((sign_a ^ sign_b) != 0) as u64;
        let mut exp = sx((exp_a as u32).wrapping_add(exp_b as u32).wrapping_add(4));
        tmp.off(fp::SIGN).sw(c, sign);
        tmp.off(fp::NORMAL_EXP).sw(c, exp);
        c.g[A2][0] = sign_b;
        c.g[A3][0] = exp_b;
        c.g[S0][0] = cross_high;
        c.g[S4][0] = low_carry;
        c.g[V1][0] = sign;
        c.g[V0][0] = (high > HIGH_MAX) as u64;

        // Unreachable with unpacked operands (both fractions have their
        // implicit one at bit 60, so the product's high word is below
        // 2^58), kept for exactness.
        if high > HIGH_MAX {
            // Too big: shift down; the bit falling out of `high` goes into
            // the top of `low` only when it is set (as in fp-bit.c).
            c.cycles += 5;
            c.g[A2][0] = TOP_BIT;
            c.g[V1][0] = HIGH_MAX;
            loop {
                c.cycles += 4;
                exp = sx((exp as u32).wrapping_add(1));
                if high & 1 != 0 {
                    c.cycles += 2;
                    low = (low >> 1) | TOP_BIT;
                }
                c.cycles += 4;
                high >>= 1;
                c.g[V0][0] = high & 1;
                if high <= HIGH_MAX {
                    break;
                }
                if event_due(c) {
                    c.g[S1][0] = high;
                    c.g[A0][0] = low;
                    c.g[A1][0] = exp;
                    deliver_event_here(c, 0x0041_2198);
                }
            }
            c.cycles += 1;
            tmp.off(fp::NORMAL_EXP).sw(c, exp);
        }

        c.cycles += 5;
        c.g[V1][0] = high & 0xff;
        if high <= NORMAL_MIN {
            // Too small: shift up, pulling in the top bit of `low`.
            c.cycles += 6;
            exp = tmp.off(fp::NORMAL_EXP).lw(c);
            c.g[T0][0] = TOP_BIT;
            c.g[A3][0] = 1;
            c.g[A2][0] = NORMAL_MIN;
            loop {
                c.cycles += 8;
                let low_top = low & TOP_BIT;
                high <<= 1;
                exp = sx((exp as u32).wrapping_sub(1));
                if low_top != 0 {
                    high |= 1;
                }
                low <<= 1;
                if high > NORMAL_MIN {
                    break;
                }
                if event_due(c) {
                    c.g[S1][0] = high;
                    c.g[A0][0] = low;
                    c.g[A1][0] = exp;
                    c.g[V1][0] = low_top;
                    c.g[V0][0] = 0;
                    deliver_event_here(c, 0x0041_21f0);
                }
            }
            c.cycles += 2;
            tmp.off(fp::NORMAL_EXP).sw(c, exp);
            c.g[V1][0] = high & 0xff;
        }
        c.g[A1][0] = exp;

        // Round to nearest even on the 8 guard bits: an exact tie rounds up
        // when bit 8 is set, or (when it is clear) when anything is left
        // in `low`.
        c.cycles += 3;
        c.g[V0][0] = 0x80;
        if high & 0xff == 0x80 {
            c.cycles += 3;
            c.g[V0][0] = high.wrapping_add(0x80);
            if high & 0x100 != 0 {
                c.cycles += 2;
                high = high.wrapping_add(0x80);
            } else {
                c.cycles += 1;
                if low != 0 {
                    high = c.g[V0][0];
                }
            }
            c.cycles += 1;
        }
        tmp.off(fp::FRACTION).sd(c, high);
        c.g[S1][0] = high;
        c.g[A0][0] = low;

        c.cycles += 3;
        c.g[V0][0] = fp::CLASS_NUMBER;
        tmp.off(fp::CLASS).sw(c, fp::CLASS_NUMBER);
        c.g[A0][0] = c.g[S7][0];
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------- __muldf3

    /// `__muldf3` against the generated body (real `__unpack_d`,
    /// `__muldi3`, `__pack_d`): every pair of operand classes (signalling
    /// and quiet NaN, zero, infinity, number, with random signs), so each
    /// special result (a, b, the canonical NaN) is taken; numbers with
    /// random and edge exponents (overflow and underflow in `__pack_d`),
    /// and fractions built to hit each rounding case: an exact guard tie
    /// with the result's low bit set, a tie with the low bit clear and
    /// bits left over (rounds up) or none left (stays).
    #[test]
    fn muldf3_matches() {
        use crate::difftest::{check, scramble, Rng};

        const EXP_INF: u64 = 0x7ff;
        const QUIET: u64 = 1 << 51;
        const FRAC: u64 = u64::MAX >> 12;

        fn double(sign: u64, exp: u64, frac: u64) -> u64 {
            (sign << 63) | ((exp & EXP_INF) << 52) | (frac & FRAC)
        }

        check(ee::s15::f_00411fd8__generated, muldf3, 300, &|c, seed| {
            let mut r = Rng(0x0411_fd8d ^ seed as u64);
            scramble(c, &mut r);
            let rnd64 = |r: &mut Rng| (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            let exp = |r: &mut Rng| match r.next_u32() % 6 {
                0 => 1,                              // smallest normal
                1 => 0x7fe,                          // largest
                2 => 0x3ff,                          // 1.x
                _ => 1 + (r.next_u32() as u64 % 0x7fe),
            };
            // class: 0 signalling NaN, 1 quiet NaN, 2 zero, 3 infinity, 4 number
            let operand = |r: &mut Rng, class: u32| {
                let sign = r.next_u32() as u64 & 1;
                let frac = rnd64(r);
                match class {
                    0 => double(sign, EXP_INF, (frac & !QUIET) | 1),
                    1 => double(sign, EXP_INF, frac | QUIET),
                    2 => double(sign, 0, frac), // no denormals: zero
                    3 => double(sign, EXP_INF, 0),
                    _ => {
                        let e = exp(r);
                        double(sign, e, frac)
                    }
                }
            };
            let (a, b) = if seed < 125 {
                let a = operand(&mut r, seed % 5);
                let b = operand(&mut r, (seed / 5) % 5);
                (a, b)
            } else {
                // Two numbers. Fraction a = 2, fraction b = 2^50 + k: the
                // product's 8 guard bits are exactly 0x80; k picks the
                // result's low bit (k odd) and what is left below the guard
                // bits (2k).
                let sign_a = r.next_u32() as u64 & 1;
                let sign_b = r.next_u32() as u64 & 1;
                let (ea, eb) = (exp(&mut r), exp(&mut r));
                match seed % 5 {
                    0 => (double(sign_a, ea, 2), double(sign_b, eb, 1 << 50)), // tie, even, exact
                    1 => (double(sign_a, ea, 2), double(sign_b, eb, (1 << 50) | 1)), // tie, odd
                    2 => (double(sign_a, ea, 2), double(sign_b, eb, (1 << 50) | 2)), // tie, even, rest
                    3 => (double(sign_a, ea, 2), double(sign_b, eb, (1 << 50) | (rnd64(&mut r) & ((1 << 43) - 1)))),
                    _ => (operand(&mut r, 4), operand(&mut r, 4)),
                }
            };
            c.g[A0][0] = a;
            c.g[A1][0] = b;
            c.pc = 0x0041_1fd8;
        });
    }
}
