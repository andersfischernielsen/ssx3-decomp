//! Native segment contact-reference getter, retained uninstalled.
//!
//! Reference only, with no override registration or automated test.

use ssx3_ee_rt::cpu::reg::A0;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const REFERENCE_FRACTION: u32 = 0xa0;

/// `float CollSegment_GetReference(const CollSegment* segment)` (0x340a10).
/// Return the constructor's stored F12 without conversion. Nash's closest
/// contact selector compares each contact fraction against this value.
pub fn coll_segment_get_reference(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(REFERENCE_FRACTION));
    crate::abi::ret(c);
}
