//! Staging module oc1488 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1488`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_0238, scale_float_store_0238, "lbl_003A0238"),
    (0x003a_0248, scale_float_store_0248, "lbl_003A0248"),
];

/// Tail of an unnamed EE leaf at 0x3a0238 (16 bytes):
/// `f0 = *(gp - 0x2504) * f1; *(a3 + 0x20) = f0;` then a tail jump to
/// 0x3a0274 (the generated `if t` is always true, so the else arm at 0x3a0248
/// is never taken). Nothing is saved or restored: the callee returns straight
/// to this function's caller.
pub fn scale_float_store_0238(c: &mut Cpu) {
    c.cycles += 4;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2504));
    c.mul_s(0, 1, 0);
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x20));
    c.pc = 0x003a_0274;
    ee::s13::f_003a0274(c);
}

/// Tail of an unnamed EE leaf at 0x3a0248 (16 bytes): the same shape as
/// [`scale_float_store_0238`] with `gp - 0x2500` and the store at `a3 + 0x24`,
/// tail-jumping to 0x3a0274 (its else arm, 0x3a0258, is never taken).
pub fn scale_float_store_0248(c: &mut Cpu) {
    c.cycles += 4;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x2500));
    c.mul_s(0, 1, 0);
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x24));
    c.pc = 0x003a_0274;
    ee::s13::f_003a0274(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// Fixture for both tails: `gp` puts the constant word at `DATA`, `a3`
    /// points at the store target, and `a1 = 0` keeps the tail callee
    /// (0x3a0274) on its no-call path.
    fn scale_store_setup(c: &mut Cpu, seed: u32, gp_off: u32) {
        let mut r = Rng(0x3a02_3800 ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = sx(DATA + gp_off);
        c.g[A3][0] = sx(DATA + 0x1000);
        c.g[A1][0] = 0;
        let k = if seed % 3 == 0 { r.edge_f32() } else { r.f32(100.0) };
        c.bus.write32(DATA, k);
        c.f[1] = if seed % 4 == 0 { r.edge_f32() } else { r.f32(10.0) };
    }

    #[test]
    fn scale_float_store_0238_matches() {
        check(ssx3_ee::s13::f_003a0238__generated, scale_float_store_0238, 300, &|c, seed| {
            scale_store_setup(c, seed, 0x2504);
        });
    }

    #[test]
    fn scale_float_store_0248_matches() {
        check(ssx3_ee::s13::f_003a0248__generated, scale_float_store_0248, 300, &|c, seed| {
            scale_store_setup(c, seed, 0x2500);
        });
    }
}
