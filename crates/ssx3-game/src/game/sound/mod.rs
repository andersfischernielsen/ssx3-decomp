//! `src/sound/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod icepick;
pub mod asyncsys;
pub mod bankmanager;
pub mod bankmonitor;
pub mod ps2soundman;
pub mod soundsys;
pub mod ssxaudio;
pub mod streamsys;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend(icepick::overrides());
    v.extend_from_slice(asyncsys::OVERRIDES);
    v.extend_from_slice(bankmanager::OVERRIDES);
    v.extend_from_slice(bankmonitor::OVERRIDES);
    v.extend_from_slice(ps2soundman::OVERRIDES);
    v.extend_from_slice(soundsys::OVERRIDES);
    v.extend_from_slice(ssxaudio::OVERRIDES);
    v.extend_from_slice(streamsys::OVERRIDES);
    v
}
