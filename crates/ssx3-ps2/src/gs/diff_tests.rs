//! Differential tests: the batched, multithreaded, specialized rasterizer
//! against the original reference implementation (`ref_raster`), on random
//! register states, primitives and VRAM contents. VRAM must match bit for bit.

use super::swizzle::*;
use super::{ref_raster, Gs, Vertex};

/// xorshift64*
pub(super) struct Rng(pub(super) u64);

impl Rng {
    pub(super) fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    pub(super) fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    pub(super) fn pick<T: Copy>(&mut self, v: &[T]) -> T {
        v[self.below(v.len() as u64) as usize]
    }
    pub(super) fn chance(&mut self, pct: u64) -> bool {
        self.below(100) < pct
    }
    pub(super) fn f32_in(&mut self, lo: f32, hi: f32) -> f32 {
        lo + (hi - lo) * (self.below(1 << 24) as f32 / (1 << 24) as f32)
    }
}

const FB_PSMS: &[u32] = &[PSMCT32, PSMCT32, PSMCT32, PSMCT24, PSMCT16, PSMCT16S, PSMZ32];
const TEX_PSMS: &[u32] = &[
    PSMCT32, PSMCT24, PSMCT16, PSMCT16S, PSMT8, PSMT8, PSMT4, PSMT4, PSMT8H, PSMT4HL, PSMT4HH,
    PSMZ32, PSMZ16,
];

pub(super) fn random_state(r: &mut Rng, gs: &mut Gs, fbp: u64, fbw: u64) {
    let ctx = 0u8; // context 1 registers; PRIM.CTXT picks
    for c in 0..2u8 {
        let fpsm = r.pick(FB_PSMS) as u64;
        let fbmsk = match r.below(4) {
            0 => r.next() & 0xFFFF_FFFF,
            1 => 0xFF00_0000,
            _ => 0,
        };
        gs.write_reg(0x4C + c, fbp | fbw << 16 | fpsm << 24 | fbmsk << 32);
        let zbp = r.pick(&[fbp + 70, 300, 448, fbp]);
        let zpsm = r.below(4) | if r.chance(20) { 8 } else { 0 }; // incl. Z16S (0xA)
        gs.write_reg(0x4E + c, zbp | zpsm << 24 | (r.chance(40) as u64) << 32);
        // Texture: often somewhere else, sometimes the frame buffer itself.
        let tbp = match r.below(5) {
            0 => fbp * 32,
            1 => (fbp + 70) * 32 + r.below(64),
            _ => r.below(0x4000),
        };
        let tpsm = r.pick(TEX_PSMS) as u64;
        let tbw = r.below(10) + 1;
        let tw = r.below(11);
        let th = r.below(11);
        let cbp = r.below(0x4000);
        let cpsm = r.pick(&[0u64, 2, 0xA]);
        let tex0 = tbp
            | tbw << 14
            | tpsm << 20
            | tw << 26
            | th << 30
            | r.below(2) << 34
            | r.below(4) << 35
            | cbp << 37
            | cpsm << 51
            | r.below(2) << 55
            | r.below(32) << 56
            | 1 << 61;
        gs.write_reg(0x06 + c, tex0);
        let mut clamp = r.below(16);
        clamp |= r.below(0x400) << 4 | r.below(0x400) << 14;
        clamp |= r.below(0x400) << 24 | r.below(0x400) << 34;
        gs.write_reg(0x08 + c, clamp);
        let tex1 = r.below(2) | r.below(8) << 5 | r.below(4) << 19 | r.below(0x1000) << 32;
        gs.write_reg(0x14 + c, tex1);
        gs.write_reg(0x18 + c, r.pick(&[0u64, 8 << 4 | 16 << 36, 2048 << 4 | 2048 << 36]));
        let w = fbw as u32 * 64;
        let sx0 = r.below(64);
        let sx1 = if r.chance(10) { w + 40 } else { w - 1 - r.below(32) as u32 } as u64;
        let sy0 = r.below(64);
        let sy1 = 200 + r.below(300);
        gs.write_reg(0x40 + c, sx0 | sx1 << 16 | sy0 << 32 | sy1 << 48);
        gs.write_reg(0x42 + c, r.below(256) | r.below(256) << 32);
        gs.write_reg(0x47 + c, r.below(1 << 19));
        gs.write_reg(0x4A + c, r.below(2));
    }
    let _ = ctx;
    gs.write_reg(0x3B, r.below(256) | (r.below(2)) << 15 | r.below(256) << 32);
    gs.write_reg(0x3D, r.next() & 0xFF_FFFF);
    gs.write_reg(0x44, r.next());
    gs.write_reg(0x45, r.below(2));
    gs.write_reg(0x46, r.below(2));
    gs.write_reg(0x49, r.below(2));
    gs.write_reg(0x22, r.pick(&[0u64, 0, 2, 3]));
}

pub(super) fn random_vertex(r: &mut Rng, w: u32) -> Vertex {
    let span = (w + 64) * 16;
    Vertex {
        x: r.below(span as u64) as u32 + if r.chance(50) { 0 } else { 8 << 4 },
        y: r.below(560 * 16) as u32,
        z: r.next() as u32,
        f: r.below(256) as u8,
        rgba: [
            r.below(256) as u8,
            r.below(256) as u8,
            r.below(256) as u8,
            r.below(256) as u8,
        ],
        s: r.f32_in(-1.0, 2.0),
        t: r.f32_in(-1.0, 2.0),
        q: if r.chance(10) { r.f32_in(-1.0, 1.0) } else { r.f32_in(0.2, 2.0) },
        u: r.below(0x4000) as u32,
        v: r.below(0x4000) as u32,
    }
}

/// A host->local CT32 upload between primitives (often over textures or the
/// frame buffer that queued primitives use), applied to both GSes.
fn upload(r: &mut Rng, gs: &mut Gs, reference: &mut [u32], fbp: u64) {
    let dbp = match r.below(3) {
        0 => (fbp * 32 + r.below(64)) as u32,
        _ => r.below(0x4000) as u32,
    };
    let dbw = r.below(4) as u32 + 1;
    let (x0, y0) = (r.below(64) as u32, r.below(64) as u32);
    let (w, h) = (4 * (r.below(16) as u32 + 1), r.below(32) as u32 + 1);
    let data: Vec<u32> = (0..w * h).map(|_| r.next() as u32).collect();
    let mut i = 0;
    for y in 0..h {
        for x in 0..w {
            write_pixel(reference, PSMCT32, dbp, dbw, x0 + x, y0 + y, data[i]);
            i += 1;
        }
    }
    gs.write_reg(0x50, (dbp as u64) << 32 | (dbw as u64) << 48);
    gs.write_reg(0x51, (x0 as u64) << 32 | (y0 as u64) << 48);
    gs.write_reg(0x52, w as u64 | (h as u64) << 32);
    gs.write_reg(0x53, 0);
    let nq = (w * h / 4) as u64;
    let mut q: Vec<u128> = vec![(nq | 2 << 58) as u128 | 1 << 15];
    for c in data.chunks_exact(4) {
        q.push(c.iter().rev().fold(0u128, |acc, &v| acc << 32 | v as u128));
    }
    // Split the image over several GIF writes, like DMA chunks.
    let cut = 1 + r.below(q.len() as u64) as usize;
    gs.gif_write(3, &q[..cut]);
    gs.gif_write(3, &q[cut..]);
}

fn run_case(seed: u64, prims_per_state: usize, states: usize) {
    let mut r = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    let mut gs = Gs::new();
    for v in gs.vram.iter_mut() {
        *v = r.next() as u32;
    }
    gs.set_deferred(true);
    let mut reference = gs.vram.clone();
    let fbp = r.pick(&[0u64, 0, 140]);
    let fbw = r.pick(&[1u64, 4, 8, 10]);
    for _ in 0..states {
        random_state(&mut r, &mut gs, fbp, fbw);
        for _ in 0..prims_per_state {
            let prim = r.pick(&[3u64, 4, 6, 6, 3, 0, 1]);
            let attr = r.below(0x400) & !0x7 & !0x80; // IIP TME FGE ABE AA1(off) FST CTXT
            gs.write_reg(0x00, prim | attr);
            let n = match prim {
                0 => 1,
                1 | 6 => 2,
                _ => 3,
            };
            let w = fbw as u32 * 64;
            let mut verts = [Vertex::default(); 3];
            for v in verts.iter_mut().take(n) {
                *v = random_vertex(&mut r, w);
            }
            if prim == 6 && r.chance(50) {
                // Copy-style sprite: texel = pixel.
                verts[0].u = verts[0].x;
                verts[0].v = verts[0].y;
                verts[1].u = verts[1].x;
                verts[1].v = verts[1].y;
            }
            let d = ref_raster::Draw::build(&gs.regs, &gs.clut);
            ref_raster::draw(&mut reference, &d, prim as u32, &verts[..n]);
            gs.draw_primitive(prim as u32, &verts[..n]);
            if r.chance(15) {
                upload(&mut r, &mut gs, &mut reference, fbp);
            }
        }
        if r.chance(30) {
            gs.sync();
        }
    }
    gs.sync();
    if let Some(i) = (0..gs.vram.len()).find(|&i| gs.vram[i] != reference[i]) {
        panic!(
            "seed {seed}: VRAM differs at word {i:#x}: {:#010x} != reference {:#010x}",
            gs.vram[i], reference[i]
        );
    }
}

#[test]
fn matches_reference_rasterizer() {
    for seed in 0..60 {
        run_case(seed, 6, 6);
    }
}

#[test]
fn matches_reference_rasterizer_long_batches() {
    for seed in 1000..1012 {
        run_case(seed, 40, 3);
    }
}
