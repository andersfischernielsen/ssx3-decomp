//! `src/wscript/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod wscriptcompile;
pub mod wscriptman;
pub mod wscriptprocess;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(wscriptcompile::OVERRIDES);
    v.extend_from_slice(wscriptman::OVERRIDES);
    v.extend_from_slice(wscriptprocess::OVERRIDES);
    v
}
