//! Boot the recompiled game: machine setup shared by the windowed and
//! headless front ends.

use std::path::PathBuf;

use ssx3_ps2::{machine, Cpu};

pub fn repo_root() -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Create the machine, load the disc and the executable, start the main thread.
pub fn boot(skip_movies_default: bool) -> Box<Cpu> {
    machine::register_functions(&ssx3_ee::FUNCTIONS);
    // Hand-written replacements (crates/ssx3-game); SSX3_NO_OVERRIDES=1 runs
    // the generated code everywhere.
    let n = ssx3_game::install();
    if n > 0 {
        eprintln!("[ssx3-game] {n} hand-written function(s) installed");
    }
    ssx3_game::debug::install();
    // Phase 3: files from the host instead of the emulated CD.
    if ssx3_game::native_files::install() {
        eprintln!("[ssx3-game] native files on");
    }
    // Recompiled IOP sound modules; used when SSX3_IOP=1 (see iop_bridge.rs).
    ssx3_iop::register_generated(&ssx3_iop_gen::GENERATED);
    let mut c = ssx3_ps2::new_cpu();
    ssx3_ps2::m!(c).kernel.host_threads = ssx3_ee::HOST_THREADS;
    // The GPU rasterizer is the default for the game (falls back to the CPU
    // when no adapter is usable); SSX3_GS=cpu forces the software path.
    if std::env::var("SSX3_GS").is_err() {
        ssx3_ps2::m!(c).hw.gs.core().set_backend(ssx3_ps2::gs::Backend::Gpu);
    }
    let iso = std::env::var("SSX3_ISO")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root().join("SSX 3 (USA).iso"));
    if let Err(e) = ssx3_ps2::m!(c).sif.open_disc(iso.clone()) {
        eprintln!("warning: cannot open disc image {}: {e}", iso.display());
    }
    let elf_path = repo_root().join("orig/SLUS_207.72");
    let elf = std::fs::read(&elf_path).unwrap_or_else(|e| panic!("{}: {e}", elf_path.display()));
    machine::boot(&mut c, &elf, "cdrom0:\\SLUS_207.72;1").expect("boot");
    // The boot movies are selected by a bitmask at gp-0x1724 (0x4a19cc;
    // initial value 7 = EA logo | THX | intro, read by ssx_BootMoviesUpdate
    // 0x1a27a0). Clearing it skips them: the headless runner does by default
    // (scripted input timings assume it), the window plays them.
    // SSX3_SKIP_MOVIES=0/1 overrides either.
    let skip = std::env::var("SSX3_SKIP_MOVIES").map(|v| v != "0").unwrap_or(skip_movies_default);
    if skip {
        c.bus.write32(0x004a_19cc, 0);
    }
    c
}

/// One-line status summary.
pub fn status(c: &Cpu) -> String {
    let threads: Vec<String> = ssx3_ps2::mr!(c)
        .kernel
        .threads
        .values()
        .map(|t| format!("{}:{:?}@{:x}p{}", t.id, t.status, if t.id == ssx3_ps2::mr!(c).kernel.current { c.pc } else { t.ctx.pc }, t.priority))
        .collect();
    format!(
        "frame {} cycles {}M thread {} prims {} | {}",
        ssx3_ps2::mr!(c).sched.frames,
        c.cycles / 1_000_000,
        ssx3_ps2::mr!(c).kernel.current,
        ssx3_ps2::mr!(c).hw.gs.prims_drawn(),
        threads.join(" ")
    )
}

/// Multi-line dump of every thread and semaphore.
pub fn dump(c: &Cpu) -> String {
    let mut out = String::new();
    for t in ssx3_ps2::mr!(c).kernel.threads.values() {
        let (pc, ra, sp) = if t.id == ssx3_ps2::mr!(c).kernel.current {
            (c.pc, c.g[31][0] as u32, c.g[29][0] as u32)
        } else {
            (t.ctx.pc, t.ctx.g[31][0] as u32, t.ctx.g[29][0] as u32)
        };
        out += &format!(
            "  thread {} {:?} prio {} entry {:#x} pc {:#x} ra {:#x} sp {:#x} wait {:?}/{}\n",
            t.id, t.status, t.priority, t.entry, pc, ra, sp, t.wait, t.wait_id
        );
        // Heuristic backtrace: words on the stack that are return addresses
        // (point just after a jal/jalr in translated code).
        let mut bt = Vec::new();
        for i in 0..512u32 {
            let v = c.bus.ram_u32(sp.wrapping_add(i * 4));
            if (0x0010_0000..0x0042_e590).contains(&v) && v & 3 == 0 && ssx3_ps2::machine::has_code(v) {
                let call = c.bus.ram_u32(v - 8);
                if call >> 26 == 3 || (call >> 26 == 0 && call & 63 == 9) {
                    bt.push(format!("{v:x}"));
                }
            }
            if bt.len() >= 16 {
                break;
            }
        }
        out += &format!("    stack: {}\n", bt.join(" "));
    }
    out += &format!("  cpu per thread (Mcycles): {:?}\n", ssx3_ps2::mr!(c).kernel.thread_cycles.iter().map(|(k, v)| (*k, v / 1_000_000)).collect::<Vec<_>>());
    for (id, s) in &ssx3_ps2::mr!(c).kernel.semas {
        if !s.waiters.is_empty() {
            out += &format!("  sema {id} count {} waiters {:?}\n", s.count, s.waiters);
        }
    }
    out
}

/// The draw-layer renderer (`ssx3-gfx`, `docs/plans/phase3-draw-layer.md`),
/// by `SSX3_RENDER`:
/// - `native` (default): it draws every frame the game publishes and is
///   shown; the emulated GS keeps running everything but rasterizing while
///   frames are published (movies and loading screens still draw there);
/// - `shadow`: it draws next to the emulated GS, which stays what is shown;
/// - `emu`: not installed (the emulated GS only).
pub fn install_renderer(c: &mut Cpu) -> Option<ssx3_gfx::Renderer> {
    let mode = std::env::var("SSX3_RENDER").unwrap_or_else(|_| "native".into());
    let native = match mode.as_str() {
        "native" => true,
        "shadow" => false,
        "emu" => return None,
        m => {
            eprintln!("SSX3_RENDER={m}: expected native, shadow or emu; using native");
            true
        }
    };
    let Some(ctx) = ssx3_ps2::gs::gpu::context() else {
        eprintln!("[gfx] no GPU: using the emulated GS");
        return None;
    };
    let r = ssx3_gfx::Renderer::new(ctx.device.clone(), ctx.queue.clone());
    r.set_want_depth(native);
    // SSX3_RENDER_SCALE=N: N× internal resolution (1..8).
    if let Some(n) = std::env::var("SSX3_RENDER_SCALE").ok().and_then(|v| v.parse::<u32>().ok()) {
        r.set_scale(n);
    }
    // The light visibility queries from the native depth (unless a debug
    // switch already forces them).
    if native && std::env::var_os("SSX3_DEBUG_LENSFX_VIS").is_none() {
        assert!(ssx3_ee::set_override(LENSFX_QUERY, lensfx_query_native));
    }
    ssx3_ps2::frame_sink::set_native(native);
    // Rendering runs on its own thread, next to the game.
    r.start_worker();
    // Native: no GS emulation. The game's frames (menus, movies, loading
    // screens and races all come through them) are drawn by ssx3-gfx; the
    // GS keeps only what the game can observe (registers and events).
    if native {
        ssx3_ps2::m!(c).hw.gs.detach();
    }
    ssx3_ps2::frame_sink::install(Box::new(r.clone()));
    eprintln!("[gfx] draw-layer renderer installed ({})", if native { "native" } else { "shadow" });
    Some(r)
}

/// `LensFX_v04` (0x2ec478): how much of a light source is visible. The game
/// reads the Z buffer around the light back from the GS (a 16×16 local→host
/// transfer through VIF1) and counts the pixels of the light's rectangle at
/// or behind its depth. Natively the same count runs over the renderer's
/// depth; with no native frame yet, the generated body runs.
///
/// Arguments: a0 the LensFX object, a1 the query record (floats: light
/// rectangle x, y, w, h at 0..3, read rectangle x, y, w, h at 4..7, depth
/// at 9), a2 where the visibility goes. Returns 1.
const LENSFX_QUERY: u32 = 0x002e_c478;

fn lensfx_query_native(c: &mut Cpu) {
    use ssx3_ee_rt::cpu::reg::*;
    if !ssx3_ps2::frame_sink::native() {
        ssx3_ee::s09::f_002ec478__generated(c);
        return;
    }
    let (rec, out) = (c.g[A1][0] as u32, c.g[A2][0] as u32);
    let f: Vec<f32> = (0..10).map(|i| f32::from_bits(c.bus.read32(rec + 4 * i))).collect();
    // Records with a read rectangle beyond the screen (or not a number)
    // are left to the game's own code.
    let area = f[6] * f[7];
    if !(0.0..=1024.0 * 1024.0).contains(&area) || !f[4..8].iter().all(|v| v.is_finite()) {
        static LOGGED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        if LOGGED.fetch_add(1, std::sync::atomic::Ordering::Relaxed) < 5 {
            eprintln!("[lensfx] unusual query record {f:?}: generated path");
        }
        ssx3_ee::s09::f_002ec478__generated(c);
        return;
    }
    let Some(v) = ssx3_ps2::frame_sink::with_depth(|w, h, z| lensfx_visibility(&f, w, h, z)) else {
        ssx3_ee::s09::f_002ec478__generated(c);
        return;
    };
    c.bus.write32(out, v.to_bits());
    c.g[V0][0] = 1;
    // Return to the caller.
    c.pc = c.g[RA][0] as u32;
}

/// `LensFX_v04`'s count over a `w`×`h` Z buffer (`f`: the query record).
/// Pixels of the light's rectangle at or behind its depth count as
/// visible. With the whole rectangle on screen the result is visible /
/// total; otherwise max(0, (visible − half) / half) truncated to an
/// integer, as the game computes it.
fn lensfx_visibility(f: &[f32], w: u32, h: u32, z: &[u32]) -> f32 {
    let (lx, ly, lw, lh) = (f[0] as i32, f[1] as i32, f[2] as i32, f[3] as i32);
    let (sx, sy, rw, rh) = (f[4] as i32, f[5] as i32, f[6] as i32, f[7] as i32);
    // __fixunssfsi.
    let depth = if f[9] < 0.0 { f[9] as i32 as u32 } else { f[9] as u32 };
    let (mut total, mut visible) = (0.0f32, 0.0f32);
    for r in 0..rh.max(0) {
        let py = sy + r;
        for k in 0..rw.max(0) {
            let px = sx + k;
            if px < lx || px >= lx + lw || py < ly || py >= ly + lh {
                continue;
            }
            total += 1.0;
            let zv = if px >= 0 && py >= 0 && (px as u32) < w && (py as u32) < h { z[(py as u32 * w + px as u32) as usize] } else { 0 };
            if zv <= depth {
                visible += 1.0;
            }
        }
    }
    let full = f[2] * f[3];
    if total == full {
        visible / total
    } else {
        let half = full * 0.5;
        let r = (visible - half) / half;
        ((r as i32) * (0.0 < r) as i32) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::lensfx_visibility;

    /// A 16×16 query at (8, 8), light depth 1000, over a 64×64 Z buffer.
    fn query(x: f32, y: f32) -> [f32; 10] {
        [x, y, 16.0, 16.0, x, y, 16.0, 16.0, 0.0, 1000.0]
    }

    #[test]
    fn unoccluded_light_is_fully_visible() {
        let z = vec![500u32; 64 * 64];
        assert_eq!(lensfx_visibility(&query(8.0, 8.0), 64, 64, &z), 1.0);
    }

    #[test]
    fn geometry_in_front_hides_its_share() {
        // The left half of the rectangle is covered by nearer geometry
        // (larger Z is nearer on the GS).
        let mut z = vec![0u32; 64 * 64];
        for y in 8..24 {
            for x in 8..16 {
                z[y * 64 + x] = 5000;
            }
        }
        assert_eq!(lensfx_visibility(&query(8.0, 8.0), 64, 64, &z), 0.5);
    }

    #[test]
    fn partly_off_screen_uses_the_game_formula() {
        // Read rectangle clamped to the screen: 8 of 16 columns are read,
        // all visible: (128 - 128) / 128 = 0.
        let z = vec![0u32; 64 * 64];
        let mut f = query(-8.0, 8.0);
        f[4] = 0.0;
        f[6] = 8.0;
        assert_eq!(lensfx_visibility(&f, 64, 64, &z), 0.0);
        // Fully on screen again but every pixel occluded: 0.
        let z = vec![5000u32; 64 * 64];
        assert_eq!(lensfx_visibility(&query(8.0, 8.0), 64, 64, &z), 0.0);
    }
}
