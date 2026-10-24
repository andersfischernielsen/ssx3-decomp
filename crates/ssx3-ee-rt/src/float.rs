//! PS2 VU single-precision arithmetic on raw IEEE bits.
//!
//! The VU has no Inf/NaN/denormals: denormal inputs are read as ±0, inputs
//! with exponent 255 are read as ±max (`0x7f7fffff`), results that overflow
//! clamp to ±max (O flag) and results that underflow flush to ±0 (U flag).
//! Rounding is toward zero (the FMAC keeps a single guard bit and no sticky
//! bit, which is what the integer add below reproduces).
//!
//! Lives in the EE core (`ssx3-ee-rt`) because the FPU uses it too; the VU
//! (in `ssx3-ps2`) calls it across the crate boundary, so everything on a hot
//! path is `#[inline]`.
//!
//! The integer add/mul and fixed-point conversions are ported from
//! `ps2-web-recomp` (`runtime/include/ps2/fpu.hpp`, MIT licence,
//! https://github.com/snuri00/ps2-web-recomp), adapted to report flags.

pub const SIGN: u32 = 0x8000_0000;
pub const ABS: u32 = 0x7fff_ffff;
pub const MAX: u32 = 0x7f7f_ffff;
pub const ONE: u32 = 0x3f80_0000;

/// Overflow / underflow produced by one lane of an FMAC operation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Exc {
    pub o: bool,
    pub u: bool,
}

#[inline]
fn exponent(x: u32) -> u32 {
    (x >> 23) & 0xff
}

#[inline]
fn mantissa(x: u32) -> u32 {
    (x & 0x7f_ffff) | 0x80_0000
}

/// Normalise an input operand: denormal -> ±0, exponent 255 -> ±max.
#[inline]
pub fn norm_in(x: u32) -> u32 {
    match exponent(x) {
        0 => x & SIGN,
        255 => (x & SIGN) | MAX,
        _ => x,
    }
}

#[inline]
fn pack(sign: u32, e: i32, m: u32, exc: &mut Exc) -> u32 {
    if e >= 255 {
        exc.o = true;
        sign | MAX
    } else if e <= 0 {
        exc.u = true;
        sign
    } else {
        sign | ((e as u32) << 23) | (m & 0x7f_ffff)
    }
}

/// a + b, truncating (one guard bit, no sticky bit).
///
/// Branch-free form of [`add_ref`] (the straightforward port, kept as the
/// test reference): both paths are computed and the result selected, so
/// the hot FMAC loop has no data-dependent branches.
#[cfg_attr(not(debug_assertions), inline(always))]
pub fn add(a: u32, b: u32) -> (u32, Exc) {
    let a = norm_in(a);
    let b = norm_in(b);
    let aa = a & ABS;
    let ba = b & ABS;
    // Larger magnitude first.
    let swap = ba > aa;
    let hi = if swap { b } else { a };
    let lo = if swap { a } else { b };
    let ha = hi & ABS;
    let la = lo & ABS;
    let e = (ha >> 23) as i32;
    let diff = e - (la >> 23) as i32;
    let ma = mantissa(ha) << 1;
    let ml = mantissa(la) << 1;
    let mb = if diff > 25 {
        0
    } else {
        ml >> (diff as u32 & 31)
    };
    let sub = (a ^ b) & SIGN != 0;
    // Subtraction: renormalise left.
    let ms = ma - mb;
    let lz = ms.leading_zeros() as i32 - 7;
    let ms = ms << (lz as u32 & 31);
    // Addition: one carry bit.
    let mad = ma + mb;
    let carry = (mad >> 25) & 1;
    let (m, e) = if sub {
        (ms, e - lz)
    } else {
        (mad >> carry, e + carry as i32)
    };
    let sign = hi & SIGN;
    let o = e >= 255;
    let u = e <= 0;
    let normal = sign | (((e as u32) & 0xff) << 23) | ((m >> 1) & 0x7f_ffff);
    let packed = if o {
        sign | MAX
    } else if u {
        sign
    } else {
        normal
    };
    // Exact cancellation and zero operands bypass rounding and flags.
    let cancel = sub && ma == mb;
    let special = ba == 0 || aa == 0 || cancel;
    let sp = if ba == 0 {
        if aa != 0 {
            a
        } else {
            a & b
        }
    } else if aa == 0 {
        b
    } else {
        0
    };
    if special {
        (sp, Exc::default())
    } else {
        (packed, Exc { o, u })
    }
}

#[cfg_attr(not(debug_assertions), inline(always))]
pub fn sub(a: u32, b: u32) -> (u32, Exc) {
    add(a, b ^ SIGN)
}

/// a * b, truncating. Branch-free form of [`mul_ref`].
#[cfg_attr(not(debug_assertions), inline(always))]
pub fn mul(a: u32, b: u32) -> (u32, Exc) {
    let a = norm_in(a);
    let b = norm_in(b);
    let sign = (a ^ b) & SIGN;
    let ea = exponent(a) as i32;
    let eb = exponent(b) as i32;
    let m = mantissa(a) as u64 * mantissa(b) as u64;
    let top = (m >> 47) as u32 & 1;
    let m = (m >> (23 + top)) as u32;
    let e = ea + eb - 127 + top as i32;
    let o = e >= 255;
    let u = e <= 0;
    let normal = sign | (((e as u32) & 0xff) << 23) | (m & 0x7f_ffff);
    let packed = if o {
        sign | MAX
    } else if u {
        sign
    } else {
        normal
    };
    if ea == 0 || eb == 0 {
        (sign, Exc::default())
    } else {
        (packed, Exc { o, u })
    }
}

/// acc + a*b (product truncated first, as the FMAC does).
#[cfg_attr(not(debug_assertions), inline(always))]
pub fn madd(acc: u32, a: u32, b: u32) -> (u32, Exc) {
    let (p, e1) = mul(a, b);
    let (r, e2) = add(acc, p);
    (
        r,
        Exc {
            o: e1.o | e2.o,
            u: e2.u,
        },
    )
}

/// acc - a*b.
#[cfg_attr(not(debug_assertions), inline(always))]
pub fn msub(acc: u32, a: u32, b: u32) -> (u32, Exc) {
    let (p, e1) = mul(a, b);
    let (r, e2) = sub(acc, p);
    (
        r,
        Exc {
            o: e1.o | e2.o,
            u: e2.u,
        },
    )
}

/// Raw sign-magnitude order of the bit pattern (no denormal flush, no
/// exponent-255 clamp): PCSX2 microVU `MIN_MAX_PS` integer compare.
#[inline]
fn raw_key(x: u32) -> i32 {
    let x = x as i32;
    x ^ (((x >> 31) as u32) >> 1) as i32
}

/// MAX: compares and returns the *raw* operand bits, so denormals (e.g.
/// integer colour data moved with `MAX.xyzw vfX, vfY, vfY`) pass through
/// unchanged (PCSX2 microVU `MIN_MAX_PS`; the VU does not flush here).
#[inline]
pub fn max(a: u32, b: u32) -> u32 {
    if raw_key(a) > raw_key(b) {
        a
    } else {
        b
    }
}

/// MINI: like [`max`], raw bits.
#[inline]
pub fn min(a: u32, b: u32) -> u32 {
    if raw_key(b) > raw_key(a) {
        a
    } else {
        b
    }
}

/// FTOIn: truncate toward zero, saturate to i32.
#[inline]
pub fn to_fixed(x: u32, fraction: i32) -> u32 {
    let x = norm_in(x);
    if exponent(x) == 0 {
        return 0;
    }
    let e = exponent(x) as i32 + fraction;
    if e <= 126 {
        return 0;
    }
    let negative = x & SIGN != 0;
    if e >= 158 {
        return if negative { 0x8000_0000 } else { 0x7fff_ffff };
    }
    let shift = e - 150;
    let m = mantissa(x);
    let v = if shift >= 0 { m << shift } else { m >> -shift };
    if negative {
        v.wrapping_neg()
    } else {
        v
    }
}

/// Convert an f64 to PS2 float bits, truncating the mantissa.
#[inline]
pub fn from_f64(d: f64) -> u32 {
    let bits = d.to_bits();
    let sign = ((bits >> 32) as u32) & SIGN;
    let e = ((bits >> 52) & 0x7ff) as i32;
    if e == 0 {
        return sign;
    }
    let mut exc = Exc::default();
    pack(sign, e - 1023 + 127, (bits >> 29) as u32, &mut exc)
}

#[inline]
pub fn to_f64(x: u32) -> f64 {
    let x = norm_in(x);
    f32::from_bits(x) as f64
}

/// ITOFn.
#[inline]
pub fn from_fixed(v: i32, fraction: i32) -> u32 {
    from_f64(v as f64 / (1u64 << fraction) as f64)
}

/// Result of a DIV-unit operation: value plus invalid (I) / divide-by-zero (D).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DivResult {
    pub value: u32,
    pub invalid: bool,
    pub div0: bool,
}

#[inline]
pub fn div(a: u32, b: u32) -> DivResult {
    let a = norm_in(a);
    let b = norm_in(b);
    let sign = (a ^ b) & SIGN;
    if b & ABS == 0 {
        let invalid = a & ABS == 0;
        return DivResult {
            value: sign | MAX,
            invalid,
            div0: !invalid,
        };
    }
    if a & ABS == 0 {
        return DivResult {
            value: sign,
            ..Default::default()
        };
    }
    DivResult {
        value: from_f64(to_f64(a) / to_f64(b)),
        ..Default::default()
    }
}

#[inline]
pub fn sqrt(b: u32) -> DivResult {
    let b = norm_in(b);
    let invalid = b & SIGN != 0 && b & ABS != 0;
    let m = b & ABS;
    let value = if m == 0 {
        0
    } else {
        from_f64(to_f64(m).sqrt())
    };
    DivResult {
        value,
        invalid,
        div0: false,
    }
}

#[inline]
pub fn rsqrt(a: u32, b: u32) -> DivResult {
    let a = norm_in(a);
    let b = norm_in(b);
    let neg = b & SIGN != 0 && b & ABS != 0;
    let bm = b & ABS;
    if bm == 0 {
        let invalid = a & ABS == 0;
        return DivResult {
            value: (a & SIGN) | MAX,
            invalid,
            div0: !invalid,
        };
    }
    if a & ABS == 0 {
        return DivResult {
            value: a & SIGN,
            invalid: neg,
            div0: false,
        };
    }
    DivResult {
        value: from_f64(to_f64(a) / to_f64(bm).sqrt()),
        invalid: neg,
        div0: false,
    }
}

/// Generic EFU result conversion (results are clamped / flushed).
#[inline]
pub fn efu(v: f64) -> u32 {
    if v.is_nan() {
        return MAX;
    }
    if v.is_infinite() {
        return if v < 0.0 { SIGN | MAX } else { MAX };
    }
    from_f64(v)
}

// ---- reference implementations (differential tests) ----

#[doc(hidden)]
/// a + b, truncating (straightforward port; reference for [`add`]).
pub fn add_ref(a: u32, b: u32) -> (u32, Exc) {
    let mut exc = Exc::default();
    let mut a = norm_in(a);
    let mut b = norm_in(b);
    let mut aa = a & ABS;
    let mut ba = b & ABS;
    if ba == 0 {
        return (if aa != 0 { a } else { a & b }, exc);
    }
    if aa == 0 {
        return (b, exc);
    }
    if ba > aa {
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut aa, &mut ba);
    }
    let mut e = (aa >> 23) as i32;
    let diff = e - (ba >> 23) as i32;
    let mut ma = mantissa(aa) << 1;
    let mb = if diff > 25 {
        0
    } else {
        (mantissa(ba) << 1) >> diff
    };
    if (a ^ b) & SIGN != 0 {
        ma -= mb;
        if ma == 0 {
            return (0, exc);
        }
        let shift = ma.leading_zeros() as i32 - 7;
        ma <<= shift;
        e -= shift;
    } else {
        ma += mb;
        if ma & 0x200_0000 != 0 {
            ma >>= 1;
            e += 1;
        }
    }
    let r = pack(a & SIGN, e, ma >> 1, &mut exc);
    (r, exc)
}

#[doc(hidden)]
/// a * b, truncating (reference for [`mul`]).
pub fn mul_ref(a: u32, b: u32) -> (u32, Exc) {
    let mut exc = Exc::default();
    let a = norm_in(a);
    let b = norm_in(b);
    let sign = (a ^ b) & SIGN;
    let ea = exponent(a) as i32;
    let eb = exponent(b) as i32;
    if ea == 0 || eb == 0 {
        return (sign, exc);
    }
    let mut m = mantissa(a) as u64 * mantissa(b) as u64;
    let mut e = ea + eb - 127;
    if m >> 47 != 0 {
        m >>= 24;
        e += 1;
    } else {
        m >>= 23;
    }
    let r = pack(sign, e, m as u32, &mut exc);
    (r, exc)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    #[test]
    fn basic_arith() {
        assert_eq!(add(f(1.5), f(2.25)).0, f(3.75));
        assert_eq!(sub(f(1.0), f(1.0)).0, 0);
        assert_eq!(mul(f(3.0), f(-2.0)).0, f(-6.0));
        assert_eq!(madd(f(1.0), f(2.0), f(3.0)).0, f(7.0));
        assert_eq!(msub(f(1.0), f(2.0), f(3.0)).0, f(-5.0));
    }

    #[test]
    fn clamps_and_flushes() {
        let (r, e) = mul(MAX, f(2.0));
        assert_eq!(r, MAX);
        assert!(e.o);
        let (r, e) = mul(f(1e-30), f(-1e-30));
        assert_eq!(r, SIGN);
        assert!(e.u);
        // Exponent-255 inputs read as max.
        assert_eq!(add(0x7f80_0000, 0).0, MAX);
        // Denormal inputs read as zero.
        assert_eq!(add(1, f(1.0)).0, f(1.0));
    }

    #[test]
    fn fixed() {
        assert_eq!(to_fixed(f(1.75), 0), 1);
        assert_eq!(to_fixed(f(-1.75), 0), (-1i32) as u32);
        assert_eq!(to_fixed(f(1.75), 4), 28);
        assert_eq!(to_fixed(f(1e20), 0), 0x7fff_ffff);
        assert_eq!(to_fixed(f(-1e20), 0), 0x8000_0000);
        assert_eq!(from_fixed(28, 4), f(1.75));
        assert_eq!(from_fixed(-3, 0), f(-3.0));
    }

    /// The branch-free `add`/`mul` against the straightforward ports, on
    /// random words and on operands built to hit every path (equal and
    /// nearby exponents, cancellation, zeros, denormals, exponent 255,
    /// overflow and underflow). SSX3_FLOAT_ITERS raises the count.
    #[test]
    fn branch_free_matches_reference() {
        let iters: u64 = std::env::var("SSX3_FLOAT_ITERS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(400_000);
        let mut x: u64 = 0x9e37_79b9_7f4a_7c15;
        let mut next = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        let word = |r: u64, near: u32| -> u32 {
            let sign = ((r >> 1) as u32 & 1) << 31;
            let exp = match r % 8 {
                0 => 0,
                1 => 255,
                2 => 1 + (r >> 8) as u32 % 3,
                3 => 252 + (r >> 8) as u32 % 3,
                _ => (near as i32 + ((r >> 8) % 61) as i32 - 30).clamp(0, 255) as u32,
            };
            let man = match (r >> 16) % 4 {
                0 => 0,
                1 => 0x7f_ffff,
                _ => (r >> 24) as u32 & 0x7f_ffff,
            };
            sign | exp << 23 | man
        };
        for i in 0..iters {
            let (r1, r2, r3) = (next(), next(), next());
            let (a, b) = if i % 4 == 0 {
                (r1 as u32, r2 as u32)
            } else {
                let near = (r3 % 256) as u32;
                (word(r1, near), word(r2, near))
            };
            assert_eq!(add(a, b), add_ref(a, b), "add {a:#010x} {b:#010x}");
            assert_eq!(mul(a, b), mul_ref(a, b), "mul {a:#010x} {b:#010x}");
            // Exact cancellation and sign-of-zero cases.
            assert_eq!(add(a, a ^ SIGN), add_ref(a, a ^ SIGN));
            assert_eq!(add(a, a), add_ref(a, a));
        }
    }
}
