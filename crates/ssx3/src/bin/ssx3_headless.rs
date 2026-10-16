//! Run the recompiled game without a window, for N frames, printing status and
//! optionally writing screenshots.
//!
//! ```sh
//! cargo run --profile game -p ssx3 --bin ssx3-headless -- 600 --shot-every 60
//! ```

use std::time::Instant;

#[path = "../game.rs"]
mod game;
#[path = "../png.rs"]
mod png;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let frames: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(300);
    let slow_ms: Option<f64> = std::env::var("SSX3_SLOW_FRAME_MS").ok().and_then(|v| v.parse().ok());
    let started = std::time::Instant::now();
    let shot_every: u64 = args
        .iter()
        .position(|a| a == "--shot-every")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let shot_dir = std::env::var("SSX3_SHOTS").unwrap_or_else(|_| "/tmp/ssx3-shots".into());
    let _ = std::fs::create_dir_all(&shot_dir);

    // Headless runs do not open the audio device unless asked (SSX3_AUDIO=1);
    // SSX3_WAV=file.wav records the IOP's audio output.
    if std::env::var_os("SSX3_AUDIO").is_none() {
        std::env::set_var("SSX3_AUDIO", "0");
    }
    let mut c = game::boot(true);
    let gfx = game::install_renderer();
    let t0 = Instant::now();
    let mut img = Vec::new();
    // Scripted input: SSX3_INPUT="300:4000:10,400:0008:10" presses Cross (0x4000)
    // at frame 300 for 10 frames, then Start (0x0008) at frame 400. An optional
    // fourth field picks the pad (1 or 2; default 1): "500:0008:8:2".
    let script: Vec<(u64, u16, u64, usize)> = std::env::var("SSX3_INPUT")
        .unwrap_or_default()
        .split(',')
        .filter_map(|e| {
            let p: Vec<&str> = e.split(':').collect();
            Some((
                p.first()?.parse().ok()?,
                u16::from_str_radix(p.get(1)?, 16).ok()?,
                p.get(2).and_then(|s| s.parse().ok()).unwrap_or(6),
                p.get(3).and_then(|s| s.parse::<usize>().ok()).map_or(0, |n| (n.max(1) - 1) & 1),
            ))
        })
        .collect();
    // GS trace capture: SSX3_GS_CAPTURE="2380:4:/path/a.gstrace,4050:4:/path/b.gstrace"
    // records frames 2380..2383 (and 4050..4053) for offline replay with
    // `gs_replay`.
    let captures: Vec<(u64, u64, String)> = std::env::var("SSX3_GS_CAPTURE")
        .unwrap_or_default()
        .split(',')
        .filter_map(|e| {
            let mut p = e.splitn(3, ':');
            Some((p.next()?.parse().ok()?, p.next()?.parse().ok()?, p.next()?.to_string()))
        })
        .collect();
    for f in 1..=frames {
        for (start, _, _) in &captures {
            if f == *start {
                ssx3_ps2::m!(c).hw.gs.core().start_capture();
            }
        }
        let mut held = [0u16; 2];
        for (at, bits, len, pad) in &script {
            if f >= *at && f < at + len {
                held[*pad] |= bits;
            }
        }
        ssx3_ps2::m!(c).sif.pad_buttons = held;
        ssx3_ps2::vu::capture::set_frame(f);
        let alive = ssx3_ps2::machine::run_frame(&mut c);
        ssx3_ee_rt::prof::frame(f);
        // Test knob for host clock mode: a host this many ms slower per frame.
        if let Some(ms) = slow_ms {
            std::thread::sleep(std::time::Duration::from_micros((ms * 1000.0) as u64));
        }
        if ssx3_ps2::gifstats::enabled() && f % 50 == 0 {
            let (pk, v, adc, far) = ssx3_ps2::gifstats::take();
            eprintln!("[gif] frames {}..{f}: path1 packets {pk} verts {v} adc {adc} far {far}", f - 50);
        }
        for (start, n, path) in &captures {
            if f + 1 == start + n || (!alive && f >= *start && f < start + n) {
                let trace = ssx3_ps2::m!(c).hw.gs.core().finish_capture();
                match std::fs::write(path, &trace) {
                    Ok(()) => eprintln!("gs capture: frames {start}..{f} -> {path} ({} bytes)", trace.len()),
                    Err(e) => eprintln!("gs capture: {path}: {e}"),
                }
            }
        }
        if f % 60 == 0 || !alive {
            eprintln!("[{:6.1}s] {}", t0.elapsed().as_secs_f64(), game::status(&c));
            if let Some(s) = ssx3_ps2::iop_bridge::status(&c) {
                eprintln!("          {s}");
            }
        }
        if shot_every > 0 && f % shot_every == 0 {
            let path = format!("{shot_dir}/frame{f:05}.png");
            // Native: the draw-layer renderer's image is the one shown.
            match gfx.as_ref().filter(|_| ssx3_ps2::frame_sink::native_active()).and_then(|g| g.read_image()) {
                Some((w, h, px)) => {
                    let img: Vec<u32> = px.iter().map(|p| p | 0xFF00_0000).collect();
                    let _ = std::fs::write(&path, png::encode(w, h, &img));
                }
                None => {
                    let (w, h) = ssx3_ps2::m!(c).hw.gs.core().display(&mut img);
                    let _ = std::fs::write(&path, png::encode(w, h, &img));
                }
            }
            // SSX3_RENDER=shadow: the draw-layer renderer's image too.
            if let Some((w, h, px)) = gfx.as_ref().filter(|_| !ssx3_ps2::frame_sink::native()).and_then(|g| g.read_image()) {
                let img: Vec<u32> = px.iter().map(|p| p | 0xFF00_0000).collect();
                let _ = std::fs::write(format!("{shot_dir}/frame{f:05}-gfx.png"), png::encode(w, h, &img));
            }
            // SSX3_FRAME_DUMP=n: the last n frames' inputs, for gfx_replay.
            if std::env::var_os("SSX3_FRAME_DUMP").is_some() {
                let dir = format!("{shot_dir}/frame{f:05}-dump");
                let _ = std::fs::create_dir_all(&dir);
                let _ = ssx3_ps2::frame_sink::write_dumps(&dir);
            }
            if std::env::var_os("SSX3_GFX_RTDUMP").is_some() {
                for (fbp, w, h, px) in gfx.as_ref().map(|g| g.read_rts()).unwrap_or_default() {
                    let img: Vec<u32> = px.iter().map(|p| p | 0xFF00_0000).collect();
                    let _ = std::fs::write(format!("{shot_dir}/frame{f:05}-rt{fbp}.png"), png::encode(w, h, &img));
                    let alpha: Vec<u32> = px.iter().map(|p| { let a = p >> 24; a | a << 8 | a << 16 | 0xFF00_0000 }).collect();
                    let _ = std::fs::write(format!("{shot_dir}/frame{f:05}-rt{fbp}-alpha.png"), png::encode(w, h, &alpha));
                }
            }
            // SSX3_HIRES_SHOTS=dir: also the N× copies (SSX3_RENDER_SCALE).
            if let Ok(dir) = std::env::var("SSX3_HIRES_SHOTS") {
                for (label, w, h, px) in ssx3_ps2::m!(c).hw.gs.core().hires_snapshots() {
                    let img: Vec<u32> = px.iter().map(|p| p | 0xFF00_0000).collect();
                    let _ = std::fs::write(format!("{dir}/frame{f:05}-{label}.png"), png::encode(w, h, &img));
                }
                let mut img = Vec::new();
                let (w, h) = ssx3_ps2::m!(c).hw.gs.core().display_presented(&mut img);
                if w > 0 {
                    let _ = std::fs::write(format!("{dir}/frame{f:05}-display.png"), png::encode(w, h, &img));
                }
            }
        }
        if !alive {
            break;
        }
    }
    if let Some(g) = &gfx {
        eprintln!("[gfx] emulated GS persistent regs: {:x?}", ssx3_ps2::m!(c).hw.gs.core().persistent_regs());
        eprintln!("[gfx] {:?}", g.stats());
        eprintln!("[gfx] {}", g.0.lock().unwrap().timing_report());
        eprintln!("[gfx] other frame buffers (fbp, fbw, psm): {:?}", g.dropped_fbp());
    }
    // GPU GS backend: bring the CPU copy of VRAM up to date for the dumps.
    ssx3_ps2::m!(c).hw.gs.core().sync_vram();
    // Debug: SSX3_DUMP_TEX=tbp:tbw:psm:w:h:path writes that VRAM texture as
    // raw RGBA (CT32/CT24) at exit.
    if let Ok(spec) = std::env::var("SSX3_DUMP_TEX") {
        let p: Vec<&str> = spec.split(':').collect();
        let n = |i: usize| u32::from_str_radix(p[i].trim_start_matches("0x"), 16).unwrap_or(0);
        let (tbp, tbw, psm, w, h) = (n(0), n(1), n(2), n(3), n(4));
        let gs = ssx3_ps2::m!(c).hw.gs.core();
        let mut out = Vec::new();
        for y in 0..h {
            for x in 0..w {
                out.extend_from_slice(&ssx3_ps2::gs::swizzle::read_pixel(&gs.vram, psm, tbp, tbw, x, y).to_le_bytes());
            }
        }
        let _ = std::fs::write(p[5], out);
    }
    if std::env::var("SSX3_VRAM_DUMP").is_ok() {
        use ssx3_ps2::gs::swizzle::{read_pixel, PSMCT32};
        let mut v = vec![0u32; 1024 * 1024];
        let gs = ssx3_ps2::m!(c).hw.gs.core();
        for y in 0..1024 {
            for x in 0..1024 {
                v[y * 1024 + x] = read_pixel(&gs.vram, PSMCT32, 0, 16, x as u32, y as u32) | 0xff00_0000;
            }
        }
        drop(gs);
        let _ = std::fs::write(format!("{shot_dir}/vram.png"), png::encode(1024, 1024, &v));
        let regs: Vec<String> = [0x00u32, 0x20, 0x70, 0x80, 0x90, 0xa0, 0xe0].iter().map(|o| format!("{o:#x}={:#x}", ssx3_ps2::m!(c).hw.gs.read_priv(*o))).collect();
        eprintln!("gs priv: {}", regs.join(" "));
    }
    if let Ok(v) = std::env::var("SSX3_MEMSCAN") {
        let want = u64::from_str_radix(v.trim_start_matches("0x"), 16).unwrap_or(0);
        let scan = |name: &str, mem: &[u8]| {
            let mut hits = 0;
            for o in (0..mem.len().saturating_sub(16)).step_by(8) {
                let lo = u64::from_le_bytes(mem[o..o + 8].try_into().unwrap());
                if lo == want && hits < 20 {
                    let hi = u64::from_le_bytes(mem[o + 8..o + 16].try_into().unwrap());
                    eprintln!("[memscan] {name}+{o:#x}: {lo:#018x} next {hi:#018x}");
                    hits += 1;
                }
            }
        };
        scan("ram", &c.bus.ram);
        scan("vu1data", &ssx3_ps2::m!(c).hw.vu1.data);
        scan("spr", &c.bus.spr);
    }
    // Debug: SSX3_RAM_DUMP=path writes the 32 MiB of EE RAM at exit (compare
    // with a PCSX2 dump via tools/pine.py).
    if let Ok(path) = std::env::var("SSX3_RAM_DUMP") {
        if let Err(e) = std::fs::write(&path, &c.bus.ram[..]) {
            eprintln!("SSX3_RAM_DUMP {path}: {e}");
        }
    }
    ssx3_ps2::iop_bridge::shutdown(&mut c);
    if let Some(s) = ssx3_ps2::render_verify::summary() {
        eprintln!("{s}");
    }
    if let Some(s) = ssx3_ps2::frame_verify::summary() {
        eprintln!("{s}");
    }
    if let Some(s) = ssx3_ps2::mat3_verify::summary() {
        eprintln!("{s}");
    }
    if let Some(s) = ssx3_ps2::mat6_verify::summary() {
        eprintln!("{s}");
    }
    if let Some(s) = ssx3_ps2::bezier_verify::summary() {
        eprintln!("{s}");
    }
    if let Some(s) = ssx3_ps2::particle_verify::summary() {
        eprintln!("{s}");
    }
    if let Some(h) = ssx3_ps2::m!(c).sched.host_clock {
        let wall = started.elapsed().as_secs_f64();
        let app = 0x004c_9428u32;
        let (ticks, updates) = (c.bus.ram_u32(app + 0x18), c.bus.ram_u32(app + 0x1c));
        let vblanks = ssx3_ps2::m!(c).sched.frames;
        eprintln!(
            "[host clock] wall {wall:.1}s, emulated {:.1}s, vblanks {} ({:.1}/s), skipped {:.1}s in {} jumps; App vsync ticks {} updates {}",
            c.cycles as f64 / ssx3_ps2::machine::EE_CLOCK as f64,
            vblanks,
            vblanks as f64 / wall,
            h.skipped as f64 / ssx3_ps2::machine::EE_CLOCK as f64,
            h.jumps,
            ticks,
            updates
        );
    }
    // Debug: SSX3_PEEK="addr:words,addr:words" dumps EE memory at exit
    // (an address of the form gp+N is taken relative to $gp).
    if let Ok(spec) = std::env::var("SSX3_PEEK") {
        let gp = c.g[28][0] as u32;
        for item in spec.split(',') {
            let (a, n) = item.split_once(':').unwrap_or((item, "4"));
            let (iop, a) = match a.strip_prefix("iop") {
                Some(r) => (true, r),
                None => (false, a),
            };
            let a = match a.strip_prefix("gp+") {
                Some(o) => gp.wrapping_add(u32::from_str_radix(o.trim_start_matches("0x"), 16).unwrap_or(0)),
                None => u32::from_str_radix(a.trim_start_matches("0x"), 16).unwrap_or(0),
            };
            let n: u32 = n.parse().unwrap_or(4);
            let w: Vec<String> = (0..n)
                .map(|i| {
                    let o = (a + 4 * i) as usize;
                    let v = if iop {
                        u32::from_le_bytes(ssx3_ps2::m!(c).iop[o & 0x1F_FFFC..(o & 0x1F_FFFC) + 4].try_into().unwrap())
                    } else {
                        c.bus.ram_u32((a + 4 * i) & 0x01FF_FFFF)
                    };
                    format!("{v:08x}")
                })
                .collect();
            eprintln!("peek {a:#x} (gp {gp:#x}): {}", w.join(" "));
        }
    }
    // SSX3_TEXDUMP="tbp:tbw:psm:w:h:cbp" (hex tbp/psm/cbp) writes the texture
    // (palettised via a CT32 CSM1 CLUT at cbp) to <shots>/tex.png.
    if let Ok(spec) = std::env::var("SSX3_TEXDUMP") {
        use ssx3_ps2::gs::swizzle::{read_pixel, PSMCT32};
        let p: Vec<u32> = spec.split(':').map(|x| u32::from_str_radix(x, 16).unwrap_or(0)).collect();
        let (tbp, tbw, psm, w, h, cbp) = (p[0], p[1], p[2], p[3], p[4], p[5]);
        let gs = ssx3_ps2::m!(c).hw.gs.core();
        let vram = &gs.vram;
        let clut = |i: u32| -> u32 {
            // CSM1, CT32: 16-colour = 8x2 block; 256-colour = 16x16 with bit 3/4 swap.
            let (x, y) = if psm == 0x14 || psm == 0x24 || psm == 0x2c {
                (i % 8, i / 8)
            } else {
                let j = (i & 0xe7) | ((i & 0x08) << 1) | ((i & 0x10) >> 1);
                (j % 16, j / 16)
            };
            read_pixel(vram, PSMCT32, cbp, 1, x, y)
        };
        let mut img = vec![0u32; (w * h) as usize];
        for y in 0..h {
            for x in 0..w {
                let v = read_pixel(vram, psm, tbp, tbw, x, y);
                let col = if psm == 0 { v } else { clut(v) };
                img[(y * w + x) as usize] = col | 0xff00_0000;
            }
        }
        let _ = std::fs::write(format!("{shot_dir}/tex.png"), png::encode(w, h, &img));
    }
    eprintln!("halted: {:?}", ssx3_ps2::m!(c).kernel.halted);
    eprint!("{}", game::dump(&c));
    eprintln!("syscalls: {:?}", ssx3_ps2::m!(c).kernel.syscall_counts.iter().map(|(k, v)| format!("{}={v}", ssx3_ps2::kernel::syscall_name(*k))).collect::<Vec<_>>());
    eprintln!("modules: {:?}", ssx3_ps2::m!(c).sif.loaded_modules);
    eprintln!("unknown sif servers: {:x?}", ssx3_ps2::m!(c).sif.unknown_servers);
    let unk: Vec<String> = ssx3_ps2::m!(c).hw.unknown.iter().take(40).map(|(a, n)| format!("{a:08x}x{n}")).collect();
    eprintln!("unknown mmio: {}", unk.join(" "));
    eprintln!("misses: {:x?}", ssx3_ps2::m!(c).sched.misses);
    if !ssx3_ps2::m!(c).sched.misses.is_empty() {
        // Record new entry points for the next `recomp-ee` run.
        use std::io::Write;
        let path = game::repo_root().join("generated/entries.txt");
        if let Ok(mut f) = std::fs::OpenOptions::new().append(true).create(true).open(&path) {
            for pc in ssx3_ps2::m!(c).sched.misses.keys() {
                let _ = writeln!(f, "{pc:x}  # runtime miss (ra {:#x})", c.g[31][0] as u32);
            }
        }
        eprintln!("recorded {} new entry point(s) in generated/entries.txt", ssx3_ps2::m!(c).sched.misses.len());
    }
    if !ssx3_ps2::m!(c).hw.sio_line.is_empty() {
        eprintln!("sio: {}", String::from_utf8_lossy(&ssx3_ps2::m!(c).hw.sio_line));
    }
}
