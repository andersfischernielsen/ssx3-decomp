//! Staging module oc1487 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1487`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_3898, lbl_003a3898, "lbl_003A3898"),
    (0x003a_38a8, lbl_003a38a8, "lbl_003A38A8"),
    (0x003a_01ac, lbl_003a01ac, "lbl_003A01AC"),
    (0x003a_01bc, lbl_003a01bc, "lbl_003A01BC"),
    (0x003a_01cc, lbl_003a01cc, "lbl_003A01CC"),
    (0x003a_01dc, lbl_003a01dc, "lbl_003A01DC"),
    (0x003a_01ec, lbl_003a01ec, "lbl_003A01EC"),
    (0x003a_0228, lbl_003a0228, "lbl_003A0228"),
];

/// Shared body of the six GP-scaled store fragments (0x3a01ac, 0x3a01bc,
/// 0x3a01cc, 0x3a01dc, 0x3a01ec, 0x3a0228): `f0 = f1 * gp[-gp_off]` (single
/// precision, `mul.s f0, f1, f0`), stored at `a3 + store_off`, then the
/// unconditional tail jump to 0x3a0274 (the loop tail that returns to `ra`).
/// The generated fall-through arm of each is guarded by a constant `true`.
fn oc1487_gp_scale_store_tail(c: &mut Cpu, gp_off: u32, store_off: u32) {
    c.cycles += 4;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(gp_off));
    c.mul_s(0, 1, 0);
    c.swc1(0, (c.g[A3][0] as u32).wrapping_add(store_off));
    c.pc = 0x003a_0274;
    ee::s13::f_003a0274(c);
}

/// `lbl_003A01AC`: `gp[-0x251c] * f1` stored at `a3 + 0x50`, then tail 0x3a0274.
pub fn lbl_003a01ac(c: &mut Cpu) {
    oc1487_gp_scale_store_tail(c, 0x251c, 0x50);
}

/// `lbl_003A01BC`: `gp[-0x2518] * f1` stored at `a3 + 0x54`, then tail 0x3a0274.
pub fn lbl_003a01bc(c: &mut Cpu) {
    oc1487_gp_scale_store_tail(c, 0x2518, 0x54);
}

/// `lbl_003A01CC`: `gp[-0x2514] * f1` stored at `a3 + 0x2c`, then tail 0x3a0274.
pub fn lbl_003a01cc(c: &mut Cpu) {
    oc1487_gp_scale_store_tail(c, 0x2514, 0x2c);
}

/// `lbl_003A01DC`: `gp[-0x2510] * f1` stored at `a3 + 0x30`, then tail 0x3a0274.
pub fn lbl_003a01dc(c: &mut Cpu) {
    oc1487_gp_scale_store_tail(c, 0x2510, 0x30);
}

/// `lbl_003A01EC`: `gp[-0x250c] * f1` stored at `a3 + 0x34`, then tail 0x3a0274.
pub fn lbl_003a01ec(c: &mut Cpu) {
    oc1487_gp_scale_store_tail(c, 0x250c, 0x34);
}

/// `lbl_003A0228`: `gp[-0x2508] * f1` stored at `a3 + 0x1c`, then tail 0x3a0274.
pub fn lbl_003a0228(c: &mut Cpu) {
    oc1487_gp_scale_store_tail(c, 0x2508, 0x1c);
}

/// Shared body of the two entries of the `UIVector` copy's table fragments
/// (0x3a3898, 0x3a38a8): `v0 = 12`, `v1 = *(t0 + 0x80)`, `v0 = a3 * 12`
/// (`mult`, low word), then the unconditional tail `call_indirect` to
/// `target` (inside 0x3a38c8 / 0x3a38e4). The generated fall-through arm
/// (a tail to the next fragment) is dead: its guard is a constant `true`.
fn oc1487_mult_tail_call(c: &mut Cpu, target: u32) {
    c.cycles += 4;
    c.g[V0][0] = 0xc;
    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x80));
    c.mult(0, 2, 7, 2);
    c.pc = target;
    call_indirect(c, target);
}

/// `lbl_003A3898`: `v0 = a3 * 12`, `v1 = *(t0 + 0x80)`, tail call 0x3a38d0.
pub fn lbl_003a3898(c: &mut Cpu) {
    oc1487_mult_tail_call(c, 0x003a_38d0);
}

/// `lbl_003A38A8`: the same prologue as 0x3a3898, tail call 0x3a38e4.
pub fn lbl_003a38a8(c: &mut Cpu) {
    oc1487_mult_tail_call(c, 0x003a_38e4);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, DATA, GAME_GP, RA as TEST_RA};

    /// Fixture of the GP-scaled fragments: a random float at `gp - gp_off`
    /// (the multiplier) and the operand `f1`, `a3` pointing at a scratch
    /// struct, and the loop tail 0x3a0274 taking its return path (`t0 + 1`
    /// is not below `a1`), so no callee runs beyond the fragment.
    fn oc1487_gp_scale_setup(c: &mut Cpu, seed: u32, salt: u64, gp_off: u32, entry: u32) {
        let mut r = machine(c, seed, salt, 1);
        c.bus.write32(GAME_GP.wrapping_sub(gp_off), r.edge_f32());
        c.f[1] = r.edge_f32();
        c.g[A3][0] = sx(DATA + 0x2000);
        c.g[T0][0] = sx(DATA + 0x3000);
        c.g[A1][0] = sx(DATA + 0x3000);
        c.g[A2][0] = sx(DATA + 0x4000);
        c.g[RA][0] = sx(TEST_RA);
        c.pc = entry;
    }

    #[test]
    fn lbl_003a01ac_matches() {
        check(
            ee::s13::f_003a01ac__generated,
            lbl_003a01ac,
            64,
            &|c, seed| oc1487_gp_scale_setup(c, seed, 0x3a01ac, 0x251c, 0x003a_01ac),
        );
    }

    #[test]
    fn lbl_003a01bc_matches() {
        check(
            ee::s13::f_003a01bc__generated,
            lbl_003a01bc,
            64,
            &|c, seed| oc1487_gp_scale_setup(c, seed, 0x3a01bc, 0x2518, 0x003a_01bc),
        );
    }

    #[test]
    fn lbl_003a01cc_matches() {
        check(
            ee::s13::f_003a01cc__generated,
            lbl_003a01cc,
            64,
            &|c, seed| oc1487_gp_scale_setup(c, seed, 0x3a01cc, 0x2514, 0x003a_01cc),
        );
    }

    #[test]
    fn lbl_003a01dc_matches() {
        check(
            ee::s13::f_003a01dc__generated,
            lbl_003a01dc,
            64,
            &|c, seed| oc1487_gp_scale_setup(c, seed, 0x3a01dc, 0x2510, 0x003a_01dc),
        );
    }

    #[test]
    fn lbl_003a01ec_matches() {
        check(
            ee::s13::f_003a01ec__generated,
            lbl_003a01ec,
            64,
            &|c, seed| oc1487_gp_scale_setup(c, seed, 0x3a01ec, 0x250c, 0x003a_01ec),
        );
    }

    #[test]
    fn lbl_003a0228_matches() {
        check(
            ee::s13::f_003a0228__generated,
            lbl_003a0228,
            64,
            &|c, seed| oc1487_gp_scale_setup(c, seed, 0x3a0228, 0x2508, 0x003a_0228),
        );
    }

    /// Fixture of the two `mult` fragments: `t0` points at a struct whose
    /// word at +0x80 is loaded into `v1`, `a3` is a small signed index (the
    /// `mult` is signed 32-bit), and the callee 0x3a38d0 / 0x3a38e4 (inside
    /// the `UIVector` copy's fragment at 0x3a38c8, through the function table)
    /// takes its loop-tail exit (`t2 + 1` is not below `a1`) so it returns.
    fn oc1487_mult_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let mut r = machine(c, seed, salt, 1);
        c.bus.write32(DATA + 0x2080, r.next_u32());
        c.bus.write32(DATA + 0x2000 + 0x7c, DATA + 0x3000);
        c.g[T0][0] = sx(DATA + 0x2000);
        c.g[A3][0] = sx(((r.next_u32() % 16) as i32 - 8) as u32);
        c.g[T2][0] = (r.next_u32() % 100) as u64;
        c.g[A1][0] = c.g[T2][0];
        c.g[A2][0] = sx(DATA + 0x4000);
        c.g[RA][0] = sx(TEST_RA);
        c.pc = entry;
    }

    #[test]
    fn lbl_003a3898_matches() {
        check(
            ee::s13::f_003a3898__generated,
            lbl_003a3898,
            64,
            &|c, seed| oc1487_mult_setup(c, seed, 0x3a3898, 0x003a_3898),
        );
    }

    #[test]
    fn lbl_003a38a8_matches() {
        check(
            ee::s13::f_003a38a8__generated,
            lbl_003a38a8,
            64,
            &|c, seed| oc1487_mult_setup(c, seed, 0x3a38a8, 0x003a_38a8),
        );
    }
}
