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
//! | object-sound bank patching | `native_files::read` | `native_files::add_read_patch` ([`read_patch`]) |

use std::sync::atomic::{AtomicBool, Ordering};

use ssx3_plugin::reg::*;
use ssx3_plugin::{Cpu, Prev};

use crate::{custom_course_rules, custom_movies, custom_tracks, Chain};

static AUDIO: std::sync::Mutex<Option<crate::custom_audio_banks::AudioBanks>> =
    std::sync::Mutex::new(None);

/// The engine's read patch: the course's mountain sound bank replaces the
/// bytes of `mtn.bnk` inside `AUDIO.BIG` while a Tricky course is active.
fn read_patch(name: &str, offset: u32, out: &mut [u8]) -> bool {
    match AUDIO.lock().unwrap().as_mut() {
        Some(banks) => banks.apply(name, offset, out),
        None => true,
    }
}

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
    if ready {
        *AUDIO.lock().unwrap() = crate::custom_audio_banks::AudioBanks::open(&root);
        ssx3_game::native_files::add_read_patch(read_patch);
    }
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

/// The engine's menu started `tricky/<id>`: make the course active and build
/// its track record before the load screen.
pub fn event_started(
    c: &mut Cpu,
    event: &ssx3_plugin::catalog::Event,
    back: u32,
) -> ssx3_game::call::Flow {
    if event.plugin != "tricky" {
        return Ok(());
    }
    let id = event.id.strip_prefix("tricky/").unwrap_or(&event.id);
    if custom_tracks::activate_in_game(c, id).is_none() {
        eprintln!("[tricky] no course {id}");
        return Ok(());
    }
    eprintln!("[tricky] active course {id} (menu)");
    custom_tracks::prepare_track_info(c, back)
}

/// A plugin event ended (back in the front end, or another event starts).
pub fn event_ended(c: &mut Cpu) {
    if custom_tracks::active().is_some() {
        custom_tracks::deactivate(c);
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
