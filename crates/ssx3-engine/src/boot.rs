//! Machine setup: data and save locations, the disc, the executable, the
//! game's hand-written functions and hooks.

use std::path::PathBuf;
use std::sync::OnceLock;

use ssx3_ps2::{machine, Cpu};

/// The checkout this crate was built from (the defaults for the disc image,
/// `orig/` and `generated/entries.txt` when no root or override names them).
pub fn repo_root() -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

static PATHS: OnceLock<bool> = OnceLock::new();

/// Where the game's files come from and where saves go. Call once, first
/// thing (before reading a settings file kept with the saves, before
/// [`crate::Engine::boot`], which calls it itself if the front end has not).
/// Later calls do nothing and return the first call's result.
///
/// - `data_root` / `save_root`: the roots of `ssx3_core::platform_paths`
///   (game data, may be read-only; memory card, settings, dumps; writable).
///   Both `None` keeps the checkout-relative defaults; one alone takes the
///   checkout (data) or `saves` (saves) for the other. Per-file settings
///   (`SSX3_ISO`, `SSX3_MEMCARD`, ...: the environment, then the front end's
///   settings, `ssx3_core::config`) still win.
/// - `find_pack`: a standalone build (`tools/bundle.py`): the disc comes from
///   the pack in or next to the executable (or in the data root), and the
///   memory card and settings live in the save root, else the user's data
///   folder (`ssx3_core::platform_paths::save_dir`), unless `SSX3_MEMCARD` /
///   `SSX3_SETTINGS` say otherwise.
///
/// Returns whether a pack was found.
pub fn init_paths(data_root: Option<PathBuf>, save_root: Option<PathBuf>, find_pack: bool) -> bool {
    *PATHS.get_or_init(|| {
        if data_root.is_some() || save_root.is_some() {
            let _ = ssx3_core::platform_paths::set(ssx3_core::platform_paths::Roots {
                data: data_root.unwrap_or_else(repo_root),
                saves: save_root.unwrap_or_else(|| PathBuf::from("saves")),
            });
        }
        find_pack && init_standalone()
    })
}

fn init_standalone() -> bool {
    let Some((path, base)) = ssx3_core::pack::Pack::locate() else {
        return false;
    };
    let shown = path.display().to_string();
    ssx3_core::iso::set_opener(Box::new(move || {
        Ok(Box::new(ssx3_core::pack::Pack::open_at(&path, base)?)
            as Box<dyn ssx3_core::iso::Source>)
    }));
    ssx3_core::platform_paths::set_standalone();
    match ssx3_core::platform_paths::save_dir() {
        Some(dir) => eprintln!(
            "[standalone] game data from {shown}; saves in {}",
            dir.display()
        ),
        None => eprintln!(
            "[standalone] game data from {shown}; no user data folder \
             (HOME, APPDATA or XDG_DATA_HOME unset or without a root), so saves in ./saves"
        ),
    }
    true
}

/// At every VSync: the game rate.
fn vblank(c: &mut Cpu) {
    ssx3_game::tick::on_vblank(c);
}

/// Create the machine, load the disc and the executable, start the main thread.
pub(crate) fn boot(skip_movies: bool) -> Box<Cpu> {
    machine::register_functions(&ssx3_ee::FUNCTIONS);
    // Hand-written replacements (crates/ssx3-game); SSX3_NO_OVERRIDES=1 runs
    // the generated code everywhere.
    let n = ssx3_game::install();
    if n > 0 {
        eprintln!("[ssx3-game] {n} hand-written function(s) installed");
    }
    // The tick rate (SSX3_TICK_HZ, settings "Game rate").
    machine::set_vsync_hz(ssx3_game::tick::install());
    machine::set_vblank_hook(vblank);
    ssx3_game::debug::install();
    // SSX3_EVENT / SSX3_RIDER: straight into an event.
    ssx3_game::event_boot::install();
    // Phase 3: the draw functions record their draws for the renderer.
    if !ssx3_game::capture::install() {
        eprintln!("[ssx3-game] draw capture off");
    }
    // Phase 3: files from the host instead of the emulated CD.
    if ssx3_game::native_files::install() {
        eprintln!("[ssx3-game] native files on");
    }
    // Recompiled IOP sound modules; used when SSX3_IOP=1 (see iop_bridge.rs).
    #[cfg(not(target_arch = "wasm32"))]
    ssx3_iop::register_generated(&ssx3_iop_gen::GENERATED);
    let mut c = ssx3_ps2::new_cpu();
    ssx3_ps2::m!(c).kernel.host_threads = ssx3_ee::HOST_THREADS;
    // The GPU rasterizer is the default for the game (falls back to the CPU
    // when no adapter is usable); SSX3_GS=cpu forces the software path.
    // (The browser: no GS rasterizer at all, see `install_renderer`.)
    #[cfg(feature = "emu-gs")]
    if ssx3_core::config::var("SSX3_GS").is_none() && !cfg!(target_arch = "wasm32") {
        ssx3_ps2::m!(c)
            .hw
            .gs
            .core()
            .set_backend(ssx3_ps2::gs::Backend::Gpu);
    }
    let iso = ssx3_core::platform_paths::data_file_utf8(
        "SSX3_ISO",
        "SSX 3 (USA).iso",
        repo_root().join("SSX 3 (USA).iso"),
    );
    if let Err(e) = ssx3_ps2::m!(c).sif.open_disc(iso.clone()) {
        eprintln!("warning: cannot open disc image {}: {e}", iso.display());
    }
    let elf = read_elf(&mut c);
    machine::boot(&mut c, &elf, "cdrom0:\\SLUS_207.72;1").expect("boot");
    // The boot movies are selected by a bitmask at gp-0x1724 (0x4a19cc;
    // initial value 7 = EA logo | THX | intro, read by ssx_BootMoviesUpdate
    // 0x1a27a0). Clearing it skips them.
    if skip_movies {
        c.bus.write32(0x004a_19cc, 0);
    }
    c
}

/// The executable: `orig/SLUS_207.72`, else the one on the disc.
fn read_elf(c: &mut Cpu) -> Vec<u8> {
    let path = ssx3_core::platform_paths::data_path(
        "orig/SLUS_207.72",
        repo_root().join("orig/SLUS_207.72"),
    );
    if !ssx3_core::platform_paths::is_standalone() {
        if let Ok(v) = std::fs::read(&path) {
            return v;
        }
    }
    let d = ssx3_ps2::m!(c)
        .sif
        .disc
        .as_mut()
        .unwrap_or_else(|| panic!("{}: missing, and no disc", path.display()));
    let e = d.find("SLUS_207.72").expect("SLUS_207.72 on the disc");
    let mut v = d.read_vec(e.sector, e.size.div_ceil(2048));
    v.truncate(e.size as usize);
    v
}
