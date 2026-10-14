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

pub mod batch;
pub mod capture;
mod display;
pub mod gpu;
mod gif;
pub mod mtgs;
mod pool;
mod raster;
mod region;
mod texcache;
pub mod swizzle;
mod transfer;

#[cfg(test)]
mod diff_tests;
#[cfg(test)]
mod mtgs_tests;
#[cfg(test)]
mod ref_raster;
#[cfg(test)]
mod tests;

use crate::Qword;
use std::collections::VecDeque;
use std::sync::Arc;

use swizzle::Surface;
pub use mtgs::{Mtgs, MtgsStats};

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
    pub(crate) draw: Option<Arc<raster::Draw>>,
    /// Primitives set up but not yet drawn (see `batch`).
    pub(crate) batch: batch::Batch,
    /// Keep batches across calls; `vram` is then only current after `sync`.
    deferred: bool,
    /// Blocks the active host->local transfer writes (computed on demand).
    pub(crate) xfer_region: Option<Box<region::Region>>,
    /// Scratch for decoded host->local pixel values.
    pub(crate) xfer_vals: Vec<u32>,
    /// Decoded textures (see `texcache`).
    pub(crate) texcache: texcache::TexCache,
    /// Count of primitives drawn (statistics).
    pub prims_drawn: u64,
    /// Trace recorder while a capture is running (see `capture`).
    pub(crate) capture: Option<Box<capture::Recorder>>,
    /// GPU rasterization backend; `None` for the CPU rasterizer.
    pub(crate) gpu: Option<Box<gpu::GpuGs>>,
    /// Debug: when `Some`, every draw setup (with the decoded texture at the
    /// time of its first primitive) is appended here (`gs_replay --draws`).
    pub draw_log: Option<Vec<DrawLog>>,
    /// PATH3 (GIF DMA channel 2) data the GIF has not accepted yet because
    /// PATH3 is masked (see `path3_dma`).
    pub(crate) p3_queue: VecDeque<Qword>,
    /// MSKPATH3 from VIF1 (GIF_STAT.M3P) and GIF_MODE.M3R.
    pub(crate) p3_mask_vif: bool,
    pub(crate) p3_mask_reg: bool,
    /// PATH3 is inside a GIF packet (a mask only stops it at the end of one).
    pub(crate) p3_mid: bool,
}

/// One draw setup recorded by `Gs::draw_log`.
#[derive(Clone, Debug, Default)]
pub struct DrawLog {
    pub prim: u64,
    pub tex0: u64,
    pub tex1: u64,
    pub clamp: u64,
    pub frame: u64,
    pub alpha: u64,
    pub test: u64,
    /// Primitives drawn with this setup, and their screen bounding box.
    pub prims: u32,
    pub bbox: [i32; 4],
    /// The whole texture (2^TW x 2^TH, capped at 1024) decoded to RGBA8.
    pub image: Option<(u32, u32, Vec<u32>)>,
}

/// Which rasterizer draws primitives.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Backend {
    /// The software rasterizer (the reference).
    Cpu,
    /// The wgpu compute rasterizer (`gs::gpu`).
    Gpu,
}

impl Drop for Gs {
    fn drop(&mut self) {
        // Background rasterization writes into `vram`.
        self.wait_inflight();
    }
}

impl Default for Gs {
    fn default() -> Self {
        Self::new()
    }
}

impl Gs {
    /// A GS with the backend chosen by `SSX3_GS` (`gpu` or `cpu`, default
    /// `cpu`). Falls back to the CPU when no GPU is usable.
    pub fn new() -> Self {
        let mut gs = Self::new_cpu();
        if std::env::var("SSX3_GS").is_ok_and(|v| v.eq_ignore_ascii_case("gpu")) {
            gs.set_backend(Backend::Gpu);
        }
        gs
    }

    /// A GS drawing with the CPU rasterizer, whatever `SSX3_GS` says.
    pub fn new_cpu() -> Self {
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
            batch: batch::Batch::default(),
            deferred: false,
            xfer_region: None,
            xfer_vals: Vec::new(),
            texcache: texcache::TexCache::default(),
            prims_drawn: 0,
            capture: None,
            gpu: None,
            draw_log: None,
            p3_queue: VecDeque::new(),
            p3_mask_vif: false,
            p3_mask_reg: false,
            p3_mid: false,
        }
    }

    /// Switch rasterizer. Returns false if the GPU backend is unavailable
    /// (the CPU one stays). VRAM carries over in both directions.
    pub fn set_backend(&mut self, b: Backend) -> bool {
        match (b, self.gpu.is_some()) {
            (Backend::Gpu, false) => {
                self.flush_wait();
                match gpu::GpuGs::new() {
                    Some(g) => {
                        // `GpuGs::new` marks every page for upload.
                        self.gpu = Some(Box::new(g));
                        true
                    }
                    None => false,
                }
            }
            (Backend::Cpu, true) => {
                self.sync_vram();
                self.gpu = None;
                self.texcache.written(&raster::Pages::ALL);
                true
            }
            _ => true,
        }
    }

    pub fn backend(&self) -> Backend {
        if self.gpu.is_some() {
            Backend::Gpu
        } else {
            Backend::Cpu
        }
    }

    /// GPU backend statistics (`None` on the CPU backend).
    pub fn gpu_stats(&self) -> Option<gpu::GpuStats> {
        self.gpu.as_ref().map(|g| g.stats)
    }

    /// The device the GPU backend draws with (for presenting its frames).
    pub fn gpu_context(&self) -> Option<&'static gpu::GpuContext> {
        self.gpu.as_ref().map(|g| g.device())
    }

    /// GPU backend: make `vram` current (reads back every page drawn on the
    /// GPU since the last readback). A no-op on the CPU backend, where
    /// `sync()` already does this.
    pub fn sync_vram(&mut self) {
        self.flush_wait();
        if let Some(g) = self.gpu.as_mut() {
            let dirty = g.gpu_dirty;
            g.readback(&dirty, &mut self.vram, "sync_vram");
        }
    }

    /// GPU backend: whether local->host transfers of GPU-drawn data may be
    /// served from the previous frame's copy instead of waiting for the GPU
    /// (default on; `SSX3_GS_SYNC_READBACK=1` turns it off).
    pub fn set_latent_readback(&mut self, on: bool) {
        if let Some(g) = self.gpu.as_mut() {
            g.latent_readback = on;
        }
    }

    /// GPU backend: wait until everything submitted has been drawn.
    pub fn gpu_wait_idle(&mut self) {
        if let Some(g) = self.gpu.as_mut() {
            g.submit(&self.vram);
            g.wait_idle();
        }
    }

    /// GPU backend: evaluate the display circuits on the GPU and return the
    /// image's buffer (no VRAM readback). `None` on the CPU backend or when
    /// nothing is displayed.
    pub fn display_gpu(&mut self) -> Option<gpu::GpuFrame> {
        self.flush_wait();
        let setup = display::gpu_setup(self).ok()?;
        let g = self.gpu.as_mut()?;
        Some(g.display_gpu(&setup, &self.vram))
    }

    // ---------------------------------------------------------------- privileged

    pub fn write_priv(&mut self, offset: u32, value: u64) {
        if let Some(c) = &mut self.capture {
            c.privileged(offset, value);
        }
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
        self.flush_wait();
        self.end_upload();
        self.pr.csr = CSR_FIXED;
        self.paths = [gif::GifPath::default(); 4];
        self.vcount = 0;
        self.xfer = transfer::Xfer::default();
        self.xfer_region = None;
        self.readback.clear();
        self.draw = None;
    }

    pub fn irq_pending(&self) -> bool {
        let events = self.pr.csr & 0x1F;
        let masked = (self.pr.imr >> 8) & 0x1F;
        events & !masked != 0
    }

    pub fn vblank(&mut self, field: u32) {
        if let Some(c) = &mut self.capture {
            c.vblank(field);
        }
        self.flush_wait();
        if let Some(g) = self.gpu.as_mut() {
            g.submit(&self.vram);
        }
        self.pr.csr |= CSR_VSINT;
        if field & 1 != 0 {
            self.pr.csr |= CSR_FIELD;
        } else {
            self.pr.csr &= !CSR_FIELD;
        }
    }

    /// Raise a CSR event (also used for HSYNC/EDW by the machine if desired).
    pub fn raise_event(&mut self, bit: u64) {
        if let Some(c) = &mut self.capture {
            c.raise(bit);
        }
        self.pr.csr |= bit & 0x1F;
    }

    // ---------------------------------------------------------------- GIF

    pub fn gif_write(&mut self, path: usize, data: &[Qword]) {
        let path = path.min(3);
        if let Some(c) = &mut self.capture {
            c.gif(path, data);
        }
        if crate::trace_path3() {
            eprintln!("[p3] gif path {path} {} qw", data.len());
        }
        let mut i = 0;
        while i < data.len() {
            let p = &mut self.paths[path];
            if p.active && p.flg >= 2 {
                // IMAGE data: hand the whole run to the transfer at once
                // (the same as qword by qword: nothing else happens between).
                let n = ((p.nloop - p.loop_) as usize).min(data.len() - i);
                p.loop_ += n as u32;
                if p.loop_ >= p.nloop {
                    p.active = false;
                }
                self.transfer_bytes(qword_bytes(&data[i..i + n]));
                i += n;
            } else {
                self.gif_qword(path, data[i]);
                i += 1;
            }
        }
        if !self.deferred {
            self.flush_wait();
        }
    }

    /// GIF DMA (channel 2) data for PATH3. The GIF takes it at once unless
    /// PATH3 is masked (VIF1 MSKPATH3 or GIF_MODE.M3R); a mask stops PATH3
    /// only at the end of a GIF packet (a tag with EOP and its data), and the
    /// rest waits in the channel until the mask is lifted (PCSX2
    /// `Gif_Unit::CanDoPath3`). SSX 3 relies on this to upload several
    /// textures to the same VRAM block within one frame: each upload is
    /// released by an MSKPATH3 0 placed in the VIF1 stream just before the
    /// draws that use it.
    pub fn path3_dma(&mut self, data: &[Qword]) {
        if !path3_mask_enabled() {
            self.gif_write(3, data);
            return;
        }
        self.p3_queue.extend(data.iter().copied());
        self.pump_path3();
    }

    /// MSKPATH3 from VIF1. Lifting the mask lets PATH3 start its next
    /// packet at once; anything after that packet only flows once the VIF
    /// moves on with PATH3 still unmasked (`path3_flow`). SSX 3 releases
    /// one texture packet per `MSKPATH3 0` / `MSKPATH3 1` pair: on hardware
    /// the packet starts in the gap between the two, and the new mask only
    /// takes effect at its end.
    pub fn set_path3_mask(&mut self, masked: bool) {
        self.p3_mask_vif = masked;
        if !masked {
            self.pump_path3_packets(1);
        }
    }

    /// The VIF1 stream advanced (or a transfer ended) with PATH3 unmasked:
    /// everything waiting flows.
    pub fn path3_flow(&mut self) {
        if !self.p3_queue.is_empty() {
            self.pump_path3();
        }
    }

    /// GIF_MODE.M3R.
    pub fn set_path3_mask_reg(&mut self, masked: bool) {
        self.p3_mask_reg = masked;
        self.pump_path3();
    }

    /// GIF_CTRL.RST: drop PATH3 data waiting behind a mask.
    pub fn path3_reset(&mut self) {
        self.p3_queue.clear();
        self.p3_mid = false;
        self.paths[3] = gif::GifPath::default();
    }

    /// Qwords of PATH3 DMA data waiting behind a mask.
    pub fn path3_pending(&self) -> usize {
        self.p3_queue.len()
    }

    /// GIF_STAT bits this GS knows: M3R (0), M3P (1), P3Q (6: PATH3 waiting),
    /// FQC (28:24).
    pub fn gif_stat(&self) -> u32 {
        let q = self.p3_queue.len().min(16) as u32;
        (self.p3_mask_reg as u32) | (self.p3_mask_vif as u32) << 1 | ((q > 0) as u32) << 6 | q << 24
    }

    fn pump_path3(&mut self) {
        self.pump_path3_packets(usize::MAX);
    }

    /// Feed up to `max` more packets (finishing an open one first), stopping
    /// at a packet end while PATH3 is masked.
    fn pump_path3_packets(&mut self, max: usize) {
        let mut packets = 0;
        while !self.p3_queue.is_empty() {
            if !self.p3_mid && (self.p3_mask_vif || self.p3_mask_reg || packets >= max) {
                return;
            }
            // Feed up to the end of the current packet.
            let (mut left, mut eop) = self.paths[3].qwords_left();
            let mut n = 0;
            let mut mid = self.p3_mid;
            for &q in self.p3_queue.iter() {
                n += 1;
                if left == 0 {
                    (left, eop) = gif::GifPath::tag_qwords(q);
                } else {
                    left -= 1;
                }
                mid = left != 0 || !eop;
                if !mid {
                    break;
                }
            }
            let chunk: Vec<Qword> = self.p3_queue.drain(..n).collect();
            if !self.p3_mid {
                packets += 1;
            }
            self.gif_write(3, &chunk);
            self.p3_mid = mid;
        }
    }

    /// True while a GIF path is in the middle of a packet.
    pub fn gif_path_busy(&self, path: usize) -> bool {
        self.paths[path.min(3)].active
    }

    /// Pop up to `max` quadwords of a pending local->host transfer.
    pub fn read_local(&mut self, max: usize) -> Vec<Qword> {
        if let Some(c) = &mut self.capture {
            c.read_local(max);
        }
        let n = max.min(self.readback.len());
        self.readback.drain(..n).collect()
    }

    /// Number of quadwords waiting in the local->host FIFO.
    pub fn local_pending(&self) -> usize {
        self.readback.len()
    }

    // ---------------------------------------------------------------- registers

    pub fn write_reg(&mut self, addr: u8, value: u64) {
        if let Some(c) = &mut self.capture {
            c.reg(addr, value);
        }
        self.reg(addr, value);
        if !self.deferred {
            self.flush_wait();
        }
    }

    /// Wait for all queued drawing to land in `vram`.
    /// On the GPU backend this submits the queued work without waiting, and
    /// `vram` is only current after `sync_vram()`.
    pub fn sync(&mut self) {
        self.flush_wait();
        if let Some(g) = self.gpu.as_mut() {
            g.submit(&self.vram);
        }
    }

    /// Tell the GS that `vram` was modified directly (not through GIF or
    /// register writes), so cached decoded textures are dropped.
    /// On the GPU backend `vram` then replaces the GPU's copy entirely, so
    /// call `sync_vram()` before modifying it.
    pub fn vram_written(&mut self) {
        self.flush_wait();
        self.texcache.written(&raster::Pages::ALL);
        if let Some(g) = self.gpu.as_mut() {
            g.cpu_dirty = raster::Pages::ALL;
            g.gpu_dirty = raster::Pages::default();
        }
    }

    /// Deferred mode lets primitive batches span `gif_write` calls, which
    /// gives the rasterizer threads much more work per batch. `vram` (and
    /// `display`) are then only current after `sync()` or `vblank()`.
    /// Off by default: every call leaves `vram` complete.
    pub fn set_deferred(&mut self, on: bool) {
        self.deferred = on;
        if !on {
            self.flush_wait();
        }
    }

    /// Register write from inside the GS (GIF), not recorded by captures.
    pub(crate) fn reg(&mut self, addr: u8, value: u64) {
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
            0x08 | 0x09 => {
                if clamp_trace() && (value & 0xC) == 0xC {
                    eprintln!("[gs] CLAMP_{} = {value:#018x} (region-repeat v)", addr - 7);
                }
                r.ctx[(addr - 0x08) as usize].clamp = value
            }
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
        // The CLUT is read now: queued drawing into it must land first.
        if csm2 {
            self.before_vram_read(&surf, cou, cov, cou + entries - 1, cov);
        } else {
            self.before_vram_read(&surf, 0, 0, 15, 15);
        }
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

    /// On the GPU backend the image is produced on the GPU and read back.
    pub fn display(&mut self, out: &mut Vec<u32>) -> (u32, u32) {
        if self.gpu.is_some() {
            self.flush_wait();
            match display::gpu_setup(self) {
                Ok(setup) => {
                    let g = self.gpu.as_mut().expect("gpu");
                    g.display_read(&setup, &self.vram, out);
                    return (setup.w, setup.h);
                }
                Err((w, h)) => {
                    out.clear();
                    out.resize((w * h) as usize, 0xFF00_0000);
                    return (w, h);
                }
            }
        }
        display::render(self, out)
    }
}

/// The bytes of `q` in memory order (little-endian qwords).
#[cfg(target_endian = "little")]
fn qword_bytes(q: &[Qword]) -> &[u8] {
    // SAFETY: u128 has no padding; on little-endian targets its memory
    // layout is `to_le_bytes`.
    unsafe { std::slice::from_raw_parts(q.as_ptr() as *const u8, q.len() * 16) }
}

fn clamp_trace() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var("SSX3_TRACE_CLAMP").is_ok())
}

/// `SSX3_PATH3_MASK=0` lets PATH3 DMA through immediately, ignoring
/// MSKPATH3/M3R (the old behaviour).
fn path3_mask_enabled() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var("SSX3_PATH3_MASK").map_or(true, |v| v != "0"))
}
