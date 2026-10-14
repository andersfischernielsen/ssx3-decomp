//! Sets of VRAM blocks (256 bytes; 16384 of them) with a page summary, used
//! by the batch scheduler to find read/write dependencies between primitives.

use super::raster::Pages;
use super::swizzle::Surface;

const WORDS: usize = 16384 / 64;

/// A set of 256-byte VRAM blocks. `pages` has a bit for every 8 KiB page
/// that may contain set blocks (32 blocks per page, i.e. half a word).
#[derive(Clone)]
pub(crate) struct Region {
    pub(crate) pages: Pages,
    blocks: Box<[u64; WORDS]>,
}

impl Default for Region {
    fn default() -> Self {
        Region {
            pages: Pages::default(),
            blocks: Box::new([0; WORDS]),
        }
    }
}

/// Iterate the set bits (page numbers) of a page set.
#[inline]
fn each_page(p: &Pages, mut f: impl FnMut(usize)) {
    for (i, &w) in p.0.iter().enumerate() {
        let mut w = w;
        while w != 0 {
            let b = w.trailing_zeros() as usize;
            f(i * 64 + b);
            w &= w - 1;
        }
    }
}

/// Bits of `blocks[page >> 1]` that belong to `page`.
#[inline]
fn half(page: usize) -> u64 {
    0xFFFF_FFFFu64 << ((page & 1) * 32)
}

impl Region {
    pub(crate) fn clear(&mut self) {
        let blocks = &mut self.blocks;
        each_page(&self.pages, |p| blocks[p >> 1] = 0);
        self.pages = Pages::default();
    }

    #[inline]
    fn add_block(&mut self, b: u32) {
        let b = (b & 16383) as usize;
        self.blocks[b >> 6] |= 1 << (b & 63);
        self.pages.add((b >> 5) as u32);
    }

    /// Add whole pages.
    pub(crate) fn add_pages(&mut self, p: &Pages) {
        let blocks = &mut self.blocks;
        each_page(p, |pg| blocks[pg >> 1] |= half(pg));
        self.pages.union(p);
    }

    /// Add the blocks holding pixels `x0..=x1` × `y0..=y1` of `s`.
    pub(crate) fn add_rect(&mut self, s: &Surface, x0: u32, y0: u32, x1: u32, y1: u32) {
        let (x0, y0) = (x0.min(0x7FF), y0.min(0x7FF));
        let (x1, y1) = (x1.min(0x7FF), y1.min(0x7FF));
        if x0 > x1 || y0 > y1 {
            return;
        }
        // Block size in pixels: 8x8 (32-bit), 16x8 (16), 16x16 (8), 32x16 (4).
        let (bws, bhs) = [(3, 3), (4, 3), (4, 4), (5, 4)][s.es as usize & 3];
        let (pws, phs) = (s.pw_shift, s.ph_shift);
        let (pc0, pc1) = (x0 >> pws, x1 >> pws);
        let (pr0, pr1) = (y0 >> phs, y1 >> phs);
        if (pc1 - pc0 + 1) * (pr1 - pr0 + 1) > 512 {
            self.add_pages(&Pages::ALL);
            return;
        }
        // Block of page-local pixel (x, y): the page's first block plus the
        // block-in-page from the swizzle table; every pixel of an aligned
        // block cell lives in the same block.
        let bshift = 6 + s.es;
        let (pw, ph) = (1u32 << pws, 1u32 << phs);
        for pr in pr0..=pr1 {
            for pc in pc0..=pc1 {
                let first = (s.base >> 6).wrapping_add((pc + pr * s.pages_per_row) * 32);
                let (lx0, lx1) = (x0.max(pc << pws) - (pc << pws), x1.min(((pc + 1) << pws) - 1) - (pc << pws));
                let (ly0, ly1) = (y0.max(pr << phs) - (pr << phs), y1.min(((pr + 1) << phs) - 1) - (pr << phs));
                if lx0 == 0 && ly0 == 0 && lx1 == pw - 1 && ly1 == ph - 1 {
                    self.add_page_blocks(first);
                    continue;
                }
                for cy in (ly0 >> bhs)..=(ly1 >> bhs) {
                    let row = ((cy << bhs) << pws) as usize;
                    for cx in (lx0 >> bws)..=(lx1 >> bws) {
                        let bip = s.table[row | (cx << bws) as usize] as u32 >> bshift;
                        self.add_block(first.wrapping_add(bip));
                    }
                }
            }
        }
    }

    /// Add the 32 blocks of a page starting at block `first`.
    fn add_page_blocks(&mut self, first: u32) {
        if first.is_multiple_of(32) {
            let p = ((first >> 5) & 511) as usize;
            self.blocks[p >> 1] |= half(p);
            self.pages.add(p as u32);
        } else {
            for b in first..first + 32 {
                self.add_block(b);
            }
        }
    }

    #[inline]
    fn has_block(&self, b: u32) -> bool {
        let b = (b & 16383) as usize;
        self.blocks[b >> 6] & (1 << (b & 63)) != 0
    }

    /// One past the last row of the `x0..=x1` × `y0..=y1` rectangle of `s`
    /// whose block is in this set (`y0` if none is).
    pub(crate) fn rows_hit_end(&self, s: &Surface, x0: u32, y0: u32, x1: u32, y1: u32) -> u32 {
        let (bws, bhs) = [(3, 3), (4, 3), (4, 4), (5, 4)][s.es as usize & 3];
        let (c0, c1) = (x0 >> bws, x1.min(0x7FF) >> bws);
        let (r0, r1) = (y0 >> bhs, y1.min(0x7FF) >> bhs);
        for r in (r0..=r1).rev() {
            for c in c0..=c1 {
                let e = s.elem(c << bws, r << bhs);
                if self.has_block((e >> s.es) >> 6) {
                    return (((r + 1) << bhs).min(y1 + 1)).max(y0);
                }
            }
        }
        y0
    }

    /// Pages all of whose blocks are in the set.
    pub(crate) fn full_pages(&self) -> Pages {
        let mut full = Pages::default();
        each_page(&self.pages, |p| {
            if self.blocks[p >> 1] & half(p) == half(p) {
                full.add(p as u32);
            }
        });
        full
    }

    pub(crate) fn union(&mut self, o: &Region) {
        let blocks = &mut self.blocks;
        each_page(&o.pages, |p| blocks[p >> 1] |= o.blocks[p >> 1] & half(p));
        self.pages.union(&o.pages);
    }

    /// Whether the two sets share a block.
    pub(crate) fn hits(&self, o: &Region) -> bool {
        let mut common = Pages::default();
        for i in 0..8 {
            common.0[i] = self.pages.0[i] & o.pages.0[i];
        }
        let mut hit = false;
        each_page(&common, |p| {
            hit |= self.blocks[p >> 1] & o.blocks[p >> 1] & half(p) != 0;
        });
        hit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gs::swizzle::*;

    #[test]
    fn rect_blocks_cover_exactly_the_written_words() {
        let cases = [(PSMCT32, 10), (PSMCT16, 4), (PSMT8, 2), (PSMT4, 2), (PSMZ24, 8)];
        for (&(psm, bw), bp) in cases.iter().flat_map(|c| [(c, 96u32), (c, 4096)]) {
            let s = Surface::new(psm, bp, bw);
            for (x0, y0, x1, y1) in [(13, 7, 77, 41), (0, 0, 255, 127), (5, 0, 300, 200)] {
                let mut r = Region::default();
                r.add_rect(&s, x0, y0, x1, y1);
                let mut want = Region::default();
                for y in y0..=y1 {
                    for x in x0..=x1 {
                        want.add_block((s.elem(x, y) >> s.es) >> 6);
                    }
                }
                assert!(r.pages == want.pages && r.blocks == want.blocks, "psm {psm:#x} bp {bp}");
            }
            let mut r = Region::default();
            r.add_rect(&s, 13, 7, 77, 41);
            let mut want = Region::default();
            for y in 7..=41 {
                for x in 13..=77 {
                    want.add_block((s.elem(x, y) >> s.es) >> 6);
                }
            }
            let mut other = Region::default();
            other.add_rect(&s, 20, 300, 30, 310);
            assert!(r.hits(&want) && !r.hits(&other));
        }
    }
}
