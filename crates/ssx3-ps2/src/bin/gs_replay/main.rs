//! Replay a GS trace recorded with `Gs::start_capture` (e.g. via the headless
//! runner's `SSX3_GS_CAPTURE`), report the time per frame, verify VRAM against
//! golden hashes and write the final displayed image as a PNG.
//!
//! ```sh
//! gs_replay TRACE [--gpu] [--sync] [--iters N] [--png OUT.png] [--write-golden FILE] [--golden FILE]
//! gs_replay TRACE --compare [--diff-png OUT.png] [--png-cpu A.png] [--png OUT.png]
//! ```
//!
//! A "frame" is everything between two VBlank events. After each VBlank (and at
//! the end of the trace) a 64-bit hash of all of VRAM is taken; `--write-golden`
//! stores those hashes, `--golden` compares against them (exit status 1 on any
//! mismatch).
//!
//! `--gpu` replays on the GPU backend (`gs::gpu`). Frame times then include
//! waiting for the GPU at every VBlank, and VRAM is only hashed (which reads
//! it back) with `--golden` / `--write-golden`.
//!
//! `--compare` replays on both backends side by side and compares the
//! displayed image after every VBlank and at the end: pixels that differ at
//! all, pixels whose largest channel difference exceeds 8 and 32, and the
//! mean absolute channel difference; plus the fraction of VRAM words that
//! differ at the end. `--diff-dir DIR` also writes the CPU, GPU and
//! difference images of every frame.

mod png;

use ssx3_ps2::gs::capture::{Event, Trace};
use ssx3_ps2::gs::{Backend, Gs};
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

#[derive(Default, Debug, Clone, Copy)]
struct Diff {
    pixels: u64,
    any: u64,
    over8: u64,
    over32: u64,
    abs_sum: u64,
}

impl Diff {
    fn add(&mut self, o: &Diff) {
        self.pixels += o.pixels;
        self.any += o.any;
        self.over8 += o.over8;
        self.over32 += o.over32;
        self.abs_sum += o.abs_sum;
    }
    fn report(&self) -> String {
        let p = self.pixels.max(1) as f64;
        format!(
            "{} px: differ {:.3}%, >8 {:.3}%, >32 {:.3}%, mean |d| {:.4}/channel",
            self.pixels,
            100.0 * self.any as f64 / p,
            100.0 * self.over8 as f64 / p,
            100.0 * self.over32 as f64 / p,
            self.abs_sum as f64 / (p * 3.0)
        )
    }
}

fn diff_images(a: &[u32], b: &[u32], out: Option<&mut Vec<u32>>) -> Diff {
    let mut d = Diff {
        pixels: a.len() as u64,
        ..Diff::default()
    };
    let mut img = Vec::new();
    for (&x, &y) in a.iter().zip(b) {
        let mut m = 0u32;
        for sh in [0, 8, 16] {
            let e = ((x >> sh) & 0xFF).abs_diff((y >> sh) & 0xFF);
            m = m.max(e);
            d.abs_sum += e as u64;
        }
        if m > 0 {
            d.any += 1;
        }
        if m > 8 {
            d.over8 += 1;
        }
        if m > 32 {
            d.over32 += 1;
        }
        if out.is_some() {
            // Differences in red over a dimmed reference.
            let g = ((x & 0xFF) + ((x >> 8) & 0xFF) + ((x >> 16) & 0xFF)) / 12;
            img.push(if m > 0 {
                0xFF00_0000 | (128 + m.min(127) as u32)
            } else {
                0xFF00_0000 | g | g << 8 | g << 16
            });
        }
    }
    if let Some(o) = out {
        *o = img;
    }
    d
}

fn compare(path: &str, bytes: &[u8], opt: &dyn Fn(&str) -> Option<String>) -> bool {
    let Trace { gs: mut cpu, events } = Trace::parse(bytes).unwrap_or_else(|e| panic!("{path}: {e}"));
    let Trace { gs: mut gpu, .. } = Trace::parse(bytes).unwrap_or_else(|e| panic!("{path}: {e}"));
    cpu.set_deferred(true);
    gpu.set_deferred(true);
    if !gpu.set_backend(Backend::Gpu) {
        eprintln!("no GPU available");
        return false;
    }
    let (mut a, mut b) = (Vec::new(), Vec::new());
    let mut total = Diff::default();
    let mut frame = 0;
    // --diff-dir DIR: CPU, GPU and difference images after every VBlank.
    let dir = opt("--diff-dir");
    let mut cmp = |cpu: &mut Gs, gpu: &mut Gs, label: &str| {
        let sa = cpu.display(&mut a);
        let sb = gpu.display(&mut b);
        if sa != sb {
            println!("{label}: display size differs {sa:?} vs {sb:?}");
            return;
        }
        let mut img = Vec::new();
        let d = diff_images(&a, &b, dir.as_ref().map(|_| &mut img));
        println!("{label}: {}", d.report());
        total.add(&d);
        if let Some(dir) = &dir {
            let name = label.replace(' ', "");
            let (w, h) = sa;
            for (suffix, im) in [("cpu", &a), ("gpu", &b), ("diff", &img)] {
                let out = format!("{dir}/{name}_{suffix}.png");
                std::fs::write(&out, png::encode(w, h, im)).unwrap_or_else(|e| panic!("{out}: {e}"));
            }
        }
    };
    for ev in &events {
        ev.apply(&mut cpu);
        ev.apply(&mut gpu);
        if let Event::VBlank { .. } = ev {
            cmp(&mut cpu, &mut gpu, &format!("vblank {frame}"));
            frame += 1;
        }
    }
    cmp(&mut cpu, &mut gpu, "end");
    println!("all frames: {}", total.report());
    cpu.sync();
    gpu.sync_vram();
    let words = cpu.vram.iter().zip(&gpu.vram).filter(|(x, y)| x != y).count();
    println!(
        "final VRAM: {words} of {} words differ ({:.3}%)",
        cpu.vram.len(),
        100.0 * words as f64 / cpu.vram.len() as f64
    );
    println!("{:?}", gpu.gpu_stats().unwrap());
    let (w, h) = cpu.display(&mut a);
    gpu.display(&mut b);
    if let Some(out) = opt("--png-cpu") {
        std::fs::write(&out, png::encode(w, h, &a)).unwrap_or_else(|e| panic!("{out}: {e}"));
    }
    if let Some(out) = opt("--png") {
        std::fs::write(&out, png::encode(w, h, &b)).unwrap_or_else(|e| panic!("{out}: {e}"));
    }
    if let Some(out) = opt("--diff-png") {
        let mut img = Vec::new();
        diff_images(&a, &b, Some(&mut img));
        std::fs::write(&out, png::encode(w, h, &img)).unwrap_or_else(|e| panic!("{out}: {e}"));
    }
    true
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let Some(path) = args.get(1) else {
        eprintln!(
            "usage: gs_replay TRACE [--gpu] [--iters N] [--png OUT.png] [--write-golden FILE] [--golden FILE]\n       gs_replay TRACE --compare [--png-cpu A.png] [--png B.png] [--diff-png D.png]"
        );
        std::process::exit(2);
    };
    let opt = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let flag = |name: &str| args.iter().any(|a| a == name);
    let iters: usize = opt("--iters").and_then(|s| s.parse().ok()).unwrap_or(1).max(1);
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    if flag("--compare") {
        std::process::exit(if compare(path, &bytes, &opt) { 0 } else { 1 });
    }
    let gpu = flag("--gpu");
    // Measure only the CPU side of the GPU backend (submission, no waiting).
    let no_wait = flag("--no-gpu-wait");
    let hash = !gpu || opt("--golden").is_some() || opt("--write-golden").is_some();

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
        gs.set_deferred(!flag("--sync"));
        if gpu {
            if !gs.set_backend(Backend::Gpu) {
                eprintln!("no GPU available");
                std::process::exit(1);
            }
            // Upload the initial VRAM before timing.
            gs.gpu_wait_idle();
        }
        let p0 = gs.prims_drawn;
        hashes.clear();
        let mut frames = Vec::new();
        let mut total = 0.0;
        let mut t = Instant::now();
        let mut end_frame = |gs: &mut Gs, frames: &mut Vec<f64>, total: &mut f64, t: &mut Instant| {
            gs.sync();
            if !no_wait {
                gs.gpu_wait_idle();
            }
            let ms = t.elapsed().as_secs_f64() * 1e3;
            frames.push(ms);
            *total += ms;
            if hash {
                gs.sync_vram();
                hashes.push(vram_hash(gs));
            }
            *t = Instant::now();
        };
        for ev in &events {
            ev.apply(&mut gs);
            if let Event::VBlank { .. } = ev {
                end_frame(&mut gs, &mut frames, &mut total, &mut t);
            }
        }
        end_frame(&mut gs, &mut frames, &mut total, &mut t);
        if total < best_total {
            best_total = total;
            best_frames = frames;
        }
        prims = gs.prims_drawn - p0;
        last = Some(gs);
    }
    let mut gs = last.unwrap();
    println!(
        "{path}: {nev} events, {} vblanks, {prims} prims, best of {iters}: {best_total:.1} ms total",
        best_frames.len() - 1
    );
    match gs.gpu_stats() {
        Some(s) => println!("{s:?}"),
        None => println!("{:?}", gs.batch_stats()),
    }
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
