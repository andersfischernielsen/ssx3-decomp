//! Memory-mapped EE hardware: INTC, DMAC (with chain tags), timers, GIF/VIF
//! registers and FIFOs, SIF mailbox registers, and the VU memory window.
//!
//! Addresses are physical. Register layout follows the EE User's Manual (see
//! `docs/ps2-bible.md`); DMA transfers complete instantly when started, like
//! ps2-web-recomp's `dmac.cpp`.

use crate::gs::Gs;
use crate::vif::Vif;
use crate::vu::Vu;
use crate::Qword;

/// INTC cause bits.
pub mod int {
    pub const GS: u32 = 0;
    pub const SBUS: u32 = 1;
    pub const VBON: u32 = 2;
    pub const VBOF: u32 = 3;
    pub const VIF0: u32 = 4;
    pub const VIF1: u32 = 5;
    pub const VU0: u32 = 6;
    pub const VU1: u32 = 7;
    pub const IPU: u32 = 8;
    pub const TIM0: u32 = 9;
}

/// DMA channel indices.
pub mod ch {
    pub const VIF0: usize = 0;
    pub const VIF1: usize = 1;
    pub const GIF: usize = 2;
    pub const FROM_IPU: usize = 3;
    pub const TO_IPU: usize = 4;
    pub const SIF0: usize = 5;
    pub const SIF1: usize = 6;
    pub const SIF2: usize = 7;
    pub const FROM_SPR: usize = 8;
    pub const TO_SPR: usize = 9;
}

const CH_BASE: [u32; 10] = [
    0x1000_8000, 0x1000_9000, 0x1000_A000, 0x1000_B000, 0x1000_B400, 0x1000_C000, 0x1000_C400,
    0x1000_C800, 0x1000_D000, 0x1000_D400,
];

#[derive(Default, Clone, Copy, Debug)]
pub struct Channel {
    pub chcr: u32,
    pub madr: u32,
    pub qwc: u32,
    pub tadr: u32,
    pub asr: [u32; 2],
    pub sadr: u32,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Timer {
    pub mode: u32,
    pub comp: u32,
    pub hold: u32,
    /// Counter value at `base` (in EE cycles).
    pub count_at_base: u32,
    pub base: u64,
}

pub struct Hw {
    pub i_stat: u32,
    pub i_mask: u32,
    pub ch: [Channel; 10],
    pub d_ctrl: u32,
    pub d_stat: u32,
    pub d_pcr: u32,
    pub d_sqwc: u32,
    pub d_rbsr: u32,
    pub d_rbor: u32,
    pub d_enable: u32,
    pub timers: [Timer; 4],
    pub gif_ctrl: u32,
    pub gif_mode: u32,
    pub sif_mscom: u32,
    pub sif_smcom: u32,
    pub sif_msflg: u32,
    pub sif_smflg: u32,
    pub sif_ctrl: u32,
    pub gs: Gs,
    pub vif0: Vif,
    pub vif1: Vif,
    pub vu0: Vu,
    pub vu1: Vu,
    /// Current EE cycle count, refreshed by the CPU before I/O accesses.
    pub now: u64,
    /// Set when an interrupt source changed; the CPU then services events.
    pub irq_dirty: bool,
    /// Debug output written via the SIO TX register.
    pub sio_line: Vec<u8>,
    pub log_unknown: bool,
    pub unknown: std::collections::BTreeMap<u32, u64>,
}

impl Default for Hw {
    fn default() -> Self {
        Self::new()
    }
}

/// Read one quadword from RAM or scratchpad (`spr` selects the scratchpad).
pub fn mem_q(ram: &[u8], sprm: &[u8], addr: u32, spr: bool) -> Qword {
    if spr || (addr >> 28) == 7 {
        let o = (addr & 0x3FF0) as usize;
        u128::from_le_bytes(sprm[o..o + 16].try_into().unwrap())
    } else {
        let o = (addr & 0x01FF_FFF0) as usize;
        u128::from_le_bytes(ram[o..o + 16].try_into().unwrap())
    }
}

pub fn mem_q_write(ram: &mut [u8], sprm: &mut [u8], addr: u32, spr: bool, v: Qword) {
    if spr || (addr >> 28) == 7 {
        let o = (addr & 0x3FF0) as usize;
        sprm[o..o + 16].copy_from_slice(&v.to_le_bytes());
    } else {
        let o = (addr & 0x01FF_FFF0) as usize;
        ram[o..o + 16].copy_from_slice(&v.to_le_bytes());
    }
}

impl Hw {
    pub fn new() -> Self {
        Self {
            i_stat: 0,
            i_mask: 0,
            ch: [Channel::default(); 10],
            d_ctrl: 0,
            d_stat: 0,
            d_pcr: 0,
            d_sqwc: 0,
            d_rbsr: 0,
            d_rbor: 0,
            d_enable: 0x1201,
            timers: [Timer::default(); 4],
            gif_ctrl: 0,
            gif_mode: 0,
            sif_mscom: 0,
            sif_smcom: 0,
            sif_msflg: 0,
            sif_smflg: 0,
            sif_ctrl: 0,
            gs: Gs::new(),
            vif0: Vif::new(0),
            vif1: Vif::new(1),
            vu0: Vu::new(0),
            vu1: Vu::new(1),
            now: 0,
            irq_dirty: false,
            sio_line: Vec::new(),
            log_unknown: true,
            unknown: Default::default(),
        }
    }

    /// Raise an INTC cause bit.
    pub fn raise(&mut self, bit: u32) {
        self.i_stat |= 1 << bit;
        self.irq_dirty = true;
    }

    /// INTC causes that are pending and unmasked.
    pub fn intc_pending(&self) -> u32 {
        self.i_stat & self.i_mask
    }

    /// DMAC channels whose completion interrupt is pending and unmasked.
    pub fn dmac_pending(&self) -> u32 {
        (self.d_stat & 0x3ff) & ((self.d_stat >> 16) & 0x3ff)
    }

    fn note_unknown(&mut self, p: u32) {
        if self.log_unknown {
            *self.unknown.entry(p).or_default() += 1;
        }
    }

    // ------------------------------------------------------------- timers

    fn timer_div(mode: u32) -> u64 {
        // EE cycles per tick: bus clock is EE/2.
        match mode & 3 {
            0 => 2,
            1 => 32,
            2 => 512,
            _ => 18_743, // ~one H-blank (294.912 MHz / 15.734 kHz)
        }
    }

    fn timer_count(&self, i: usize) -> u32 {
        let t = &self.timers[i];
        if t.mode & 0x80 == 0 {
            return t.count_at_base;
        }
        let ticks = (self.now.saturating_sub(t.base)) / Self::timer_div(t.mode);
        let mut v = t.count_at_base as u64 + ticks;
        if t.mode & 0x40 != 0 && t.comp != 0 {
            v %= t.comp as u64 + 1;
        }
        (v & 0xffff) as u32
    }

    fn timer_rebase(&mut self, i: usize) {
        let c = self.timer_count(i);
        self.timers[i].count_at_base = c;
        self.timers[i].base = self.now;
    }

    // -------------------------------------------------------------- reads

    pub fn read32(&mut self, p: u32) -> u32 {
        match p {
            0x1000_0000..=0x1000_1FFF => {
                let i = ((p >> 11) & 3) as usize;
                match p & 0x7F0 {
                    0x00 => self.timer_count(i),
                    0x10 => self.timers[i].mode,
                    0x20 => self.timers[i].comp,
                    0x30 => self.timers[i].hold,
                    _ => 0,
                }
            }
            0x1000_2000..=0x1000_2FFF => 0, // IPU: idle
            0x1000_3000 => self.gif_ctrl,
            0x1000_3010 => self.gif_mode,
            0x1000_3020 => 0, // GIF_STAT: idle, no path active
            0x1000_3800..=0x1000_3BFF => self.vif0.read_reg(p - 0x1000_3800),
            0x1000_3C00..=0x1000_3FFF => self.vif1.read_reg(p - 0x1000_3C00),
            0x1000_8000..=0x1000_DFFF => {
                let Some(c) = CH_BASE.iter().position(|b| p & !0xFF == *b) else {
                    return 0;
                };
                let chn = &self.ch[c];
                match p & 0xF0 {
                    0x00 => chn.chcr,
                    0x10 => chn.madr,
                    0x20 => chn.qwc,
                    0x30 => chn.tadr,
                    0x40 => chn.asr[0],
                    0x50 => chn.asr[1],
                    0x80 => chn.sadr,
                    _ => 0,
                }
            }
            0x1000_E000 => self.d_ctrl,
            0x1000_E010 => self.d_stat,
            0x1000_E020 => self.d_pcr,
            0x1000_E030 => self.d_sqwc,
            0x1000_E040 => self.d_rbsr,
            0x1000_E050 => self.d_rbor,
            0x1000_F000 => self.i_stat,
            0x1000_F010 => self.i_mask,
            0x1000_F200 => self.sif_mscom,
            0x1000_F210 => self.sif_smcom,
            0x1000_F220 => self.sif_msflg,
            0x1000_F230 => self.sif_smflg,
            0x1000_F240 => self.sif_ctrl,
            0x1000_F520 => self.d_enable,
            0x1100_0000..=0x1100_FFFF => {
                let (mem, off) = self.vu_mem(p);
                u32::from_le_bytes(mem[off..off + 4].try_into().unwrap())
            }
            0x1200_0000..=0x1200_1FFF => {
                let v = self.gs.read_priv(p & 0x1FF8 & !7);
                if p & 4 != 0 {
                    (v >> 32) as u32
                } else {
                    v as u32
                }
            }
            _ => {
                self.note_unknown(p);
                0
            }
        }
    }

    pub fn read64(&mut self, p: u32) -> u64 {
        if (0x1200_0000..0x1200_2000).contains(&p) {
            return self.gs.read_priv(p & 0x1FF8);
        }
        self.read32(p) as u64 | (self.read32(p + 4) as u64) << 32
    }

    pub fn read128(&mut self, p: u32) -> Qword {
        match p {
            0x1000_6000 => self.gs.read_local(1).first().copied().unwrap_or(0),
            0x1100_0000..=0x1100_FFFF => {
                let (mem, off) = self.vu_mem(p);
                u128::from_le_bytes(mem[off..off + 16].try_into().unwrap())
            }
            _ => {
                let lo = self.read64(p) as u128;
                lo | (self.read64(p + 8) as u128) << 64
            }
        }
    }

    fn vu_mem(&mut self, p: u32) -> (&mut Vec<u8>, usize) {
        let off = (p & 0xFFFF) as usize;
        match off {
            0x0000..=0x3FFF => (&mut self.vu0.code, off & 0xFFF),
            0x4000..=0x7FFF => (&mut self.vu0.data, off & 0xFFF),
            0x8000..=0xBFFF => (&mut self.vu1.code, off & 0x3FFF),
            _ => (&mut self.vu1.data, off & 0x3FFF),
        }
    }

    // ------------------------------------------------------------- writes

    pub fn write8(&mut self, p: u32, v: u8, ram: &mut [u8], spr: &mut [u8]) {
        if p == 0x1000_F180 {
            self.sio_putc(v);
            return;
        }
        let cur = self.read32(p & !3);
        let sh = (p & 3) * 8;
        self.write32(p & !3, (cur & !(0xff << sh)) | ((v as u32) << sh), ram, spr);
    }

    pub fn write16(&mut self, p: u32, v: u16, ram: &mut [u8], spr: &mut [u8]) {
        let cur = self.read32(p & !3);
        let sh = (p & 2) * 8;
        self.write32(p & !3, (cur & !(0xffff << sh)) | ((v as u32) << sh), ram, spr);
    }

    fn sio_putc(&mut self, v: u8) {
        if v == b'\n' {
            let s = String::from_utf8_lossy(&self.sio_line).into_owned();
            eprintln!("[ee-sio] {s}");
            self.sio_line.clear();
        } else if v != b'\r' {
            self.sio_line.push(v);
        }
    }

    pub fn write32(&mut self, p: u32, v: u32, ram: &mut [u8], spr: &mut [u8]) {
        match p {
            0x1000_0000..=0x1000_1FFF => {
                let i = ((p >> 11) & 3) as usize;
                match p & 0x7F0 {
                    0x00 => {
                        self.timers[i].count_at_base = v & 0xffff;
                        self.timers[i].base = self.now;
                    }
                    0x10 => {
                        self.timer_rebase(i);
                        // Writing 1 to EQUF/OVFF clears them.
                        let flags = self.timers[i].mode & 0xC00 & !(v & 0xC00);
                        self.timers[i].mode = (v & 0x3FF) | flags;
                    }
                    0x20 => self.timers[i].comp = v & 0xffff,
                    0x30 => self.timers[i].hold = v & 0xffff,
                    _ => {}
                }
            }
            0x1000_2000..=0x1000_2FFF => {} // IPU commands: ignored for now
            0x1000_3000 => {
                self.gif_ctrl = v & 8;
                if v & 1 != 0 {
                    self.gif_ctrl = 0;
                }
            }
            0x1000_3010 => self.gif_mode = v,
            0x1000_3800..=0x1000_3BFF => {
                self.vif0.write_reg(p - 0x1000_3800, v);
                if !self.vif0.stalled() && self.vif0.pending_words() > 0 {
                    self.vif0.resume(&mut self.vu0, &mut self.gs);
                    self.vif_irq();
                }
            }
            0x1000_3C00..=0x1000_3FFF => {
                self.vif1.write_reg(p - 0x1000_3C00, v);
                if !self.vif1.stalled() && self.vif1.pending_words() > 0 {
                    self.vif1.resume(&mut self.vu1, &mut self.gs);
                    self.vif_irq();
                }
            }
            0x1000_8000..=0x1000_DFFF => {
                let Some(c) = CH_BASE.iter().position(|b| p & !0xFF == *b) else {
                    return;
                };
                match p & 0xF0 {
                    0x00 => {
                        self.ch[c].chcr = v;
                        if v & 0x100 != 0 {
                            self.dma_start(c, ram, spr);
                        }
                    }
                    0x10 => self.ch[c].madr = v,
                    0x20 => self.ch[c].qwc = v & 0xffff,
                    0x30 => self.ch[c].tadr = v,
                    0x40 => self.ch[c].asr[0] = v,
                    0x50 => self.ch[c].asr[1] = v,
                    0x80 => self.ch[c].sadr = v & 0x3FFF,
                    _ => {}
                }
            }
            0x1000_E000 => self.d_ctrl = v,
            0x1000_E010 => {
                // Low bits: write 1 clears status; high bits: write 1 toggles mask.
                let stat = self.d_stat & 0x3ff & !(v & 0x3ff);
                let mask = ((self.d_stat >> 16) ^ (v >> 16)) & 0x3ff;
                let other = self.d_stat & 0xE000_E000 & !(v & 0xE000);
                self.d_stat = stat | (mask << 16) | other;
                self.irq_dirty = true;
            }
            0x1000_E020 => self.d_pcr = v,
            0x1000_E030 => self.d_sqwc = v,
            0x1000_E040 => self.d_rbsr = v,
            0x1000_E050 => self.d_rbor = v,
            0x1000_F000 => {
                self.i_stat &= !v;
                self.irq_dirty = true;
            }
            0x1000_F010 => {
                self.i_mask ^= v & 0x7fff;
                self.irq_dirty = true;
            }
            0x1000_F180 => self.sio_putc(v as u8),
            0x1000_F200 => self.sif_mscom = v,
            0x1000_F210 => self.sif_smcom = v,
            0x1000_F220 => self.sif_msflg |= v,
            0x1000_F230 => self.sif_smflg &= !v,
            0x1000_F240 => self.sif_ctrl = v,
            0x1000_F590 => self.d_enable = v,
            0x1100_0000..=0x1100_FFFF => {
                let (mem, off) = self.vu_mem(p);
                mem[off..off + 4].copy_from_slice(&v.to_le_bytes());
            }
            0x1200_0000..=0x1200_1FFF => {
                let a = p & 0x1FF8;
                let cur = self.gs.read_priv(a);
                let nv = if p & 4 != 0 {
                    (cur & 0xffff_ffff) | (v as u64) << 32
                } else {
                    (cur & !0xffff_ffff) | v as u64
                };
                self.gs.write_priv(a, nv);
                self.irq_dirty = true;
            }
            _ => self.note_unknown(p),
        }
    }

    pub fn write64(&mut self, p: u32, v: u64, ram: &mut [u8], spr: &mut [u8]) {
        if (0x1200_0000..0x1200_2000).contains(&p) {
            self.gs.write_priv(p & 0x1FF8, v);
            self.irq_dirty = true;
            return;
        }
        self.write32(p, v as u32, ram, spr);
        self.write32(p + 4, (v >> 32) as u32, ram, spr);
    }

    pub fn write128(&mut self, p: u32, v: Qword, ram: &mut [u8], spr: &mut [u8]) {
        match p & !0xF {
            0x1000_4000 => {
                self.vif0.write(&[v], &mut self.vu0, &mut self.gs);
                self.vif_irq();
            }
            0x1000_5000 => {
                self.vif1.write(&[v], &mut self.vu1, &mut self.gs);
                self.vif_irq();
            }
            0x1000_6000 => {
                self.gs.gif_write(3, &[v]);
                self.irq_dirty = true;
            }
            0x1000_7010 => {} // IPU in FIFO
            0x1100_0000..=0x1100_FFFF => {
                let (mem, off) = self.vu_mem(p);
                mem[off..off + 16].copy_from_slice(&v.to_le_bytes());
            }
            _ => {
                self.write64(p, v as u64, ram, spr);
                self.write64(p + 8, (v >> 64) as u64, ram, spr);
            }
        }
    }

    fn vif_irq(&mut self) {
        if self.vif0.take_irq() {
            self.raise(int::VIF0);
        }
        if self.vif1.take_irq() {
            self.raise(int::VIF1);
        }
    }

    // ---------------------------------------------------------------- DMA

    /// Run a DMA transfer to completion.
    fn dma_start(&mut self, c: usize, ram: &mut [u8], spr: &mut [u8]) {
        let chcr = self.ch[c].chcr;
        let mode = (chcr >> 2) & 3;
        let to_mem = chcr & 1 == 0;
        match c {
            ch::SIF0 | ch::SIF1 | ch::SIF2 => {
                // SIF channels are driven by the IOP HLE (see `sif.rs`); the
                // EE only arms them. Leave STR set so the IOP side can see it.
                return;
            }
            ch::FROM_IPU | ch::TO_IPU => {}
            ch::FROM_SPR => {
                // SPR → RAM, normal mode (chain/dest-chain treated as normal).
                let n = self.ch[c].qwc;
                for i in 0..n {
                    let v = mem_q(ram, spr, self.ch[c].sadr + i * 16, true);
                    let madr = self.ch[c].madr;
                    mem_q_write(ram, spr, madr + i * 16, madr & 0x8000_0000 != 0, v);
                }
                self.ch[c].madr += n * 16;
                self.ch[c].sadr = (self.ch[c].sadr + n * 16) & 0x3FFF;
                self.ch[c].qwc = 0;
            }
            _ if c == ch::VIF1 && to_mem && mode == 0 => {
                // VIF1 → memory (GS download path). Rare; complete empty.
                self.ch[c].qwc = 0;
            }
            _ => {
                if mode == 1 {
                    self.dma_chain(c, ram, spr);
                } else {
                    let n = self.ch[c].qwc;
                    let madr = self.ch[c].madr;
                    let data: Vec<Qword> = (0..n)
                        .map(|i| mem_q(ram, spr, madr + i * 16, madr & 0x8000_0000 != 0))
                        .collect();
                    self.ch[c].madr += n * 16;
                    self.ch[c].qwc = 0;
                    self.dma_sink(c, &data, ram, spr);
                }
            }
        }
        self.ch[c].chcr &= !0x100;
        self.d_stat |= 1 << c;
        self.irq_dirty = true;
    }

    /// Deliver source data to the channel's peripheral.
    fn dma_sink(&mut self, c: usize, data: &[Qword], ram: &mut [u8], spr: &mut [u8]) {
        match c {
            ch::VIF0 => {
                self.vif0.write(data, &mut self.vu0, &mut self.gs);
                self.vif_irq();
            }
            ch::VIF1 => {
                self.vif1.write(data, &mut self.vu1, &mut self.gs);
                self.vif_irq();
            }
            ch::GIF => self.gs.gif_write(3, data),
            ch::TO_SPR => {
                for (i, v) in data.iter().enumerate() {
                    let a = (self.ch[c].sadr + i as u32 * 16) & 0x3FFF;
                    mem_q_write(ram, spr, a, true, *v);
                }
                self.ch[c].sadr = (self.ch[c].sadr + data.len() as u32 * 16) & 0x3FFF;
            }
            _ => {}
        }
    }

    /// Source-chain mode: walk DMAtags.
    fn dma_chain(&mut self, c: usize, ram: &mut [u8], spr: &mut [u8]) {
        let tte = self.ch[c].chcr & 0x40 != 0;
        let tie = self.ch[c].chcr & 0x80 != 0;
        let mut asp = (self.ch[c].chcr >> 4) & 3;
        // If QWC is non-zero the channel first finishes the pending data.
        if self.ch[c].qwc > 0 {
            let n = self.ch[c].qwc;
            let madr = self.ch[c].madr;
            let data: Vec<Qword> = (0..n)
                .map(|i| mem_q(ram, spr, madr + i * 16, madr & 0x8000_0000 != 0))
                .collect();
            self.ch[c].qwc = 0;
            self.dma_sink(c, &data, ram, spr);
            let id = (self.ch[c].chcr >> 28) & 7;
            if id == 0 || id == 7 {
                return;
            }
        }
        for _ in 0..1_000_000 {
            let tadr = self.ch[c].tadr;
            let tag = mem_q(ram, spr, tadr, tadr & 0x8000_0000 != 0);
            let qwc = (tag & 0xffff) as u32;
            let id = ((tag >> 28) & 7) as u32;
            let irq = (tag >> 31) & 1 != 0;
            let addr = ((tag >> 32) & 0x7fff_fff0) as u32;
            let tag_spr = (tag >> 63) & 1 != 0;
            self.ch[c].chcr = (self.ch[c].chcr & 0xffff) | (((tag as u32) >> 16) << 16);
            if tte && (c == ch::VIF0 || c == ch::VIF1) {
                // The upper 64 bits of the tag are VIFcodes.
                let t = tag & !0xffff_ffff_ffff_ffffu128;
                self.dma_sink(c, &[t], ram, spr);
            }
            let (data_addr, data_spr, mut next, mut end) = match id {
                0 => (addr, tag_spr, tadr, true),                              // refe
                1 => (tadr + 16, tadr & 0x8000_0000 != 0, tadr + 16 + qwc * 16, false), // cnt
                2 => (tadr + 16, tadr & 0x8000_0000 != 0, addr, false),         // next
                3 | 4 => (addr, tag_spr, tadr + 16, false),                     // ref/refs
                5 => {
                    // call
                    let ret = tadr + 16 + qwc * 16;
                    if asp < 2 {
                        self.ch[c].asr[asp as usize] = ret;
                        asp += 1;
                    }
                    (tadr + 16, tadr & 0x8000_0000 != 0, addr, false)
                }
                6 => {
                    // ret
                    if asp > 0 {
                        asp -= 1;
                        let r = self.ch[c].asr[asp as usize];
                        (tadr + 16, tadr & 0x8000_0000 != 0, r, false)
                    } else {
                        (tadr + 16, tadr & 0x8000_0000 != 0, tadr, true)
                    }
                }
                _ => (tadr + 16, tadr & 0x8000_0000 != 0, tadr, true), // end
            };
            let data: Vec<Qword> = (0..qwc)
                .map(|i| mem_q(ram, spr, data_addr + i * 16, data_spr))
                .collect();
            self.dma_sink(c, &data, ram, spr);
            self.ch[c].madr = data_addr + qwc * 16;
            if irq && tie {
                end = true;
            }
            if id == 0 {
                next = tadr + 16;
            }
            self.ch[c].tadr = next;
            if end {
                break;
            }
        }
        self.ch[c].chcr = (self.ch[c].chcr & !0x30) | (asp << 4);
        self.ch[c].qwc = 0;
    }
}
