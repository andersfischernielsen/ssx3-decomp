//! Cache of decoded textures (`raster::Decoded`).
//!
//! Sampling swizzled, palettized texels is most of the rasterizer's work, so
//! textures are decoded to plain RGBA8 once and reused while their VRAM is
//! unchanged. Validity is tracked with a write generation per 8 KiB page:
//! every GS write to VRAM (drawing, host->local and local->local transfers)
//! bumps the generation of the pages it touches, and an entry is only used
//! while none of its pages has been written since it was decoded.
//!
//! Code that modifies `Gs::vram` directly must call `Gs::vram_written`.

use std::collections::HashMap;
use std::sync::Arc;

use super::raster::{self, Decoded, Draw, Pages, TexKey};

/// Total decoded bytes kept before the cache is emptied.
const MAX_BYTES: usize = 96 << 20;

struct Entry {
    pages: Pages,
    stamp: u64,
    dec: Arc<Decoded>,
}

pub(crate) struct TexCache {
    entries: HashMap<TexKey, Entry>,
    page_gen: Box<[u64; 512]>,
    gen: u64,
    /// Pages of an unfinished host->local transfer: never valid.
    pub(crate) live: Pages,
    bytes: usize,
    /// Hits, decodes, decoded texels.
    pub(crate) stats: (u64, u64, u64),
}

impl Default for TexCache {
    fn default() -> Self {
        TexCache {
            entries: HashMap::new(),
            page_gen: Box::new([0; 512]),
            gen: 0,
            live: Pages::default(),
            bytes: 0,
            stats: (0, 0, 0),
        }
    }
}

fn each_page(p: &Pages, mut f: impl FnMut(usize)) {
    for (i, &w) in p.0.iter().enumerate() {
        let mut w = w;
        while w != 0 {
            f(i * 64 + w.trailing_zeros() as usize);
            w &= w - 1;
        }
    }
}

impl TexCache {
    /// Record a write to `pages`.
    pub(crate) fn written(&mut self, pages: &Pages) {
        self.gen += 1;
        let (gen, pg) = (self.gen, &mut self.page_gen);
        each_page(pages, |p| pg[p] = gen);
    }

    fn valid(&self, e: &Entry) -> bool {
        if e.pages.hits(&self.live) {
            return false;
        }
        let mut ok = true;
        each_page(&e.pages, |p| ok &= self.page_gen[p] <= e.stamp);
        ok
    }

    /// Decoded texels for `d`'s texture as VRAM is now, if it is decodable.
    /// A texture not cached yet is only decoded if it has at most `budget`
    /// texels (decoding costs about as much as sampling each texel once).
    pub(crate) fn get(&mut self, vram: &[u32], d: &Draw, budget: u64) -> Option<Arc<Decoded>> {
        let rect = raster::decode_rect(d)?;
        let key = raster::tex_key(d, rect);
        if let Some(e) = self.entries.get(&key) {
            if self.valid(e) {
                self.stats.0 += 1;
                return Some(e.dec.clone());
            }
        }
        let area = (rect[2] - rect[0] + 1) as u64 * (rect[3] - rect[1] + 1) as u64;
        if area > budget {
            return None;
        }
        let ts = d.tex_surface().expect("textured draw");
        let pages = Pages::rect(ts, rect[0], rect[1], rect[2], rect[3]);
        if pages.hits(&self.live) {
            return None;
        }
        let dec = Arc::new(raster::decode(vram, d, rect));
        self.stats.1 += 1;
        self.stats.2 += (dec.bytes() / 4) as u64;
        if self.bytes + dec.bytes() > MAX_BYTES {
            self.entries.clear();
            self.bytes = 0;
        }
        self.bytes += dec.bytes();
        let e = Entry {
            pages,
            stamp: self.gen,
            dec: dec.clone(),
        };
        if let Some(old) = self.entries.insert(key, e) {
            self.bytes -= old.dec.bytes();
        }
        Some(dec)
    }
}
