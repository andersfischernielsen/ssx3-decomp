//! Host<->local and local->local transfers (BITBLTBUF/TRXPOS/TRXREG/TRXDIR).

use super::raster::{Mem, Pages, SurfRow};
use super::region::Region;
use super::swizzle::{bits_per_pixel, Surface};
use super::Gs;
use crate::Qword;

#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Xfer {
    /// Host->local transfer in progress.
    pub active: bool,
    pub x: u32,
    pub y: u32,
    /// Partial pixel bytes carried between writes (24-bit formats).
    pub carry: [u8; 4],
    pub ncarry: usize,
}

struct Params {
    sbp: u32,
    sbw: u32,
    spsm: u32,
    dbp: u32,
    dbw: u32,
    dpsm: u32,
    ssax: u32,
    ssay: u32,
    dsax: u32,
    dsay: u32,
    rrw: u32,
    rrh: u32,
}

impl Gs {
    fn xfer_params(&self) -> Params {
        let b = self.regs.bitbltbuf;
        let p = self.regs.trxpos;
        let r = self.regs.trxreg;
        Params {
            sbp: (b & 0x3FFF) as u32,
            sbw: ((b >> 16) & 0x3F) as u32,
            spsm: ((b >> 24) & 0x3F) as u32,
            dbp: ((b >> 32) & 0x3FFF) as u32,
            dbw: ((b >> 48) & 0x3F) as u32,
            dpsm: ((b >> 56) & 0x3F) as u32,
            ssax: (p & 0x7FF) as u32,
            ssay: ((p >> 16) & 0x7FF) as u32,
            dsax: ((p >> 32) & 0x7FF) as u32,
            dsay: ((p >> 48) & 0x7FF) as u32,
            rrw: (r & 0xFFF) as u32,
            rrh: ((r >> 32) & 0xFFF) as u32,
        }
    }

    pub(super) fn start_transfer(&mut self) {
        self.end_upload();
        self.xfer = Xfer::default();
        self.xfer_region = None;
        let p = self.xfer_params();
        if std::env::var_os("SSX3_TRACE_XFER").is_some() {
            let b = self.regs.bitbltbuf;
            eprintln!(
                "[xfer] dir {} sbp {:#x} sbw {} spsm {:#x} -> dbp {:#x} dbw {} dpsm {:#x} pos {:#x} size {:#x}",
                self.regs.trxdir & 3, b & 0x3fff, (b >> 16) & 0x3f, (b >> 24) & 0x3f,
                (b >> 32) & 0x3fff, (b >> 48) & 0x3f, (b >> 56) & 0x3f, self.regs.trxpos, self.regs.trxreg
            );
        }
        if self.regs.trxdir & 3 != 0 {
            self.flush_wait();
        }
        match self.regs.trxdir & 3 {
            0 => self.xfer.active = p.rrw != 0 && p.rrh != 0,
            1 => self.local_to_host(&p),
            2 => self.local_to_local(&p),
            _ => {}
        }
    }

    /// Feed host->local IMAGE / HWREG bytes.
    pub(super) fn transfer_bytes(&mut self, data: &[u8]) {
        if !self.xfer.active || self.regs.trxdir & 3 != 0 {
            return;
        }
        let p = self.xfer_params();
        let surf = Surface::new(p.dpsm, p.dbp, p.dbw);
        let first = self.xfer_region.is_none();
        let r = self.xfer_region.take().unwrap_or_else(|| {
            let mut r = Region::default();
            if p.dsax + p.rrw > 2048 || p.dsay + p.rrh > 2048 {
                r.add_pages(&Pages::ALL);
            } else {
                r.add_rect(&surf, p.dsax, p.dsay, p.dsax + p.rrw - 1, p.dsay + p.rrh - 1);
            }
            Box::new(r)
        });
        // Queued primitives must not see (or be overwritten by) this data.
        // Formats that store whole words cover whole pages of the mirror.
        let whole = matches!(surf.kind, super::swizzle::Kind::W32 | super::swizzle::Kind::H16 | super::swizzle::Kind::B8 | super::swizzle::Kind::N4);
        // GPU backend: partial-word formats (CT24, T8H, T4HL/HH) into pages
        // drawn on the GPU are written on the GPU instead of reading back.
        let direct = !whole && self.gpu.as_mut().is_some_and(|g| g.direct_upload(&r, &self.vram));
        if !direct {
            self.before_vram_write_full(&r, whole);
        }
        let start = self.xfer.y * p.rrw + self.xfer.x;
        let mut vals: Vec<u32> = Vec::new();
        if first {
            // Decoded textures in these pages are stale until the upload ends.
            self.texcache.live = r.pages;
            self.texcache.written(&r.pages);
        }
        self.xfer_region = Some(r);
        let bpp = bits_per_pixel(p.dpsm);
        let mut x = self.xfer.x;
        let mut y = self.xfer.y;
        let mut active = true;
        let m = Mem::new(&mut self.vram);
        let mut row = SurfRow::new(&surf, (p.dsay + y) & 0x7FF);
        let mut put = |v: u32| {
            if !active {
                return;
            }
            if direct {
                vals.push(v);
            } else {
                row.write(m, (p.dsax + x) & 0x7FF, v);
            }
            x += 1;
            if x >= p.rrw {
                x = 0;
                y += 1;
                if y >= p.rrh {
                    active = false;
                } else {
                    row = SurfRow::new(&surf, (p.dsay + y) & 0x7FF);
                }
            }
        };
        match bpp {
            32 => {
                for c in data.chunks_exact(4) {
                    put(u32::from_le_bytes([c[0], c[1], c[2], c[3]]));
                }
            }
            24 => {
                let xf = &mut self.xfer;
                for &b in data {
                    xf.carry[xf.ncarry] = b;
                    xf.ncarry += 1;
                    if xf.ncarry == 3 {
                        xf.ncarry = 0;
                        put(u32::from_le_bytes([
                            xf.carry[0],
                            xf.carry[1],
                            xf.carry[2],
                            0,
                        ]));
                    }
                }
            }
            16 => {
                for c in data.chunks_exact(2) {
                    put(u16::from_le_bytes([c[0], c[1]]) as u32);
                }
            }
            8 => {
                for &b in data {
                    put(b as u32);
                }
            }
            _ => {
                for &b in data {
                    put((b & 0xF) as u32);
                    put((b >> 4) as u32);
                }
            }
        }
        self.xfer.x = x;
        self.xfer.y = y;
        self.xfer.active = active;
        if direct && !vals.is_empty() {
            let g = self.gpu.as_mut().expect("gpu");
            g.pixel_write(&surf, [p.dsax, p.dsay, p.rrw, start], &vals);
        }
        if !active {
            self.end_upload();
        }
    }

    /// A host->local transfer ended (or was abandoned): its pages are final.
    pub(super) fn end_upload(&mut self) {
        if !self.texcache.live.is_empty() {
            let live = self.texcache.live;
            self.texcache.written(&live);
            self.texcache.live = Pages::default();
        }
    }

    /// Before reading the `w` x `h` rectangle at (x, y) of `surf` (which may
    /// wrap at 2048): make it current in `vram` (GPU backend).
    fn before_rect_read(&mut self, surf: &Surface, x: u32, y: u32, w: u32, h: u32) {
        if w == 0 || h == 0 {
            return;
        }
        if x + w > 2048 || y + h > 2048 {
            self.before_vram_read(surf, 0, 0, 2047, 2047);
        } else {
            self.before_vram_read(surf, x, y, x + w - 1, y + h - 1);
        }
    }

    /// GPU backend: serve a local->host transfer of GPU-drawn data from the
    /// previous frame (`GpuGs::latent_read`). False when it must be exact.
    fn latent_host_read(&mut self, surf: &Surface, x: u32, y: u32, w: u32, h: u32) -> bool {
        if !self.gpu.as_ref().is_some_and(|g| g.latent_readback) {
            return false;
        }
        if w == 0 || h == 0 {
            return true;
        }
        let mut r = Region::default();
        if x + w > 2048 || y + h > 2048 {
            r.add_rect(surf, 0, 0, 2047, 2047);
        } else {
            r.add_rect(surf, x, y, x + w - 1, y + h - 1);
        }
        let g = self.gpu.as_mut().expect("gpu");
        if !g.gpu_dirty.hits(&r.pages) && !g.batch_writes(&r) {
            return false;
        }
        g.latent_read(&r.pages, &mut self.vram);
        true
    }

    fn local_to_host(&mut self, p: &Params) {
        let surf = Surface::new(p.spsm, p.sbp, p.sbw);
        if !self.latent_host_read(&surf, p.ssax, p.ssay, p.rrw, p.rrh) {
            self.before_rect_read(&surf, p.ssax, p.ssay, p.rrw, p.rrh);
        }
        let bpp = bits_per_pixel(p.spsm);
        let total_bits = p.rrw as u64 * p.rrh as u64 * bpp as u64;
        let mut bytes: Vec<u8> = Vec::with_capacity(total_bits.div_ceil(8) as usize + 16);
        let mut nib: Option<u8> = None;
        for y in 0..p.rrh {
            for x in 0..p.rrw {
                let v = surf.read(&self.vram, (p.ssax + x) & 0x7FF, (p.ssay + y) & 0x7FF);
                match bpp {
                    32 => bytes.extend_from_slice(&v.to_le_bytes()),
                    24 => bytes.extend_from_slice(&v.to_le_bytes()[..3]),
                    16 => bytes.extend_from_slice(&(v as u16).to_le_bytes()),
                    8 => bytes.push(v as u8),
                    _ => match nib.take() {
                        None => nib = Some(v as u8 & 0xF),
                        Some(lo) => bytes.push(lo | ((v as u8 & 0xF) << 4)),
                    },
                }
            }
        }
        if let Some(lo) = nib {
            bytes.push(lo);
        }
        bytes.resize(bytes.len().next_multiple_of(16), 0);
        self.readback.clear();
        for c in bytes.chunks_exact(16) {
            let mut b = [0u8; 16];
            b.copy_from_slice(c);
            self.readback.push_back(Qword::from_le_bytes(b));
        }
    }

    fn local_to_local(&mut self, p: &Params) {
        let src = Surface::new(p.spsm, p.sbp, p.sbw);
        let dst = Surface::new(p.dpsm, p.dbp, p.dbw);
        self.before_rect_read(&src, p.ssax, p.ssay, p.rrw, p.rrh);
        // Read everything first so overlapping copies behave deterministically.
        let mut tmp = Vec::with_capacity((p.rrw * p.rrh) as usize);
        for y in 0..p.rrh {
            for x in 0..p.rrw {
                tmp.push(src.read(&self.vram, (p.ssax + x) & 0x7FF, (p.ssay + y) & 0x7FF));
            }
        }
        let pages = if p.dsax + p.rrw > 2048 || p.dsay + p.rrh > 2048 {
            Pages::ALL
        } else if p.rrw == 0 || p.rrh == 0 {
            Pages::default()
        } else {
            Pages::rect(&dst, p.dsax, p.dsay, p.dsax + p.rrw - 1, p.dsay + p.rrh - 1)
        };
        self.texcache.written(&pages);
        if self.gpu.is_some() {
            let mut r = Region::default();
            r.add_pages(&pages);
            self.before_vram_write(&r);
        }
        let mut i = 0;
        for y in 0..p.rrh {
            for x in 0..p.rrw {
                dst.write(
                    &mut self.vram,
                    (p.dsax + x) & 0x7FF,
                    (p.dsay + y) & 0x7FF,
                    tmp[i],
                );
                i += 1;
            }
        }
    }
}
