//! `src/seg/seg_2fe810.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::r32;
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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0040_e878, ieee754_rem_pio2f, "__ieee754_rem_pio2f"),
    (0x0040_ec58, kernel_rem_pio2f, "__kernel_rem_pio2f"),
];

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

pub(crate) mod callee {
    use super::*;

    /// `float scalbnf(float x, int n)`.
    pub(crate) const SCALBNF: Callee = Callee {
        addr: 0x0040_f9f8,
        f: ee::s15::scalbnf,
    };

    /// `float floorf(float x)`.
    pub(crate) const FLOORF: Callee = Callee {
        addr: 0x0040_f8e8,
        f: ee::s15::floorf,
    };
}

pub(crate) mod kernel_rem_pio2f_reg {
    use ssx3_ee_rt::cpu::reg::*;

    /// `jz`: the number of 8-bit chunks in use.
    pub(crate) const JZ: usize = S0;

    /// `jx = nx - 1`; from the chop on, the result `n & 7`.
    pub(crate) const JX: usize = S1;

    /// `q0`: the binary exponent of the lowest chunk.
    pub(crate) const Q0: usize = S2;

    /// `n`: the integer part of x * 2/pi.
    pub(crate) const N: usize = S3;

    /// `jk = init_jk[prec]` (also used as `jp`).
    pub(crate) const JK: usize = S4;

    /// `&q[0]`.
    pub(crate) const QS: usize = S5;

    /// `jv`: the first `ipio2` chunk needed.
    pub(crate) const JV: usize = S6;

    /// `y`.
    pub(crate) const Y: usize = S7;

    /// `4 * jz` as of the last distill.
    pub(crate) const JZ4: usize = FP;

    /// FPU f20: `z`.
    pub(crate) const Z: usize = 20;

    /// FPU f2: `fw`, the running sum.
    pub(crate) const FW: usize = 2;
}

pub(crate) mod kernel_rem_pio2f_slot {

    pub(crate) const SIZE: u32 = 0x210;

    /// `int iq[20]`: the fraction of x * 2/pi in 8-bit chunks, lowest first.
    pub(crate) const IQ: u32 = 0x00;

    /// `float f[20]`: the `ipio2` chunks the products use, as floats.
    pub(crate) const F: u32 = 0x50;

    /// `float fq[20]`: the `PIo2[] * q[]` sums.
    pub(crate) const FQ: u32 = 0xa0;

    /// `float q[20]`: the `x[] * f[]` sums, later the scaled chunks.
    pub(crate) const Q: u32 = 0xf0;

    /// The arguments x, prec and ipio2.
    pub(crate) const X: u32 = 0x140;

    pub(crate) const PREC: u32 = 0x144;

    pub(crate) const IPIO2: u32 = 0x148;

    /// `ih`: 0, 1 (q > 0.5 by the chunk bits) or 2 (by z >= 0.5).
    pub(crate) const IH: u32 = 0x14c;

    /// `prec < 3`, for the final switch.
    pub(crate) const PREC_BELOW_3: u32 = 0x150;

    /// `jk < 0` (no q terms and empty PIo2 sums).
    pub(crate) const JK_NEGATIVE: u32 = 0x154;

    pub(crate) const RA: u32 = 0x1f0;

    pub(crate) const F20: u32 = 0x200;
}

pub(crate) mod kernel_rem_pio2f_head {

    /// The `f[]` fill.
    pub(crate) const FILL_F: u32 = 0x0040_ed08;

    /// The first `q[i]` terms, i = 0..=jk, and their x * f products.
    pub(crate) const Q_TERM: u32 = 0x0040_ed78;

    pub(crate) const Q_DOT: u32 = 0x0040_ed98;

    /// `recompute:`.
    pub(crate) const RECOMPUTE: u32 = 0x0040_edf4;

    /// q[] into iq[] (most significant last).
    pub(crate) const DISTILL: u32 = 0x0040_ee28;

    /// 1 - q, chunk by chunk with the borrow.
    pub(crate) const CARRY: u32 = 0x0040_ef58;

    /// `j |= iq[i]` for i = jz-1 down to jk.
    pub(crate) const ZERO_SCAN: u32 = 0x0040_f028;

    /// `for (k = 1; iq[jk - k] == 0; k++)`.
    pub(crate) const K_SCAN: u32 = 0x0040_f068;

    /// The k extra terms and their products.
    pub(crate) const NEW_TERM: u32 = 0x0040_f0b0;

    pub(crate) const NEW_DOT: u32 = 0x0040_f0f0;

    /// Dropping zero chunks at the top.
    pub(crate) const CHOP: u32 = 0x0040_f180;

    /// `q[i] = fw * iq[i]`.
    pub(crate) const SCALE: u32 = 0x0040_f260;

    /// The `fq[jz - i]` sums and their PIo2 * q products.
    pub(crate) const FQ_TERM: u32 = 0x0040_f298;

    pub(crate) const FQ_DOT: u32 = 0x0040_f2c0;

    /// prec 0: the sum of fq[].
    pub(crate) const SUM0: u32 = 0x0040_f378;

    /// prec 1 and 2: the sum of fq[], then fq[1..=jz] onto fq[0] - sum.
    pub(crate) const SUM12: u32 = 0x0040_f3c8;

    pub(crate) const SUM12_TAIL: u32 = 0x0040_f418;

    /// prec 3: the two renormalising passes and the sum of fq[2..].
    pub(crate) const FOLD3_PASS1: u32 = 0x0040_f468;

    pub(crate) const FOLD3_PASS2: u32 = 0x0040_f4b8;

    pub(crate) const SUM3: u32 = 0x0040_f510;
}

// ---------------------------------------------------------------------------
// __kernel_rem_pio2f (0x40ec58)

/// `init_jk[] = {4, 7, 9}` (0x4995e8), the number of terms per precision;
/// `init_jk[3]` reads the zero word after it.
pub(crate) const KERNEL_REM_PIO2F_INIT_JK: u32 = 0x0049_95e8;

/// `PIo2[]` (0x4995f8): pi/2 in 8-bit pieces. The loop addresses it as
/// `lui t3, 0x4a` minus 0x6a08 and keeps the upper half in t3.
pub(crate) const KERNEL_REM_PIO2F_PIO2: u32 = 0x0049_95f8;

pub(crate) const KERNEL_REM_PIO2F_PIO2_HI: u32 = 0x004a_0000;

/// The float literals, built with `lui at` (`two8`, `twon8`, 0.125, 8,
/// 0.5, 1).
pub(crate) const KERNEL_REM_PIO2F_TWO8: u32 = 0x4380_0000;

pub(crate) const KERNEL_REM_PIO2F_TWON8: u32 = 0x3b80_0000;

pub(crate) const KERNEL_REM_PIO2F_EIGHTH: u32 = 0x3e00_0000;

pub(crate) const KERNEL_REM_PIO2F_EIGHT: u32 = 0x4100_0000;

pub(crate) const KERNEL_REM_PIO2F_HALF: u32 = 0x3f00_0000;

pub(crate) const KERNEL_REM_PIO2F_ONE: u32 = 0x3f80_0000;

/// The callee-saved GPRs and their `sd` slots.
pub(crate) const KERNEL_REM_PIO2F_SAVED: [(usize, u32); 9] = [
    (S0, 0x160),
    (S1, 0x170),
    (S2, 0x180),
    (S3, 0x190),
    (S4, 0x1a0),
    (S5, 0x1b0),
    (S6, 0x1c0),
    (S7, 0x1d0),
    (FP, 0x1e0),
];

/// `sp + off` in the frame.
#[inline]
pub(crate) fn kernel_rem_pio2f_at(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `(char *) iq + byte_off`.
#[inline]
pub(crate) fn kernel_rem_pio2f_iq(c: &Cpu, byte_off: u32) -> u32 {
    kernel_rem_pio2f_at(c, kernel_rem_pio2f_slot::IQ).wrapping_add(byte_off)
}

/// A back-edge event check at `head`.
#[inline]
pub(crate) fn kernel_rem_pio2f_back_edge(c: &mut Cpu, head: u32) {
    if event_due(c) {
        deliver_event_here(c, head);
    }
}

/// `int __kernel_rem_pio2f(float *x, float *y, int e0, int nx, int prec,
/// const __int32_t *ipio2)` (0x40ec58): fdlibm's reduction of a huge float
/// by pi/2. x[0..nx] are the argument's 8-bit pieces (x[0] * 2^e0 the
/// leading one); the product with the 2/pi chunks in `ipio2` is distilled
/// into 8-bit chunks
/// (adding terms while the fraction comes out all zero), the fraction is
/// multiplied by pi/2 and folded into y[] (one float for prec 0, two for 1
/// and 2, three for 3; nothing for others). Returns n & 7, the quadrant.
pub fn kernel_rem_pio2f(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    guest(c, |c| {
        kernel_rem_pio2f_setup(c);
        kernel_rem_pio2f_fill_f(c);
        kernel_rem_pio2f_first_terms(c);
        c.cycles += 1;
        c.g[JZ][0] = c.g[JK][0];
        loop {
            kernel_rem_pio2f_distill(c);
            kernel_rem_pio2f_take_n(c)?;
            kernel_rem_pio2f_round_up(c)?;
            if !kernel_rem_pio2f_add_terms(c) {
                break;
            }
            kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::RECOMPUTE);
        }
        kernel_rem_pio2f_chop(c)?;
        kernel_rem_pio2f_scale(c)?;
        kernel_rem_pio2f_pio2_sums(c);
        kernel_rem_pio2f_fold(c);
        kernel_rem_pio2f_epilogue(c);
        Ok(())
    })
}

/// The prologue, the argument spills and `jv`, `q0`, `jx`, `jk`. Leaves
/// a1 = j = jv - jx, a2 = i = 0, a3 = m = jx + jk.
pub(crate) fn kernel_rem_pio2f_setup(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    c.cycles += 37;
    c.g[SP][0] = sx(sp(c).wrapping_sub(slot::SIZE));
    for (reg, off) in KERNEL_REM_PIO2F_SAVED {
        let at = kernel_rem_pio2f_at(c, off);
        c.sd(at, c.g[reg][0]);
    }
    c.sd(kernel_rem_pio2f_at(c, slot::RA), c.g[RA][0]);
    c.swc1(Z, kernel_rem_pio2f_at(c, slot::F20));
    c.sw(kernel_rem_pio2f_at(c, slot::PREC), c.g[T0][0]);
    // jv = (e0 - 3) / 8 rounded toward zero ((e0 + 4) >> 3 below zero),
    // at least 0.
    c.g[T4][0] = sx(r32(c, A2).wrapping_sub(3));
    c.g[T3][0] = u64::MAX;
    c.g[V0][0] = sx(r32(c, A2).wrapping_add(4));
    c.g[T2][0] = ((c.g[T3][0] as i64) < (c.g[T4][0] as i64)) as u64;
    if c.g[T2][0] != 0 {
        c.g[V0][0] = c.g[T4][0];
    }
    c.g[JV][0] = sx((r32(c, V0) as i32 >> 3) as u32);
    c.g[T3][0] = ((c.g[T3][0] as i64) < (c.g[JV][0] as i64)) as u64;
    if c.g[T3][0] == 0 {
        c.g[JV][0] = 0;
    }
    // &init_jk[prec]
    c.g[V1][0] = sx(KERNEL_REM_PIO2F_INIT_JK);
    c.g[V0][0] = sx(r32(c, T0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    // q0 = e0 - 8 * (jv + 1)
    c.g[V1][0] = sx(r32(c, JV).wrapping_add(1));
    c.g[V1][0] = sx(r32(c, V1) << 3);
    c.g[JX][0] = sx(r32(c, A3).wrapping_sub(1));
    c.g[Q0][0] = sx(r32(c, A2).wrapping_sub(r32(c, V1)));
    c.g[Y][0] = c.g[A1][0];
    c.g[A1][0] = sx(r32(c, JV).wrapping_sub(r32(c, JX)));
    c.g[A2][0] = 0;
    c.g[JK][0] = c.lw(r32(c, V0));
    c.sw(kernel_rem_pio2f_at(c, slot::X), c.g[A0][0]);
    c.g[A3][0] = sx(r32(c, JX).wrapping_add(r32(c, JK)));
    c.sw(kernel_rem_pio2f_at(c, slot::IPIO2), c.g[T1][0]);
}

/// `f[i] = j < 0 ? 0 : (float) ipio2[j]` for i = 0..=m (j = jv - jx + i),
/// and the `jk < 0` / `prec < 3` flags. Leaves t0 = (jk < 0).
pub(crate) fn kernel_rem_pio2f_fill_f(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    if (c.g[A3][0] as i64) < 0 {
        // m < 0: nothing to fill.
        c.cycles += 7;
        c.g[A2][0] = c.lw(kernel_rem_pio2f_at(c, slot::PREC));
        c.g[A1][0] = ((c.g[JK][0] as i64) < 0) as u64;
        c.sw(kernel_rem_pio2f_at(c, slot::JK_NEGATIVE), c.g[A1][0]);
        c.g[QS][0] = sx(kernel_rem_pio2f_at(c, slot::Q));
        c.g[A2][0] = ((c.g[A2][0] as i64) < 3) as u64;
        c.sw(kernel_rem_pio2f_at(c, slot::PREC_BELOW_3), c.g[A2][0]);
        c.g[T0][0] = c.lw(kernel_rem_pio2f_at(c, slot::JK_NEGATIVE));
        return;
    }
    c.cycles += 7;
    c.g[V0][0] = ((c.g[JK][0] as i64) < 0) as u64;
    c.g[V1][0] = ((c.g[T0][0] as i64) < 3) as u64;
    c.sw(kernel_rem_pio2f_at(c, slot::JK_NEGATIVE), c.g[V0][0]);
    c.g[T1][0] = sx(kernel_rem_pio2f_at(c, slot::F));
    c.sw(kernel_rem_pio2f_at(c, slot::PREC_BELOW_3), c.g[V1][0]);
    c.g[QS][0] = sx(kernel_rem_pio2f_at(c, slot::Q));
    loop {
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, A2) << 2);
        c.g[V1][0] = sx(r32(c, T1).wrapping_add(r32(c, V0))); // &f[i]
        if (c.g[A1][0] as i64) < 0 {
            c.cycles += 1;
            c.sw(r32(c, V1), 0);
        } else {
            c.cycles += 7;
            c.g[A0][0] = c.lw(kernel_rem_pio2f_at(c, slot::IPIO2));
            c.g[V0][0] = sx(r32(c, A1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.lwc1(0, r32(c, V0));
            c.cvt_s_w(0, 0);
            c.swc1(0, r32(c, V1));
        }
        c.cycles += 4;
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
        c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[A2][0] as i64)) as u64;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        if c.g[V0][0] != 0 {
            break;
        }
        kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::FILL_F);
    }
    c.cycles += 2;
    c.g[T0][0] = c.lw(kernel_rem_pio2f_at(c, slot::JK_NEGATIVE));
}

/// `fw = sum of x[j] * f[base - j]` for j = 0..=jx onto f2, with a1 = j,
/// `base` in register `index`, t1 = f, a3 = 4 * i; leaves v1 = &q[i].
pub(crate) fn kernel_rem_pio2f_dot(c: &mut Cpu, index: usize, head: u32) {
    use kernel_rem_pio2f_reg::*;
    loop {
        c.cycles += 13;
        c.g[V0][0] = sx(r32(c, index).wrapping_sub(r32(c, A1)));
        c.g[A2][0] = c.lw(kernel_rem_pio2f_at(c, kernel_rem_pio2f_slot::X));
        c.g[V1][0] = sx(r32(c, A1) << 2);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, V0)));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A2)));
        c.lwc1(1, r32(c, V0)); // f[base - j]
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
        c.lwc1(0, r32(c, V1)); // x[j]
        c.g[V0][0] = ((c.g[JX][0] as i64) < (c.g[A1][0] as i64)) as u64;
        c.mul_s(0, 0, 1);
        c.add_s(FW, FW, 0);
        if c.g[V0][0] != 0 {
            break;
        }
        kernel_rem_pio2f_back_edge(c, head);
    }
    c.cycles += 2;
    c.g[V1][0] = sx(r32(c, QS).wrapping_add(r32(c, A3)));
}

/// jx < 0: no products; t0 = i + 1, v1 = &q[i].
pub(crate) fn kernel_rem_pio2f_no_products(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    c.cycles += 3;
    c.g[T0][0] = sx(r32(c, A2).wrapping_add(1));
    c.g[A3][0] = sx(r32(c, A2) << 2);
    c.g[V1][0] = sx(r32(c, QS).wrapping_add(r32(c, A3)));
}

/// `q[i] = fw` and i = t0; true once i passes the register `last`.
pub(crate) fn kernel_rem_pio2f_store_term(c: &mut Cpu, last: usize) -> bool {
    use kernel_rem_pio2f_reg::*;
    c.cycles += 4;
    c.g[A2][0] = c.g[T0][0];
    c.g[V0][0] = ((c.g[last][0] as i64) < (c.g[A2][0] as i64)) as u64;
    c.swc1(FW, r32(c, V1));
    c.g[V0][0] != 0
}

/// `q[i] = sum of x[j] * f[jx + i - j]` for i = 0..=jk (none when jk < 0,
/// t0 set).
pub(crate) fn kernel_rem_pio2f_first_terms(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    c.cycles += 2;
    c.g[A2][0] = 0;
    if c.g[T0][0] != 0 {
        return;
    }
    c.cycles += 2;
    c.g[T3][0] = ((c.g[JX][0] as i64) < 0) as u64;
    loop {
        c.cycles += 3;
        c.mtc1(ZERO, FW);
        c.g[A1][0] = 0;
        if c.g[T3][0] != 0 {
            kernel_rem_pio2f_no_products(c);
        } else {
            c.cycles += 5;
            c.g[A0][0] = sx(r32(c, JX).wrapping_add(r32(c, A2)));
            c.g[A3][0] = sx(r32(c, A2) << 2);
            c.g[T1][0] = sx(kernel_rem_pio2f_at(c, kernel_rem_pio2f_slot::F));
            c.g[T0][0] = sx(r32(c, A2).wrapping_add(1));
            kernel_rem_pio2f_dot(c, A0, kernel_rem_pio2f_head::Q_DOT);
        }
        if kernel_rem_pio2f_store_term(c, JK) {
            break;
        }
        kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::Q_TERM);
    }
}

/// Distil q[0..=jz] into iq[]: from the top term down, `fw = (int)(z /
/// 256)`, `iq[i] = (int)(z - 256 * fw)`, `z = q[j - 1] + fw`. z ends as
/// the integer part (q[0] plus the carry).
pub(crate) fn kernel_rem_pio2f_distill(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    c.cycles += 5;
    c.g[JZ4][0] = sx(r32(c, JZ) << 2);
    c.g[A1][0] = c.g[JZ][0];
    c.g[V0][0] = sx(r32(c, QS).wrapping_add(r32(c, JZ4)));
    c.lwc1(Z, r32(c, V0)); // z = q[jz]
    if (c.g[JZ][0] as i64) <= 0 {
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = sx(r32(c, JZ4).wrapping_sub(4));
    rem_pio2f_const(c, 4, KERNEL_REM_PIO2F_TWON8);
    rem_pio2f_const(c, 3, KERNEL_REM_PIO2F_TWO8);
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, QS))); // &q[jz - 1]
    c.g[V1][0] = c.g[SP][0]; // &iq[0]
    loop {
        c.cycles += 15;
        c.mul_s(0, Z, 4);
        c.lwc1(1, r32(c, A0));
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(4));
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        c.cvt_w_s(2, 0);
        c.mfc1(V0, 2);
        c.mtc1(V0, 2);
        c.cvt_s_w(FW, 2);
        c.mul_s(0, FW, 3);
        c.sub_s(0, Z, 0);
        c.add_s(Z, 1, FW);
        c.cvt_w_s(1, 0);
        c.swc1(1, r32(c, V1));
        let more = (c.g[A1][0] as i64) > 0;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
        if !more {
            break;
        }
        kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::DISTILL);
    }
}

/// `z = scalbnf(z, q0)`, `z -= 8 * floorf(z / 8)`, `n = (int) z`, `z -= n`,
/// then `ih`: from the chunk below the point when q0 >= 0 (moving its bits
/// above the point into n when q0 > 0), else 2 when z >= 0.5.
pub(crate) fn kernel_rem_pio2f_take_n(c: &mut Cpu) -> crate::call::Flow {
    use kernel_rem_pio2f_reg::*;
    let ih_at = kernel_rem_pio2f_at(c, kernel_rem_pio2f_slot::IH);
    c.cycles += 19;
    c.mov_s(12, Z);
    c.g[A0][0] = c.g[Q0][0];
    callee::SCALBNF.call(c, 0x0040_ee70)?;
    rem_pio2f_const(c, 12, KERNEL_REM_PIO2F_EIGHTH);
    c.mov_s(Z, 0);
    c.sw(ih_at, 0);
    c.mul_s(12, Z, 12);
    callee::FLOORF.call(c, 0x0040_ee88)?;
    rem_pio2f_const(c, 1, KERNEL_REM_PIO2F_EIGHT);
    c.mul_s(0, 0, 1);
    c.sub_s(Z, Z, 0);
    c.cvt_w_s(0, Z);
    c.mfc1(N, 0);
    c.mtc1(N, 0);
    c.cvt_s_w(0, 0);
    let q0 = c.g[Q0][0] as i64;
    c.sub_s(Z, Z, 0);
    if q0 > 0 {
        // i = iq[jz-1] >> (8 - q0); n += i; iq[jz-1] -= i << (8 - q0);
        // ih = iq[jz-1] >> (7 - q0).
        c.cycles += 16;
        c.g[V0][0] = sx(r32(c, JZ).wrapping_sub(1));
        c.g[V1][0] = 8;
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, Q0)));
        c.g[A1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0))); // &iq[jz - 1]
        c.g[A0][0] = 7;
        c.g[V0][0] = c.lw(r32(c, A1));
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, Q0)));
        c.g[A2][0] = sx((r32(c, V0) as i32 >> (r32(c, V1) & 31)) as u32);
        c.g[V1][0] = sx(r32(c, A2) << (r32(c, V1) & 31));
        c.g[N][0] = sx(r32(c, N).wrapping_add(r32(c, A2)));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
        c.g[A0][0] = sx((r32(c, V0) as i32 >> (r32(c, A0) & 31)) as u32);
        c.sw(ih_at, c.g[A0][0]);
        c.sw(r32(c, A1), c.g[V0][0]);
        return Ok(());
    }
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, JZ).wrapping_sub(1));
    if c.g[Q0][0] == 0 {
        // ih = iq[jz-1] >> 8.
        c.cycles += 6;
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0)));
        c.g[A0][0] = c.lw(r32(c, V1));
        c.g[A0][0] = sx((r32(c, A0) as i32 >> 8) as u32);
        c.sw(ih_at, c.g[A0][0]);
    } else {
        c.cycles += 6;
        rem_pio2f_const(c, 0, KERNEL_REM_PIO2F_HALF);
        c.c_le_s(0, Z);
        c.g[V0][0] = 2;
        if c.fcc() {
            c.cycles += 1;
            c.sw(ih_at, c.g[V0][0]);
        }
    }
    Ok(())
}

/// ih > 0 (q > 0.5): n += 1 and the chunks become 1 - q (the lowest
/// non-zero one `0x100 - j`, every one above it `0xff - j`; a3 = carry),
/// the bits above the point cleared again for q0 = 1, 2; for ih = 2 also
/// `z = 1 - z`, less `scalbnf(1, q0)` when a chunk was non-zero.
pub(crate) fn kernel_rem_pio2f_round_up(c: &mut Cpu) -> crate::call::Flow {
    use kernel_rem_pio2f_reg::*;
    let ih_at = kernel_rem_pio2f_at(c, kernel_rem_pio2f_slot::IH);
    c.cycles += 3;
    c.g[V1][0] = c.lw(ih_at);
    if (c.g[V1][0] as i64) <= 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[N][0] = sx(r32(c, N).wrapping_add(1));
    c.g[A3][0] = 0;
    if (c.g[JZ][0] as i64) > 0 {
        c.cycles += 5;
        c.g[T0][0] = 0x100;
        c.g[A0][0] = 0xff;
        c.g[V1][0] = c.g[SP][0]; // &iq[0]
        c.g[A2][0] = c.g[JZ][0];
        loop {
            c.cycles += 2;
            c.g[A1][0] = c.lw(r32(c, V1)); // j = iq[i]
            let store = if c.g[A3][0] != 0 {
                c.cycles += 1;
                c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, A1)));
                true
            } else {
                c.cycles += 2;
                c.g[V0][0] = sx(r32(c, T0).wrapping_sub(r32(c, A1)));
                if c.g[A1][0] != 0 {
                    c.cycles += 2;
                    c.g[A3][0] = 1;
                    true
                } else {
                    false
                }
            };
            if store {
                c.cycles += 1;
                c.sw(r32(c, V1), c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            let more = c.g[A2][0] != 0;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            if !more {
                break;
            }
            kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::CARRY);
        }
    }
    // switch (q0): case 1: iq[jz-1] &= 0x7f; case 2: iq[jz-1] &= 0x3f.
    c.cycles += 2;
    c.g[V0][0] = 1;
    let mut ih_loaded = false;
    if (c.g[Q0][0] as i64) > 0 {
        c.cycles += 2;
        let q0_is_1 = c.g[Q0][0] == c.g[V0][0];
        c.g[V0][0] = 2;
        let mask = if q0_is_1 {
            c.cycles += 6;
            Some(0x7fu64)
        } else {
            c.cycles += 2;
            let q0_is_2 = c.g[Q0][0] == c.g[V0][0];
            c.g[A0][0] = c.lw(ih_at);
            if q0_is_2 {
                c.cycles += 5;
                Some(0x3fu64)
            } else {
                c.cycles += 2;
                ih_loaded = true;
                None
            }
        };
        if let Some(mask) = mask {
            c.g[V0][0] = sx(r32(c, JZ).wrapping_sub(1));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[A0][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0))); // &iq[jz - 1]
            c.g[V1][0] = c.lw(r32(c, A0)) & mask;
            c.cycles += 1;
            c.sw(r32(c, A0), c.g[V1][0]);
        }
    }
    if !ih_loaded {
        c.cycles += 2;
        c.g[A0][0] = c.lw(ih_at);
        c.g[V0][0] = 2;
    }
    c.cycles += 2;
    if c.g[A0][0] != c.g[V0][0] {
        return Ok(());
    }
    // ih == 2
    c.cycles += 4;
    rem_pio2f_const(c, 12, KERNEL_REM_PIO2F_ONE);
    c.sub_s(Z, 12, Z);
    if c.g[A3][0] == 0 {
        return Ok(());
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[Q0][0];
    callee::SCALBNF.call(c, 0x0040_effc)?;
    c.sub_s(Z, Z, 0);
    Ok(())
}

/// z == 0 and the chunks `iq[jk..jz-1]` all zero: the fraction needs more
/// terms. Adds k of them (k - 1 = the zero chunks below `iq[jk]`), filling
/// `f[jx + i]` and `q[i]` for i = jz+1..=jz+k, and returns true to distil
/// again. Otherwise leaves fcc = (z == 0) and returns false.
pub(crate) fn kernel_rem_pio2f_add_terms(c: &mut Cpu) -> bool {
    use kernel_rem_pio2f_head as head;
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    c.cycles += 5;
    c.mtc1(ZERO, 0);
    c.c_eq_s(Z, 0);
    c.g[A2][0] = sx(r32(c, JZ).wrapping_sub(1));
    if !c.fcc() {
        return false;
    }
    // j = iq[jz-1] | ... | iq[jk] (a1)
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[JK][0] as i64)) as u64;
    c.g[A1][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = sx(r32(c, A2) << 2);
        c.g[A0][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0))); // &iq[jz - 1]
        loop {
            c.cycles += 7;
            c.g[V1][0] = c.lw(r32(c, A0));
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(4));
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[JK][0] as i64)) as u64;
            c.g[A1][0] |= c.g[V1][0];
            if c.g[V0][0] != 0 {
                break;
            }
            kernel_rem_pio2f_back_edge(c, head::ZERO_SCAN);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, JK).wrapping_sub(1));
    if c.g[A1][0] != 0 {
        c.cycles += 3;
        c.mtc1(ZERO, 0);
        c.c_eq_s(Z, 0);
        return false;
    }
    // k (t0): 1 + the zero chunks iq[jk-1], iq[jk-2], ...
    c.cycles += 5;
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0)));
    c.g[A0][0] = c.lw(r32(c, V1));
    c.g[T0][0] = 1;
    c.cycles += 2;
    c.g[A1][0] = sx(r32(c, JZ).wrapping_add(1));
    if c.g[A0][0] == 0 {
        loop {
            c.cycles += 7;
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
            c.g[V0][0] = sx(r32(c, JK).wrapping_sub(r32(c, T0)));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0)));
            c.g[A0][0] = c.lw(r32(c, V1));
            if c.g[A0][0] != 0 {
                break;
            }
            kernel_rem_pio2f_back_edge(c, head::K_SCAN);
        }
        c.cycles += 2;
    }
    c.g[A0][0] = sx(r32(c, JZ).wrapping_add(r32(c, T0))); // the new jz
    c.cycles += 4;
    c.g[A2][0] = c.g[A1][0]; // i = jz + 1
    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
    c.g[JZ][0] = c.g[A0][0];
    if c.g[V0][0] != 0 {
        // No new term (cannot happen: k >= 1).
        return true;
    }
    c.cycles += 3;
    c.g[T1][0] = sx(kernel_rem_pio2f_at(c, slot::F));
    c.g[T3][0] = ((c.g[JX][0] as i64) < 0) as u64;
    loop {
        // f[jx + i] = (float) ipio2[jv + i]
        c.cycles += 13;
        c.g[A1][0] = c.lw(kernel_rem_pio2f_at(c, slot::IPIO2));
        c.g[V0][0] = sx(r32(c, JV).wrapping_add(r32(c, A2)));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[T2][0] = sx(r32(c, JX).wrapping_add(r32(c, A2)));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
        c.g[V1][0] = sx(r32(c, T2) << 2);
        c.lwc1(0, r32(c, V0));
        c.cvt_s_w(0, 0);
        c.g[V1][0] = sx(r32(c, T1).wrapping_add(r32(c, V1)));
        c.g[A1][0] = 0;
        c.mtc1(ZERO, FW);
        c.swc1(0, r32(c, V1));
        if c.g[T3][0] != 0 {
            kernel_rem_pio2f_no_products(c);
        } else {
            c.cycles += 3;
            c.g[T0][0] = sx(r32(c, A2).wrapping_add(1));
            c.g[A3][0] = sx(r32(c, A2) << 2);
            kernel_rem_pio2f_dot(c, T2, head::NEW_DOT);
        }
        if kernel_rem_pio2f_store_term(c, A0) {
            break;
        }
        kernel_rem_pio2f_back_edge(c, head::NEW_TERM);
    }
    c.cycles += 2;
    c.g[JZ][0] = c.g[A0][0];
    true
}

/// Chop: z == 0 drops the zero chunks at the top (jz -= 1, q0 -= 8 each);
/// otherwise `z = scalbnf(z, -q0)` becomes the top chunk, split in two
/// when it is 256 or more. Sets s1 = n & 7.
pub(crate) fn kernel_rem_pio2f_chop(c: &mut Cpu) -> crate::call::Flow {
    use kernel_rem_pio2f_reg::*;
    c.cycles += 2;
    let zero = c.fcc();
    c.mov_s(12, Z);
    if zero {
        c.cycles += 6;
        c.g[JZ][0] = sx(r32(c, JZ).wrapping_sub(1));
        c.g[V0][0] = sx(r32(c, JZ) << 2);
        c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0)));
        c.g[A0][0] = c.lw(r32(c, V1)); // iq[jz]
        c.g[Q0][0] = sx(r32(c, Q0).wrapping_sub(8));
        c.cycles += 1;
        c.g[JX][0] = c.g[N][0] & 7;
        if c.g[A0][0] == 0 {
            loop {
                c.cycles += 7;
                c.g[JZ][0] = sx(r32(c, JZ).wrapping_sub(1));
                c.g[Q0][0] = sx(r32(c, Q0).wrapping_sub(8));
                c.g[V0][0] = sx(r32(c, JZ) << 2);
                c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0)));
                c.g[A0][0] = c.lw(r32(c, V1));
                if c.g[A0][0] != 0 {
                    break;
                }
                kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::CHOP);
            }
            c.cycles += 2;
        }
        return Ok(());
    }
    c.cycles += 9;
    c.g[A0][0] = sx(0u32.wrapping_sub(r32(c, Q0)));
    callee::SCALBNF.call(c, 0x0040_f1ac)?;
    rem_pio2f_const(c, 1, KERNEL_REM_PIO2F_TWO8);
    c.mov_s(Z, 0);
    c.c_le_s(1, Z);
    c.g[A0][0] = sx(kernel_rem_pio2f_iq(c, r32(c, JZ4))); // &iq[jz]
    if c.fcc() {
        // z >= two8: iq[jz] = low 8 bits, iq[jz + 1] = the rest.
        c.cycles += 19;
        rem_pio2f_const(c, 0, KERNEL_REM_PIO2F_TWON8);
        c.g[JZ][0] = sx(r32(c, JZ).wrapping_add(1));
        c.g[V0][0] = sx(r32(c, JZ) << 2);
        c.mul_s(0, Z, 0);
        c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0)));
        c.g[JX][0] = c.g[N][0] & 7;
        c.cvt_w_s(2, 0);
        c.mfc1(V0, 2);
        c.mtc1(V0, 2);
        c.cvt_s_w(FW, 2);
        c.mul_s(0, FW, 1);
        c.sub_s(0, Z, 0);
        c.cvt_w_s(1, 0);
        c.swc1(1, r32(c, A0));
        c.cvt_w_s(0, FW);
        c.swc1(0, r32(c, V1));
        c.g[Q0][0] = sx(r32(c, Q0).wrapping_add(8));
    } else {
        c.cycles += 5;
        c.g[V0][0] = sx(kernel_rem_pio2f_iq(c, r32(c, JZ4)));
        c.cvt_w_s(0, Z);
        c.swc1(0, r32(c, V0));
        c.g[JX][0] = c.g[N][0] & 7;
    }
    Ok(())
}

/// `fw = scalbnf(1, q0)`; `q[i] = fw * iq[i]`, `fw *= twon8` for i = jz
/// down to 0. Leaves a2 = jz.
pub(crate) fn kernel_rem_pio2f_scale(c: &mut Cpu) -> crate::call::Flow {
    use kernel_rem_pio2f_reg::*;
    c.cycles += 7;
    rem_pio2f_const(c, 12, KERNEL_REM_PIO2F_ONE);
    c.g[A0][0] = c.g[Q0][0];
    callee::SCALBNF.call(c, 0x0040_f23c)?;
    c.g[A2][0] = c.g[JZ][0];
    c.mov_s(FW, 0);
    if (c.g[A2][0] as i64) < 0 {
        return Ok(());
    }
    c.cycles += 6;
    c.g[V0][0] = sx(r32(c, A2) << 2);
    rem_pio2f_const(c, 1, KERNEL_REM_PIO2F_TWON8);
    c.g[V1][0] = sx(kernel_rem_pio2f_iq(c, r32(c, V0))); // &iq[jz]
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, QS))); // &q[jz]
    loop {
        c.cycles += 9;
        c.lwc1(0, r32(c, V1));
        c.cvt_s_w(0, 0);
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(4));
        c.mul_s(0, FW, 0);
        c.mul_s(FW, FW, 1);
        c.swc1(0, r32(c, V0));
        let more = (c.g[A2][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::SCALE);
    }
    c.cycles += 1;
    c.g[A2][0] = c.g[JZ][0];
    Ok(())
}

/// The fq slot of term i (v0 = 4 * (jz - i), t1 = jz - i) and the next i
/// (a2 = a1 = i - 1).
pub(crate) fn kernel_rem_pio2f_fq_next(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx(r32(c, T1) << 2);
    c.g[A2][0] = c.g[A1][0];
}

/// `fq[jz - i] = sum of PIo2[k] * q[i + k]` for k = 0..=min(jp, jz - i),
/// i = jz down to 0 (a2 = i, t0 = k, a3 = fq). Leaves v1 = (prec < 3).
pub(crate) fn kernel_rem_pio2f_pio2_sums(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    c.cycles += 2;
    if (c.g[A2][0] as i64) < 0 {
        c.g[V1][0] = c.lw(kernel_rem_pio2f_at(c, slot::PREC_BELOW_3));
        return;
    }
    c.cycles += 2;
    c.g[A3][0] = sx(kernel_rem_pio2f_at(c, slot::FQ));
    loop {
        c.cycles += 4;
        c.g[V0][0] = c.lw(kernel_rem_pio2f_at(c, slot::JK_NEGATIVE));
        c.mtc1(ZERO, FW);
        c.g[T0][0] = 0;
        if c.g[V0][0] != 0 {
            // jp < 0: no products.
            c.cycles += 2;
            c.g[T1][0] = sx(r32(c, JZ).wrapping_sub(r32(c, A2)));
            c.g[A1][0] = sx(r32(c, A2).wrapping_sub(1));
            kernel_rem_pio2f_fq_next(c);
        } else {
            c.cycles += 3;
            c.g[T1][0] = sx(r32(c, JZ).wrapping_sub(r32(c, A2)));
            c.g[T2][0] = c.g[T1][0];
            if (c.g[T1][0] as i64) < 0 {
                // Cannot happen: i <= jz.
                c.cycles += 1;
                c.g[A1][0] = sx(r32(c, A2).wrapping_sub(1));
                kernel_rem_pio2f_fq_next(c);
            } else {
                c.cycles += 3;
                c.g[A1][0] = sx(r32(c, A2).wrapping_sub(1));
                c.g[T3][0] = sx(KERNEL_REM_PIO2F_PIO2_HI);
                loop {
                    c.cycles += 13;
                    c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, T0)));
                    c.g[A0][0] = sx(r32(c, T0) << 2);
                    c.g[V0][0] = sx(KERNEL_REM_PIO2F_PIO2);
                    c.g[V1][0] = sx(r32(c, V1) << 2);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                    c.g[V1][0] = sx(r32(c, QS).wrapping_add(r32(c, V1)));
                    c.lwc1(0, r32(c, A0)); // PIo2[k]
                    c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                    c.lwc1(1, r32(c, V1)); // q[i + k]
                    c.g[V0][0] = ((c.g[JK][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    c.mul_s(0, 0, 1);
                    c.add_s(FW, FW, 0);
                    if c.g[V0][0] != 0 {
                        // k > jp
                        kernel_rem_pio2f_fq_next(c);
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    let more = c.g[V0][0] == 0;
                    c.g[V0][0] = sx(r32(c, T1) << 2);
                    if !more {
                        // k > jz - i
                        c.cycles += 2;
                        c.g[A2][0] = c.g[A1][0];
                        break;
                    }
                    kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::FQ_DOT);
                }
            }
        }
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
        let more = (c.g[A2][0] as i64) >= 0;
        c.swc1(FW, r32(c, V0));
        if !more {
            break;
        }
        kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::FQ_TERM);
    }
    c.cycles += 1;
    c.g[V1][0] = c.lw(kernel_rem_pio2f_at(c, slot::PREC_BELOW_3));
}

/// `fw += fq[i]` for i = a2 down to 0 (a3 = fq).
pub(crate) fn kernel_rem_pio2f_sum_down(c: &mut Cpu, head: u32) {
    use kernel_rem_pio2f_reg::*;
    loop {
        c.cycles += 7;
        c.g[V0][0] = sx(r32(c, A2) << 2);
        c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
        c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
        c.lwc1(0, r32(c, V0));
        c.add_s(FW, FW, 0);
        if (c.g[A2][0] as i64) < 0 {
            break;
        }
        kernel_rem_pio2f_back_edge(c, head);
    }
}

/// `switch (prec)`: y[] from fq[] (negated when ih != 0), v0 = n & 7, and
/// ra reloaded for the epilogue.
pub(crate) fn kernel_rem_pio2f_fold(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    let ra_at = kernel_rem_pio2f_at(c, slot::RA);
    c.cycles += 2;
    c.g[A0][0] = c.lw(kernel_rem_pio2f_at(c, slot::PREC));
    if c.g[V1][0] == 0 {
        // prec >= 3
        c.cycles += 4;
        c.g[A1][0] = c.lw(kernel_rem_pio2f_at(c, slot::PREC));
        c.g[V0][0] = 3;
        let three = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = c.g[JX][0];
        if !three {
            c.cycles += 2;
            c.g[RA][0] = c.ld(ra_at);
            return;
        }
        kernel_rem_pio2f_fold3(c);
    } else {
        c.cycles += 2;
        if (c.g[A0][0] as i64) > 0 {
            c.g[A2][0] = c.g[JZ][0];
            kernel_rem_pio2f_fold12(c);
        } else {
            c.cycles += 2;
            let zero = c.g[A0][0] == 0;
            c.g[V0][0] = c.g[JX][0];
            if !zero {
                c.cycles += 2;
                c.g[RA][0] = c.ld(ra_at);
                return;
            }
            kernel_rem_pio2f_fold0(c);
        }
    }
    c.cycles += 1;
    c.g[RA][0] = c.ld(ra_at);
}

/// prec 0: y[0] = the sum of fq[jz..0].
pub(crate) fn kernel_rem_pio2f_fold0(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    let y = r32(c, Y);
    c.cycles += 4;
    c.g[A2][0] = c.g[JZ][0];
    c.mtc1(ZERO, FW);
    if (c.g[A2][0] as i64) < 0 {
        c.swc1(FW, y);
    } else {
        c.cycles += 2;
        c.g[A3][0] = sx(kernel_rem_pio2f_at(c, slot::FQ));
        kernel_rem_pio2f_sum_down(c, kernel_rem_pio2f_head::SUM0);
        c.cycles += 1;
        c.swc1(FW, y);
    }
    c.cycles += 3;
    c.g[A2][0] = c.lw(kernel_rem_pio2f_at(c, slot::IH));
    let ih = c.g[A2][0] != 0;
    c.g[V0][0] = c.g[JX][0];
    if ih {
        c.cycles += 3;
        c.neg_s(0, FW);
        c.swc1(0, y);
    }
}

/// prec 1 and 2: y[0] = the sum of fq[jz..0], y[1] = (fq[0] - y[0]) plus
/// fq[1..=jz] (a2 = jz).
pub(crate) fn kernel_rem_pio2f_fold12(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    let y = r32(c, Y);
    let fq = kernel_rem_pio2f_at(c, slot::FQ);
    c.cycles += 3;
    c.mtc1(ZERO, FW);
    let empty = (c.g[A2][0] as i64) < 0;
    c.lwc1(3, fq);
    if empty {
        c.cycles += 2;
        c.g[V1][0] = ((c.g[A2][0] as i64) < 1) as u64;
        c.swc1(FW, y);
    } else {
        c.cycles += 3;
        c.g[A3][0] = sx(fq);
        c.g[V1][0] = ((c.g[A2][0] as i64) < 1) as u64;
        kernel_rem_pio2f_sum_down(c, kernel_rem_pio2f_head::SUM12);
        c.cycles += 2;
        c.swc1(FW, y);
    }
    c.cycles += 3;
    c.g[T0][0] = c.lw(kernel_rem_pio2f_at(c, slot::IH));
    if c.g[T0][0] != 0 {
        c.cycles += 3;
        c.neg_s(0, FW);
        c.swc1(0, y);
    }
    c.sub_s(FW, 3, FW);
    c.cycles += 2;
    c.g[A2][0] = 1;
    if c.g[V1][0] == 0 {
        c.cycles += 1;
        c.g[A3][0] = sx(fq);
        loop {
            c.cycles += 8;
            c.g[V0][0] = sx(r32(c, A2) << 2);
            c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(1));
            c.lwc1(0, r32(c, V0));
            c.g[V1][0] = ((c.g[JZ][0] as i64) < (c.g[A2][0] as i64)) as u64;
            c.add_s(FW, FW, 0);
            if c.g[V1][0] != 0 {
                break;
            }
            kernel_rem_pio2f_back_edge(c, kernel_rem_pio2f_head::SUM12_TAIL);
        }
    }
    c.cycles += 4;
    c.swc1(FW, y.wrapping_add(4));
    c.g[V0][0] = c.lw(kernel_rem_pio2f_at(c, slot::IH));
    let ih = c.g[V0][0] != 0;
    c.g[V0][0] = c.g[JX][0];
    if ih {
        c.cycles += 3;
        c.neg_s(0, FW);
        c.swc1(0, y.wrapping_add(4));
    }
}

/// One renormalising step of prec 3 at i (a2, a3 = fq): `fw = fq[i-1] +
/// fq[i]; fq[i] += fq[i-1] - fw; fq[i-1] = fw`; a2 = i - 1.
pub(crate) fn kernel_rem_pio2f_fold3_step(c: &mut Cpu) {
    use kernel_rem_pio2f_reg::*;
    c.g[A0][0] = sx(r32(c, A2).wrapping_sub(1));
    c.g[V0][0] = sx(r32(c, A2) << 2);
    c.g[V1][0] = sx(r32(c, A0) << 2);
    c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.g[V1][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
    c.lwc1(1, r32(c, V0)); // fq[i]
    c.lwc1(0, r32(c, V1)); // fq[i - 1]
    c.g[A2][0] = c.g[A0][0];
    c.add_s(FW, 0, 1);
    c.sub_s(0, 0, FW);
    c.add_s(1, 1, 0);
    c.swc1(1, r32(c, V0));
    c.swc1(FW, r32(c, V1));
}

/// prec 3: two renormalising passes over fq[] (down to i = 1, then 2),
/// then y = (fq[0], fq[1], the sum of fq[jz..2]).
pub(crate) fn kernel_rem_pio2f_fold3(c: &mut Cpu) {
    use kernel_rem_pio2f_head as head;
    use kernel_rem_pio2f_reg::*;
    use kernel_rem_pio2f_slot as slot;
    let y = r32(c, Y);
    let fq = kernel_rem_pio2f_at(c, slot::FQ);
    c.cycles += 3;
    c.g[A2][0] = c.g[JZ][0];
    let skip = (c.g[A2][0] as i64) <= 0;
    c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
    if !skip {
        c.cycles += 2;
        c.g[A3][0] = sx(fq);
        loop {
            c.cycles += 14;
            kernel_rem_pio2f_fold3_step(c);
            if (c.g[A2][0] as i64) <= 0 {
                break;
            }
            kernel_rem_pio2f_back_edge(c, head::FOLD3_PASS1);
        }
        c.cycles += 2;
        c.g[A2][0] = c.g[JZ][0];
        c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.g[A2][0] = c.g[JZ][0];
    } else {
        c.cycles += 2;
        c.g[A3][0] = sx(fq);
        loop {
            c.cycles += 15;
            kernel_rem_pio2f_fold3_step(c);
            c.g[A1][0] = ((c.g[A2][0] as i64) < 2) as u64;
            if c.g[A1][0] != 0 {
                break;
            }
            kernel_rem_pio2f_back_edge(c, head::FOLD3_PASS2);
        }
        c.cycles += 1;
        c.g[A2][0] = c.g[JZ][0];
    }
    // fw = fq[jz] + ... + fq[2]
    c.cycles += 4;
    c.mtc1(ZERO, FW);
    c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
    let short = c.g[V0][0] != 0;
    c.lwc1(3, fq);
    c.cycles += 2;
    if short {
        c.lwc1(1, fq.wrapping_add(4));
    } else {
        c.g[A3][0] = sx(fq);
        c.lwc1(1, fq.wrapping_add(4));
        loop {
            c.cycles += 8;
            c.g[V0][0] = sx(r32(c, A2) << 2);
            c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(1));
            c.lwc1(0, r32(c, V0));
            c.g[V1][0] = ((c.g[A2][0] as i64) < 2) as u64;
            c.add_s(FW, FW, 0);
            if c.g[V1][0] != 0 {
                break;
            }
            kernel_rem_pio2f_back_edge(c, head::SUM3);
        }
        c.cycles += 2;
    }
    c.g[V1][0] = c.lw(kernel_rem_pio2f_at(c, slot::IH));
    c.cycles += 2;
    if c.g[V1][0] != 0 {
        c.neg_s(0, 3);
        c.cycles += 5;
        c.neg_s(1, 1);
        c.neg_s(FW, FW);
        c.swc1(0, y);
        c.swc1(1, y.wrapping_add(4));
        c.swc1(FW, y.wrapping_add(8));
    } else {
        c.cycles += 4;
        c.swc1(FW, y.wrapping_add(8));
        c.swc1(3, y);
        c.swc1(1, y.wrapping_add(4));
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[JX][0];
}

/// Restore the saved registers and return.
pub(crate) fn kernel_rem_pio2f_epilogue(c: &mut Cpu) {
    c.cycles += 12;
    for (reg, off) in KERNEL_REM_PIO2F_SAVED {
        c.g[reg][0] = c.ld(kernel_rem_pio2f_at(c, off));
    }
    c.lwc1(20, kernel_rem_pio2f_at(c, kernel_rem_pio2f_slot::F20));
    let ra = r32(c, RA);
    c.g[SP][0] = sx(sp(c).wrapping_add(kernel_rem_pio2f_slot::SIZE));
    c.pc = ra;
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

    use crate::difftest::check_paths;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::Events;
    use crate::game::misc::tests::event_at;
    use crate::game::misc::tests::require_heads;
    use crate::game::seg::seg_2fe810::REM_PIO2F_TWO_OVER_PI;

    /// The generated body the tests compare against.
    pub(crate) const KERNEL_REM_PIO2F_GENERATED: GuestFn = ee::s15::f_0040ec58__generated;

    pub(crate) const KERNEL_REM_PIO2F_ENTRY: u32 = 0x0040_ec58;

    /// The x pieces, y[] (pre-filled) and a made-up `ipio2` table.
    pub(crate) const KERNEL_REM_PIO2F_X_AT: u32 = DATA + 0x24;

    pub(crate) const KERNEL_REM_PIO2F_Y_AT: u32 = DATA + 0x134;

    pub(crate) const KERNEL_REM_PIO2F_TABLE_AT: u32 = DATA + 0x404;

    /// The function's frame below the test stack.
    pub(crate) const KERNEL_REM_PIO2F_FRAME_AT: u32 =
        crate::difftest::SP - kernel_rem_pio2f_slot::SIZE;

    /// A machine with registers scrambled, the game image loaded (init_jk,
    /// PIo2, two_over_pi), gp set and pc at the entry.
    pub(crate) fn kernel_rem_pio2f_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x0040_ec58_0000 ^ salt ^ (seed as u64) << 20);
        scramble(c, &mut r);
        elf_image(c);
        c.g[GP][0] = sx(GAME_GP);
        c.pc = KERNEL_REM_PIO2F_ENTRY;
        r
    }

    /// One call: x pieces (float bits), e0, prec, the `ipio2` table, and a
    /// replacement for `init_jk[prec]` (its word written over in RAM).
    pub(crate) struct KernelRemPio2fCall {
        pub(crate) x: Vec<u32>,
        pub(crate) e0: i32,
        pub(crate) prec: i32,
        pub(crate) ipio2: u32,
        pub(crate) jk: Option<i32>,
    }

    /// Place `call`: y[] and the frame pre-filled with junk, the three words
    /// below the frame (`iq[-1..-3]`, read when jk is 0 or below) set from
    /// `below`, the arguments in a0-a3/t0/t1.
    pub(crate) fn kernel_rem_pio2f_place(
        c: &mut Cpu,
        r: &mut Rng,
        call: &KernelRemPio2fCall,
        below: [u32; 3],
    ) {
        fill(
            c,
            r,
            KERNEL_REM_PIO2F_FRAME_AT - 0x40,
            (kernel_rem_pio2f_slot::SIZE + 0x40) / 4,
        );
        for (i, w) in below.iter().enumerate() {
            c.bus
                .write32(KERNEL_REM_PIO2F_FRAME_AT - 4 * (i as u32 + 1), *w);
        }
        fill(c, r, KERNEL_REM_PIO2F_Y_AT, 4);
        for (i, w) in call.x.iter().enumerate() {
            c.bus.write32(KERNEL_REM_PIO2F_X_AT + 4 * i as u32, *w);
        }
        if let Some(jk) = call.jk {
            let at = KERNEL_REM_PIO2F_INIT_JK.wrapping_add((call.prec as u32) << 2);
            c.bus.write32(at, jk as u32);
        }
        c.g[A0][0] = sx(KERNEL_REM_PIO2F_X_AT);
        c.g[A1][0] = sx(KERNEL_REM_PIO2F_Y_AT);
        c.g[A2][0] = sx(call.e0 as u32);
        c.g[A3][0] = sx(call.x.len() as u32);
        c.g[T0][0] = sx(call.prec as u32);
        c.g[T1][0] = sx(call.ipio2);
    }

    /// A made-up `ipio2` table of chunks 1..=254 with `zeros` cleared.
    pub(crate) fn kernel_rem_pio2f_table(c: &mut Cpu, r: &mut Rng, first: u32, zeros: &[usize]) {
        for i in 0..64u32 {
            let v = if i == 0 {
                first
            } else {
                1 + r.next_u32() % 254
            };
            c.bus.write32(KERNEL_REM_PIO2F_TABLE_AT + 4 * i, v);
        }
        for &i in zeros {
            c.bus.write32(KERNEL_REM_PIO2F_TABLE_AT + 4 * i as u32, 0);
        }
    }

    /// The caller's x pieces: x[0] in 128..=255, the others 0..=255, the
    /// last non-zero, each times `scale`.
    pub(crate) fn kernel_rem_pio2f_pieces(r: &mut Rng, nx: usize, scale: f32) -> Vec<u32> {
        let mut x: Vec<u32> = (0..nx)
            .map(|i| {
                if i == 0 {
                    128 + r.next_u32() % 128
                } else {
                    r.next_u32() % 256
                }
            })
            .collect();
        if x[nx - 1] == 0 {
            x[nx - 1] = 1;
        }
        x.iter().map(|&v| (v as f32 * scale).to_bits()).collect()
    }

    /// A seeded call: precision 0-3, q0 = -5..2 (e0 - 3 mod 8), and an
    /// input kind: the caller's pieces with the game's two_over_pi table
    /// (kinds 0, 1), pieces that are multiples of 256 (a zero low chunk,
    /// kind 2), x = 1 with a made-up table whose chunks 1 (and 2)
    /// are zero (recomputation, kind 3), e0 below 3 (jv clamped, z >= 256,
    /// kind 4). `wide` picks a large jv and, for prec 3, init_jk[3] = 4..9
    /// instead of the 0 in the image. Every digit is independent of the
    /// others: the seed is `((((wide * 5 + kind) * 8 + residue) * 4 + prec)
    /// * 3 + nx) * 3 + below`.
    pub(crate) fn kernel_rem_pio2f_case(c: &mut Cpu, seed: u32) {
        let mut r = kernel_rem_pio2f_machine(c, seed, 0xec58);
        let prec = (seed / 9 % 4) as i32;
        let residue = (seed / 36 % 8) as i32;
        let kind = seed / 288 % 5;
        let wide = seed / 1440 % 2 == 1;
        let nx = 1 + (seed / 3 % 3) as usize;
        let below = match seed % 3 {
            0 => [0, 0, 1 + r.next_u32() % 255],
            1 => [0, 1 + r.next_u32() % 255, 1 + r.next_u32() % 255],
            _ => [
                1 + r.next_u32() % 255,
                1 + r.next_u32() % 255,
                1 + r.next_u32() % 255,
            ],
        };
        let jv = (if wide {
            2 + r.next_u32() % 12
        } else {
            r.next_u32() % 2
        }) as i32;
        let jk = if prec == 3 && wide {
            Some(4 + (r.next_u32() % 6) as i32)
        } else {
            None
        };
        let mut call = KernelRemPio2fCall {
            x: Vec::new(),
            e0: 3 + 8 * jv + residue,
            prec,
            ipio2: REM_PIO2F_TWO_OVER_PI,
            jk,
        };
        match kind {
            0 | 1 => call.x = kernel_rem_pio2f_pieces(&mut r, nx, 1.0),
            2 => call.x = kernel_rem_pio2f_pieces(&mut r, nx, 256.0),
            3 => {
                call.x = [1.0f32, 0.0, 0.0][..nx]
                    .iter()
                    .map(|v| v.to_bits())
                    .collect();
                call.e0 = 3 + residue;
                call.ipio2 = KERNEL_REM_PIO2F_TABLE_AT;
                let first = 1 + r.next_u32() % 254;
                let zeros: &[usize] = if wide { &[1, 2][..] } else { &[1][..] };
                kernel_rem_pio2f_table(c, &mut r, first, zeros);
            }
            _ => {
                call.x = kernel_rem_pio2f_pieces(&mut r, nx, 1.0);
                call.e0 = -12 + 2 * residue;
            }
        }
        kernel_rem_pio2f_place(c, &mut r, &call, below);
    }

    /// `__kernel_rem_pio2f` over every precision, q0 class and input kind:
    /// the game's 2/pi table, recomputation with more terms, zero chunks,
    /// rounding up with the borrow, z >= 256, init_jk[3] as in the image.
    #[test]
    fn kernel_rem_pio2f_matches() {
        check(
            KERNEL_REM_PIO2F_GENERATED,
            kernel_rem_pio2f,
            2880,
            &|c, seed| kernel_rem_pio2f_case(c, seed),
        );
    }

    /// Short calls with init_jk[prec] = -1 (m < 0, no terms, z read from
    /// `q[-1]`, the chunk below the frame as `iq[jz-1]`), and prec 15 (the
    /// image's 0 after init_jk, an unknown precision):
    /// 0: prec 0, q0 = 1, ih = 1 (mask 0x7f), y[0] = -0: 273 cycles;
    /// 1: prec 1, q0 = 2, ih = 1 (mask 0x3f), both y negated: 287;
    /// 2: prec 3, q0 = 0, ih = 1 from iq[jz-1] >> 8, y negated: 265;
    /// 3: prec 3, q0 = -5, z >= 0.5 (ih = 2, no borrow), y negated: 270;
    /// 4: prec 3, q0 = -5, ih = 0: 255;
    /// 5: prec 0, e0 = -6 (jv clamped to 0, q0 = -14), ih = 2, z >= 256
    ///    split, one chunk scaled, jp < 0 sum, y[0] summed, negated: 315;
    /// 6: prec 15 (jk = 0), one term and one PIo2 product, no y: 330;
    /// 7: prec -2, no y: 238.
    #[test]
    fn kernel_rem_pio2f_short_paths_match() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = kernel_rem_pio2f_machine(c, seed, 0xec59);
            // (prec, init_jk patch, e0, z = q[-1], iq[-2])
            let (prec, jk, e0, z, iq2): (i32, Option<i32>, i32, f32, u32) = match seed % 8 {
                0 => (0, Some(-1), 9, 3.25, 0x1c5),
                1 => (1, Some(-1), 10, 3.375, 0xe1),
                2 => (3, Some(-1), 8, 3.25, 0x1ff),
                3 => (3, Some(-1), 3, 20.0, 0x11),
                4 => (3, Some(-1), 3, 8.0, 0x11),
                5 => (0, Some(-1), -6, 12288.0, 0x11),
                6 => (15, None, 3, 0.0, 0x11),
                _ => (-2, Some(-1), 3, 8.0, 0x11),
            };
            let call = KernelRemPio2fCall {
                x: vec![1.0f32.to_bits()],
                e0,
                prec,
                ipio2: REM_PIO2F_TWO_OVER_PI,
                jk,
            };
            kernel_rem_pio2f_place(c, &mut r, &call, [0x33, iq2, 0x55]);
            // q[-1] is fq[19], read as z when jz = -1.
            let q_minus_1 = KERNEL_REM_PIO2F_FRAME_AT + kernel_rem_pio2f_slot::Q - 4;
            c.bus.write32(q_minus_1, z.to_bits());
        };
        check_paths(
            KERNEL_REM_PIO2F_GENERATED,
            kernel_rem_pio2f,
            64,
            &[273, 287, 265, 270, 255, 315, 330, 238],
            &setup,
        );
    }

    /// The main paths, pinned with the generated body's cycle totals: one
    /// seed per precision, input kind, table width, piece count, chunk
    /// shape and q0 class (see `kernel_rem_pio2f_case`).
    #[test]
    fn kernel_rem_pio2f_paths_match() {
        const SEEDS: [u32; 16] = [
            36, 81, 126, 171, 72, 432, 792, 864, 1224, 1854, 21, 24, 19, 20, 180, 252,
        ];
        const TOTALS: [u64; 16] = [
            1150, 2069, 2084, 775, 1320, 1320, 1284, 1009, 1131, 2621, 2228, 2366, 2205, 2205,
            1150, 1166,
        ];
        check_paths(
            KERNEL_REM_PIO2F_GENERATED,
            kernel_rem_pio2f,
            SEEDS.len() as u32,
            &TOTALS,
            &|c, seed| kernel_rem_pio2f_case(c, SEEDS[seed as usize]),
        );
    }

    /// The event-test calls (x = 1, 0, 0 with a made-up table, so the
    /// paths are known): 0: prec 2, q0 = 1, chunks 1 and 2 zero (three more
    /// terms, the zero scan, two chopped chunks); 1: prec 0, q0 = -5,
    /// z = 0.5 (ih = 2) and chunk 4 zero (the borrow loop's three arms);
    /// 2: prec 3 with init_jk[3] = 4 and ih = 2 (both prec-3 passes).
    pub(crate) fn kernel_rem_pio2f_event_call(c: &mut Cpu, seed: u32, base: u32) {
        let mut r = kernel_rem_pio2f_machine(c, seed, 0xec5a);
        let mut t = Rng(0x0040_ec58 + base as u64);
        let (prec, jk, e0, first, zeros): (i32, Option<i32>, i32, u32, &[usize]) = match base {
            0 => (2, None, 9, 0x55, &[1, 2][..]),
            1 => (0, None, 3, 0x70, &[4][..]),
            _ => (3, Some(4), 3, 0x51, &[][..]),
        };
        kernel_rem_pio2f_table(c, &mut t, first, zeros);
        let call = KernelRemPio2fCall {
            x: [1.0f32, 0.0, 0.0].iter().map(|v| v.to_bits()).collect(),
            e0,
            prec,
            ipio2: KERNEL_REM_PIO2F_TABLE_AT,
            jk,
        };
        kernel_rem_pio2f_place(c, &mut r, &call, [0x33, 0x44, 0x55]);
    }

    thread_local! {
        /// The back-edge checks of a spied run: (loop head, cycles).
        static KERNEL_REM_PIO2F_CHECKS: std::cell::RefCell<Vec<(u32, u64)>> =
            const { std::cell::RefCell::new(Vec::new()) };
        /// Per event-test call: each loop head once, with the cycles of its
        /// first back-edge check after entering at cycle 1000.
        static KERNEL_REM_PIO2F_HEADS: std::cell::RefCell<Vec<Vec<(u32, u64)>>> =
            const { std::cell::RefCell::new(Vec::new()) };
    }

    /// Event hook that notes every back-edge check and delivers nothing.
    pub(crate) fn kernel_rem_pio2f_note_check(c: &mut Cpu) -> bool {
        assert!(
            c.cycles < crate::difftest::CYCLE_BUDGET,
            "runaway at {:#x}",
            c.pc
        );
        KERNEL_REM_PIO2F_CHECKS.with(|v| v.borrow_mut().push((c.pc, c.cycles)));
        c.next_event = 0;
        false
    }

    /// The loop heads of event-test call `base` and the cycle offset (from
    /// 1000) of each one's first back-edge check, from a spied generated
    /// run (the paths do not depend on the scrambled registers or junk).
    pub(crate) fn kernel_rem_pio2f_heads(base: u32) -> Vec<(u32, u64)> {
        let cached = KERNEL_REM_PIO2F_HEADS.with(|h| h.borrow().get(base as usize).cloned());
        if let Some(heads) = cached {
            return heads;
        }
        let mut all = Vec::new();
        for b in 0..3 {
            let mut c = crate::difftest::fresh();
            kernel_rem_pio2f_event_call(&mut c, 0, b);
            let _ = event_at(&mut c, KERNEL_REM_PIO2F_ENTRY, 0, 2);
            KERNEL_REM_PIO2F_CHECKS.with(|v| v.borrow_mut().clear());
            c.hooks.event = kernel_rem_pio2f_note_check;
            c.next_event = 0;
            KERNEL_REM_PIO2F_GENERATED(&mut c);
            let checks = KERNEL_REM_PIO2F_CHECKS.with(|v| std::mem::take(&mut *v.borrow_mut()));
            let mut heads: Vec<(u32, u64)> = Vec::new();
            for (pc, cycles) in checks {
                if !heads.iter().any(|&(h, _)| h == pc) {
                    heads.push((pc, cycles - 1000));
                }
            }
            all.push(heads);
        }
        KERNEL_REM_PIO2F_HEADS.with(|h| *h.borrow_mut() = all.clone());
        all[base as usize].clone()
    }

    /// Seeds per event-test call: two modes per targeted head.
    pub(crate) const KERNEL_REM_PIO2F_EVENT_SLOTS: u32 = 32;

    /// An event-test call with the first event due at the first back-edge
    /// check of one loop head in turn, unwinding (even slots) or not.
    pub(crate) fn kernel_rem_pio2f_events_setup(c: &mut Cpu, seed: u32) -> Events {
        let base = seed / KERNEL_REM_PIO2F_EVENT_SLOTS % 3;
        let slot = seed % KERNEL_REM_PIO2F_EVENT_SLOTS;
        let heads = kernel_rem_pio2f_heads(base);
        kernel_rem_pio2f_event_call(c, seed, base);
        let (_, at) = heads[(slot / 2) as usize % heads.len()];
        event_at(c, KERNEL_REM_PIO2F_ENTRY, at, slot % 2)
    }

    driven!(kernel_rem_pio2f_gen_driven, KERNEL_REM_PIO2F_GENERATED);

    driven!(kernel_rem_pio2f_hand_driven, kernel_rem_pio2f);

    /// `__kernel_rem_pio2f` with the first event at each of its 20 loop
    /// heads (resumes expected: none, every loop is structured).
    #[test]
    fn kernel_rem_pio2f_events() {
        use kernel_rem_pio2f_head as h;
        let seeds = 3 * KERNEL_REM_PIO2F_EVENT_SLOTS;
        check_events(
            kernel_rem_pio2f_gen_driven,
            kernel_rem_pio2f_hand_driven,
            seeds,
            &kernel_rem_pio2f_events_setup,
        );
        require_heads(
            KERNEL_REM_PIO2F_GENERATED,
            seeds,
            &[
                h::FILL_F,
                h::Q_TERM,
                h::Q_DOT,
                h::RECOMPUTE,
                h::DISTILL,
                h::CARRY,
                h::ZERO_SCAN,
                h::K_SCAN,
                h::NEW_TERM,
                h::NEW_DOT,
                h::CHOP,
                h::SCALE,
                h::FQ_TERM,
                h::FQ_DOT,
                h::SUM0,
                h::SUM12,
                h::SUM12_TAIL,
                h::FOLD3_PASS1,
                h::FOLD3_PASS2,
                h::SUM3,
            ],
            &kernel_rem_pio2f_events_setup,
        );
    }
}
