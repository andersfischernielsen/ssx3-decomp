//! `src/ai/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod ai;
pub mod aianimeventmap;
pub mod airpredictor;
pub mod computer;
pub mod control;
pub mod dangerman;
pub mod motion;
pub mod rider;
pub mod ridermetrix;
pub mod trick;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend(control::overrides());
    v.extend(motion::overrides());
    v.extend_from_slice(ai::OVERRIDES);
    v.extend_from_slice(aianimeventmap::OVERRIDES);
    v.extend_from_slice(airpredictor::OVERRIDES);
    v.extend_from_slice(computer::OVERRIDES);
    v.extend_from_slice(dangerman::OVERRIDES);
    v.extend_from_slice(rider::OVERRIDES);
    v.extend_from_slice(ridermetrix::OVERRIDES);
    v.extend_from_slice(trick::OVERRIDES);
    v
}
