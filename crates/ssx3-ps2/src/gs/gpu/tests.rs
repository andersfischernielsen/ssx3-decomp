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
    gpu.set_latent_readback(false);
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

// ---------------------------------------------------------------- native runs

use super::{NativeRecord, NativeSurface, NativeSurfaces, DEPTH_SCALE};

/// A GPU GS over random VRAM, and a copy of that VRAM.
fn gpu_random(seed: u64) -> Option<(Gs, Vec<u32>)> {
    let mut gs = Gs::new_cpu();
    let mut r = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
    for v in gs.vram.iter_mut() {
        *v = r.next() as u32;
    }
    let orig = gs.vram.clone();
    if !gs.set_backend(Backend::Gpu) {
        eprintln!("no GPU adapter: skipping GPU GS test");
        return None;
    }
    gs.set_deferred(true);
    Some((gs, orig))
}

/// Frame at block 0 and Z right after it (block `z_off` further), `fbw` wide, `h` tall.
fn surfaces(fpsm: u32, zpsm: u32, fbw: u32, h: u32, z_off: u32) -> NativeSurfaces {
    let rows = if fpsm == PSMCT32 || fpsm == PSMCT24 { 32 } else { 64 };
    let fb_pages = h.div_ceil(rows) * fbw;
    NativeSurfaces {
        frame: Some(NativeSurface { bp: 0, bw: fbw, psm: fpsm }),
        z: Some(NativeSurface { bp: (fb_pages + 1) * 32 + z_off, bw: fbw, psm: zpsm }),
        width: fbw * 64,
        height: h,
    }
}

/// The GS's 8-bit-per-channel view of a frame pixel (gs.wgsl `read_fb`).
fn frame_rgba8(psm: u32, raw: u32) -> u32 {
    match psm {
        PSMCT24 => raw | 0x8000_0000,
        PSMCT16 | PSMCT16S => {
            (raw & 0x1F) << 3
                | ((raw >> 5) & 0x1F) << 11
                | ((raw >> 10) & 0x1F) << 19
                | if raw & 0x8000 != 0 { 0x8000_0000 } else { 0 }
        }
        _ => raw,
    }
}

const FRAME_PSMS: [u32; 4] = [PSMCT32, PSMCT24, PSMCT16, PSMCT16S];
const Z_PSMS: [u32; 3] = [PSMZ24, PSMZ16, PSMZ16S];

/// Unswizzle -> swizzle with no passes leaves every VRAM word as it was,
/// for every frame/Z format pair and several buffer widths, with several
/// runs in one submission.
#[test]
fn native_roundtrip_is_identity() {
    let mut seed = 500;
    for &fbw in &[1u32, 5, 10, 16] {
        for (i, &fpsm) in FRAME_PSMS.iter().enumerate() {
            let Some((mut gs, orig)) = gpu_random(seed) else { return };
            seed += 1;
            for (j, &zpsm) in Z_PSMS.iter().enumerate() {
                let h = [200, 448, 37][(i + j) % 3];
                let s = surfaces(fpsm, zpsm, fbw, h, (j as u32 * 5) % 32);
                gs.native_run(Some(s), Vec::new()).expect("native run");
            }
            gs.sync();
            gs.sync_vram();
            let bad = gs.vram.iter().zip(&orig).filter(|(a, b)| a != b).count();
            assert_eq!(bad, 0, "fbw {fbw} frame psm {fpsm:#x}: {bad} words changed");
            assert_eq!(gs.gpu_stats().unwrap().native.runs, 3);
        }
    }
}

/// The run's targets hold exactly what the CPU swizzle model reads:
/// colour as the GS's 8-bit channels, depth as Z / 2^24 (bit-exact).
#[test]
fn native_targets_match_cpu_model() {
    let mut seed = 600;
    for &fpsm in &FRAME_PSMS {
        for &zpsm in &Z_PSMS {
            for &fbw in &[3u32, 10] {
                let Some((mut gs, orig)) = gpu_random(seed) else { return };
                seed += 1;
                let s = surfaces(fpsm, zpsm, fbw, 123, 0);
                gs.native_run(Some(s), Vec::new()).expect("native run");
                gs.sync();
                let (w, h) = (s.width, s.height);
                let (c, d) = gs.gpu.as_mut().unwrap().native_read_targets(w, h);
                let f = s.frame.unwrap();
                let z = s.z.unwrap();
                let fs = Surface::new(f.psm, f.bp, f.bw);
                let zs = Surface::new(z.psm, z.bp, z.bw);
                for y in 0..h {
                    for x in 0..w {
                        let i = (y * w + x) as usize;
                        let want = frame_rgba8(fpsm, fs.read(&orig, x, y));
                        assert_eq!(c[i], want, "colour at ({x}, {y}), psm {fpsm:#x}");
                        let zv = zs.read(&orig, x, y);
                        let dw = zv as f32 / DEPTH_SCALE;
                        assert_eq!(d[i].to_bits(), dw.to_bits(), "depth at ({x}, {y}), psm {zpsm:#x}");
                        assert_eq!((d[i] * DEPTH_SCALE) as u32, zv);
                    }
                }
            }
        }
    }
}

/// A clear render pass inside a run: colour and depth land in VRAM through
/// the swizzle exactly as the CPU model writes them, and nothing outside
/// the surfaces changes.
#[test]
fn native_pass_writes_back() {
    let mut seed = 700;
    for &fpsm in &[PSMCT32, PSMCT24, PSMCT16] {
        for &zpsm in &[PSMZ24, PSMZ16] {
            let Some((mut gs, orig)) = gpu_random(seed) else { return };
            seed += 1;
            let s = surfaces(fpsm, zpsm, 10, 96, 0);
            let (rgba, zval) = (0x7856_3412u32, if zpsm == PSMZ24 { 0x12_3456u32 } else { 0xBEEF });
            let pass: NativeRecord = Box::new(move |t, enc| {
                let _ = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("test-clear"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: t.color,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: (rgba & 0xFF) as f64 / 255.0,
                                g: ((rgba >> 8) & 0xFF) as f64 / 255.0,
                                b: ((rgba >> 16) & 0xFF) as f64 / 255.0,
                                a: (rgba >> 24) as f64 / 255.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: t.depth,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(zval as f32 / DEPTH_SCALE),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
            });
            gs.native_run(Some(s), vec![pass]).expect("native run");
            gs.sync();
            gs.sync_vram();
            let mut want = orig.clone();
            let f = s.frame.unwrap();
            let z = s.z.unwrap();
            let fs = Surface::new(f.psm, f.bp, f.bw);
            let zs = Surface::new(z.psm, z.bp, z.bw);
            let packed = match fpsm {
                PSMCT16 => (rgba >> 3) & 0x1F | ((rgba >> 11) & 0x1F) << 5 | ((rgba >> 19) & 0x1F) << 10,
                _ => rgba,
            };
            for y in 0..s.height {
                for x in 0..s.width {
                    fs.write(&mut want, x, y, packed);
                    zs.write(&mut want, x, y, zval);
                }
            }
            let bad = gs.vram.iter().zip(&want).filter(|(a, b)| a != b).count();
            assert_eq!(bad, 0, "frame {fpsm:#x} z {zpsm:#x}: {bad} words differ");
        }
    }
}

/// A run sits exactly between the primitives sent before and after it.
#[test]
fn native_run_is_ordered_in_the_stream() {
    let Some((mut gs, _)) = gpu_random(800) else { return };
    frame(&mut gs);
    gs.write_reg(0x00, 6);
    gs.write_reg(0x01, 0x8011_2233);
    gs.write_reg(0x05, xyz(0, 0));
    gs.write_reg(0x05, xyz(100, 100));
    let s = gs.native_surfaces(0);
    assert_eq!((s.width, s.height), (640, 448));
    let pass: NativeRecord = Box::new(|t, enc| {
        let _ = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("test-clear"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: t.color,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color { r: 1.0, g: 0.0, b: 0.0, a: 128.0 / 255.0 }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: t.depth,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0.5), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    });
    gs.native_run(None, vec![pass]).expect("native run");
    gs.write_reg(0x01, 0x8044_5566);
    gs.write_reg(0x05, xyz(50, 50));
    gs.write_reg(0x05, xyz(150, 150));
    gs.sync();
    gs.sync_vram();
    let fb = Surface::new(PSMCT32, 0, 10);
    let zb = Surface::new(PSMZ24, 140 * 32, 10);
    // Before the run: overwritten by the clear.
    assert_eq!(fb.read(&gs.vram, 10, 10), 0x8000_00FF);
    assert_eq!(zb.read(&gs.vram, 10, 10), 0x80_0000);
    // After the run: the second sprite.
    assert_eq!(fb.read(&gs.vram, 120, 120), 0x8044_5566);
    assert_eq!(zb.read(&gs.vram, 120, 120), 0x1234);
    assert_eq!(fb.read(&gs.vram, 600, 400), 0x8000_00FF);
}

/// Refused runs queue nothing.
#[test]
fn native_run_refusals() {
    use super::NativeError;
    let Some((mut gs, _)) = gpu_random(900) else { return };
    let mut s = surfaces(PSMCT32, PSMZ24, 10, 448, 0);
    s.z.as_mut().unwrap().psm = PSMZ32;
    assert_eq!(gs.native_run(Some(s), Vec::new()), Err(NativeError::ZFormat(PSMZ32)));
    let mut s = surfaces(PSMCT32, PSMZ24, 10, 448, 0);
    s.z.as_mut().unwrap().bp = 32;
    assert_eq!(gs.native_run(Some(s), Vec::new()), Err(NativeError::Aliased));
    let mut s = surfaces(PSMCT32, PSMZ24, 10, 448, 0);
    s.frame.as_mut().unwrap().bp = 500 * 32;
    assert_eq!(gs.native_run(Some(s), Vec::new()), Err(NativeError::Wraps));
    let mut s = surfaces(PSMCT32, PSMZ24, 10, 448, 0);
    s.width = 700;
    assert_eq!(gs.native_run(Some(s), Vec::new()), Err(NativeError::TooLarge));
    let st = gs.gpu_stats().unwrap().native;
    assert_eq!((st.runs, st.refused), (0, 4));
    let mut cpu = Gs::new_cpu();
    assert_eq!(cpu.native_run(None, Vec::new()), Err(NativeError::NoGpu));
}

/// Cost of an empty round trip of a 640x448 CT32 frame + Z24 (run with
/// `SSX3_GS_PROFILE=1 cargo test --release -p ssx3-ps2 native_roundtrip_cost -- --ignored --nocapture`
/// for GPU timestamps).
#[test]
#[ignore]
fn native_roundtrip_cost() {
    let Some((mut gs, _)) = gpu_random(1000) else { return };
    let s = surfaces(PSMCT32, PSMZ24, 10, 448, 0);
    // Warm up.
    for _ in 0..20 {
        gs.native_run(Some(s), Vec::new()).unwrap();
    }
    gs.sync();
    gs.gpu_wait_idle();
    let gpu0 = gs.gpu_stats().unwrap().native.gpu_ns;
    let n = 1000;
    let t0 = std::time::Instant::now();
    for i in 0..n {
        gs.native_run(Some(s), Vec::new()).unwrap();
        if i % 50 == 49 {
            gs.sync();
        }
    }
    gs.sync();
    gs.gpu_wait_idle();
    let wall = t0.elapsed().as_secs_f64();
    let gpu = gs.gpu_stats().unwrap().native.gpu_ns - gpu0;
    eprintln!(
        "native round trip 640x448 CT32+Z24: wall {:.1} us/run, GPU timestamps {:.1} us/run",
        wall * 1e6 / n as f64,
        gpu as f64 / 1e3 / n as f64
    );
}

// ---------------------------------------------------------------- native type-3 triangles

/// Native-3 triangles (hardware rasterizer) against the compute GS on the
/// same random triangles: prints how many frame/Z words differ and by how
/// much (rasterization-rule and interpolation residuals). Run with
/// `SSX3_RENDER_NATIVE=type3` and `--nocapture`.
#[test]
fn n3_triangles_vs_compute() {
    for scene in 0..8u32 {
        let Some((mut emu, _)) = gpu_random(7) else { return };
        let Some((mut nat, _)) = gpu_random(7) else { return };
        let mut r = Rng(0x1234_5678 + scene as u64);
        for gs in [&mut emu, &mut nat] {
            frame(gs);
            // CT32 texture at block 0x2000, 256x256 noise.
            upload(gs, 0x3000, 4, 256, 256, |x, y| {
                let h = (x.wrapping_mul(0x9E37_79B9) ^ y.wrapping_mul(0x85EB_CA6B)).rotate_left(13);
                h & 0x00FF_FFFF | 0x8000_0000
            });
            gs.write_reg(0x47, 1 << 16 | 2 << 17); // ZTE, GEQUAL
            gs.write_reg(0x46, 1); // COLCLAMP
        }
        let (prim_attr, tex0, tex1, alpha): (u64, u64, u64, u64) = match scene {
            0 => (0, 0, 0, 0),                                         // flat
            1 => (1 << 3, 0, 0, 0),                                    // gouraud
            2 => (1 << 3 | 1 << 4, 1, 0, 0),                           // textured nearest, modulate
            3 => (1 << 3 | 1 << 4, 1, 1 << 5 | 1 << 6, 0),             // textured bilinear
            4 => (1 << 3 | 1 << 6, 0, 0, 0x44),                        // (Cs-Cd)As+Cd
            6 => (1 << 3 | 1 << 4, 1, 1 << 5 | 1 << 6, 0),             // constant gouraud, bilinear
            7 => (1 << 3, 0, 0, 0),                                    // FB_ONLY alpha test
            _ => (1 << 3 | 1 << 4 | 1 << 6, 1, 1 << 5 | 1 << 6, 0x48), // Cs As + Cd, bilinear
        };
        for gs in [&mut emu, &mut nat] {
            if tex0 != 0 {
                gs.write_reg(0x06, 0x3000 | 4 << 14 | 8 << 26 | 8 << 30 | 1 << 34);
                gs.write_reg(0x14, tex1);
            }
            gs.write_reg(0x42, alpha);
            if scene == 7 {
                // ATE GREATER 0x40, AFAIL = FB_ONLY, ZTE GEQUAL.
                gs.write_reg(0x47, 1 | 6 << 1 | 0x40 << 4 | 1 << 12 | 1 << 16 | 2 << 17);
            }
        }
        for i in 0..400 {
            let mut v = [Vertex::default(); 3];
            for k in 0..3 {
                let q = r.f32_in(0.3, 2.0);
                v[k] = Vertex {
                    x: (r.below(700 * 16) as u32).wrapping_sub(30 * 16),
                    y: (r.below(500 * 16) as u32).wrapping_sub(30 * 16),
                    z: r.below(1 << 24) as u32,
                    f: 0,
                    rgba: [r.below(256) as u8, r.below(256) as u8, r.below(256) as u8, r.below(129) as u8],
                    s: r.f32_in(0.0, 1.0) * q,
                    t: r.f32_in(0.0, 1.0) * q,
                    q,
                    u: 0,
                    v: 0,
                };
                if scene == 6 && k > 0 {
                    v[k].rgba = v[0].rgba;
                }
                // Small triangles too.
                if i % 2 == 1 && k > 0 {
                    v[k].x = v[0].x.wrapping_add(r.below(40 * 16) as u32);
                    v[k].y = v[0].y.wrapping_add(r.below(40 * 16) as u32);
                }
            }
            for (j, gs) in [&mut emu, &mut nat].into_iter().enumerate() {
                gs.write_reg(0x00, 3 | prim_attr);
                gs.n3_capture = j == 1;
                gs.draw_primitive(3, &v);
                gs.n3_capture = false;
            }
        }
        emu.sync_vram();
        nat.sync_vram();
        let st = nat.gpu_stats().unwrap().n3;
        let fb = Surface::new(PSMCT32, 0, 10);
        let zb = Surface::new(PSMZ24, 140 * 32, 10);
        let (mut cov, mut c1, mut cbig, mut zd, mut maxz) = (0, 0, 0, 0, 0i64);
        for y in 0..448 {
            for x in 0..640 {
                let a = fb.read(&emu.vram, x, y);
                let b = fb.read(&nat.vram, x, y);
                if a != b {
                    let m = (0..4).map(|k| ((a >> (8 * k)) & 0xFF) as i32 - ((b >> (8 * k)) & 0xFF) as i32).map(i32::abs).max().unwrap();
                    if m <= 1 {
                        c1 += 1;
                    } else if m > 8 {
                        cbig += 1;
                    } else {
                        cov += 1;
                    }
                }
                let za = zb.read(&emu.vram, x, y) & 0xFF_FFFF;
                let zbv = zb.read(&nat.vram, x, y) & 0xFF_FFFF;
                if za != zbv {
                    zd += 1;
                    maxz = maxz.max((za as i64 - zbv as i64).abs());
                }
            }
        }
        eprintln!(
            "n3 scene {scene}: native tris {} fallback {:?}; frame px differing: <=1 {c1}, 2..8 {cov}, >8 {cbig}; z differ {zd} (max {maxz})",
            st.tris, st.fallback
        );
    }
}

/// Destination-read draws (As > 0x80, Ad, DATE, PABE, partial FBMSK, no
/// COLCLAMP), CT16 frames and sprites through the native path against the
/// compute GS, over random VRAM (so destination alpha varies). These have
/// no fixed-function form; the shader computes the GS blend from the
/// destination pixel, so only interpolation residuals (+-1, rarely more at
/// texel boundaries) may differ, and nothing falls back.
#[test]
fn n3_dst_read_ct16_sprites_vs_compute() {
    use super::N3Fallback as Fb;
    // (name, frame psm, ALPHA_1, TEST_1 extra bits, PABE, COLCLAMP, FBMSK, textured, sprites)
    let scenes: &[(&str, u32, u64, u64, bool, bool, u32, bool, bool)] = &[
        ("(Cd-Cs)As, As<=0xFF", PSMCT32, 1 | 2 << 6, 0, false, true, 0, true, false),
        ("(Cs-Cd)As+Cd, As<=0xFF", PSMCT32, 0x44, 0, false, true, 0, false, false),
        ("Cs*Ad+Cd", PSMCT32, 2 << 2 | 1 << 4 | 1 << 6, 0, false, true, 0, true, false),
        ("DATE + (Cs-Cd)As+Cd", PSMCT32, 0x44, 1 << 14, false, true, 0, false, false),
        ("PABE, no COLCLAMP", PSMCT32, 2 << 2 | 1 << 6, 0, true, false, 0, true, false),
        ("partial FBMSK", PSMCT32, 0x44, 0, false, true, 0x0F00_F0F0, false, false),
        ("CT16, no blend, FB_ONLY", PSMCT16, 0, 1 | 6 << 1 | 0x40 << 4 | 1 << 12, false, true, 0, false, false),
        ("CT16, (Cs-Cd)As+Cd", PSMCT16, 0x44, 0, false, true, 0, true, false),
        ("sprites, (Cs-Cd)As+Cd", PSMCT32, 0x44, 0, false, true, 0, true, true),
        ("sprites, Cs*As+Cd, FB_ONLY", PSMCT32, 2 << 2 | 1 << 6, 1 | 6 << 1 | 0x40 << 4 | 1 << 12, false, true, 0, true, true),
        ("sprites, CT16, flat", PSMCT16, 0, 0, false, true, 0, false, true),
    ];
    for (si, &(name, fpsm, alpha, test, pabe, colclamp, fbmsk, tex, sprites)) in scenes.iter().enumerate() {
        let Some((mut emu, _)) = gpu_random(11 + si as u64) else { return };
        let Some((mut nat, _)) = gpu_random(11 + si as u64) else { return };
        let mut r = Rng(0xABCD_0000 + si as u64);
        for gs in [&mut emu, &mut nat] {
            frame(gs);
            gs.write_reg(0x4C, 10 << 16 | (fpsm as u64) << 24 | (fbmsk as u64) << 32);
            upload(gs, 0x3000, 4, 256, 256, |x, y| {
                let h = (x.wrapping_mul(0x9E37_79B9) ^ y.wrapping_mul(0x85EB_CA6B)).rotate_left(13);
                // Texel alpha over the whole 0..=255 range.
                h
            });
            gs.write_reg(0x47, test | 1 << 16 | 2 << 17); // ZTE, GEQUAL
            gs.write_reg(0x46, colclamp as u64);
            gs.write_reg(0x49, pabe as u64);
            gs.write_reg(0x42, alpha);
            if tex {
                gs.write_reg(0x06, 0x3000 | 4 << 14 | 8 << 26 | 8 << 30 | 1 << 34);
                gs.write_reg(0x14, 0);
            }
        }
        let attr: u64 = 1 << 3 | (tex as u64) << 4 | ((alpha != 0) as u64) << 6;
        for i in 0..300 {
            let mut v = [Vertex::default(); 3];
            for k in 0..3 {
                let q = r.f32_in(0.3, 2.0);
                v[k] = Vertex {
                    x: (r.below(700 * 16) as u32).wrapping_sub(30 * 16),
                    y: (r.below(500 * 16) as u32).wrapping_sub(30 * 16),
                    z: r.below(1 << 24) as u32,
                    f: 0,
                    rgba: [r.below(256) as u8, r.below(256) as u8, r.below(256) as u8, r.below(256) as u8],
                    s: r.f32_in(0.0, 1.0) * q,
                    t: r.f32_in(0.0, 1.0) * q,
                    q,
                    u: 0,
                    v: 0,
                };
                if i % 2 == 1 && k > 0 {
                    v[k].x = v[0].x.wrapping_add(r.below(40 * 16) as u32);
                    v[k].y = v[0].y.wrapping_add(r.below(40 * 16) as u32);
                }
            }
            let (prim, n) = if sprites { (6u32, 2usize) } else { (3, 3) };
            if sprites {
                // Sprites: Q of the second vertex.
                v[0].q = v[1].q;
            }
            for (j, gs) in [&mut emu, &mut nat].into_iter().enumerate() {
                gs.write_reg(0x00, prim as u64 | attr);
                gs.n3_capture = j == 1;
                gs.draw_primitive(prim, &v[..n]);
                gs.n3_capture = false;
            }
        }
        emu.sync_vram();
        nat.sync_vram();
        let st = nat.gpu_stats().unwrap().n3;
        let fb = Surface::new(fpsm, 0, 10);
        let zb = Surface::new(PSMZ24, 140 * 32, 10);
        let (mut c1, mut cmid, mut cbig, mut zd) = (0, 0, 0, 0);
        for y in 0..448 {
            for x in 0..640 {
                let a = frame_rgba8(fpsm, fb.read(&emu.vram, x, y));
                let b = frame_rgba8(fpsm, fb.read(&nat.vram, x, y));
                if a != b {
                    let m = (0..4).map(|k| ((a >> (8 * k)) & 0xFF) as i32 - ((b >> (8 * k)) & 0xFF) as i32).map(i32::abs).max().unwrap();
                    match m {
                        0..=1 => c1 += 1,
                        // One CT16 step.
                        2..=8 => cmid += 1,
                        _ => cbig += 1,
                    }
                }
                if zb.read(&emu.vram, x, y) & 0xFF_FFFF != zb.read(&nat.vram, x, y) & 0xFF_FFFF {
                    zd += 1;
                }
            }
        }
        eprintln!(
            "n3 dst-read scene {si} ({name}): native prims {} (sprites {}, dst-read {}, copies {}), fallback {:?}; px differing <=1 {c1}, 2..8 {cmid}, >8 {cbig}; z differ {zd}",
            st.tris, st.sprites, st.dr_prims, st.dr_copies, st.fallback
        );
        let fallbacks: u64 = st.fallback.iter().sum();
        assert_eq!(fallbacks - st.fallback[Fb::NotTri as usize], 0, "{name}: fallbacks");
        assert!(st.tris > 200, "{name}: native prims");
        assert_eq!(zd, 0, "{name}: Z differs");
        // Interpolation residuals only: a handful of pixels near texel or
        // rounding boundaries (each a few in 10^4 of the frame).
        assert!(cbig <= 40 && c1 + cmid <= 2500, "{name}: {c1} {cmid} {cbig}");
    }
}

/// Frame (`fpsm`) and Z24 differences between two GSs over the test frame:
/// pixels off by <= 1, 2..=8, more, and differing Z words.
fn frame_diff(emu: &Gs, nat: &Gs, fpsm: u32) -> (u32, u32, u32, u32) {
    let fb = Surface::new(fpsm, 0, 10);
    let zb = Surface::new(PSMZ24, 140 * 32, 10);
    let (mut c1, mut cmid, mut cbig, mut zd) = (0, 0, 0, 0);
    for y in 0..448 {
        for x in 0..640 {
            let a = frame_rgba8(fpsm, fb.read(&emu.vram, x, y));
            let b = frame_rgba8(fpsm, fb.read(&nat.vram, x, y));
            if a != b {
                let m = (0..4).map(|k| ((a >> (8 * k)) & 0xFF) as i32 - ((b >> (8 * k)) & 0xFF) as i32).map(i32::abs).max().unwrap();
                match m {
                    0..=1 => c1 += 1,
                    2..=8 => cmid += 1,
                    _ => cbig += 1,
                }
            }
            if zb.read(&emu.vram, x, y) & 0xFF_FFFF != zb.read(&nat.vram, x, y) & 0xFF_FFFF {
                zd += 1;
            }
        }
    }
    (c1, cmid, cbig, zd)
}

/// Lines and line strips through the native path against the compute GS:
/// the fragment shader covers exactly the pixels of the compute GS's line
/// walk and takes their attributes and Z from the same line record.
#[test]
fn n3_lines_vs_compute() {
    // (name, PRIM attributes, ALPHA_1, TEST_1 extra bits, textured, blend unit rounding allowed)
    let scenes: &[(&str, u64, u64, u64, bool, bool)] = &[
        ("flat", 0, 0, 0, false, false),
        ("gouraud", 1 << 3, 0, 0, false, false),
        ("gouraud, FB_ONLY alpha test", 1 << 3, 0, 1 | 6 << 1 | 0x40 << 4 | 1 << 12, false, false),
        ("flat, FB_ONLY alpha test", 0, 0, 1 | 6 << 1 | 0x40 << 4 | 1 << 12, false, false),
        ("textured bilinear, modulate", 1 << 3 | 1 << 4, 0, 0, true, false),
        ("gouraud, (Cs-Cd)As+Cd", 1 << 3 | 1 << 6, 0x44, 0, false, true),
        ("gouraud, Cs*As+Cd, As > 0x80", 1 << 3 | 1 << 6, 2 << 2 | 1 << 6, 0, false, false),
    ];
    for (si, &(name, attr, alpha, test, tex, rounding)) in scenes.iter().enumerate() {
        let Some((mut emu, _)) = gpu_random(31 + si as u64) else { return };
        let Some((mut nat, _)) = gpu_random(31 + si as u64) else { return };
        let mut r = Rng(0x11AE_0000 + si as u64);
        for gs in [&mut emu, &mut nat] {
            frame(gs);
            upload(gs, 0x3000, 4, 256, 256, |x, y| (x.wrapping_mul(0x9E37_79B9) ^ y.wrapping_mul(0x85EB_CA6B)).rotate_left(13));
            gs.write_reg(0x47, test | 1 << 16 | 2 << 17); // ZTE, GEQUAL
            gs.write_reg(0x46, 1);
            gs.write_reg(0x42, alpha);
            if tex {
                gs.write_reg(0x06, 0x3000 | 4 << 14 | 8 << 26 | 8 << 30 | 1 << 34);
                gs.write_reg(0x14, 1 << 5 | 1 << 6);
            }
        }
        let mut prev = Vertex::default();
        for i in 0..1500 {
            let mut v = [Vertex::default(); 2];
            for k in 0..2 {
                let q = r.f32_in(0.3, 2.0);
                v[k] = Vertex {
                    // Sub-pixel endpoints, some outside the frame.
                    x: (r.below(700 * 16) as u32).wrapping_sub(30 * 16),
                    y: (r.below(500 * 16) as u32).wrapping_sub(30 * 16),
                    z: r.below(1 << 24) as u32,
                    f: 0,
                    rgba: [r.below(256) as u8, r.below(256) as u8, r.below(256) as u8, r.below(256) as u8],
                    s: r.f32_in(0.0, 1.0) * q,
                    t: r.f32_in(0.0, 1.0) * q,
                    q,
                    u: 0,
                    v: 0,
                };
                // Short lines, axis-aligned and diagonal ones too.
                if k == 1 && i % 3 != 0 {
                    v[1].x = v[0].x.wrapping_add(r.below(80 * 16) as u32).wrapping_sub(40 * 16);
                    v[1].y = v[0].y.wrapping_add(r.below(80 * 16) as u32).wrapping_sub(40 * 16);
                    match i % 9 {
                        1 => v[1].y = v[0].y,
                        2 => v[1].x = v[0].x,
                        4 => v[1].y = v[0].y.wrapping_add(v[1].x.wrapping_sub(v[0].x)),
                        _ => {}
                    }
                }
            }
            // Every other one continues a strip (PRIM 2).
            let prim = if i % 2 == 1 {
                v[0] = prev;
                2u32
            } else {
                1
            };
            prev = v[1];
            for (j, gs) in [&mut emu, &mut nat].into_iter().enumerate() {
                gs.write_reg(0x00, prim as u64 | attr);
                gs.n3_capture = j == 1;
                gs.draw_primitive(prim, &v);
                gs.n3_capture = false;
            }
        }
        emu.sync_vram();
        nat.sync_vram();
        let st = nat.gpu_stats().unwrap().n3;
        let (c1, cmid, cbig, zd) = frame_diff(&emu, &nat, PSMCT32);
        eprintln!(
            "n3 lines scene {si} ({name}): native prims {} (lines {}), fallback {:?}; px differing <=1 {c1}, 2..8 {cmid}, >8 {cbig}; z differ {zd}",
            st.tris, st.lines, st.fallback
        );
        assert_eq!(st.fallback.iter().sum::<u64>(), 0, "{name}: fallbacks");
        assert!(st.lines > 1000, "{name}: native lines");
        assert_eq!(zd, 0, "{name}: Z differs");
        assert_eq!(cbig + cmid, 0, "{name}: pixels differ");
        if !rounding {
            assert_eq!(c1, 0, "{name}: pixels differ");
        }
    }
}

/// Primitives sampling the frame or Z buffer they draw to (frame copies,
/// colour filters through PSMT8H, Z read as a texture), interleaved with
/// ordinary drawing, through the native path against the compute GS: each
/// samples what earlier primitives drew and nothing of its own pixels.
#[test]
fn n3_texture_feedback_vs_compute() {
    // (name, texture TEX0 (frame at 0, Z24 at block 140 * 32), PRIM extra, ALPHA_1, bilinear)
    let t8h_clut = 0x3800u64 << 37 | 1 << 61;
    let scenes: &[(&str, u64, u64, u64, bool)] = &[
        ("frame copy, decal", 10 << 14 | 10 << 26 | 9 << 30 | 1 << 34 | 1 << 35, 0, 0, false),
        ("frame copy, bilinear, (Cs-Cd)As+Cd", 10 << 14 | 10 << 26 | 9 << 30 | 1 << 34, 1 << 6, 0x44, true),
        ("frame alpha as PSMT8H index, decal, (Cd-Cs)As+Cs", 10 << 14 | 0x1B << 20 | 10 << 26 | 9 << 30 | 1 << 34 | 1 << 35 | t8h_clut, 1 << 6, 1 | 2 << 4, false),
        ("Z24 as texture, Cs*As+Cd", 140 * 32 | 10 << 14 | 0x31 << 20 | 10 << 26 | 9 << 30 | 1 << 34 | 1 << 35, 1 << 6, 2 << 2 | 1 << 6, false),
        ("Z high byte as PSMT8H index, decal", 140 * 32 | 10 << 14 | 0x1B << 20 | 10 << 26 | 9 << 30 | 1 << 34 | 1 << 35 | t8h_clut, 0, 0, false),
    ];
    for (si, &(name, tex0, pattr, alpha, bilinear)) in scenes.iter().enumerate() {
        let Some((mut emu, _)) = gpu_random(51 + si as u64) else { return };
        let Some((mut nat, _)) = gpu_random(51 + si as u64) else { return };
        let mut r = Rng(0xFEED_0000 + si as u64);
        for gs in [&mut emu, &mut nat] {
            frame(gs);
            // A 16x16 CT32 CLUT at block 0x3800.
            upload(gs, 0x3800, 1, 16, 16, |x, y| (x * 0x0011_2233 + y * 0x0B0A_0908) | 0x4000_0000);
            gs.write_reg(0x46, 1);
            gs.write_reg(0x42, alpha);
            gs.write_reg(0x14, if bilinear { 1 << 5 | 1 << 6 } else { 0 });
        }
        let mut n = 0;
        for i in 0..240 {
            let feedback = i % 2 == 1;
            let x0 = r.below(600) as u32;
            let y0 = r.below(420) as u32;
            let (w, h) = (1 + r.below(200) as u32, 1 + r.below(150) as u32);
            let mut v = [Vertex::default(); 2];
            v[0].x = x0 * 16;
            v[0].y = y0 * 16;
            v[1].x = (x0 + w).min(640) * 16;
            v[1].y = (y0 + h).min(448) * 16;
            v[0].z = r.below(1 << 24) as u32;
            v[1].z = r.below(1 << 24) as u32;
            v[1].rgba = [r.below(256) as u8, r.below(256) as u8, r.below(256) as u8, r.below(256) as u8];
            // Texel coordinates: the same rectangle (a pixel reads its own
            // texel), or shifted (overlapping itself or not), sub-texel too.
            let (du, dv) = match i % 6 {
                1 => (0, 0),
                3 => (r.below(64 * 16) as i32 - 32 * 16, r.below(64 * 16) as i32 - 32 * 16),
                _ => (r.below(400 * 16) as i32 - 200 * 16, r.below(300 * 16) as i32 - 150 * 16),
            };
            v[0].u = (x0 as i32 * 16 + du).clamp(0, 1023 * 16) as u32;
            v[0].v = (y0 as i32 * 16 + dv).clamp(0, 511 * 16) as u32;
            v[1].u = (v[1].x as i32 + du).clamp(0, 1023 * 16) as u32;
            v[1].v = (v[1].y as i32 + dv).clamp(0, 511 * 16) as u32;
            for (j, gs) in [&mut emu, &mut nat].into_iter().enumerate() {
                if feedback {
                    gs.write_reg(0x06, tex0);
                    gs.write_reg(0x00, 6 | 1 << 4 | 1 << 8 | pattr); // sprite, TME, FST
                } else {
                    gs.write_reg(0x00, 6);
                }
                gs.n3_capture = j == 1;
                gs.draw_primitive(6, &v);
                gs.n3_capture = false;
            }
            n += feedback as u32;
        }
        emu.sync_vram();
        nat.sync_vram();
        let st = nat.gpu_stats().unwrap().n3;
        let (c1, cmid, cbig, zd) = frame_diff(&emu, &nat, PSMCT32);
        eprintln!(
            "n3 feedback scene {si} ({name}): native prims {} (feedback {}, runs ended for it {}), fallback {:?}; px differing <=1 {c1}, 2..8 {cmid}, >8 {cbig}; z differ {zd}",
            st.tris, st.feedback, st.feedback_flushes, st.fallback
        );
        assert_eq!(st.fallback.iter().sum::<u64>(), 0, "{name}: fallbacks");
        assert!(st.feedback >= n as u64, "{name}: feedback prims");
        assert_eq!(zd, 0, "{name}: Z differs");
        assert_eq!(c1 + cmid + cbig, 0, "{name}: pixels differ");
    }
}

// ---------------------------------------------------------------- N× copies
// (hires, SSX3_RENDER_SCALE): fast checks that scaling never changes VRAM,
// that the N× copy is a faithful higher-resolution version of the 1× frame,
// and that frames used as textures and VRAM writes keep it consistent.

/// Textured gouraud triangles through the native path into the 640x448
/// frame at block 0 (the scene of `n3_triangles_vs_compute` scene 3).
fn hires_scene(gs: &mut Gs, seed: u64, tris: usize) {
    frame(gs);
    // A smooth texture: finer sampling must average back to the 1× result.
    upload(gs, 0x3000, 4, 256, 256, |x, y| x | y << 8 | ((x + y) / 2) << 16 | 0x8000_0000);
    gs.write_reg(0x47, 1 << 16 | 2 << 17); // ZTE, GEQUAL
    gs.write_reg(0x46, 1); // COLCLAMP
    gs.write_reg(0x06, 0x3000 | 4 << 14 | 8 << 26 | 8 << 30 | 1 << 34);
    gs.write_reg(0x14, 0);
    gs.write_reg(0x42, 0);
    let mut r = Rng(seed);
    for _ in 0..tris {
        let mut v = [Vertex::default(); 3];
        for k in 0..3 {
            let q = r.f32_in(0.3, 2.0);
            v[k] = Vertex {
                x: (r.below(700 * 16) as u32).wrapping_sub(30 * 16),
                y: (r.below(500 * 16) as u32).wrapping_sub(30 * 16),
                z: r.below(1 << 24) as u32,
                f: 0,
                rgba: [r.below(256) as u8, r.below(256) as u8, r.below(256) as u8, 128],
                s: r.f32_in(0.0, 1.0) * q,
                t: r.f32_in(0.0, 1.0) * q,
                q,
                u: 0,
                v: 0,
            };
        }
        gs.write_reg(0x00, 3 | 1 << 3 | 1 << 4);
        gs.n3_capture = true;
        gs.draw_primitive(3, &v);
        gs.n3_capture = false;
    }
}

/// The N× copy of the CT32 buffer at block `bp` (width 10) as (w, h, RGBA).
fn hires_copy(gs: &mut Gs, bp: u32) -> (u32, u32, Vec<u32>) {
    let label = format!("{bp:05x}-10-00");
    let snaps = gs.hires_snapshots();
    let s = snaps.into_iter().find(|s| s.0 == label).unwrap_or_else(|| panic!("no N× copy {label}"));
    (s.1, s.2, s.3)
}

/// Share of 2x2 blocks of an N=2 copy over 640x448 that are not uniform
/// (an upscale of 1× content has none).
fn non_uniform(w: u32, px: &[u32]) -> f64 {
    let mut n = 0;
    for y in 0..448u32 {
        for x in 0..640u32 {
            let at = |dx: u32, dy: u32| px[((2 * y + dy) * w + 2 * x + dx) as usize] & 0xFF_FFFF;
            if at(0, 0) != at(1, 0) || at(0, 0) != at(0, 1) || at(0, 0) != at(1, 1) {
                n += 1;
            }
        }
    }
    n as f64 / (640.0 * 448.0)
}

#[test]
fn hires_keeps_vram_exact_and_renders_finer() {
    let Some((mut a, _)) = gpu_random(21) else { return };
    let Some((mut b, _)) = gpu_random(21) else { return };
    b.set_hires_scale(2);
    hires_scene(&mut a, 99, 300);
    hires_scene(&mut b, 99, 300);
    a.sync_vram();
    b.sync_vram();
    assert!(a.vram == b.vram, "N× copies changed VRAM");
    let (w, h, px) = hires_copy(&mut b, 0);
    assert!(w >= 1280 && h >= 896);
    // The copy averaged over each 2x2 block is close to the 1× frame.
    let fb = Surface::new(PSMCT32, 0, 10);
    let (mut sum, mut big) = (0f64, 0);
    for y in 0..448u32 {
        for x in 0..640u32 {
            let lo = fb.read(&b.vram, x, y);
            for c in 0..3 {
                let ch = |v: u32| ((v >> (8 * c)) & 0xFF) as f64;
                let avg = (0..4).map(|k| ch(px[((2 * y + k / 2) * w + 2 * x + k % 2) as usize])).sum::<f64>() / 4.0;
                let d = (avg - ch(lo)).abs();
                sum += d;
                big += (d > 48.0) as u32;
            }
        }
    }
    let mean = sum / (640.0 * 448.0 * 3.0);
    let nu = non_uniform(w, &px);
    eprintln!("hires 2x: mean |2x2 avg - 1x| {mean:.2}, channels off by >48: {big}, non-uniform blocks {:.1}%", nu * 100.0);
    assert!(mean < 6.0, "N× copy drifted from the 1× frame (mean {mean:.2})");
    assert!((big as f64) < 0.02 * 640.0 * 448.0 * 3.0, "too many large differences: {big}");
    assert!(nu > 0.05, "N× copy has no detail beyond 1× ({:.1}%)", nu * 100.0);
}

#[test]
fn hires_frame_as_texture_copies_exactly() {
    let Some((mut gs, _)) = gpu_random(22) else { return };
    gs.set_hires_scale(2);
    hires_scene(&mut gs, 7, 300);
    // Copy frame A (block 0) into frame B (page 200) with a 1:1 decal
    // sprite that samples A as a texture (nearest, FST).
    gs.write_reg(0x4C, 200 | 10 << 16);
    gs.write_reg(0x47, 0); // no Z
    gs.write_reg(0x06, 10 << 14 | 10 << 26 | 9 << 30 | 1 << 34 | 1 << 35);
    // Region clamp to the frame (the 1024x512 texture extent would reach
    // the Z buffer).
    gs.write_reg(0x08, 2 | 2 << 2 | 0 << 4 | 639 << 14 | 0 << 24 | 447u64 << 34);
    gs.write_reg(0x14, 0);
    gs.write_reg(0x00, 6 | 1 << 4 | 1 << 8);
    let mut v = [Vertex::default(); 2];
    v[0] = Vertex { x: 0, y: 0, rgba: [128; 4], u: 0, v: 0, q: 1.0, ..Default::default() };
    v[1] = Vertex { x: 640 * 16, y: 448 * 16, rgba: [128; 4], u: 640 * 16, v: 448 * 16, q: 1.0, ..Default::default() };
    gs.n3_capture = true;
    gs.draw_primitive(6, &v);
    gs.n3_capture = false;
    gs.sync_vram();
    let (w, _, a) = hires_copy(&mut gs, 0);
    let (_, _, b) = hires_copy(&mut gs, 200 * 32);
    let mut diff = 0;
    for y in 0..896u32 {
        for x in 0..1280u32 {
            let i = (y * w + x) as usize;
            diff += ((a[i] ^ b[i]) & 0xFF_FFFF != 0) as u32;
        }
    }
    eprintln!("hires copy: {diff} of {} N× pixels differ, B non-uniform {:.1}%", 1280 * 896, non_uniform(w, &b) * 100.0);
    assert_eq!(diff, 0, "a 1:1 copy of an N× frame must reproduce it exactly");
    // And 1× stays the GS's: B equals A in VRAM.
    let fa = Surface::new(PSMCT32, 0, 10);
    let fbb = Surface::new(PSMCT32, 200 * 32, 10);
    let mism = (0..448u32).flat_map(|y| (0..640u32).map(move |x| (x, y))).filter(|&(x, y)| fa.read(&gs.vram, x, y) & 0xFF_FFFF != fbb.read(&gs.vram, x, y) & 0xFF_FFFF).count();
    assert_eq!(mism, 0, "1× copy differs");
}

#[test]
fn hires_refreshes_after_vram_upload() {
    let Some((mut gs, _)) = gpu_random(23) else { return };
    gs.set_hires_scale(2);
    hires_scene(&mut gs, 5, 200);
    gs.sync_vram();
    // Overwrite a 64x32 rectangle of the frame from the host, then draw
    // one more native triangle elsewhere so a run picks the frame up again.
    gs.write_reg(0x50, 10u64 << 48);
    gs.write_reg(0x51, 100 | 50 << 16);
    gs.write_reg(0x52, 64 | 32 << 32);
    gs.write_reg(0x53, 0);
    let data: Vec<u32> = (0..64 * 32).map(|i| 0x0011_2233u32.wrapping_mul(i as u32 + 1) | 0x8000_0000).collect();
    let mut q: Vec<u128> = vec![(data.len() as u64 / 4 | 2 << 58) as u128 | 1 << 15];
    for c in data.chunks_exact(4) {
        q.push(c.iter().rev().fold(0u128, |acc, &v| acc << 32 | v as u128));
    }
    gs.gif_write(3, &q);
    hires_scene(&mut gs, 6, 1);
    gs.sync_vram();
    let (w, _, px) = hires_copy(&mut gs, 0);
    let fb = Surface::new(PSMCT32, 0, 10);
    let mut wrong = 0;
    for y in 50..82u32 {
        for x in 100..164u32 {
            let lo = fb.read(&gs.vram, x, y) & 0xFF_FFFF;
            // Unless the extra triangle covered it, the copy shows the upload.
            if lo == data[((y - 50) * 64 + x - 100) as usize] & 0xFF_FFFF {
                for k in 0..4u32 {
                    wrong += (px[((2 * y + k / 2) * w + 2 * x + k % 2) as usize] & 0xFF_FFFF != lo) as u32;
                }
            }
        }
    }
    assert_eq!(wrong, 0, "N× copy not refreshed after a VRAM upload");
}

/// Show the CT32 640x448 frame at block 0 (circuit 1 only).
fn show_frame0(gs: &mut Gs) -> (u32, u32, Vec<u32>) {
    gs.write_priv(0x00, 1);
    gs.write_priv(0x70, 10 << 9);
    gs.write_priv(0x80, 2559u64 << 32 | 447 << 44 | 3 << 23);
    let f = gs.display_gpu().expect("display");
    let px = gs.gpu.as_mut().unwrap().read_frame(&f);
    (f.width, f.height, px)
}

#[test]
fn hires_display_shows_the_n_copy() {
    let Some((mut gs, _)) = gpu_random(24) else { return };
    gs.set_hires_scale(2);
    hires_scene(&mut gs, 3, 300);
    let (w, h, px) = show_frame0(&mut gs);
    assert_eq!((w, h), (1280, 896), "display not at N×");
    let (cw, _, copy) = hires_copy(&mut gs, 0);
    let diff = (0..896u32).flat_map(|y| (0..1280u32).map(move |x| (x, y))).filter(|&(x, y)| px[(y * 1280 + x) as usize] != copy[(y * cw + x) as usize] | 0xFF00_0000).count();
    eprintln!("hires display: {diff} pixels differ from the N× copy, non-uniform {:.1}%", non_uniform(1280, &px) * 100.0);
    assert_eq!(diff, 0, "N× display differs from the N× copy");
    // Screenshots (`display`) stay 1× and exact.
    let mut lo = Vec::new();
    assert_eq!(gs.display(&mut lo), (640, 448));
    gs.sync_vram();
    let fb = Surface::new(PSMCT32, 0, 10);
    assert!((0..448u32).all(|y| (0..640u32).all(|x| lo[(y * 640 + x) as usize] == fb.read(&gs.vram, x, y) | 0xFF00_0000)), "1× display changed");
}

#[test]
fn hires_display_falls_back_to_vram_when_stale() {
    let Some((mut gs, _)) = gpu_random(25) else { return };
    gs.set_hires_scale(2);
    hires_scene(&mut gs, 4, 200);
    // A host upload into the frame after the last native run: the N× copy
    // no longer stands for it, so the display reads VRAM (at nearest).
    gs.write_reg(0x50, 10u64 << 48);
    gs.write_reg(0x51, 100 | 50 << 16);
    gs.write_reg(0x52, 64 | 32 << 32);
    gs.write_reg(0x53, 0);
    let data: Vec<u32> = (0..64 * 32).map(|i| 0x0011_2233u32.wrapping_mul(i as u32 + 1) | 0x8000_0000).collect();
    let mut q: Vec<u128> = vec![(data.len() as u64 / 4 | 2 << 58) as u128 | 1 << 15];
    for c in data.chunks_exact(4) {
        q.push(c.iter().rev().fold(0u128, |acc, &v| acc << 32 | v as u128));
    }
    gs.gif_write(3, &q);
    let (w, h, px) = show_frame0(&mut gs);
    assert_eq!((w, h), (1280, 896));
    gs.sync_vram();
    let fb = Surface::new(PSMCT32, 0, 10);
    let wrong = (0..896u32).flat_map(|y| (0..1280u32).map(move |x| (x, y))).filter(|&(x, y)| px[(y * 1280 + x) as usize] != fb.read(&gs.vram, x / 2, y / 2) | 0xFF00_0000).count();
    assert_eq!(wrong, 0, "stale N× copy shown");
}
