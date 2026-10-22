//! `src/ai/motion/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod wipeoutmotion;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(wipeoutmotion::OVERRIDES);
    v
}
