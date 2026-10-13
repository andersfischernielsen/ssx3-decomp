//! R5900 FPU (COP1) semantics. Single precision only, with PS2 rules: there
//! are no infinities or NaNs (exponent 255 is an ordinary large number), and
//! denormals are flushed to zero. Overflowing results clamp to ±`0x7f7fffff`.

use crate::cpu::{sx, Cpu};

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
        self.f[fd] = wr(rd(self.f[fs]) + rd(self.f[ft]));
    }
    pub fn sub_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = wr(rd(self.f[fs]) - rd(self.f[ft]));
    }
    pub fn mul_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = wr(rd(self.f[fs]) * rd(self.f[ft]));
    }
    pub fn div_s(&mut self, fd: usize, fs: usize, ft: usize) {
        let (a, b) = (rd(self.f[fs]), rd(self.f[ft]));
        self.f[fd] = if b == 0.0 {
            ((self.f[fs] ^ self.f[ft]) & 0x8000_0000) | FMAX
        } else {
            wr(a / b)
        };
    }
    pub fn sqrt_s(&mut self, fd: usize, ft: usize) {
        self.f[fd] = wr(rd(self.f[ft]).abs().sqrt());
    }
    pub fn rsqrt_s(&mut self, fd: usize, fs: usize, ft: usize) {
        let b = rd(self.f[ft]).abs().sqrt();
        self.f[fd] = if b == 0.0 {
            (self.f[fs] & 0x8000_0000) | FMAX
        } else {
            wr(rd(self.f[fs]) / b)
        };
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
        self.facc = wr(rd(self.f[fs]) + rd(self.f[ft]));
    }
    pub fn suba_s(&mut self, fs: usize, ft: usize) {
        self.facc = wr(rd(self.f[fs]) - rd(self.f[ft]));
    }
    pub fn mula_s(&mut self, fs: usize, ft: usize) {
        self.facc = wr(rd(self.f[fs]) * rd(self.f[ft]));
    }
    pub fn madd_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = wr(rd(self.facc) + rd(wr(rd(self.f[fs]) * rd(self.f[ft]))));
    }
    pub fn msub_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = wr(rd(self.facc) - rd(wr(rd(self.f[fs]) * rd(self.f[ft]))));
    }
    pub fn madda_s(&mut self, fs: usize, ft: usize) {
        self.facc = wr(rd(self.facc) + rd(wr(rd(self.f[fs]) * rd(self.f[ft]))));
    }
    pub fn msuba_s(&mut self, fs: usize, ft: usize) {
        self.facc = wr(rd(self.facc) - rd(wr(rd(self.f[fs]) * rd(self.f[ft]))));
    }
    pub fn max_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = if rd(self.f[fs]) >= rd(self.f[ft]) { self.f[fs] } else { self.f[ft] };
    }
    pub fn min_s(&mut self, fd: usize, fs: usize, ft: usize) {
        self.f[fd] = if rd(self.f[fs]) <= rd(self.f[ft]) { self.f[fs] } else { self.f[ft] };
    }
    /// `cvt.w.s`: truncate toward zero, saturating.
    pub fn cvt_w_s(&mut self, fd: usize, fs: usize) {
        let v = self.f[fs];
        self.f[fd] = if v & 0x7f80_0000 <= 0x4e80_0000 {
            rd(v) as i32 as u32
        } else if v & 0x8000_0000 != 0 {
            0x8000_0000
        } else {
            0x7fff_ffff
        };
    }
    pub fn cvt_s_w(&mut self, fd: usize, fs: usize) {
        self.f[fd] = wr(self.f[fs] as i32 as f32);
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
        let c = rd(self.f[fs]) == rd(self.f[ft]);
        self.set_cc(c);
    }
    pub fn c_lt_s(&mut self, fs: usize, ft: usize) {
        let c = rd(self.f[fs]) < rd(self.f[ft]);
        self.set_cc(c);
    }
    pub fn c_le_s(&mut self, fs: usize, ft: usize) {
        let c = rd(self.f[fs]) <= rd(self.f[ft]);
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
        let mut c = Cpu::new();
        c.f[1] = (-3.7f32).to_bits();
        c.cvt_w_s(2, 1);
        assert_eq!(c.f[2] as i32, -3);
        c.f[1] = 1e20f32.to_bits();
        c.cvt_w_s(2, 1);
        assert_eq!(c.f[2], 0x7fff_ffff);
    }
}
