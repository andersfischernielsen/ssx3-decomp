//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for world objects, scripters and render-object helpers, each with a differential test against the generated
//! body.

use ssx3_ee_rt::funcs::GuestFn;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];
