//! # ssx3-game
//!
//! Hand-written, typed Rust for SSX 3 functions, replacing the statically
//! recompiled versions one at a time (phase 2 in `docs/plans/roadmap.md`;
//! design in `docs/investigation/phase2-overrides.md`).
//!
//! A replacement is an ordinary `fn(&mut Cpu)` with the same contract as the
//! generated function: arguments and results in the MIPS registers
//! ([`abi`]), guest memory through typed views ([`guest`]), PS2 float
//! semantics ([`ps2f`]), and the same EE cycle cost, so that runs stay
//! byte-identical. [`install`] puts them into the generated crates' slots.

use ssx3_ee_rt::funcs::GuestFn;

pub mod abi;
#[cfg(test)]
pub mod difftest;
pub mod guest;
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
