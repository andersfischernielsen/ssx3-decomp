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
pub mod plugin_host;
#[cfg(feature = "oc620")]
pub mod oc620;
#[cfg(feature = "oc621")]
pub mod oc621;
#[cfg(feature = "oc622")]
pub mod oc622;
#[cfg(feature = "oc623")]
pub mod oc623;
#[cfg(feature = "oc624")]
pub mod oc624;
#[cfg(feature = "oc625")]
pub mod oc625;
#[cfg(feature = "oc626")]
pub mod oc626;
#[cfg(feature = "oc627")]
pub mod oc627;
#[cfg(feature = "oc628")]
pub mod oc628;
#[cfg(feature = "oc629")]
pub mod oc629;
#[cfg(feature = "oc630")]
pub mod oc630;
#[cfg(feature = "oc631")]
pub mod oc631;
#[cfg(feature = "oc632")]
pub mod oc632;
#[cfg(feature = "oc633")]
pub mod oc633;
#[cfg(feature = "oc634")]
pub mod oc634;
#[cfg(feature = "oc635")]
pub mod oc635;
#[cfg(feature = "oc636")]
pub mod oc636;
#[cfg(feature = "oc637")]
pub mod oc637;
#[cfg(feature = "oc638")]
pub mod oc638;
#[cfg(feature = "oc639")]
pub mod oc639;
#[cfg(feature = "oc640")]
pub mod oc640;
#[cfg(feature = "oc642")]
pub mod oc642;
#[cfg(feature = "oc643")]
pub mod oc643;
#[cfg(feature = "oc644")]
pub mod oc644;
#[cfg(feature = "oc645")]
pub mod oc645;
#[cfg(feature = "oc646")]
pub mod oc646;
#[cfg(feature = "oc647")]
pub mod oc647;
#[cfg(feature = "oc648")]
pub mod oc648;
#[cfg(feature = "oc649")]
pub mod oc649;
#[cfg(feature = "oc650")]
pub mod oc650;
#[cfg(feature = "oc651")]
pub mod oc651;
#[cfg(feature = "oc652")]
pub mod oc652;
#[cfg(feature = "oc653")]
pub mod oc653;
#[cfg(feature = "oc654")]
pub mod oc654;
#[cfg(feature = "oc655")]
pub mod oc655;
#[cfg(feature = "oc656")]
pub mod oc656;
#[cfg(feature = "oc657")]
pub mod oc657;
#[cfg(feature = "oc658")]
pub mod oc658;
#[cfg(feature = "oc659")]
pub mod oc659;
#[cfg(feature = "oc660")]
pub mod oc660;
#[cfg(feature = "oc661")]
pub mod oc661;
#[cfg(feature = "oc662")]
pub mod oc662;
#[cfg(feature = "oc663")]
pub mod oc663;
#[cfg(feature = "oc664")]
pub mod oc664;
#[cfg(feature = "oc665")]
pub mod oc665;
#[cfg(feature = "oc666")]
pub mod oc666;
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
    #[cfg(feature = "oc666")]
    v.extend_from_slice(oc666::OVERRIDES);
    #[cfg(feature = "oc665")]
    v.extend_from_slice(oc665::OVERRIDES);
    #[cfg(feature = "oc664")]
    v.extend_from_slice(oc664::OVERRIDES);
    #[cfg(feature = "oc663")]
    v.extend_from_slice(oc663::OVERRIDES);
    #[cfg(feature = "oc662")]
    v.extend_from_slice(oc662::OVERRIDES);
    #[cfg(feature = "oc661")]
    v.extend_from_slice(oc661::OVERRIDES);
    #[cfg(feature = "oc660")]
    v.extend_from_slice(oc660::OVERRIDES);
    #[cfg(feature = "oc659")]
    v.extend_from_slice(oc659::OVERRIDES);
    #[cfg(feature = "oc658")]
    v.extend_from_slice(oc658::OVERRIDES);
    #[cfg(feature = "oc657")]
    v.extend_from_slice(oc657::OVERRIDES);
    #[cfg(feature = "oc656")]
    v.extend_from_slice(oc656::OVERRIDES);
    #[cfg(feature = "oc655")]
    v.extend_from_slice(oc655::OVERRIDES);
    #[cfg(feature = "oc654")]
    v.extend_from_slice(oc654::OVERRIDES);
    #[cfg(feature = "oc653")]
    v.extend_from_slice(oc653::OVERRIDES);
    #[cfg(feature = "oc652")]
    v.extend_from_slice(oc652::OVERRIDES);
    #[cfg(feature = "oc651")]
    v.extend_from_slice(oc651::OVERRIDES);
    #[cfg(feature = "oc650")]
    v.extend_from_slice(oc650::OVERRIDES);
    #[cfg(feature = "oc649")]
    v.extend_from_slice(oc649::OVERRIDES);
    #[cfg(feature = "oc648")]
    v.extend_from_slice(oc648::OVERRIDES);
    #[cfg(feature = "oc647")]
    v.extend_from_slice(oc647::OVERRIDES);
    #[cfg(feature = "oc646")]
    v.extend_from_slice(oc646::OVERRIDES);
    #[cfg(feature = "oc645")]
    v.extend_from_slice(oc645::OVERRIDES);
    #[cfg(feature = "oc644")]
    v.extend_from_slice(oc644::OVERRIDES);
    #[cfg(feature = "oc643")]
    v.extend_from_slice(oc643::OVERRIDES);
    #[cfg(feature = "oc642")]
    v.extend_from_slice(oc642::OVERRIDES);
    #[cfg(feature = "oc640")]
    v.extend_from_slice(oc640::OVERRIDES);
    #[cfg(feature = "oc639")]
    v.extend_from_slice(oc639::OVERRIDES);
    #[cfg(feature = "oc638")]
    v.extend_from_slice(oc638::OVERRIDES);
    #[cfg(feature = "oc637")]
    v.extend_from_slice(oc637::OVERRIDES);
    #[cfg(feature = "oc636")]
    v.extend_from_slice(oc636::OVERRIDES);
    #[cfg(feature = "oc635")]
    v.extend_from_slice(oc635::OVERRIDES);
    #[cfg(feature = "oc634")]
    v.extend_from_slice(oc634::OVERRIDES);
    #[cfg(feature = "oc633")]
    v.extend_from_slice(oc633::OVERRIDES);
    #[cfg(feature = "oc632")]
    v.extend_from_slice(oc632::OVERRIDES);
    #[cfg(feature = "oc631")]
    v.extend_from_slice(oc631::OVERRIDES);
    #[cfg(feature = "oc630")]
    v.extend_from_slice(oc630::OVERRIDES);
    #[cfg(feature = "oc629")]
    v.extend_from_slice(oc629::OVERRIDES);
    #[cfg(feature = "oc628")]
    v.extend_from_slice(oc628::OVERRIDES);
    #[cfg(feature = "oc627")]
    v.extend_from_slice(oc627::OVERRIDES);
    #[cfg(feature = "oc626")]
    v.extend_from_slice(oc626::OVERRIDES);
    #[cfg(feature = "oc625")]
    v.extend_from_slice(oc625::OVERRIDES);
    #[cfg(feature = "oc624")]
    v.extend_from_slice(oc624::OVERRIDES);
    #[cfg(feature = "oc623")]
    v.extend_from_slice(oc623::OVERRIDES);
    #[cfg(feature = "oc622")]
    v.extend_from_slice(oc622::OVERRIDES);
    #[cfg(feature = "oc621")]
    v.extend_from_slice(oc621::OVERRIDES);
    #[cfg(feature = "oc620")]
    v.extend_from_slice(oc620::OVERRIDES);
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
