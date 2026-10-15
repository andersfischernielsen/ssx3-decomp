//! Tier B: small functions with calls (wrappers, virtual dispatch, allocators, getters that call) (docs/investigation/phase2-candidates.md).

use ssx3_ee_rt::funcs::GuestFn;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];
