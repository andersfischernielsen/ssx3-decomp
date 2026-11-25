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

// The generated EE code.
#[cfg(feature = "ee-threads")]
extern crate ssx3_ee_threads as ssx3_ee;

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
/// `SSX3_EVENT`: boot straight into an event (a development aid).
pub mod career_guard;
#[cfg(feature = "chase")]
pub mod chase;
#[cfg(feature = "containers")]
pub mod containers;
pub mod debug;
#[cfg(test)]
pub mod difftest;
#[cfg(feature = "dispatch")]
pub mod dispatch;
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
#[cfg(feature = "oc582")]
pub mod oc582;
#[cfg(feature = "oc583")]
pub mod oc583;
#[cfg(feature = "oc585")]
pub mod oc585;
#[cfg(feature = "oc587")]
pub mod oc587;
#[cfg(feature = "oc588")]
pub mod oc588;
#[cfg(feature = "oc589")]
pub mod oc589;
#[cfg(feature = "oc590")]
pub mod oc590;
#[cfg(feature = "oc591")]
pub mod oc591;
#[cfg(feature = "oc592")]
pub mod oc592;
#[cfg(feature = "oc593")]
pub mod oc593;
#[cfg(feature = "oc594")]
pub mod oc594;
#[cfg(feature = "oc595")]
pub mod oc595;
#[cfg(feature = "oc596")]
pub mod oc596;
#[cfg(feature = "oc597")]
pub mod oc597;
#[cfg(feature = "oc599")]
pub mod oc599;
#[cfg(feature = "oc600")]
pub mod oc600;
#[cfg(feature = "oc601")]
pub mod oc601;
#[cfg(feature = "oc602")]
pub mod oc602;
#[cfg(feature = "oc603")]
pub mod oc603;
#[cfg(feature = "oc604")]
pub mod oc604;
#[cfg(feature = "oc605")]
pub mod oc605;
#[cfg(feature = "oc606")]
pub mod oc606;
#[cfg(feature = "oc607")]
pub mod oc607;
#[cfg(feature = "oc608")]
pub mod oc608;
#[cfg(feature = "oc609")]
pub mod oc609;
pub mod plugin_host;
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
    #[cfg(feature = "oc609")]
    v.extend_from_slice(oc609::OVERRIDES);
    #[cfg(feature = "oc608")]
    v.extend_from_slice(oc608::OVERRIDES);
    #[cfg(feature = "oc607")]
    v.extend_from_slice(oc607::OVERRIDES);
    #[cfg(feature = "oc606")]
    v.extend_from_slice(oc606::OVERRIDES);
    #[cfg(feature = "oc605")]
    v.extend_from_slice(oc605::OVERRIDES);
    #[cfg(feature = "oc604")]
    v.extend_from_slice(oc604::OVERRIDES);
    #[cfg(feature = "oc603")]
    v.extend_from_slice(oc603::OVERRIDES);
    #[cfg(feature = "oc602")]
    v.extend_from_slice(oc602::OVERRIDES);
    #[cfg(feature = "oc601")]
    v.extend_from_slice(oc601::OVERRIDES);
    #[cfg(feature = "oc600")]
    v.extend_from_slice(oc600::OVERRIDES);
    #[cfg(feature = "oc599")]
    v.extend_from_slice(oc599::OVERRIDES);
    #[cfg(feature = "oc597")]
    v.extend_from_slice(oc597::OVERRIDES);
    #[cfg(feature = "oc596")]
    v.extend_from_slice(oc596::OVERRIDES);
    #[cfg(feature = "oc595")]
    v.extend_from_slice(oc595::OVERRIDES);
    #[cfg(feature = "oc582")]
    v.extend_from_slice(oc582::OVERRIDES);
    #[cfg(feature = "oc583")]
    v.extend_from_slice(oc583::OVERRIDES);
    #[cfg(feature = "oc585")]
    v.extend_from_slice(oc585::OVERRIDES);
    #[cfg(feature = "oc587")]
    v.extend_from_slice(oc587::OVERRIDES);
    #[cfg(feature = "oc588")]
    v.extend_from_slice(oc588::OVERRIDES);
    #[cfg(feature = "oc589")]
    v.extend_from_slice(oc589::OVERRIDES);
    #[cfg(feature = "oc590")]
    v.extend_from_slice(oc590::OVERRIDES);
    #[cfg(feature = "oc591")]
    v.extend_from_slice(oc591::OVERRIDES);
    #[cfg(feature = "oc592")]
    v.extend_from_slice(oc592::OVERRIDES);
    #[cfg(feature = "oc593")]
    v.extend_from_slice(oc593::OVERRIDES);
    #[cfg(feature = "oc594")]
    v.extend_from_slice(oc594::OVERRIDES);
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
