//! Staging module oc499 (wave wave228): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc499`.

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
    (0x0017_7650, direction_ne_chase_static_init, "DirectionNeChaseCameraAlgorithm_StaticInit"),
];

/// Base of the colour records the static init fills (`D_004C5890`).
const DIRECTION_NE_COLOUR_BASE: u32 = 0x004c_5890;

/// The 56 colour records, 16 bytes each, at `DIRECTION_NE_COLOUR_BASE + 16 * i`:
/// four words (RGBA floats as bits; a zero word where the original stores zero).
/// Taken from the original's stores (checked against the game's RAM image).
const DIRECTION_NE_COLOURS: [[u32; 4]; 56] = [
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f000000], // 0x004c5890
    [0x3effeeb3, 0x3f800000, 0x3f800000, 0x00000000], // 0x004c58a0
    [0x3e04a01b, 0x3f800000, 0x3f800000, 0x00000000], // 0x004c58b0
    [0x3e65a6c6, 0x3f70b2fb, 0x3f0ae1ac, 0x39d349bf], // 0x004c58c0
    [0x3eee2fbe, 0x3ea4b685, 0x3f7e8d54, 0x00000000], // 0x004c58d0
    [0x3e36323c, 0x3f800000, 0x3f800000, 0x00000000], // 0x004c58e0
    [0x3ee1e18f, 0x3ed3e8de, 0x3ea46aa1, 0x3ee1309c], // 0x004c58f0
    [0x3f004157, 0x3e7e75bc, 0x3f800000, 0x3f800000], // 0x004c5900
    [0x3e8dc24c, 0x3f23ba56, 0x3e0975f3, 0x00000000], // 0x004c5910
    [0x3f64c5da, 0x3f06bead, 0x3f01512f, 0x3f5e6517], // 0x004c5920
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5930
    [0x3f800000, 0x00000000, 0x00000000, 0x00000000], // 0x004c5940
    [0x3f800000, 0x3f0ccccd, 0x3f08f5c3, 0x3f0ccccd], // 0x004c5950
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5960
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5970
    [0x3f800000, 0x00000000, 0x00000000, 0x00000000], // 0x004c5980
    [0x3f800000, 0x00000000, 0x00000000, 0x00000000], // 0x004c5990
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c59a0
    [0x3f800000, 0x3f000000, 0x3f800000, 0x3f000000], // 0x004c59b0
    [0x3f800000, 0x3f000000, 0x3f800000, 0x3f000000], // 0x004c59c0
    [0x3f800000, 0x3f000000, 0x3f800000, 0x3f000000], // 0x004c59d0
    [0x3f800000, 0x3f48665e, 0x3d7ccd10, 0x3d233376], // 0x004c59e0
    [0x3f800000, 0x3f800000, 0x3f4c7fdc, 0x00000000], // 0x004c59f0
    [0x3f7d198b, 0x3f6c4d01, 0x3eafad08, 0x00000000], // 0x004c5a00
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5a10
    [0x3f800000, 0x3f800000, 0x3f4c3255, 0x00000000], // 0x004c5a20
    [0x3f800000, 0x3f6c19b1, 0x3eaf0004, 0x00000000], // 0x004c5a30
    [0x3f800000, 0x3f48e64b, 0x3d780022, 0x3d266666], // 0x004c5a40
    [0x3f800000, 0x3f4ccccd, 0x3ee66666, 0x3f800000], // 0x004c5a50
    [0x3f800000, 0x3ea80022, 0x3f800000, 0x3eb5332b], // 0x004c5a60
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5a70
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5a80
    [0x3f800000, 0x3f5c800f, 0x3ec332f0, 0x00000000], // 0x004c5a90
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5aa0
    [0x3f800000, 0x3ea61cbd, 0x3f800000, 0x00000000], // 0x004c5ab0
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5ac0
    [0x3f800000, 0x3f800000, 0x00000000, 0x00000000], // 0x004c5ad0
    [0x3f336178, 0x00000000, 0x00000000, 0x00000000], // 0x004c5ae0
    [0x3f800000, 0x3f48690e, 0x3d7b84dc, 0x3d29d51b], // 0x004c5af0
    [0x3f800000, 0x3ea65626, 0x3f800000, 0x3eb4a255], // 0x004c5b00
    [0x3f800000, 0x3f800000, 0x3f4c386d, 0x00000000], // 0x004c5b10
    [0x3f800000, 0x3efcccab, 0x3f800000, 0x3efbccd1], // 0x004c5b20
    [0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000], // 0x004c5b30
    [0x3f800000, 0x3f4ccccd, 0x3f800000, 0x3f4ccccd], // 0x004c5b40
    [0x3f800000, 0x3f800000, 0x00000000, 0x00000000], // 0x004c5b50
    [0x3f800000, 0x3f800000, 0x00000000, 0x00000000], // 0x004c5b60
    [0x3f800000, 0x3f800000, 0x00000000, 0x00000000], // 0x004c5b70
    [0x3f800000, 0x3ea7995f, 0x3f800000, 0x3eb53309], // 0x004c5b80
    [0x3f800000, 0x3f800000, 0x3d79db23, 0x3d1fbe77], // 0x004c5b90
    [0x3f800000, 0x3f800000, 0x3f4cccab, 0x00000000], // 0x004c5ba0
    [0x3f800000, 0x3f757ffa, 0x3f1d47f1, 0x00000000], // 0x004c5bb0
    [0x3f800000, 0x3f6db329, 0x3eae1262, 0x00000000], // 0x004c5bc0
    [0x3f800000, 0x3f5fb2ff, 0x3e7f4cf5, 0x3c8fffbd], // 0x004c5bd0
    [0x3f800000, 0x3f48ccbc, 0x3d7b3312, 0x3d219978], // 0x004c5be0
    [0x3f800000, 0x00000000, 0x3f800000, 0x00000000], // 0x004c5bf0
    [0x3f800000, 0x3f800000, 0x00000000, 0x00000000], // 0x004c5c00
];

/// Temporaries the original leaves behind after the fill (its schedule of
/// constant loads): the register, then its final value. The saved s0..s7 and
/// fp are restored by the epilogue and are not listed.
pub(crate) const DIRECTION_NE_LEFT_GPR: [(usize, u64); 17] = [
    (AT, 0x3f000000),
    (V0, 0x4c5b50),
    (V1, 0x4c5b60),
    (A0, 0x4c5b80),
    (A1, 0x4c0000),
    (A2, 0x4c0000),
    (A3, 0x4c0000),
    (T0, 0x4c5a60),
    (T1, 0x4c5a70),
    (T2, 0x4c0000),
    (T3, 0x4c5a10),
    (T4, 0x4c5b10),
    (T5, 0x4c5bf0),
    (T6, 0x4c5b90),
    (T7, 0x4c0000),
    (T8, 0x4c0000),
    (T9, 0x4c0000),
];

/// Float registers the original leaves holding literal-pool values (f0..f19).
pub(crate) const DIRECTION_NE_LEFT_FPR: [(usize, u32); 20] = [
    (0, 0x3f800000),
    (1, 0x3d219978),
    (2, 0x3c8fffbd),
    (3, 0x3f6db329),
    (4, 0x3eae1262),
    (5, 0x3f5fb2ff),
    (6, 0x3e7f4cf5),
    (7, 0x3f48ccbc),
    (8, 0x3d7b3312),
    (9, 0x3f4c386d),
    (10, 0x3efcccab),
    (11, 0x3ec332f0),
    (12, 0x3f4ccccd),
    (13, 0x3ea4b685),
    (14, 0x3f7e8d54),
    (15, 0x3ed3e8de),
    (16, 0x3ea46aa1),
    (17, 0x3f70b2fb),
    (18, 0x3f0ae1ac),
    (19, 0x3f23ba56),
];

/// `void DirectionNeChaseCameraAlgorithm_StaticInit(int initialize, int priority)`
/// (0x00177650): the one-time static init of the direction-NE chase camera's
/// colour table. Works only for `priority == 0xffff` and a non-zero
/// `initialize`: writes the 56 colour records at `D_004C5890`, leaves the
/// temporaries of its schedule in the registers, and returns.
/// Cycles: 12 on entry, 2 more once the priority matches, 477 for the fill,
/// 11 on exit (the priority-mismatch path: 23; the initialize-zero path: 25).
pub fn direction_ne_chase_static_init(c: &mut Cpu) {
    c.cycles += 12;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
    c.g[V0][0] = 0xffff;
    let frame = c.g[SP][0] as u32;
    c.sq(S0, frame.wrapping_add(0x80));
    c.sq(S1, frame.wrapping_add(0x70));
    c.sq(S2, frame.wrapping_add(0x60));
    c.sq(S3, frame.wrapping_add(0x50));
    c.sq(S4, frame.wrapping_add(0x40));
    c.sq(S5, frame.wrapping_add(0x30));
    c.sq(S6, frame.wrapping_add(0x20));
    c.sq(S7, frame.wrapping_add(0x10));
    c.sq(FP, frame);
    // `priority` (a1) must be 0xffff, and `initialize` (a0) non-zero.
    if c.g[A1][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[T5][0] = 0x4c0000;
        if c.g[A0][0] != 0 {
            c.cycles += 477;
            direction_ne_fill_colours(c);
            direction_ne_leave_temporaries(c);
        }
    }
    c.cycles += 11;
    c.lq(S0, frame.wrapping_add(0x80));
    c.lq(S1, frame.wrapping_add(0x70));
    c.lq(S2, frame.wrapping_add(0x60));
    c.lq(S3, frame.wrapping_add(0x50));
    c.lq(S4, frame.wrapping_add(0x40));
    c.lq(S5, frame.wrapping_add(0x30));
    c.lq(S6, frame.wrapping_add(0x20));
    c.lq(S7, frame.wrapping_add(0x10));
    c.lq(FP, frame);
    c.g[SP][0] = sx(frame.wrapping_add(0x90));
    ret(c);
}

/// Stores the colour records (four words each, in order).
pub(crate) fn direction_ne_fill_colours(c: &mut Cpu) {
    for (i, record) in DIRECTION_NE_COLOURS.iter().enumerate() {
        let base = DIRECTION_NE_COLOUR_BASE + 16 * i as u32;
        for (k, word) in record.iter().enumerate() {
            Addr(base + 4 * k as u32).sw(c, *word as u64);
        }
    }
}

/// Sets the temporaries the original leaves behind.
pub(crate) fn direction_ne_leave_temporaries(c: &mut Cpu) {
    for &(reg, value) in DIRECTION_NE_LEFT_GPR.iter() {
        c.g[reg][0] = value;
    }
    for &(reg, bits) in DIRECTION_NE_LEFT_FPR.iter() {
        c.f[reg] = bits;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, GAME_GP};

    /// The literal-pool words the generated body loads from `gp - K`, as
    /// (address, bits): the fixture writes them so both sides load the same
    /// constants.
    const DIRECTION_NE_POOL: &[(u32, u32)] = &[
    (0x0049d1bc, 0x3effeeb3),
    (0x0049d1c0, 0x3e04a01b),
    (0x0049d1c4, 0x3e65a6c6),
    (0x0049d1c8, 0x39d349bf),
    (0x0049d1cc, 0x3eee2fbe),
    (0x0049d1d0, 0x3e36323c),
    (0x0049d1d4, 0x3ee1e18f),
    (0x0049d1d8, 0x3ee1309c),
    (0x0049d1dc, 0x3f004157),
    (0x0049d1e0, 0x3e8dc24c),
    (0x0049d1e4, 0x3f70b2fb),
    (0x0049d1e8, 0x3f0ae1ac),
    (0x0049d1ec, 0x3ea4b685),
    (0x0049d1f0, 0x3f7e8d54),
    (0x0049d1f4, 0x3ed3e8de),
    (0x0049d1f8, 0x3ea46aa1),
    (0x0049d1fc, 0x3e7e75bc),
    (0x0049d200, 0x3f23ba56),
    (0x0049d204, 0x3f64c5da),
    (0x0049d208, 0x3f5e6517),
    (0x0049d20c, 0x3f0ccccd),
    (0x0049d210, 0x3e0975f3),
    (0x0049d214, 0x3f06bead),
    (0x0049d218, 0x3f01512f),
    (0x0049d21c, 0x3f08f5c3),
    (0x0049d220, 0x3d233376),
    (0x0049d224, 0x3f7d198b),
    (0x0049d228, 0x3d7ccd10),
    (0x0049d22c, 0x3f4c7fdc),
    (0x0049d230, 0x3f48665e),
    (0x0049d234, 0x3f6c4d01),
    (0x0049d238, 0x3eafad08),
    (0x0049d23c, 0x3d266666),
    (0x0049d240, 0x3eb5332b),
    (0x0049d244, 0x3ea80022),
    (0x0049d248, 0x3f5c800f),
    (0x0049d24c, 0x3ec332f0),
    (0x0049d250, 0x3d780022),
    (0x0049d254, 0x3f4c3255),
    (0x0049d258, 0x3f6c19b1),
    (0x0049d25c, 0x3eaf0004),
    (0x0049d260, 0x3f48e64b),
    (0x0049d264, 0x3ee66666),
    (0x0049d268, 0x3f4ccccd),
    (0x0049d26c, 0x3f336178),
    (0x0049d270, 0x3d29d51b),
    (0x0049d274, 0x3eb4a255),
    (0x0049d278, 0x3efbccd1),
    (0x0049d27c, 0x3ea61cbd),
    (0x0049d280, 0x3f48690e),
    (0x0049d284, 0x3d7b84dc),
    (0x0049d288, 0x3ea65626),
    (0x0049d28c, 0x3f4c386d),
    (0x0049d290, 0x3efcccab),
    (0x0049d294, 0x3eb53309),
    (0x0049d298, 0x3d1fbe77),
    (0x0049d29c, 0x3ea7995f),
    (0x0049d2a0, 0x3d79db23),
    (0x0049d2a4, 0x3f4cccab),
    (0x0049d2a8, 0x3f757ffa),
    (0x0049d2ac, 0x3f1d47f1),
    (0x0049d2b0, 0x3c8fffbd),
    (0x0049d2b4, 0x3d219978),
    (0x0049d2b8, 0x3f6db329),
    (0x0049d2bc, 0x3eae1262),
    (0x0049d2c0, 0x3f5fb2ff),
    (0x0049d2c4, 0x3e7f4cf5),
    (0x0049d2c8, 0x3f48ccbc),
    (0x0049d2cc, 0x3d7b3312),
    ];

    #[test]
    fn direction_ne_chase_static_init_matches() {
        check(
            ssx3_ee::s02::f_00177650__generated,
            direction_ne_chase_static_init,
            200,
            &|c, seed| {
                let mut r = Rng(0x1776_5000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = if seed % 4 == 0 { 0 } else { r.next_u32() as u64 };
                c.g[A1][0] = if seed % 3 == 0 { r.next_u32() as u64 } else { 0xffff };
                c.pc = 0x0017_7650;
                for &(addr, bits) in DIRECTION_NE_POOL {
                    c.bus.write32(addr, bits);
                }
            },
        );
    }
}
