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
#[cfg(feature = "oc1020")]
pub mod oc1020;
#[cfg(feature = "oc1021")]
pub mod oc1021;
#[cfg(feature = "oc1022")]
pub mod oc1022;
#[cfg(feature = "oc1023")]
pub mod oc1023;
#[cfg(feature = "oc1024")]
pub mod oc1024;
#[cfg(feature = "oc1025")]
pub mod oc1025;
#[cfg(feature = "oc1026")]
pub mod oc1026;
#[cfg(feature = "oc1027")]
pub mod oc1027;
#[cfg(feature = "oc1028")]
pub mod oc1028;
#[cfg(feature = "oc1029")]
pub mod oc1029;
#[cfg(feature = "oc1030")]
pub mod oc1030;
#[cfg(feature = "oc1031")]
pub mod oc1031;
#[cfg(feature = "oc1032")]
pub mod oc1032;
#[cfg(feature = "oc1033")]
pub mod oc1033;
#[cfg(feature = "oc1034")]
pub mod oc1034;
#[cfg(feature = "oc1035")]
pub mod oc1035;
#[cfg(feature = "oc1036")]
pub mod oc1036;
#[cfg(feature = "oc1037")]
pub mod oc1037;
#[cfg(feature = "oc1038")]
pub mod oc1038;
#[cfg(feature = "oc1039")]
pub mod oc1039;
#[cfg(feature = "oc1040")]
pub mod oc1040;
#[cfg(feature = "oc1041")]
pub mod oc1041;
#[cfg(feature = "oc1042")]
pub mod oc1042;
#[cfg(feature = "oc1043")]
pub mod oc1043;
#[cfg(feature = "oc1044")]
pub mod oc1044;
#[cfg(feature = "oc1045")]
pub mod oc1045;
#[cfg(feature = "oc1046")]
pub mod oc1046;
#[cfg(feature = "oc1047")]
pub mod oc1047;
#[cfg(feature = "oc1048")]
pub mod oc1048;
#[cfg(feature = "oc1049")]
pub mod oc1049;
#[cfg(feature = "oc1050")]
pub mod oc1050;
#[cfg(feature = "oc1051")]
pub mod oc1051;
#[cfg(feature = "oc1052")]
pub mod oc1052;
#[cfg(feature = "oc1053")]
pub mod oc1053;
#[cfg(feature = "oc1054")]
pub mod oc1054;
#[cfg(feature = "oc1055")]
pub mod oc1055;
#[cfg(feature = "oc1056")]
pub mod oc1056;
#[cfg(feature = "oc1057")]
pub mod oc1057;
#[cfg(feature = "oc1058")]
pub mod oc1058;
#[cfg(feature = "oc1059")]
pub mod oc1059;
#[cfg(feature = "oc1060")]
pub mod oc1060;
#[cfg(feature = "oc1061")]
pub mod oc1061;
#[cfg(feature = "oc1062")]
pub mod oc1062;
#[cfg(feature = "oc1063")]
pub mod oc1063;
#[cfg(feature = "oc1064")]
pub mod oc1064;
#[cfg(feature = "oc1065")]
pub mod oc1065;
#[cfg(feature = "oc1066")]
pub mod oc1066;
#[cfg(feature = "oc1067")]
pub mod oc1067;
#[cfg(feature = "oc1068")]
pub mod oc1068;
#[cfg(feature = "oc1069")]
pub mod oc1069;
#[cfg(feature = "oc1070")]
pub mod oc1070;
#[cfg(feature = "oc1071")]
pub mod oc1071;
#[cfg(feature = "oc1072")]
pub mod oc1072;
#[cfg(feature = "oc1073")]
pub mod oc1073;
#[cfg(feature = "oc643")]
pub mod oc643;
#[cfg(feature = "oc658")]
pub mod oc658;
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
    #[cfg(feature = "oc1054")]
    v.extend_from_slice(oc1054::OVERRIDES);
    #[cfg(feature = "oc1055")]
    v.extend_from_slice(oc1055::OVERRIDES);
    #[cfg(feature = "oc1056")]
    v.extend_from_slice(oc1056::OVERRIDES);
    #[cfg(feature = "oc1057")]
    v.extend_from_slice(oc1057::OVERRIDES);
    #[cfg(feature = "oc1058")]
    v.extend_from_slice(oc1058::OVERRIDES);
    #[cfg(feature = "oc1059")]
    v.extend_from_slice(oc1059::OVERRIDES);
    #[cfg(feature = "oc1060")]
    v.extend_from_slice(oc1060::OVERRIDES);
    #[cfg(feature = "oc1061")]
    v.extend_from_slice(oc1061::OVERRIDES);
    #[cfg(feature = "oc1062")]
    v.extend_from_slice(oc1062::OVERRIDES);
    #[cfg(feature = "oc1063")]
    v.extend_from_slice(oc1063::OVERRIDES);
    #[cfg(feature = "oc1064")]
    v.extend_from_slice(oc1064::OVERRIDES);
    #[cfg(feature = "oc1065")]
    v.extend_from_slice(oc1065::OVERRIDES);
    #[cfg(feature = "oc1066")]
    v.extend_from_slice(oc1066::OVERRIDES);
    #[cfg(feature = "oc1067")]
    v.extend_from_slice(oc1067::OVERRIDES);
    #[cfg(feature = "oc1068")]
    v.extend_from_slice(oc1068::OVERRIDES);
    #[cfg(feature = "oc1069")]
    v.extend_from_slice(oc1069::OVERRIDES);
    #[cfg(feature = "oc1070")]
    v.extend_from_slice(oc1070::OVERRIDES);
    #[cfg(feature = "oc1071")]
    v.extend_from_slice(oc1071::OVERRIDES);
    #[cfg(feature = "oc1072")]
    v.extend_from_slice(oc1072::OVERRIDES);
    #[cfg(feature = "oc1073")]
    v.extend_from_slice(oc1073::OVERRIDES);
    #[cfg(feature = "oc1039")]
    v.extend_from_slice(oc1039::OVERRIDES);
    #[cfg(feature = "oc1040")]
    v.extend_from_slice(oc1040::OVERRIDES);
    #[cfg(feature = "oc1041")]
    v.extend_from_slice(oc1041::OVERRIDES);
    #[cfg(feature = "oc1042")]
    v.extend_from_slice(oc1042::OVERRIDES);
    #[cfg(feature = "oc1043")]
    v.extend_from_slice(oc1043::OVERRIDES);
    #[cfg(feature = "oc1044")]
    v.extend_from_slice(oc1044::OVERRIDES);
    #[cfg(feature = "oc1045")]
    v.extend_from_slice(oc1045::OVERRIDES);
    #[cfg(feature = "oc1046")]
    v.extend_from_slice(oc1046::OVERRIDES);
    #[cfg(feature = "oc1047")]
    v.extend_from_slice(oc1047::OVERRIDES);
    #[cfg(feature = "oc1048")]
    v.extend_from_slice(oc1048::OVERRIDES);
    #[cfg(feature = "oc1049")]
    v.extend_from_slice(oc1049::OVERRIDES);
    #[cfg(feature = "oc1050")]
    v.extend_from_slice(oc1050::OVERRIDES);
    #[cfg(feature = "oc1051")]
    v.extend_from_slice(oc1051::OVERRIDES);
    #[cfg(feature = "oc1052")]
    v.extend_from_slice(oc1052::OVERRIDES);
    #[cfg(feature = "oc1053")]
    v.extend_from_slice(oc1053::OVERRIDES);
    #[cfg(feature = "oc1025")]
    v.extend_from_slice(oc1025::OVERRIDES);
    #[cfg(feature = "oc1026")]
    v.extend_from_slice(oc1026::OVERRIDES);
    #[cfg(feature = "oc1027")]
    v.extend_from_slice(oc1027::OVERRIDES);
    #[cfg(feature = "oc1028")]
    v.extend_from_slice(oc1028::OVERRIDES);
    #[cfg(feature = "oc1029")]
    v.extend_from_slice(oc1029::OVERRIDES);
    #[cfg(feature = "oc1030")]
    v.extend_from_slice(oc1030::OVERRIDES);
    #[cfg(feature = "oc1031")]
    v.extend_from_slice(oc1031::OVERRIDES);
    #[cfg(feature = "oc1032")]
    v.extend_from_slice(oc1032::OVERRIDES);
    #[cfg(feature = "oc1033")]
    v.extend_from_slice(oc1033::OVERRIDES);
    #[cfg(feature = "oc1034")]
    v.extend_from_slice(oc1034::OVERRIDES);
    #[cfg(feature = "oc1035")]
    v.extend_from_slice(oc1035::OVERRIDES);
    #[cfg(feature = "oc1036")]
    v.extend_from_slice(oc1036::OVERRIDES);
    #[cfg(feature = "oc1037")]
    v.extend_from_slice(oc1037::OVERRIDES);
    #[cfg(feature = "oc1038")]
    v.extend_from_slice(oc1038::OVERRIDES);
    #[cfg(feature = "oc1020")]
    v.extend_from_slice(oc1020::OVERRIDES);
    #[cfg(feature = "oc1021")]
    v.extend_from_slice(oc1021::OVERRIDES);
    #[cfg(feature = "oc1022")]
    v.extend_from_slice(oc1022::OVERRIDES);
    #[cfg(feature = "oc1023")]
    v.extend_from_slice(oc1023::OVERRIDES);
    #[cfg(feature = "oc1024")]
    v.extend_from_slice(oc1024::OVERRIDES);
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
