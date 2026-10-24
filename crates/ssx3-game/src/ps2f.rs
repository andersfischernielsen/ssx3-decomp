//! PS2 floating point for replacements, bit-exact with the hardware: no
//! infinities or NaNs, denormals flushed, results truncated toward zero.
//! Uses the same functions as the VU/FPU models (`ssx3_ee_rt::float`).

use ssx3_ee_rt::float;

/// A VU0/VU1 FMAC float (macro-mode COP2 arithmetic). Operators and the
/// `madd`/`msub` helpers round exactly like the corresponding VU instruction,
/// so expressions must keep the game's operation order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VuF32(pub u32);

impl VuF32 {
    pub const ZERO: VuF32 = VuF32(0);
    pub const ONE: VuF32 = VuF32(0x3f80_0000);

    pub fn from_f32(x: f32) -> VuF32 {
        VuF32(x.to_bits())
    }
    pub fn to_f32(self) -> f32 {
        f32::from_bits(self.0)
    }
    /// `acc + a * b` as one MADD (product rounded, then the sum).
    #[inline]
    pub fn madd(acc: VuF32, a: VuF32, b: VuF32) -> VuF32 {
        VuF32(float::madd(acc.0, a.0, b.0).0)
    }
    /// `acc - a * b` as one MSUB.
    #[inline]
    pub fn msub(acc: VuF32, a: VuF32, b: VuF32) -> VuF32 {
        VuF32(float::msub(acc.0, a.0, b.0).0)
    }
    /// `a / b` as DIV (the value that lands in Q).
    #[inline]
    pub fn div(a: VuF32, b: VuF32) -> VuF32 {
        VuF32(float::div(a.0, b.0).value)
    }
    /// `sqrt(b)` as SQRT (the value that lands in Q).
    #[inline]
    pub fn sqrt(b: VuF32) -> VuF32 {
        VuF32(float::sqrt(b.0).value)
    }
    /// `a / sqrt(b)` as RSQRT (the value that lands in Q).
    #[inline]
    pub fn rsqrt(a: VuF32, b: VuF32) -> VuF32 {
        VuF32(float::rsqrt(a.0, b.0).value)
    }
}

impl std::ops::Add for VuF32 {
    type Output = VuF32;
    #[inline]
    fn add(self, b: VuF32) -> VuF32 {
        VuF32(float::add(self.0, b.0).0)
    }
}
impl std::ops::Sub for VuF32 {
    type Output = VuF32;
    #[inline]
    fn sub(self, b: VuF32) -> VuF32 {
        VuF32(float::sub(self.0, b.0).0)
    }
}
impl std::ops::Mul for VuF32 {
    type Output = VuF32;
    #[inline]
    fn mul(self, b: VuF32) -> VuF32 {
        VuF32(float::mul(self.0, b.0).0)
    }
}

/// An FPU (COP1) single: same number format and truncating arithmetic as
/// the VU, but `madd`/`msub` round twice (product, then sum) and compares use
/// the PS2 total order ([`FpuF32::key`]). Matches `ssx3_ee_rt::fpu`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FpuF32(pub u32);

impl FpuF32 {
    pub const ZERO: FpuF32 = FpuF32(0);
    pub const ONE: FpuF32 = FpuF32(0x3f80_0000);

    pub fn from_f32(x: f32) -> FpuF32 {
        FpuF32(x.to_bits())
    }
    pub fn to_f32(self) -> f32 {
        f32::from_bits(self.0)
    }
    /// `div.s`.
    #[inline]
    pub fn div(self, b: FpuF32) -> FpuF32 {
        FpuF32(float::div(self.0, b.0).value)
    }
    /// `sqrt.s`.
    #[inline]
    pub fn sqrt(self) -> FpuF32 {
        FpuF32(float::sqrt(self.0).value)
    }
    /// `rsqrt.s`: `self / sqrt(b)`.
    #[inline]
    pub fn rsqrt(self, b: FpuF32) -> FpuF32 {
        FpuF32(float::rsqrt(self.0, b.0).value)
    }
    /// `abs.s` (bit operation).
    #[inline]
    pub fn abs(self) -> FpuF32 {
        FpuF32(self.0 & 0x7fff_ffff)
    }
    /// `max.s` / `min.s`.
    #[inline]
    pub fn max(self, b: FpuF32) -> FpuF32 {
        FpuF32(float::max(self.0, b.0))
    }
    #[inline]
    pub fn min(self, b: FpuF32) -> FpuF32 {
        FpuF32(float::min(self.0, b.0))
    }
    /// `madd.s`: `acc + a * b`, the product rounded first.
    #[inline]
    pub fn madd(acc: FpuF32, a: FpuF32, b: FpuF32) -> FpuF32 {
        acc + a * b
    }
    /// `msub.s`: `acc - a * b`, the product rounded first.
    #[inline]
    pub fn msub(acc: FpuF32, a: FpuF32, b: FpuF32) -> FpuF32 {
        acc - a * b
    }
    /// `cvt.w.s`: truncate toward zero, saturating.
    #[inline]
    pub fn to_i32(self) -> i32 {
        float::to_fixed(self.0, 0) as i32
    }
    /// `cvt.s.w`.
    #[inline]
    pub fn from_i32(v: i32) -> FpuF32 {
        FpuF32(float::from_fixed(v, 0))
    }
    /// The total order `c.eq/lt/le.s` compare by: sign-magnitude as an
    /// integer, denormals as zero.
    #[inline]
    pub fn key(self) -> i32 {
        let x = if self.0 & 0x7f80_0000 == 0 {
            self.0 & 0x8000_0000
        } else {
            self.0
        };
        let m = (x & 0x7fff_ffff) as i32;
        if x & 0x8000_0000 != 0 {
            -m
        } else {
            m
        }
    }
    /// `c.lt.s`.
    #[inline]
    pub fn lt(self, b: FpuF32) -> bool {
        self.key() < b.key()
    }
    /// `c.le.s`.
    #[inline]
    pub fn le(self, b: FpuF32) -> bool {
        self.key() <= b.key()
    }
    /// `c.eq.s`.
    #[inline]
    pub fn eq_s(self, b: FpuF32) -> bool {
        self.key() == b.key()
    }
}

impl std::ops::Add for FpuF32 {
    type Output = FpuF32;
    #[inline]
    fn add(self, b: FpuF32) -> FpuF32 {
        FpuF32(float::add(self.0, b.0).0)
    }
}
impl std::ops::Sub for FpuF32 {
    type Output = FpuF32;
    #[inline]
    fn sub(self, b: FpuF32) -> FpuF32 {
        FpuF32(float::sub(self.0, b.0).0)
    }
}
impl std::ops::Mul for FpuF32 {
    type Output = FpuF32;
    #[inline]
    fn mul(self, b: FpuF32) -> FpuF32 {
        FpuF32(float::mul(self.0, b.0).0)
    }
}
impl std::ops::Neg for FpuF32 {
    type Output = FpuF32;
    /// `neg.s` (bit operation).
    #[inline]
    fn neg(self) -> FpuF32 {
        FpuF32(self.0 ^ 0x8000_0000)
    }
}

/// Four VU lanes (x, y, z, w), as stored in a quadword.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Vec4 {
    pub x: VuF32,
    pub y: VuF32,
    pub z: VuF32,
    pub w: VuF32,
}

impl Vec4 {
    pub fn from_bits(q: u128) -> Vec4 {
        Vec4 {
            x: VuF32(q as u32),
            y: VuF32((q >> 32) as u32),
            z: VuF32((q >> 64) as u32),
            w: VuF32((q >> 96) as u32),
        }
    }
    pub fn to_bits(self) -> u128 {
        (self.x.0 as u128)
            | (self.y.0 as u128) << 32
            | (self.z.0 as u128) << 64
            | (self.w.0 as u128) << 96
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `x - x` is +0 for every input (the matrix code relies on it to clear w).
    #[test]
    fn self_sub_is_plus_zero() {
        let mut s = 0x1234_5678u32;
        for _ in 0..200_000 {
            s ^= s << 13;
            s ^= s >> 17;
            s ^= s << 5;
            assert_eq!((VuF32(s) - VuF32(s)).0, 0, "{s:#x}");
        }
    }
}

/// VU0 registers a replacement leaves behind. Callers don't use them, but a
/// later function may spill a VF register to its stack frame, so to keep RAM
/// byte-identical a replacement of VU0 code writes the final values of the
/// registers the original used as scratch (only the lanes it wrote).
pub mod vu0_scratch {
    use super::Vec4;
    use ssx3_ee_rt::Cpu;

    /// Lane mask in the VU `dest` order: x = 8, y = 4, z = 2, w = 1.
    pub fn set_vf(c: &mut Cpu, reg: usize, dest: u8, v: Vec4) {
        let mut cur = c.bus.plat.vf(reg);
        let new = [v.x.0, v.y.0, v.z.0, v.w.0];
        for (l, lane) in cur.iter_mut().enumerate() {
            if dest & (8 >> l) != 0 {
                *lane = new[l];
            }
        }
        c.bus.plat.set_vf(reg, cur);
    }

    /// The accumulator ACC (only the lanes in `dest` are written).
    pub fn set_acc(c: &mut Cpu, dest: u8, v: Vec4) {
        let mut cur = c.bus.plat.acc();
        let new = [v.x.0, v.y.0, v.z.0, v.w.0];
        for (l, lane) in cur.iter_mut().enumerate() {
            if dest & (8 >> l) != 0 {
                *lane = new[l];
            }
        }
        c.bus.plat.set_acc(cur);
    }

    /// The Q register (VI22).
    pub fn set_q(c: &mut Cpu, q: super::VuF32) {
        c.bus.plat.write_ctrl(22, q.0);
    }
}
