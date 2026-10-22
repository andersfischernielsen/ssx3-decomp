//! `src/dirtysock/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod dnasauth;
pub mod gamecomm;
pub mod tags;
pub mod tags2;
pub mod tagsunk;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(dnasauth::OVERRIDES);
    v.extend_from_slice(gamecomm::OVERRIDES);
    v.extend_from_slice(tags::OVERRIDES);
    v.extend_from_slice(tags2::OVERRIDES);
    v.extend_from_slice(tagsunk::OVERRIDES);
    v
}
