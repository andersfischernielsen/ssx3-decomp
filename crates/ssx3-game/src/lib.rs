//! # ssx3-game
//!
//! Hand-written, typed Rust for SSX 3 functions, replacing the statically
//! recompiled versions one at a time (phase 2 in `docs/plans/roadmap.md`;
//! design in `docs/investigation/phase2-overrides.md`).
//!
//! A replacement is an ordinary `fn(&mut Cpu)` with the same contract as the
//! generated function: arguments and results in the MIPS registers
//! ([`abi`]), guest memory through typed views ([`guest`]), PS2 float
//! semantics ([`ps2f`]), MMI ops ([`mmi`]), calls to other guest functions
//! ([`call`]), event checks in loops ([`events`]), and the same EE cycle cost, so that runs stay
//! byte-identical. [`install`] puts them into the generated crates' slots.

// The generated EE code in the chosen form (features `ee-threads` / `ee-unwind`).
#[cfg(all(feature = "ee-threads", not(feature = "ee-unwind")))]
extern crate ssx3_ee_threads as ssx3_ee;
#[cfg(feature = "ee-unwind")]
extern crate ssx3_ee_unwind as ssx3_ee;

use ssx3_ee_rt::funcs::GuestFn;

pub mod abi;
#[cfg(feature = "ai")]
pub mod ai;
#[cfg(feature = "anim")]
pub mod anim;
#[cfg(feature = "audio")]
pub mod audio;
pub mod call;
#[cfg(feature = "camera")]
pub mod camera;
/// Debug-only overrides behind environment switches.
pub mod capture;
#[cfg(feature = "chase")]
pub mod chase;
#[cfg(feature = "containers")]
pub mod containers;
pub mod debug;
#[cfg(test)]
pub mod difftest;
#[cfg(feature = "dispatch")]
pub mod dispatch;
/// `SSX3_EVENT`: boot straight into an event (a development aid).
pub mod event_boot;
pub mod events;
/// The replacements, laid out like the C++ source tree (`src/<dir>/<unit>`).
pub mod game;
pub mod guest;
#[cfg(feature = "math")]
pub mod math;
pub mod mmi;
pub mod movie;
pub mod packetless;
pub mod ps2f;
#[cfg(feature = "render")]
pub mod render;
#[cfg(feature = "replay_script")]
pub mod replay_script;
pub mod rider;
#[cfg(feature = "rider_states")]
pub mod rider_states;
pub mod shadow;
#[cfg(feature = "skeleton")]
pub mod skeleton;
#[cfg(feature = "sound")]
pub mod sound;
#[cfg(feature = "state")]
pub mod state;
#[cfg(feature = "streaming")]
pub mod streaming;
#[cfg(feature = "ui")]
pub mod ui;
#[cfg(feature = "util")]
pub mod util;
#[cfg(feature = "world")]
pub mod world;
#[cfg(feature = "world_cache")]
pub mod world_cache;
// Phase 3: game files from the host (SSX3_NATIVE_FILES); uses `streaming`.
#[cfg(feature = "audio_system")]
pub mod audio_system;
#[cfg(feature = "graphics")]
pub mod graphics;
#[cfg(feature = "streaming")]
pub mod native_files;
#[cfg(feature = "oc346")]
pub mod oc346;
#[cfg(feature = "oc347")]
pub mod oc347;
#[cfg(feature = "oc348")]
pub mod oc348;
#[cfg(feature = "oc349")]
pub mod oc349;
#[cfg(feature = "oc350")]
pub mod oc350;
#[cfg(feature = "oc351")]
pub mod oc351;
#[cfg(feature = "oc352")]
pub mod oc352;
#[cfg(feature = "oc353")]
pub mod oc353;
#[cfg(feature = "oc354")]
pub mod oc354;
#[cfg(feature = "oc355")]
pub mod oc355;
#[cfg(feature = "oc356")]
pub mod oc356;
#[cfg(feature = "oc357")]
pub mod oc357;
#[cfg(feature = "oc358")]
pub mod oc358;
#[cfg(feature = "oc359")]
pub mod oc359;
#[cfg(feature = "oc360")]
pub mod oc360;
#[cfg(feature = "oc361")]
pub mod oc361;
#[cfg(feature = "oc362")]
pub mod oc362;
#[cfg(feature = "oc363")]
pub mod oc363;
#[cfg(feature = "oc364")]
pub mod oc364;
#[cfg(feature = "oc365")]
pub mod oc365;
#[cfg(feature = "oc366")]
pub mod oc366;
#[cfg(feature = "oc367")]
pub mod oc367;
#[cfg(feature = "oc368")]
pub mod oc368;
#[cfg(feature = "oc369")]
pub mod oc369;
#[cfg(feature = "oc370")]
pub mod oc370;
#[cfg(feature = "oc371")]
pub mod oc371;
#[cfg(feature = "oc372")]
pub mod oc372;
#[cfg(feature = "oc373")]
pub mod oc373;
#[cfg(feature = "oc374")]
pub mod oc374;
#[cfg(feature = "oc375")]
pub mod oc375;
#[cfg(feature = "rider_logic")]
pub mod rider_logic;
#[cfg(feature = "snd_lib")]
pub mod snd_lib;
#[cfg(feature = "world_painter")]
pub mod world_painter;

/// The configurable tick rate.
pub mod tick;

/// Detail distance (terrain grids, rider LOD).

/// Replacements outside the work-package modules: (address, function, name).
const CORE: &[(u32, GuestFn, &str)] = &[(
    0x0011_e098,
    rider::build_world_matrix,
    "Rider_BuildWorldMatrix",
)];

/// Every installed replacement (core + enabled work packages). Each package
/// module exports its own `OVERRIDES`; all of them must be listed in
/// `overrides.txt`.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v: Vec<(u32, GuestFn, &'static str)> = CORE.to_vec();
    v.extend(game::overrides());
    #[cfg(feature = "oc346")]
    v.extend_from_slice(oc346::OVERRIDES);
    #[cfg(feature = "oc347")]
    v.extend_from_slice(oc347::OVERRIDES);
    #[cfg(feature = "oc348")]
    v.extend_from_slice(oc348::OVERRIDES);
    #[cfg(feature = "oc349")]
    v.extend_from_slice(oc349::OVERRIDES);
    #[cfg(feature = "oc350")]
    v.extend_from_slice(oc350::OVERRIDES);
    #[cfg(feature = "oc351")]
    v.extend_from_slice(oc351::OVERRIDES);
    #[cfg(feature = "oc352")]
    v.extend_from_slice(oc352::OVERRIDES);
    #[cfg(feature = "oc353")]
    v.extend_from_slice(oc353::OVERRIDES);
    #[cfg(feature = "oc354")]
    v.extend_from_slice(oc354::OVERRIDES);
    #[cfg(feature = "oc355")]
    v.extend_from_slice(oc355::OVERRIDES);
    #[cfg(feature = "oc356")]
    v.extend_from_slice(oc356::OVERRIDES);
    #[cfg(feature = "oc357")]
    v.extend_from_slice(oc357::OVERRIDES);
    #[cfg(feature = "oc358")]
    v.extend_from_slice(oc358::OVERRIDES);
    #[cfg(feature = "oc359")]
    v.extend_from_slice(oc359::OVERRIDES);
    #[cfg(feature = "oc360")]
    v.extend_from_slice(oc360::OVERRIDES);
    #[cfg(feature = "oc361")]
    v.extend_from_slice(oc361::OVERRIDES);
    #[cfg(feature = "oc362")]
    v.extend_from_slice(oc362::OVERRIDES);
    #[cfg(feature = "oc363")]
    v.extend_from_slice(oc363::OVERRIDES);
    #[cfg(feature = "oc364")]
    v.extend_from_slice(oc364::OVERRIDES);
    #[cfg(feature = "oc365")]
    v.extend_from_slice(oc365::OVERRIDES);
    #[cfg(feature = "oc366")]
    v.extend_from_slice(oc366::OVERRIDES);
    #[cfg(feature = "oc367")]
    v.extend_from_slice(oc367::OVERRIDES);
    #[cfg(feature = "oc368")]
    v.extend_from_slice(oc368::OVERRIDES);
    #[cfg(feature = "oc369")]
    v.extend_from_slice(oc369::OVERRIDES);
    #[cfg(feature = "oc370")]
    v.extend_from_slice(oc370::OVERRIDES);
    #[cfg(feature = "oc371")]
    v.extend_from_slice(oc371::OVERRIDES);
    #[cfg(feature = "oc372")]
    v.extend_from_slice(oc372::OVERRIDES);
    #[cfg(feature = "oc373")]
    v.extend_from_slice(oc373::OVERRIDES);
    #[cfg(feature = "oc374")]
    v.extend_from_slice(oc374::OVERRIDES);
    #[cfg(feature = "oc375")]
    v.extend_from_slice(oc375::OVERRIDES);
    v.extend_from_slice(movie::overrides());
    #[cfg(feature = "math")]
    v.extend_from_slice(math::OVERRIDES);
    #[cfg(feature = "anim")]
    v.extend_from_slice(anim::OVERRIDES);
    #[cfg(feature = "camera")]
    v.extend_from_slice(camera::OVERRIDES);
    #[cfg(feature = "render")]
    v.extend_from_slice(render::OVERRIDES);
    #[cfg(feature = "sound")]
    v.extend_from_slice(sound::OVERRIDES);
    #[cfg(feature = "state")]
    v.extend_from_slice(state::OVERRIDES);
    #[cfg(feature = "util")]
    v.extend_from_slice(util::OVERRIDES);
    #[cfg(feature = "dispatch")]
    v.extend_from_slice(dispatch::OVERRIDES);
    #[cfg(feature = "rider_states")]
    v.extend_from_slice(rider_states::OVERRIDES);
    #[cfg(feature = "skeleton")]
    v.extend_from_slice(skeleton::OVERRIDES);
    #[cfg(feature = "chase")]
    v.extend_from_slice(chase::OVERRIDES);
    #[cfg(feature = "ai")]
    v.extend_from_slice(ai::OVERRIDES);
    #[cfg(feature = "world")]
    v.extend_from_slice(world::OVERRIDES);
    #[cfg(feature = "ui")]
    v.extend_from_slice(ui::OVERRIDES);
    #[cfg(feature = "audio")]
    v.extend_from_slice(audio::OVERRIDES);
    #[cfg(feature = "containers")]
    v.extend_from_slice(containers::OVERRIDES);
    #[cfg(feature = "world_cache")]
    v.extend_from_slice(world_cache::OVERRIDES);
    #[cfg(feature = "replay_script")]
    v.extend_from_slice(replay_script::OVERRIDES);
    #[cfg(feature = "streaming")]
    v.extend_from_slice(streaming::OVERRIDES);
    #[cfg(feature = "graphics")]
    v.extend_from_slice(graphics::OVERRIDES);
    #[cfg(feature = "rider_logic")]
    v.extend_from_slice(rider_logic::OVERRIDES);
    #[cfg(feature = "world_painter")]
    v.extend_from_slice(world_painter::OVERRIDES);
    #[cfg(feature = "audio_system")]
    v.extend_from_slice(audio_system::OVERRIDES);
    #[cfg(feature = "snd_lib")]
    v.extend_from_slice(snd_lib::OVERRIDES);
    v
}

/// Install the replacements (call once at boot, before guest code runs).
/// `SSX3_NO_OVERRIDES=1` keeps the generated bodies everywhere (A/B checks);
/// `SSX3_OVERRIDES=addr,addr` installs only the listed ones,
/// `SSX3_SKIP_OVERRIDES=addr,addr` all but the listed ones.
/// Returns how many were installed.
pub fn install() -> usize {
    if ssx3_core::config::var_os("SSX3_NO_OVERRIDES").is_some() {
        return 0;
    }
    let only: Option<Vec<u32>> = ssx3_core::config::var("SSX3_OVERRIDES").map(|s| {
        s.split(',')
            .filter_map(|a| u32::from_str_radix(a.trim().trim_start_matches("0x"), 16).ok())
            .collect()
    });
    // `SSX3_SKIP_OVERRIDES=addr,addr`: all but these (bisecting a wave).
    let skip: Vec<u32> = ssx3_core::config::var("SSX3_SKIP_OVERRIDES")
        .map(|s| {
            s.split(',')
                .filter_map(|a| u32::from_str_radix(a.trim().trim_start_matches("0x"), 16).ok())
                .collect()
        })
        .unwrap_or_default();
    let shadow = shadow::wanted();
    let mut n = 0;
    for (addr, f, name) in overrides() {
        if only.as_ref().is_some_and(|o| !o.contains(&addr)) || skip.contains(&addr) {
            continue;
        }
        // (`SSX3_SHADOW`: the replacement checked against the generated
        // body at run time, `shadow.rs`.)
        let f = match &shadow {
            Some(list) if list.as_ref().is_none_or(|l| l.contains(&addr)) => {
                shadow::wrap(addr, name, f).unwrap_or(f)
            }
            _ => f,
        };
        assert!(
            ssx3_ee::set_override(addr, f),
            "{name} ({addr:#x}) has no override slot: list it in crates/ssx3-game/overrides.txt and rerun recomp-ee"
        );
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    #[test]
    /// Every enabled replacement is listed in overrides.txt (the file may list
    /// more: packages not enabled in this build).
    fn overrides_txt_matches_table() {
        let txt: Vec<u32> = include_str!("../overrides.txt")
            .lines()
            .filter_map(|l| {
                let w = l.split('#').next()?.split_whitespace().next()?;
                u32::from_str_radix(w.trim_start_matches("0x"), 16).ok()
            })
            .collect();
        for (a, _, name) in super::overrides() {
            assert!(
                txt.contains(&a),
                "{name} ({a:#x}) is not listed in overrides.txt"
            );
        }
    }
}
