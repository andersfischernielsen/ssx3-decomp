//! Replay a GS trace recorded with `Gs::start_capture` (e.g. via the headless
//! runner's `SSX3_GS_CAPTURE`), report the time per frame, verify VRAM against
//! golden hashes and write the final displayed image as a PNG.
//!
//! ```sh
//! gs_replay TRACE [--gpu] [--sync] [--iters N] [--png OUT.png] [--write-golden FILE] [--golden FILE]
//! gs_replay TRACE --compare [--diff-png OUT.png] [--png-cpu A.png] [--png OUT.png]
//! gs_replay TRACE --draws DIR [--events]
//! gs_replay TRACE --packets [--far PIXELS]
//! ```
//!
//! `--packets` lists every PATH1/PATH2 PACKED GIF tag with its vertices in
//! window coordinates (relative to XYOFFSET_1), Z, Q and ADC; vertices far
//! outside the window are marked `FAR`. Negative Q on a drawn vertex means
//! the VU kicked a point behind the eye.
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

/// `--draws DIR`: replay on the CPU, log every draw setup, and per frame
/// segment write `DIR/fNN.txt` (one line per setup) plus `DIR/fNN.png`
/// (the displayed image after the segment) and each distinct texture as
/// `DIR/tex_<tex0>_<hash>.png`.
fn draws(path: &str, bytes: &[u8], dir: &str) {
    let Trace { mut gs, events } = Trace::parse(bytes).unwrap_or_else(|e| panic!("{path}: {e}"));
    std::fs::create_dir_all(dir).unwrap();
    gs.draw_log = Some(Vec::new());
    let mut seg = 0;
    let mut written = std::collections::HashSet::new();
    let mut dump = |gs: &mut Gs, seg: usize| {
        gs.sync();
        let log = gs.draw_log.replace(Vec::new()).unwrap();
        let mut txt = String::new();
        for (i, e) in log.iter().enumerate() {
            let t = e.tex0;
            let mut name = String::from("-");
            if let Some((w, h, img)) = &e.image {
                let mut hsh: u64 = 0xcbf2_9ce4_8422_2325;
                for &v in img {
                    hsh = (hsh ^ v as u64).wrapping_mul(0x100_0000_01B3);
                }
                name = format!("tex_{t:016x}_{:08x}.png", hsh as u32);
                if written.insert(name.clone()) {
                    let opaque: Vec<u32> = img.iter().map(|v| v | 0xFF00_0000).collect();
                    std::fs::write(format!("{dir}/{name}"), png::encode(*w, *h, &opaque)).unwrap();
                    let alpha: Vec<u32> = img.iter().map(|v| { let a = v >> 24; a | a << 8 | a << 16 | 0xFF00_0000 }).collect();
                    std::fs::write(format!("{dir}/alpha-{name}"), png::encode(*w, *h, &alpha)).unwrap();
                }
            }
            txt += &format!(
                "{i:5} prim {:#05x} prims {:5} bbox {:?} tbp {:#x} tbw {} psm {:#x} tw {} th {} tcc {} tfx {} cbp {:#x} cpsm {:#x} csm {} csa {} cld {} | tex1 {:#x} clamp {:#x} frame {:#x} alpha {:#x} test {:#x} {name}\n",
                e.prim, e.prims, e.bbox,
                t & 0x3fff, (t >> 14) & 0x3f, (t >> 20) & 0x3f, (t >> 26) & 0xf, (t >> 30) & 0xf,
                (t >> 34) & 1, (t >> 35) & 3, (t >> 37) & 0x3fff, (t >> 51) & 0xf, (t >> 55) & 1,
                (t >> 56) & 0x1f, (t >> 61) & 7, e.tex1, e.clamp, e.frame, e.alpha, e.test
            );
        }
        std::fs::write(format!("{dir}/f{seg:02}.txt"), txt).unwrap();
        let mut img = Vec::new();
        let (w, h) = gs.display(&mut img);
        std::fs::write(format!("{dir}/f{seg:02}.png"), png::encode(w, h, &img)).unwrap();
    };
    // --events: also print the event stream with the draw setups it creates
    // (and, with SSX3_TRACE_XFER, the transfers) in order.
    let verbose = std::env::args().any(|a| a == "--events");
    for (ei, ev) in events.iter().enumerate() {
        let before = gs.draw_log.as_ref().map_or(0, |l| l.len());
        ev.apply(&mut gs);
        if verbose {
            match ev {
                Event::Gif { path, data } => println!("[ev {ei}] gif path {path} {} qw", data.len()),
                Event::Reg { addr, value } => println!("[ev] reg {addr:#x} = {value:#x}"),
                Event::Priv { offset, value } => println!("[ev] priv {offset:#x} = {value:#x}"),
                Event::VBlank { .. } => println!("[ev] vblank (segment {seg})"),
                _ => {}
            }
            for e in &gs.draw_log.as_ref().unwrap()[before..] {
                let t = e.tex0;
                println!(
                    "[draw] prim {:#x} tbp {:#x} psm {:#x} cbp {:#x} tme {} texa {:#x}",
                    e.prim, t & 0x3fff, (t >> 20) & 0x3f, (t >> 37) & 0x3fff, (e.prim >> 4) & 1, gs.persistent_regs()[0].1
                );
            }
        }
        if let Event::VBlank { .. } = ev {
            dump(&mut gs, seg);
            seg += 1;
        }
    }
    dump(&mut gs, seg);
    println!("{seg} segments written to {dir}");
}

/// `--packets`: decode every PATH1/PATH2 PACKED GIF packet in the trace and
/// print each tag with its vertices (window coordinates relative to
/// XYOFFSET_1, Z, Q, ADC). Vertices further than `--far N` pixels (default
/// 2048) outside the 640x448 window are marked `FAR`.
fn packets(path: &str, bytes: &[u8], far: f64) {
    let Trace { events, .. } = Trace::parse(bytes).unwrap_or_else(|e| panic!("{path}: {e}"));
    let mut ofs = [(0f64, 0f64); 2];
    let mut seg = 0;
    for (ei, ev) in events.iter().enumerate() {
        let (p, data) = match ev {
            Event::Gif { path, data } if *path != 3 => (*path, data),
            Event::VBlank { .. } => {
                seg += 1;
                continue;
            }
            _ => continue,
        };
        let mut i = 0;
        while i < data.len() {
            let tag = data[i];
            i += 1;
            let nloop = (tag & 0x7fff) as usize;
            let pre = (tag >> 46) & 1 != 0;
            let prim = ((tag >> 47) & 0x7ff) as u64;
            let flg = ((tag >> 58) & 3) as u32;
            let nreg = match ((tag >> 60) & 0xf) as usize {
                0 => 16,
                n => n,
            };
            let regs = (tag >> 64) as u64;
            if flg != 0 {
                // REGLIST / IMAGE: skip.
                let n = if flg == 1 { (nloop * nreg).div_ceil(2) } else { nloop };
                i += n;
                continue;
            }
            let mut cur_prim = if pre { prim } else { u64::MAX };
            let mut q = 1.0f32;
            let mut verts = Vec::new();
            let mut ad = Vec::new();
            for _ in 0..nloop {
                for r in 0..nreg {
                    if i >= data.len() {
                        break;
                    }
                    let d = data[i];
                    i += 1;
                    match (regs >> (4 * r)) & 0xf {
                        0 => cur_prim = d as u64 & 0x7ff,
                        2 => q = f32::from_bits((d >> 64) as u32),
                        4 | 5 => {
                            let x = (d & 0xffff) as u32;
                            let y = ((d >> 32) & 0xffff) as u32;
                            let z = ((d >> 64) & 0xffff_ffff) as u32;
                            let adc = (d >> 111) & 1 != 0;
                            verts.push((x, y, z, q, adc));
                        }
                        0xe => {
                            let a = ((d >> 64) & 0xff) as u8;
                            let v = d as u64;
                            if a == 0x18 || a == 0x19 {
                                ofs[(a - 0x18) as usize] =
                                    ((v & 0xffff) as f64 / 16.0, ((v >> 32) & 0xffff) as f64 / 16.0);
                            }
                            ad.push((a, v));
                        }
                        _ => {}
                    }
                }
            }
            let ctx = if cur_prim != u64::MAX { ((cur_prim >> 9) & 1) as usize } else { 0 };
            let (ox, oy) = ofs[ctx];
            let mut line = format!("seg {seg} ev {ei} path {p} prim {cur_prim:#x} nloop {nloop} nreg {nreg}");
            for (a, v) in &ad {
                line += &format!(" ad[{a:#x}]={v:#x}");
            }
            println!("{line}");
            for (k, (x, y, z, q, adc)) in verts.iter().enumerate() {
                let sx = *x as f64 / 16.0 - ox;
                let sy = *y as f64 / 16.0 - oy;
                let is_far = sx < -far || sx > 640.0 + far || sy < -far || sy > 448.0 + far;
                println!(
                    "  v{k:3} x {sx:9.2} y {sy:9.2} z {z:#010x} q {q:12.6e} adc {}{}",
                    *adc as u8,
                    if is_far { " FAR" } else { "" }
                );
            }
        }
    }
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
    if flag("--packets") {
        let far = opt("--far").and_then(|s| s.parse().ok()).unwrap_or(2048.0);
        packets(path, &bytes, far);
        return;
    }
    if let Some(dir) = opt("--draws") {
        draws(path, &bytes, &dir);
        return;
    }
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
