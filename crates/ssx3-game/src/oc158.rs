//! Placeholder module.
//!
//! `cBE_getBE` (0x14dc80, ported in `ai` as `be_library_get_instance`),
//! `Rider_TryStartRailslide` (0x106848) and `Rider_HandleWallRailContact`
//! (0x106f78, both ported in `oc34`) are *not* installed: each has a host-clean
//! test that covers only its pre-hit path, but installing any of them diverges
//! the browser/unwind replay at 0x50b650 (the rail-hit / BE-cold branches run
//! there and are not reproduced). They stay ported in their modules.

use ssx3_ee_rt::funcs::GuestFn;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];
