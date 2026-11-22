//! Plugins in the engine: folders next to the game's data and Rust plugins
//! linked into the front end (`ssx3-plugin`, `docs/plans/plugins.md`).

use std::path::PathBuf;

use ssx3_core::{config, platform_paths};
use ssx3_plugin::discover::{Filter, Roots};
use ssx3_plugin::{Loaded, Plugin};

use crate::boot::repo_root;

/// Where plugin folders, disc images and derived content are looked for:
///
/// - plugin folders: `SSX3_PLUGIN_DIR`, then `plugins/` in the data root;
/// - disc images: `SSX3_DISC_DIR`, the folder of the SSX 3 image, the data root;
/// - derived content: `plugins/<id>/` in the cache folder.
pub fn roots() -> Roots {
    let data = platform_paths::data_path("", repo_root());
    let mut plugin_dirs: Vec<PathBuf> = Vec::new();
    if let Some(d) = config::var_os("SSX3_PLUGIN_DIR") {
        plugin_dirs.push(PathBuf::from(d));
    }
    plugin_dirs.push(platform_paths::data_path(
        "plugins",
        repo_root().join("plugins"),
    ));
    let mut disc_dirs: Vec<PathBuf> = Vec::new();
    if let Some(d) = config::var_os("SSX3_DISC_DIR") {
        disc_dirs.push(PathBuf::from(d));
    }
    let iso = platform_paths::data_file(
        "SSX3_ISO",
        "SSX 3 (USA).iso",
        repo_root().join("SSX 3 (USA).iso"),
    );
    if let Some(dir) = iso.parent() {
        disc_dirs.push(dir.to_path_buf());
    }
    disc_dirs.push(data);
    disc_dirs.dedup();
    Roots {
        plugin_dirs,
        disc_dirs,
        cache: platform_paths::cache_dir().unwrap_or_else(|| repo_root().join(".cache")),
    }
}

/// Which plugins may load (`SSX3_PLUGINS`: unset all, `0` none, or a list).
pub fn filter() -> Filter {
    Filter::from_setting(config::var("SSX3_PLUGINS").as_deref())
}

/// Load the plugins: after the game's own overrides (hooks wrap them), before
/// the file layer starts (overlays go in front of the disc).
pub(crate) fn load(code: Vec<Box<dyn Plugin>>) -> Loaded {
    let mut slots = ssx3_game::plugin_host::GameSlots;
    let loaded = ssx3_plugin::load(&roots(), &filter(), &code, &mut slots);
    for line in &loaded.log {
        eprintln!("[plugins] {line}");
    }
    for overlay in &loaded.overlays {
        ssx3_game::native_files::add_overlay(overlay.clone());
    }
    ssx3_game::plugin_host::set_catalog(loaded.catalog.clone());
    loaded
}
