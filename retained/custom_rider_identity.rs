//! Inspected native identity leaves, retained uninstalled.
//! No tests were added or run, as requested by the user.

use crate::abi;
use ssx3_ee_rt::cpu::reg::{A0, V0};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

/// Rider_GetIndex0x140b80: this rider's index, not a surface or board ID.
pub fn rider_get_index(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x86c));
    abi::ret(c);
}

/// Rider_IsHuman0x140bc0: the native rider's human-control flag.
pub fn rider_is_human(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x874));
    abi::ret(c);
}
