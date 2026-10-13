//! PS2 VU single-precision arithmetic on raw IEEE bits.
//!
//! The VU has no Inf/NaN/denormals: denormal inputs are read as ±0, inputs
//! with exponent 255 are read as ±max (`0x7f7fffff`), results that overflow
//! clamp to ±max (O flag) and results that underflow flush to ±0 (U flag).
//! Rounding is toward zero (the FMAC keeps a single guard bit and no sticky
//! bit, which is what the integer add below reproduces).
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

/// a + b, truncating.
pub fn add(a: u32, b: u32) -> (u32, Exc) {
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

pub fn sub(a: u32, b: u32) -> (u32, Exc) {
    add(a, b ^ SIGN)
}

/// a * b, truncating.
pub fn mul(a: u32, b: u32) -> (u32, Exc) {
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

/// acc + a*b (product truncated first, as the FMAC does).
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

#[inline]
fn key(x: u32) -> i32 {
    let x = norm_in(x);
    let m = (x & ABS) as i32;
    if x & SIGN != 0 {
        -m
    } else {
        m
    }
}

pub fn max(a: u32, b: u32) -> u32 {
    if key(a) >= key(b) {
        norm_in(a)
    } else {
        norm_in(b)
    }
}

pub fn min(a: u32, b: u32) -> u32 {
    if key(a) <= key(b) {
        norm_in(a)
    } else {
        norm_in(b)
    }
}

/// FTOIn: truncate toward zero, saturate to i32.
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

pub fn to_f64(x: u32) -> f64 {
    let x = norm_in(x);
    f32::from_bits(x) as f64
}

/// ITOFn.
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
pub fn efu(v: f64) -> u32 {
    if v.is_nan() {
        return MAX;
    }
    if v.is_infinite() {
        return if v < 0.0 { SIGN | MAX } else { MAX };
    }
    from_f64(v)
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
}
