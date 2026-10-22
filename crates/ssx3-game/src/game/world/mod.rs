//! `src/world/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod streamman;
pub mod world;
pub mod worldcache;
pub mod worldview;
pub mod wscriptcache;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(streamman::OVERRIDES);
    v.extend_from_slice(world::OVERRIDES);
    v.extend_from_slice(worldcache::OVERRIDES);
    v.extend_from_slice(worldview::OVERRIDES);
    v.extend_from_slice(wscriptcache::OVERRIDES);
    v
}
