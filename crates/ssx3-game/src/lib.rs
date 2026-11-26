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
#[cfg(feature = "oc814")]
pub mod oc814;
#[cfg(feature = "oc815")]
pub mod oc815;
#[cfg(feature = "oc816")]
pub mod oc816;
#[cfg(feature = "oc817")]
pub mod oc817;
#[cfg(feature = "oc818")]
pub mod oc818;
#[cfg(feature = "oc819")]
pub mod oc819;
#[cfg(feature = "oc820")]
pub mod oc820;
#[cfg(feature = "oc821")]
pub mod oc821;
#[cfg(feature = "oc822")]
pub mod oc822;
#[cfg(feature = "oc823")]
pub mod oc823;
#[cfg(feature = "oc824")]
pub mod oc824;
#[cfg(feature = "oc825")]
pub mod oc825;
#[cfg(feature = "oc826")]
pub mod oc826;
#[cfg(feature = "oc827")]
pub mod oc827;
#[cfg(feature = "oc828")]
pub mod oc828;
#[cfg(feature = "oc829")]
pub mod oc829;
#[cfg(feature = "oc830")]
pub mod oc830;
#[cfg(feature = "oc831")]
pub mod oc831;
#[cfg(feature = "oc832")]
pub mod oc832;
#[cfg(feature = "oc833")]
pub mod oc833;
#[cfg(feature = "oc834")]
pub mod oc834;
#[cfg(feature = "oc835")]
pub mod oc835;
#[cfg(feature = "oc836")]
pub mod oc836;
#[cfg(feature = "oc837")]
pub mod oc837;
#[cfg(feature = "oc838")]
pub mod oc838;
#[cfg(feature = "oc839")]
pub mod oc839;
#[cfg(feature = "oc840")]
pub mod oc840;
#[cfg(feature = "oc841")]
pub mod oc841;
#[cfg(feature = "oc842")]
pub mod oc842;
#[cfg(feature = "oc843")]
pub mod oc843;
#[cfg(feature = "oc844")]
pub mod oc844;
#[cfg(feature = "oc845")]
pub mod oc845;
#[cfg(feature = "oc846")]
pub mod oc846;
#[cfg(feature = "oc847")]
pub mod oc847;
#[cfg(feature = "oc848")]
pub mod oc848;
#[cfg(feature = "oc849")]
pub mod oc849;
#[cfg(feature = "oc850")]
pub mod oc850;
#[cfg(feature = "oc851")]
pub mod oc851;
#[cfg(feature = "oc852")]
pub mod oc852;
#[cfg(feature = "oc853")]
pub mod oc853;
#[cfg(feature = "oc854")]
pub mod oc854;
#[cfg(feature = "oc855")]
pub mod oc855;
#[cfg(feature = "oc856")]
pub mod oc856;
#[cfg(feature = "oc857")]
pub mod oc857;
#[cfg(feature = "oc858")]
pub mod oc858;
#[cfg(feature = "oc859")]
pub mod oc859;
#[cfg(feature = "oc860")]
pub mod oc860;
#[cfg(feature = "oc861")]
pub mod oc861;
#[cfg(feature = "oc862")]
pub mod oc862;
#[cfg(feature = "oc863")]
pub mod oc863;
#[cfg(feature = "oc864")]
pub mod oc864;
#[cfg(feature = "oc865")]
pub mod oc865;
#[cfg(feature = "oc866")]
pub mod oc866;
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
    #[cfg(feature = "oc851")]
    v.extend_from_slice(oc851::OVERRIDES);
    #[cfg(feature = "oc852")]
    v.extend_from_slice(oc852::OVERRIDES);
    #[cfg(feature = "oc853")]
    v.extend_from_slice(oc853::OVERRIDES);
    #[cfg(feature = "oc854")]
    v.extend_from_slice(oc854::OVERRIDES);
    #[cfg(feature = "oc855")]
    v.extend_from_slice(oc855::OVERRIDES);
    #[cfg(feature = "oc856")]
    v.extend_from_slice(oc856::OVERRIDES);
    #[cfg(feature = "oc857")]
    v.extend_from_slice(oc857::OVERRIDES);
    #[cfg(feature = "oc858")]
    v.extend_from_slice(oc858::OVERRIDES);
    #[cfg(feature = "oc859")]
    v.extend_from_slice(oc859::OVERRIDES);
    #[cfg(feature = "oc860")]
    v.extend_from_slice(oc860::OVERRIDES);
    #[cfg(feature = "oc861")]
    v.extend_from_slice(oc861::OVERRIDES);
    #[cfg(feature = "oc862")]
    v.extend_from_slice(oc862::OVERRIDES);
    #[cfg(feature = "oc863")]
    v.extend_from_slice(oc863::OVERRIDES);
    #[cfg(feature = "oc864")]
    v.extend_from_slice(oc864::OVERRIDES);
    #[cfg(feature = "oc865")]
    v.extend_from_slice(oc865::OVERRIDES);
    #[cfg(feature = "oc866")]
    v.extend_from_slice(oc866::OVERRIDES);
    #[cfg(feature = "oc830")]
    v.extend_from_slice(oc830::OVERRIDES);
    #[cfg(feature = "oc831")]
    v.extend_from_slice(oc831::OVERRIDES);
    #[cfg(feature = "oc832")]
    v.extend_from_slice(oc832::OVERRIDES);
    #[cfg(feature = "oc833")]
    v.extend_from_slice(oc833::OVERRIDES);
    #[cfg(feature = "oc834")]
    v.extend_from_slice(oc834::OVERRIDES);
    #[cfg(feature = "oc835")]
    v.extend_from_slice(oc835::OVERRIDES);
    #[cfg(feature = "oc836")]
    v.extend_from_slice(oc836::OVERRIDES);
    #[cfg(feature = "oc837")]
    v.extend_from_slice(oc837::OVERRIDES);
    #[cfg(feature = "oc838")]
    v.extend_from_slice(oc838::OVERRIDES);
    #[cfg(feature = "oc839")]
    v.extend_from_slice(oc839::OVERRIDES);
    #[cfg(feature = "oc840")]
    v.extend_from_slice(oc840::OVERRIDES);
    #[cfg(feature = "oc841")]
    v.extend_from_slice(oc841::OVERRIDES);
    #[cfg(feature = "oc842")]
    v.extend_from_slice(oc842::OVERRIDES);
    #[cfg(feature = "oc843")]
    v.extend_from_slice(oc843::OVERRIDES);
    #[cfg(feature = "oc844")]
    v.extend_from_slice(oc844::OVERRIDES);
    #[cfg(feature = "oc845")]
    v.extend_from_slice(oc845::OVERRIDES);
    #[cfg(feature = "oc846")]
    v.extend_from_slice(oc846::OVERRIDES);
    #[cfg(feature = "oc847")]
    v.extend_from_slice(oc847::OVERRIDES);
    #[cfg(feature = "oc848")]
    v.extend_from_slice(oc848::OVERRIDES);
    #[cfg(feature = "oc849")]
    v.extend_from_slice(oc849::OVERRIDES);
    #[cfg(feature = "oc850")]
    v.extend_from_slice(oc850::OVERRIDES);
    #[cfg(feature = "oc814")]
    v.extend_from_slice(oc814::OVERRIDES);
    #[cfg(feature = "oc815")]
    v.extend_from_slice(oc815::OVERRIDES);
    #[cfg(feature = "oc816")]
    v.extend_from_slice(oc816::OVERRIDES);
    #[cfg(feature = "oc817")]
    v.extend_from_slice(oc817::OVERRIDES);
    #[cfg(feature = "oc818")]
    v.extend_from_slice(oc818::OVERRIDES);
    #[cfg(feature = "oc819")]
    v.extend_from_slice(oc819::OVERRIDES);
    #[cfg(feature = "oc820")]
    v.extend_from_slice(oc820::OVERRIDES);
    #[cfg(feature = "oc821")]
    v.extend_from_slice(oc821::OVERRIDES);
    #[cfg(feature = "oc822")]
    v.extend_from_slice(oc822::OVERRIDES);
    #[cfg(feature = "oc823")]
    v.extend_from_slice(oc823::OVERRIDES);
    #[cfg(feature = "oc824")]
    v.extend_from_slice(oc824::OVERRIDES);
    #[cfg(feature = "oc825")]
    v.extend_from_slice(oc825::OVERRIDES);
    #[cfg(feature = "oc826")]
    v.extend_from_slice(oc826::OVERRIDES);
    #[cfg(feature = "oc827")]
    v.extend_from_slice(oc827::OVERRIDES);
    #[cfg(feature = "oc828")]
    v.extend_from_slice(oc828::OVERRIDES);
    #[cfg(feature = "oc829")]
    v.extend_from_slice(oc829::OVERRIDES);
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
