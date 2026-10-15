//! Tier B: memory/string utilities with loops (memset, memcpy, EA_Mem*, str*) (docs/investigation/phase2-candidates.md).

use ssx3_ee_rt::funcs::GuestFn;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];
