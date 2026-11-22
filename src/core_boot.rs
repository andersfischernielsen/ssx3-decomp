//! Boot-time wiring that the fork did inside `native_files.rs` and
//! `event_boot.rs`: where the importer's files are, and the `SSX3_EVENT`
//! development boot into a Tricky event.
//!
//! | what | fork | plugin |
//! | --- | --- | --- |
//! | asset root, movie/rule roots | `native_files::install` | [`configure`] from `plugin_host::overlays()` |
//! | `SSX3_EVENT=<course>` | `event_boot::install` | engine's `SSX3_EVENT=tricky/<id>`; [`configure`] activates the course |
//! | `prepare_track_info` at boot | `event_boot::start` | chain on `FE_SetOnlineMode` |
//! | world archive redirect | `native_files::open` | the engine's plugin world swap + overlay (the importer names the archive `<world>.BIG`) |
//! | object-sound bank patching | `native_files::read` | NOT converted (needs a file-read hook in the engine) |

use std::sync::atomic::{AtomicBool, Ordering};

use ssx3_plugin::reg::*;
use ssx3_plugin::{Cpu, Prev};

use crate::{custom_course_rules, custom_movies, custom_tracks, Chain};

static PREPARE_PENDING: AtomicBool = AtomicBool::new(false);

/// The Tricky plugin's overlay folder, if the plugin is loaded.
fn overlay() -> Option<std::path::PathBuf> {
    ssx3_game::plugin_host::overlays()
        .iter()
        .find(|p| p.parent().and_then(|d| d.file_name()).is_some_and(|n| n == "tricky"))
        .cloned()
}

/// Called from `install`: roots for the importer's files, and the activation of
/// the course named by `SSX3_EVENT=tricky/<id>`.
pub fn configure() {
    let Some(root) = overlay() else { return };
    let ready = root.join("DATA/WORLDS").is_dir();
    custom_tracks::set_assets_available(ready);
    custom_movies::set_root(ready.then(|| root.clone()));
    custom_course_rules::set_root(ready.then(|| root.clone()));
    // The world texture slots the prepared courses need (the importer records
    // them in each course's manifest): TexMan must size its table at boot.
    let mut slots = 1500;
    if let Ok(dir) = std::fs::read_dir(root.join("DATA/WORLDS/TRICKY")) {
        for entry in dir.flatten() {
            if entry.path().extension().is_some_and(|e| e == "json") {
                let text = std::fs::read_to_string(entry.path()).unwrap_or_default();
                if let Some(i) = text.find("\"world_texture_slots\":") {
                    let n: String = text[i + 22..].chars().skip_while(|c| !c.is_ascii_digit())
                        .take_while(char::is_ascii_digit).collect();
                    slots = slots.max(n.parse().unwrap_or(0));
                }
            }
        }
    }
    custom_tracks::set_world_texture_slots(slots);
    if let Ok(want) = std::env::var("SSX3_EVENT") {
        if let Some(id) = want.strip_prefix("tricky/") {
            if custom_tracks::activate(id).is_some() {
                PREPARE_PENDING.store(true, Ordering::Release);
                eprintln!("[tricky] active course {id}");
            }
        }
    }
}

static PREV_ONLINE: Prev = Prev::new();

/// `FE_SetOnlineMode` (0x1a86e8) is the first call of the boot into an event:
/// build the course's TrackInfo record before the template event reads it.
fn set_online_mode(c: &mut Cpu) {
    if PREPARE_PENDING.swap(false, Ordering::AcqRel) {
        let (a0, ra, pc) = (c.g[A0][0], c.g[RA][0], c.pc);
        ssx3_game::call::guest(c, |c| custom_tracks::prepare_track_info(c, ra as u32));
        c.g[A0][0] = a0;
        c.g[RA][0] = ra;
        c.pc = pc;
    }
    PREV_ONLINE.call(c);
}

pub static CHAINS: &[Chain] = &[Chain {
    addr: 0x001a_86e8,
    f: set_online_mode,
    prev: &PREV_ONLINE,
    name: "FE_SetOnlineMode",
}];
