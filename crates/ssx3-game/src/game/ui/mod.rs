//! `src/ui/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod focusman;
pub mod uianimation;
pub mod uiengine;
pub mod uilistbox;
pub mod uimenu;
pub mod uiscreen;
pub mod uistate;
pub mod uistatestack;
pub mod uitext;
pub mod uithing;
pub mod uivector;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(focusman::OVERRIDES);
    v.extend_from_slice(uianimation::OVERRIDES);
    v.extend_from_slice(uiengine::OVERRIDES);
    v.extend_from_slice(uilistbox::OVERRIDES);
    v.extend_from_slice(uimenu::OVERRIDES);
    v.extend_from_slice(uiscreen::OVERRIDES);
    v.extend_from_slice(uistate::OVERRIDES);
    v.extend_from_slice(uistatestack::OVERRIDES);
    v.extend_from_slice(uitext::OVERRIDES);
    v.extend_from_slice(uithing::OVERRIDES);
    v.extend_from_slice(uivector::OVERRIDES);
    v
}
