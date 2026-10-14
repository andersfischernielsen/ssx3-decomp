//! Replay a GS trace recorded with `Gs::start_capture` (e.g. via the headless
//! runner's `SSX3_GS_CAPTURE`), report the time per frame, verify VRAM against
//! golden hashes and write the final displayed image as a PNG.
//!
//! ```sh
//! gs_replay TRACE [--sync] [--iters N] [--png OUT.png] [--write-golden FILE] [--golden FILE]
//! ```
//!
//! A "frame" is everything between two VBlank events. After each VBlank (and at
//! the end of the trace) a 64-bit hash of all of VRAM is taken; `--write-golden`
//! stores those hashes, `--golden` compares against them (exit status 1 on any
//! mismatch).

mod png;

use ssx3_ps2::gs::capture::{Event, Trace};
use ssx3_ps2::gs::Gs;
use std::time::Instant;

fn vram_hash(gs: &Gs) -> u64 {
    // FNV-1a over 64-bit lanes, then a final avalanche.
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for pair in gs.vram.chunks_exact(2) {
        let v = pair[0] as u64 | (pair[1] as u64) << 32;
        h = (h ^ v).wrapping_mul(0x0000_0100_0000_01B3);
    }
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
    h ^ (h >> 33)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let Some(path) = args.get(1) else {
        eprintln!(
            "usage: gs_replay TRACE [--iters N] [--png OUT.png] [--write-golden FILE] [--golden FILE]"
        );
        std::process::exit(2);
    };
    let opt = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let iters: usize = opt("--iters").and_then(|s| s.parse().ok()).unwrap_or(1).max(1);
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}"));

    let mut best_frames: Vec<f64> = Vec::new();
    let mut best_total = f64::MAX;
    let mut hashes = Vec::new();
    let mut last: Option<Gs> = None;
    let mut prims = 0;
    let mut nev = 0;
    for _ in 0..iters {
        let Trace { mut gs, events } =
            Trace::parse(&bytes).unwrap_or_else(|e| panic!("{path}: {e}"));
        nev = events.len();
        gs.set_deferred(!args.iter().any(|a| a == "--sync"));
        let p0 = gs.prims_drawn;
        hashes.clear();
        let mut frames = Vec::new();
        let mut total = 0.0;
        let mut t = Instant::now();
        for ev in &events {
            ev.apply(&mut gs);
            if let Event::VBlank { .. } = ev {
                gs.sync();
                let ms = t.elapsed().as_secs_f64() * 1e3;
                frames.push(ms);
                total += ms;
                hashes.push(vram_hash(&gs));
                t = Instant::now();
            }
        }
        gs.sync();
        let ms = t.elapsed().as_secs_f64() * 1e3;
        frames.push(ms);
        total += ms;
        hashes.push(vram_hash(&gs));
        if total < best_total {
            best_total = total;
            best_frames = frames;
        }
        prims = gs.prims_drawn - p0;
        last = Some(gs);
    }
    let gs = last.unwrap();
    println!(
        "{path}: {nev} events, {} vblanks, {prims} prims, best of {iters}: {best_total:.1} ms total",
        hashes.len() - 1
    );
    println!("{:?}", gs.batch_stats());
    let fr: Vec<String> = best_frames.iter().map(|f| format!("{f:.1}")).collect();
    println!("ms per frame segment: [{}]", fr.join(", "));
    // The first and last segments are partial frames.
    let full: &[f64] = if best_frames.len() > 2 {
        &best_frames[1..best_frames.len() - 1]
    } else {
        &best_frames
    };
    println!(
        "mean ms/frame (full frames): {:.2}",
        full.iter().sum::<f64>() / full.len() as f64
    );

    let text: String = hashes.iter().map(|h| format!("{h:016x}\n")).collect();
    if let Some(out) = opt("--write-golden") {
        std::fs::write(&out, &text).unwrap_or_else(|e| panic!("{out}: {e}"));
        println!("wrote {} hashes to {out}", hashes.len());
    }
    let mut ok = true;
    if let Some(g) = opt("--golden") {
        let want = std::fs::read_to_string(&g).unwrap_or_else(|e| panic!("{g}: {e}"));
        let want: Vec<&str> = want.lines().collect();
        let got: Vec<String> = hashes.iter().map(|h| format!("{h:016x}")).collect();
        if want.len() != got.len() {
            ok = false;
            println!("golden: hash count differs ({} vs {})", got.len(), want.len());
        }
        for (i, (a, b)) in got.iter().zip(&want).enumerate() {
            if a != b {
                ok = false;
                println!("golden: MISMATCH after segment {i}: {a} != {b}");
            }
        }
        if ok {
            println!("golden: all {} VRAM hashes match", got.len());
        }
    }
    if let Some(out) = opt("--png") {
        let mut img = Vec::new();
        let (w, h) = gs.display(&mut img);
        std::fs::write(&out, png::encode(w, h, &img)).unwrap_or_else(|e| panic!("{out}: {e}"));
        println!("wrote {w}x{h} {out}");
    }
    if !ok {
        std::process::exit(1);
    }
}
