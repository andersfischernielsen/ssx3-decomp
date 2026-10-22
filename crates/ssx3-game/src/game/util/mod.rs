//! `src/util/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod list;
pub mod locale;
pub mod menu;
pub mod objectinterface;
pub mod statemachine;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(list::OVERRIDES);
    v.extend_from_slice(locale::OVERRIDES);
    v.extend_from_slice(menu::OVERRIDES);
    v.extend_from_slice(objectinterface::OVERRIDES);
    v.extend_from_slice(statemachine::OVERRIDES);
    v
}
