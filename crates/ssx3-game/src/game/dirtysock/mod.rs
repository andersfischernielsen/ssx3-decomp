//! `src/dirtysock/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod buddyapi;
pub mod dnasauth;
pub mod gamecomm;
pub mod lobbyutil;
pub mod slogin;
pub mod tags;
pub mod tagsunk;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(buddyapi::OVERRIDES);
    v.extend_from_slice(dnasauth::OVERRIDES);
    v.extend_from_slice(gamecomm::OVERRIDES);
    v.extend_from_slice(lobbyutil::OVERRIDES);
    v.extend_from_slice(slogin::OVERRIDES);
    v.extend_from_slice(tags::OVERRIDES);
    v.extend_from_slice(tagsunk::OVERRIDES);
    v
}
