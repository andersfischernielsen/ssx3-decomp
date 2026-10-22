//! `src/movie/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod movieplayer;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(movieplayer::OVERRIDES);
    v
}
