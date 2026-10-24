//! R5900 MMI (multimedia) instruction semantics: 128-bit SIMD over the GPRs.
//!
//! Each method takes register indices already decoded by the recompiler.
//! Lane conventions: `w` = 32-bit word lanes (4), `h` = halfwords (8),
//! `b` = bytes (16); lane 0 is least significant.

use crate::cpu::{sx, Cpu};

#[inline]
fn q(r: [u64; 2]) -> u128 {
    r[0] as u128 | (r[1] as u128) << 64
}
#[inline]
fn uq(v: u128) -> [u64; 2] {
    [v as u64, (v >> 64) as u64]
}
#[inline]
fn w(r: [u64; 2]) -> [u32; 4] {
    [
        r[0] as u32,
        (r[0] >> 32) as u32,
        r[1] as u32,
        (r[1] >> 32) as u32,
    ]
}
#[inline]
fn uw(v: [u32; 4]) -> [u64; 2] {
    [
        v[0] as u64 | (v[1] as u64) << 32,
        v[2] as u64 | (v[3] as u64) << 32,
    ]
}
#[inline]
fn h(r: [u64; 2]) -> [u16; 8] {
    let mut o = [0u16; 8];
    for (i, x) in o.iter_mut().enumerate() {
        *x = (r[i / 4] >> ((i % 4) * 16)) as u16;
    }
    o
}
#[inline]
fn uh(v: [u16; 8]) -> [u64; 2] {
    let mut o = [0u64; 2];
    for (i, x) in v.iter().enumerate() {
        o[i / 4] |= (*x as u64) << ((i % 4) * 16);
    }
    o
}
#[inline]
fn b(r: [u64; 2]) -> [u8; 16] {
    q(r).to_le_bytes()
}
#[inline]
fn ub(v: [u8; 16]) -> [u64; 2] {
    uq(u128::from_le_bytes(v))
}

fn sat32(v: i64) -> u32 {
    v.clamp(i32::MIN as i64, i32::MAX as i64) as i32 as u32
}
fn sat16(v: i32) -> u16 {
    v.clamp(i16::MIN as i32, i16::MAX as i32) as i16 as u16
}
fn sat8(v: i32) -> u8 {
    v.clamp(i8::MIN as i32, i8::MAX as i32) as i8 as u8
}

macro_rules! lanes {
    ($name:ident, $get:ident, $put:ident, $t:ty, $n:expr, |$a:ident, $b2:ident| $e:expr) => {
        pub fn $name(&mut self, rd: usize, rs: usize, rt: usize) {
            let x = $get(self.g[rs]);
            let y = $get(self.g[rt]);
            let mut o = [0 as $t; $n];
            for i in 0..$n {
                let $a = x[i];
                let $b2 = y[i];
                o[i] = $e;
            }
            self.wr(rd, $put(o));
        }
    };
}

impl Cpu {
    #[inline]
    fn wr(&mut self, rd: usize, v: [u64; 2]) {
        if rd != 0 {
            self.g[rd] = v;
        }
    }

    // ---- arithmetic (MMI0 / MMI1) ----
    lanes!(paddw, w, uw, u32, 4, |a, b| a.wrapping_add(b));
    lanes!(psubw, w, uw, u32, 4, |a, b| a.wrapping_sub(b));
    lanes!(paddh, h, uh, u16, 8, |a, b| a.wrapping_add(b));
    lanes!(psubh, h, uh, u16, 8, |a, b| a.wrapping_sub(b));
    lanes!(paddb, b, ub, u8, 16, |a, b| a.wrapping_add(b));
    lanes!(psubb, b, ub, u8, 16, |a, b| a.wrapping_sub(b));
    lanes!(paddsw, w, uw, u32, 4, |a, b| sat32(
        a as i32 as i64 + b as i32 as i64
    ));
    lanes!(psubsw, w, uw, u32, 4, |a, b| sat32(
        a as i32 as i64 - b as i32 as i64
    ));
    lanes!(paddsh, h, uh, u16, 8, |a, b| sat16(
        a as i16 as i32 + b as i16 as i32
    ));
    lanes!(psubsh, h, uh, u16, 8, |a, b| sat16(
        a as i16 as i32 - b as i16 as i32
    ));
    lanes!(paddsb, b, ub, u8, 16, |a, b| sat8(
        a as i8 as i32 + b as i8 as i32
    ));
    lanes!(psubsb, b, ub, u8, 16, |a, b| sat8(
        a as i8 as i32 - b as i8 as i32
    ));
    lanes!(
        padduw,
        w,
        uw,
        u32,
        4,
        |a, b| (a as u64 + b as u64).min(u32::MAX as u64) as u32
    );
    lanes!(psubuw, w, uw, u32, 4, |a, b| a.saturating_sub(b));
    lanes!(padduh, h, uh, u16, 8, |a, b| a.saturating_add(b));
    lanes!(psubuh, h, uh, u16, 8, |a, b| a.saturating_sub(b));
    lanes!(paddub, b, ub, u8, 16, |a, b| a.saturating_add(b));
    lanes!(psubub, b, ub, u8, 16, |a, b| a.saturating_sub(b));
    lanes!(pcgtw, w, uw, u32, 4, |a, b| if (a as i32) > (b as i32) {
        !0
    } else {
        0
    });
    lanes!(pcgth, h, uh, u16, 8, |a, b| if (a as i16) > (b as i16) {
        !0
    } else {
        0
    });
    lanes!(pcgtb, b, ub, u8, 16, |a, b| if (a as i8) > (b as i8) {
        !0
    } else {
        0
    });
    lanes!(pceqw, w, uw, u32, 4, |a, b| if a == b { !0 } else { 0 });
    lanes!(pceqh, h, uh, u16, 8, |a, b| if a == b { !0 } else { 0 });
    lanes!(pceqb, b, ub, u8, 16, |a, b| if a == b { !0 } else { 0 });
    lanes!(pmaxw, w, uw, u32, 4, |a, b| (a as i32).max(b as i32) as u32);
    lanes!(pminw, w, uw, u32, 4, |a, b| (a as i32).min(b as i32) as u32);
    lanes!(pmaxh, h, uh, u16, 8, |a, b| (a as i16).max(b as i16) as u16);
    lanes!(pminh, h, uh, u16, 8, |a, b| (a as i16).min(b as i16) as u16);
    lanes!(pand, w, uw, u32, 4, |a, b| a & b);
    lanes!(por, w, uw, u32, 4, |a, b| a | b);
    lanes!(pxor, w, uw, u32, 4, |a, b| a ^ b);
    lanes!(pnor, w, uw, u32, 4, |a, b| !(a | b));

    pub fn pabsw(&mut self, rd: usize, rt: usize) {
        let mut x = w(self.g[rt]);
        for v in x.iter_mut() {
            *v = if *v == 0x8000_0000 {
                0x7fff_ffff
            } else {
                (*v as i32).unsigned_abs()
            };
        }
        self.wr(rd, uw(x));
    }
    pub fn pabsh(&mut self, rd: usize, rt: usize) {
        let mut x = h(self.g[rt]);
        for v in x.iter_mut() {
            *v = if *v == 0x8000 {
                0x7fff
            } else {
                (*v as i16).unsigned_abs()
            };
        }
        self.wr(rd, uh(x));
    }
    pub fn padsbh(&mut self, rd: usize, rs: usize, rt: usize) {
        let x = h(self.g[rs]);
        let y = h(self.g[rt]);
        let mut o = [0u16; 8];
        for i in 0..8 {
            o[i] = if i < 4 {
                x[i].wrapping_sub(y[i])
            } else {
                x[i].wrapping_add(y[i])
            };
        }
        self.wr(rd, uh(o));
    }

    // ---- pack / extend / interleave ----
    pub fn pextlw(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (w(self.g[rs]), w(self.g[rt]));
        self.wr(rd, uw([t[0], s[0], t[1], s[1]]));
    }
    pub fn pextuw(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (w(self.g[rs]), w(self.g[rt]));
        self.wr(rd, uw([t[2], s[2], t[3], s[3]]));
    }
    pub fn pextlh(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (h(self.g[rs]), h(self.g[rt]));
        self.wr(rd, uh([t[0], s[0], t[1], s[1], t[2], s[2], t[3], s[3]]));
    }
    pub fn pextuh(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (h(self.g[rs]), h(self.g[rt]));
        self.wr(rd, uh([t[4], s[4], t[5], s[5], t[6], s[6], t[7], s[7]]));
    }
    pub fn pextlb(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (b(self.g[rs]), b(self.g[rt]));
        let mut o = [0u8; 16];
        for i in 0..8 {
            o[2 * i] = t[i];
            o[2 * i + 1] = s[i];
        }
        self.wr(rd, ub(o));
    }
    pub fn pextub(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (b(self.g[rs]), b(self.g[rt]));
        let mut o = [0u8; 16];
        for i in 0..8 {
            o[2 * i] = t[8 + i];
            o[2 * i + 1] = s[8 + i];
        }
        self.wr(rd, ub(o));
    }
    pub fn ppacw(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (w(self.g[rs]), w(self.g[rt]));
        self.wr(rd, uw([t[0], t[2], s[0], s[2]]));
    }
    pub fn ppach(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (h(self.g[rs]), h(self.g[rt]));
        self.wr(rd, uh([t[0], t[2], t[4], t[6], s[0], s[2], s[4], s[6]]));
    }
    pub fn ppacb(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (b(self.g[rs]), b(self.g[rt]));
        let mut o = [0u8; 16];
        for i in 0..8 {
            o[i] = t[2 * i];
            o[8 + i] = s[2 * i];
        }
        self.wr(rd, ub(o));
    }
    pub fn pext5(&mut self, rd: usize, rt: usize) {
        let mut x = w(self.g[rt]);
        for v in x.iter_mut() {
            let p = *v;
            *v = ((p & 0x1f) << 3)
                | (((p >> 5) & 0x1f) << 11)
                | (((p >> 10) & 0x1f) << 19)
                | (((p >> 15) & 1) << 31);
        }
        self.wr(rd, uw(x));
    }
    pub fn ppac5(&mut self, rd: usize, rt: usize) {
        let mut x = w(self.g[rt]);
        for v in x.iter_mut() {
            let p = *v;
            *v = ((p >> 3) & 0x1f)
                | (((p >> 11) & 0x1f) << 5)
                | (((p >> 19) & 0x1f) << 10)
                | (((p >> 31) & 1) << 15);
        }
        self.wr(rd, uw(x));
    }
    pub fn pcpyld(&mut self, rd: usize, rs: usize, rt: usize) {
        let v = [self.g[rt][0], self.g[rs][0]];
        self.wr(rd, v);
    }
    pub fn pcpyud(&mut self, rd: usize, rs: usize, rt: usize) {
        let v = [self.g[rs][1], self.g[rt][1]];
        self.wr(rd, v);
    }
    pub fn pcpyh(&mut self, rd: usize, rt: usize) {
        let t = h(self.g[rt]);
        self.wr(rd, uh([t[0], t[0], t[0], t[0], t[4], t[4], t[4], t[4]]));
    }
    pub fn pinth(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (h(self.g[rs]), h(self.g[rt]));
        self.wr(rd, uh([t[0], s[4], t[1], s[5], t[2], s[6], t[3], s[7]]));
    }
    pub fn pinteh(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (h(self.g[rs]), h(self.g[rt]));
        self.wr(rd, uh([t[0], s[0], t[2], s[2], t[4], s[4], t[6], s[6]]));
    }
    pub fn pexeh(&mut self, rd: usize, rt: usize) {
        let t = h(self.g[rt]);
        self.wr(rd, uh([t[2], t[1], t[0], t[3], t[6], t[5], t[4], t[7]]));
    }
    pub fn prevh(&mut self, rd: usize, rt: usize) {
        let t = h(self.g[rt]);
        self.wr(rd, uh([t[3], t[2], t[1], t[0], t[7], t[6], t[5], t[4]]));
    }
    pub fn pexch(&mut self, rd: usize, rt: usize) {
        let t = h(self.g[rt]);
        self.wr(rd, uh([t[0], t[2], t[1], t[3], t[4], t[6], t[5], t[7]]));
    }
    pub fn pexew(&mut self, rd: usize, rt: usize) {
        let t = w(self.g[rt]);
        self.wr(rd, uw([t[2], t[1], t[0], t[3]]));
    }
    pub fn pexcw(&mut self, rd: usize, rt: usize) {
        let t = w(self.g[rt]);
        self.wr(rd, uw([t[0], t[2], t[1], t[3]]));
    }
    pub fn prot3w(&mut self, rd: usize, rt: usize) {
        let t = w(self.g[rt]);
        self.wr(rd, uw([t[1], t[2], t[0], t[3]]));
    }
    pub fn plzcw(&mut self, rd: usize, rs: usize) {
        let s = self.g[rs][0];
        let f = |x: u32| {
            let y = if (x as i32) < 0 { !x } else { x };
            y.leading_zeros().saturating_sub(1).min(31)
        };
        let lo = f(s as u32) as u64;
        let hi = f((s >> 32) as u32) as u64;
        if rd != 0 {
            self.g[rd][0] = lo | (hi << 32);
        }
    }

    // ---- shifts ----
    pub fn psllh(&mut self, rd: usize, rt: usize, sa: u32) {
        let mut t = h(self.g[rt]);
        t.iter_mut().for_each(|v| *v <<= sa & 15);
        self.wr(rd, uh(t));
    }
    pub fn psrlh(&mut self, rd: usize, rt: usize, sa: u32) {
        let mut t = h(self.g[rt]);
        t.iter_mut().for_each(|v| *v >>= sa & 15);
        self.wr(rd, uh(t));
    }
    pub fn psrah(&mut self, rd: usize, rt: usize, sa: u32) {
        let mut t = h(self.g[rt]);
        t.iter_mut()
            .for_each(|v| *v = ((*v as i16) >> (sa & 15)) as u16);
        self.wr(rd, uh(t));
    }
    pub fn psllw(&mut self, rd: usize, rt: usize, sa: u32) {
        let mut t = w(self.g[rt]);
        t.iter_mut().for_each(|v| *v <<= sa & 31);
        self.wr(rd, uw(t));
    }
    pub fn psrlw(&mut self, rd: usize, rt: usize, sa: u32) {
        let mut t = w(self.g[rt]);
        t.iter_mut().for_each(|v| *v >>= sa & 31);
        self.wr(rd, uw(t));
    }
    pub fn psraw(&mut self, rd: usize, rt: usize, sa: u32) {
        let mut t = w(self.g[rt]);
        t.iter_mut()
            .for_each(|v| *v = ((*v as i32) >> (sa & 31)) as u32);
        self.wr(rd, uw(t));
    }
    pub fn psllvw(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (w(self.g[rs]), w(self.g[rt]));
        self.wr(rd, [sx(t[0] << (s[0] & 31)), sx(t[2] << (s[2] & 31))]);
    }
    pub fn psrlvw(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (w(self.g[rs]), w(self.g[rt]));
        self.wr(rd, [sx(t[0] >> (s[0] & 31)), sx(t[2] >> (s[2] & 31))]);
    }
    pub fn psravw(&mut self, rd: usize, rs: usize, rt: usize) {
        let (s, t) = (w(self.g[rs]), w(self.g[rt]));
        self.wr(
            rd,
            [
                sx(((t[0] as i32) >> (s[0] & 31)) as u32),
                sx(((t[2] as i32) >> (s[2] & 31)) as u32),
            ],
        );
    }
    pub fn qfsrv(&mut self, rd: usize, rs: usize, rt: usize) {
        let s = self.sa & 127;
        let hi = q(self.g[rs]);
        let lo = q(self.g[rt]);
        let v = if s == 0 {
            lo
        } else {
            (lo >> s) | (hi << (128 - s))
        };
        self.wr(rd, uq(v));
    }
    pub fn mtsab(&mut self, rs: usize, imm: u32) {
        self.sa = ((self.g[rs][0] as u32 ^ imm) & 15) * 8;
    }
    pub fn mtsah(&mut self, rs: usize, imm: u32) {
        self.sa = ((self.g[rs][0] as u32 ^ imm) & 7) * 16;
    }

    // ---- HI/LO movement ----
    pub fn pmfhi(&mut self, rd: usize) {
        let v = [self.hi[0], self.hi[1]];
        self.wr(rd, v);
    }
    pub fn pmflo(&mut self, rd: usize) {
        let v = [self.lo[0], self.lo[1]];
        self.wr(rd, v);
    }
    pub fn pmthi(&mut self, rs: usize) {
        self.hi = self.g[rs];
    }
    pub fn pmtlo(&mut self, rs: usize) {
        self.lo = self.g[rs];
    }
    pub fn pmfhl(&mut self, rd: usize, fmt: u32) {
        let (hi, lo) = (w(self.hi), w(self.lo));
        let v = match fmt {
            0 => uw([lo[0], hi[0], lo[2], hi[2]]),
            1 => uw([lo[1], hi[1], lo[3], hi[3]]),
            2 => {
                let a = ((hi[0] as u64) << 32 | lo[0] as u64) as i64;
                let c = ((hi[2] as u64) << 32 | lo[2] as u64) as i64;
                [sx(sat32(a)), sx(sat32(c))]
            }
            3 => {
                let (hh, lh) = (h(self.hi), h(self.lo));
                uh([lh[0], lh[2], hh[0], hh[2], lh[4], lh[6], hh[4], hh[6]])
            }
            4 => {
                let s = |x: u32| sat16(x as i32);
                uh([
                    s(lo[0]),
                    s(lo[1]),
                    s(hi[0]),
                    s(hi[1]),
                    s(lo[2]),
                    s(lo[3]),
                    s(hi[2]),
                    s(hi[3]),
                ])
            }
            _ => return,
        };
        self.wr(rd, v);
    }
    pub fn pmthl(&mut self, rs: usize, fmt: u32) {
        if fmt != 0 {
            return;
        }
        let s = w(self.g[rs]);
        let mut hi = w(self.hi);
        let mut lo = w(self.lo);
        lo[0] = s[0];
        hi[0] = s[1];
        lo[2] = s[2];
        hi[2] = s[3];
        self.hi = uw(hi);
        self.lo = uw(lo);
    }

    // ---- multiply / divide ----
    fn pmul_w(&mut self, rd: usize, rs: usize, rt: usize, signed: bool, op: i32) {
        let (s, t) = (w(self.g[rs]), w(self.g[rt]));
        let mut out = [0u64; 2];
        for (k, i) in [0usize, 2].into_iter().enumerate() {
            let prod: i64 = if signed {
                (s[i] as i32 as i64) * (t[i] as i32 as i64)
            } else {
                ((s[i] as u64) * (t[i] as u64)) as i64
            };
            let acc = ((self.hi[k] as u32 as u64) << 32 | self.lo[k] as u32 as u64) as i64;
            let r = match op {
                0 => prod,
                1 => acc.wrapping_add(prod),
                _ => acc.wrapping_sub(prod),
            };
            self.lo[k] = sx(r as u32);
            self.hi[k] = sx((r >> 32) as u32);
            out[k] = r as u64;
        }
        self.wr(rd, out);
    }
    pub fn pmultw(&mut self, rd: usize, rs: usize, rt: usize) {
        self.pmul_w(rd, rs, rt, true, 0)
    }
    pub fn pmultuw(&mut self, rd: usize, rs: usize, rt: usize) {
        self.pmul_w(rd, rs, rt, false, 0)
    }
    pub fn pmaddw(&mut self, rd: usize, rs: usize, rt: usize) {
        self.pmul_w(rd, rs, rt, true, 1)
    }
    pub fn pmadduw(&mut self, rd: usize, rs: usize, rt: usize) {
        self.pmul_w(rd, rs, rt, false, 1)
    }
    pub fn pmsubw(&mut self, rd: usize, rs: usize, rt: usize) {
        self.pmul_w(rd, rs, rt, true, 2)
    }
    pub fn pdivw(&mut self, rs: usize, rt: usize) {
        let (s, t) = (w(self.g[rs]), w(self.g[rt]));
        for (k, i) in [0usize, 2].into_iter().enumerate() {
            let (n, d) = (s[i] as i32, t[i] as i32);
            let (qq, r) = if d == 0 {
                (if n < 0 { 1 } else { -1 }, n)
            } else if n == i32::MIN && d == -1 {
                (i32::MIN, 0)
            } else {
                (n / d, n % d)
            };
            self.lo[k] = sx(qq as u32);
            self.hi[k] = sx(r as u32);
        }
    }
    pub fn pdivuw(&mut self, rs: usize, rt: usize) {
        let (s, t) = (w(self.g[rs]), w(self.g[rt]));
        for (k, i) in [0usize, 2].into_iter().enumerate() {
            let (n, d) = (s[i], t[i]);
            let (qq, r) = if d == 0 {
                (u32::MAX, n)
            } else {
                (n / d, n % d)
            };
            self.lo[k] = sx(qq);
            self.hi[k] = sx(r);
        }
    }
    pub fn pdivbw(&mut self, rs: usize, rt: usize) {
        let s = w(self.g[rs]);
        let d = self.g[rt][0] as u16 as i16 as i32;
        let mut lo = [0u32; 4];
        let mut hi = [0u32; 4];
        for i in 0..4 {
            let n = s[i] as i32;
            let (qq, r) = if d == 0 {
                (if n < 0 { 1 } else { -1 }, n)
            } else if n == i32::MIN && d == -1 {
                (i32::MIN, 0)
            } else {
                (n / d, n % d)
            };
            lo[i] = qq as u32;
            hi[i] = r as i16 as i32 as u32;
        }
        self.lo = uw(lo);
        self.hi = uw(hi);
    }
    fn pmul_h(&mut self, rd: usize, rs: usize, rt: usize, op: i32) {
        let (s, t) = (h(self.g[rs]), h(self.g[rt]));
        let p = |i: usize| (s[i] as i16 as i32).wrapping_mul(t[i] as i16 as i32);
        let mut lo = w(self.lo);
        let mut hi = w(self.hi);
        // lane i → (lo|hi, word index)
        let map = [
            (0, 0),
            (0, 1),
            (1, 0),
            (1, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 3),
        ];
        for (i, (which, idx)) in map.into_iter().enumerate() {
            let dst = if which == 0 {
                &mut lo[idx]
            } else {
                &mut hi[idx]
            };
            *dst = match op {
                0 => p(i) as u32,
                1 => (*dst as i32).wrapping_add(p(i)) as u32,
                _ => (*dst as i32).wrapping_sub(p(i)) as u32,
            };
        }
        self.lo = uw(lo);
        self.hi = uw(hi);
        self.wr(rd, uw([lo[0], hi[0], lo[2], hi[2]]));
    }
    pub fn pmulth(&mut self, rd: usize, rs: usize, rt: usize) {
        self.pmul_h(rd, rs, rt, 0)
    }
    pub fn pmaddh(&mut self, rd: usize, rs: usize, rt: usize) {
        self.pmul_h(rd, rs, rt, 1)
    }
    pub fn pmsubh(&mut self, rd: usize, rs: usize, rt: usize) {
        self.pmul_h(rd, rs, rt, 2)
    }
    fn phm(&mut self, rd: usize, rs: usize, rt: usize, sub: bool) {
        let (s, t) = (h(self.g[rs]), h(self.g[rt]));
        let p = |i: usize| (s[i] as i16 as i32).wrapping_mul(t[i] as i16 as i32);
        let f = |a: usize| {
            if sub {
                p(a + 1).wrapping_sub(p(a)) as u32
            } else {
                p(a + 1).wrapping_add(p(a)) as u32
            }
        };
        let mut lo = w(self.lo);
        let mut hi = w(self.hi);
        lo[0] = f(0);
        hi[0] = f(2);
        lo[2] = f(4);
        hi[2] = f(6);
        self.lo = uw(lo);
        self.hi = uw(hi);
        self.wr(rd, uw([lo[0], hi[0], lo[2], hi[2]]));
    }
    pub fn phmadh(&mut self, rd: usize, rs: usize, rt: usize) {
        self.phm(rd, rs, rt, false)
    }
    pub fn phmsbh(&mut self, rd: usize, rs: usize, rt: usize) {
        self.phm(rd, rs, rt, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cpu() -> Box<Cpu> {
        Cpu::detached()
    }

    #[test]
    fn pextlw_interleaves_low_words() {
        let mut c = cpu();
        c.g[1] = uw([1, 2, 3, 4]);
        c.g[2] = uw([5, 6, 7, 8]);
        c.pextlw(3, 1, 2);
        assert_eq!(w(c.g[3]), [5, 1, 6, 2]);
        c.pextuw(3, 1, 2);
        assert_eq!(w(c.g[3]), [7, 3, 8, 4]);
    }

    #[test]
    fn pcpy_and_qfsrv() {
        let mut c = cpu();
        c.g[1] = [0x11, 0x22];
        c.g[2] = [0x33, 0x44];
        c.pcpyld(3, 1, 2);
        assert_eq!(c.g[3], [0x33, 0x11]);
        c.pcpyud(3, 1, 2);
        assert_eq!(c.g[3], [0x22, 0x44]);
        c.g[4][0] = 4;
        c.mtsab(4, 0);
        assert_eq!(c.sa, 32);
        c.g[1] = uw([0xa, 0xb, 0xc, 0xd]);
        c.g[2] = uw([1, 2, 3, 4]);
        c.qfsrv(3, 1, 2);
        assert_eq!(w(c.g[3]), [2, 3, 4, 0xa]);
    }

    #[test]
    fn plzcw_counts_redundant_sign_bits() {
        let mut c = cpu();
        c.g[1][0] = 1 | (0xffff_ffffu64 << 32);
        c.plzcw(2, 1);
        assert_eq!(c.g[2][0] as u32, 30);
        assert_eq!((c.g[2][0] >> 32) as u32, 31);
    }

    #[test]
    fn madd_accumulates() {
        let mut c = cpu();
        c.g[1][0] = 3;
        c.g[2][0] = 4;
        c.mult(0, 0, 1, 2);
        c.madd(0, 5, 1, 2);
        assert_eq!(c.g[5][0], 24);
        assert_eq!(c.hi[0], 0);
    }
}
