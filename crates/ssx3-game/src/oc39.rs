//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for lighting and view setup (irradiance, projection, view matrices, world painter colours, weather/sun blends, HUD meter), each with a differential test against the generated
//! body.

use ssx3_ee_rt::funcs::GuestFn;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];
