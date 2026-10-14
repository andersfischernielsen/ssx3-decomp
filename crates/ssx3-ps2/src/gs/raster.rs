//! Software rasterizer: primitive setup, texturing, per-pixel tests and blending.
//!
//! Coordinates are 12.4 fixed point; pixels are sampled at their integer
//! (top-left) corner, as the GS does. Triangles use integer edge functions
//! with a top-left fill rule; attributes are interpolated with plane equations
//! in f64 (S/T/Q linearly in screen space, then divided, which is what the
//! GS's pre-divided STQ expect).
//!
//! Drawing is split in two halves so it can run on several threads:
//! - `setup` (GS thread) turns a kicked primitive into a `Prim`: clipped
//!   bounding box, edge functions and attribute gradients.
//! - `raster` draws the rows of a `Prim` selected by a `Rows` filter. Every
//!   value a pixel depends on is computed from the row and column directly
//!   (never accumulated across rows), so any partition of the rows produces
//!   exactly the same pixels as drawing the primitive in one go.

use super::swizzle::{self, Kind, Surface, VRAM_WORDS};
use super::{Regs, Vertex};
use std::sync::Arc;

// --------------------------------------------------------------------- memory

/// Raw VRAM handle shared by rasterizer threads. Every access is masked to
/// the 4 MiB, so it can never go out of bounds; the batch scheduler makes
/// sure that concurrently drawn rows never touch the same words.
#[derive(Clone, Copy)]
pub(crate) struct Mem(*mut u32);
// SAFETY: see the type docs; races are excluded by the scheduler.
unsafe impl Send for Mem {}
unsafe impl Sync for Mem {}

impl Mem {
    pub(crate) fn new(v: &mut [u32]) -> Mem {
        assert_eq!(v.len(), VRAM_WORDS);
        Mem(v.as_mut_ptr())
    }
    #[inline(always)]
    fn rd(self, i: usize) -> u32 {
        // SAFETY: masked into the VRAM_WORDS allocation.
        unsafe { *self.0.add(i & (VRAM_WORDS - 1)) }
    }
    #[inline(always)]
    fn wr(self, i: usize, v: u32) {
        // SAFETY: masked into the VRAM_WORDS allocation.
        unsafe { *self.0.add(i & (VRAM_WORDS - 1)) = v }
    }
}

/// Addressing of one row of a surface, with everything that depends only on
/// `y` precomputed. Equivalent to `Surface::elem`.
#[derive(Clone, Copy)]
pub(crate) struct SurfRow {
    tab: *const u16,
    pwm: u32,
    pw_shift: u32,
    es: u32,
    mask: u32,
    base: u32,
    kind: Kind,
}

impl SurfRow {
    #[inline(always)]
    pub(crate) fn new(s: &Surface, y: u32) -> SurfRow {
        let y = y & 0x7FF;
        let ph_mask = (1 << s.ph_shift) - 1;
        let toff = ((y & ph_mask) << s.pw_shift) as usize;
        debug_assert!(toff + (1 << s.pw_shift) <= s.table.len());
        // SAFETY: `y & ph_mask` < page height, so the row lies inside the table.
        let tab = unsafe { s.table.as_ptr().add(toff) };
        SurfRow {
            tab,
            pwm: (1 << s.pw_shift) - 1,
            pw_shift: s.pw_shift,
            es: s.es,
            mask: ((VRAM_WORDS as u32) << s.es).wrapping_sub(1),
            base: s
                .base
                .wrapping_add(((y >> s.ph_shift) * s.pages_per_row).wrapping_mul(2048)),
            kind: s.kind,
        }
    }

    #[inline(always)]
    fn locate(&self, x: u32) -> (usize, u32) {
        let x = x & 0x7FF;
        let wb = self.base.wrapping_add((x >> self.pw_shift).wrapping_mul(2048));
        // SAFETY: `x & pwm` < row length (1 << pw_shift).
        let off = unsafe { *self.tab.add((x & self.pwm) as usize) } as u32;
        let e = (wb << self.es).wrapping_add(off) & self.mask;
        let word = (e >> self.es) as usize;
        let shift = (e & ((1 << self.es) - 1)) << (5 - self.es);
        (word, shift)
    }

    #[inline(always)]
    fn read(&self, m: Mem, x: u32) -> u32 {
        self.read_at(m, self.locate(x))
    }

    #[inline(always)]
    fn read_at(&self, m: Mem, (w, s): (usize, u32)) -> u32 {
        let v = m.rd(w);
        match self.kind {
            Kind::W32 => v,
            Kind::W24 => v & 0x00FF_FFFF,
            Kind::H16 => (v >> s) & 0xFFFF,
            Kind::B8 => (v >> s) & 0xFF,
            Kind::N4 => (v >> s) & 0xF,
            Kind::Hi8 => v >> 24,
            Kind::Hl4 => (v >> 24) & 0xF,
            Kind::Hh4 => v >> 28,
        }
    }

    /// `Surface::write` of pixel (x, this row).
    #[inline(always)]
    pub(crate) fn write(&self, m: Mem, x: u32, value: u32) {
        self.write_at(m, self.locate(x), value)
    }

    #[inline(always)]
    fn write_at(&self, m: Mem, (w, s): (usize, u32), value: u32) {
        let slot = m.rd(w);
        let v = match self.kind {
            Kind::W32 => value,
            Kind::W24 => (slot & 0xFF00_0000) | (value & 0x00FF_FFFF),
            Kind::H16 => (slot & !(0xFFFF << s)) | ((value & 0xFFFF) << s),
            Kind::B8 => (slot & !(0xFF << s)) | ((value & 0xFF) << s),
            Kind::N4 => (slot & !(0xF << s)) | ((value & 0xF) << s),
            Kind::Hi8 => (slot & 0x00FF_FFFF) | (value << 24),
            Kind::Hl4 => (slot & 0xF0FF_FFFF) | ((value & 0xF) << 24),
            Kind::Hh4 => (slot & 0x0FFF_FFFF) | ((value & 0xF) << 28),
        };
        m.wr(w, v);
    }

    #[inline(always)]
    fn write_masked_at(&self, m: Mem, at: (usize, u32), value: u32, keep: u32) {
        if keep == 0 {
            self.write_at(m, at, value);
        } else {
            let old = self.read_at(m, at);
            self.write_at(m, at, (value & !keep) | (old & keep));
        }
    }
}

// --------------------------------------------------------------------- page sets

/// A set of 8 KiB VRAM pages (512 of them).
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(crate) struct Pages(pub [u64; 8]);

impl Pages {
    pub(crate) const ALL: Pages = Pages([u64::MAX; 8]);

    #[inline]
    pub(crate) fn add(&mut self, p: u32) {
        let p = p & 511;
        self.0[(p >> 6) as usize] |= 1 << (p & 63);
    }
    #[inline]
    pub(crate) fn hits(&self, o: &Pages) -> bool {
        self.0.iter().zip(&o.0).any(|(a, b)| a & b != 0)
    }
    #[inline]
    pub(crate) fn union(&mut self, o: &Pages) {
        for (a, b) in self.0.iter_mut().zip(&o.0) {
            *a |= b;
        }
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.0.iter().all(|&w| w == 0)
    }

    /// Pages touched by the pixels `x0..=x1` × `y0..=y1` of `s`.
    pub(crate) fn rect(s: &Surface, x0: u32, y0: u32, x1: u32, y1: u32) -> Pages {
        let mut p = Pages::default();
        let (x0, y0) = (x0.min(0x7FF), y0.min(0x7FF));
        let (x1, y1) = (x1.min(0x7FF), y1.min(0x7FF));
        if x0 > x1 || y0 > y1 {
            return p;
        }
        let base_page = s.base >> 11;
        let unaligned = s.base & 2047 != 0;
        let (c0, c1) = (x0 >> s.pw_shift, x1 >> s.pw_shift);
        let (r0, r1) = (y0 >> s.ph_shift, y1 >> s.ph_shift);
        if (r1 - r0 + 1) * (c1 - c0 + 1) >= 512 {
            return Pages::ALL;
        }
        for r in r0..=r1 {
            for c in c0..=c1 {
                let pg = base_page.wrapping_add(c + r * s.pages_per_row);
                p.add(pg);
                if unaligned {
                    p.add(pg.wrapping_add(1));
                }
            }
        }
        p
    }
}

// --------------------------------------------------------------------- draw state

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum FbFmt {
    C32,
    C24,
    C16,
}

/// Blend equation classes with a cheaper evaluation.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Blend {
    /// A == B and D == Cs: `(X - X) * C >> 7 + Cs` is just Cs.
    Src,
    /// A = Cs, B = Cd, D = Cd.
    Lerp,
    General,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum TexKind {
    C32,
    C24,
    C16,
    Pal,
}

#[derive(Clone)]
pub(crate) struct Tex {
    pub(crate) surf: Surface,
    pub(crate) kind: TexKind,
    pub(crate) tw: i32,
    pub(crate) th: i32,
    pub(crate) wms: u32,
    pub(crate) wmt: u32,
    pub(crate) minu: i32,
    pub(crate) maxu: i32,
    pub(crate) minv: i32,
    pub(crate) maxv: i32,
    pub(crate) ta0: u32,
    pub(crate) ta1: u32,
    pub(crate) aem: bool,
    pub(crate) palette: Box<[u32; 256]>,
    pub(crate) pal_mask: usize,
    pub(crate) mag_linear: bool,
    pub(crate) min_linear: bool,
    pub(crate) lcm: bool,
    pub(crate) l: u32,
    pub(crate) k: f32,
    /// `(1 << tw)` and `(1 << th)` as f64.
    pub(crate) scale_s: f64,
    pub(crate) scale_t: f64,
}

/// Mipmapping state (MXL > 0 with a MIPMAP minification filter).
pub(crate) struct Mip {
    /// MXL: highest level.
    pub(crate) mxl: u32,
    /// MMIN 2..=5: within-level filter (4 and 5: bilinear) and level
    /// selection (odd: blend the two nearest levels).
    pub(crate) mmin: u32,
    /// Levels 1..=MXL (level 0 is `Draw::tex`).
    pub(crate) levels: Vec<Tex>,
}

pub(crate) struct Draw {
    pub(crate) iip: bool,
    pub(crate) fge: bool,
    pub(crate) abe: bool,
    pub(crate) fst: bool,
    pub(crate) fb: Surface,
    pub(crate) fb_fmt: FbFmt,
    pub(crate) fbmsk: u32,
    pub(crate) zb: Surface,
    pub(crate) zmax: u32,
    pub(crate) zte: bool,
    pub(crate) ztst: u32,
    pub(crate) zmsk: bool,
    pub(crate) ate: bool,
    pub(crate) atst: u32,
    pub(crate) aref: i32,
    pub(crate) afail: u32,
    pub(crate) date: bool,
    pub(crate) datm: bool,
    pub(crate) blend: Blend,
    pub(crate) ba: u32,
    pub(crate) bb: u32,
    pub(crate) bc: u32,
    pub(crate) bd: u32,
    pub(crate) fix: i32,
    pub(crate) pabe: bool,
    pub(crate) colclamp: bool,
    pub(crate) fba: bool,
    pub(crate) sx0: i32,
    pub(crate) sx1: i32,
    pub(crate) sy0: i32,
    pub(crate) sy1: i32,
    pub(crate) scanmsk: u32,
    pub(crate) dither: Option<[[i32; 4]; 4]>,
    pub(crate) ofx: i32,
    pub(crate) ofy: i32,
    pub(crate) tfx: u32,
    pub(crate) tcc: bool,
    pub(crate) fogcol: [i32; 3],
    pub(crate) tex: Option<Tex>,
    pub(crate) mip: Option<Box<Mip>>,
    /// VRAM pages the frame / Z buffer may write (scissor rectangle), and the
    /// texture may read.
    pub(crate) fb_pages: Pages,
    pub(crate) zb_pages: Pages,
    pub(crate) tex_pages: Pages,
    /// Width in pixels after which the frame / Z addresses wrap into the next
    /// page row (pixels beyond it alias other rows).
    pub(crate) fb_width: i32,
    pub(crate) zb_width: i32,
    /// Rows may be drawn independently (see `batch`): the buffer formats
    /// only share words between pixels of one row, and frame and Z do not
    /// alias.
    pub(crate) row_parallel: bool,
    /// Z buffer is read or written at all.
    pub(crate) z_used: bool,
    /// The texture's pages (for its whole wrap range) overlap the buffers;
    /// primitives then need their own, tighter texture page set.
    pub(crate) tex_self: bool,
}

#[inline]
fn expand16(c: u32, ta0: u32, ta1: u32, aem: bool) -> u32 {
    let r = (c & 0x1F) << 3;
    let g = ((c >> 5) & 0x1F) << 3;
    let b = ((c >> 10) & 0x1F) << 3;
    let a = if c & 0x8000 != 0 {
        ta1
    } else if aem && c & 0x7FFF == 0 {
        0
    } else {
        ta0
    };
    r | (g << 8) | (b << 16) | (a << 24)
}

/// Range of texel coordinates a wrap mode can produce.
pub(crate) fn wrap_range(mode: u32, size_log: i32, min: i32, max: i32) -> (u32, u32) {
    match mode {
        0 | 1 => (0, (1u32 << size_log) - 1),
        2 => (min as u32, max.max(min) as u32),
        // (c & min) | max: at least `max`, at most `min | max`.
        _ => (max as u32, (min | max) as u32),
    }
}

/// Identity of a buffer mapping: same key means same (x, y) -> word.
pub(crate) fn surf_key(s: &Surface) -> (u32, u32, u32) {
    (s.psm, s.base, s.pages_per_row)
}

impl Draw {
    pub(crate) fn build(regs: &Regs, clut: &[u16; 512]) -> Draw {
        let prim = regs.prim;
        let attr = if regs.prmodecont & 1 != 0 {
            prim
        } else {
            (prim & 7) | (regs.prmode & !7)
        };
        let ctx = &regs.ctx[((attr >> 9) & 1) as usize];

        let fbp = (ctx.frame & 0x1FF) as u32 * 32;
        let fbw = ((ctx.frame >> 16) & 0x3F) as u32;
        let fpsm = ((ctx.frame >> 24) & 0x3F) as u32;
        let fbmsk = (ctx.frame >> 32) as u32;
        let fb_fmt = match fpsm {
            swizzle::PSMCT24 | swizzle::PSMZ24 => FbFmt::C24,
            swizzle::PSMCT16 | swizzle::PSMCT16S | swizzle::PSMZ16 | swizzle::PSMZ16S => FbFmt::C16,
            _ => FbFmt::C32,
        };
        let zbp = (ctx.zbuf & 0x1FF) as u32 * 32;
        let zpsm = 0x30 | ((ctx.zbuf >> 24) & 0xF) as u32;
        let zmax = match zpsm {
            swizzle::PSMZ24 => 0xFF_FFFF,
            swizzle::PSMZ16 | swizzle::PSMZ16S => 0xFFFF,
            _ => 0xFFFF_FFFF,
        };
        let test = ctx.test;
        let alpha = ctx.alpha;
        let sc = ctx.scissor;

        let dthe = regs.dthe & 1 != 0 && fb_fmt == FbFmt::C16;
        let dither = if dthe {
            let mut m = [[0i32; 4]; 4];
            for (i, row) in m.iter_mut().enumerate() {
                for (j, v) in row.iter_mut().enumerate() {
                    let bits = ((regs.dimx >> (i * 16 + j * 4)) & 7) as i32;
                    *v = if bits & 4 != 0 { bits - 8 } else { bits };
                }
            }
            Some(m)
        } else {
            None
        };

        let tme = attr & 0x10 != 0;
        let tex0 = ctx.tex0;
        if tme && tex_trace() {
            use std::sync::Mutex;
            static SEEN: Mutex<Option<std::collections::HashSet<(u64, u64, u64)>>> = Mutex::new(None);
            let mut g = SEEN.lock().unwrap();
            let set = g.get_or_insert_with(Default::default);
            if set.len() < 4000 && set.insert((tex0, ctx.clamp, ctx.tex1)) {
                eprintln!(
                    "[tex] tbp {:#x} tbw {} psm {:#x} tw {} th {} cbp {:#x} cpsm {:#x} csm {} | clamp {:#x} | tex1 {:#x} | attr {attr:#x}",
                    tex0 & 0x3fff, (tex0 >> 14) & 0x3f, (tex0 >> 20) & 0x3f, (tex0 >> 26) & 0xf, (tex0 >> 30) & 0xf,
                    (tex0 >> 37) & 0x3fff, (tex0 >> 51) & 0xf, (tex0 >> 55) & 1, ctx.clamp, ctx.tex1
                );
            }
        }
        let tex = if tme {
            let tbp = (tex0 & 0x3FFF) as u32;
            let tbw = ((tex0 >> 14) & 0x3F) as u32;
            let tpsm = ((tex0 >> 20) & 0x3F) as u32;
            let tw = ((tex0 >> 26) & 0xF).min(10) as i32;
            let th = ((tex0 >> 30) & 0xF).min(10) as i32;
            let cpsm = ((tex0 >> 51) & 0xF) as u32;
            let csa = ((tex0 >> 56) & 0x1F) as u32;
            let texa = regs.texa;
            let ta0 = (texa & 0xFF) as u32;
            let aem = (texa >> 15) & 1 != 0;
            let ta1 = ((texa >> 32) & 0xFF) as u32;
            let kind = match tpsm {
                swizzle::PSMCT24 | swizzle::PSMZ24 => TexKind::C24,
                swizzle::PSMCT16 | swizzle::PSMCT16S | swizzle::PSMZ16 | swizzle::PSMZ16S => {
                    TexKind::C16
                }
                swizzle::PSMT8
                | swizzle::PSMT8H
                | swizzle::PSMT4
                | swizzle::PSMT4HL
                | swizzle::PSMT4HH => TexKind::Pal,
                _ => TexKind::C32,
            };
            let mut palette = Box::new([0u32; 256]);
            let mut pal_mask = 0;
            if kind == TexKind::Pal {
                let n = if matches!(tpsm, swizzle::PSMT8 | swizzle::PSMT8H) {
                    256
                } else {
                    16
                };
                pal_mask = n - 1;
                let is32 = !matches!(cpsm, 0x2 | 0xA);
                for i in 0..n as u32 {
                    palette[i as usize] = if is32 {
                        let slot = (((csa & 15) * 16 + i) & 255) as usize;
                        clut[slot] as u32 | (clut[slot + 256] as u32) << 16
                    } else {
                        expand16(clut[((csa * 16 + i) & 511) as usize] as u32, ta0, ta1, aem)
                    };
                }
            }
            let tex1 = ctx.tex1;
            let cl = ctx.clamp;
            let mmin = ((tex1 >> 6) & 7) as u32;
            let kraw = ((tex1 >> 32) & 0xFFF) as i32;
            let k = ((kraw << 20) >> 20) as f32 / 16.0;
            Some(Tex {
                surf: Surface::new(tpsm, tbp, tbw),
                kind,
                tw,
                th,
                wms: (cl & 3) as u32,
                wmt: ((cl >> 2) & 3) as u32,
                minu: ((cl >> 4) & 0x3FF) as i32,
                maxu: ((cl >> 14) & 0x3FF) as i32,
                minv: ((cl >> 24) & 0x3FF) as i32,
                maxv: ((cl >> 34) & 0x3FF) as i32,
                ta0,
                ta1,
                aem,
                palette,
                pal_mask,
                mag_linear: (tex1 >> 5) & 1 != 0,
                min_linear: matches!(mmin, 1 | 4 | 5),
                lcm: tex1 & 1 != 0,
                l: ((tex1 >> 19) & 3) as u32,
                k,
                scale_s: (1 << tw) as f64,
                scale_t: (1 << th) as f64,
            })
        } else {
            None
        };

        let mip = tex.as_ref().and_then(|t0| mip_levels(t0, ctx));

        let fb = Surface::new(fpsm, fbp, fbw);
        let zb = Surface::new(zpsm, zbp, fbw);
        let zte = (test >> 16) & 1 != 0;
        let sx0 = (sc & 0x7FF) as i32;
        let sx1 = ((sc >> 16) & 0x7FF) as i32;
        let sy0 = ((sc >> 32) & 0x7FF) as i32;
        let sy1 = ((sc >> 48) & 0x7FF) as i32;
        let fb_pages = Pages::rect(&fb, sx0 as u32, sy0 as u32, sx1 as u32, sy1 as u32);
        let ztst = ((test >> 17) & 3) as u32;
        let zmsk = (ctx.zbuf >> 32) & 1 != 0;
        // Z is read for GEQUAL/GREATER and written unless masked; ALWAYS with
        // ZMSK never touches the buffer.
        let z_used = zte && ztst != 0 && (ztst >= 2 || !zmsk);
        let zb_pages = if z_used {
            Pages::rect(&zb, sx0 as u32, sy0 as u32, sx1 as u32, sy1 as u32)
        } else {
            Pages::default()
        };
        let mut tex_pages = Pages::default();
        for t in tex.iter().chain(mip.iter().flat_map(|m| m.levels.iter())) {
            let (u0, u1) = wrap_range(t.wms, t.tw, t.minu, t.maxu);
            let (v0, v1) = wrap_range(t.wmt, t.th, t.minv, t.maxv);
            tex_pages.union(&Pages::rect(&t.surf, u0, v0, u1, v1));
        }
        let wraps = fb_pages == Pages::ALL || zb_pages == Pages::ALL;
        let row_kind = |k: Kind| matches!(k, Kind::W32 | Kind::W24 | Kind::H16);
        let mut row_parallel = row_kind(fb.kind) && (!z_used || row_kind(zb.kind)) && !wraps;
        if z_used && surf_key(&fb) != surf_key(&zb) && fb_pages.hits(&zb_pages) {
            row_parallel = false;
        }
        let tex_self = tex_pages.hits(&fb_pages) || tex_pages.hits(&zb_pages);

        Draw {
            iip: attr & 0x08 != 0,
            fge: attr & 0x20 != 0,
            abe: attr & 0x40 != 0,
            fst: attr & 0x100 != 0,
            fb,
            fb_fmt,
            fbmsk,
            zb,
            zmax,
            zte,
            ztst,
            zmsk,
            z_used,
            tex_self,
            // ATST=ALWAYS never fails, so it is the same as no test.
            ate: test & 1 != 0 && (test >> 1) & 7 != 1,
            atst: ((test >> 1) & 7) as u32,
            aref: ((test >> 4) & 0xFF) as i32,
            afail: ((test >> 12) & 3) as u32,
            date: (test >> 14) & 1 != 0,
            datm: (test >> 15) & 1 != 0,
            blend: match (alpha & 3, (alpha >> 2) & 3, (alpha >> 6) & 3) {
                (a, b, 0) if a == b => Blend::Src,
                (0, 1, 1) => Blend::Lerp,
                _ => Blend::General,
            },
            ba: (alpha & 3) as u32,
            bb: ((alpha >> 2) & 3) as u32,
            bc: ((alpha >> 4) & 3) as u32,
            bd: ((alpha >> 6) & 3) as u32,
            fix: ((alpha >> 32) & 0xFF) as i32,
            pabe: regs.pabe & 1 != 0,
            colclamp: regs.colclamp & 1 != 0,
            fba: ctx.fba & 1 != 0,
            sx0,
            sx1,
            sy0,
            sy1,
            scanmsk: (regs.scanmsk & 3) as u32,
            dither,
            ofx: (ctx.xyoffset & 0xFFFF) as i32,
            ofy: ((ctx.xyoffset >> 32) & 0xFFFF) as i32,
            tfx: ((tex0 >> 35) & 3) as u32,
            tcc: (tex0 >> 34) & 1 != 0,
            fogcol: [
                (regs.fogcol & 0xFF) as i32,
                ((regs.fogcol >> 8) & 0xFF) as i32,
                ((regs.fogcol >> 16) & 0xFF) as i32,
            ],
            tex,
            mip,
            fb_pages,
            zb_pages,
            tex_pages,
            fb_width: (fb.pages_per_row << fb.pw_shift) as i32,
            zb_width: (zb.pages_per_row << zb.pw_shift) as i32,
            row_parallel,
        }
    }

    pub(crate) fn tex_surface(&self) -> Option<&Surface> {
        self.tex.as_ref().map(|t| &t.surf)
    }

    /// Texture levels sampled: level 0, then mipmap levels 1..=MXL.
    pub(crate) fn tex_levels(&self) -> Vec<&Tex> {
        self.tex
            .iter()
            .chain(self.mip.iter().flat_map(|m| m.levels.iter()))
            .collect()
    }


    #[inline(always)]
    fn read_fb(&self, row: &SurfRow, m: Mem, at: (usize, u32)) -> u32 {
        let c = row.read_at(m, at);
        match self.fb_fmt {
            FbFmt::C32 => c,
            FbFmt::C24 => c | 0x8000_0000,
            FbFmt::C16 => {
                ((c & 0x1F) << 3)
                    | (((c >> 5) & 0x1F) << 11)
                    | (((c >> 10) & 0x1F) << 19)
                    | if c & 0x8000 != 0 { 0x8000_0000 } else { 0 }
            }
        }
    }

    #[inline(always)]
    fn write_fb(&self, row: &SurfRow, m: Mem, at: (usize, u32), c: u32, keep: u32) {
        match self.fb_fmt {
            FbFmt::C32 => row.write_masked_at(m, at, c, keep),
            FbFmt::C24 => row.write_masked_at(m, at, c & 0xFF_FFFF, keep & 0xFF_FFFF),
            FbFmt::C16 => {
                let pack = |v: u32| {
                    ((v >> 3) & 0x1F)
                        | (((v >> 11) & 0x1F) << 5)
                        | (((v >> 19) & 0x1F) << 10)
                        | ((v >> 16) & 0x8000)
                };
                row.write_masked_at(m, at, pack(c), pack(keep));
            }
        }
    }
}

/// Interpolated fragment inputs.
#[derive(Clone, Copy, Default)]
pub(crate) struct Frag {
    pub(crate) r: i32,
    pub(crate) g: i32,
    pub(crate) b: i32,
    pub(crate) a: i32,
    /// Texel-space coordinates.
    pub(crate) u: f32,
    pub(crate) v: f32,
    /// Interpolated Q (mipmap LOD).
    pub(crate) q: f32,
    pub(crate) f: i32,
    pub(crate) z: f64,
}

// --------------------------------------------------------------------- texturing

#[inline(always)]
fn wrap(c: i32, mode: u32, size_log: i32, min: i32, max: i32) -> i32 {
    match mode {
        0 => c & ((1 << size_log) - 1),
        1 => c.clamp(0, (1 << size_log) - 1),
        2 => c.clamp(min, max.max(min)),
        _ => (c & min) | max,
    }
}

impl Tex {
    pub(crate) fn surface(&self) -> &Surface {
        &self.surf
    }

    /// Raw texel value to RGBA8 (TEXA expansion, palette lookup).
    #[inline(always)]
    fn conv(&self, raw: u32) -> u32 {
        match self.kind {
            TexKind::C32 => raw,
            TexKind::C24 => {
                let a = if self.aem && raw == 0 { 0 } else { self.ta0 };
                raw | (a << 24)
            }
            TexKind::C16 => expand16(raw, self.ta0, self.ta1, self.aem),
            TexKind::Pal => self.palette[(raw as usize & self.pal_mask) & 255],
        }
    }
    #[inline(always)]
    fn wrap_u(&self, c: i32) -> u32 {
        wrap(c, self.wms, self.tw, self.minu, self.maxu) as u32
    }
    #[inline(always)]
    fn wrap_v(&self, c: i32) -> u32 {
        wrap(c, self.wmt, self.th, self.minv, self.maxv) as u32
    }
}

#[inline(always)]
fn sample(m: Mem, t: &Tex, u: f32, v: f32, linear: bool) -> u32 {
    if !linear {
        let x = t.wrap_u(u.floor() as i32);
        let row = SurfRow::new(&t.surf, t.wrap_v(v.floor() as i32));
        return t.conv(row.read(m, x));
    }
    let iu = ((u - 0.5) * 256.0).floor() as i32;
    let iv = ((v - 0.5) * 256.0).floor() as i32;
    let (x0, y0) = (iu >> 8, iv >> 8);
    let (fx, fy) = ((iu & 255) as u32, (iv & 255) as u32);
    // Texels with a zero weight do not contribute, so they are not fetched
    // (this also keeps the texels a primitive reads predictable, see
    // `prim_tex_rect`). With both weights zero the result is exactly c00.
    let xa = t.wrap_u(x0);
    let ra = SurfRow::new(&t.surf, t.wrap_v(y0));
    let c00 = t.conv(ra.read(m, xa));
    if fx | fy == 0 {
        return c00;
    }
    let xb = if fx != 0 { t.wrap_u(x0 + 1) } else { 0 };
    let c10 = if fx != 0 { t.conv(ra.read(m, xb)) } else { 0 };
    let (c01, c11) = if fy != 0 {
        let rb = SurfRow::new(&t.surf, t.wrap_v(y0 + 1));
        let c01 = t.conv(rb.read(m, xa));
        let c11 = if fx != 0 { t.conv(rb.read(m, xb)) } else { 0 };
        (c01, c11)
    } else {
        (0, 0)
    };
    bilerp(c00, c10, c01, c11, fx, fy)
}

/// Texels of a texture's whole reachable rectangle (`wrap_range` in U and
/// V), converted to RGBA8, so sampling needs no swizzling or palette lookups.
pub(crate) struct Decoded {
    u0: u32,
    v0: u32,
    w: u32,
    data: Vec<u32>,
}

impl Decoded {
    #[inline(always)]
    fn row(&self, y: u32) -> usize {
        ((y - self.v0) * self.w) as usize
    }
    #[inline(always)]
    fn at(&self, row: usize, x: u32) -> u32 {
        self.data[row + (x - self.u0) as usize]
    }
    pub(crate) fn bytes(&self) -> usize {
        self.data.len() * 4
    }
    pub(crate) fn into_data(self) -> Vec<u32> {
        self.data
    }
}

/// Largest texture rectangle (in texels) worth decoding.
const DECODE_MAX: u32 = 512 * 1024;

/// The texel rectangle `Decoded` covers for a texture (level), or `None`
/// if it is too large to decode.
pub(crate) fn decode_rect(t: &Tex) -> Option<[u32; 4]> {
    let (u0, u1) = wrap_range(t.wms, t.tw, t.minu, t.maxu);
    let (v0, v1) = wrap_range(t.wmt, t.th, t.minv, t.maxv);
    let area = (u1 - u0 + 1) * (v1 - v0 + 1);
    (area <= DECODE_MAX).then_some([u0, v0, u1, v1])
}

/// Everything decoded texels depend on besides VRAM: surface, rectangle and
/// conversion (TEXA and palette).
#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct TexKey {
    surf: (u32, u32, u32),
    rect: [u32; 4],
    conv: (u8, u32, u32, bool),
    palette: Option<Box<[u32; 256]>>,
}

pub(crate) fn tex_key(t: &Tex, rect: [u32; 4]) -> TexKey {
    TexKey {
        surf: surf_key(&t.surf),
        rect,
        conv: (t.kind as u8, t.ta0, t.ta1, t.aem),
        palette: (t.kind == TexKind::Pal).then(|| t.palette.clone()),
    }
}

/// Decode `rect` of texture (level) `t` from `vram`.
pub(crate) fn decode(vram: &[u32], t: &Tex, rect: [u32; 4]) -> Decoded {
    let [u0, v0, u1, v1] = rect;
    let w = u1 - u0 + 1;
    let h = v1 - v0 + 1;
    let mut data = vec![0u32; (w * h) as usize];
    let fill = |rows: &mut [u32], y0: u32| {
        for (i, out) in rows.chunks_exact_mut(w as usize).enumerate() {
            let row = SurfRow::new(&t.surf, y0 + i as u32);
            for (x, o) in (u0..=u1).zip(out.iter_mut()) {
                let (wd, sh) = row.locate(x);
                let v = vram[wd];
                let raw = match row.kind {
                    Kind::W32 => v,
                    Kind::W24 => v & 0x00FF_FFFF,
                    Kind::H16 => (v >> sh) & 0xFFFF,
                    Kind::B8 => (v >> sh) & 0xFF,
                    Kind::N4 => (v >> sh) & 0xF,
                    Kind::Hi8 => v >> 24,
                    Kind::Hl4 => (v >> 24) & 0xF,
                    Kind::Hh4 => v >> 28,
                };
                *o = t.conv(raw);
            }
        }
    };
    const CHUNK: u32 = 16;
    if w * h < 32 * 1024 {
        fill(&mut data, v0);
    } else {
        // Large textures: decode 16-row chunks on the worker pool.
        let chunks: Vec<std::sync::Mutex<&mut [u32]>> = data
            .chunks_mut((w * CHUNK) as usize)
            .map(std::sync::Mutex::new)
            .collect();
        super::pool::Pool::global().run(chunks.len(), &|u| {
            let mut c = chunks[u].lock().unwrap_or_else(|e| e.into_inner());
            fill(&mut c, v0 + u as u32 * CHUNK);
        });
    }
    Decoded { u0, v0, w, data }
}

/// `sample` on decoded texels: identical results while VRAM is unchanged.
#[inline(always)]
fn sample_dec(dec: &Decoded, t: &Tex, u: f32, v: f32, linear: bool) -> u32 {
    if !linear {
        let x = t.wrap_u(u.floor() as i32);
        let y = t.wrap_v(v.floor() as i32);
        return dec.at(dec.row(y), x);
    }
    let iu = ((u - 0.5) * 256.0).floor() as i32;
    let iv = ((v - 0.5) * 256.0).floor() as i32;
    let (x0, y0) = (iu >> 8, iv >> 8);
    let (fx, fy) = ((iu & 255) as u32, (iv & 255) as u32);
    let xa = t.wrap_u(x0);
    let ra = dec.row(t.wrap_v(y0));
    let c00 = dec.at(ra, xa);
    if fx | fy == 0 {
        return c00;
    }
    let xb = if fx != 0 { t.wrap_u(x0 + 1) } else { xa };
    let c10 = if fx != 0 { dec.at(ra, xb) } else { 0 };
    let (c01, c11) = if fy != 0 {
        let rb = dec.row(t.wrap_v(y0 + 1));
        (dec.at(rb, xa), if fx != 0 { dec.at(rb, xb) } else { 0 })
    } else {
        (0, 0)
    };
    bilerp(c00, c10, c01, c11, fx, fy)
}

/// Mipmap levels 1..=MXL from TEX1 / MIPTBP1 / MIPTBP2, when MXL > 0 and
/// MMIN selects a mipmap filter.
fn mip_levels(t0: &Tex, ctx: &super::Ctx) -> Option<Box<Mip>> {
    let tex1 = ctx.tex1;
    let mxl = ((tex1 >> 2) & 7).min(6) as u32;
    let mmin = ((tex1 >> 6) & 7) as u32;
    if mxl == 0 || !(2..=5).contains(&mmin) || no_mipmap() {
        return None;
    }
    // Base pointer (blocks) and width (64-texel units) of each level:
    // MIPTBP1 holds levels 1-3 and MIPTBP2 levels 4-6, 20 bits per level.
    // With TEX1.MTBA, levels 1-3 instead follow each other in memory after
    // level 0, each half as wide.
    let field = |reg: u64, i: u32| {
        let v = reg >> (i * 20);
        ((v & 0x3FFF) as u32, ((v >> 14) & 0x3F) as u32)
    };
    let mtba = (tex1 >> 9) & 1 != 0;
    let bpp = swizzle::bits_per_pixel(t0.surf.psm);
    let (mut bp, mut bw) = ((ctx.tex0 & 0x3FFF) as u32, ((ctx.tex0 >> 14) & 0x3F) as u32);
    let mut levels = Vec::with_capacity(mxl as usize);
    for l in 1..=mxl {
        let (lbp, lbw) = if l <= 3 && mtba {
            // Size of the previous level, in 256-byte blocks.
            let w = 1u32 << (t0.tw - l as i32 + 1).max(0);
            let h = 1u32 << (t0.th - l as i32 + 1).max(0);
            bp += (w * h * bpp).div_ceil(8 * 256);
            bw = (bw / 2).max(1);
            (bp, bw)
        } else if l <= 3 {
            field(ctx.miptbp1, l - 1)
        } else {
            field(ctx.miptbp2, l - 4)
        };
        let mut t = t0.clone();
        t.surf = Surface::new(t0.surf.psm, lbp, lbw);
        t.tw = (t0.tw - l as i32).max(0);
        t.th = (t0.th - l as i32).max(0);
        t.minu >>= l;
        t.maxu >>= l;
        t.minv >>= l;
        t.maxv >>= l;
        t.scale_s = (1 << t.tw) as f64;
        t.scale_t = (1 << t.th) as f64;
        levels.push(t);
    }
    Some(Box::new(Mip { mxl, mmin, levels }))
}

/// Sample a mipmapped texture. LOD is K (TEX1.LCM=1) or
/// `log2(1/|Q|) * 2^L + K` for the pixel's Q. LOD <= 0 magnifies level 0
/// with MMAG; otherwise MMIN picks the nearest level (2, 4) or blends the
/// two nearest (3, 5), with nearest (2, 3) or bilinear (4, 5) sampling
/// inside a level and LOD clamped to MXL. Level `l` is sampled at
/// texel coordinates `(u, v) / 2^l`.
#[inline(always)]
fn sample_mip(m: Mem, t0: &Tex, mip: &Mip, decs: &[Option<Arc<Decoded>>], f: &Frag) -> u32 {
    let lod = if t0.lcm {
        t0.k
    } else {
        let q = f.q.abs();
        let q = if q > 0.0 { q } else { 1.0 };
        (1.0 / q).log2() * (1u32 << t0.l) as f32 + t0.k
    };
    let level = |l: u32, linear: bool| {
        let t = if l == 0 { t0 } else { &mip.levels[l as usize - 1] };
        let scale = 1.0 / (1u32 << l) as f32;
        let (u, v) = (f.u * scale, f.v * scale);
        match decs.get(l as usize) {
            Some(Some(dec)) => sample_dec(dec, t, u, v, linear),
            _ => sample(m, t, u, v, linear),
        }
    };
    if lod.is_nan() || lod <= 0.0 {
        return level(0, t0.mag_linear);
    }
    let linear = mip.mmin >= 4;
    let lod = lod.min(mip.mxl as f32);
    if mip.mmin & 1 == 0 {
        // *_MIPMAP_NEAREST: the nearest level.
        return level(((lod + 0.5) as u32).min(mip.mxl), linear);
    }
    // *_MIPMAP_LINEAR: blend levels floor(LOD) and floor(LOD) + 1.
    let l0 = lod as u32;
    let c0 = level(l0, linear);
    let w = ((lod - l0 as f32) * 256.0) as u32;
    if l0 >= mip.mxl || w == 0 {
        return c0;
    }
    let c1 = level(l0 + 1, linear);
    let mut out = 0;
    for sh in [0, 8, 16, 24] {
        let (a, b) = ((c0 >> sh) & 0xFF, (c1 >> sh) & 0xFF);
        out |= ((a * (256 - w) + b * w + 128) >> 8) << sh;
    }
    out
}

/// Per channel: `((c00*(256-fx) + c10*fx)*(256-fy) +
/// (c01*(256-fx) + c11*fx)*fy + 0x8000) >> 16`, computed two channels per
/// u64 (32-bit lanes; every intermediate
/// stays below 2^25, so lanes never carry into each other and the result is
/// at most 255).
#[inline(always)]
fn bilerp(c00: u32, c10: u32, c01: u32, c11: u32, fx: u32, fy: u32) -> u32 {
    #[inline(always)]
    fn split(c: u32) -> (u64, u64) {
        let rb = (c & 0xFF) as u64 | (((c >> 16) & 0xFF) as u64) << 32;
        let ga = ((c >> 8) & 0xFF) as u64 | ((c >> 24) as u64) << 32;
        (rb, ga)
    }
    let (fx, fy) = (fx as u64, fy as u64);
    let (ix, iy) = (256 - fx, 256 - fy);
    let (rb00, ga00) = split(c00);
    let (rb10, ga10) = split(c10);
    let (rb01, ga01) = split(c01);
    let (rb11, ga11) = split(c11);
    const ROUND: u64 = 0x8000 | 0x8000 << 32;
    let rb = ((rb00 * ix + rb10 * fx) * iy + (rb01 * ix + rb11 * fx) * fy + ROUND) >> 16;
    let ga = ((ga00 * ix + ga10 * fx) * iy + (ga01 * ix + ga11 * fx) * fy + ROUND) >> 16;
    (rb & 0xFF) as u32
        | ((ga & 0xFF) as u32) << 8
        | (((rb >> 32) & 0xFF) as u32) << 16
        | (((ga >> 32) & 0xFF) as u32) << 24
}

// --------------------------------------------------------------------- pixel pipeline

/// Per-row state of the pixel pipeline.
struct RowCtx<'a> {
    /// Pre-decoded texels per texture level, where available.
    decs: &'a [Option<Arc<Decoded>>],
    fb: SurfRow,
    zb: SurfRow,
    dith: [i32; 4],
}

impl<'a> RowCtx<'a> {
    #[inline(always)]
    fn new(d: &Draw, y: i32, decs: &'a [Option<Arc<Decoded>>]) -> RowCtx<'a> {
        RowCtx {
            decs,
            fb: SurfRow::new(&d.fb, y as u32),
            zb: SurfRow::new(&d.zb, y as u32),
            dith: match &d.dither {
                Some(m) => m[(y & 3) as usize],
                None => [0; 4],
            },
        }
    }
}

/// Whether row `y` is drawn at all (scissor in Y and SCANMSK).
#[inline(always)]
fn row_visible(d: &Draw, y: i32) -> bool {
    if y < d.sy0 || y > d.sy1 {
        return false;
    }
    !matches!((d.scanmsk, y & 1), (2, 0) | (3, 1))
}

/// Texturing variants the span loops are specialized for.
const TX_NONE: u8 = 0;
const TX_NEAREST: u8 = 1;
const TX_LINEAR: u8 = 2;
/// The same, sampling pre-decoded texels (`Decoded`).
const TX_NEAREST_DEC: u8 = 3;
const TX_LINEAR_DEC: u8 = 4;
/// Mipmapped: level and filter chosen per pixel.
const TX_MIP: u8 = 5;

#[inline(always)]
fn tex_mode(d: &Draw, linear: bool, dec: bool) -> u8 {
    if d.mip.is_some() {
        return TX_MIP;
    }
    match (&d.tex, linear, dec) {
        (None, _, _) => TX_NONE,
        (Some(_), false, false) => TX_NEAREST,
        (Some(_), true, false) => TX_LINEAR,
        (Some(_), false, true) => TX_NEAREST_DEC,
        (Some(_), true, true) => TX_LINEAR_DEC,
    }
}

/// The pixel pipeline, unspecialized (points and lines).
#[inline(never)]
fn pixel(m: Mem, d: &Draw, rc: &RowCtx, x: i32, f: &Frag, linear: bool) {
    match tex_mode(d, linear, false) {
        TX_NONE => pixel_g::<TX_NONE>(m, d, rc, x, f.z, || *f),
        TX_NEAREST => pixel_g::<TX_NEAREST>(m, d, rc, x, f.z, || *f),
        TX_MIP => pixel_g::<TX_MIP>(m, d, rc, x, f.z, || *f),
        _ => pixel_g::<TX_LINEAR>(m, d, rc, x, f.z, || *f),
    }
}

/// The pixel pipeline for one pixel of a visible row; `x` is inside the
/// scissor. `TX` must match `tex_mode` of the draw and primitive.
#[inline(always)]
///
/// `fz` is the fragment's depth; the rest of the fragment comes from `frag`,
/// which is only evaluated for pixels that pass the early tests.
fn pixel_g<const TX: u8>(
    m: Mem,
    d: &Draw,
    rc: &RowCtx,
    x: i32,
    fz: f64,
    frag: impl FnOnce() -> Frag,
) {
    let xu = x as u32;
    // Nothing is written before the destination alpha and depth tests, and a
    // pixel failing either has no effect at all, so they run first (texturing
    // occluded pixels is the bulk of the work otherwise).
    let mut dst: Option<u32> = None;
    if d.date && d.fb_fmt != FbFmt::C24 {
        let c = d.read_fb(&rc.fb, m, rc.fb.locate(xu));
        if (c >> 31 != 0) != d.datm {
            return;
        }
        dst = Some(c);
    }
    let mut z = 0;
    let mut zat = (0, 0);
    if d.zte {
        if d.ztst == 0 {
            return;
        }
        // ALWAYS with ZMSK neither reads nor writes Z.
        if d.z_used {
            z = fz.clamp(0.0, d.zmax as f64) as u32;
            let pass = match d.ztst {
                1 => true,
                2 => {
                    zat = rc.zb.locate(xu);
                    z >= rc.zb.read_at(m, zat)
                }
                _ => {
                    zat = rc.zb.locate(xu);
                    z > rc.zb.read_at(m, zat)
                }
            };
            if !pass {
                return;
            }
        }
    }
    let f = frag();
    let (mut r, mut g, mut b, mut a) = (f.r, f.g, f.b, f.a);

    if let (true, Some(t)) = (TX != TX_NONE, &d.tex) {
        let c = match (TX, rc.decs.first()) {
            (TX_MIP, _) => match &d.mip {
                Some(mip) => sample_mip(m, t, mip, rc.decs, &f),
                None => 0,
            },
            (TX_NEAREST_DEC | TX_LINEAR_DEC, Some(Some(dec))) => {
                sample_dec(dec, t, f.u, f.v, TX == TX_LINEAR_DEC)
            }
            _ => sample(m, t, f.u, f.v, TX == TX_LINEAR || TX == TX_LINEAR_DEC),
        };
        let tr = (c & 0xFF) as i32;
        let tg = ((c >> 8) & 0xFF) as i32;
        let tb = ((c >> 16) & 0xFF) as i32;
        let ta = (c >> 24) as i32;
        match d.tfx {
            0 => {
                r = ((tr * r) >> 7).min(255);
                g = ((tg * g) >> 7).min(255);
                b = ((tb * b) >> 7).min(255);
                if d.tcc {
                    a = ((ta * a) >> 7).min(255);
                }
            }
            1 => {
                r = tr;
                g = tg;
                b = tb;
                if d.tcc {
                    a = ta;
                }
            }
            2 => {
                r = (((tr * r) >> 7) + a).min(255);
                g = (((tg * g) >> 7) + a).min(255);
                b = (((tb * b) >> 7) + a).min(255);
                if d.tcc {
                    a = (ta + a).min(255);
                }
            }
            _ => {
                r = (((tr * r) >> 7) + a).min(255);
                g = (((tg * g) >> 7) + a).min(255);
                b = (((tb * b) >> 7) + a).min(255);
                if d.tcc {
                    a = ta;
                }
            }
        }
    }

    if d.fge {
        let fv = f.f.clamp(0, 255);
        r = (fv * r + (255 - fv) * d.fogcol[0]) >> 8;
        g = (fv * g + (255 - fv) * d.fogcol[1]) >> 8;
        b = (fv * b + (255 - fv) * d.fogcol[2]) >> 8;
    }

    let mut write_fb = true;
    let mut write_z = d.zte && !d.zmsk;
    let mut keep = d.fbmsk;
    if d.ate {
        let pass = match d.atst {
            0 => false,
            1 => true,
            2 => a < d.aref,
            3 => a <= d.aref,
            4 => a == d.aref,
            5 => a >= d.aref,
            6 => a > d.aref,
            _ => a != d.aref,
        };
        if !pass {
            match d.afail {
                0 => return,
                1 => write_z = false,
                2 => write_fb = false,
                _ => {
                    write_z = false;
                    keep |= 0xFF00_0000;
                }
            }
        }
    }


    if d.z_used && write_z {
        if d.ztst < 2 {
            zat = rc.zb.locate(xu);
        }
        rc.zb.write_at(m, zat, z);
    }

    if !write_fb || keep == 0xFFFF_FFFF {
        return;
    }
    let fbat = rc.fb.locate(xu);
    if d.abe && d.blend != Blend::Src && (!d.pabe || a >= 0x80) {
        let dc = dst.unwrap_or_else(|| d.read_fb(&rc.fb, m, fbat));
        if d.blend == Blend::Lerp {
            // (Cs - Cd) * C >> 7 + Cd
            let c = match d.bc {
                0 => a,
                1 => (dc >> 24) as i32,
                _ => d.fix,
            };
            let (dr, dg, db) = ((dc & 0xFF) as i32, ((dc >> 8) & 0xFF) as i32, ((dc >> 16) & 0xFF) as i32);
            r = (((r - dr) * c) >> 7) + dr;
            g = (((g - dg) * c) >> 7) + dg;
            b = (((b - db) * c) >> 7) + db;
        } else {
        let cd = [
            (dc & 0xFF) as i32,
            ((dc >> 8) & 0xFF) as i32,
            ((dc >> 16) & 0xFF) as i32,
        ];
        let ad = (dc >> 24) as i32;
        let cs = [r, g, b];
        let c = match d.bc {
            0 => a,
            1 => ad,
            _ => d.fix,
        };
        let mut out = [0i32; 3];
        for i in 0..3 {
            let sel = |s: u32| match s {
                0 => cs[i],
                1 => cd[i],
                _ => 0,
            };
            out[i] = (((sel(d.ba) - sel(d.bb)) * c) >> 7) + sel(d.bd);
        }
        r = out[0];
        g = out[1];
        b = out[2];
        }
    }
    if d.dither.is_some() {
        let dv = rc.dith[(x & 3) as usize];
        r += dv;
        g += dv;
        b += dv;
    }
    if d.colclamp {
        r = r.clamp(0, 255);
        g = g.clamp(0, 255);
        b = b.clamp(0, 255);
    } else {
        r &= 0xFF;
        g &= 0xFF;
        b &= 0xFF;
    }
    let mut oa = (a & 0xFF) as u32;
    if d.fba {
        oa |= 0x80;
    }
    let c = r as u32 | (g as u32) << 8 | (b as u32) << 16 | oa << 24;
    d.write_fb(&rc.fb, m, fbat, c, keep);
}

/// Full per-pixel path for points and lines (scissor checked here).
#[inline]
fn pixel_checked(m: Mem, d: &Draw, rows: Rows, x: i32, y: i32, f: &Frag, linear: bool) {
    if x < d.sx0 || x > d.sx1 || !row_visible(d, y) || !rows.owns(y) {
        return;
    }
    let rc = RowCtx::new(d, y, &[]);
    pixel(m, d, &rc, x, f, linear);
}

// --------------------------------------------------------------------- primitives

pub(crate) const R: usize = 0;
pub(crate) const G: usize = 1;
pub(crate) const B: usize = 2;
pub(crate) const A: usize = 3;
pub(crate) const S: usize = 4;
pub(crate) const T: usize = 5;
pub(crate) const Q: usize = 6;
pub(crate) const F: usize = 7;
pub(crate) const Z: usize = 8;
pub(crate) const NATTR: usize = 9;

fn attrs(d: &Draw, v: &Vertex) -> [f64; NATTR] {
    let (s, t, q) = if d.fst {
        (v.u as f64, v.v as f64, 1.0)
    } else {
        (v.s as f64, v.t as f64, v.q as f64)
    };
    [
        v.rgba[0] as f64,
        v.rgba[1] as f64,
        v.rgba[2] as f64,
        v.rgba[3] as f64,
        s,
        t,
        q,
        v.f as f64,
        v.z as f64,
    ]
}

/// Texture coordinates from interpolated S, T, Q.
#[inline(always)]
pub(crate) fn tex_uv(d: &Draw, s: f64, t: f64, q: f64) -> (f32, f32) {
    match &d.tex {
        Some(tx) => {
            if d.fst {
                ((s / 16.0) as f32, (t / 16.0) as f32)
            } else {
                let q = if q != 0.0 { q } else { f64::MIN_POSITIVE };
                ((s / q * tx.scale_s) as f32, (t / q * tx.scale_t) as f32)
            }
        }
        None => (0.0, 0.0),
    }
}

/// Build a fragment from interpolated attributes.
#[inline(always)]
fn frag(d: &Draw, at: &[f64; NATTR]) -> Frag {
    let (u, v) = tex_uv(d, at[S], at[T], at[Q]);
    Frag {
        r: at[R] as i32,
        g: at[G] as i32,
        b: at[B] as i32,
        a: at[A] as i32,
        u,
        v,
        q: at[Q] as f32,
        f: at[F] as i32,
        z: at[Z],
    }
}

/// Choose bilinear vs nearest for a primitive from TEX1 (no mipmapping).
fn linear_filter(d: &Draw, verts: &[Vertex]) -> bool {
    let Some(t) = &d.tex else { return false };
    let lod = if t.lcm {
        t.k
    } else {
        let q = if d.fst {
            1.0
        } else {
            let sum: f32 = verts.iter().map(|v| v.q.abs()).sum();
            sum / verts.len() as f32
        };
        let q = if q > 0.0 { q } else { 1.0 };
        (1.0 / q).log2() * (1u32 << t.l) as f32 + t.k
    };
    if lod <= 0.0 {
        t.mag_linear
    } else {
        t.min_linear
    }
}

/// Texel range `lo..=hi` (one axis) after wrapping, or the whole wrap range.
fn wrap_span(lo: f64, hi: f64, mode: u32, size_log: i32, min: i32, max: i32) -> (u32, u32) {
    if !(lo.is_finite() && hi.is_finite()) || lo < -1e9 || hi > 1e9 {
        return wrap_range(mode, size_log, min, max);
    }
    // Bilinear reads floor(u - 0.5) and the texel after it; leave a margin
    // for rounding in the per-pixel interpolation.
    wrap_span_i(lo.floor() as i64 - 2, hi.floor() as i64 + 2, mode, size_log, min, max)
}

/// Texel range of unwrapped coordinates `lo..=hi` after wrapping.
fn wrap_span_i(lo: i64, hi: i64, mode: u32, size_log: i32, min: i32, max: i32) -> (u32, u32) {
    let full = wrap_range(mode, size_log, min, max);
    let size = 1i64 << size_log;
    match mode {
        0 => {
            if hi - lo + 1 >= size {
                return full;
            }
            let (a, b) = ((lo & (size - 1)) as u32, (hi & (size - 1)) as u32);
            if a <= b {
                (a, b)
            } else {
                full
            }
        }
        1 => (lo.clamp(0, size - 1) as u32, hi.clamp(0, size - 1) as u32),
        2 => {
            let (mn, mx) = (min as i64, max.max(min) as i64);
            (lo.clamp(mn, mx) as u32, hi.clamp(mn, mx) as u32)
        }
        _ => {
            if hi - lo >= 1024 {
                return full;
            }
            let f = |c: i64| ((c as i32 & min) | max) as u32;
            (lo..=hi).fold((u32::MAX, 0), |(a, b), c| (a.min(f(c)), b.max(f(c))))
        }
    }
}

/// Texels (`[u0, v0, u1, v1]`) a single sprite or triangle can read: the texel
/// rectangle spanned by its vertices' texture coordinates (exact for UV and
/// for STQ when all Q share a sign, since S/Q over a triangle stays within
/// the vertex values). `None` means the whole texture.
pub(crate) fn prim_tex_rect(d: &Draw, p: &Prim, prim: u32, verts: &[Vertex]) -> Option<[u32; 4]> {
    if d.mip.is_some() {
        return None;
    }
    let t = d.tex.as_ref()?;
    if let Shape::Sprite {
        px0,
        px1,
        x0,
        y0,
        a0,
        at,
        dsdx,
        dtdy,
        ..
    } = &p.shape
    {
        // Exact: the coordinates are monotonic in the pixel position, so the
        // extreme texels come from the first and last row / column, computed
        // exactly as `raster` does.
        let su = |px: i32| a0[S] + dsdx * ((px * 16 - x0) as f64 / 16.0);
        let tv = |py: i32| a0[T] + dtdy * ((py * 16 - y0) as f64 / 16.0);
        let (ua, va) = tex_uv(d, su(*px0), tv(p.y0), at[Q]);
        let (ub, vb) = tex_uv(d, su(*px1), tv(p.y1), at[Q]);
        let span = |a: f32, b: f32| -> Option<(i64, i64)> {
            let (a, b) = (a.min(b), a.max(b));
            if !(a.is_finite() && b.is_finite()) || a < -1e9 || b > 1e9 {
                return None;
            }
            Some(if p.linear {
                let lo = ((a - 0.5) * 256.0).floor() as i32;
                let hi = ((b - 0.5) * 256.0).floor() as i32;
                ((lo >> 8) as i64, ((hi >> 8) + (hi & 255 != 0) as i32) as i64)
            } else {
                (a.floor() as i32 as i64, b.floor() as i32 as i64)
            })
        };
        if let (Some((ul, uh)), Some((vl, vh))) = (span(ua, ub), span(va, vb)) {
            let (u0, u1) = wrap_span_i(ul, uh, t.wms, t.tw, t.minu, t.maxu);
            let (v0, v1) = wrap_span_i(vl, vh, t.wmt, t.th, t.minv, t.maxv);
            return Some([u0, v0, u1, v1]);
        }
        return None;
    }
    let n = match prim {
        3..=5 => 3,
        6 => 2,
        _ => return None,
    };
    let mut us = [0.0f64; 3];
    let mut vs = [0.0f64; 3];
    for (i, v) in verts[..n].iter().enumerate() {
        if d.fst {
            us[i] = v.u as f64 / 16.0;
            vs[i] = v.v as f64 / 16.0;
        } else {
            // A sprite's Q is the second vertex's.
            let q = if prim == 6 { verts[1].q } else { v.q } as f64;
            let usable = if q > 0.0 {
                true
            } else {
                q < 0.0 && verts[..n].iter().all(|w| w.q < 0.0)
            };
            if !usable {
                return None;
            }
            if prim != 6 && !verts[..n].iter().all(|w| (w.q > 0.0) == (q > 0.0)) {
                return None;
            }
            us[i] = v.s as f64 / q * t.scale_s;
            vs[i] = v.t as f64 / q * t.scale_t;
        }
    }
    let (us, vs) = (&us[..n], &vs[..n]);
    let mn = |a: &[f64]| a.iter().copied().fold(f64::INFINITY, f64::min);
    let mx = |a: &[f64]| a.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let (u0, u1) = wrap_span(mn(us), mx(us), t.wms, t.tw, t.minu, t.maxu);
    let (v0, v1) = wrap_span(mn(vs), mx(vs), t.wmt, t.th, t.minv, t.maxv);
    Some([u0, v0, u1, v1])
}

/// Whether texel (x, y) of `a` is the same word as pixel (x, y) of `b`.
pub(crate) fn same_words(a: &Surface, b: &Surface) -> bool {
    std::ptr::eq(a.table, b.table)
        && a.es == b.es
        && a.base == b.base
        && a.pages_per_row == b.pages_per_row
}

/// Texel rows (or columns) `lo..=hi` that a coordinate reads.
#[inline]
fn texel_span(c: f32, linear: bool) -> (i64, i64) {
    if linear {
        let i = ((c - 0.5) * 256.0).floor() as i32;
        ((i >> 8) as i64, ((i >> 8) + (i & 255 != 0) as i32) as i64)
    } else {
        let i = c.floor() as i32 as i64;
        (i, i)
    }
}

/// For a sprite whose texture is (part of) a buffer it draws to: true when
/// every pixel row only reads texels from its own 8-row band and columns
/// that do not wrap into other rows. Rows can then still be drawn in
/// parallel: each band's reads and writes stay in submission order.
pub(crate) fn sprite_rows_local(d: &Draw, p: &Prim) -> bool {
    if d.mip.is_some() {
        return false;
    }
    let (
        Some(t),
        Shape::Sprite {
            px0,
            px1,
            x0,
            y0,
            a0,
            at,
            dsdx,
            dtdy,
            ..
        },
    ) = (&d.tex, &p.shape)
    else {
        return false;
    };
    let su = |px: i32| a0[S] + dsdx * ((px * 16 - x0) as f64 / 16.0);
    let tv = |py: i32| a0[T] + dtdy * ((py * 16 - y0) as f64 / 16.0);
    let ok = |c: f32| c.is_finite() && c.abs() < 1e9;
    let (ua, _) = tex_uv(d, su(*px0), 0.0, at[Q]);
    let (ub, _) = tex_uv(d, su(*px1), 0.0, at[Q]);
    if !ok(ua) || !ok(ub) {
        return false;
    }
    let (la, ha) = texel_span(ua.min(ub), p.linear);
    let (lb, hb) = texel_span(ua.max(ub), p.linear);
    let (_, u1) = wrap_span_i(la.min(lb), ha.max(hb), t.wms, t.tw, t.minu, t.maxu);
    if u1 >= t.surf.pages_per_row << t.surf.pw_shift {
        return false;
    }
    for py in p.y0..=p.y1 {
        let (_, v) = tex_uv(d, 0.0, tv(py), at[Q]);
        if !ok(v) {
            return false;
        }
        let (lo, hi) = texel_span(v, p.linear);
        let (r0, r1) = wrap_span_i(lo, hi, t.wmt, t.th, t.minv, t.maxv);
        let band = py >> BAND_SHIFT;
        if (r0 as i32) >> BAND_SHIFT != band || (r1 as i32) >> BAND_SHIFT != band {
            return false;
        }
    }
    true
}

/// Triangle setup: clipped bounding box, edge functions, attribute planes.
pub(crate) struct TriSetup {
    pub(crate) bx0: i32,
    pub(crate) bx1: i32,
    pub(crate) by0: i32,
    /// Edge function at (bx0, by0) and its steps per pixel in x and y.
    pub(crate) e0: [i64; 3],
    pub(crate) edx: [i64; 3],
    pub(crate) edy: [i64; 3],
    /// `a0 + dadx * (bx0 - ox)`: the attribute at column bx0, row oy.
    pub(crate) c: [f64; NATTR],
    pub(crate) dadx: [f64; NATTR],
    pub(crate) dady: [f64; NATTR],
    pub(crate) oy: f64,
}

pub(crate) enum Shape {
    Point {
        x: i32,
        y: i32,
        f: Frag,
    },
    Line {
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        a0: [f64; NATTR],
        a1: [f64; NATTR],
    },
    Sprite {
        px0: i32,
        px1: i32,
        x0: i32,
        y0: i32,
        a0: [f64; NATTR],
        at: [f64; NATTR],
        dsdx: f64,
        dtdy: f64,
        base: Frag,
    },
    Tri(Box<TriSetup>),
}

/// A primitive ready to be rasterized.
pub(crate) struct Prim {
    pub(crate) draw: u32,
    pub(crate) linear: bool,
    /// Rows the primitive may touch.
    pub(crate) y0: i32,
    pub(crate) y1: i32,
    /// Sample VRAM directly, not decoded texels (its texture changes while
    /// the batch is drawn).
    pub(crate) direct: bool,
    /// Rough number of pixels covered.
    pub(crate) area: u32,
    /// Columns the primitive may touch.
    pub(crate) x0: i32,
    pub(crate) x1: i32,
    pub(crate) shape: Shape,
}

/// Which rows of a primitive to draw.
#[derive(Clone, Copy)]
pub(crate) enum Rows {
    All,
    /// Rows in bands of `1 << BAND_SHIFT` with `band % units == unit`.
    Unit { unit: i32, units: i32 },
}

pub(crate) const BAND_SHIFT: i32 = 3;

impl Rows {
    #[inline(always)]
    fn owns(self, y: i32) -> bool {
        match self {
            Rows::All => true,
            Rows::Unit { unit, units } => (y >> BAND_SHIFT).rem_euclid(units) == unit,
        }
    }

    /// Call `f` for every owned row in `y0..=y1`, in increasing order.
    #[inline(always)]
    fn each(self, y0: i32, y1: i32, mut f: impl FnMut(i32)) {
        match self {
            Rows::All => {
                for y in y0..=y1 {
                    f(y);
                }
            }
            Rows::Unit { unit, units } => {
                if y0 > y1 {
                    return;
                }
                let b0 = y0 >> BAND_SHIFT;
                let mut b = b0 + (unit - b0).rem_euclid(units);
                while (b << BAND_SHIFT) <= y1 {
                    let r0 = (b << BAND_SHIFT).max(y0);
                    let r1 = ((b << BAND_SHIFT) + (1 << BAND_SHIFT) - 1).min(y1);
                    for y in r0..=r1 {
                        f(y);
                    }
                    b += units;
                }
            }
        }
    }
}

/// Set up a kicked primitive. `None` when nothing can be drawn.
pub(crate) fn setup(d: &Draw, draw: u32, prim: u32, verts: &[Vertex]) -> Option<Prim> {
    let linear = linear_filter(d, verts);
    let mk = |x0: i32, y0: i32, y1: i32, x1: i32, shape| {
        Some(Prim {
            draw,
            linear,
            direct: false,
            area: ((x1 - x0 + 1) * (y1 - y0 + 1)).max(1) as u32,
            x0,
            y0,
            y1,
            x1,
            shape,
        })
    };
    match prim {
        0 => {
            let v = &verts[0];
            let x = (v.x as i32 - d.ofx + 8) >> 4;
            let y = (v.y as i32 - d.ofy + 8) >> 4;
            if x < d.sx0 || x > d.sx1 || y < d.sy0 || y > d.sy1 {
                return None;
            }
            let f = frag(d, &attrs(d, v));
            mk(x, y, y, x, Shape::Point { x, y, f })
        }
        1 | 2 => {
            let (v0, v1) = (&verts[0], &verts[1]);
            let x0 = (v0.x as i32 - d.ofx) as f64 / 16.0;
            let y0 = (v0.y as i32 - d.ofy) as f64 / 16.0;
            let x1 = (v1.x as i32 - d.ofx) as f64 / 16.0;
            let y1 = (v1.y as i32 - d.ofy) as f64 / 16.0;
            let mut a0 = attrs(d, v0);
            let a1 = attrs(d, v1);
            if !d.iip {
                a0[..4].copy_from_slice(&a1[..4]);
            }
            let shape = Shape::Line {
                x0,
                y0,
                x1,
                y1,
                a0,
                a1,
            };
            // Pixels are clipped individually; rows/columns are bounded by the scissor.
            mk(d.sx0, d.sy0, d.sy1, d.sx1, shape)
        }
        3..=5 => tri_setup(d, verts).map(|(y0, y1, x1, t)| Prim {
            draw,
            linear,
            direct: false,
            area: ((x1 - t.bx0 + 1) * (y1 - y0 + 1) / 2).max(1) as u32,
            x0: t.bx0,
            y0,
            y1,
            x1,
            shape: Shape::Tri(t),
        }),
        6 => {
            let (v0, v1) = (&verts[0], &verts[1]);
            let x0 = v0.x as i32 - d.ofx;
            let y0 = v0.y as i32 - d.ofy;
            let x1 = v1.x as i32 - d.ofx;
            let y1 = v1.y as i32 - d.ofy;
            let (left, right) = (x0.min(x1), x0.max(x1));
            let (top, bottom) = (y0.min(y1), y0.max(y1));
            // Pixel p is covered when left <= p*16 < right.
            let px0 = ((left + 15) >> 4).max(d.sx0);
            let px1 = (((right + 15) >> 4) - 1).min(d.sx1);
            let py0 = ((top + 15) >> 4).max(d.sy0);
            let py1 = (((bottom + 15) >> 4) - 1).min(d.sy1);
            if px0 > px1 || py0 > py1 {
                return None;
            }
            let a0 = attrs(d, v0);
            let a1 = attrs(d, v1);
            // Flat attributes come from the second vertex; S/T (or U/V) interpolate.
            let at = a1;
            let dsdx = if x1 != x0 {
                (a1[S] - a0[S]) * 16.0 / (x1 - x0) as f64
            } else {
                0.0
            };
            let dtdy = if y1 != y0 {
                (a1[T] - a0[T]) * 16.0 / (y1 - y0) as f64
            } else {
                0.0
            };
            let base = frag(d, &at);
            mk(
                px0,
                py0,
                py1,
                px1,
                Shape::Sprite {
                    px0,
                    px1,
                    x0,
                    y0,
                    a0,
                    at,
                    dsdx,
                    dtdy,
                    base,
                },
            )
        }
        _ => None,
    }
}

fn tri_setup(d: &Draw, verts: &[Vertex]) -> Option<(i32, i32, i32, Box<TriSetup>)> {
    let mut p = [(0i64, 0i64); 3];
    let mut a = [[0.0f64; NATTR]; 3];
    for i in 0..3 {
        p[i] = (
            (verts[i].x as i32 - d.ofx) as i64,
            (verts[i].y as i32 - d.ofy) as i64,
        );
        a[i] = attrs(d, &verts[i]);
    }
    if !d.iip {
        let flat = a[2];
        for ai in a.iter_mut().take(2) {
            ai[..4].copy_from_slice(&flat[..4]);
        }
    }
    let mut area = (p[1].0 - p[0].0) * (p[2].1 - p[0].1) - (p[2].0 - p[0].0) * (p[1].1 - p[0].1);
    if area == 0 {
        return None;
    }
    if area < 0 {
        p.swap(1, 2);
        a.swap(1, 2);
        area = -area;
    }

    // Bounding box in pixels, clipped to the scissor.
    let minx = p.iter().map(|q| q.0).min().unwrap();
    let maxx = p.iter().map(|q| q.0).max().unwrap();
    let miny = p.iter().map(|q| q.1).min().unwrap();
    let maxy = p.iter().map(|q| q.1).max().unwrap();
    let bx0 = (((minx + 15) >> 4) as i32).max(d.sx0);
    let bx1 = ((maxx >> 4) as i32).min(d.sx1);
    let by0 = (((miny + 15) >> 4) as i32).max(d.sy0);
    let by1 = ((maxy >> 4) as i32).min(d.sy1);
    if bx0 > bx1 || by0 > by1 {
        return None;
    }

    // Edge functions E_ab(p) = (xb-xa)(py-ya) - (yb-ya)(px-xa), positive inside.
    let mut e0 = [0i64; 3];
    let mut edx = [0i64; 3];
    let mut edy = [0i64; 3];
    for i in 0..3 {
        let (xa, ya) = p[i];
        let (xb, yb) = p[(i + 1) % 3];
        let gx = -(yb - ya);
        let gy = xb - xa;
        let top_left = gx > 0 || (gx == 0 && gy > 0);
        let sx = bx0 as i64 * 16;
        let sy = by0 as i64 * 16;
        let e = gy * (sy - ya) + gx * (sx - xa);
        edx[i] = gx * 16;
        edy[i] = gy * 16;
        e0[i] = if top_left { e } else { e - 1 };
    }

    // Attribute plane gradients (per pixel).
    let fx = |i: usize| p[i].0 as f64 / 16.0;
    let fy = |i: usize| p[i].1 as f64 / 16.0;
    let (x10, y10) = (fx(1) - fx(0), fy(1) - fy(0));
    let (x20, y20) = (fx(2) - fx(0), fy(2) - fy(0));
    let inv = 1.0 / (area as f64 / 256.0);
    let mut dadx = [0.0f64; NATTR];
    let mut dady = [0.0f64; NATTR];
    let mut c = [0.0f64; NATTR];
    let (ox, oy) = (fx(0), fy(0));
    for k in 0..NATTR {
        let d1 = a[1][k] - a[0][k];
        let d2 = a[2][k] - a[0][k];
        dadx[k] = (d1 * y20 - d2 * y10) * inv;
        dady[k] = (d2 * x10 - d1 * x20) * inv;
        c[k] = a[0][k] + dadx[k] * (bx0 as f64 - ox);
    }
    Some((
        by0,
        by1,
        bx1,
        Box::new(TriSetup {
            bx0,
            bx1,
            by0,
            e0,
            edx,
            edy,
            c,
            dadx,
            dady,
            oy,
        }),
    ))
}

/// Draw the rows of `p` selected by `rows`. `dec`, when given, holds the
/// decoded texels of `d`'s texture (they must match VRAM).
pub(crate) fn raster(m: Mem, d: &Draw, p: &Prim, rows: Rows, decs: &[Option<Arc<Decoded>>]) {
    let dec = decs.first().and_then(|x| x.as_deref()).is_some();
    let linear = p.linear;
    match &p.shape {
        Shape::Point { x, y, f } => pixel_checked(m, d, rows, *x, *y, f, linear),
        Shape::Line {
            x0,
            y0,
            x1,
            y1,
            a0,
            a1,
        } => {
            let (x0, y0, x1, y1) = (*x0, *y0, *x1, *y1);
            let steps = (x1 - x0).abs().max((y1 - y0).abs()).round() as i32;
            let n = steps.max(1);
            let mut at = [0.0; NATTR];
            for i in 0..n {
                let t = if steps > 0 {
                    i as f64 / steps as f64
                } else {
                    0.0
                };
                let px = (x0 + (x1 - x0) * t).round() as i32;
                let py = (y0 + (y1 - y0) * t).round() as i32;
                for k in 0..NATTR {
                    at[k] = a0[k] + (a1[k] - a0[k]) * t;
                }
                pixel_checked(m, d, rows, px, py, &frag(d, &at), linear);
            }
        }
        Shape::Sprite {
            px0,
            px1,
            x0,
            y0,
            a0,
            at,
            dsdx,
            dtdy,
            base,
        } => {
            let sp = SpriteRows {
                px0: *px0,
                px1: *px1,
                x0: *x0,
                y0: *y0,
                a0,
                q: at[Q],
                dsdx: *dsdx,
                dtdy: *dtdy,
                base,
            };
            match tex_mode(d, linear, dec) {
                TX_NONE => sprite_rows::<TX_NONE>(m, d, &sp, p, rows, decs),
                TX_NEAREST => sprite_rows::<TX_NEAREST>(m, d, &sp, p, rows, decs),
                TX_LINEAR => sprite_rows::<TX_LINEAR>(m, d, &sp, p, rows, decs),
                TX_NEAREST_DEC => sprite_rows::<TX_NEAREST_DEC>(m, d, &sp, p, rows, decs),
                TX_LINEAR_DEC => sprite_rows::<TX_LINEAR_DEC>(m, d, &sp, p, rows, decs),
                _ => sprite_rows::<TX_MIP>(m, d, &sp, p, rows, decs),
            }
        }
        Shape::Tri(t) => match tex_mode(d, linear, dec) {
            TX_NONE => tri_rows::<TX_NONE>(m, d, t, p, rows, decs),
            TX_NEAREST => tri_rows::<TX_NEAREST>(m, d, t, p, rows, decs),
            TX_LINEAR => tri_rows::<TX_LINEAR>(m, d, t, p, rows, decs),
            TX_NEAREST_DEC => tri_rows::<TX_NEAREST_DEC>(m, d, t, p, rows, decs),
            TX_LINEAR_DEC => tri_rows::<TX_LINEAR_DEC>(m, d, t, p, rows, decs),
            _ => tri_rows::<TX_MIP>(m, d, t, p, rows, decs),
        },
    }
}

struct SpriteRows<'a> {
    px0: i32,
    px1: i32,
    x0: i32,
    y0: i32,
    a0: &'a [f64; NATTR],
    q: f64,
    dsdx: f64,
    dtdy: f64,
    base: &'a Frag,
}

fn sprite_rows<const TX: u8>(
    m: Mem,
    d: &Draw,
    sp: &SpriteRows,
    p: &Prim,
    rows: Rows,
    decs: &[Option<Arc<Decoded>>],
) {
    rows.each(p.y0, p.y1, |py| {
        if !row_visible(d, py) {
            return;
        }
        let rc = RowCtx::new(d, py, decs);
        if TX != TX_NONE {
            let t = sp.a0[T] + sp.dtdy * ((py * 16 - sp.y0) as f64 / 16.0);
            for px in sp.px0..=sp.px1 {
                let s = sp.a0[S] + sp.dsdx * ((px * 16 - sp.x0) as f64 / 16.0);
                let frag = || {
                    let (u, v) = tex_uv(d, s, t, sp.q);
                    Frag {
                        u,
                        v,
                        q: sp.q as f32,
                        ..*sp.base
                    }
                };
                pixel_g::<TX>(m, d, &rc, px, sp.base.z, frag);
            }
        } else {
            for px in sp.px0..=sp.px1 {
                pixel_g::<TX>(m, d, &rc, px, sp.base.z, || *sp.base);
            }
        }
    });
}

/// First and last x offset (from bx0) where `e + dx * i >= 0`, for i >= 0.
#[inline(always)]
pub(crate) fn edge_span(e: i64, dx: i64, lo: &mut i64, hi: &mut i64) {
    if dx > 0 {
        if e < 0 {
            // smallest i with e + dx*i >= 0
            *lo = (*lo).max((-e + dx - 1) / dx);
        }
    } else if dx < 0 {
        if e < 0 {
            *hi = -1;
        } else {
            *hi = (*hi).min(e / -dx);
        }
    } else if e < 0 {
        *hi = -1;
    }
}

fn tri_rows<const TX: u8>(
    m: Mem,
    d: &Draw,
    t: &TriSetup,
    p: &Prim,
    rows: Rows,
    decs: &[Option<Arc<Decoded>>],
) {
    let tex = TX != TX_NONE;
    let want_f = d.fge;
    let want_z = d.zte;
    rows.each(p.y0, p.y1, |y| {
        if !row_visible(d, y) {
            return;
        }
        let dyp = (y - t.by0) as i64;
        let mut lo = 0i64;
        let mut hi = (t.bx1 - t.bx0) as i64;
        for k in 0..3 {
            edge_span(t.e0[k] + t.edy[k] * dyp, t.edx[k], &mut lo, &mut hi);
        }
        if lo > hi {
            return;
        }
        let dyf = y as f64 - t.oy;
        let ra: [f64; NATTR] = std::array::from_fn(|k| t.c[k] + t.dady[k] * dyf);
        let rc = RowCtx::new(d, y, decs);
        for i in lo..=hi {
            let x = t.bx0 + i as i32;
            let dxp = i as f64;
            let fz = if want_z { ra[Z] + t.dadx[Z] * dxp } else { 0.0 };
            let frag = || {
                let mut f = Frag {
                    r: (ra[R] + t.dadx[R] * dxp) as i32,
                    g: (ra[G] + t.dadx[G] * dxp) as i32,
                    b: (ra[B] + t.dadx[B] * dxp) as i32,
                    a: (ra[A] + t.dadx[A] * dxp) as i32,
                    ..Frag::default()
                };
                if tex {
                    let q = ra[Q] + t.dadx[Q] * dxp;
                    (f.u, f.v) = tex_uv(d, ra[S] + t.dadx[S] * dxp, ra[T] + t.dadx[T] * dxp, q);
                    f.q = q as f32;
                }
                if want_f {
                    f.f = (ra[F] + t.dadx[F] * dxp) as i32;
                }
                f
            };
            pixel_g::<TX>(m, d, &rc, x, fz, frag);
        }
    });
}

fn tex_trace() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var("SSX3_TRACE_TEX").is_ok())
}

/// Debug: SSX3_NO_MIPMAP=1 always samples level 0.
fn no_mipmap() -> bool {
    static E: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *E.get_or_init(|| std::env::var_os("SSX3_NO_MIPMAP").is_some())
}
