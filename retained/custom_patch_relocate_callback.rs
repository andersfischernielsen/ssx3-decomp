//! Native SND_RelocatePatchCallback (0x3c3ea8), retained uninstalled.
//!
//! Generated EE defines the register, memory and cycle contract. No test,
//! build, replay or game execution accompanies this retained translation.

use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

const RELOCATION_DESTINATION: u32 = 0x64;
const PARSED_ADDRESS: u32 = 0x68;

#[inline]
fn r(c: &Cpu, register: usize) -> u32 {
    c.g[register][0] as u32
}

/// `int SND_RelocatePatchCallback(parsed_patch, new_base, first_address)`.
/// Initialize the shared first-address word from parsed +0x68 when zero,
/// then store `new_base + parsed_address - first_address` through parsed
/// +0x64. These are parser scratch fields; this leaf identifies no serialized
/// BNK tag and calls no other function.
pub fn snd_relocate_patch_callback(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw(r(c, A2));
    c.g[A3][0] = c.g[A0][0];
    let origin_exists = c.g[V0][0] != 0;
    c.g[T0][0] = c.g[A1][0];
    if !origin_exists {
        c.cycles += 2;
        c.g[V0][0] = c.lw(r(c, A3).wrapping_add(PARSED_ADDRESS));
        c.sw(r(c, A2), c.g[V0][0]);
    }

    c.cycles += 8;
    c.g[V1][0] = c.lw(r(c, A3).wrapping_add(PARSED_ADDRESS));
    c.g[V0][0] = 0;
    c.g[A0][0] = c.lw(r(c, A2));
    c.g[A1][0] = c.lw(r(c, A3).wrapping_add(RELOCATION_DESTINATION));
    c.g[V1][0] = sx(r(c, V1).wrapping_sub(r(c, A0)));
    c.g[V1][0] = sx(r(c, T0).wrapping_add(r(c, V1)));
    let return_pc = r(c, RA);
    c.sw(r(c, A1), c.g[V1][0]);
    c.pc = return_pc;
}
