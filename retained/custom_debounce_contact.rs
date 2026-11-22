//! Native Debounce contact callback (0x003614f8), retained uninstalled.
//!
//! Generated EE is the authority. No tests or replays were added or run.

use crate::abi;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0036_14f8, debounce_v40, "Debounce_v40")];

/// `void Debounce_v40(Debounce*, ContactB*, ContactA*, RiderInterface*)`.
/// Consume the contact without dispatching an object script or changing any
/// register/memory value. An attached debounce therefore suppresses repeated
/// contact scripts while the node remains installed.
pub fn debounce_v40(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}
