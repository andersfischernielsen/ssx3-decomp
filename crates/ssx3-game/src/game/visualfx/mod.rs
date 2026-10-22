//! `src/visualfx/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod angerfx;
pub mod attachedemitterfx;
pub mod avalanche;
pub mod boardsparkfx;
pub mod boardtrailfx;
pub mod boardwakefx;
pub mod crowdrender2d;
pub mod hudfx;
pub mod lensfx;
pub mod renderstateman;
pub mod worldlightman;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(angerfx::OVERRIDES);
    v.extend_from_slice(attachedemitterfx::OVERRIDES);
    v.extend_from_slice(avalanche::OVERRIDES);
    v.extend_from_slice(boardsparkfx::OVERRIDES);
    v.extend_from_slice(boardtrailfx::OVERRIDES);
    v.extend_from_slice(boardwakefx::OVERRIDES);
    v.extend_from_slice(crowdrender2d::OVERRIDES);
    v.extend_from_slice(hudfx::OVERRIDES);
    v.extend_from_slice(lensfx::OVERRIDES);
    v.extend_from_slice(renderstateman::OVERRIDES);
    v.extend_from_slice(worldlightman::OVERRIDES);
    v
}
