//! Native WorldObj post-contact callback, retained uninstalled.

use crate::abi;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

// Automated validation is disabled by the user; no override is installed.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

/// `void WorldObj_v41(WorldObj* self, float delay)` (0x3609f8).
/// The default post-script contact notification does nothing. In particular,
/// Fence's inherited slot0x148 neither consumes F12 nor applies an impulse,
/// so notification after its creation script does not duplicate the first hit.
/// All registers and memory remain unchanged; only two cycles and PC advance.
pub fn world_obj_after_contact(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}
