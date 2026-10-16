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
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [out, dumps @ ..] = args.as_slice() else {
        eprintln!("usage: gfx_replay OUT.png DUMP...");
        std::process::exit(2);
    };
    let ctx = ssx3_ps2::gs::gpu::context().expect("GPU");
    let r = ssx3_gfx::Renderer::new(ctx.device.clone(), ctx.queue.clone());
    let t = std::time::Instant::now();
    for (i, path) in dumps.iter().enumerate() {
        let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}"));
        let (mem, carry) = frame_sink::load_dump(&bytes).unwrap_or_else(|e| panic!("{path}: {e}"));
        let built = frame_sink::build_dump(&mem, &carry).unwrap_or_else(|| panic!("{path}: no frame"));
        let mut g = r.0.lock().unwrap();
        if i == 0 {
            g.seed_vu(&mem.vu1);
        }
        g.render(&built, &mem);
    }
    let (w, h, px) = r.read_image().expect("image");
    let img: Vec<u32> = px.iter().map(|p| p | 0xFF00_0000).collect();
    std::fs::write(out, png::encode(w, h, &img)).unwrap();
    eprintln!("{} frames in {:.0} ms -> {out} ({w}x{h})", dumps.len(), t.elapsed().as_secs_f64() * 1e3);
}
