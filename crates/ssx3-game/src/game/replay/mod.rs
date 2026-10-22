//! `src/replay/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod frameptr;
pub mod playbackman;
pub mod replay;
pub mod replay2;
pub mod replaycache;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(frameptr::OVERRIDES);
    v.extend_from_slice(playbackman::OVERRIDES);
    v.extend_from_slice(replay::OVERRIDES);
    v.extend_from_slice(replay2::OVERRIDES);
    v.extend_from_slice(replaycache::OVERRIDES);
    v
}
