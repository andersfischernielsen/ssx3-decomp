//! TexMan textures decoded from EE memory (`docs/investigation/structs/render.md`
//! TexMan / TexInfo; upload layout from `TexMan_EmitImageUpload` 0x368170 and
//! `TexMan_EmitClutUpload` 0x3684f0).
//!
//! The game uploads each texture level from `TexInfo.pixels` as a linear
//! image, usually in the texture's own PSM, so a texel is simply the next
//! element of the pixel data. Two formats are pre-swizzled in RAM and
//! uploaded as PSMCT32 instead (10: 8-bit, 0xc: 4-bit), then read as T8/T4;
//! those go through the GS swizzle model. CLUTs are uploaded as a 16×16
//! (8-bit) or 8×2 (4-bit) PSMCT32 image and read with CSM1, which on a linear
//! upload swaps index bits 3 and 4 for 256 entries.
//!
//! Texels come out as RGBA8 with the GS's alpha (0x80 = 1.0).

use ssx3_ps2::gs::swizzle::{self, Surface};
use ssx3_render::chain::Mem;
use ssx3_render::frame::word;

/// TexMan field offsets.
pub mod tm {
    /// `TexInfo *tex[2000]` in TexMan.
    pub const TABLE: u32 = 0x8;
    pub const MAX: u32 = 2000;
}

/// TexInfo field offsets (0x58 bytes).
pub mod ti {
    pub const FORMAT: u32 = 0x0c;
    pub const BPP: u32 = 0x18;
    pub const PIXELS: u32 = 0x1c;
    pub const CLUT: u32 = 0x20;
    pub const MIPS: u32 = 0x24;
    pub const TBP: u32 = 0x28;
    pub const TEX0: u32 = 0x38;
    pub const TEX1: u32 = 0x40;
}

/// What identifies a decoded texture's contents.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TexKey {
    pub info: u32,
    pub pixels: u32,
    pub clut: u32,
    pub format: u32,
    /// TEX0 without TBP/CBP (VRAM placement changes per frame for
    /// streamed textures).
    pub tex0: u64,
    pub texa: u64,
    /// Sampled contents (pixels and CLUT).
    pub hash: u64,
}

pub struct Decoded {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u32>,
}

fn byte(m: &dyn Mem, a: u32) -> u8 {
    (m.qword(a & !15) >> (8 * (a & 15))) as u8
}

fn read_bytes(m: &dyn Mem, a: u32, n: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(n);
    let mut q = a & !15;
    let mut skip = (a & 15) as usize;
    while out.len() < n {
        let v = m.qword(q).to_le_bytes();
        for b in &v[skip..] {
            if out.len() == n {
                break;
            }
            out.push(*b);
        }
        skip = 0;
        q += 16;
    }
    out
}

fn dword(m: &dyn Mem, a: u32) -> u64 {
    word(m, a) as u64 | (word(m, a + 4) as u64) << 32
}

/// The TexInfo pointer of TexMan id `id` (0: none).
pub fn info(m: &dyn Mem, texman: u32, id: u16) -> u32 {
    if (id as u32) >= tm::MAX {
        return 0;
    }
    word(m, texman + tm::TABLE + 4 * id as u32)
}

pub fn tex0(m: &dyn Mem, info: u32) -> u64 {
    dword(m, info + ti::TEX0)
}

/// Cheap content fingerprint: 32 evenly spaced words of the level-0 pixels
/// and the whole CLUT.
fn fingerprint(m: &dyn Mem, pixels: u32, bytes: u32, clut: u32, clut_bytes: u32) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut mix = |v: u32| {
        h ^= v as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
    };
    if bytes >= 4 {
        for i in 0..32u32 {
            mix(word(m, (pixels + (bytes / 32 * i)) & !3));
        }
    }
    for i in (0..clut_bytes).step_by(4) {
        mix(word(m, clut + i));
    }
    h
}

/// [`key`], reused while the memory it reads is not written
/// (`Mem::unchanged`; live snapshots only): per (TexInfo, TEXA), the key,
/// the bytes of pixels and CLUT it covers, the snapshot it was made in.
#[derive(Default)]
pub struct KeyCache(std::collections::HashMap<(u32, u64), (TexKey, u32, u32, u64)>);

impl KeyCache {
    pub fn key(&mut self, m: &dyn Mem, info: u32, texa: u64) -> TexKey {
        let snap = m.snapshot();
        if snap != 0 {
            if let Some(&(k, bytes, clut_bytes, seen)) = self.0.get(&(info, texa)) {
                let same = m.unchanged(info, info + ti::TEX1, seen)
                    && m.unchanged(k.pixels, k.pixels + bytes.max(4), seen)
                    && (clut_bytes == 0 || m.unchanged(k.clut, k.clut + clut_bytes, seen));
                if same {
                    return k;
                }
            }
        }
        let k = key(m, info, texa);
        if snap != 0 {
            if self.0.len() > 8192 {
                self.0.clear();
            }
            let (bytes, clut_bytes) = sizes(m, info);
            self.0.insert((info, texa), (k, bytes, clut_bytes, snap));
        }
        k
    }
}

/// The bytes of a texture's level-0 pixels and of its CLUT.
fn sizes(m: &dyn Mem, info: u32) -> (u32, u32) {
    let t0 = tex0(m, info);
    let (w, h) = (1u32 << ((t0 >> 26) & 15), 1u32 << ((t0 >> 30) & 15));
    let bpp = word(m, info + ti::BPP).clamp(4, 32);
    let clut_bytes = match word(m, info + ti::FORMAT) {
        0xb | 0xc => 0x40,
        9 | 10 => 0x400,
        _ => 0,
    };
    (w * h * bpp / 8, clut_bytes)
}

pub fn key(m: &dyn Mem, info: u32, texa: u64) -> TexKey {
    let t0 = tex0(m, info);
    let format = word(m, info + ti::FORMAT);
    let pixels = word(m, info + ti::PIXELS);
    let clut = word(m, info + ti::CLUT);
    let (w, h) = (1u32 << ((t0 >> 26) & 15), 1u32 << ((t0 >> 30) & 15));
    let bpp = word(m, info + ti::BPP).clamp(4, 32);
    let clut_bytes = match format {
        0xb | 0xc => 0x40,
        9 | 10 => 0x400,
        _ => 0,
    };
    let hash = fingerprint(m, pixels, w * h * bpp / 8, clut, clut_bytes);
    TexKey { info, pixels, clut, format, tex0: t0 & !((0x3fff) | (0x3fffu64 << 37)), texa, hash }
}

/// TEXA expansion of a 16-bit texel (RGBA5551).
fn ct16(v: u32, texa: u64) -> u32 {
    let (ta0, ta1, aem) = ((texa & 0xff) as u32, ((texa >> 32) & 0xff) as u32, (texa >> 15) & 1 != 0);
    let r = (v & 0x1f) << 3;
    let g = ((v >> 5) & 0x1f) << 3;
    let b = ((v >> 10) & 0x1f) << 3;
    let a = if v & 0x8000 != 0 {
        ta1
    } else if aem && v & 0x7fff == 0 {
        0
    } else {
        ta0
    };
    r | g << 8 | b << 16 | a << 24
}

/// Decode level 0 of the texture `info` (TEX0 from the TexInfo).
pub fn decode(m: &dyn Mem, info: u32, texa: u64) -> Option<Decoded> {
    let t0 = tex0(m, info);
    let format = word(m, info + ti::FORMAT);
    let pixels = word(m, info + ti::PIXELS);
    let clut_ptr = word(m, info + ti::CLUT);
    let psm = ((t0 >> 20) & 0x3f) as u32;
    let tbw = (((t0 >> 14) & 0x3f) as u32).max(1);
    let (w, h) = (1u32 << ((t0 >> 26) & 15), 1u32 << ((t0 >> 30) & 15));
    if pixels == 0 || w > 1024 || h > 1024 {
        return None;
    }
    let n = (w * h) as usize;
    let indexed = matches!(psm, swizzle::PSMT8 | swizzle::PSMT8H | swizzle::PSMT4 | swizzle::PSMT4HL | swizzle::PSMT4HH);
    // CLUT (always uploaded as CT32 by TexMan; CPSM 16-bit handled too).
    let cpsm = ((t0 >> 51) & 0xf) as u32;
    let clut: Vec<u32> = if indexed {
        let entries = if matches!(psm, swizzle::PSMT8 | swizzle::PSMT8H) { 256 } else { 16 };
        let raw = read_bytes(m, clut_ptr, entries * 4);
        let ent = |i: usize| u32::from_le_bytes([raw[4 * i], raw[4 * i + 1], raw[4 * i + 2], raw[4 * i + 3]]);
        (0..entries)
            .map(|i| {
                // CSM1: entries 8-15 and 16-23 of each 32 are swapped.
                let j = if entries == 256 { (i & !0x18) | (i & 0x8) << 1 | (i & 0x10) >> 1 } else { i };
                let v = ent(j);
                if cpsm == swizzle::PSMCT16 || cpsm == swizzle::PSMCT16S {
                    ct16(v & 0xffff, texa)
                } else {
                    v
                }
            })
            .collect()
    } else {
        Vec::new()
    };
    let rgba: Vec<u32> = match format {
        10 | 0xc => {
            // Pre-swizzled: written as PSMCT32 (width and height as in
            // TexMan_EmitImageUpload), read as T8/T4.
            let (uw, uh) = if format == 10 {
                (w / 2, h / 2)
            } else if w != h {
                (w / 4, h / 2)
            } else {
                (h / 2, w / 4)
            };
            let (uw, uh) = (uw.max(1), uh.max(1));
            let raw = read_bytes(m, pixels, (uw * uh * 4) as usize);
            // Scratch VRAM big enough for the texture (4 MiB words max).
            let pages = ((w * h * if format == 10 { 8 } else { 4 } / 8) / 8192 + 2) as usize;
            let mut vram = vec![0u32; (pages * 2048).max(4096) * 2];
            let wsurf = Surface::new(swizzle::PSMCT32, 0, (uw + 63) / 64);
            for y in 0..uh {
                for x in 0..uw {
                    let i = ((y * uw + x) * 4) as usize;
                    wsurf.write(&mut vram, x, y, u32::from_le_bytes([raw[i], raw[i + 1], raw[i + 2], raw[i + 3]]));
                }
            }
            let rsurf = Surface::new(psm, 0, tbw);
            (0..n)
                .map(|i| {
                    let (x, y) = (i as u32 % w, i as u32 / w);
                    clut.get(rsurf.read(&vram, x, y) as usize).copied().unwrap_or(0)
                })
                .collect()
        }
        _ => match psm {
            swizzle::PSMT8 | swizzle::PSMT8H => {
                let raw = read_bytes(m, pixels, n);
                raw.iter().map(|&i| clut[i as usize]).collect()
            }
            swizzle::PSMT4 | swizzle::PSMT4HL | swizzle::PSMT4HH => {
                let raw = read_bytes(m, pixels, n.div_ceil(2));
                (0..n).map(|i| clut[((raw[i / 2] >> (4 * (i & 1))) & 15) as usize]).collect()
            }
            swizzle::PSMCT16 | swizzle::PSMCT16S => {
                let raw = read_bytes(m, pixels, n * 2);
                (0..n).map(|i| ct16(u16::from_le_bytes([raw[2 * i], raw[2 * i + 1]]) as u32, texa)).collect()
            }
            swizzle::PSMCT24 => {
                let ta0 = (texa & 0xff) as u32;
                let raw = read_bytes(m, pixels, n * 3);
                (0..n).map(|i| raw[3 * i] as u32 | (raw[3 * i + 1] as u32) << 8 | (raw[3 * i + 2] as u32) << 16 | ta0 << 24).collect()
            }
            _ => {
                let raw = read_bytes(m, pixels, n * 4);
                (0..n).map(|i| u32::from_le_bytes([raw[4 * i], raw[4 * i + 1], raw[4 * i + 2], raw[4 * i + 3]])).collect()
            }
        },
    };
    let _ = byte;
    Some(Decoded { width: w, height: h, rgba })
}
