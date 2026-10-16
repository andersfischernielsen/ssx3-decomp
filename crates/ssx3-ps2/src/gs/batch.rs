//! Primitive batching and multithreaded rasterization.
//!
//! Kicked primitives are set up immediately (`raster::setup`) but drawn
//! later, in batches. A batch is drawn by splitting the screen into bands of
//! 8 rows; band `b` belongs to unit `b % UNITS`, and the units are spread
//! over the worker pool. Every unit walks all primitives of the batch in
//! submission order and draws only its own rows, so each pixel still sees
//! the primitives in exactly the original order.
//!
//! That is only equivalent to drawing serially when no two units can touch
//! the same VRAM word and nothing a primitive reads changes during the
//! batch. The batch tracks the 256-byte VRAM blocks its primitives write
//! (bounding boxes in the frame and Z buffers) and read (textures), and is
//! flushed before:
//! - a primitive whose texture reads blocks the batch writes, or whose
//!   buffers write blocks the batch reads;
//! - a primitive whose frame/Z buffer overlaps pages written through a
//!   different buffer mapping (base, width or format);
//! - a CLUT load from blocks the batch writes, host->local transfer data for
//!   blocks the batch touches, and every local->host / local->local transfer;
//! - `sync()`, `vblank()`, CSR reset, and (unless deferred mode is on) the
//!   end of every `gif_write` / `write_reg` call, so `vram` is always current
//!   for callers by default.
//!
//! A texture may overlap what the batch draws when each row only reads
//! texels of its own band through the same address mapping (full-screen
//! copy and colour-lookup passes do this); see `Batch::row_local`.
//!
//! Primitives that cannot be split by rows (texture overlapping their own
//! output otherwise, buffers in formats whose words span rows, or pixels
//! beyond the buffer width, which wrap into other rows) are drawn alone.

use std::sync::Arc;

use super::raster::{self, Pages, Decoded, Draw, Mem, Prim, Rows};
use super::region::Region;
use super::swizzle::Surface;
use super::{Gs, Vertex};

/// Number of row-band units a batch is split into.
const UNITS: i32 = 64;
/// Batches smaller than this (in estimated pixels) are drawn on one thread.
const PARALLEL_MIN_WORK: u64 = 8192;
/// Flush after this many primitives to bound memory.
const MAX_PRIMS: usize = 1 << 16;

#[derive(Default)]
pub(crate) struct Batch {
    draws: Vec<Arc<Draw>>,
    prims: Vec<Prim>,
    /// Per unit, the primitives with rows in it (indices, in order).
    bins: Vec<Vec<u32>>,
    write: WriteSet,
    read: Region,
    /// Row-local texture reads (see `row_local`), per texture mapping.
    local_reads: Vec<(Surface, Region)>,
    local_read: Region,
    /// Buffer mappings written by the batch, with their (scissor) pages.
    surfs: Vec<(Surface, raster::Pages)>,
    /// Index of the last draw whose whole texture was added to `read`.
    full_tex_draw: Option<usize>,
    work: u64,
    /// Scratch sets for the primitive being queued.
    prim_write: Region,
    prim_tex: Region,
    /// The previous batch, if it is still being drawn.
    inflight: Option<InFlight>,
    pub(crate) stats: Stats,
}

/// Batching statistics (see `Gs::batch_stats`).
#[derive(Default, Clone, Copy, Debug)]
pub struct Stats {
    /// Non-empty flushes, and how many of them ran on several threads.
    pub flushes: u64,
    pub parallel_flushes: u64,
    /// Primitives drawn through batches / alone (not row-splittable).
    pub batched_prims: u64,
    pub serial_prims: u64,
    /// Flushes forced by a primitive's dependencies, a CLUT load, or a transfer.
    pub conflict_flushes: u64,
    pub clut_flushes: u64,
    pub transfer_flushes: u64,
    /// Decoded-texture cache: lookups served, textures decoded, texels decoded.
    pub tex_hits: u64,
    pub tex_decodes: u64,
    pub tex_decoded_texels: u64,
}

impl Batch {
    pub(crate) fn is_empty(&self) -> bool {
        self.prims.is_empty()
    }

    /// Whether drawing with `d` after the current batch needs a flush first:
    /// its buffers overlap pages written through a different mapping.
    fn aliases(&self, d: &Draw) -> bool {
        if self.prims.is_empty() {
            return false;
        }
        let mut mine = vec![(raster::surf_key(&d.fb), d.fb_pages)];
        if d.z_used {
            mine.push((raster::surf_key(&d.zb), d.zb_pages));
        }
        mine.iter().any(|(k, p)| {
            self.surfs
                .iter()
                .any(|(s2, p2)| *k != raster::surf_key(s2) && p.hits(p2))
        })
    }

    fn register(&mut self, d: Arc<Draw>) {
        let mut add = |surf: Surface, pages: raster::Pages| {
            if pages.is_empty() {
                return;
            }
            let key = raster::surf_key(&surf);
            match self.surfs.iter_mut().find(|(s, _)| raster::surf_key(s) == key) {
                Some((_, p)) => p.union(&pages),
                None => self.surfs.push((surf, pages)),
            }
        };
        add(d.fb, d.fb_pages);
        if d.z_used {
            add(d.zb, d.zb_pages);
        }
        self.draws.push(d);
    }

    /// A primitive reading texture blocks that the batch (or the primitive
    /// itself) writes can still be drawn by rows if every overlapping buffer
    /// maps texel (x, y) to the word of pixel (x, y) and each row reads only
    /// from its own band.
    fn row_local(&self, d: &Draw, p: &Prim) -> bool {
        let Some(ts) = d.tex_surface() else {
            return false;
        };
        let tex = &self.prim_tex.pages;
        self.surfs
            .iter()
            .filter(|(_, pg)| pg.hits(tex))
            .all(|(s, _)| raster::same_words(ts, s))
            && raster::sprite_rows_local(d, p)
    }

    /// Writing `w` through `surf` conflicts with a row-local read.
    fn local_war(&self, surf: &Surface, w: &Region) -> bool {
        self.local_reads
            .iter()
            .any(|(s, r)| r.hits(w) && !raster::same_words(s, surf))
    }

    /// Any queued primitive reads or writes a block of `r`.
    fn touches(&mut self, r: &Region) -> bool {
        self.write.hits(r) || r.hits(&self.read) || r.hits(&self.local_read)
    }

    /// `prim_write` := the blocks of the rectangle in the draw's buffers.
    fn fill_prim_write(&mut self, d: &Draw, [x0, y0, x1, y1]: [u32; 4]) {
        self.prim_write.clear();
        self.prim_write.add_rect(&d.fb, x0, y0, x1, y1);
        if d.z_used {
            self.prim_write.add_rect(&d.zb, x0, y0, x1, y1);
        }
    }

    fn clear(&mut self) {
        self.draws.clear();
        self.prims.clear();
        self.bins.iter_mut().for_each(Vec::clear);
        self.write.clear();
        self.read.clear();
        self.local_reads.clear();
        self.local_read.clear();
        self.surfs.clear();
        self.full_tex_draw = None;
        self.work = 0;
    }
}

fn estimate(p: &Prim) -> u64 {
    let rows = (p.y1 - p.y0 + 1).max(1) as u64;
    match &p.shape {
        raster::Shape::Point { .. } => 1,
        raster::Shape::Line { .. } => 64,
        raster::Shape::Sprite { .. } | raster::Shape::Tri(_) => rows * 32,
    }
}

impl Gs {
    /// Debug (`draw_log`): record the current draw setup and its texture.
    fn log_draw(&mut self) {
        self.flush();
        self.wait_inflight();
        let d = self.draw.clone().expect("draw setup");
        let attr = if self.regs.prmodecont & 1 != 0 { self.regs.prim } else { (self.regs.prim & 7) | (self.regs.prmode & !7) };
        let ctx = &self.regs.ctx[((attr >> 9) & 1) as usize];
        let mut e = super::DrawLog {
            prim: attr,
            tex0: ctx.tex0,
            tex1: ctx.tex1,
            clamp: ctx.clamp,
            frame: ctx.frame,
            alpha: ctx.alpha,
            test: ctx.test,
            ..Default::default()
        };
        if let (Some(t), None) = (d.tex.as_ref(), self.gpu.as_ref()) {
            let w = (1u32 << t.tw).min(1024);
            let h = (1u32 << t.th).min(1024);
            let dec = raster::decode(&self.vram, t, [0, 0, w - 1, h - 1]);
            e.image = Some((w, h, dec.into_data()));
        }
        self.draw_log.as_mut().expect("log").push(e);
    }

    /// Queue a kicked primitive for drawing.
    pub(super) fn draw_primitive(&mut self, prim: u32, verts: &[Vertex]) {
        self.prims_drawn += 1;
        if self.draws_suppressed {
            return;
        }
        if self.draw.is_none() {
            self.draw = Some(Arc::new(Draw::build(&self.regs, &self.clut)));
            if self.draw_log.is_some() {
                self.log_draw();
            }
        }
        let d = self.draw.clone().expect("draw setup");
        let Some(mut p) = raster::setup(&d, 0, prim, verts) else {
            return;
        };
        if let Some(e) = self.draw_log.as_mut().and_then(|l| l.last_mut()) {
            if e.prims == 0 {
                e.bbox = [p.x0, p.y0, p.x1, p.y1];
            } else {
                e.bbox = [e.bbox[0].min(p.x0), e.bbox[1].min(p.y0), e.bbox[2].max(p.x1), e.bbox[3].max(p.y1)];
            }
            e.prims += 1;
        }
        if self.n3_capture {
            let surfaces = self.n3_surfaces();
            if let Some(g) = self.gpu.as_mut() {
                if g.n3_push(&d, &p, prim, verts, surfaces, &self.vram) {
                    return;
                }
            }
        }
        if let Some(g) = self.gpu.as_mut() {
            g.push(&d, &p, prim, verts, &self.vram);
            return;
        }
        if !d.row_parallel || p.x1 >= d.fb_width || (d.z_used && p.x1 >= d.zb_width) {
            // Not splittable by rows; pixels beyond the buffer width alias
            // other rows.
            self.draw_alone(&d, &p);
            return;
        }
        let registered = self
            .batch
            .draws
            .last()
            .is_some_and(|last| Arc::ptr_eq(last, &d));
        if !registered {
            if self.batch.aliases(&d) {
                self.batch.stats.conflict_flushes += 1;
                self.flush();
            }
            self.batch.register(d.clone());
        }

        let b = &mut self.batch;
        // What this primitive writes: its bounding box in each buffer. Most
        // checks only need pages; the block set (`prim_write`) is built when
        // a page-level test hits.
        let (x0, y0, x1, y1) = (p.x0 as u32, p.y0 as u32, p.x1 as u32, p.y1 as u32);
        let mut rect = [x0, y0, x1, y1];
        let page_sets = |rect: [u32; 4]| {
            let [x0, y0, x1, y1] = rect;
            let fb = Pages::rect(&d.fb, x0, y0, x1, y1);
            let zb = if d.z_used {
                Pages::rect(&d.zb, x0, y0, x1, y1)
            } else {
                Pages::default()
            };
            let mut all = fb;
            all.union(&zb);
            (fb, zb, all)
        };
        let (mut fb_pages, mut zb_pages, mut wpages) = page_sets(rect);
        let mut fine = false;
        // Blocks it reads: the whole texture, unless that overlaps something
        // written; then just the texels its coordinates can reach.
        let mut exact_tex = false;
        if d.tex_surface().is_some() && (d.tex_self || d.tex_pages.hits(&b.write.pages)) {
            b.prim_tex.clear();
            match raster::prim_tex_rect(&d, &p, prim, verts) {
                Some([u0, v0, u1, v1]) => {
                    let ts = d.tex_surface().expect("textured");
                    b.prim_tex.add_rect(ts, u0, v0, u1, v1);
                }
                None => b.prim_tex.add_pages(&d.tex_pages),
            }
            exact_tex = true;
        }
        let (mut own, mut hit, mut local) = (false, false, false);
        if exact_tex {
            if b.prim_tex.pages.hits(&wpages) {
                b.fill_prim_write(&d, rect);
                fine = true;
                own = b.prim_tex.hits(&b.prim_write);
            }
            hit = b.write.hits(&b.prim_tex);
            if (own || hit) && b.row_local(&d, &p) {
                (own, hit, local) = (false, false, true);
            }
        }
        if own {
            // The primitive reads blocks it writes itself. If those blocks
            // are only written by its first rows, draw the first rows alone
            // (in order, as serial drawing would) and queue the rest: later
            // rows then read final data. Otherwise draw all of it alone.
            let split = match p.shape {
                raster::Shape::Tri(_) | raster::Shape::Sprite { .. } => {
                    let mut end = b.prim_tex.rows_hit_end(&d.fb, x0, y0, x1, y1);
                    if d.z_used {
                        end = end.max(b.prim_tex.rows_hit_end(&d.zb, x0, y0, x1, y1));
                    }
                    end as i32
                }
                _ => p.y1 + 1,
            };
            if split > p.y1 {
                self.draw_alone(&d, &p);
                return;
            }
            let last = p.y1;
            p.y1 = split - 1;
            self.draw_alone(&d, &p);
            p.y0 = split;
            p.y1 = last;
            self.batch.register(d.clone());
            rect = [x0, split as u32, x1, y1];
            (fb_pages, zb_pages, wpages) = page_sets(rect);
            fine = false;
        }
        let b = &mut self.batch;
        // Read-after-write, or writing blocks that queued primitives read.
        let mut war = false;
        if wpages.hits(&b.read.pages) {
            if !fine {
                b.fill_prim_write(&d, rect);
                fine = true;
            }
            war = b.prim_write.hits(&b.read);
        }
        if !war && wpages.hits(&b.local_read.pages) {
            // Check each buffer against its own mapping.
            let mut fbw = Region::default();
            fbw.add_rect(&d.fb, x0, y0, x1, y1);
            war = b.local_war(&d.fb, &fbw);
            if d.z_used {
                let mut zbw = Region::default();
                zbw.add_rect(&d.zb, x0, y0, x1, y1);
                war |= b.local_war(&d.zb, &zbw);
            }
        }
        if hit || war {
            b.stats.conflict_flushes += 1;
            self.flush();
            self.batch.register(d.clone());
        }
        let b = &mut self.batch;
        let draw_idx = b.draws.len() - 1;
        p.draw = draw_idx as u32;
        if local {
            p.direct = true;
            let ts = *d.tex_surface().expect("textured");
            let key = raster::surf_key(&ts);
            b.local_read.union(&b.prim_tex);
            match b.local_reads.iter_mut().find(|(s, _)| raster::surf_key(s) == key) {
                Some((_, r)) => r.union(&b.prim_tex),
                None => b.local_reads.push((ts, b.prim_tex.clone())),
            }
        } else if exact_tex {
            b.read.union(&b.prim_tex);
        } else if d.tex_surface().is_some() && b.full_tex_draw != Some(draw_idx) {
            b.read.add_pages(&d.tex_pages);
            b.full_tex_draw = Some(draw_idx);
        }
        let _ = fine;
        b.write.add(&d.fb, rect, &fb_pages);
        if d.z_used {
            b.write.add(&d.zb, rect, &zb_pages);
        }
        b.work += estimate(&p);
        let idx = b.prims.len() as u32;
        if b.bins.is_empty() {
            b.bins = vec![Vec::new(); UNITS as usize];
        }
        let (b0, b1) = (p.y0 >> raster::BAND_SHIFT, p.y1 >> raster::BAND_SHIFT);
        if b1 - b0 + 1 >= UNITS {
            b.bins.iter_mut().for_each(|bin| bin.push(idx));
        } else {
            for band in b0..=b1 {
                b.bins[band.rem_euclid(UNITS) as usize].push(idx);
            }
        }
        b.prims.push(p);
        if b.prims.len() >= MAX_PRIMS {
            self.flush();
        }
    }

    /// The frame (and Z) surfaces of the current draw context for a native
    /// run; Z is left out when the draw does not use it.
    fn n3_surfaces(&self) -> super::gpu::NativeSurfaces {
        let r = &self.regs;
        let attr = if r.prmodecont & 1 != 0 { r.prim } else { (r.prim & 7) | (r.prmode & !7) };
        let c = &r.ctx[((attr >> 9) & 1) as usize];
        let mut s = super::gpu::NativeSurfaces::from_regs(c.frame, c.zbuf, c.scissor);
        let z_used = self.draw.as_ref().is_some_and(|d| d.z_used);
        if !z_used {
            s.z = None;
        }
        s
    }

    /// Flush, then draw one primitive on this thread.
    fn draw_alone(&mut self, d: &Draw, p: &Prim) {
        self.batch.stats.serial_prims += 1;
        self.sync();
        raster::raster(Mem::new(&mut self.vram), d, p, Rows::All, &[]);
        let (x0, y0, x1, y1) = (p.x0 as u32, p.y0 as u32, p.x1 as u32, p.y1 as u32);
        let mut pages = raster::Pages::rect(&d.fb, x0, y0, x1, y1);
        if d.z_used {
            pages.union(&raster::Pages::rect(&d.zb, x0, y0, x1, y1));
        }
        self.texcache.written(&pages);
    }

    /// Counters describing how drawing was batched so far.
    pub fn batch_stats(&self) -> Stats {
        let mut s = self.batch.stats;
        (s.tex_hits, s.tex_decodes, s.tex_decoded_texels) = self.texcache.stats;
        s
    }

    /// Wait for the batch being drawn in the background, if any.
    pub(super) fn wait_inflight(&mut self) {
        if let Some(f) = self.batch.inflight.take() {
            super::pool::Pool::global().wait(f.ticket);
        }
    }

    /// Start drawing everything queued. Large batches are drawn by the
    /// worker pool in the background (see `wait_inflight`); the previous
    /// background batch is always finished first.
    pub(super) fn flush(&mut self) {
        if let Some(g) = self.gpu.as_mut() {
            // GPU: close the batch; it is submitted at `sync` / `vblank`.
            g.close(&self.vram);
            return;
        }
        self.wait_inflight();
        if self.batch.is_empty() {
            return;
        }
        self.batch.stats.flushes += 1;
        self.batch.stats.batched_prims += self.batch.prims.len() as u64;
        // Decoded textures, per draw, for primitives that sample a texture
        // that stays unchanged during the batch. A texture is worth decoding
        // when it is sampled at least as often as it has texels (or is small).
        let n = self.batch.draws.len();
        let mut fetches = vec![0u64; n];
        for p in self.batch.prims.iter().filter(|p| !p.direct) {
            fetches[p.draw as usize] += p.area as u64 * if p.linear { 4 } else { 1 };
        }
        let mut decs: Vec<Vec<Option<Arc<Decoded>>>> = vec![Vec::new(); n];
        for (i, dec) in decs.iter_mut().enumerate() {
            if fetches[i] > 0 {
                let budget = fetches[i].max(16 * 1024);
                for t in self.batch.draws[i].tex_levels() {
                    dec.push(self.texcache.get(&self.vram, t, budget));
                }
            }
        }
        // Primitives sampling decoded copies no longer read their textures
        // from VRAM, so uploads may overwrite them while the batch draws.
        let all_decoded = (0..n).all(|i| {
            fetches[i] == 0
                || self.batch.draws[i].tex_surface().is_none()
                || decs[i].iter().all(Option::is_some)
        });
        let job = Arc::new(Job {
            draws: std::mem::take(&mut self.batch.draws),
            prims: std::mem::take(&mut self.batch.prims),
            bins: std::mem::take(&mut self.batch.bins),
            decs,
            m: Mem::new(&mut self.vram),
        });
        let written = self.batch.write.pages;
        self.texcache.written(&written);
        let pool = super::pool::Pool::global();
        if self.batch.work < PARALLEL_MIN_WORK || pool.threads() == 1 {
            job.draw_all();
            self.batch.clear();
        } else {
            self.batch.stats.parallel_flushes += 1;
            let j = job.clone();
            let ticket = pool.spawn(
                UNITS as usize,
                Arc::new(move |u| j.draw_unit(u)),
            );
            let b = &mut self.batch;
            b.inflight = Some(InFlight {
                ticket,
                write: std::mem::take(&mut b.write),
                read: if all_decoded {
                    Region::default()
                } else {
                    std::mem::take(&mut b.read)
                },
                local_read: std::mem::take(&mut b.local_read),
            });
            b.clear();
        }
    }

    /// Draw everything queued and wait until it is in `vram`.
    pub(super) fn flush_wait(&mut self) {
        self.flush();
        self.wait_inflight();
    }

    /// Before overwriting the `x0..=x1` × `y0..=y1` rectangle of `surf`
    /// (`r`): finish drawing whatever reads or writes it.
    pub(super) fn before_vram_write(&mut self, r: &Region) {
        self.before_vram_write_full(r, false);
    }

    /// `before_vram_write`, where `whole_words` says the write stores whole
    /// 32-bit words for every block of `r` (so fully covered pages need not
    /// be read back first on the GPU backend).
    pub(super) fn before_vram_write_full(&mut self, r: &Region, whole_words: bool) {
        if let Some(g) = self.gpu.as_mut() {
            // Pending native triangles that read or draw these pages go first.
            g.n3_touch(&r.pages, &self.vram);
            // The CPU writes the mirror and whole pages are uploaded later,
            // so the pages must be current in the mirror first.
            if g.batch_touches(r) {
                g.note_transfer_close();
                g.close(&self.vram);
            }
            let mut need = r.pages;
            if whole_words {
                let full = r.full_pages();
                for (n, f) in need.0.iter_mut().zip(full.0) {
                    *n &= !f;
                }
            }
            if g.gpu_dirty.hits(&need) {
                g.readback(&need, &mut self.vram, "cpu write");
            }
            // Whole pages are uploaded, so the GPU's copy is replaced.
            for (d, p) in g.gpu_dirty.0.iter_mut().zip(r.pages.0) {
                *d &= !p;
            }
            g.cpu_dirty.union(&r.pages);
            return;
        }
        if !self.batch.is_empty() && self.batch.touches(r) {
            self.batch.stats.transfer_flushes += 1;
            self.flush();
        }
        if self.batch.inflight.as_mut().is_some_and(|f| f.touches(r)) {
            self.wait_inflight();
        }
    }

    /// Before reading a rectangle of `surf`: finish drawing whatever writes it.
    pub(super) fn before_vram_read(&mut self, surf: &Surface, x0: u32, y0: u32, x1: u32, y1: u32) {
        if let Some(g) = self.gpu.as_mut() {
            let mut r = Region::default();
            r.add_rect(surf, x0, y0, x1, y1);
            g.n3_touch(&r.pages, &self.vram);
            if g.batch_writes(&r) {
                g.close(&self.vram);
            }
            if g.gpu_dirty.hits(&r.pages) {
                g.readback(&r.pages, &mut self.vram, "cpu read");
            }
            return;
        }
        let inflight = self.batch.inflight.is_some();
        if self.batch.is_empty() && !inflight {
            return;
        }
        let mut r = Region::default();
        r.add_rect(surf, x0, y0, x1, y1);
        if !self.batch.is_empty() && self.batch.write.hits(&r) {
            self.batch.stats.clut_flushes += 1;
            self.flush_wait();
        } else if self.batch.inflight.as_mut().is_some_and(|f| f.write.hits(&r)) {
            self.wait_inflight();
        }
    }
}

/// A batch handed to the rasterizer threads.
struct Job {
    draws: Vec<Arc<Draw>>,
    prims: Vec<Prim>,
    bins: Vec<Vec<u32>>,
    decs: Vec<Vec<Option<Arc<Decoded>>>>,
    m: Mem,
}

impl Job {
    fn draw_prim(&self, p: &Prim, rows: Rows) {
        let i = p.draw as usize;
        let decs: &[_] = if p.direct { &[] } else { &self.decs[i] };
        raster::raster(self.m, &self.draws[i], p, rows, decs);
    }

    /// Draw every primitive, all rows.
    fn draw_all(&self) {
        for p in &self.prims {
            self.draw_prim(p, Rows::All);
        }
    }

    /// Draw the rows of one unit.
    fn draw_unit(&self, unit: usize) {
        let rows = Rows::Unit {
            unit: unit as i32,
            units: UNITS,
        };
        for &i in &self.bins[unit] {
            self.draw_prim(&self.prims[i as usize], rows);
        }
    }
}

/// The batch being drawn in the background, and the blocks it touches.
pub(crate) struct InFlight {
    ticket: super::pool::Ticket,
    write: WriteSet,
    read: Region,
    local_read: Region,
}

impl InFlight {
    fn touches(&mut self, r: &Region) -> bool {
        self.write.hits(r) || r.hits(&self.read) || r.hits(&self.local_read)
    }
}

/// Blocks written by queued primitives, kept as page sets plus rectangles;
/// the block-level set is only built when a page-level test hits.
#[derive(Default)]
pub(crate) struct WriteSet {
    pub(crate) pages: Pages,
    fine: Region,
    pending: Vec<(Surface, [u32; 4])>,
}

impl WriteSet {
    pub(crate) fn add(&mut self, s: &Surface, rect: [u32; 4], pages: &Pages) {
        self.pages.union(pages);
        self.pending.push((*s, rect));
    }

    /// Whether a written block is in `r`.
    pub(crate) fn hits(&mut self, r: &Region) -> bool {
        if !r.pages.hits(&self.pages) {
            return false;
        }
        for (s, [x0, y0, x1, y1]) in self.pending.drain(..) {
            self.fine.add_rect(&s, x0, y0, x1, y1);
        }
        self.fine.hits(r)
    }

    pub(crate) fn clear(&mut self) {
        self.pages = Pages::default();
        self.fine.clear();
        self.pending.clear();
    }
}
