//! IOP interrupt controller and DMA controller (the parts the sound modules
//! use: SPU2 DMA channels 4 and 7, DICR/DICR2 completion flags).
//!
//! Interrupt line numbering follows intrman: 0..31 are INTC lines (0 VBlank,
//! 3 DMA, 9 SPU2, ...); 32+ch are DMA channels 0..6 and 40+(ch-7) channels 7..12.
//! Ported from ps2-web-recomp `runtime/src/iop/hw.cpp` (MIT).

use crate::{kernel::Timed, Iop};

pub const LINE_VBLANK: u32 = 0;
pub const LINE_DMA: u32 = 3;
pub const LINE_SPU2: u32 = 9;

#[derive(Default, Clone, Copy, Debug)]
pub struct DmaChan {
    pub madr: u32,
    pub bcr: u32,
    pub chcr: u32,
    pub tadr: u32,
}

#[derive(Debug)]
pub struct Hw {
    pub istat: u32,
    pub imask: u32,
    pub ictrl: u32,
    pub dicr: u32,
    pub dicr2: u32,
    pub dpcr: u32,
    pub dpcr2: u32,
    pub dma: [DmaChan; 14],
}

impl Default for Hw {
    fn default() -> Self {
        Self {
            istat: 0,
            imask: 0,
            ictrl: 1,
            dicr: 0,
            dicr2: 0,
            dpcr: 0,
            dpcr2: 0,
            dma: [DmaChan::default(); 14],
        }
    }
}

impl Hw {
    pub fn raise(&mut self, line: u32) {
        if line < 32 {
            self.istat |= 1 << line;
        }
    }

    pub fn set_mask(&mut self, line: u32, on: bool) {
        if line < 32 {
            if on {
                self.imask |= 1 << line;
            } else {
                self.imask &= !(1 << line);
            }
            return;
        }
        if on {
            self.imask |= 1 << LINE_DMA;
            self.dicr |= 0x80_0000;
            self.dicr2 |= 0x80_0000;
        }
        let (reg, bit) = if line < 39 {
            (&mut self.dicr, 1 << (16 + line - 32))
        } else {
            (&mut self.dicr2, 1 << (16 + line - 40))
        };
        if on {
            *reg |= bit;
        } else {
            *reg &= !bit;
        }
    }

    fn next_pending(&mut self) -> Option<u32> {
        let pending = self.istat & self.imask;
        if pending & (1 << LINE_DMA) != 0 {
            let f = (self.dicr >> 24) & (self.dicr >> 16) & 0x7F;
            if f != 0 {
                return Some(32 + f.trailing_zeros());
            }
            let f2 = (self.dicr2 >> 24) & (self.dicr2 >> 16) & 0x3F;
            if f2 != 0 {
                return Some(40 + f2.trailing_zeros());
            }
            self.istat &= !(1 << LINE_DMA);
        }
        let rest = self.istat & self.imask & !(1 << LINE_DMA);
        (rest != 0).then(|| rest.trailing_zeros())
    }

    fn acknowledge(&mut self, line: u32) {
        if line < 32 {
            self.istat &= !(1 << line);
            return;
        }
        if line < 39 {
            self.dicr &= !(1 << (24 + line - 32));
        } else {
            self.dicr2 &= !(1 << (24 + line - 40));
        }
        if ((self.dicr >> 24) & 0x7F) | ((self.dicr2 >> 24) & 0x3F) == 0 {
            self.istat &= !(1 << LINE_DMA);
        }
    }
}

/// Run registered handlers for pending, enabled interrupts (nested calls).
pub fn deliver_interrupts(c: &mut Iop) {
    if c.in_handler > 0 {
        return;
    }
    for _ in 0..64 {
        let Some(line) = c.hw.next_pending() else {
            return;
        };
        c.hw.acknowledge(line);
        let Some(h) = c.k.intr.get(&line).copied() else {
            continue;
        };
        if h.handler == 0 {
            continue;
        }
        if c.trace {
            eprintln!("[iop] interrupt {line} -> {:#x} at {}", h.handler, c.cycles);
        }
        let r = c.call_guest(h.handler, &[h.arg]);
        if r == 0 && line < 32 {
            c.hw.set_mask(line, false);
        }
    }
}

pub fn dma_done(c: &mut Iop, ch: u32) {
    let ch = ch as usize;
    c.hw.dma[ch].chcr &= !0x0100_0000;
    if ch < 7 {
        c.hw.dicr |= 1 << (24 + ch);
    } else {
        c.hw.dicr2 |= 1 << (24 + ch - 7);
    }
    c.hw.istat |= 1 << LINE_DMA;
}

fn dma_channel(a: u32) -> Option<(usize, u32)> {
    if (0x1F80_1080..0x1F80_10F0).contains(&a) {
        return Some((((a - 0x1F80_1080) >> 4) as usize, (a & 0xF) >> 2));
    }
    if (0x1F80_1500..0x1F80_1570).contains(&a) {
        return Some((7 + ((a - 0x1F80_1500) >> 4) as usize, (a & 0xF) >> 2));
    }
    None
}

fn start_dma(c: &mut Iop, ch: usize) {
    let d = c.hw.dma[ch];
    let blocks = (d.bcr >> 16).max(1);
    let words = (d.bcr & 0xFFFF) * blocks;
    if c.trace {
        eprintln!(
            "[iop-dma] ch {ch} madr {:#x} bcr {:#x} chcr {:#x}",
            d.madr, d.bcr, d.chcr
        );
    }
    if ch == 4 || ch == 7 {
        crate::spu2::dma(
            c,
            if ch == 4 { 0 } else { 1 },
            d.madr,
            words,
            d.chcr & 1 != 0,
            ch as u32,
        );
        return;
    }
    if ch == 11 || ch == 12 {
        // SIO2 in (memory → SIO2) / out (SIO2 → memory): instant.
        let block = ((d.bcr & 0xFFFF) * 4) as usize;
        let bytes = (words * 4) as usize;
        if c.sio2.trace {
            eprintln!(
                "[sio2] dma ch {ch} madr {:#x} bcr {:#x} chcr {:#x}",
                d.madr, d.bcr, d.chcr
            );
        }
        if ch == 11 {
            let data = c.read_bytes(d.madr, bytes);
            c.sio2.dma_in(&data, block);
        } else {
            let data = c.sio2.dma_out(bytes);
            c.write_bytes(d.madr, &data);
        }
        c.hw.dma[ch].madr = d.madr.wrapping_add(bytes as u32);
        let at = c.cycles + 16 + words as u64;
        c.k.schedule(at, Timed::DmaDone(ch as u32));
        return;
    }
    eprintln!(
        "[iop-dma] channel {ch} not emulated (madr {:#x} bcr {:#x} chcr {:#x})",
        d.madr, d.bcr, d.chcr
    );
    let at = c.cycles + 64;
    c.k.schedule(at, Timed::DmaDone(ch as u32));
}

/// dmacman `dmac_request`: program a channel without starting it.
pub fn dma_request(c: &mut Iop, ch: usize, madr: u32, words: u32, blocks: u32, to_device: bool) {
    if ch >= c.hw.dma.len() {
        return;
    }
    let d = &mut c.hw.dma[ch];
    d.madr = madr & 0x00FF_FFFF;
    d.bcr = (words & 0xFFFF) | (blocks << 16);
    d.chcr = 0x200 | to_device as u32;
}

/// dmacman `dmac_transfer`: start a programmed channel.
pub fn dma_transfer(c: &mut Iop, ch: usize) {
    if ch >= c.hw.dma.len() {
        return;
    }
    c.hw.dma[ch].chcr |= 0x0100_0000;
    start_dma(c, ch);
}

pub fn read(c: &mut Iop, a: u32, _size: u32) -> Option<u32> {
    if let Some(v) = c.sio2.read(a) {
        return Some(v);
    }
    Some(match a {
        0x1F80_1070 => c.hw.istat,
        0x1F80_1074 => c.hw.imask,
        0x1F80_1078 => {
            let v = c.hw.ictrl;
            c.hw.ictrl = 0;
            v
        }
        0x1F80_10F0 => c.hw.dpcr,
        0x1F80_10F4 => c.hw.dicr,
        0x1F80_1570 => c.hw.dpcr2,
        0x1F80_1574 => c.hw.dicr2,
        _ => {
            if let Some((ch, reg)) = dma_channel(a) {
                let d = &c.hw.dma[ch];
                return Some([d.madr, d.bcr, d.chcr, d.tadr][reg as usize]);
            }
            // Root counters: report the IOP clock.
            if (0x1F80_1100..0x1F80_1130).contains(&a) || (0x1F80_1480..0x1F80_14B0).contains(&a) {
                if a & 0xF == 0 {
                    return Some(c.cycles as u32);
                }
            }
            return None;
        }
    })
}

pub fn write(c: &mut Iop, a: u32, v: u32, size: u32) -> bool {
    if (crate::sio2::BASE..crate::sio2::END).contains(&a) {
        if c.sio2.write(a, v) {
            c.hw.raise(crate::sio2::LINE_SIO2);
        }
        return true;
    }
    match a {
        0x1F80_1070 => c.hw.istat &= v,
        0x1F80_1074 => c.hw.imask = v,
        0x1F80_1078 => c.hw.ictrl = v & 1,
        0x1F80_10F0 => c.hw.dpcr = v,
        0x1F80_10F4 => {
            c.hw.dicr = (c.hw.dicr & 0x7F00_0000 & !(v & 0x7F00_0000)) | (v & 0x00FF_FFFF)
        }
        0x1F80_1570 => c.hw.dpcr2 = v,
        0x1F80_1574 => {
            c.hw.dicr2 = (c.hw.dicr2 & 0x7F00_0000 & !(v & 0x7F00_0000)) | (v & 0x00FF_FFFF)
        }
        _ => {
            let Some((ch, reg)) = dma_channel(a & !3) else {
                return false;
            };
            let shift = (a & 3) * 8;
            let mask = if size >= 4 {
                u32::MAX
            } else {
                ((1u32 << (size * 8)) - 1) << shift
            };
            let d = &mut c.hw.dma[ch];
            let t = match reg {
                0 => &mut d.madr,
                1 => &mut d.bcr,
                2 => &mut d.chcr,
                _ => &mut d.tadr,
            };
            *t = (*t & !mask) | ((v << shift) & mask);
            if reg == 2 && d.chcr & 0x0100_0000 != 0 && mask & 0x0100_0000 != 0 {
                start_dma(c, ch);
            }
        }
    }
    true
}
