//! R5900 FPU (COP1) semantics. Single precision only, with PS2 rules: there
//! are no infinities or NaNs (exponent 255 is an ordinary large number), and
//! denormals are flushed to zero. Overflowing results clamp to ±`0x7f7fffff`.
//! Rounding is toward zero; arithmetic shares the bit-exact integer routines in
//! `float` (ported from ps2-web-recomp `fpu.hpp`, MIT).

use crate::cpu::{sx, Cpu};
use crate::float as F;

/// Total order used by PS2 compares: sign-magnitude as an integer, denormals
/// read as zero (ps2-web-recomp `fp::key`).
#[inline]
fn key(x: u32) -> i32 {
    let x = if x & 0x7f80_0000 == 0 { x & 0x8000_0000 } else { x };
    let m = (x & 0x7fff_ffff) as i32;
    if x & 0x8000_0000 != 0 {
        -m
    } else {
        m
    }
}

pub const FMAX: u32 = 0x7f7f_ffff;
/// FCR31 condition bit (set by `c.*.s`, read by `bc1f/bc1t`).
pub const FCC: u32 = 1 << 23;

/// Interpret raw PS2 float bits as a host `f32` (exp 255 → ±max, denormal → 0).
#[inline]
pub fn rd(v: u32) -> f32 {
    let e = v & 0x7f80_0000;
    if e == 0x7f80_0000 {
        f32::from_bits((v & 0x8000_0000) | FMAX)
    } else if e == 0 {
        f32::from_bits(v & 0x8000_0000)
    } else {
        f32::from_bits(v)
    }
}

/// Convert a host result back to PS2 float bits with clamping.
#[inline]
pub fn wr(f: f32) -> u32 {
    let v = f.to_bits();
    let e = v & 0x7f80_0000;
    if e == 0x7f80_0000 {
        if f.is_nan() {
            // 0/0, inf-inf: hardware returns a signed maximum.
            (v & 0x8000_0000) | FMAX
        } else {
            (v & 0x8000_0000) | FMAX
        }
    } else if e == 0 {
        v & 0x8000_0000
    } else {
        v
    }
}

impl Cpu {
    pub fn mfc1(&mut self, rt: usize, fs: usize) {
        if rt != 0 {
            self.g[rt][0] = sx(self.f[fs]);
        }
    }
    pub fn mtc1(&mut self, rt: usize, fs: usize) {
        self.f[fs] = self.g[rt][0] as u32;
    }
    pub fn cfc1(&mut self, rt: usize, fs: usize) {
        let v = if fs == 31 { self.fcr31 } else if fs == 0 { 0x2e30 } else { 0 };
        if rt != 0 {
            self.g[rt][0] = sx(v);
        }
    }
    pub fn ctc1(&mut self, rt: usize, fs: usize) {
        if fs == 31 {
            self.fcr31 = self.g[rt][0] as u32;
        }
    }
    #[inline]
    pub fn fcc(&self) -> bool {
        self.fcr31 & FCC != 0
    }

    pub fn add_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = F::add(self.f[fs], self.f[ft]).0;
    }
    pub fn sub_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = F::sub(self.f[fs], self.f[ft]).0;
    }
    pub fn mul_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = F::mul(self.f[fs], self.f[ft]).0;
    }
    pub fn div_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = F::div(self.f[fs], self.f[ft]).value;
    }
    pub fn sqrt_s(&mut self, fd: usize, ft: usize) {
        self.f[fd] = F::sqrt(self.f[ft]).value;
    }
    pub fn rsqrt_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = F::rsqrt(self.f[fs], self.f[ft]).value;
    }
    pub fn abs_s(&mut self, fd: usize, fs: usize) {
        self.f[fd] = self.f[fs] & 0x7fff_ffff;
    }
    pub fn mov_s(&mut self, fd: usize, fs: usize) {
        self.f[fd] = self.f[fs];
    }
    pub fn neg_s(&mut self, fd: usize, fs: usize) {
        self.f[fd] = self.f[fs] ^ 0x8000_0000;
    }
    pub fn adda_s(&mut self, fs: usize, ft: usize) {
        self.facc = F::add(self.f[fs], self.f[ft]).0;
    }
    pub fn suba_s(&mut self, fs: usize, ft: usize) {
        self.facc = F::sub(self.f[fs], self.f[ft]).0;
    }
    pub fn mula_s(&mut self, fs: usize, ft: usize) {
        self.facc = F::mul(self.f[fs], self.f[ft]).0;
    }
    pub fn madd_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = F::add(self.facc, F::mul(self.f[fs], self.f[ft]).0).0;
    }
    pub fn msub_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = F::sub(self.facc, F::mul(self.f[fs], self.f[ft]).0).0;
    }
    pub fn madda_s(&mut self, fs: usize, ft: usize) {
        self.facc = F::add(self.facc, F::mul(self.f[fs], self.f[ft]).0).0;
    }
    pub fn msuba_s(&mut self, fs: usize, ft: usize) {
        self.facc = F::sub(self.facc, F::mul(self.f[fs], self.f[ft]).0).0;
    }
    pub fn max_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = F::max(self.f[fs], self.f[ft]);
    }
    pub fn min_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = F::min(self.f[fs], self.f[ft]);
    }
    /// `cvt.w.s`: truncate toward zero, saturating.
    pub fn cvt_w_s(&mut self, fd: usize, fs: usize) {
        self.f[fd] = F::to_fixed(self.f[fs], 0);
    }
    pub fn cvt_s_w(&mut self, fd: usize, fs: usize) {
        self.f[fd] = F::from_fixed(self.f[fs] as i32, 0);
    }
    fn set_cc(&mut self, c: bool) {
        if c {
            self.fcr31 |= FCC;
        } else {
            self.fcr31 &= !FCC;
        }
    }
    pub fn c_f_s(&mut self) {
        self.set_cc(false);
    }
    pub fn c_eq_s(&mut self, fs: usize, ft: usize) {
        let c = key(self.f[fs]) == key(self.f[ft]);
        self.set_cc(c);
    }
    pub fn c_lt_s(&mut self, fs: usize, ft: usize) {
        let c = key(self.f[fs]) < key(self.f[ft]);
        self.set_cc(c);
    }
    pub fn c_le_s(&mut self, fs: usize, ft: usize) {
        let c = key(self.f[fs]) <= key(self.f[ft]);
        self.set_cc(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_and_flushes() {
        assert_eq!(wr(f32::INFINITY), FMAX);
        assert_eq!(wr(f32::NEG_INFINITY), 0x8000_0000 | FMAX);
        assert_eq!(wr(f32::from_bits(1)), 0);
        assert_eq!(rd(0x7f80_0000), f32::from_bits(FMAX));
    }

    #[test]
    fn cvt_saturates() {
        let mut c = Cpu::detached();
        c.f[1] = (-3.7f32).to_bits();
        c.cvt_w_s(2, 1);
        assert_eq!(c.f[2] as i32, -3);
        c.f[1] = 1e20f32.to_bits();
        c.cvt_w_s(2, 1);
        assert_eq!(c.f[2], 0x7fff_ffff);
    }
}
