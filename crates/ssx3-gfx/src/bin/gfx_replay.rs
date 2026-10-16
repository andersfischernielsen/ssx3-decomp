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
            frame_sink::load_dump(&bytes).unwrap_or_else(|e| panic!("{path}: {e}"))
        })
        .collect();
    if let Some((w, h)) = std::env::var("SSX3_RENDER_SIZE").ok().and_then(|v| v.split_once('x').and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?)))) {
        r.set_output_size(Some((w, h)));
    }
    if let Some(n) = std::env::var("SSX3_RENDER_SCALE").ok().and_then(|v| v.parse::<u32>().ok()) {
        r.set_scale(n);
    }
    if std::env::var_os("SSX3_REPLAY_DEPTH").is_some() {
        r.set_want_depth(true);
    }
    let mut t = std::time::Instant::now();
    let mut times = Vec::new();
    for rep in 0..repeat {
        if rep == 1 {
            t = std::time::Instant::now();
        }
        for (i, (mem, carry)) in frames.iter().enumerate() {
            let t0 = std::time::Instant::now();
            let built = frame_sink::build_dump(mem, carry).expect("no frame");
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
    let dumps = &dumps[..];
    let (w, h, px) = r.read_image().expect("image");
    let img: Vec<u32> = px.iter().map(|p| p | 0xFF00_0000).collect();
    std::fs::write(out, png::encode(w, h, &img)).unwrap();
    eprintln!("{} frames in {:.0} ms -> {out} ({w}x{h})", dumps.len(), t.elapsed().as_secs_f64() * 1e3);
}
