//! Install three ports that were written earlier but left out of `OVERRIDES`
//! because their tests only cover the pre-hit paths (`oc34`'s two rider rail
//! helpers and `ai`'s `cBE_getBE`). The race replay gate is the real cover for
//! the untested branches.

use crate::ai::be_library_get_instance;
use crate::oc34::{rider_handle_wall_rail_contact, rider_try_start_railslide};
use ssx3_ee_rt::funcs::GuestFn;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0014_dc80, be_library_get_instance, "cBE_getBE"),
    (0x0010_6848, rider_try_start_railslide, "Rider_TryStartRailslide"),
    (0x0010_6f78, rider_handle_wall_rail_contact, "Rider_HandleWallRailContact"),
];
