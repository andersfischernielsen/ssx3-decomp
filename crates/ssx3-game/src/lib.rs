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
#[cfg(feature = "oc700")]
pub mod oc700;
#[cfg(feature = "oc701")]
pub mod oc701;
#[cfg(feature = "oc702")]
pub mod oc702;
#[cfg(feature = "oc703")]
pub mod oc703;
#[cfg(feature = "oc704")]
pub mod oc704;
#[cfg(feature = "oc705")]
pub mod oc705;
#[cfg(feature = "oc706")]
pub mod oc706;
#[cfg(feature = "oc707")]
pub mod oc707;
#[cfg(feature = "oc708")]
pub mod oc708;
#[cfg(feature = "oc709")]
pub mod oc709;
#[cfg(feature = "oc710")]
pub mod oc710;
#[cfg(feature = "oc711")]
pub mod oc711;
#[cfg(feature = "oc712")]
pub mod oc712;
#[cfg(feature = "oc713")]
pub mod oc713;
#[cfg(feature = "oc714")]
pub mod oc714;
#[cfg(feature = "oc715")]
pub mod oc715;
#[cfg(feature = "oc716")]
pub mod oc716;
#[cfg(feature = "oc717")]
pub mod oc717;
#[cfg(feature = "oc718")]
pub mod oc718;
#[cfg(feature = "oc719")]
pub mod oc719;
#[cfg(feature = "oc720")]
pub mod oc720;
#[cfg(feature = "oc721")]
pub mod oc721;
#[cfg(feature = "oc722")]
pub mod oc722;
#[cfg(feature = "oc723")]
pub mod oc723;
#[cfg(feature = "oc724")]
pub mod oc724;
#[cfg(feature = "oc725")]
pub mod oc725;
#[cfg(feature = "oc726")]
pub mod oc726;
#[cfg(feature = "oc727")]
pub mod oc727;
#[cfg(feature = "oc728")]
pub mod oc728;
#[cfg(feature = "oc729")]
pub mod oc729;
#[cfg(feature = "oc731")]
pub mod oc731;
#[cfg(feature = "oc732")]
pub mod oc732;
#[cfg(feature = "oc733")]
pub mod oc733;
#[cfg(feature = "oc734")]
pub mod oc734;
#[cfg(feature = "oc735")]
pub mod oc735;
#[cfg(feature = "oc736")]
pub mod oc736;
#[cfg(feature = "oc737")]
pub mod oc737;
#[cfg(feature = "oc738")]
pub mod oc738;
#[cfg(feature = "oc739")]
pub mod oc739;
#[cfg(feature = "oc740")]
pub mod oc740;
#[cfg(feature = "oc741")]
pub mod oc741;
#[cfg(feature = "oc742")]
pub mod oc742;
#[cfg(feature = "oc743")]
pub mod oc743;
#[cfg(feature = "oc744")]
pub mod oc744;
#[cfg(feature = "oc745")]
pub mod oc745;
#[cfg(feature = "oc746")]
pub mod oc746;
#[cfg(feature = "oc747")]
pub mod oc747;
#[cfg(feature = "oc748")]
pub mod oc748;
#[cfg(feature = "oc749")]
pub mod oc749;
#[cfg(feature = "oc750")]
pub mod oc750;
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
    #[cfg(feature = "oc748")]
    v.extend_from_slice(oc748::OVERRIDES);
    #[cfg(feature = "oc749")]
    v.extend_from_slice(oc749::OVERRIDES);
    #[cfg(feature = "oc750")]
    v.extend_from_slice(oc750::OVERRIDES);
    #[cfg(feature = "oc736")]
    v.extend_from_slice(oc736::OVERRIDES);
    #[cfg(feature = "oc737")]
    v.extend_from_slice(oc737::OVERRIDES);
    #[cfg(feature = "oc738")]
    v.extend_from_slice(oc738::OVERRIDES);
    #[cfg(feature = "oc739")]
    v.extend_from_slice(oc739::OVERRIDES);
    #[cfg(feature = "oc740")]
    v.extend_from_slice(oc740::OVERRIDES);
    #[cfg(feature = "oc741")]
    v.extend_from_slice(oc741::OVERRIDES);
    #[cfg(feature = "oc742")]
    v.extend_from_slice(oc742::OVERRIDES);
    #[cfg(feature = "oc743")]
    v.extend_from_slice(oc743::OVERRIDES);
    #[cfg(feature = "oc744")]
    v.extend_from_slice(oc744::OVERRIDES);
    #[cfg(feature = "oc745")]
    v.extend_from_slice(oc745::OVERRIDES);
    #[cfg(feature = "oc746")]
    v.extend_from_slice(oc746::OVERRIDES);
    #[cfg(feature = "oc747")]
    v.extend_from_slice(oc747::OVERRIDES);
    #[cfg(feature = "oc729")]
    v.extend_from_slice(oc729::OVERRIDES);
    #[cfg(feature = "oc731")]
    v.extend_from_slice(oc731::OVERRIDES);
    #[cfg(feature = "oc732")]
    v.extend_from_slice(oc732::OVERRIDES);
    #[cfg(feature = "oc733")]
    v.extend_from_slice(oc733::OVERRIDES);
    #[cfg(feature = "oc734")]
    v.extend_from_slice(oc734::OVERRIDES);
    #[cfg(feature = "oc735")]
    v.extend_from_slice(oc735::OVERRIDES);
    #[cfg(feature = "oc707")]
    v.extend_from_slice(oc707::OVERRIDES);
    #[cfg(feature = "oc708")]
    v.extend_from_slice(oc708::OVERRIDES);
    #[cfg(feature = "oc709")]
    v.extend_from_slice(oc709::OVERRIDES);
    #[cfg(feature = "oc710")]
    v.extend_from_slice(oc710::OVERRIDES);
    #[cfg(feature = "oc711")]
    v.extend_from_slice(oc711::OVERRIDES);
    #[cfg(feature = "oc712")]
    v.extend_from_slice(oc712::OVERRIDES);
    #[cfg(feature = "oc713")]
    v.extend_from_slice(oc713::OVERRIDES);
    #[cfg(feature = "oc714")]
    v.extend_from_slice(oc714::OVERRIDES);
    #[cfg(feature = "oc715")]
    v.extend_from_slice(oc715::OVERRIDES);
    #[cfg(feature = "oc716")]
    v.extend_from_slice(oc716::OVERRIDES);
    #[cfg(feature = "oc717")]
    v.extend_from_slice(oc717::OVERRIDES);
    #[cfg(feature = "oc718")]
    v.extend_from_slice(oc718::OVERRIDES);
    #[cfg(feature = "oc719")]
    v.extend_from_slice(oc719::OVERRIDES);
    #[cfg(feature = "oc720")]
    v.extend_from_slice(oc720::OVERRIDES);
    #[cfg(feature = "oc721")]
    v.extend_from_slice(oc721::OVERRIDES);
    #[cfg(feature = "oc722")]
    v.extend_from_slice(oc722::OVERRIDES);
    #[cfg(feature = "oc723")]
    v.extend_from_slice(oc723::OVERRIDES);
    #[cfg(feature = "oc724")]
    v.extend_from_slice(oc724::OVERRIDES);
    #[cfg(feature = "oc725")]
    v.extend_from_slice(oc725::OVERRIDES);
    #[cfg(feature = "oc726")]
    v.extend_from_slice(oc726::OVERRIDES);
    #[cfg(feature = "oc727")]
    v.extend_from_slice(oc727::OVERRIDES);
    #[cfg(feature = "oc728")]
    v.extend_from_slice(oc728::OVERRIDES);
    #[cfg(feature = "oc701")]
    v.extend_from_slice(oc701::OVERRIDES);
    #[cfg(feature = "oc702")]
    v.extend_from_slice(oc702::OVERRIDES);
    #[cfg(feature = "oc703")]
    v.extend_from_slice(oc703::OVERRIDES);
    #[cfg(feature = "oc704")]
    v.extend_from_slice(oc704::OVERRIDES);
    #[cfg(feature = "oc705")]
    v.extend_from_slice(oc705::OVERRIDES);
    #[cfg(feature = "oc706")]
    v.extend_from_slice(oc706::OVERRIDES);
    #[cfg(feature = "oc700")]
    v.extend_from_slice(oc700::OVERRIDES);
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
