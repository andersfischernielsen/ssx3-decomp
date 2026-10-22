//! `src/intersect/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod aifwddiff;
pub mod riderspheretree;
pub mod worldsphtree;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(aifwddiff::OVERRIDES);
    v.extend_from_slice(riderspheretree::OVERRIDES);
    v.extend_from_slice(worldsphtree::OVERRIDES);
    v
}
