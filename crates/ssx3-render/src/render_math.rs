//! The float arithmetic of the CPU-side renderer: PS2-exact (the default,
//! always for `SSX3_GPU_VU=0`, the bit-exact reference), or host `f32` when
//! `SSX3_FAST_MATH=1` (requires the GPU path).
//!
//! Host float is only output-side: renderer depth feeds the flare queries, so
//! guest RAM is checked per scene. Matrix composition always stays exact
//! ([`fast::exact_xform`], [`fast::exact_xform3`], [`fast::exact_screen_col`]):
//! one different last bit in `M × model` or `M × bone` tilts a whole mesh's
//! depth plane and flips the order of coplanar layers over a wide area, while
//! per-vertex and per-particle work only moves single pixels.

use std::sync::OnceLock;

// Read once per process: the overlay toggle applies after a restart, so
// there is deliberately no refresh hook.
static FAST_MATH: OnceLock<bool> = OnceLock::new();

#[cold]
fn read_setting() -> bool {
    let var = ssx3_core::config::var("SSX3_FAST_MATH");
    var.is_some_and(|v| !v.is_empty() && v != "0") && crate::gpu::enabled()
}

/// Whether the renderer's float paths use host `f32`.
#[inline(always)]
pub fn on() -> bool {
    *FAST_MATH.get_or_init(read_setting)
}

/// Four PS2 float (or integer) lanes as raw bits, x..w.
type Vec4Bits = [u32; 4];

#[inline(always)]
fn as_f32(x: u32) -> f32 {
    f32::from_bits(x)
}

/// The PS2-exact helpers, re-exported: [`exact::fast`] and
/// [`exact::float`] are `ssx3_vu::vu`'s helpers under those names.
pub mod exact {
    pub use ssx3_vu::vu::{fast, float};
}

// One plain-Rust version for every architecture (no `unsafe`): the lanes
// are `[f32; 4]` values, which the release compiler vectorises on aarch64.
// `std::simd` is the intended replacement once it is stable (checked
// 2026-11-12: still nightly-only on rustc 1.99.1).
pub mod host {
    use super::{as_f32, Vec4Bits};

    #[inline(always)]
    fn to_lanes(v: &Vec4Bits) -> [f32; 4] {
        std::array::from_fn(|l| as_f32(v[l]))
    }
    #[inline(always)]
    fn from_lanes(v: [f32; 4]) -> Vec4Bits {
        std::array::from_fn(|l| v[l].to_bits())
    }
    #[inline(always)]
    pub fn add(a: &Vec4Bits, b: &Vec4Bits) -> Vec4Bits {
        let (a, b) = (to_lanes(a), to_lanes(b));
        from_lanes(std::array::from_fn(|l| a[l] + b[l]))
    }
    #[inline(always)]
    pub fn sub(a: &Vec4Bits, b: &Vec4Bits) -> Vec4Bits {
        let (a, b) = (to_lanes(a), to_lanes(b));
        from_lanes(std::array::from_fn(|l| a[l] - b[l]))
    }
    #[inline(always)]
    pub fn mul(a: &Vec4Bits, b: &Vec4Bits) -> Vec4Bits {
        let (a, b) = (to_lanes(a), to_lanes(b));
        from_lanes(std::array::from_fn(|l| a[l] * b[l]))
    }
    #[inline(always)]
    pub fn madd(acc: &Vec4Bits, a: &Vec4Bits, b: &Vec4Bits) -> Vec4Bits {
        let (acc, a, b) = (to_lanes(acc), to_lanes(a), to_lanes(b));
        from_lanes(std::array::from_fn(|l| acc[l] + a[l] * b[l]))
    }
    #[inline(always)]
    pub fn msub(acc: &Vec4Bits, a: &Vec4Bits, b: &Vec4Bits) -> Vec4Bits {
        let (acc, a, b) = (to_lanes(acc), to_lanes(a), to_lanes(b));
        from_lanes(std::array::from_fn(|l| acc[l] - a[l] * b[l]))
    }
    #[inline(always)]
    pub fn itof<const FRAC_BITS: i32>(s: &Vec4Bits) -> Vec4Bits {
        let scale = 1.0 / (1u64 << FRAC_BITS) as f32;
        from_lanes(std::array::from_fn(|l| s[l] as i32 as f32 * scale))
    }
    #[inline(always)]
    pub fn ftoi<const FRAC_BITS: i32>(s: &Vec4Bits) -> Vec4Bits {
        let scale = (1u64 << FRAC_BITS) as f32;
        let v = to_lanes(s);
        std::array::from_fn(|l| (v[l] * scale) as i32 as u32)
    }
    #[inline(always)]
    pub fn add_scalar(a: u32, b: u32) -> u32 {
        (as_f32(a) + as_f32(b)).to_bits()
    }
    #[inline(always)]
    pub fn sub_scalar(a: u32, b: u32) -> u32 {
        (as_f32(a) - as_f32(b)).to_bits()
    }
    #[inline(always)]
    pub fn mul_scalar(a: u32, b: u32) -> u32 {
        (as_f32(a) * as_f32(b)).to_bits()
    }
    #[inline(always)]
    pub fn madd_scalar(acc: u32, a: u32, b: u32) -> u32 {
        (as_f32(acc) + as_f32(a) * as_f32(b)).to_bits()
    }
    #[inline(always)]
    pub fn div_scalar(a: u32, b: u32) -> u32 {
        (as_f32(a) / as_f32(b)).to_bits()
    }
}

/// The vector helpers the renderer calls, under the same names as
/// `ssx3_vu::vu::fast`: [`exact::fast`] or [`host`], by [`super::on`].
pub mod fast {
    use super::exact::fast as exact;
    use super::{host, on, Vec4Bits};
    pub use exact::{abs4, clip, max4, min4};

    // The host path computes all four lanes and ignores the dest mask and
    // the operand flags; every caller passes the full mask (0b1111) and
    // both operand flags set, so nothing is silently dropped there.
    #[inline(always)]
    pub fn add4<const D: u8, const NS: bool, const NT: bool>(
        s: &Vec4Bits,
        t: &Vec4Bits,
    ) -> (Vec4Bits, u32) {
        if on() {
            debug_assert!(D == 0b1111 && NS && NT);
            return (host::add(s, t), 0);
        }
        exact::add4::<D, NS, NT>(s, t)
    }
    #[inline(always)]
    pub fn sub4<const D: u8, const NS: bool, const NT: bool>(
        s: &Vec4Bits,
        t: &Vec4Bits,
    ) -> (Vec4Bits, u32) {
        if on() {
            debug_assert!(D == 0b1111 && NS && NT);
            return (host::sub(s, t), 0);
        }
        exact::sub4::<D, NS, NT>(s, t)
    }
    #[inline(always)]
    pub fn mul4<const D: u8, const NS: bool, const NT: bool>(
        s: &Vec4Bits,
        t: &Vec4Bits,
    ) -> (Vec4Bits, u32) {
        if on() {
            debug_assert!(D == 0b1111 && NS && NT);
            return (host::mul(s, t), 0);
        }
        exact::mul4::<D, NS, NT>(s, t)
    }
    #[inline(always)]
    pub fn madd4<const D: u8, const NA: bool, const NS: bool, const NT: bool>(
        a: &Vec4Bits,
        s: &Vec4Bits,
        t: &Vec4Bits,
    ) -> (Vec4Bits, u32) {
        if on() {
            debug_assert!(D == 0b1111 && NA && NS && NT);
            return (host::madd(a, s, t), 0);
        }
        exact::madd4::<D, NA, NS, NT>(a, s, t)
    }
    #[inline(always)]
    pub fn msub4<const D: u8, const NA: bool, const NS: bool, const NT: bool>(
        a: &Vec4Bits,
        s: &Vec4Bits,
        t: &Vec4Bits,
    ) -> (Vec4Bits, u32) {
        if on() {
            debug_assert!(D == 0b1111 && NA && NS && NT);
            return (host::msub(a, s, t), 0);
        }
        exact::msub4::<D, NA, NS, NT>(a, s, t)
    }
    #[inline(always)]
    pub fn itof4<const FRAC_BITS: i32>(s: &Vec4Bits) -> Vec4Bits {
        if on() {
            return host::itof::<FRAC_BITS>(s);
        }
        exact::itof4::<FRAC_BITS>(s)
    }
    #[inline(always)]
    pub fn ftoi4<const FRAC_BITS: i32>(s: &Vec4Bits) -> Vec4Bits {
        if on() {
            return host::ftoi::<FRAC_BITS>(s);
        }
        exact::ftoi4::<FRAC_BITS>(s)
    }

    /// `M × p` as MULAx / MADDAy / MADDAz / MADDw, always exact.
    /// Same bits as `mat3::xform_xyzw` / `mat6::xform` with `w = p[3]`.
    #[inline(always)]
    pub fn exact_xform(m: &[Vec4Bits; 4], p: &Vec4Bits) -> Vec4Bits {
        let acc = exact::mul4::<0b1111, true, true>(&m[0], &[p[0]; 4]).0;
        let acc = exact::madd4::<0b1111, true, true, true>(&acc, &m[1], &[p[1]; 4]).0;
        let acc = exact::madd4::<0b1111, true, true, true>(&acc, &m[2], &[p[2]; 4]).0;
        exact::madd4::<0b1111, true, true, true>(&acc, &m[3], &[p[3]; 4]).0
    }
    /// The skeleton's env rotation `M × v` (3 columns), always exact.
    #[inline(always)]
    pub fn exact_xform3(m: &[Vec4Bits; 3], p: &Vec4Bits) -> Vec4Bits {
        let acc = exact::mul4::<0b1111, true, true>(&m[0], &[p[0]; 4]).0;
        let acc = exact::madd4::<0b1111, true, true, true>(&acc, &m[1], &[p[1]; 4]).0;
        exact::madd4::<0b1111, true, true, true>(&acc, &m[2], &[p[2]; 4]).0
    }
    /// The mesh screen matrix's column `guardScale·c + guardOffset·c.w`, always exact.
    #[inline(always)]
    pub fn exact_screen_col(
        guard_scale: Vec4Bits,
        c: Vec4Bits,
        guard_offset: Vec4Bits,
    ) -> Vec4Bits {
        let a = exact::mul4::<0b1111, true, true>(&guard_scale, &c).0;
        exact::madd4::<0b1111, true, true, true>(&a, &guard_offset, &[c[3]; 4]).0
    }
}

/// The scalar helpers the renderer calls, under the same names as
/// `ssx3_vu::vu::float`: [`exact::float`] or [`host`], by [`super::on`].
pub mod float {
    use super::exact::float as exact;
    use super::{host, on};
    pub use exact::{efu, rsqrt, to_f64, DivResult, Exc};

    #[inline(always)]
    pub fn add(a: u32, b: u32) -> (u32, Exc) {
        if on() {
            return (host::add_scalar(a, b), Exc::default());
        }
        exact::add(a, b)
    }
    #[inline(always)]
    pub fn sub(a: u32, b: u32) -> (u32, Exc) {
        if on() {
            return (host::sub_scalar(a, b), Exc::default());
        }
        exact::sub(a, b)
    }
    #[inline(always)]
    pub fn mul(a: u32, b: u32) -> (u32, Exc) {
        if on() {
            return (host::mul_scalar(a, b), Exc::default());
        }
        exact::mul(a, b)
    }
    #[inline(always)]
    pub fn madd(acc: u32, a: u32, b: u32) -> (u32, Exc) {
        if on() {
            return (host::madd_scalar(acc, a, b), Exc::default());
        }
        exact::madd(acc, a, b)
    }

    /// The divisor's exponent field (bits 23..30).
    const EXPONENT_MASK: u32 = 0xff;
    const EXPONENT_SHIFT: u32 = 23;

    // Zero and subnormal divisors stay on the exact path: host division
    // by them would produce an infinity or NaN whose bits and flags the
    // PS2 DIV unit does not produce, so only normal divisors go fast.
    #[inline(always)]
    fn divisor_is_normal(b: u32) -> bool {
        (b >> EXPONENT_SHIFT) & EXPONENT_MASK != 0
    }

    #[inline(always)]
    pub fn div(a: u32, b: u32) -> DivResult {
        if on() && divisor_is_normal(b) {
            return DivResult {
                value: host::div_scalar(a, b),
                invalid: false,
                div0: false,
            };
        }
        exact::div(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::{fast, host, Vec4Bits};

    fn bits(v: [f32; 4]) -> Vec4Bits {
        std::array::from_fn(|l| v[l].to_bits())
    }

    #[test]
    fn host_ops_match_host_f32() {
        let a = bits([1.5, -2.0, 0.25, 100.0]);
        let b = bits([2.25, 0.5, 4.0, -1.0]);
        assert_eq!(host::add(&a, &b), bits([3.75, -1.5, 4.25, 99.0]));
        assert_eq!(host::sub(&a, &b), bits([-0.75, -2.5, -3.75, 101.0]));
        assert_eq!(host::mul(&a, &b), bits([3.375, -1.0, 1.0, -100.0]));
        assert_eq!(host::madd(&a, &a, &b), bits([4.875, -3.0, 1.25, 0.0]));
        assert_eq!(host::msub(&a, &a, &b), bits([-1.875, -1.0, -0.75, 200.0]));
        // itof4/ftoi4 with 4 fractional bits: scale 1/16 and 16.
        assert_eq!(
            host::itof::<4>(&[16, (-32i32) as u32, 8, 0]),
            bits([1.0, -2.0, 0.5, 0.0])
        );
        assert_eq!(
            host::ftoi::<4>(&bits([1.5, -0.25, 100.0, 0.0])),
            [24, (-4i32) as u32, 1600, 0]
        );
        // NaN saturates/maps like the scalar version (as i32 gives 0).
        assert_eq!(
            host::ftoi::<0>(&bits([f32::NAN, 1.0, -1.0, 0.5])),
            [0, 1, (-1i32) as u32, 0]
        );
    }

    #[test]
    fn exact_helpers_match_the_exact_vu_sequence() {
        use super::exact::fast as exact;
        let m: [Vec4Bits; 4] = [
            bits([1.0, 2.0, 0.5, -1.0]),
            bits([0.25, -0.5, 2.0, 1.0]),
            bits([2.0, 1.0, -0.25, 0.5]),
            bits([-1.0, 0.5, 1.0, 2.0]),
        ];
        let p = bits([0.5, -2.0, 1.0, 4.0]);
        let want = {
            let acc = exact::mul4::<0b1111, true, true>(&m[0], &[p[0]; 4]).0;
            let acc = exact::madd4::<0b1111, true, true, true>(&acc, &m[1], &[p[1]; 4]).0;
            let acc = exact::madd4::<0b1111, true, true, true>(&acc, &m[2], &[p[2]; 4]).0;
            exact::madd4::<0b1111, true, true, true>(&acc, &m[3], &[p[3]; 4]).0
        };
        assert_eq!(fast::exact_xform(&m, &p), want);
        let m3: [Vec4Bits; 3] = [m[0], m[1], m[2]];
        let want3 = {
            let acc = exact::mul4::<0b1111, true, true>(&m3[0], &[p[0]; 4]).0;
            let acc = exact::madd4::<0b1111, true, true, true>(&acc, &m3[1], &[p[1]; 4]).0;
            exact::madd4::<0b1111, true, true, true>(&acc, &m3[2], &[p[2]; 4]).0
        };
        assert_eq!(fast::exact_xform3(&m3, &p), want3);
        let (gs, c, go) = (
            bits([2.0, 0.5, -1.0, 1.0]),
            m[1],
            bits([10.0, 20.0, 30.0, 40.0]),
        );
        let want_col = {
            let a = exact::mul4::<0b1111, true, true>(&gs, &c).0;
            exact::madd4::<0b1111, true, true, true>(&a, &go, &[c[3]; 4]).0
        };
        assert_eq!(fast::exact_screen_col(gs, c, go), want_col);
    }
}
