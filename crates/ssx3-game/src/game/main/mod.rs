//! `src/main/`.

use ssx3_ee_rt::funcs::GuestFn;

pub mod debugmenu;
pub mod game;
pub mod game2;
pub mod gameload;
pub mod gamemode;
pub mod gamerender;
pub mod gamestate;
pub mod loadscreens2;
pub mod loadscreens_prestart;
pub mod racestate;
pub mod sectionman;
pub mod ssxapp;
pub mod ssxappdtor;
pub mod streampause;

/// Every replacement in this directory.
pub fn overrides() -> Vec<(u32, GuestFn, &'static str)> {
    let mut v = Vec::new();
    v.extend_from_slice(debugmenu::OVERRIDES);
    v.extend_from_slice(game::OVERRIDES);
    v.extend_from_slice(game2::OVERRIDES);
    v.extend_from_slice(gameload::OVERRIDES);
    v.extend_from_slice(gamemode::OVERRIDES);
    v.extend_from_slice(gamerender::OVERRIDES);
    v.extend_from_slice(gamestate::OVERRIDES);
    v.extend_from_slice(loadscreens2::OVERRIDES);
    v.extend_from_slice(loadscreens_prestart::OVERRIDES);
    v.extend_from_slice(racestate::OVERRIDES);
    v.extend_from_slice(sectionman::OVERRIDES);
    v.extend_from_slice(ssxapp::OVERRIDES);
    v.extend_from_slice(ssxappdtor::OVERRIDES);
    v.extend_from_slice(streampause::OVERRIDES);
    v
}
