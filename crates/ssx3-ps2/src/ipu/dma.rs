//! The IPU's two DMA channels as seen from the IPU: ch4 (toIPU) is *pulled*
//! by the IPU one quadword at a time as its input FIFO drains, and ch3
//! (fromIPU) accepts output quadwords while it has QWC left.
//!
//! This is what makes libmpeg's flow work: it arms ch4 with a source chain
//! over its whole bitstream ring and lets the IPU consume at its own pace,
//! and `sceIpuStopDMA`/`sceIpuRestartDMA` rewind the bitstream with
//! `D4_MADR - 16 * (IPU_BP.IFC + IPU_BP.FP)` — which is only right if the IPU
//! never holds more than its FIFO's worth of unconsumed data. Modelled on
//! ps2-web-recomp's `dmac::ipu_pull`/`ipu_push`.

use super::{Ipu, IpuDma};
use crate::hw::{mem_q, mem_q_write, Channel};
use crate::Qword;

/// Index of the fromIPU / toIPU channels in `Hw::ch`.
const FROM_IPU: usize = 3;
const TO_IPU: usize = 4;

const STR: u32 = 0x100;

/// [`IpuDma`] over the DMAC channel registers and RAM/scratchpad.
pub struct HwPort<'a> {
    ch3: &'a mut Channel,
    ch4: &'a mut Channel,
    /// ch4 has run its final tag (refe/end, or IRQ with TIE): once QWC hits
    /// zero the transfer is complete.
    tag_end: bool,
    ram: &'a mut [u8],
    spr: &'a mut [u8],
    /// Channels (as D_STAT bits) that completed during this access.
    done: u32,
}

impl HwPort<'_> {
    fn finish(&mut self, c: usize) {
        let chn = if c == FROM_IPU {
            &mut *self.ch3
        } else {
            &mut *self.ch4
        };
        chn.chcr &= !STR;
        self.done |= 1 << c;
    }
}

impl IpuDma for HwPort<'_> {
    fn pull(&mut self) -> Option<Qword> {
        if self.ch4.chcr & STR == 0 {
            return None;
        }
        let chain = (self.ch4.chcr >> 2) & 3 == 1;
        while self.ch4.qwc == 0 {
            if !chain || self.tag_end {
                self.finish(TO_IPU);
                return None;
            }
            // Source chain: fetch the next DMAtag.
            let tadr = self.ch4.tadr;
            let tag = mem_q(self.ram, self.spr, tadr, tadr & 0x8000_0000 != 0) as u64;
            self.ch4.chcr = (self.ch4.chcr & 0xFFFF) | ((tag as u32) & 0xFFFF_0000);
            let qwc = (tag & 0xFFFF) as u32;
            let id = ((tag >> 28) & 7) as u32;
            let addr = ((tag >> 32) as u32 & 0x7FFF_FFF0) | (((tag >> 63) as u32) << 31);
            let irq = (tag >> 31) & 1 != 0;
            let tie = self.ch4.chcr & 0x80 != 0;
            let mut data = tadr + 16;
            match id {
                0 => {
                    // refe
                    data = addr;
                    self.ch4.tadr += 16;
                    self.tag_end = true;
                }
                1 => self.ch4.tadr = data + qwc * 16, // cnt
                2 => self.ch4.tadr = addr,            // next
                3 | 4 => {
                    // ref / refs
                    data = addr;
                    self.ch4.tadr += 16;
                }
                _ => self.tag_end = true, // end (call/ret are not used with toIPU)
            }
            if tie && irq {
                self.tag_end = true;
            }
            self.ch4.madr = data;
            self.ch4.qwc = qwc;
            if qwc == 0 && self.tag_end {
                self.finish(TO_IPU);
                return None;
            }
        }
        let madr = self.ch4.madr;
        let q = mem_q(self.ram, self.spr, madr, madr & 0x8000_0000 != 0);
        self.ch4.madr += 16;
        self.ch4.qwc -= 1;
        if self.ch4.qwc == 0 && (!chain || self.tag_end) {
            self.finish(TO_IPU);
        }
        Some(q)
    }

    fn push(&mut self, q: Qword) -> bool {
        if self.ch3.chcr & STR == 0 || self.ch3.qwc == 0 {
            return false;
        }
        let madr = self.ch3.madr;
        mem_q_write(self.ram, self.spr, madr, madr & 0x8000_0000 != 0, q);
        self.ch3.madr += 16;
        self.ch3.qwc -= 1;
        if self.ch3.qwc == 0 {
            self.finish(FROM_IPU);
        }
        true
    }
}

/// Note that `chcr` was just written with STR set on channel 3 or 4. Returns
/// D_STAT bits of channels that are already complete (an empty fromIPU).
pub fn started(ipu: &mut Ipu, ch: &mut [Channel; 10], c: usize) -> u32 {
    if c == TO_IPU {
        let chn = &ch[TO_IPU];
        // A chain started with QWC != 0 first finishes the current tag's data;
        // it ends there if that tag (still in CHCR's upper half) was refe/end
        // or raised IRQ with TIE.
        let id = (chn.chcr >> 28) & 7;
        let irq_tie = chn.chcr & 0x8000_0000 != 0 && chn.chcr & 0x80 != 0;
        ipu.dma4_tag_end = chn.qwc != 0 && (id == 0 || id == 7 || irq_tie);
        0
    } else if ch[FROM_IPU].qwc == 0 {
        ch[FROM_IPU].chcr &= !STR;
        1 << FROM_IPU
    } else {
        0
    }
}

/// Run `f` with the IPU connected to its DMA channels. Returns `f`'s result
/// and the D_STAT bits of channels that completed.
pub fn run<T>(
    ipu: &mut Ipu,
    ch: &mut [Channel; 10],
    ram: &mut [u8],
    spr: &mut [u8],
    f: impl FnOnce(&mut Ipu, &mut dyn IpuDma) -> T,
) -> (T, u32) {
    let (lo, hi) = ch.split_at_mut(TO_IPU);
    let mut port = HwPort {
        ch3: &mut lo[FROM_IPU],
        ch4: &mut hi[0],
        tag_end: ipu.dma4_tag_end,
        ram,
        spr,
        done: 0,
    };
    let r = f(ipu, &mut port);
    ipu.dma4_tag_end = port.tag_end;
    (r, port.done)
}
