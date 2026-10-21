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
#[cfg(feature = "ee-unwind")]
extern crate ssx3_ee_unwind as ssx3_ee;
#[cfg(all(feature = "ee-threads", not(feature = "ee-unwind")))]
extern crate ssx3_ee_threads as ssx3_ee;

use ssx3_ee_rt::funcs::GuestFn;

pub mod abi;
/// Debug-only overrides behind environment switches.
pub mod capture;
pub mod packetless;
pub mod movie;
pub mod debug;
#[cfg(feature = "oc28")]
pub mod oc28;
#[cfg(feature = "oc29")]
pub mod oc29;
#[cfg(feature = "oc30")]
pub mod oc30;
#[cfg(feature = "oc31")]
pub mod oc31;
#[cfg(feature = "oc32")]
pub mod oc32;
#[cfg(feature = "oc33")]
pub mod oc33;
#[cfg(feature = "oc34")]
pub mod oc34;
#[cfg(feature = "oc35")]
pub mod oc35;
#[cfg(feature = "oc36")]
pub mod oc36;
#[cfg(feature = "oc37")]
pub mod oc37;
#[cfg(feature = "oc38")]
pub mod oc38;
#[cfg(feature = "oc39")]
pub mod oc39;
#[cfg(feature = "oc40")]
pub mod oc40;
#[cfg(feature = "oc41")]
pub mod oc41;
#[cfg(feature = "oc42")]
pub mod oc42;
#[cfg(feature = "oc43")]
pub mod oc43;
#[cfg(feature = "oc44")]
pub mod oc44;
#[cfg(feature = "oc45")]
pub mod oc45;
#[cfg(feature = "oc46")]
pub mod oc46;
#[cfg(feature = "oc47")]
pub mod oc47;
#[cfg(feature = "oc48")]
pub mod oc48;
#[cfg(feature = "oc49")]
pub mod oc49;
#[cfg(feature = "oc50")]
pub mod oc50;
#[cfg(feature = "oc51")]
pub mod oc51;
#[cfg(feature = "oc52")]
pub mod oc52;
#[cfg(feature = "oc53")]
pub mod oc53;
#[cfg(feature = "oc54")]
pub mod oc54;
#[cfg(feature = "oc55")]
pub mod oc55;
#[cfg(feature = "oc56")]
pub mod oc56;
#[cfg(feature = "oc57")]
pub mod oc57;
#[cfg(feature = "oc58")]
pub mod oc58;
#[cfg(feature = "oc59")]
pub mod oc59;
#[cfg(feature = "oc60")]
pub mod oc60;
#[cfg(feature = "oc61")]
pub mod oc61;
#[cfg(feature = "oc62")]
pub mod oc62;
#[cfg(feature = "oc63")]
pub mod oc63;
#[cfg(feature = "oc64")]
pub mod oc64;
#[cfg(feature = "oc65")]
pub mod oc65;
#[cfg(feature = "oc66")]
pub mod oc66;
#[cfg(feature = "oc67")]
pub mod oc67;
#[cfg(feature = "oc68")]
pub mod oc68;
#[cfg(feature = "oc69")]
pub mod oc69;
#[cfg(feature = "oc70")]
pub mod oc70;
#[cfg(feature = "oc71")]
pub mod oc71;
#[cfg(feature = "oc72")]
pub mod oc72;
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
    #[cfg(feature = "oc29")]
    v.extend_from_slice(oc29::OVERRIDES);
    #[cfg(feature = "oc30")]
    v.extend_from_slice(oc30::OVERRIDES);
    #[cfg(feature = "oc31")]
    v.extend_from_slice(oc31::OVERRIDES);
    #[cfg(feature = "oc32")]
    v.extend_from_slice(oc32::OVERRIDES);
    #[cfg(feature = "oc33")]
    v.extend_from_slice(oc33::OVERRIDES);
    #[cfg(feature = "oc34")]
    v.extend_from_slice(oc34::OVERRIDES);
    #[cfg(feature = "oc35")]
    v.extend_from_slice(oc35::OVERRIDES);
    #[cfg(feature = "oc36")]
    v.extend_from_slice(oc36::OVERRIDES);
    #[cfg(feature = "oc37")]
    v.extend_from_slice(oc37::OVERRIDES);
    #[cfg(feature = "oc38")]
    v.extend_from_slice(oc38::OVERRIDES);
    #[cfg(feature = "oc39")]
    v.extend_from_slice(oc39::OVERRIDES);
    #[cfg(feature = "oc40")]
    v.extend_from_slice(oc40::OVERRIDES);
    #[cfg(feature = "oc41")]
    v.extend_from_slice(oc41::OVERRIDES);
    #[cfg(feature = "oc42")]
    v.extend_from_slice(oc42::OVERRIDES);
    #[cfg(feature = "oc43")]
    v.extend_from_slice(oc43::OVERRIDES);
    #[cfg(feature = "oc44")]
    v.extend_from_slice(oc44::OVERRIDES);
    #[cfg(feature = "oc45")]
    v.extend_from_slice(oc45::OVERRIDES);
    #[cfg(feature = "oc46")]
    v.extend_from_slice(oc46::OVERRIDES);
    #[cfg(feature = "oc47")]
    v.extend_from_slice(oc47::OVERRIDES);
    #[cfg(feature = "oc48")]
    v.extend_from_slice(oc48::OVERRIDES);
    #[cfg(feature = "oc49")]
    v.extend_from_slice(oc49::OVERRIDES);
    #[cfg(feature = "oc50")]
    v.extend_from_slice(oc50::OVERRIDES);
    #[cfg(feature = "oc51")]
    v.extend_from_slice(oc51::OVERRIDES);
    #[cfg(feature = "oc52")]
    v.extend_from_slice(oc52::OVERRIDES);
    #[cfg(feature = "oc53")]
    v.extend_from_slice(oc53::OVERRIDES);
    #[cfg(feature = "oc54")]
    v.extend_from_slice(oc54::OVERRIDES);
    #[cfg(feature = "oc55")]
    v.extend_from_slice(oc55::OVERRIDES);
    #[cfg(feature = "oc56")]
    v.extend_from_slice(oc56::OVERRIDES);
    #[cfg(feature = "oc57")]
    v.extend_from_slice(oc57::OVERRIDES);
    #[cfg(feature = "oc58")]
    v.extend_from_slice(oc58::OVERRIDES);
    #[cfg(feature = "oc59")]
    v.extend_from_slice(oc59::OVERRIDES);
    #[cfg(feature = "oc60")]
    v.extend_from_slice(oc60::OVERRIDES);
    #[cfg(feature = "oc61")]
    v.extend_from_slice(oc61::OVERRIDES);
    #[cfg(feature = "oc62")]
    v.extend_from_slice(oc62::OVERRIDES);
    #[cfg(feature = "oc63")]
    v.extend_from_slice(oc63::OVERRIDES);
    #[cfg(feature = "oc64")]
    v.extend_from_slice(oc64::OVERRIDES);
    #[cfg(feature = "oc65")]
    v.extend_from_slice(oc65::OVERRIDES);
    #[cfg(feature = "oc66")]
    v.extend_from_slice(oc66::OVERRIDES);
    #[cfg(feature = "oc67")]
    v.extend_from_slice(oc67::OVERRIDES);
    #[cfg(feature = "oc68")]
    v.extend_from_slice(oc68::OVERRIDES);
    #[cfg(feature = "oc69")]
    v.extend_from_slice(oc69::OVERRIDES);
    #[cfg(feature = "oc70")]
    v.extend_from_slice(oc70::OVERRIDES);
    #[cfg(feature = "oc71")]
    v.extend_from_slice(oc71::OVERRIDES);
    #[cfg(feature = "oc72")]
    v.extend_from_slice(oc72::OVERRIDES);
    v
}

/// Install the replacements (call once at boot, before guest code runs).
/// `SSX3_NO_OVERRIDES=1` keeps the generated bodies everywhere (A/B checks);
/// `SSX3_OVERRIDES=addr,addr` installs only the listed ones,
/// `SSX3_SKIP_OVERRIDES=addr,addr` all but the listed ones.
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
    // `SSX3_SKIP_OVERRIDES=addr,addr`: all but these (bisecting a wave).
    let skip: Vec<u32> = std::env::var("SSX3_SKIP_OVERRIDES")
        .map(|s| s.split(',').filter_map(|a| u32::from_str_radix(a.trim().trim_start_matches("0x"), 16).ok()).collect())
        .unwrap_or_default();
    let mut n = 0;
    for (addr, f, name) in overrides() {
        if only.as_ref().is_some_and(|o| !o.contains(&addr)) || skip.contains(&addr) {
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
