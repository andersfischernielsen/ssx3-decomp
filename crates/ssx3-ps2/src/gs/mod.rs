//! Graphics Synthesizer: GIF paths, registers, swizzled VRAM, rasterizer.
//!
//! INTERFACE CONTRACT (other modules code against exactly this API):
//! - `Gs::new()`
//! - `write_priv(offset, value)` / `read_priv(offset)`: privileged registers;
//!   `offset` is relative to `0x1200_0000` (CSR is `0x1000`, IMR `0x1010`, ...).
//! - `gif_write(path, data)`: feed whole quadwords to GIF PATH1/2/3 (index 1..=3).
//!   Tag state persists per path across calls (packets may be split).
//! - `write_reg(addr, value)`: a single GS register write (as A+D does).
//! - `irq_pending()`: true while CSR has an unmasked SIGNAL/FINISH/HSYNC/VSYNC/EDW event.
//! - `vblank(field)`: called by the machine at VBlank start; sets CSR.VSINT/FIELD.
//! - `display(&mut out) -> (w, h)`: RGBA8 image of what PMODE/DISPFB/DISPLAY show.
//! - `read_local(...)`: local->host transfer (TRXDIR=1) output, drained by GIF FIFO reads.
//!
//! Implementation: a CPU software GS. See `docs/investigation/gs.md`.
//! Swizzle tables and several semantics are ported from ps2-web-recomp
//! (MIT, https://github.com/snuri00/ps2-web-recomp) and PCSX2 GSdx behaviour.

mod display;
mod gif;
mod raster;
pub mod swizzle;
mod transfer;

#[cfg(test)]
mod tests;

use crate::Qword;
use std::collections::VecDeque;

use swizzle::Surface;

/// CSR event bits.
pub const CSR_SIGNAL: u64 = 1 << 0;
pub const CSR_FINISH: u64 = 1 << 1;
pub const CSR_HSINT: u64 = 1 << 2;
pub const CSR_VSINT: u64 = 1 << 3;
pub const CSR_EDWINT: u64 = 1 << 4;
const CSR_RESET: u64 = 1 << 9;
const CSR_FIELD: u64 = 1 << 13;
/// FIFO empty (bits 15:14 = 01), REV 0x1B (23:16), ID 0x55 (31:24).
const CSR_FIXED: u64 = 0x551B_4000;

/// Per-context drawing registers.
#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Ctx {
    pub tex0: u64,
    pub clamp: u64,
    pub tex1: u64,
    pub xyoffset: u64,
    pub miptbp1: u64,
    pub miptbp2: u64,
    pub scissor: u64,
    pub alpha: u64,
    pub test: u64,
    pub fba: u64,
    pub frame: u64,
    pub zbuf: u64,
}

/// One queued vertex.
#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Vertex {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub f: u8,
    pub rgba: [u8; 4],
    pub s: f32,
    pub t: f32,
    pub q: f32,
    pub u: u32,
    pub v: u32,
}

/// General-purpose GS registers.
#[derive(Clone, Debug)]
pub(crate) struct Regs {
    pub prim: u64,
    pub rgbaq: u64,
    pub st: u64,
    pub uv: u64,
    pub fog: u64,
    pub ctx: [Ctx; 2],
    pub prmodecont: u64,
    pub prmode: u64,
    pub texclut: u64,
    pub scanmsk: u64,
    pub texa: u64,
    pub fogcol: u64,
    pub dimx: u64,
    pub dthe: u64,
    pub colclamp: u64,
    pub pabe: u64,
    pub bitbltbuf: u64,
    pub trxpos: u64,
    pub trxreg: u64,
    pub trxdir: u64,
}

impl Default for Regs {
    fn default() -> Self {
        Regs {
            prim: 0,
            rgbaq: (1.0f32.to_bits() as u64) << 32,
            st: 0,
            uv: 0,
            fog: 0,
            ctx: [Ctx::default(); 2],
            prmodecont: 1,
            prmode: 0,
            texclut: 0,
            scanmsk: 0,
            texa: 0,
            fogcol: 0,
            dimx: 0,
            dthe: 0,
            colclamp: 0,
            pabe: 0,
            bitbltbuf: 0,
            trxpos: 0,
            trxreg: 0,
            trxdir: 3,
        }
    }
}

/// Privileged (memory-mapped) registers.
#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Priv {
    pub pmode: u64,
    pub smode1: u64,
    pub smode2: u64,
    pub srfsh: u64,
    pub synch1: u64,
    pub synch2: u64,
    pub syncv: u64,
    pub dispfb: [u64; 2],
    pub display: [u64; 2],
    pub extbuf: u64,
    pub extdata: u64,
    pub extwrite: u64,
    pub bgcolor: u64,
    pub csr: u64,
    pub imr: u64,
    pub busdir: u64,
    pub siglblid: u64,
}

pub struct Gs {
    pub vram: Vec<u32>,
    pub(crate) pr: Priv,
    pub(crate) regs: Regs,
    /// Internal Q latched by PACKED ST writes.
    pub(crate) internal_q: f32,
    pub(crate) vq: [Vertex; 3],
    pub(crate) vcount: usize,
    /// CLUT buffer: 512 16-bit entries (CT32 entries split lo/hi halves).
    pub(crate) clut: [u16; 512],
    pub(crate) cbp0: u32,
    pub(crate) cbp1: u32,
    pub(crate) clut_gen: u64,
    pub(crate) paths: [gif::GifPath; 4],
    pub(crate) xfer: transfer::Xfer,
    pub(crate) readback: VecDeque<Qword>,
    /// Cached per-draw setup; `None` when registers changed since last build.
    pub(crate) draw: Option<Box<raster::Draw>>,
    /// Count of primitives drawn (statistics).
    pub prims_drawn: u64,
}

impl Default for Gs {
    fn default() -> Self {
        Self::new()
    }
}

impl Gs {
    pub fn new() -> Self {
        Self {
            vram: vec![0; swizzle::VRAM_WORDS],
            pr: Priv {
                csr: CSR_FIXED,
                imr: 0x7F00,
                ..Priv::default()
            },
            regs: Regs::default(),
            internal_q: 1.0,
            vq: [Vertex::default(); 3],
            vcount: 0,
            clut: [0; 512],
            cbp0: u32::MAX,
            cbp1: u32::MAX,
            clut_gen: 0,
            paths: [gif::GifPath::default(); 4],
            xfer: transfer::Xfer::default(),
            readback: VecDeque::new(),
            draw: None,
            prims_drawn: 0,
        }
    }

    // ---------------------------------------------------------------- privileged

    pub fn write_priv(&mut self, offset: u32, value: u64) {
        let p = &mut self.pr;
        match offset & 0x1FF0 {
            0x0000 => p.pmode = value,
            0x0010 => p.smode1 = value,
            0x0020 => p.smode2 = value,
            0x0030 => p.srfsh = value,
            0x0040 => p.synch1 = value,
            0x0050 => p.synch2 = value,
            0x0060 => p.syncv = value,
            0x0070 => p.dispfb[0] = value,
            0x0080 => p.display[0] = value,
            0x0090 => p.dispfb[1] = value,
            0x00A0 => p.display[1] = value,
            0x00B0 => p.extbuf = value,
            0x00C0 => p.extdata = value,
            0x00D0 => p.extwrite = value,
            0x00E0 => p.bgcolor = value,
            0x1000 => {
                // Writing 1 to an event bit acknowledges it.
                p.csr &= !(value & 0x1F);
                if value & CSR_RESET != 0 {
                    self.reset();
                }
            }
            0x1010 => p.imr = value & 0x7F00,
            0x1040 => p.busdir = value,
            0x1080 => p.siglblid = value,
            _ => {}
        }
    }

    pub fn read_priv(&mut self, offset: u32) -> u64 {
        let p = &self.pr;
        match offset & 0x1FF0 {
            0x0000 => p.pmode,
            0x0010 => p.smode1,
            0x0020 => p.smode2,
            0x0030 => p.srfsh,
            0x0040 => p.synch1,
            0x0050 => p.synch2,
            0x0060 => p.syncv,
            0x0070 => p.dispfb[0],
            0x0080 => p.display[0],
            0x0090 => p.dispfb[1],
            0x00A0 => p.display[1],
            0x00B0 => p.extbuf,
            0x00C0 => p.extdata,
            0x00D0 => p.extwrite,
            0x00E0 => p.bgcolor,
            0x1000 => (p.csr & (0x1F | CSR_FIELD)) | CSR_FIXED,
            0x1010 => p.imr,
            0x1040 => p.busdir,
            0x1080 => p.siglblid,
            _ => 0,
        }
    }

    /// GS soft reset (CSR.RESET): drops GIF/vertex/transfer state, keeps VRAM.
    fn reset(&mut self) {
        self.pr.csr = CSR_FIXED;
        self.paths = [gif::GifPath::default(); 4];
        self.vcount = 0;
        self.xfer = transfer::Xfer::default();
        self.readback.clear();
        self.draw = None;
    }

    pub fn irq_pending(&self) -> bool {
        let events = self.pr.csr & 0x1F;
        let masked = (self.pr.imr >> 8) & 0x1F;
        events & !masked != 0
    }

    pub fn vblank(&mut self, field: u32) {
        self.pr.csr |= CSR_VSINT;
        if field & 1 != 0 {
            self.pr.csr |= CSR_FIELD;
        } else {
            self.pr.csr &= !CSR_FIELD;
        }
    }

    /// Raise a CSR event (also used for HSYNC/EDW by the machine if desired).
    pub fn raise_event(&mut self, bit: u64) {
        self.pr.csr |= bit & 0x1F;
    }

    // ---------------------------------------------------------------- GIF

    pub fn gif_write(&mut self, path: usize, data: &[Qword]) {
        let path = path.min(3);
        for &q in data {
            self.gif_qword(path, q);
        }
    }

    /// True while a GIF path is in the middle of a packet.
    pub fn gif_path_busy(&self, path: usize) -> bool {
        self.paths[path.min(3)].active
    }

    /// Pop up to `max` quadwords of a pending local->host transfer.
    pub fn read_local(&mut self, max: usize) -> Vec<Qword> {
        let n = max.min(self.readback.len());
        self.readback.drain(..n).collect()
    }

    /// Number of quadwords waiting in the local->host FIFO.
    pub fn local_pending(&self) -> usize {
        self.readback.len()
    }

    // ---------------------------------------------------------------- registers

    pub fn write_reg(&mut self, addr: u8, value: u64) {
        match addr {
            0x00 => {
                self.regs.prim = value & 0x7FF;
                self.vcount = 0;
                self.draw = None;
            }
            0x01 => self.regs.rgbaq = value,
            0x02 => self.regs.st = value,
            0x03 => self.regs.uv = value & 0x3FFF_3FFF,
            0x04 => self.vertex(
                value as u32 & 0xFFFF,
                (value >> 16) as u32 & 0xFFFF,
                (value >> 32) as u32 & 0xFF_FFFF,
                (value >> 56) as u8,
                true,
            ),
            0x05 => {
                let f = (self.regs.fog >> 56) as u8;
                self.vertex(
                    value as u32 & 0xFFFF,
                    (value >> 16) as u32 & 0xFFFF,
                    (value >> 32) as u32,
                    f,
                    true,
                )
            }
            0x0C => self.vertex(
                value as u32 & 0xFFFF,
                (value >> 16) as u32 & 0xFFFF,
                (value >> 32) as u32 & 0xFF_FFFF,
                (value >> 56) as u8,
                false,
            ),
            0x0D => {
                let f = (self.regs.fog >> 56) as u8;
                self.vertex(
                    value as u32 & 0xFFFF,
                    (value >> 16) as u32 & 0xFFFF,
                    (value >> 32) as u32,
                    f,
                    false,
                )
            }
            0x0A => self.regs.fog = value,
            _ => {
                self.draw = None;
                self.write_state_reg(addr, value);
            }
        }
    }

    fn write_state_reg(&mut self, addr: u8, value: u64) {
        let r = &mut self.regs;
        match addr {
            0x06 | 0x07 => {
                let c = (addr - 0x06) as usize;
                r.ctx[c].tex0 = value;
                self.load_clut(value);
            }
            0x08 | 0x09 => r.ctx[(addr - 0x08) as usize].clamp = value,
            0x14 | 0x15 => r.ctx[(addr - 0x14) as usize].tex1 = value,
            0x16 | 0x17 => {
                let c = (addr - 0x16) as usize;
                const MASK: u64 = 0xFFFF_FFE0_03F0_0000;
                r.ctx[c].tex0 = (r.ctx[c].tex0 & !MASK) | (value & MASK);
                let t = r.ctx[c].tex0;
                self.load_clut(t);
            }
            0x18 | 0x19 => r.ctx[(addr - 0x18) as usize].xyoffset = value,
            0x1A => r.prmodecont = value,
            0x1B => r.prmode = value,
            0x1C => r.texclut = value,
            0x22 => r.scanmsk = value,
            0x34 | 0x35 => r.ctx[(addr - 0x34) as usize].miptbp1 = value,
            0x36 | 0x37 => r.ctx[(addr - 0x36) as usize].miptbp2 = value,
            0x3B => r.texa = value,
            0x3D => r.fogcol = value,
            0x3F => {} // TEXFLUSH: nothing cached across VRAM writes.
            0x40 | 0x41 => r.ctx[(addr - 0x40) as usize].scissor = value,
            0x42 | 0x43 => r.ctx[(addr - 0x42) as usize].alpha = value,
            0x44 => r.dimx = value,
            0x45 => r.dthe = value,
            0x46 => r.colclamp = value,
            0x47 | 0x48 => r.ctx[(addr - 0x47) as usize].test = value,
            0x49 => r.pabe = value,
            0x4A | 0x4B => r.ctx[(addr - 0x4A) as usize].fba = value,
            0x4C | 0x4D => r.ctx[(addr - 0x4C) as usize].frame = value,
            0x4E | 0x4F => r.ctx[(addr - 0x4E) as usize].zbuf = value,
            0x50 => r.bitbltbuf = value,
            0x51 => r.trxpos = value,
            0x52 => r.trxreg = value,
            0x53 => {
                r.trxdir = value & 3;
                self.start_transfer();
            }
            0x54 => self.transfer_bytes(&value.to_le_bytes()),
            0x60 => {
                let mask = value >> 32;
                let id = value & 0xFFFF_FFFF;
                let p = &mut self.pr;
                let lo = (p.siglblid & 0xFFFF_FFFF & !mask) | (id & mask);
                p.siglblid = (p.siglblid & !0xFFFF_FFFF) | lo;
                p.csr |= CSR_SIGNAL;
            }
            0x61 => self.pr.csr |= CSR_FINISH,
            0x62 => {
                let mask = value >> 32;
                let id = value & 0xFFFF_FFFF;
                let p = &mut self.pr;
                let hi = ((p.siglblid >> 32) & !mask) | (id & mask);
                p.siglblid = (p.siglblid & 0xFFFF_FFFF) | (hi << 32);
            }
            _ => {}
        }
    }

    /// Append a vertex to the queue; draw when enough vertices are present.
    fn vertex(&mut self, x: u32, y: u32, z: u32, f: u8, kick: bool) {
        let r = &self.regs;
        let rgba = (r.rgbaq as u32).to_le_bytes();
        let v = Vertex {
            x,
            y,
            z,
            f,
            rgba,
            q: f32::from_bits((r.rgbaq >> 32) as u32),
            s: f32::from_bits(r.st as u32),
            t: f32::from_bits((r.st >> 32) as u32),
            u: r.uv as u32 & 0x3FFF,
            v: (r.uv >> 16) as u32 & 0x3FFF,
        };
        if self.vcount >= 3 {
            self.vcount = 2;
        }
        self.vq[self.vcount] = v;
        self.vcount += 1;
        let prim = (self.regs.prim & 7) as u32;
        let needed = match prim {
            0 => 1,
            1 | 2 | 6 => 2,
            3..=5 => 3,
            _ => {
                self.vcount = 0;
                return;
            }
        };
        if self.vcount < needed {
            return;
        }
        if kick {
            let verts = self.vq;
            self.draw_primitive(prim, &verts[..needed]);
        }
        match prim {
            2 => {
                self.vq[0] = self.vq[1];
                self.vcount = 1;
            }
            4 => {
                self.vq[0] = self.vq[1];
                self.vq[1] = self.vq[2];
                self.vcount = 2;
            }
            5 => {
                self.vq[1] = self.vq[2];
                self.vcount = 2;
            }
            _ => self.vcount = 0,
        }
    }

    fn draw_primitive(&mut self, prim: u32, verts: &[Vertex]) {
        if self.draw.is_none() {
            self.draw = Some(Box::new(raster::Draw::build(&self.regs, &self.clut)));
        }
        let draw = self.draw.as_deref().expect("draw setup");
        raster::draw(&mut self.vram, draw, prim, verts);
        self.prims_drawn += 1;
    }

    // ---------------------------------------------------------------- CLUT

    fn load_clut(&mut self, tex0: u64) {
        let psm = ((tex0 >> 20) & 0x3F) as u32;
        let eight = matches!(psm, swizzle::PSMT8 | swizzle::PSMT8H);
        let four = matches!(psm, swizzle::PSMT4 | swizzle::PSMT4HL | swizzle::PSMT4HH);
        if !eight && !four {
            return;
        }
        let cbp = ((tex0 >> 37) & 0x3FFF) as u32;
        match (tex0 >> 61) & 7 {
            1 => {}
            2 => self.cbp0 = cbp,
            3 => self.cbp1 = cbp,
            4 => {
                if self.cbp0 == cbp {
                    return;
                }
                self.cbp0 = cbp;
            }
            5 => {
                if self.cbp1 == cbp {
                    return;
                }
                self.cbp1 = cbp;
            }
            _ => return,
        }
        let cpsm = ((tex0 >> 51) & 0xF) as u32;
        let csm2 = (tex0 >> 55) & 1 != 0;
        let csa = ((tex0 >> 56) & 0x1F) as u32;
        let entries: u32 = if eight { 256 } else { 16 };
        let cpsm_full = match cpsm {
            0x2 => swizzle::PSMCT16,
            0xA => swizzle::PSMCT16S,
            _ => swizzle::PSMCT32,
        };
        let is32 = cpsm_full == swizzle::PSMCT32;
        let (surf, cou, cov) = if csm2 {
            let tc = self.regs.texclut;
            let cbw = (tc & 0x3F) as u32;
            let cou = ((tc >> 6) & 0x3F) as u32 * 16;
            let cov = ((tc >> 12) & 0x3FF) as u32;
            (Surface::new(cpsm_full, cbp, cbw), cou, cov)
        } else {
            (Surface::new(cpsm_full, cbp, 1), 0, 0)
        };
        for i in 0..entries {
            let (x, y) = if csm2 {
                (cou + i, cov)
            } else if eight {
                // CSM1 8-bit: 16x16 with index bits 3 and 4 swapped.
                let j = (i & !0x18) | ((i & 0x08) << 1) | ((i & 0x10) >> 1);
                (j & 15, j >> 4)
            } else {
                (i & 7, i >> 3)
            };
            let c = surf.read(&self.vram, x, y);
            if is32 {
                let slot = (((csa & 15) * 16 + i) & 255) as usize;
                self.clut[slot] = c as u16;
                self.clut[slot + 256] = (c >> 16) as u16;
            } else {
                let slot = ((csa * 16 + i) & 511) as usize;
                self.clut[slot] = c as u16;
            }
        }
        self.clut_gen += 1;
        self.draw = None;
    }

    // ---------------------------------------------------------------- display

    pub fn display(&self, out: &mut Vec<u32>) -> (u32, u32) {
        display::render(self, out)
    }
}
