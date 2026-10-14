//! Four-lane FMAC arithmetic on AArch64 NEON, bit-identical to the scalar
//! [`float::add`](super::float::add) / [`float::mul`](super::float::mul)
//! (same integer algorithm, both paths computed and selected per lane).
//! Translated microcode spends most of its time here; one NEON op does the
//! work of four scalar lanes. Other targets use the scalar code
//! (`fast::fmac2!`). Tested against the scalar reference in `tests`.

#![allow(unused_unsafe)]

use core::arch::aarch64::*;

const SIGN: u32 = 0x8000_0000;
const ABS: u32 = 0x7fff_ffff;
const MAX: u32 = 0x7f7f_ffff;

/// Lane results: value and all-ones masks for overflow / underflow.
#[derive(Clone, Copy)]
pub struct R {
    pub v: uint32x4_t,
    pub o: uint32x4_t,
    pub u: uint32x4_t,
}

#[inline(always)]
fn k(x: u32) -> uint32x4_t {
    unsafe { vdupq_n_u32(x) }
}

#[inline(always)]
pub fn load(x: &[u32; 4]) -> uint32x4_t {
    unsafe { vld1q_u32(x.as_ptr()) }
}

#[inline(always)]
pub fn store(v: uint32x4_t) -> [u32; 4] {
    let mut o = [0u32; 4];
    unsafe { vst1q_u32(o.as_mut_ptr(), v) };
    o
}

/// Denormal -> ±0, exponent 255 -> ±max (`float::norm_in`).
#[inline(always)]
fn norm(x: uint32x4_t) -> uint32x4_t {
    unsafe {
        let e = vandq_u32(vshrq_n_u32::<23>(x), k(0xff));
        let s = vandq_u32(x, k(SIGN));
        let z = vceqq_u32(e, k(0));
        let f = vceqq_u32(e, k(0xff));
        vbslq_u32(z, s, vbslq_u32(f, vorrq_u32(s, k(MAX)), x))
    }
}

#[inline(always)]
fn mantissa(x: uint32x4_t) -> uint32x4_t {
    unsafe { vorrq_u32(vandq_u32(x, k(0x7f_ffff)), k(0x80_0000)) }
}

/// `pack` for normal-path results: clamp / flush and the O, U masks.
#[inline(always)]
fn pack(sign: uint32x4_t, e: int32x4_t, m: uint32x4_t) -> R {
    unsafe {
        let o = vcgeq_s32(e, vdupq_n_s32(255));
        let u = vcleq_s32(e, vdupq_n_s32(0));
        let eb = vshlq_n_u32::<23>(vandq_u32(vreinterpretq_u32_s32(e), k(0xff)));
        let normal = vorrq_u32(vorrq_u32(sign, eb), vandq_u32(m, k(0x7f_ffff)));
        let v = vbslq_u32(o, vorrq_u32(sign, k(MAX)), vbslq_u32(u, sign, normal));
        R { v, o, u }
    }
}

/// a * b, truncating (`float::mul`).
#[inline(always)]
pub fn mul(a: uint32x4_t, b: uint32x4_t) -> R {
    unsafe {
        let a = norm(a);
        let b = norm(b);
        let sign = vandq_u32(veorq_u32(a, b), k(SIGN));
        let ea = vandq_u32(vshrq_n_u32::<23>(a), k(0xff));
        let eb = vandq_u32(vshrq_n_u32::<23>(b), k(0xff));
        let ma = mantissa(a);
        let mb = mantissa(b);
        let plo = vmull_u32(vget_low_u32(ma), vget_low_u32(mb));
        let phi = vmull_high_u32(ma, mb);
        let tlo = vshrq_n_u64::<47>(plo);
        let thi = vshrq_n_u64::<47>(phi);
        let n23 = vdupq_n_u64(23);
        let mlo = vshlq_u64(plo, vnegq_s64(vreinterpretq_s64_u64(vaddq_u64(tlo, n23))));
        let mhi = vshlq_u64(phi, vnegq_s64(vreinterpretq_s64_u64(vaddq_u64(thi, n23))));
        let m = vuzp1q_u32(vreinterpretq_u32_u64(mlo), vreinterpretq_u32_u64(mhi));
        let top = vuzp1q_u32(vreinterpretq_u32_u64(tlo), vreinterpretq_u32_u64(thi));
        let e = vsubq_u32(vaddq_u32(vaddq_u32(ea, eb), top), k(127));
        let r = pack(sign, vreinterpretq_s32_u32(e), m);
        let zero = vorrq_u32(vceqq_u32(ea, k(0)), vceqq_u32(eb, k(0)));
        R {
            v: vbslq_u32(zero, sign, r.v),
            o: vbicq_u32(r.o, zero),
            u: vbicq_u32(r.u, zero),
        }
    }
}

/// a + b, truncating with one guard bit (`float::add`).
#[inline(always)]
pub fn add(a: uint32x4_t, b: uint32x4_t) -> R {
    unsafe {
        let a = norm(a);
        let b = norm(b);
        let aa = vandq_u32(a, k(ABS));
        let ba = vandq_u32(b, k(ABS));
        let swap = vcgtq_u32(ba, aa);
        let hi = vbslq_u32(swap, b, a);
        let ha = vbslq_u32(swap, ba, aa);
        let la = vbslq_u32(swap, aa, ba);
        let e = vshrq_n_u32::<23>(ha);
        let diff = vsubq_u32(e, vshrq_n_u32::<23>(la));
        let ma = vshlq_n_u32::<1>(mantissa(ha));
        let ml = vshlq_n_u32::<1>(mantissa(la));
        let sh = vnegq_s32(vreinterpretq_s32_u32(vminq_u32(diff, k(31))));
        let mb = vbicq_u32(vshlq_u32(ml, sh), vcgtq_u32(diff, k(25)));
        let sub = vtstq_u32(veorq_u32(a, b), k(SIGN));
        // Subtraction: renormalise left by the leading zeros.
        let ms = vsubq_u32(ma, mb);
        let lz = vsubq_u32(vclzq_u32(ms), k(7));
        let ms2 = vshlq_u32(ms, vreinterpretq_s32_u32(lz));
        // Addition: at most one carry bit.
        let mad = vaddq_u32(ma, mb);
        let carry = vshrq_n_u32::<25>(mad);
        let ma2 = vshlq_u32(mad, vnegq_s32(vreinterpretq_s32_u32(carry)));
        let m = vbslq_u32(sub, ms2, ma2);
        let ee = vbslq_u32(sub, vsubq_u32(e, lz), vaddq_u32(e, carry));
        let r = pack(
            vandq_u32(hi, k(SIGN)),
            vreinterpretq_s32_u32(ee),
            vshrq_n_u32::<1>(m),
        );
        // Zero operands and exact cancellation.
        let az = vceqq_u32(aa, k(0));
        let bz = vceqq_u32(ba, k(0));
        let cancel = vandq_u32(sub, vceqq_u32(ma, mb));
        let sp = vbslq_u32(
            bz,
            vbslq_u32(az, vandq_u32(a, b), a),
            vandq_u32(az, b),
        );
        let special = vorrq_u32(vorrq_u32(az, bz), cancel);
        R {
            v: vbslq_u32(special, sp, r.v),
            o: vbicq_u32(r.o, special),
            u: vbicq_u32(r.u, special),
        }
    }
}

#[inline(always)]
pub fn neg(x: uint32x4_t) -> uint32x4_t {
    unsafe { veorq_u32(x, k(SIGN)) }
}

/// acc + a*b (`float::madd`): O from either step, U from the add.
#[inline(always)]
pub fn madd(acc: uint32x4_t, a: uint32x4_t, b: uint32x4_t) -> R {
    let p = mul(a, b);
    let r = add(acc, p.v);
    R {
        v: r.v,
        o: unsafe { vorrq_u32(p.o, r.o) },
        u: r.u,
    }
}

/// acc - a*b (`float::msub`).
#[inline(always)]
pub fn msub(acc: uint32x4_t, a: uint32x4_t, b: uint32x4_t) -> R {
    let p = mul(a, b);
    let r = add(acc, neg(p.v));
    R {
        v: r.v,
        o: unsafe { vorrq_u32(p.o, r.o) },
        u: r.u,
    }
}

/// MAC flags (Z S U O nibbles, x = high bit) of the lanes in `D`.
#[inline(always)]
pub fn mac<const D: u8>(r: &R) -> u32 {
    unsafe {
        let lane = |x: u32, y: u32, z: u32, w: u32| -> uint32x4_t {
            let a = [x, y, z, w];
            vld1q_u32(a.as_ptr())
        };
        let z = vceqq_u32(vandq_u32(r.v, k(ABS)), k(0));
        let s = vtstq_u32(r.v, k(SIGN));
        let bits = vorrq_u32(
            vorrq_u32(
                vandq_u32(z, lane(0x8, 0x4, 0x2, 0x1)),
                vandq_u32(s, lane(0x80, 0x40, 0x20, 0x10)),
            ),
            vorrq_u32(
                vandq_u32(r.u, lane(0x800, 0x400, 0x200, 0x100)),
                vandq_u32(r.o, lane(0x8000, 0x4000, 0x2000, 0x1000)),
            ),
        );
        let m = |b: u8| if D & b != 0 { u32::MAX } else { 0 };
        let dm = lane(m(8), m(4), m(2), m(1));
        vaddvq_u32(vandq_u32(bits, dm))
    }
}

#[cfg(test)]
mod tests {
    use super::super::float;
    use super::*;

    #[test]
    fn neon_matches_scalar() {
        let mut x: u64 = 0x2545_f491_4f6c_dd1d;
        let mut next = move || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x
        };
        let mut word = |near: u32| -> u32 {
            let r = next();
            let sign = (r as u32 & 1) << 31;
            let exp = match (r >> 1) % 8 {
                0 => 0,
                1 => 255,
                2 => (r >> 8) as u32 % 4,
                3 => 251 + (r >> 8) as u32 % 5,
                _ => (near as i32 + ((r >> 8) % 61) as i32 - 30).clamp(0, 255) as u32,
            };
            let man = match (r >> 16) % 4 {
                0 => 0,
                1 => 0x7f_ffff,
                _ => (r >> 24) as u32 & 0x7f_ffff,
            };
            if (r >> 40) % 5 == 0 {
                (r >> 3) as u32
            } else {
                sign | exp << 23 | man
            }
        };
        let iters: u64 = std::env::var("SSX3_FLOAT_ITERS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(200_000);
        for i in 0..iters {
            let near = (i % 256) as u32;
            let a: [u32; 4] = std::array::from_fn(|_| word(near));
            let b: [u32; 4] = std::array::from_fn(|l| if i % 7 == 0 { a[l] ^ SIGN } else { word(near) });
            let c: [u32; 4] = std::array::from_fn(|_| word(near));
            let check = |name: &str, r: R, f: &dyn Fn(usize) -> (u32, float::Exc)| {
                let v = store(r.v);
                let o = store(r.o);
                let u = store(r.u);
                for l in 0..4 {
                    let (ev, ee) = f(l);
                    assert_eq!(
                        (v[l], o[l] != 0, u[l] != 0),
                        (ev, ee.o, ee.u),
                        "{name} lane {l}: {:#010x} {:#010x} {:#010x}",
                        a[l],
                        b[l],
                        c[l]
                    );
                }
            };
            let (va, vb, vc) = (load(&a), load(&b), load(&c));
            check("add", add(va, vb), &|l| float::add_ref(a[l], b[l]));
            check("mul", mul(va, vb), &|l| float::mul_ref(a[l], b[l]));
            check("madd", madd(vc, va, vb), &|l| float::madd(c[l], a[l], b[l]));
            check("msub", msub(vc, va, vb), &|l| float::msub(c[l], a[l], b[l]));
            let r = madd(vc, va, vb);
            let v = store(r.v);
            let mut want = 0u32;
            for l in 0..4u32 {
                if 0b1011 & (8 >> l) != 0 {
                    let (x, e) = float::madd(c[l as usize], a[l as usize], b[l as usize]);
                    assert_eq!(x, v[l as usize]);
                    let bb = 3 - l;
                    want |= ((x & ABS == 0) as u32) << bb
                        | ((x & SIGN != 0) as u32) << (bb + 4)
                        | (e.u as u32) << (bb + 8)
                        | (e.o as u32) << (bb + 12);
                }
            }
            assert_eq!(mac::<0b1011>(&r), want);
        }
    }
}
