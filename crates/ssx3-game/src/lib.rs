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

use ssx3_ee_rt::funcs::GuestFn;

pub mod abi;
/// Debug-only overrides behind environment switches.
pub mod debug;
#[cfg(feature = "oc28")]
pub mod oc28;
#[cfg(feature = "oc27")]
pub mod oc27;
#[cfg(feature = "oc26")]
pub mod oc26;
#[cfg(feature = "oc25")]
pub mod oc25;
#[cfg(feature = "oc24")]
pub mod oc24;
#[cfg(feature = "oc23")]
pub mod oc23;
#[cfg(feature = "oc22")]
pub mod oc22;
#[cfg(feature = "oc21")]
pub mod oc21;
#[cfg(feature = "oc20")]
pub mod oc20;
#[cfg(feature = "oc19")]
pub mod oc19;
#[cfg(feature = "oc18")]
pub mod oc18;
#[cfg(feature = "oc17")]
pub mod oc17;
#[cfg(feature = "oc16")]
pub mod oc16;
#[cfg(feature = "oc15")]
pub mod oc15;
#[cfg(feature = "oc14")]
pub mod oc14;
#[cfg(feature = "oc13")]
pub mod oc13;
#[cfg(feature = "oc12")]
pub mod oc12;
#[cfg(feature = "oc11")]
pub mod oc11;
#[cfg(feature = "oc10")]
pub mod oc10;
#[cfg(feature = "oc9")]
pub mod oc9;
#[cfg(feature = "oc8")]
pub mod oc8;
#[cfg(feature = "oc7")]
pub mod oc7;
#[cfg(feature = "oc6")]
pub mod oc6;
#[cfg(feature = "oc5")]
pub mod oc5;
#[cfg(feature = "oc4")]
pub mod oc4;
#[cfg(feature = "oc3")]
pub mod oc3;
#[cfg(feature = "oc2")]
pub mod oc2;
#[cfg(feature = "oc1")]
pub mod oc1;
pub mod call;
#[cfg(test)]
pub mod difftest;
pub mod events;
pub mod guest;
pub mod mmi;
pub mod ps2f;
pub mod rider;
#[cfg(feature = "math")]
pub mod math;
#[cfg(feature = "anim")]
pub mod anim;
#[cfg(feature = "camera")]
pub mod camera;
#[cfg(feature = "render")]
pub mod render;
#[cfg(feature = "sound")]
pub mod sound;
#[cfg(feature = "state")]
pub mod state;
#[cfg(feature = "util")]
pub mod util;
#[cfg(feature = "dispatch")]
pub mod dispatch;
#[cfg(feature = "rider_states")]
pub mod rider_states;
#[cfg(feature = "skeleton")]
pub mod skeleton;
#[cfg(feature = "chase")]
pub mod chase;
#[cfg(feature = "ai")]
pub mod ai;
#[cfg(feature = "world")]
pub mod world;
#[cfg(feature = "ui")]
pub mod ui;
#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "containers")]
pub mod containers;
#[cfg(feature = "world_cache")]
pub mod world_cache;
#[cfg(feature = "replay_script")]
pub mod replay_script;
#[cfg(feature = "streaming")]
pub mod streaming;
// Phase 3: game files from the host (SSX3_NATIVE_FILES); uses `streaming`.
#[cfg(feature = "streaming")]
pub mod native_files;
#[cfg(feature = "graphics")]
pub mod graphics;
#[cfg(feature = "rider_logic")]
pub mod rider_logic;
#[cfg(feature = "world_painter")]
pub mod world_painter;
#[cfg(feature = "audio_system")]
pub mod audio_system;
#[cfg(feature = "snd_lib")]
pub mod snd_lib;

/// Replacements outside the work-package modules: (address, function, name).
const CORE: &[(u32, GuestFn, &str)] = &[
    (0x0011_e098, rider::build_world_matrix, "Rider_BuildWorldMatrix"),
];

/// Every installed replacement (core + enabled work packages). Each package
/// module exports its own `OVERRIDES`; all of them must be listed in
/// `overrides.txt`.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v: Vec<(u32, GuestFn, &'static str)> = CORE.to_vec();
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
    #[cfg(feature = "oc1")]
    v.extend_from_slice(oc1::OVERRIDES);
    #[cfg(feature = "oc2")]
    v.extend_from_slice(oc2::OVERRIDES);
    #[cfg(feature = "oc3")]
    v.extend_from_slice(oc3::OVERRIDES);
    #[cfg(feature = "oc4")]
    v.extend_from_slice(oc4::OVERRIDES);
    #[cfg(feature = "oc5")]
    v.extend_from_slice(oc5::OVERRIDES);
    #[cfg(feature = "oc6")]
    v.extend_from_slice(oc6::OVERRIDES);
    #[cfg(feature = "oc7")]
    v.extend_from_slice(oc7::OVERRIDES);
    #[cfg(feature = "oc8")]
    v.extend_from_slice(oc8::OVERRIDES);
    #[cfg(feature = "oc9")]
    v.extend_from_slice(oc9::OVERRIDES);
    #[cfg(feature = "oc10")]
    v.extend_from_slice(oc10::OVERRIDES);
    #[cfg(feature = "oc11")]
    v.extend_from_slice(oc11::OVERRIDES);
    #[cfg(feature = "oc12")]
    v.extend_from_slice(oc12::OVERRIDES);
    #[cfg(feature = "oc13")]
    v.extend_from_slice(oc13::OVERRIDES);
    #[cfg(feature = "oc14")]
    v.extend_from_slice(oc14::OVERRIDES);
    #[cfg(feature = "oc15")]
    v.extend_from_slice(oc15::OVERRIDES);
    #[cfg(feature = "oc16")]
    v.extend_from_slice(oc16::OVERRIDES);
    #[cfg(feature = "oc17")]
    v.extend_from_slice(oc17::OVERRIDES);
    #[cfg(feature = "oc18")]
    v.extend_from_slice(oc18::OVERRIDES);
    #[cfg(feature = "oc19")]
    v.extend_from_slice(oc19::OVERRIDES);
    #[cfg(feature = "oc20")]
    v.extend_from_slice(oc20::OVERRIDES);
    #[cfg(feature = "oc21")]
    v.extend_from_slice(oc21::OVERRIDES);
    #[cfg(feature = "oc22")]
    v.extend_from_slice(oc22::OVERRIDES);
    #[cfg(feature = "oc23")]
    v.extend_from_slice(oc23::OVERRIDES);
    #[cfg(feature = "oc24")]
    v.extend_from_slice(oc24::OVERRIDES);
    #[cfg(feature = "oc25")]
    v.extend_from_slice(oc25::OVERRIDES);
    #[cfg(feature = "oc26")]
    v.extend_from_slice(oc26::OVERRIDES);
    #[cfg(feature = "oc27")]
    v.extend_from_slice(oc27::OVERRIDES);
    #[cfg(feature = "oc28")]
    v.extend_from_slice(oc28::OVERRIDES);
    v
}

/// Install the replacements (call once at boot, before guest code runs).
/// `SSX3_NO_OVERRIDES=1` keeps the generated bodies everywhere (A/B checks);
/// `SSX3_OVERRIDES=addr,addr` installs only the listed ones.
/// Returns how many were installed.
pub fn install() -> usize {
    if std::env::var_os("SSX3_NO_OVERRIDES").is_some() {
        return 0;
    }
    let only: Option<Vec<u32>> = std::env::var("SSX3_OVERRIDES").ok().map(|s| {
        s.split(',')
            .filter_map(|a| u32::from_str_radix(a.trim().trim_start_matches("0x"), 16).ok())
            .collect()
    });
    let mut n = 0;
    for (addr, f, name) in overrides() {
        if only.as_ref().is_some_and(|o| !o.contains(&addr)) {
            continue;
        }
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
            assert!(txt.contains(&a), "{name} ({a:#x}) is not listed in overrides.txt");
        }
    }
}
