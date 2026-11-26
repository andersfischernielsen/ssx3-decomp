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
#[cfg(feature = "oc643")]
pub mod oc643;
#[cfg(feature = "oc658")]
pub mod oc658;
#[cfg(feature = "oc710")]
pub mod oc710;
#[cfg(feature = "oc751")]
pub mod oc751;
#[cfg(feature = "oc752")]
pub mod oc752;
#[cfg(feature = "oc753")]
pub mod oc753;
#[cfg(feature = "oc754")]
pub mod oc754;
#[cfg(feature = "oc755")]
pub mod oc755;
#[cfg(feature = "oc756")]
pub mod oc756;
#[cfg(feature = "oc757")]
pub mod oc757;
#[cfg(feature = "oc758")]
pub mod oc758;
#[cfg(feature = "oc759")]
pub mod oc759;
#[cfg(feature = "oc760")]
pub mod oc760;
#[cfg(feature = "oc762")]
pub mod oc762;
#[cfg(feature = "oc763")]
pub mod oc763;
#[cfg(feature = "oc764")]
pub mod oc764;
#[cfg(feature = "oc765")]
pub mod oc765;
#[cfg(feature = "oc766")]
pub mod oc766;
#[cfg(feature = "oc767")]
pub mod oc767;
#[cfg(feature = "oc768")]
pub mod oc768;
#[cfg(feature = "oc769")]
pub mod oc769;
#[cfg(feature = "oc770")]
pub mod oc770;
#[cfg(feature = "oc771")]
pub mod oc771;
#[cfg(feature = "oc772")]
pub mod oc772;
#[cfg(feature = "oc773")]
pub mod oc773;
#[cfg(feature = "oc774")]
pub mod oc774;
#[cfg(feature = "oc775")]
pub mod oc775;
#[cfg(feature = "oc776")]
pub mod oc776;
#[cfg(feature = "oc777")]
pub mod oc777;
#[cfg(feature = "oc778")]
pub mod oc778;
#[cfg(feature = "oc779")]
pub mod oc779;
#[cfg(feature = "oc780")]
pub mod oc780;
#[cfg(feature = "oc781")]
pub mod oc781;
#[cfg(feature = "oc782")]
pub mod oc782;
#[cfg(feature = "oc783")]
pub mod oc783;
#[cfg(feature = "oc784")]
pub mod oc784;
#[cfg(feature = "oc785")]
pub mod oc785;
#[cfg(feature = "oc786")]
pub mod oc786;
#[cfg(feature = "oc787")]
pub mod oc787;
#[cfg(feature = "oc788")]
pub mod oc788;
#[cfg(feature = "oc789")]
pub mod oc789;
#[cfg(feature = "oc790")]
pub mod oc790;
#[cfg(feature = "oc791")]
pub mod oc791;
#[cfg(feature = "oc792")]
pub mod oc792;
#[cfg(feature = "oc793")]
pub mod oc793;
#[cfg(feature = "oc794")]
pub mod oc794;
#[cfg(feature = "oc795")]
pub mod oc795;
#[cfg(feature = "oc796")]
pub mod oc796;
#[cfg(feature = "oc797")]
pub mod oc797;
#[cfg(feature = "oc798")]
pub mod oc798;
#[cfg(feature = "oc799")]
pub mod oc799;
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
    #[cfg(feature = "oc786")]
    v.extend_from_slice(oc786::OVERRIDES);
    #[cfg(feature = "oc787")]
    v.extend_from_slice(oc787::OVERRIDES);
    #[cfg(feature = "oc788")]
    v.extend_from_slice(oc788::OVERRIDES);
    #[cfg(feature = "oc789")]
    v.extend_from_slice(oc789::OVERRIDES);
    #[cfg(feature = "oc790")]
    v.extend_from_slice(oc790::OVERRIDES);
    #[cfg(feature = "oc791")]
    v.extend_from_slice(oc791::OVERRIDES);
    #[cfg(feature = "oc792")]
    v.extend_from_slice(oc792::OVERRIDES);
    #[cfg(feature = "oc793")]
    v.extend_from_slice(oc793::OVERRIDES);
    #[cfg(feature = "oc794")]
    v.extend_from_slice(oc794::OVERRIDES);
    #[cfg(feature = "oc795")]
    v.extend_from_slice(oc795::OVERRIDES);
    #[cfg(feature = "oc796")]
    v.extend_from_slice(oc796::OVERRIDES);
    #[cfg(feature = "oc797")]
    v.extend_from_slice(oc797::OVERRIDES);
    #[cfg(feature = "oc798")]
    v.extend_from_slice(oc798::OVERRIDES);
    #[cfg(feature = "oc799")]
    v.extend_from_slice(oc799::OVERRIDES);
    #[cfg(feature = "oc772")]
    v.extend_from_slice(oc772::OVERRIDES);
    #[cfg(feature = "oc773")]
    v.extend_from_slice(oc773::OVERRIDES);
    #[cfg(feature = "oc774")]
    v.extend_from_slice(oc774::OVERRIDES);
    #[cfg(feature = "oc775")]
    v.extend_from_slice(oc775::OVERRIDES);
    #[cfg(feature = "oc776")]
    v.extend_from_slice(oc776::OVERRIDES);
    #[cfg(feature = "oc777")]
    v.extend_from_slice(oc777::OVERRIDES);
    #[cfg(feature = "oc778")]
    v.extend_from_slice(oc778::OVERRIDES);
    #[cfg(feature = "oc779")]
    v.extend_from_slice(oc779::OVERRIDES);
    #[cfg(feature = "oc780")]
    v.extend_from_slice(oc780::OVERRIDES);
    #[cfg(feature = "oc781")]
    v.extend_from_slice(oc781::OVERRIDES);
    #[cfg(feature = "oc782")]
    v.extend_from_slice(oc782::OVERRIDES);
    #[cfg(feature = "oc783")]
    v.extend_from_slice(oc783::OVERRIDES);
    #[cfg(feature = "oc784")]
    v.extend_from_slice(oc784::OVERRIDES);
    #[cfg(feature = "oc785")]
    v.extend_from_slice(oc785::OVERRIDES);
    #[cfg(feature = "oc751")]
    v.extend_from_slice(oc751::OVERRIDES);
    #[cfg(feature = "oc752")]
    v.extend_from_slice(oc752::OVERRIDES);
    #[cfg(feature = "oc753")]
    v.extend_from_slice(oc753::OVERRIDES);
    #[cfg(feature = "oc754")]
    v.extend_from_slice(oc754::OVERRIDES);
    #[cfg(feature = "oc755")]
    v.extend_from_slice(oc755::OVERRIDES);
    #[cfg(feature = "oc756")]
    v.extend_from_slice(oc756::OVERRIDES);
    #[cfg(feature = "oc757")]
    v.extend_from_slice(oc757::OVERRIDES);
    #[cfg(feature = "oc758")]
    v.extend_from_slice(oc758::OVERRIDES);
    #[cfg(feature = "oc759")]
    v.extend_from_slice(oc759::OVERRIDES);
    #[cfg(feature = "oc760")]
    v.extend_from_slice(oc760::OVERRIDES);
    #[cfg(feature = "oc762")]
    v.extend_from_slice(oc762::OVERRIDES);
    #[cfg(feature = "oc763")]
    v.extend_from_slice(oc763::OVERRIDES);
    #[cfg(feature = "oc764")]
    v.extend_from_slice(oc764::OVERRIDES);
    #[cfg(feature = "oc765")]
    v.extend_from_slice(oc765::OVERRIDES);
    #[cfg(feature = "oc766")]
    v.extend_from_slice(oc766::OVERRIDES);
    #[cfg(feature = "oc767")]
    v.extend_from_slice(oc767::OVERRIDES);
    #[cfg(feature = "oc768")]
    v.extend_from_slice(oc768::OVERRIDES);
    #[cfg(feature = "oc769")]
    v.extend_from_slice(oc769::OVERRIDES);
    #[cfg(feature = "oc770")]
    v.extend_from_slice(oc770::OVERRIDES);
    #[cfg(feature = "oc771")]
    v.extend_from_slice(oc771::OVERRIDES);
    #[cfg(feature = "oc710")]
    v.extend_from_slice(oc710::OVERRIDES);
    #[cfg(feature = "oc658")]
    v.extend_from_slice(oc658::OVERRIDES);
    #[cfg(feature = "oc643")]
    v.extend_from_slice(oc643::OVERRIDES);
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

/// Threads the frame driver runs as steps (woken by a signal, run at the top of
/// the frame loop) instead of scheduling them by priority.
pub const STEP_THREAD_ENTRIES: &[u32] = &[
    0x003c_19a8, // SND_EeThread
    0x0031_ac60, // worker
];

/// The guest threads' entries and the waits (syscall return addresses) at which
/// each may unwind and be re-entered later (`docs/plans/single-loop.md`). Every
/// function between the entry and the wait is generated code (with a resume label
/// from `generated/entries.txt`) or a dispatcher-shaped port. A wait anywhere else
/// is done in place on the host stack.
pub const NATIVE_THREAD_ENTRIES: &[(u32, &[u32])] = &[
    (0x0010_0008, &[0x0031_aa8c]), // main thread (crt0 entry): PS2ExecMan_WaitFrame's wait
    (0x0031_ac08, &[0x0031_ac30]), // PS2ExecMan_VSyncThread
    (0x0031_ac60, &[0x0031_aca4]), // PS2ExecMan_WorkerThread: the job wait
    (0x003c_19a8, &[0x003c_19f0]), // SND_EeThread: the mixer wait
    (0x0042_4aa0, &[0x0042_4ae8]), // libkernl_IntrHelperThread (parks once)
    (0x003e_3be0, &[0x003e_3c20]), // EAFile_CdReadThread (parks once)
    (0x0040_09a8, &[0x0040_0a00]), // libcdvd_CallbackThread (parks once)
    (0x0038_2740, &[0x0038_27e0]), // Renderer_DmaKickThreadEntry (parks once)
];

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
