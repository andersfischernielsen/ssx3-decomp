//! Staging module oc1537 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1537`.

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
    (0x0015_0f48, tail_stub_t1_150f48, "lbl_00150F48"),
    (0x0015_0f70, tail_stub_t1_150f70, "lbl_00150F70"),
    (0x0015_1220, tail_stub_v1_151220, "lbl_00151220"),
    (0x0015_1248, tail_stub_v1_151248, "lbl_00151248"),
    (0x0015_1270, tail_stub_v1_151270, "lbl_00151270"),
    (0x0015_1298, tail_stub_v1_151298, "lbl_00151298"),
    (0x0015_12c0, tail_stub_v1_1512c0, "lbl_001512C0"),
    (0x0015_12e8, tail_stub_v1_1512e8, "lbl_001512E8"),
];

/// The ten instructions the eight 40-byte tail stubs share: `v1 = rs1 * 0xf88`,
/// `a0 = rs2 * 0x9b50`, then `v1 += 0x4a6ca8` and `a0 += v1`. Each stub adds its
/// own offset to `a0` and jumps into the dispatcher label that reads the byte at
/// that pointer.
fn oc1537_stub_base(c: &mut Cpu, rs1: usize, rs2: usize) {
    c.cycles += 10;
    c.g[V1][0] = 0xf88;
    c.g[A0][0] = 0x9b50;
    c.mult(0, V1, rs1, V1);
    c.mult(0, A0, rs2, A0);
    c.g[V0][0] = 0x4a0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6ca8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
}

/// `0x00150f48` (40 B): computes the byte pointer `t1 = t4 * 0xf88 + t3 * 0x9b50 +
/// 0x4a6ca8 + 0xbe3` and tail-jumps into the dispatcher's `0x150fbc` label, which
/// reads the byte at `t1`.
pub fn tail_stub_t1_150f48(c: &mut Cpu) {
    oc1537_stub_base(c, T4, T3);
    c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xbe3));
    c.pc = 0x0015_0fbc;
    call_indirect(c, 0x0015_0fbc);
}

/// `0x00150f70` (40 B): as `tail_stub_t1_150f48` with offset `0xbe4`, into the
/// same `0x150fbc` label.
pub fn tail_stub_t1_150f70(c: &mut Cpu) {
    oc1537_stub_base(c, T4, T3);
    c.g[T1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xbe4));
    c.pc = 0x0015_0fbc;
    call_indirect(c, 0x0015_0fbc);
}

/// `0x00151220` (40 B): the `v1` variant of the stubs (pointer from `a2`, `a1`):
/// `v1 = a2 * 0xf88 + a1 * 0x9b50 + 0x4a6ca8 + 0xbdf`, then tail-jump into the
/// dispatcher's `0x151334` label, which reads the byte at `v1`.
pub fn tail_stub_v1_151220(c: &mut Cpu) {
    oc1537_stub_base(c, A2, A1);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xbdf));
    c.pc = 0x0015_1334;
    call_indirect(c, 0x0015_1334);
}

/// `0x00151248` (40 B): as `tail_stub_v1_151220` with offset `0xbe0`.
pub fn tail_stub_v1_151248(c: &mut Cpu) {
    oc1537_stub_base(c, A2, A1);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xbe0));
    c.pc = 0x0015_1334;
    call_indirect(c, 0x0015_1334);
}

/// `0x00151270` (40 B): as `tail_stub_v1_151220` with offset `0xbe1`.
pub fn tail_stub_v1_151270(c: &mut Cpu) {
    oc1537_stub_base(c, A2, A1);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xbe1));
    c.pc = 0x0015_1334;
    call_indirect(c, 0x0015_1334);
}

/// `0x00151298` (40 B): as `tail_stub_v1_151220` with offset `0xbe2`.
pub fn tail_stub_v1_151298(c: &mut Cpu) {
    oc1537_stub_base(c, A2, A1);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xbe2));
    c.pc = 0x0015_1334;
    call_indirect(c, 0x0015_1334);
}

/// `0x001512c0` (40 B): as `tail_stub_v1_151220` with offset `0xbe3`.
pub fn tail_stub_v1_1512c0(c: &mut Cpu) {
    oc1537_stub_base(c, A2, A1);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xbe3));
    c.pc = 0x0015_1334;
    call_indirect(c, 0x0015_1334);
}

/// `0x001512e8` (40 B): as `tail_stub_v1_151220` with offset `0xbe4`.
pub fn tail_stub_v1_1512e8(c: &mut Cpu) {
    oc1537_stub_base(c, A2, A1);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0xbe4));
    c.pc = 0x0015_1334;
    call_indirect(c, 0x0015_1334);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng};

    /// The stub fixture: the two scaled indices are small (so the byte pointer
    /// stays in RAM), and the byte at the pointer is 5 on one seed in three (the
    /// dispatcher's early return) and random otherwise. Everything else is junk.
    fn oc1537_stub_setup(c: &mut Cpu, seed: u32, rs1: usize, rs2: usize, off: u32) {
        let mut r = Rng(0x1537_0000 ^ seed as u64);
        scramble(c, &mut r);
        let a = (seed % 3) as u64;
        let b = (seed / 3 % 2) as u64;
        c.g[rs1][0] = a;
        c.g[rs2][0] = b;
        let p = 0x004a_6ca8 + off + (b as u32) * 0x9b50 + (a as u32) * 0xf88;
        let byte = if seed % 3 == 0 { 5 } else { r.next_u32() as u8 };
        c.bus.write8(p, byte);
    }

    #[test]
    fn tail_stub_t1_150f48_matches() {
        check(
            ee::s01::f_00150f48__generated,
            tail_stub_t1_150f48,
            120,
            &|c, seed| oc1537_stub_setup(c, seed, T4, T3, 0xbe3),
        );
    }

    #[test]
    fn tail_stub_t1_150f70_matches() {
        check(
            ee::s01::f_00150f70__generated,
            tail_stub_t1_150f70,
            120,
            &|c, seed| oc1537_stub_setup(c, seed, T4, T3, 0xbe4),
        );
    }

    #[test]
    fn tail_stub_v1_151220_matches() {
        check(
            ee::s01::f_00151220__generated,
            tail_stub_v1_151220,
            120,
            &|c, seed| oc1537_stub_setup(c, seed, A2, A1, 0xbdf),
        );
    }

    #[test]
    fn tail_stub_v1_151248_matches() {
        check(
            ee::s01::f_00151248__generated,
            tail_stub_v1_151248,
            120,
            &|c, seed| oc1537_stub_setup(c, seed, A2, A1, 0xbe0),
        );
    }

    #[test]
    fn tail_stub_v1_151270_matches() {
        check(
            ee::s01::f_00151270__generated,
            tail_stub_v1_151270,
            120,
            &|c, seed| oc1537_stub_setup(c, seed, A2, A1, 0xbe1),
        );
    }

    #[test]
    fn tail_stub_v1_151298_matches() {
        check(
            ee::s01::f_00151298__generated,
            tail_stub_v1_151298,
            120,
            &|c, seed| oc1537_stub_setup(c, seed, A2, A1, 0xbe2),
        );
    }

    #[test]
    fn tail_stub_v1_1512c0_matches() {
        check(
            ee::s01::f_001512c0__generated,
            tail_stub_v1_1512c0,
            120,
            &|c, seed| oc1537_stub_setup(c, seed, A2, A1, 0xbe3),
        );
    }

    #[test]
    fn tail_stub_v1_1512e8_matches() {
        check(
            ee::s01::f_001512e8__generated,
            tail_stub_v1_1512e8,
            120,
            &|c, seed| oc1537_stub_setup(c, seed, A2, A1, 0xbe4),
        );
    }
}
