//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for rider ground state and race progress (walls, boost, drag, timers, speeds, AI path, cruise input), each with a differential test against the generated
//! body.

use ssx3_ee_rt::funcs::GuestFn;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];
