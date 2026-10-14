//! GPU backend tests against the CPU rasterizer. They skip (pass with a
//! note) when no wgpu adapter is available, e.g. on headless CI.

use super::super::diff_tests::{random_state, random_vertex, Rng};
use super::super::swizzle::*;
use super::super::{Backend, Gs, Vertex};

/// A CPU and a GPU GS with the same random VRAM, or `None` without a GPU.
fn pair(seed: u64) -> Option<(Gs, Gs)> {
    let mut cpu = Gs::new_cpu();
    let mut r = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    for v in cpu.vram.iter_mut() {
        *v = r.next() as u32;
    }
    let mut gpu = Gs::new_cpu();
    gpu.vram.copy_from_slice(&cpu.vram);
    if !gpu.set_backend(Backend::Gpu) {
        eprintln!("no GPU adapter: skipping GPU GS test");
        return None;
    }
    cpu.set_deferred(true);
    gpu.set_deferred(true);
    Some((cpu, gpu))
}

fn both(cpu: &mut Gs, gpu: &mut Gs, f: impl Fn(&mut Gs)) {
    f(cpu);
    f(gpu);
}

fn differing_words(cpu: &mut Gs, gpu: &mut Gs) -> usize {
    cpu.sync();
    gpu.sync_vram();
    cpu.vram.iter().zip(&gpu.vram).filter(|(a, b)| a != b).count()
}

fn xyz(x: u32, y: u32) -> u64 {
    (x * 16) as u64 | ((y * 16) as u64) << 16 | 0x1234 << 32
}

/// 640x448 CT32 frame at 0, Z24 at page 140, scissor full.
fn frame(gs: &mut Gs) {
    gs.write_reg(0x4C, 10 << 16);
    gs.write_reg(0x4E, 140 | 1 << 24);
    gs.write_reg(0x40, 639 << 16 | 447 << 48);
    gs.write_reg(0x18, 0);
    gs.write_reg(0x47, 1 << 16 | 1 << 17); // ZTE, ALWAYS
}

/// Upload a `w`x`h` CT32 image at block `dbp` (width 64*`dbw`).
fn upload(gs: &mut Gs, dbp: u32, dbw: u32, w: u32, h: u32, px: impl Fn(u32, u32) -> u32) {
    gs.write_reg(0x50, (dbp as u64) << 32 | (dbw as u64) << 48);
    gs.write_reg(0x51, 0);
    gs.write_reg(0x52, w as u64 | (h as u64) << 32);
    gs.write_reg(0x53, 0);
    let data: Vec<u32> = (0..h).flat_map(|y| (0..w).map(move |x| (x, y))).map(|(x, y)| px(x, y)).collect();
    let mut q: Vec<u128> = vec![(data.len() as u64 / 4 | 2 << 58) as u128 | 1 << 15];
    for c in data.chunks_exact(4) {
        q.push(c.iter().rev().fold(0u128, |acc, &v| acc << 32 | v as u128));
    }
    gs.gif_write(3, &q);
}

#[test]
fn flat_and_gouraud_primitives_match() {
    let Some((mut cpu, mut gpu)) = pair(1) else { return };
    both(&mut cpu, &mut gpu, |gs| {
        frame(gs);
        // Flat sprite, then flat triangles.
        gs.write_reg(0x00, 6);
        gs.write_reg(0x01, 0x8040_2010);
        gs.write_reg(0x05, xyz(10, 20));
        gs.write_reg(0x05, xyz(300, 200));
        gs.write_reg(0x00, 3);
        for (i, &(x, y)) in [(5u32, 5u32), (600, 30), (100, 400), (630, 440), (20, 300), (500, 100)].iter().enumerate() {
            gs.write_reg(0x01, 0x8000_00FF | (i as u64 * 40) << 8);
            gs.write_reg(0x05, xyz(x, y));
        }
    });
    assert_eq!(differing_words(&mut cpu, &mut gpu), 0);
}

#[test]
fn textured_copy_and_clut_match() {
    let Some((mut cpu, mut gpu)) = pair(2) else { return };
    both(&mut cpu, &mut gpu, |gs| {
        frame(gs);
        // CT32 texture at block 0x2000 (64 wide), copied 1:1 by a UV sprite.
        upload(gs, 0x2000, 1, 64, 64, |x, y| 0x8000_0000 | x << 2 | y << 10 | (x ^ y) << 16);
        gs.write_reg(0x06, 0x2000 | 1 << 14 | 6 << 26 | 6 << 30 | 1 << 35); // DECAL
        gs.write_reg(0x14, 0);
        gs.write_reg(0x00, 6 | 1 << 4 | 1 << 8); // sprite, TME, FST
        gs.write_reg(0x03, 0);
        gs.write_reg(0x05, xyz(100, 100));
        gs.write_reg(0x03, (64 * 16) | (64 * 16) << 16);
        gs.write_reg(0x05, xyz(164, 164));
        // A T8 texture through a CT32 CLUT (CSM1), scaled 2x.
        upload(gs, 0x2400, 1, 16, 16, |x, y| 0x8000_0000 | (x * 16) | (y * 16) << 8);
        upload(gs, 0x2800, 2, 64, 16, |x, y| (x * 7 + y * 13) & 0xFF * 0x0101_0101);
        let tex0 = 0x2800u64 | 2 << 14 | (PSMT8 as u64) << 20 | 6 << 26 | 4 << 30 | 1 << 35 | 0x2400 << 37 | 1 << 61;
        gs.write_reg(0x06, tex0);
        gs.write_reg(0x00, 6 | 1 << 4 | 1 << 8);
        gs.write_reg(0x03, 0);
        gs.write_reg(0x05, xyz(200, 50));
        gs.write_reg(0x03, (64 * 16) | (16 * 16) << 16);
        gs.write_reg(0x05, xyz(328, 82));
    });
    assert_eq!(differing_words(&mut cpu, &mut gpu), 0);
    let (mut a, mut b) = (Vec::new(), Vec::new());
    for gs in [&mut cpu, &mut gpu] {
        gs.write_priv(0x00, 1);
        gs.write_priv(0x70, 10 << 9);
        gs.write_priv(0x80, 2559u64 << 32 | 447 << 44 | 3 << 23);
    }
    assert_eq!(cpu.display(&mut a), gpu.display(&mut b));
    assert!(a == b, "displayed images differ");
}

#[test]
fn readback_after_gpu_drawing() {
    let Some((mut cpu, mut gpu)) = pair(3) else { return };
    both(&mut cpu, &mut gpu, |gs| {
        frame(gs);
        gs.write_reg(0x00, 6);
        gs.write_reg(0x01, 0x8011_2233);
        gs.write_reg(0x05, xyz(0, 0));
        gs.write_reg(0x05, xyz(32, 8));
        // Local->host of the drawn rectangle (CT32, 8x4 at (4, 2)).
        gs.write_reg(0x50, 10 << 16);
        gs.write_reg(0x51, 4 | 2 << 16);
        gs.write_reg(0x52, 8 | 4 << 32);
        gs.write_reg(0x53, 1);
    });
    let a = cpu.read_local(64);
    let b = gpu.read_local(64);
    assert_eq!(a.len(), 8);
    assert_eq!(a, b);
    assert!(a.iter().all(|&q| q as u32 == 0x8011_2233));
}

/// Random register states and primitives (the CPU differential test's
/// generator): the GPU interpolates in f32 rather than f64, so a small
/// fraction of words may differ, but no more.
#[test]
fn random_scenes_mostly_match() {
    let mut total = 0usize;
    let mut differ = 0usize;
    for seed in 0..12u64 {
        let Some((mut cpu, mut gpu)) = pair(100 + seed) else { return };
        let mut r = Rng(seed.wrapping_mul(0x2545_F491_4F6C_DD1D) | 1);
        let (fbp, fbw) = (r.pick(&[0u64, 140]), r.pick(&[4u64, 8, 10]));
        for _ in 0..4 {
            let state = r.next();
            for gs in [&mut cpu, &mut gpu] {
                random_state(&mut Rng(state | 1), gs, fbp, fbw);
            }
            for _ in 0..8 {
                let prim = r.pick(&[3u64, 4, 6, 6, 3, 0, 1]);
                let attr = r.below(0x400) & !0x7 & !0x80;
                let n = match prim {
                    0 => 1,
                    1 | 6 => 2,
                    _ => 3,
                };
                let mut verts = [Vertex::default(); 3];
                for v in verts.iter_mut().take(n) {
                    *v = random_vertex(&mut r, fbw as u32 * 64);
                }
                for gs in [&mut cpu, &mut gpu] {
                    gs.write_reg(0x00, prim | attr);
                    gs.draw_primitive(prim as u32, &verts[..n]);
                }
            }
        }
        let before = Gs::new_cpu().vram.len();
        differ += differing_words(&mut cpu, &mut gpu);
        total += before;
    }
    let frac = differ as f64 / total as f64;
    eprintln!("random scenes: {differ} of {total} words differ ({:.4}%)", frac * 100.0);
    assert!(frac < 0.005, "too many differing words: {frac}");
}
