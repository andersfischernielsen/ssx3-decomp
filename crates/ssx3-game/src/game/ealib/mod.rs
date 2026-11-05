//! `src/ealib/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod seg_2dcae8;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(seg_2dcae8::OVERRIDES);
    v
}
