//! GIF tag parsing for PATH1..3 (PACKED / REGLIST / IMAGE).

use super::Gs;
use crate::Qword;

#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct GifPath {
    pub active: bool,
    pub nloop: u32,
    pub loop_: u32,
    pub nreg: u32,
    pub reg: u32,
    pub flg: u32,
    pub regs: u64,
    pub eop: bool,
}

impl GifPath {
    /// Data qwords a GIF tag carries, and its EOP flag.
    pub(crate) fn tag_qwords(tag: Qword) -> (u64, bool) {
        let lo = tag as u64;
        let nloop = lo & 0x7FFF;
        let nreg = match (lo >> 60) & 0xF {
            0 => 16,
            n => n,
        };
        let n = match (lo >> 58) & 3 {
            0 => nloop * nreg,
            1 => (nloop * nreg).div_ceil(2),
            _ => nloop,
        };
        (n, (lo >> 15) & 1 != 0)
    }

    /// Data qwords left in the current tag (0 when idle), and its EOP flag.
    pub(crate) fn qwords_left(&self) -> (u64, bool) {
        if !self.active {
            return (0, self.eop);
        }
        let (nloop, loop_, nreg, reg) = (self.nloop as u64, self.loop_ as u64, self.nreg as u64, self.reg as u64);
        let n = match self.flg {
            0 => (nloop - loop_) * nreg - reg,
            1 => {
                let total = nloop * nreg;
                let done = loop_ * nreg + reg;
                total.div_ceil(2) - done / 2
            }
            _ => nloop - loop_,
        };
        (n, self.eop)
    }

    #[inline]
    fn cur_reg(&self) -> u8 {
        ((self.regs >> (self.reg * 4)) & 0xF) as u8
    }

    /// Advance to the next register descriptor; returns false when the packet ends.
    #[inline]
    fn step(&mut self) -> bool {
        self.reg += 1;
        if self.reg >= self.nreg {
            self.reg = 0;
            self.loop_ += 1;
            if self.loop_ >= self.nloop {
                self.active = false;
                return false;
            }
        }
        true
    }
}

impl Gs {
    pub(super) fn gif_qword(&mut self, path: usize, q: Qword) {
        let lo = q as u64;
        let hi = (q >> 64) as u64;
        let p = &mut self.paths[path];
        if !p.active {
            p.nloop = (lo & 0x7FFF) as u32;
            p.eop = (lo >> 15) & 1 != 0;
            p.flg = ((lo >> 58) & 3) as u32;
            p.nreg = ((lo >> 60) & 0xF) as u32;
            if p.nreg == 0 {
                p.nreg = 16;
            }
            p.regs = hi;
            p.loop_ = 0;
            p.reg = 0;
            p.active = p.nloop != 0;
            if crate::trace_path3() && path != 1 {
                eprintln!("[p3]   tag path {path} nloop {} flg {} nreg {} eop {}", p.nloop, p.flg, p.nreg, p.eop);
            }
            let pre = (lo >> 46) & 1 != 0;
            let flg = p.flg;
            // Each GIFtag resets the internal Q to 1.0.
            self.internal_q = 1.0;
            if pre && flg == 0 {
                self.reg(0x00, (lo >> 47) & 0x7FF);
            }
            return;
        }
        match p.flg {
            0 => {
                let reg = p.cur_reg();
                p.step();
                self.write_packed(reg, lo, hi);
            }
            1 => {
                for half in [lo, hi] {
                    let p = &mut self.paths[path];
                    if !p.active {
                        break; // odd NREG*NLOOP: the last 64 bits are padding
                    }
                    let reg = p.cur_reg();
                    p.step();
                    if reg < 0xE {
                        self.reg(reg, half);
                    }
                }
            }
            _ => {
                p.loop_ += 1;
                if p.loop_ >= p.nloop {
                    p.active = false;
                }
                let bytes = q.to_le_bytes();
                self.transfer_bytes(&bytes);
            }
        }
    }

    fn write_packed(&mut self, reg: u8, lo: u64, hi: u64) {
        let w0 = lo as u32;
        let w1 = (lo >> 32) as u32;
        let w2 = hi as u32;
        let w3 = (hi >> 32) as u32;
        match reg {
            0x0 => self.reg(0x00, lo & 0x7FF),
            0x1 => {
                let v = (w0 & 0xFF) as u64
                    | ((w1 & 0xFF) as u64) << 8
                    | ((w2 & 0xFF) as u64) << 16
                    | ((w3 & 0xFF) as u64) << 24
                    | (self.internal_q.to_bits() as u64) << 32;
                self.reg(0x01, v);
            }
            0x2 => {
                self.internal_q = f32::from_bits(w2);
                self.reg(0x02, lo);
            }
            0x3 => self.reg(0x03, (w0 & 0x3FFF) as u64 | ((w1 & 0x3FFF) as u64) << 16),
            0x4 => {
                let v = (w0 & 0xFFFF) as u64
                    | ((w1 & 0xFFFF) as u64) << 16
                    | (((w2 >> 4) & 0xFF_FFFF) as u64) << 32
                    | (((w3 >> 4) & 0xFF) as u64) << 56;
                let adc = (hi >> 47) & 1 != 0;
                self.reg(if adc { 0x0C } else { 0x04 }, v);
            }
            0x5 => {
                let v = (w0 & 0xFFFF) as u64 | ((w1 & 0xFFFF) as u64) << 16 | (w2 as u64) << 32;
                let adc = (hi >> 47) & 1 != 0;
                self.reg(if adc { 0x0D } else { 0x05 }, v);
            }
            0x6..=0x9 => self.reg(reg, lo),
            0xA => self.reg(0x0A, (((w3 >> 4) & 0xFF) as u64) << 56),
            0xC => {
                let v = (w0 & 0xFFFF) as u64
                    | ((w1 & 0xFFFF) as u64) << 16
                    | (((w2 >> 4) & 0xFF_FFFF) as u64) << 32
                    | (((w3 >> 4) & 0xFF) as u64) << 56;
                self.reg(0x0C, v);
            }
            0xD => {
                let v = (w0 & 0xFFFF) as u64 | ((w1 & 0xFFFF) as u64) << 16 | (w2 as u64) << 32;
                self.reg(0x0D, v);
            }
            0xE => self.reg((hi & 0xFF) as u8, lo),
            _ => {} // 0xB reserved, 0xF NOP
        }
    }
}
