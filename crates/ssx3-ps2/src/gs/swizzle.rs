//! GS local memory layout: pixel storage formats, page/block/column swizzles.
//!
//! The block and column tables (and the PSMT8/PSMT4 column-word formulas) are
//! ported from `ps2-web-recomp` (runtime/src/gs/gs.cpp, MIT licensed,
//! https://github.com/snuri00/ps2-web-recomp), which in turn matches the
//! well-known PCSX2 GSdx `blockTable*` / `columnTable*` layouts.
//!
//! Local memory is 4 MiB, modelled as 1M little-endian 32-bit words. A page is
//! 8 KiB (2048 words), a block 256 bytes (64 words). Every format addresses
//! "elements" (32-, 16-, 8- or 4-bit) inside a page through a precomputed table
//! that maps (x mod page-width, y mod page-height) to an element index.

use std::sync::OnceLock;

pub const PSMCT32: u32 = 0x00;
pub const PSMCT24: u32 = 0x01;
pub const PSMCT16: u32 = 0x02;
pub const PSMCT16S: u32 = 0x0A;
pub const PSMT8: u32 = 0x13;
pub const PSMT4: u32 = 0x14;
pub const PSMT8H: u32 = 0x1B;
pub const PSMT4HL: u32 = 0x24;
pub const PSMT4HH: u32 = 0x2C;
pub const PSMZ32: u32 = 0x30;
pub const PSMZ24: u32 = 0x31;
pub const PSMZ16: u32 = 0x32;
pub const PSMZ16S: u32 = 0x3A;

/// Number of 32-bit words in GS local memory.
pub const VRAM_WORDS: usize = 1 << 20;

const BLOCK32: [[u32; 8]; 4] = [
    [0, 1, 4, 5, 16, 17, 20, 21],
    [2, 3, 6, 7, 18, 19, 22, 23],
    [8, 9, 12, 13, 24, 25, 28, 29],
    [10, 11, 14, 15, 26, 27, 30, 31],
];
const BLOCKZ32: [[u32; 8]; 4] = [
    [24, 25, 28, 29, 8, 9, 12, 13],
    [26, 27, 30, 31, 10, 11, 14, 15],
    [16, 17, 20, 21, 0, 1, 4, 5],
    [18, 19, 22, 23, 2, 3, 6, 7],
];
const COLUMN32: [[u32; 8]; 2] = [[0, 1, 4, 5, 8, 9, 12, 13], [2, 3, 6, 7, 10, 11, 14, 15]];
const BLOCK16: [[u32; 4]; 8] = [
    [0, 2, 8, 10],
    [1, 3, 9, 11],
    [4, 6, 12, 14],
    [5, 7, 13, 15],
    [16, 18, 24, 26],
    [17, 19, 25, 27],
    [20, 22, 28, 30],
    [21, 23, 29, 31],
];
const BLOCK16S: [[u32; 4]; 8] = [
    [0, 2, 16, 18],
    [1, 3, 17, 19],
    [8, 10, 24, 26],
    [9, 11, 25, 27],
    [4, 6, 20, 22],
    [5, 7, 21, 23],
    [12, 14, 28, 30],
    [13, 15, 29, 31],
];
const BLOCKZ16: [[u32; 4]; 8] = [
    [24, 26, 16, 18],
    [25, 27, 17, 19],
    [28, 30, 20, 22],
    [29, 31, 21, 23],
    [8, 10, 0, 2],
    [9, 11, 1, 3],
    [12, 14, 4, 6],
    [13, 15, 5, 7],
];
const BLOCKZ16S: [[u32; 4]; 8] = [
    [24, 26, 8, 10],
    [25, 27, 9, 11],
    [16, 18, 0, 2],
    [17, 19, 1, 3],
    [28, 30, 12, 14],
    [29, 31, 13, 15],
    [20, 22, 4, 6],
    [21, 23, 5, 7],
];
const COLUMN16: [[u32; 16]; 2] = [
    [0, 2, 8, 10, 16, 18, 24, 26, 1, 3, 9, 11, 17, 19, 25, 27],
    [4, 6, 12, 14, 20, 22, 28, 30, 5, 7, 13, 15, 21, 23, 29, 31],
];
const COLUMN_WORD: [[[u32; 8]; 2]; 2] = [
    [[0, 1, 4, 5, 8, 9, 12, 13], [2, 3, 6, 7, 10, 11, 14, 15]],
    [[8, 9, 12, 13, 0, 1, 4, 5], [10, 11, 14, 15, 2, 3, 6, 7]],
];

/// Per-page element tables, indexed `[y * page_width + x]`.
pub struct Tables {
    pub t32: Vec<u16>,
    pub z32: Vec<u16>,
    pub t16: Vec<u16>,
    pub t16s: Vec<u16>,
    pub z16: Vec<u16>,
    pub z16s: Vec<u16>,
    pub t8: Vec<u16>,
    pub t4: Vec<u16>,
}

fn build() -> Tables {
    let mut t = Tables {
        t32: vec![0; 32 * 64],
        z32: vec![0; 32 * 64],
        t16: vec![0; 64 * 64],
        t16s: vec![0; 64 * 64],
        z16: vec![0; 64 * 64],
        z16s: vec![0; 64 * 64],
        t8: vec![0; 64 * 128],
        t4: vec![0; 128 * 128],
    };
    // 32-bit: page 64x32, block 8x8, word index.
    for y in 0..32usize {
        for x in 0..64usize {
            let inner = ((y & 7) / 2) as u32 * 16 + COLUMN32[y & 1][x & 7];
            t.t32[y * 64 + x] = (BLOCK32[y / 8][x / 8] * 64 + inner) as u16;
            t.z32[y * 64 + x] = (BLOCKZ32[y / 8][x / 8] * 64 + inner) as u16;
        }
    }
    // 16-bit: page 64x64, block 16x8, halfword index.
    for y in 0..64usize {
        for x in 0..64usize {
            let inner = ((y & 7) / 2) as u32 * 32 + COLUMN16[y & 1][x & 15];
            let i = y * 64 + x;
            t.t16[i] = (BLOCK16[y / 8][x / 16] * 128 + inner) as u16;
            t.t16s[i] = (BLOCK16S[y / 8][x / 16] * 128 + inner) as u16;
            t.z16[i] = (BLOCKZ16[y / 8][x / 16] * 128 + inner) as u16;
            t.z16s[i] = (BLOCKZ16S[y / 8][x / 16] * 128 + inner) as u16;
        }
    }
    // 8-bit: page 128x64, block 16x16, byte index.
    for y in 0..64usize {
        for x in 0..128usize {
            let block = BLOCK32[(y / 16) & 3][(x / 16) & 7];
            let wx = x % 16;
            let wy16 = y % 16;
            let column = (wy16 / 4) as u32;
            let wy = wy16 % 4;
            let mut table = (wy & 2) >> 1;
            let byte = (((wx & 8) >> 2) + ((wy & 2) >> 1)) as u32;
            table ^= (y / 4) & 1;
            let word = block * 64 + column * 16 + COLUMN_WORD[table][wy & 1][wx & 7];
            t.t8[y * 128 + x] = (word * 4 + byte) as u16;
        }
    }
    // 4-bit: page 128x128, block 32x16, nibble index.
    for y in 0..128usize {
        for x in 0..128usize {
            let block = BLOCK16[(y / 16) & 7][(x / 32) & 3];
            let wx = x % 32;
            let wy16 = y % 16;
            let column = wy16 / 4;
            let wy = wy16 % 4;
            let shift = (wx & 0x18) + ((wy & 2) << 1);
            let nibble = (shift / 4) as u32;
            let mut table = (wy & 2) >> 1;
            table ^= column & 1;
            let word = block * 64 + column as u32 * 16 + COLUMN_WORD[table][wy & 1][wx & 7];
            t.t4[y * 128 + x] = (word * 8 + nibble) as u16;
        }
    }
    t
}

pub fn tables() -> &'static Tables {
    static T: OnceLock<Tables> = OnceLock::new();
    T.get_or_init(build)
}

/// How a storage format packs pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Full 32-bit word.
    W32,
    /// Low 24 bits of a 32-bit word (upper byte preserved on write).
    W24,
    /// 16-bit halfword.
    H16,
    /// 8-bit byte.
    B8,
    /// 4-bit nibble.
    N4,
    /// Bits 31:24 of a 32-bit word.
    Hi8,
    /// Bits 27:24 of a 32-bit word.
    Hl4,
    /// Bits 31:28 of a 32-bit word.
    Hh4,
}

/// A buffer in local memory: base pointer (in 64-word blocks), width (in
/// 64-pixel units) and format, with its addressing precomputed.
#[derive(Clone, Copy)]
pub struct Surface {
    pub table: &'static [u16],
    pub pw_shift: u32,
    pub ph_shift: u32,
    /// log2(elements per 32-bit word).
    pub es: u32,
    pub kind: Kind,
    pub base: u32,
    pub pages_per_row: u32,
    pub psm: u32,
}

impl Surface {
    pub fn new(psm: u32, bp: u32, bw: u32) -> Surface {
        let t = tables();
        let (table, pw_shift, ph_shift, es, kind): (&'static [u16], u32, u32, u32, Kind) = match psm
        {
            PSMCT32 => (&t.t32, 6, 5, 0, Kind::W32),
            PSMCT24 => (&t.t32, 6, 5, 0, Kind::W24),
            PSMZ32 => (&t.z32, 6, 5, 0, Kind::W32),
            PSMZ24 => (&t.z32, 6, 5, 0, Kind::W24),
            PSMCT16 => (&t.t16, 6, 6, 1, Kind::H16),
            PSMCT16S => (&t.t16s, 6, 6, 1, Kind::H16),
            PSMZ16 => (&t.z16, 6, 6, 1, Kind::H16),
            PSMZ16S => (&t.z16s, 6, 6, 1, Kind::H16),
            PSMT8 => (&t.t8, 7, 6, 2, Kind::B8),
            PSMT4 => (&t.t4, 7, 7, 3, Kind::N4),
            PSMT8H => (&t.t32, 6, 5, 0, Kind::Hi8),
            PSMT4HL => (&t.t32, 6, 5, 0, Kind::Hl4),
            PSMT4HH => (&t.t32, 6, 5, 0, Kind::Hh4),
            _ => (&t.t32, 6, 5, 0, Kind::W32),
        };
        Surface {
            table,
            pw_shift,
            ph_shift,
            es,
            kind,
            base: (bp & 0x3FFF) * 64,
            pages_per_row: ((bw * 64) >> pw_shift).max(1),
            psm,
        }
    }

    /// Global element index (in units of this format's element size).
    #[inline]
    pub fn elem(&self, x: u32, y: u32) -> u32 {
        let x = x & 0x7FF;
        let y = y & 0x7FF;
        let page = (x >> self.pw_shift) + (y >> self.ph_shift) * self.pages_per_row;
        let pw_mask = (1 << self.pw_shift) - 1;
        let ph_mask = (1 << self.ph_shift) - 1;
        let off = self.table[(((y & ph_mask) << self.pw_shift) | (x & pw_mask)) as usize] as u32;
        let word_base = self.base.wrapping_add(page.wrapping_mul(2048));
        (word_base << self.es).wrapping_add(off) & ((VRAM_WORDS as u32) << self.es).wrapping_sub(1)
    }

    /// Word index and bit shift of pixel (x, y).
    #[inline]
    pub fn locate(&self, x: u32, y: u32) -> (usize, u32) {
        let e = self.elem(x, y);
        let word = (e >> self.es) as usize;
        let shift = (e & ((1 << self.es) - 1)) << (5 - self.es);
        (word, shift)
    }

    #[inline]
    pub fn read(&self, vram: &[u32], x: u32, y: u32) -> u32 {
        let (w, s) = self.locate(x, y);
        let v = vram[w];
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

    #[inline]
    pub fn write(&self, vram: &mut [u32], x: u32, y: u32, value: u32) {
        let (w, s) = self.locate(x, y);
        let slot = &mut vram[w];
        *slot = match self.kind {
            Kind::W32 => value,
            Kind::W24 => (*slot & 0xFF00_0000) | (value & 0x00FF_FFFF),
            Kind::H16 => (*slot & !(0xFFFF << s)) | ((value & 0xFFFF) << s),
            Kind::B8 => (*slot & !(0xFF << s)) | ((value & 0xFF) << s),
            Kind::N4 => (*slot & !(0xF << s)) | ((value & 0xF) << s),
            Kind::Hi8 => (*slot & 0x00FF_FFFF) | (value << 24),
            Kind::Hl4 => (*slot & 0xF0FF_FFFF) | ((value & 0xF) << 24),
            Kind::Hh4 => (*slot & 0x0FFF_FFFF) | ((value & 0xF) << 28),
        };
    }

    /// Write with a bit mask: bits set in `keep` retain their old value.
    #[inline]
    pub fn write_masked(&self, vram: &mut [u32], x: u32, y: u32, value: u32, keep: u32) {
        if keep == 0 {
            self.write(vram, x, y, value);
        } else {
            let old = self.read(vram, x, y);
            self.write(vram, x, y, (value & !keep) | (old & keep));
        }
    }
}

/// Bits per pixel of a storage format as seen by host transfers.
pub fn bits_per_pixel(psm: u32) -> u32 {
    match psm {
        PSMCT24 | PSMZ24 => 24,
        PSMCT16 | PSMCT16S | PSMZ16 | PSMZ16S => 16,
        PSMT8 | PSMT8H => 8,
        PSMT4 | PSMT4HL | PSMT4HH => 4,
        _ => 32,
    }
}

/// Read one pixel (convenience, unoptimised).
pub fn read_pixel(vram: &[u32], psm: u32, bp: u32, bw: u32, x: u32, y: u32) -> u32 {
    Surface::new(psm, bp, bw).read(vram, x, y)
}

/// Write one pixel (convenience, unoptimised).
pub fn write_pixel(vram: &mut [u32], psm: u32, bp: u32, bw: u32, x: u32, y: u32, v: u32) {
    Surface::new(psm, bp, bw).write(vram, x, y, v)
}
