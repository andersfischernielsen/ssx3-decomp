//! `src/animation/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod animmodel;
pub mod animsequencer;
pub mod mdfarchive;
pub mod rideranim;
pub mod rideranimbase;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(animmodel::OVERRIDES);
    v.extend_from_slice(animsequencer::OVERRIDES);
    v.extend_from_slice(mdfarchive::OVERRIDES);
    v.extend_from_slice(rideranim::OVERRIDES);
    v.extend_from_slice(rideranimbase::OVERRIDES);
    v
}
