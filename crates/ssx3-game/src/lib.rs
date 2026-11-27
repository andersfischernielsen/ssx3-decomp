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
#[cfg(feature = "oc1000")]
pub mod oc1000;
#[cfg(feature = "oc1001")]
pub mod oc1001;
#[cfg(feature = "oc1002")]
pub mod oc1002;
#[cfg(feature = "oc1003")]
pub mod oc1003;
#[cfg(feature = "oc1004")]
pub mod oc1004;
#[cfg(feature = "oc1005")]
pub mod oc1005;
#[cfg(feature = "oc1006")]
pub mod oc1006;
#[cfg(feature = "oc1007")]
pub mod oc1007;
#[cfg(feature = "oc1008")]
pub mod oc1008;
#[cfg(feature = "oc1009")]
pub mod oc1009;
#[cfg(feature = "oc1010")]
pub mod oc1010;
#[cfg(feature = "oc1011")]
pub mod oc1011;
#[cfg(feature = "oc1012")]
pub mod oc1012;
#[cfg(feature = "oc1013")]
pub mod oc1013;
#[cfg(feature = "oc1014")]
pub mod oc1014;
#[cfg(feature = "oc1015")]
pub mod oc1015;
#[cfg(feature = "oc1016")]
pub mod oc1016;
#[cfg(feature = "oc1017")]
pub mod oc1017;
#[cfg(feature = "oc1018")]
pub mod oc1018;
#[cfg(feature = "oc1019")]
pub mod oc1019;
#[cfg(feature = "oc643")]
pub mod oc643;
#[cfg(feature = "oc658")]
pub mod oc658;
#[cfg(feature = "oc974")]
pub mod oc974;
#[cfg(feature = "oc975")]
pub mod oc975;
#[cfg(feature = "oc976")]
pub mod oc976;
#[cfg(feature = "oc977")]
pub mod oc977;
#[cfg(feature = "oc978")]
pub mod oc978;
#[cfg(feature = "oc979")]
pub mod oc979;
#[cfg(feature = "oc980")]
pub mod oc980;
#[cfg(feature = "oc981")]
pub mod oc981;
#[cfg(feature = "oc982")]
pub mod oc982;
#[cfg(feature = "oc983")]
pub mod oc983;
#[cfg(feature = "oc984")]
pub mod oc984;
#[cfg(feature = "oc985")]
pub mod oc985;
#[cfg(feature = "oc986")]
pub mod oc986;
#[cfg(feature = "oc987")]
pub mod oc987;
#[cfg(feature = "oc988")]
pub mod oc988;
#[cfg(feature = "oc989")]
pub mod oc989;
#[cfg(feature = "oc990")]
pub mod oc990;
#[cfg(feature = "oc991")]
pub mod oc991;
#[cfg(feature = "oc992")]
pub mod oc992;
#[cfg(feature = "oc993")]
pub mod oc993;
#[cfg(feature = "oc994")]
pub mod oc994;
#[cfg(feature = "oc995")]
pub mod oc995;
#[cfg(feature = "oc996")]
pub mod oc996;
#[cfg(feature = "oc997")]
pub mod oc997;
#[cfg(feature = "oc998")]
pub mod oc998;
#[cfg(feature = "oc999")]
pub mod oc999;
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
    #[cfg(feature = "oc1007")]
    v.extend_from_slice(oc1007::OVERRIDES);
    #[cfg(feature = "oc1008")]
    v.extend_from_slice(oc1008::OVERRIDES);
    #[cfg(feature = "oc1009")]
    v.extend_from_slice(oc1009::OVERRIDES);
    #[cfg(feature = "oc1010")]
    v.extend_from_slice(oc1010::OVERRIDES);
    #[cfg(feature = "oc1011")]
    v.extend_from_slice(oc1011::OVERRIDES);
    #[cfg(feature = "oc1012")]
    v.extend_from_slice(oc1012::OVERRIDES);
    #[cfg(feature = "oc1013")]
    v.extend_from_slice(oc1013::OVERRIDES);
    #[cfg(feature = "oc1014")]
    v.extend_from_slice(oc1014::OVERRIDES);
    #[cfg(feature = "oc1015")]
    v.extend_from_slice(oc1015::OVERRIDES);
    #[cfg(feature = "oc1016")]
    v.extend_from_slice(oc1016::OVERRIDES);
    #[cfg(feature = "oc1017")]
    v.extend_from_slice(oc1017::OVERRIDES);
    #[cfg(feature = "oc1018")]
    v.extend_from_slice(oc1018::OVERRIDES);
    #[cfg(feature = "oc1019")]
    v.extend_from_slice(oc1019::OVERRIDES);
    #[cfg(feature = "oc995")]
    v.extend_from_slice(oc995::OVERRIDES);
    #[cfg(feature = "oc996")]
    v.extend_from_slice(oc996::OVERRIDES);
    #[cfg(feature = "oc997")]
    v.extend_from_slice(oc997::OVERRIDES);
    #[cfg(feature = "oc998")]
    v.extend_from_slice(oc998::OVERRIDES);
    #[cfg(feature = "oc999")]
    v.extend_from_slice(oc999::OVERRIDES);
    #[cfg(feature = "oc1000")]
    v.extend_from_slice(oc1000::OVERRIDES);
    #[cfg(feature = "oc1001")]
    v.extend_from_slice(oc1001::OVERRIDES);
    #[cfg(feature = "oc1002")]
    v.extend_from_slice(oc1002::OVERRIDES);
    #[cfg(feature = "oc1003")]
    v.extend_from_slice(oc1003::OVERRIDES);
    #[cfg(feature = "oc1004")]
    v.extend_from_slice(oc1004::OVERRIDES);
    #[cfg(feature = "oc1005")]
    v.extend_from_slice(oc1005::OVERRIDES);
    #[cfg(feature = "oc1006")]
    v.extend_from_slice(oc1006::OVERRIDES);
    #[cfg(feature = "oc974")]
    v.extend_from_slice(oc974::OVERRIDES);
    #[cfg(feature = "oc975")]
    v.extend_from_slice(oc975::OVERRIDES);
    #[cfg(feature = "oc976")]
    v.extend_from_slice(oc976::OVERRIDES);
    #[cfg(feature = "oc977")]
    v.extend_from_slice(oc977::OVERRIDES);
    #[cfg(feature = "oc978")]
    v.extend_from_slice(oc978::OVERRIDES);
    #[cfg(feature = "oc979")]
    v.extend_from_slice(oc979::OVERRIDES);
    #[cfg(feature = "oc980")]
    v.extend_from_slice(oc980::OVERRIDES);
    #[cfg(feature = "oc981")]
    v.extend_from_slice(oc981::OVERRIDES);
    #[cfg(feature = "oc982")]
    v.extend_from_slice(oc982::OVERRIDES);
    #[cfg(feature = "oc983")]
    v.extend_from_slice(oc983::OVERRIDES);
    #[cfg(feature = "oc984")]
    v.extend_from_slice(oc984::OVERRIDES);
    #[cfg(feature = "oc985")]
    v.extend_from_slice(oc985::OVERRIDES);
    #[cfg(feature = "oc986")]
    v.extend_from_slice(oc986::OVERRIDES);
    #[cfg(feature = "oc987")]
    v.extend_from_slice(oc987::OVERRIDES);
    #[cfg(feature = "oc988")]
    v.extend_from_slice(oc988::OVERRIDES);
    #[cfg(feature = "oc989")]
    v.extend_from_slice(oc989::OVERRIDES);
    #[cfg(feature = "oc990")]
    v.extend_from_slice(oc990::OVERRIDES);
    #[cfg(feature = "oc991")]
    v.extend_from_slice(oc991::OVERRIDES);
    #[cfg(feature = "oc992")]
    v.extend_from_slice(oc992::OVERRIDES);
    #[cfg(feature = "oc993")]
    v.extend_from_slice(oc993::OVERRIDES);
    #[cfg(feature = "oc994")]
    v.extend_from_slice(oc994::OVERRIDES);
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
