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
pub mod guest;
pub mod ps2f;
pub mod rider;

/// Every replacement: (address, function, name). Must match `overrides.txt`.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_e098, rider::build_world_matrix, "Rider_BuildWorldMatrix"),
];

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
    for &(addr, f, name) in OVERRIDES {
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
    fn overrides_txt_matches_table() {
        let txt: Vec<u32> = include_str!("../overrides.txt")
            .lines()
            .filter_map(|l| {
                let w = l.split('#').next()?.split_whitespace().next()?;
                u32::from_str_radix(w.trim_start_matches("0x"), 16).ok()
            })
            .collect();
        let table: Vec<u32> = super::OVERRIDES.iter().map(|o| o.0).collect();
        assert_eq!(txt, table, "overrides.txt and OVERRIDES differ");
    }
}
