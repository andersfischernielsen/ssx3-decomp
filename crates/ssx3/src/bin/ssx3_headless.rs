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
    let t0 = Instant::now();
    let mut img = Vec::new();
    // Scripted input: SSX3_INPUT="300:4000:10,400:0008:10" presses Cross (0x4000)
    // at frame 300 for 10 frames, then Start (0x0008) at frame 400.
    let script: Vec<(u64, u16, u64)> = std::env::var("SSX3_INPUT")
        .unwrap_or_default()
        .split(',')
        .filter_map(|e| {
            let p: Vec<&str> = e.split(':').collect();
            Some((p.first()?.parse().ok()?, u16::from_str_radix(p.get(1)?, 16).ok()?, p.get(2).and_then(|s| s.parse().ok()).unwrap_or(6)))
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
                c.bus.hw.gs.start_capture();
            }
        }
        let mut held = 0u16;
        for (at, bits, len) in &script {
            if f >= *at && f < at + len {
                held |= bits;
            }
        }
        c.sif.pad_buttons[0] = held;
        ssx3_ps2::vu::capture::set_frame(f);
        let alive = ssx3_ps2::machine::run_frame(&mut c);
        if ssx3_ps2::gifstats::enabled() && f % 50 == 0 {
            let (pk, v, adc, far) = ssx3_ps2::gifstats::take();
            eprintln!("[gif] frames {}..{f}: path1 packets {pk} verts {v} adc {adc} far {far}", f - 50);
        }
        for (start, n, path) in &captures {
            if f + 1 == start + n || (!alive && f >= *start && f < start + n) {
                let trace = c.bus.hw.gs.finish_capture();
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
            let (w, h) = c.bus.hw.gs.display(&mut img);
            let path = format!("{shot_dir}/frame{f:05}.png");
            let _ = std::fs::write(&path, png::encode(w, h, &img));
        }
        if !alive {
            break;
        }
    }
    // GPU GS backend: bring the CPU copy of VRAM up to date for the dumps.
    c.bus.hw.gs.sync_vram();
    if std::env::var("SSX3_VRAM_DUMP").is_ok() {
        use ssx3_ps2::gs::swizzle::{read_pixel, PSMCT32};
        let mut v = vec![0u32; 1024 * 1024];
        for y in 0..1024 {
            for x in 0..1024 {
                v[y * 1024 + x] = read_pixel(&c.bus.hw.gs.vram, PSMCT32, 0, 16, x as u32, y as u32) | 0xff00_0000;
            }
        }
        let _ = std::fs::write(format!("{shot_dir}/vram.png"), png::encode(1024, 1024, &v));
        let regs: Vec<String> = [0x00u32, 0x20, 0x70, 0x80, 0x90, 0xa0, 0xe0].iter().map(|o| format!("{o:#x}={:#x}", c.bus.hw.gs.read_priv(*o))).collect();
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
        scan("vu1data", &c.bus.hw.vu1.data);
        scan("spr", &c.bus.spr);
    }
    ssx3_ps2::iop_bridge::shutdown(&mut c);
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
                        u32::from_le_bytes(c.bus.iop[o & 0x1F_FFFC..(o & 0x1F_FFFC) + 4].try_into().unwrap())
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
        let vram = &c.bus.hw.gs.vram;
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
    eprintln!("halted: {:?}", c.kernel.halted);
    eprint!("{}", game::dump(&c));
    eprintln!("syscalls: {:?}", c.kernel.syscall_counts.iter().map(|(k, v)| format!("{}={v}", ssx3_ps2::kernel::syscall_name(*k))).collect::<Vec<_>>());
    eprintln!("modules: {:?}", c.sif.loaded_modules);
    eprintln!("unknown sif servers: {:x?}", c.sif.unknown_servers);
    let unk: Vec<String> = c.bus.hw.unknown.iter().take(40).map(|(a, n)| format!("{a:08x}x{n}")).collect();
    eprintln!("unknown mmio: {}", unk.join(" "));
    eprintln!("misses: {:x?}", c.sched.misses);
    if !c.sched.misses.is_empty() {
        // Record new entry points for the next `recomp-ee` run.
        use std::io::Write;
        let path = game::repo_root().join("generated/entries.txt");
        if let Ok(mut f) = std::fs::OpenOptions::new().append(true).create(true).open(&path) {
            for pc in c.sched.misses.keys() {
                let _ = writeln!(f, "{pc:x}  # runtime miss (ra {:#x})", c.g[31][0] as u32);
            }
        }
        eprintln!("recorded {} new entry point(s) in generated/entries.txt", c.sched.misses.len());
    }
    if !c.bus.hw.sio_line.is_empty() {
        eprintln!("sio: {}", String::from_utf8_lossy(&c.bus.hw.sio_line));
    }
}
