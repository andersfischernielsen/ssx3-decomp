//! `src/render/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod bezierman;
pub mod font;
pub mod graphicsman;
pub mod irradiance;
pub mod lightman;
pub mod morton;
pub mod particle;
pub mod ps2bezierman;
pub mod ps2graphicsman;
pub mod ps2lightman;
pub mod render2d;
pub mod shps;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(bezierman::OVERRIDES);
    v.extend_from_slice(font::OVERRIDES);
    v.extend_from_slice(graphicsman::OVERRIDES);
    v.extend_from_slice(irradiance::OVERRIDES);
    v.extend_from_slice(lightman::OVERRIDES);
    v.extend_from_slice(morton::OVERRIDES);
    v.extend_from_slice(particle::OVERRIDES);
    v.extend_from_slice(ps2bezierman::OVERRIDES);
    v.extend_from_slice(ps2graphicsman::OVERRIDES);
    v.extend_from_slice(ps2lightman::OVERRIDES);
    v.extend_from_slice(render2d::OVERRIDES);
    v.extend_from_slice(shps::OVERRIDES);
    v
}
