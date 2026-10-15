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
        (self.x.0 as u128) | (self.y.0 as u128) << 32 | (self.z.0 as u128) << 64 | (self.w.0 as u128) << 96
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

    /// The Q register (VI22).
    pub fn set_q(c: &mut Cpu, q: super::VuF32) {
        c.bus.plat.write_ctrl(22, q.0);
    }
}
