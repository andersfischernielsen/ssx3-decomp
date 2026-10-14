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

    let mut c = game::boot();
    let t0 = Instant::now();
    let mut img = Vec::new();
    for f in 1..=frames {
        let alive = ssx3_ps2::machine::run_frame(&mut c);
        if f % 60 == 0 || !alive {
            eprintln!("[{:6.1}s] {}", t0.elapsed().as_secs_f64(), game::status(&c));
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
    eprintln!("halted: {:?}", c.kernel.halted);
    eprint!("{}", game::dump(&c));
    eprintln!("syscalls: {:?}", c.kernel.syscall_counts.iter().map(|(k, v)| format!("{}={v}", ssx3_ps2::kernel::syscall_name(*k))).collect::<Vec<_>>());
    eprintln!("modules: {:?}", c.sif.loaded_modules);
    eprintln!("unknown sif servers: {:x?}", c.sif.unknown_servers);
    let unk: Vec<String> = c.bus.hw.unknown.iter().take(40).map(|(a, n)| format!("{a:08x}x{n}")).collect();
    eprintln!("unknown mmio: {}", unk.join(" "));
    eprintln!("misses: {:x?}", c.sched.misses);
    if !c.bus.hw.sio_line.is_empty() {
        eprintln!("sio: {}", String::from_utf8_lossy(&c.bus.hw.sio_line));
    }
}
