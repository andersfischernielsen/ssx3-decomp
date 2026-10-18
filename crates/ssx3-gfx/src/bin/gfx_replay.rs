//! Render frame dumps (`SSX3_FRAME_DUMP`, written by the headless runner
//! next to its screenshots) with `ssx3-gfx`, without running the game.
//!
//! ```sh
//! gfx_replay OUT.png frame-2.bin frame-1.bin frame-0.bin
//! ```
//!
//! The dumps are rendered in the order given (oldest first, so the VU memory
//! image and GS state carried between frames warm up; the first one seeds
//! the VU1 memory image from the emulator's), and the last frame's
//! image is written to `OUT.png`. The `SSX3_GFX_*` debug switches apply;
//! frame numbers for `SSX3_GFX_TRACE` count the dumps from 1.

#[path = "../../../ssx3-ps2/src/bin/gs_replay/png.rs"]
mod png;

use ssx3_ps2::frame_sink;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    // --repeat N: render the dumps N times (steady-state timing).
    let repeat = match args.iter().position(|a| a == "--repeat") {
        Some(i) => {
            let n = args.get(i + 1).and_then(|v| v.parse().ok()).unwrap_or(1);
            args.drain(i..(i + 2).min(args.len()));
            n
        }
        None => 1usize,
    };
    // --interp K: K images per frame (K - 1 interpolated); all of the last
    // frame's are written as OUT-0.png .. OUT-{K-1}.png too.
    let interp = match args.iter().position(|a| a == "--interp") {
        Some(i) => {
            let n = args.get(i + 1).and_then(|v| v.parse().ok()).unwrap_or(1);
            args.drain(i..(i + 2).min(args.len()));
            n
        }
        None => 1u32,
    };
    let [out, dumps @ ..] = args.as_slice() else {
        eprintln!("usage: gfx_replay OUT.png DUMP...");
        std::process::exit(2);
    };
    let ctx = ssx3_ps2::gs::gpu::context().expect("GPU");
    let r = ssx3_gfx::Renderer::new(ctx.device.clone(), ctx.queue.clone());
    let frames: Vec<_> = dumps
        .iter()
        .map(|path| {
            let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}"));
            let (mut mem, carry) = frame_sink::load_dump(&bytes).unwrap_or_else(|e| panic!("{path}: {e}"));
            // SSX3_DIRECT=0: decode every packet (the captured draws unused).
            if std::env::var("SSX3_DIRECT").is_ok_and(|v| v == "0") {
                mem.direct = Default::default();
            }
            (mem, carry)
        })
        .collect();
    if let Some((w, h)) = std::env::var("SSX3_RENDER_SIZE").ok().and_then(|v| v.split_once('x').and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?)))) {
        r.set_output_size(Some((w, h)));
    }
    if let Some(n) = std::env::var("SSX3_RENDER_SCALE").ok().and_then(|v| v.parse::<u32>().ok()) {
        r.set_scale(n);
    }
    r.set_subframes(interp);
    if std::env::var_os("SSX3_REPLAY_DEPTH").is_some() {
        r.set_want_depth(true);
    }
    let mut t = std::time::Instant::now();
    let mut times = Vec::new();
    // (Kept across repeats: the steady state the game sees.)
    let mut chains = ssx3_render::frame::ChainCache::default();
    for rep in 0..repeat {
        if rep == 1 {
            t = std::time::Instant::now();
        }
        for (i, (mem, carry)) in frames.iter().enumerate() {
            let t0 = std::time::Instant::now();
            let built = frame_sink::build_dump_cached(mem, carry, &mut chains).expect("no frame");
            if std::env::var_os("SSX3_CHAIN_CACHE_CHECK").is_some() {
                let bad = built.issues.iter().filter(|i| i.starts_with("chain cache")).count();
                eprintln!("[chain-cache] frame {i}: hits/misses so far {:?}, {bad} replay mismatches", chains.stats);
            }
            let t1 = std::time::Instant::now();
            let mut g = r.0.lock().unwrap();
            if i == 0 && rep == 0 {
                g.seed_vu(&mem.vu1);
            }
            g.render(&built, mem);
            if rep > 0 {
                times.push(((t1 - t0).as_secs_f64() * 1e3, t1.elapsed().as_secs_f64() * 1e3));
            }
        }
    }
    if !times.is_empty() {
        let n = times.len() as f64;
        let (b, rr) = times.iter().fold((0.0, 0.0), |a, t| (a.0 + t.0, a.1 + t.1));
        eprintln!("steady state: frame build {:.2} ms, render {:.2} ms per frame", b / n, rr / n);
        eprintln!("{}", r.0.lock().unwrap().timing_report());
    }
    if std::env::var_os("SSX3_GPU_STATS").is_some() {
        let s = r.stats();
        eprintln!("[gpu-stats] native draws {}, VU calls {} ({} without a program), batches {}, frames {}", s.native_draws, s.vu_calls, s.vu_missing, s.batches, s.frames);
        for ((h, e, i, g), n) in ssx3_render::gpu::STATS.lock().unwrap().iter() {
            eprintln!("[gpu-stats] {h:016x} entry {e:#06x} itop {i} gpu {g}: {n}");
        }
    }
    let dumps = &dumps[..];
    let (w, h, px) = r.read_image().expect("image");
    let img: Vec<u32> = px.iter().map(|p| p | 0xFF00_0000).collect();
    std::fs::write(out, png::encode(w, h, &img)).unwrap();
    if interp > 1 {
        for (i, (w, h, px)) in r.read_presents().into_iter().enumerate() {
            let img: Vec<u32> = px.iter().map(|p| p | 0xFF00_0000).collect();
            let name = format!("{}-{i}.png", out.trim_end_matches(".png"));
            std::fs::write(&name, png::encode(w, h, &img)).unwrap();
        }
    }
    // SSX3_DUMP_RTS=DIR: the render targets too, as DIR/rt<fbp>.png (RGB)
    // and DIR/rt<fbp>-a.png (alpha, stored as a/128).
    if let Ok(dir) = std::env::var("SSX3_DUMP_RTS") {
        std::fs::create_dir_all(&dir).unwrap();
        for (fbp, w, h, px) in r.read_rts() {
            let rgb: Vec<u32> = px.iter().map(|p| p | 0xFF00_0000).collect();
            std::fs::write(format!("{dir}/rt{fbp}.png"), png::encode(w, h, &rgb)).unwrap();
            let a: Vec<u32> = px.iter().map(|p| { let a = p >> 24; a | a << 8 | a << 16 | 0xFF00_0000 }).collect();
            std::fs::write(format!("{dir}/rt{fbp}-a.png"), png::encode(w, h, &a)).unwrap();
        }
    }
    eprintln!("{} frames in {:.0} ms -> {out} ({w}x{h})", dumps.len(), t.elapsed().as_secs_f64() * 1e3);
}
