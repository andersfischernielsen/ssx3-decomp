//! `src/seg/seg_2fe810.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0040_e878, ieee754_rem_pio2f, "__ieee754_rem_pio2f")];

// ---------------------------------------------------------------------------
// __ieee754_rem_pio2f (0x40e878)

/// `npio2_hw` (0x499540) and `two_over_pi` (0x499228) of newlib's
/// `e_rem_pio2f.c`.
pub(crate) const REM_PIO2F_NPIO2_HW: u32 = 0x0049_9540;

pub(crate) const REM_PIO2F_TWO_OVER_PI: u32 = 0x0049_9228;

pub(crate) const FABSF: Callee = Callee {
    addr: 0x0040_f8a0,
    f: ee::s15::fabsf,
};

pub(crate) const KERNEL_REM_PIO2F: Callee = Callee {
    addr: 0x0040_ec58,
    f: ee::s15::__kernel_rem_pio2f,
};

/// `f = bits` through AT (`lui/ori at; mtc1 at, f`).
#[inline]
pub(crate) fn rem_pio2f_const(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.f[f] = bits;
}

/// The range test of one case: `cycles`, v0 = `v0` (the next constant the
/// original builds), `ix <= limit`.
pub(crate) fn rem_pio2f_below(c: &mut Cpu, cycles: u64, v0: u64, ix: i64, limit: i64) -> bool {
    c.cycles += cycles;
    c.g[V0][0] = v0;
    ix <= limit
}

/// `x -/+ pio2_1` (and the tail) for |x| < 3pi/4: subtracts for x > 0,
/// else adds. y[0] = f1, y[1] = f0; v0 = n = 1 or -1.
pub(crate) fn rem_pio2f_small(c: &mut Cpu, positive: bool) {
    let op = |c: &mut Cpu, fd: usize, fs: usize, ft: usize| {
        if positive {
            c.sub_s(fd, fs, ft)
        } else {
            c.add_s(fd, fs, ft)
        }
    };
    c.cycles += 9;
    rem_pio2f_const(c, 0, 0x3fc9_0f80); // pio2_1
    c.g[V0][0] = c.g[S0][0] & 0xffff_ffff_ffff_fff0;
    c.g[V1][0] = 0x3fc9_0fd0;
    op(c, 12, 12, 0);
    if c.g[V0][0] != c.g[V1][0] {
        c.cycles += 5;
        rem_pio2f_const(c, 2, 0x3735_4443); // pio2_1t
        op(c, 1, 12, 2);
    } else {
        // Near pi/2: 33+53 bits of pi.
        c.cycles += 8;
        rem_pio2f_const(c, 0, 0x3735_4400); // pio2_2
        rem_pio2f_const(c, 2, 0x2e85_a308); // pio2_2t
        op(c, 12, 12, 0);
        op(c, 1, 12, 2);
    }
    c.cycles += 6;
    let y = c.g[S1][0] as u32;
    c.sub_s(0, 12, 1);
    c.swc1(1, y);
    op(c, 0, 0, 2);
    c.swc1(0, y + 4);
    c.g[V0][0] = if positive { 1 } else { u64::MAX };
}

/// `int __ieee754_rem_pio2f(float x, float* y)` (0x40e878): x reduced by
/// n * pi/2 into y[0] + y[1]; returns n (mod 8 is all callers use). Small
/// cases inline, medium by n = round(|x| * 2/pi) with up to three pi/2
/// parts, huge ones through `__kernel_rem_pio2f`.
pub fn ieee754_rem_pio2f(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        let sp = sp(c);
        c.sd(sp + 0x20, c.g[S1][0]);
        c.sd(sp + 0x40, c.g[RA][0]);
        c.sd(sp + 0x30, c.g[S2][0]);
        c.sd(sp + 0x10, c.g[S0][0]);
        c.g[S2][0] = sx(c.f[12]); // hx
        c.g[V1][0] = 0x7fff_ffff;
        c.g[S0][0] = c.g[S2][0] & 0x7fff_ffff; // ix
        c.g[S1][0] = c.g[A0][0];
        let ix = c.g[S0][0] as i64;
        let y = c.g[S1][0] as u32;
        if ix <= 0x3f49_0fd8 {
            // |x| <= pi/4: y = (x, 0), n = 0.
            c.g[V0][0] = 0;
            c.cycles += 4;
            c.swc1(12, y);
            Addr(y + 4).sw(c, 0);
        } else if rem_pio2f_below(c, 5, 0x4349_0000, ix, 0x4016_cbe3) {
            // |x| < 3pi/4.
            c.cycles += 2;
            c.g[V1][0] = 0x3fc9_0000;
            rem_pio2f_small(c, (c.g[S2][0] as i64) > 0);
        } else if rem_pio2f_below(c, 4, 0x7f7f_0000, ix, 0x4349_0f80) {
            // |x| <= 2^7 * pi/2: medium.
            c.cycles += 25;
            FABSF.call(c, 0x0040_e9dc)?;
            rem_pio2f_const(c, 1, 0x3f22_f984); // invpio2
            c.mov_s(5, 0); // t = |x|
            rem_pio2f_const(c, 2, 0x3f00_0000);
            c.mul_s(1, 5, 1);
            rem_pio2f_const(c, 0, 0x3fc9_0f80); // pio2_1
            rem_pio2f_const(c, 3, 0x3735_4443); // pio2_1t
            c.add_s(1, 1, 2);
            c.cvt_w_s(2, 1);
            c.mfc1(A1, 2); // n
            c.mtc1(A1, 6);
            c.cvt_s_w(6, 6); // fn
            let n = c.g[A1][0];
            c.g[V0][0] = ((n as i64) < 32) as u64;
            c.mul_s(0, 6, 0);
            c.mul_s(3, 6, 3);
            c.sub_s(4, 5, 0); // r = t - fn * pio2_1
            let y = c.g[S1][0] as u32;
            let mut quick = false;
            if (n as i64) < 32 {
                c.cycles += 11;
                let at = REM_PIO2F_NPIO2_HW.wrapping_add(((n as u32).wrapping_sub(1)) << 2);
                c.g[V1][0] = sx(at);
                c.g[A0][0] = Addr(at).lw(c);
                c.g[V0][0] = c.g[S0][0] & 0xffff_ffff_ffff_ff00;
                c.sub_s(0, 4, 3);
                if c.g[V0][0] != c.g[A0][0] {
                    c.cycles += 2;
                    c.swc1(0, y);
                    c.cycles += 1;
                    c.lwc1(1, y);
                    quick = true;
                }
            } else {
                c.cycles += 1;
                c.sub_s(0, 4, 3);
            }
            if !quick {
                c.cycles += 9;
                c.g[A0][0] = sx(((c.g[S0][0] as u32) as i32 >> 23) as u32); // j
                c.mfc1(V0, 0);
                Addr(y).sw(c, c.g[V0][0]);
                let i = rem_pio2f_exponent_gap(c);
                c.g[V0][0] = (i < 9) as u64;
                if i < 9 {
                    c.lwc1(1, y);
                } else {
                    // Second iteration: 2nd 24 bits of pi/2.
                    c.cycles += 22;
                    rem_pio2f_const(c, 0, 0x3735_4400); // pio2_2
                    c.mov_s(5, 4);
                    rem_pio2f_const(c, 1, 0x2e85_a308); // pio2_2t
                    c.mul_s(3, 6, 0);
                    c.mul_s(1, 6, 1);
                    c.sub_s(4, 4, 3);
                    c.sub_s(0, 5, 4);
                    c.sub_s(0, 0, 3);
                    c.sub_s(3, 1, 0);
                    c.sub_s(2, 4, 3);
                    c.mfc1(V0, 2);
                    Addr(y).sw(c, c.g[V0][0]);
                    let i = rem_pio2f_exponent_gap(c);
                    c.g[V0][0] = (i < 26) as u64;
                    if i < 26 {
                        c.lwc1(1, y);
                    } else {
                        // Third iteration.
                        c.cycles += 15;
                        rem_pio2f_const(c, 0, 0x2e85_a300); // pio2_3
                        c.mov_s(5, 4);
                        rem_pio2f_const(c, 2, 0x248d_3132); // pio2_3t
                        c.mul_s(3, 6, 0);
                        c.mul_s(2, 6, 2);
                        c.sub_s(4, 4, 3);
                        c.sub_s(0, 5, 4);
                        c.sub_s(0, 0, 3);
                        c.sub_s(3, 2, 0);
                        c.sub_s(1, 4, 3);
                        c.swc1(1, y);
                        c.cycles += 1;
                        c.lwc1(1, y);
                    }
                }
            }
            // y[1] = (r - y[0]) - w; negate for x < 0.
            c.cycles += 4;
            c.sub_s(0, 4, 1);
            c.sub_s(0, 0, 3);
            c.swc1(0, y + 4);
            if (c.g[S2][0] as i64) >= 0 {
                c.cycles += 1;
                c.g[V0][0] = c.g[A1][0];
            } else {
                c.cycles += 3;
                c.neg_s(1, 1);
                c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
                rem_pio2f_store_negated(c);
            }
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x7f7f_ffff;
            let nan = ix > 0x7f7f_ffff;
            c.g[V0][0] = 0;
            if nan {
                // Inf or NaN: y = (x - x, x - x), n = 0.
                c.cycles += 4;
                c.sub_s(0, 12, 12);
                c.swc1(0, y);
                c.swc1(0, y + 4);
            } else {
                rem_pio2f_large(c)?;
            }
        }
        c.cycles += 6;
        let sp = self::sp(c);
        c.g[RA][0] = c.ld(sp + 0x40);
        c.g[S2][0] = c.ld(sp + 0x30);
        c.g[S1][0] = c.ld(sp + 0x20);
        c.g[S0][0] = c.ld(sp + 0x10);
        c.g[SP][0] = sx(sp.wrapping_add(0x50));
        c.pc = c.g[RA][0] as u32;
        Ok(())
    })
}

/// `i = j - ((y[0] >> 23) & 0xff)` from the word in v0 (a0 = j); v1 = i.
pub(crate) fn rem_pio2f_exponent_gap(c: &mut Cpu) -> i64 {
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 23) & 0xff;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
    c.g[V1][0] as i64
}

/// x < 0: y[0] = -f1 (already negated), y[1] = -f0.
pub(crate) fn rem_pio2f_store_negated(c: &mut Cpu) {
    c.cycles += 4;
    let y = c.g[S1][0] as u32;
    c.neg_s(0, 0);
    c.swc1(1, y);
    c.swc1(0, y + 4);
}

/// |x| > 2^7 * pi/2: split x into three 8-bit-exponent floats on the stack
/// and hand them to `__kernel_rem_pio2f`.
pub(crate) fn rem_pio2f_large(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 10;
    let sp = sp(c);
    c.g[V1][0] = sx(((c.g[S0][0] as u32) as i32 >> 23) as u32);
    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x86)); // e0
    c.g[V0][0] = sx((c.g[A2][0] as u32) << 23);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.mtc1(S0, 12); // z
    rem_pio2f_const(c, 2, 0x4380_0000); // two8
    c.g[V1][0] = 1;
    c.g[A0][0] = sx(sp);
    loop {
        c.cycles += 10;
        c.cvt_w_s(0, 12);
        c.mfc1(V0, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.mtc1(V0, 0);
        c.cvt_s_w(0, 0);
        c.sub_s(1, 12, 0);
        c.swc1(0, c.g[A0][0] as u32);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        c.mul_s(12, 1, 2);
        if (c.g[V1][0] as i64) < 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0040_eb90);
        }
    }
    // tx[2] = z; skip trailing zeros (nx).
    c.cycles += 6;
    c.f[1] = 0;
    c.g[A3][0] = 3;
    c.c_eq_s(12, 1);
    c.swc1(12, sp + 8);
    if c.fcc() {
        c.cycles += 2;
        c.g[A0][0] = 2;
        c.g[A3][0] = 2;
        loop {
            c.cycles += 9;
            c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[V0][0] = sx(sp.wrapping_add(c.g[V1][0] as u32));
            c.lwc1(0, c.g[V0][0] as u32);
            c.c_eq_s(0, 1);
            if !c.fcc() {
                break;
            }
            c.g[A3][0] = c.g[A0][0];
            if event_due(c) {
                deliver_event_here(c, 0x0040_ebd8);
            }
        }
    }
    c.cycles += 8;
    c.g[T1][0] = sx(REM_PIO2F_TWO_OVER_PI);
    c.g[A0][0] = sx(sp);
    c.g[A1][0] = c.g[S1][0];
    c.g[T0][0] = 2;
    KERNEL_REM_PIO2F.call(c, 0x0040_ec14)?;
    c.g[A1][0] = c.g[V0][0];
    if (c.g[S2][0] as i64) >= 0 {
        c.cycles += 1;
        c.g[V0][0] = c.g[A1][0];
    } else {
        c.cycles += 4;
        let y = c.g[S1][0] as u32;
        c.lwc1(1, y);
        c.g[V0][0] = sx(0u32.wrapping_sub(c.g[A1][0] as u32));
        c.lwc1(0, y + 4);
        c.neg_s(1, 1);
        rem_pio2f_store_negated(c);
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    /// A machine with the game image loaded (jump tables, `.lit4`), `gp`
    /// set and `pc` at `entry` (the generated slot dispatches on it).
    pub(crate) fn oc383_machine(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        let mut r = Rng(0x0383_0000_0000 ^ salt ^ (seed as u64) << 20);
        scramble(c, &mut r);
        elf_image(c);
        c.g[GP][0] = sx(GAME_GP);
        c.pc = entry;
        r
    }

    #[test]
    fn ieee754_rem_pio2f_matches() {
        check(
            ee::s15::f_0040e878__generated,
            ieee754_rem_pio2f,
            400,
            &|c, seed| {
                let mut r = oc383_machine(c, seed, 0xe878, 0x0040_e878);
                let mag = match seed % 10 {
                    0 => r.next_u32() % 0x3f49_0fd9,              // |x| <= pi/4
                    1 => 0x3f49_0fd9 + r.next_u32() % 0xcd_bc0b,  // < 3pi/4
                    2 => 0x3fc9_0fd0 + r.next_u32() % 0x10,       // ~pi/2
                    3 => 0x4016_cbe4 + r.next_u32() % 0x1b2_4000, // medium
                    4 => (n_pio2(r.next_u32() % 31 + 1)) ^ (r.next_u32() % 0x100), // near n*pi/2
                    5 => 0x4349_0f81 + r.next_u32() % 0x3b00_0000, // large
                    6 => [0x7f80_0000, 0x7fc0_0000, 0x7f7f_ffff, 0x4349_0f80]
                        [(r.next_u32() % 4) as usize],
                    7 => 0x4400_0000 + r.next_u32() % 0x0800_0000, // large, few bits
                    _ => r.next_u32() & 0x7fff_ffff,
                };
                // Large values with trailing zero parts (the nx scan).
                let mag = if seed % 10 == 7 {
                    mag & 0xffff_0000
                } else {
                    mag
                };
                let sign = if r.next_u32() & 1 == 0 {
                    0
                } else {
                    0x8000_0000
                };
                c.f[12] = mag | sign;
                fill(c, &mut r, DATA, 4);
                c.g[A0][0] = sx(DATA);
            },
        );

        /// n * pi/2 as a float's bits (the npio2_hw entries' neighbourhood).
        fn n_pio2(n: u32) -> u32 {
            (n as f32 * std::f32::consts::FRAC_PI_2).to_bits()
        }
    }
}
