//! `src/camera/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod camera;
pub mod cameraalgolist;
pub mod cameraalgorithms;
pub mod cameracontroller;
pub mod script;
pub mod trigger;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend(script::overrides());
    v.extend(trigger::overrides());
    v.extend_from_slice(camera::OVERRIDES);
    v.extend_from_slice(cameraalgolist::OVERRIDES);
    v.extend_from_slice(cameraalgorithms::OVERRIDES);
    v.extend_from_slice(cameracontroller::OVERRIDES);
    v
}
