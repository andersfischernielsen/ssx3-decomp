//! `src/scripter/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod bxscriptengine;
pub mod datamanager;
pub mod scanimmgr;
pub mod scsndmgr;
pub mod ssxscriptengine;
pub mod videngine;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(bxscriptengine::OVERRIDES);
    v.extend_from_slice(datamanager::OVERRIDES);
    v.extend_from_slice(scanimmgr::OVERRIDES);
    v.extend_from_slice(scsndmgr::OVERRIDES);
    v.extend_from_slice(ssxscriptengine::OVERRIDES);
    v.extend_from_slice(videngine::OVERRIDES);
    v
}
