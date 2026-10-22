//! `src/seg/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod seg_14b10;
pub mod seg_156860;
pub mod seg_1d3510;
pub mod seg_21e5a8;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(seg_14b10::OVERRIDES);
    v.extend_from_slice(seg_156860::OVERRIDES);
    v.extend_from_slice(seg_1d3510::OVERRIDES);
    v.extend_from_slice(seg_21e5a8::OVERRIDES);
    v
}
