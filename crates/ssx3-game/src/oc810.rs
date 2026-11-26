//! Staging module oc810 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc810`.
//!
//! 0x002f5298 WorldLightMan_AllocOverlapList: already ported
//! (game/visualfx/worldlightman.rs), skipped.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x002f83b0,
        attached_emitter_type_data_init_part1,
        "AttachedEmitterTypeData_InitPart1",
    ),
    (
        0x002f9838,
        attached_emitter_type_data_init_part2,
        "AttachedEmitterTypeData_InitPart2",
    ),
    (0x0030bd20, wscript_man_load_state, "WScriptMan_LoadState"),
];

/// `AttachedEmitterTypeData_InitPart1` (0x002f83b0): static initialiser of
/// the attached-emitter type table (first half). Loads the table base from
/// `gp+0xac0` and fills the per-type default fields (floats, zeros and small
/// ints) in one straight line; no calls.
pub fn attached_emitter_type_data_init_part1(c: &mut Cpu) {
    c.cycles += 786;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xac0));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 1);
    c.g[V1][0] = 0x1;
    c.g[AT][0] = 0xffffffffbf800000;
    c.mtc1(1, 11);
    c.g[A1][0] = 0x5;
    c.g[AT][0] = 0x42200000;
    c.mtc1(1, 7);
    c.g[A0][0] = 0x18;
    c.g[AT][0] = 0x43160000;
    c.mtc1(1, 0);
    c.g[A2][0] = 0x2;
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.g[A3][0] = 0x19;
    c.swc1(20, c.g[SP][0] as u32);
    c.g[T0][0] = 0x3;
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x74));
    c.g[T1][0] = 0x8;
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x10));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x54));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x64));
    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x8));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x20), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x24), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x28), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x30), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x34), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x38), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x40), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x44), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x48), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x50), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x58), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x60), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x68), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x70), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x78), 0u64);
    c.g[AT][0] = 0x41a00000;
    c.mtc1(1, 9);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x80), 0u64);
    c.g[AT][0] = 0x430c0000;
    c.mtc1(1, 13);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x84));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x88));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x8c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x90));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x94));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x98));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x9c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xa0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc8), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xcc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc4), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd0), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xd4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xd8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfc), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x100));
    c.g[AT][0] = 0x42c80000;
    c.mtc1(1, 3);
    c.g[AT][0] = 0x41200000;
    c.mtc1(1, 6);
    c.g[AT][0] = 0x42480000;
    c.mtc1(1, 5);
    c.g[AT][0] = 0x3fc00000;
    c.mtc1(1, 16);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x104));
    c.g[AT][0] = 0x428c0000;
    c.mtc1(1, 0);
    c.g[AT][0] = 0x3e800000;
    c.mtc1(1, 2);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x108));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x120));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x124));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x11c));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x128));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x140));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x150));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x164));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x174));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x184));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x10c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x110), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x114), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x118));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x130), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x134), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x138), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x144), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x148), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x154), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x158), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x15c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x160), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x168), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x16c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x170), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x178), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x17c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x180), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x188), 0u64);
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3770));
    c.g[AT][0] = 0x40400000;
    c.mtc1(1, 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x18c), 0u64);
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x190));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1fc));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1f4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x194));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x198));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x19c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1a0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1a4), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1a8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1e0), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x1e4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1e8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1f0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x200), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x204), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x208), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x20c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x210));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x214));
    c.g[AT][0] = 0x41f00000;
    c.mtc1(1, 0);
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x376c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x218));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x230));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x234));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x274));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x284));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x294));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x21c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x220), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x224), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x228));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x22c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x238), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x23c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x240), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x244), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x248), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x24c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x250), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x254), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x258), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x25c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x260), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x264), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x268), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x26c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x270), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x278), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x27c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x280), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x288), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x28c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x290), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x298), 0u64);
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3768));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x29c), 0u64);
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x2a0));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x304));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2a4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2a8));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2ac));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2b0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2b4), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2b8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2e4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2f0), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x2f4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x2f8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x2fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x300), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x308), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x30c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x310), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x314), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x318), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x31c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x320));
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3764));
    c.g[AT][0] = 0x43fa0000;
    c.mtc1(1, 12);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x324));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3760));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x328));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x340));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x384));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x394));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x3a4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x32c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x330), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x334), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x338));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x33c));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x344));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x348), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x34c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x350), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x354), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x358), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x35c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x360), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x364), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x368), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x36c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x370), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x374), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x378), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x37c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x380), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x388), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x38c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x390), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x398), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x39c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3a8), 0u64);
    c.g[AT][0] = 0xffffffffc3480000;
    c.mtc1(1, 2);
    c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x375c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3ac), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x3b0));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x414));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x3b4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x3b8));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x3bc));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x3c0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3c4), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x3c8));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x3cc));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x3d0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3f0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x3f4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x400), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x404));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x408));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x40c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x410), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x418), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x41c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x420), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x424), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x428), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x42c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x430));
    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3758));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x434));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x438));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4b4));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x450));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x454));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x494));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4a4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x43c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x440), c.g[A2][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x444), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x448));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x44c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x458), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x45c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x460), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x464), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x468), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x46c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x470), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x474), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x478), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x47c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x480), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x484), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x488), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x48c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x490), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x498), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x49c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4bc), 0u64);
    c.g[AT][0] = 0x3fa00000;
    c.mtc1(1, 0);
    c.g[AT][0] = 0x41000000;
    c.mtc1(1, 2);
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x4c0));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x518));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x528));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x50c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x504), c.g[A0][0]);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4c4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4c8));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4cc));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4d0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4d4), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x4d8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4f0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4f4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x500), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x508), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x510), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x514));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x51c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x520), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x524), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x52c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x530), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x534), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x538), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x53c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x540));
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 14);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x544));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x548));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x55c));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x560));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x5c4));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x564));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x5a4));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x5b4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x54c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x550), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x554), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x558));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x568), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x56c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x570), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x574), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x578), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x57c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x580), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x584), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x588), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x58c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x590), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x594), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x598), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x59c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5c8), 0u64);
    c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x3754));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5cc), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3750));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x374c));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3748));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3744));
    c.g[AT][0] = 0x42f00000;
    c.mtc1(1, 5);
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3740));
    c.g[AT][0] = 0x40e00000;
    c.mtc1(1, 7);
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x5d0));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x5d4));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x5d8));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x5e0));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x5e8));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x5f0));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x61c));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x628));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x638));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x5dc));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0x5ec));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x614), c.g[A3][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5f4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x5fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x600), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x604), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x608), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x60c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x610), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x618), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x620), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x624));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x62c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x630), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x634), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x63c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x640), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x644), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x648), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x64c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x650));
    c.g[AT][0] = 0x40c00000;
    c.mtc1(1, 2);
    c.g[AT][0] = 0x41c00000;
    c.mtc1(1, 5);
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x373c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x654));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x658));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x670));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x680));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x65c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x660), c.g[A2][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x664), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x668));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x66c));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x674));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x678));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x67c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x684), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x688), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x68c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x690), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x694), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x698), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x69c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6a4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6d8), 0u64);
    c.g[AT][0] = 0xffffffffc1ac0000;
    c.mtc1(1, 3);
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3738));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3734));
    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3730));
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x372c));
    c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x3728));
    c.g[AT][0] = 0xffffffffc0400000;
    c.mtc1(1, 17);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6dc), 0u64);
    c.g[AT][0] = 0xffffffffc15c0000;
    c.mtc1(1, 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6e0), 0u64);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x748));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x6e8));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x6ec));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x6fc));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x704));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x750));
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x758));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x6e4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x6f0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x6f4), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x6f8));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x700));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x708), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x70c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x710), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x714), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x718), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x71c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x720), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x728), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x72c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x724), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x730), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x734));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x738));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x73c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x740), 0u64);
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x744));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x74c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x754), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x75c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x760));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x764));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x768));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x780));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x790));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x770), c.g[A2][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x76c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x774), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x778));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x77c));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x784));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x788));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x78c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x794), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x798), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x79c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7a4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7ec), 0u64);
    c.g[AT][0] = 0x41820000;
    c.mtc1(1, 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x7f0), 0u64);
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x7f8));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x7fc));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x80c));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x814));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x858));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x860));
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x868));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x7f4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x800));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x804), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x808));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x810));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x818), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x81c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x820), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x824), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x828), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x82c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x830), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x838), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x83c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x834), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x840), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x844));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x848));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x84c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x850), 0u64);
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x854));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x85c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x864), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x86c), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x870));
    c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x3724));
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x3720));
    c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x371c));
    c.g[AT][0] = 0x42af0000;
    c.mtc1(1, 17);
    c.g[AT][0] = 0x43870000;
    c.mtc1(1, 18);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x874));
    c.g[AT][0] = 0x425a0000;
    c.mtc1(1, 0);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x878));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x87c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x880), c.g[T0][0]);
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x89c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x884), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x888));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x88c));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x890));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x894));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x898));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x8a0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8a4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8ac), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x8b0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8bc), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x8c0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8dc), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x8e0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8ec), 0u64);
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8f0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8f4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8f8), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3718));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3714));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3710));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x370c));
    c.g[AT][0] = 0xffffffffc21e0000;
    c.mtc1(1, 16);
    c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x3708));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x900), 0u64);
    c.g[AT][0] = 0x42dc0000;
    c.mtc1(1, 20);
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0x928));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x904));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x908));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x90c));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x910));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x914), 0u64);
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x918));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x91c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x920), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x924), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x92c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x930), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x934), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x938), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x93c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x940), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x948), 0u64);
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x94c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x944), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x950), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x954));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x958));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x95c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x960), 0u64);
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x964));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x968), 0u64);
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x96c));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0x970));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x974), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x978), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x97c), 0u64);
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x980));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x984));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x988));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x990), c.g[T1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x998));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x9a0));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x9a8));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x9ac));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x9b0));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x9d0));
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x9f0));
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0xa00));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x98c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x994), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x99c));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x9a4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9bc), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x9c0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9f4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x9fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa04), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa08), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa0c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa10), 0u64);
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0xa38));
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xa8c));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xa14));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xa20));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0xa28));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xa5c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa54), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa60), c.g[V1][0]);
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xa64));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xa68));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0xa74));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xa7c));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0xa80));
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xa90));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xa18));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xa1c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa24), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa2c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa30), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa34), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa3c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa40), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa44), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa48), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa4c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa50), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa58), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa6c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa70), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa78), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa84), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xa88), 0u64);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xa94));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xa98));
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.lwc1(20, c.g[SP][0] as u32);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xa9c));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t;
}

/// `AttachedEmitterTypeData_InitPart2` (0x002f9838): second half of the
/// attached-emitter type table initialiser (same base, `gp+0xac0`); a straight
/// line of stores, no calls.
pub fn attached_emitter_type_data_init_part2(c: &mut Cpu) {
    c.cycles += 879;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xac0));
    c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x28));
    c.g[A0][0] = 0x5;
    c.g[AT][0] = 0xffffffffbf800000;
    c.mtc1(1, 13);
    c.g[V1][0] = 0x1;
    c.lwc1(25, (c.g[GP][0] as u32).wrapping_sub(0x35f0));
    c.g[A1][0] = 0x29;
    c.g[AT][0] = 0x43c80000;
    c.mtc1(1, 3);
    c.g[T0][0] = 0xa;
    c.g[AT][0] = 0x40a00000;
    c.mtc1(1, 5);
    c.g[T2][0] = 0x8;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x35ec));
    c.g[A2][0] = 0x4;
    c.g[AT][0] = 0x3e800000;
    c.mtc1(1, 1);
    c.g[T1][0] = 0xf;
    c.swc1(31, (c.g[SP][0] as u32).wrapping_add(0x58));
    c.g[A3][0] = 0x3;
    c.swc1(30, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[T3][0] = 0x26;
    c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x18));
    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.swc1(20, c.g[SP][0] as u32);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xaac));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xab4));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xb14));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xab0));
    c.swc1(25, (c.g[V0][0] as u32).wrapping_add(0xabc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xaa0), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xaa4), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xaa8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xab8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xac0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xac4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xac8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xacc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xad0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xad4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xad8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xadc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xae0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xae4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xae8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xaec), 0u64);
    c.g[AT][0] = 0xffffffffc3390000;
    c.mtc1(1, 4);
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xaf4));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xb04));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xaf0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xaf8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xafc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb00), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb08), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb0c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb10), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb18), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb1c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x35e8));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xb20));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xb30));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 2);
    c.g[AT][0] = 0x41a00000;
    c.mtc1(1, 11);
    c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x35e4));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x35e0));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x35dc));
    c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x35d8));
    c.g[AT][0] = 0x430c0000;
    c.mtc1(1, 17);
    c.g[AT][0] = 0x40400000;
    c.mtc1(1, 26);
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xb58));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xb2c));
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0xb3c));
    c.g[AT][0] = 0x41820000;
    c.mtc1(1, 20);
    c.g[AT][0] = 0x41900000;
    c.mtc1(1, 21);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xb24));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xb28));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb34), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xb38));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb40), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb44), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb48), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb4c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb50), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb54), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb5c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb60), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb68), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb6c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb64), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb70), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0xb74));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0xb78));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x35d4));
    c.g[AT][0] = 0x418e0000;
    c.mtc1(1, 10);
    c.g[AT][0] = 0x40000000;
    c.mtc1(1, 15);
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xb84));
    c.swc1(26, (c.g[V0][0] as u32).wrapping_add(0xb88));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0xb8c));
    c.g[AT][0] = 0x40800000;
    c.mtc1(1, 31);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb7c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb80), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb90), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb94), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb98), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xb9c), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0xba0));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xba4));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x35d0));
    c.g[AT][0] = 0x42f00000;
    c.mtc1(1, 1);
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xba8));
    c.g[AT][0] = 0x42960000;
    c.mtc1(1, 16);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xbac));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xbc8));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc24));
    c.swc1(31, (c.g[V0][0] as u32).wrapping_add(0xbc0));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0xbcc));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0xbe0));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0xbf0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbb0), c.g[T0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbb4), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xbb8));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xbbc));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0xbc4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbd0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbd4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbd8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbdc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbe4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbe8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbf4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbf8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xbfc), 0u64);
    c.g[AT][0] = 0x41200000;
    c.mtc1(1, 22);
    c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x35cc));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0xc04));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0xc14));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc00), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc08), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc0c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc10), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc18), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc1c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc20), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc28), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc2c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x35c8));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x35c4));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x35c0));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x35bc));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x35b8));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x35b4));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x35b0));
    c.g[AT][0] = 0x428c0000;
    c.mtc1(1, 8);
    c.swc1(22, (c.g[V0][0] as u32).wrapping_add(0xc30));
    c.lwc1(30, (c.g[GP][0] as u32).wrapping_sub(0x35ac));
    c.g[AT][0] = 0xffffffffc2700000;
    c.mtc1(1, 19);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc34));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xc38));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xc3c));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xc40));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xc68));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0xc6c));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc70));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0xc7c));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0xc58));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0xc5c));
    c.lwc1(27, (c.g[GP][0] as u32).wrapping_sub(0x35a8));
    c.g[AT][0] = 0xffffffffc35f0000;
    c.mtc1(1, 28);
    c.g[AT][0] = 0x425c0000;
    c.mtc1(1, 29);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc44), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc48));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc4c));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc50));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc54), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc60), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc64), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc78), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc74), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc80), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0xc84));
    c.swc1(30, (c.g[V0][0] as u32).wrapping_add(0xc88));
    c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x35a4));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0xc94));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0xc9c));
    c.swc1(27, (c.g[V0][0] as u32).wrapping_add(0xca0));
    c.swc1(28, (c.g[V0][0] as u32).wrapping_add(0xca8));
    c.swc1(29, (c.g[V0][0] as u32).wrapping_add(0xcb0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc8c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc90), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc98), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xca4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcac), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xcb4));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x35a0));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x359c));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3598));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3594));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3590));
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xcb8));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xcd0));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xcd4));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xcd8));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xce0));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xcbc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcc4), c.g[A2][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcc0), c.g[T2][0]);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xcc8));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xccc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcdc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xce4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xce8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcf0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcf4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcf8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xcfc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd00), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd04), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd08), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd0c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd10), 0u64);
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x358c));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3588));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd14), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd18), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd1c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd20), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd24), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd28), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd2c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd30), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd34), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd38), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd3c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3584));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3580));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x357c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd40), 0u64);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xd4c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xd50));
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0xd5c));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xd6c));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xd70));
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xda4));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0xdac));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xd68));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xd78));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xd44));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xd48));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd54), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xd58));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd60), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd64), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd74), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd7c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd80), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd88), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd8c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd84), c.g[A1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd90), c.g[V1][0]);
    c.swc1(26, (c.g[V0][0] as u32).wrapping_add(0xda8));
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0xd94));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0xd98));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xd9c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xda0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdb0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdb4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdb8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdbc), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0xdc0));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xdc4));
    c.g[AT][0] = 0x40200000;
    c.mtc1(1, 0);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xdc8));
    c.g[AT][0] = 0x42c80000;
    c.mtc1(1, 18);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xde8));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0xdec));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0xde0));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0xe00));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0xe10));
    c.g[AT][0] = 0x42af0000;
    c.mtc1(1, 20);
    c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x3578));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdd0), c.g[T1][0]);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xdcc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdd4), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xdd8));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xddc));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0xde4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdf0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdf4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdf8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xdfc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe04), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe08), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe0c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe14), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe18), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe1c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe20), 0u64);
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xe24));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0xe34));
    c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x3574));
    c.g[AT][0] = 0x43480000;
    c.mtc1(1, 23);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe28), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe2c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe30), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe38), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe3c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe40), 0u64);
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0xe44));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe48), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe4c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3570));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x356c));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3568));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3564));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3560));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x355c));
    c.swc1(22, (c.g[V0][0] as u32).wrapping_add(0xe50));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xe54));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xe58));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xe5c));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xe60));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xe6c));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xe70));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0xe7c));
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xe8c));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0xe90));
    c.swc1(27, (c.g[V0][0] as u32).wrapping_add(0xec0));
    c.swc1(28, (c.g[V0][0] as u32).wrapping_add(0xec8));
    c.swc1(29, (c.g[V0][0] as u32).wrapping_add(0xecc));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xe68));
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0xe78));
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0xe88));
    c.swc1(23, (c.g[V0][0] as u32).wrapping_add(0xe9c));
    c.g[AT][0] = 0x42d20000;
    c.mtc1(1, 24);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe64), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe74), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe80), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe84), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe98), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xe94), c.g[A0][0]);
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 8);
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0xeb4));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0xebc));
    c.swc1(24, (c.g[V0][0] as u32).wrapping_add(0xed0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xea0), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0xea4));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0xea8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xeac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xeb0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xeb8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xec4), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xed4));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3558));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3554));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3550));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x354c));
    c.g[AT][0] = 0x43870000;
    c.mtc1(1, 5);
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0xed8));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xedc));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xef0));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xef8));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xefc));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0xf20));
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0xf34));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0xf44));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xf54));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0xf10));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xee0), c.g[T0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xee4), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xee8));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xeec));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0xef4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf00), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf04), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf08), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf0c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf14), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf18), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf1c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf24), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf28), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf2c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf30), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf38), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf3c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf40), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf48), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf4c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf50), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf58), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf5c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3548));
    c.swc1(22, (c.g[V0][0] as u32).wrapping_add(0xf60));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xf64));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x3544));
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x3540));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x353c));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x3538));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x3534));
    c.g[AT][0] = 0x43960000;
    c.mtc1(1, 5);
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x3530));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x352c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0xf68));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0xf6c));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xf70));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xf8c));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0xf9c));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0xfa0));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0xfac));
    c.swc1(30, (c.g[V0][0] as u32).wrapping_add(0xfb8));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0xfc4));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0xfcc));
    c.swc1(23, (c.g[V0][0] as u32).wrapping_add(0xfd0));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0xfd8));
    c.swc1(24, (c.g[V0][0] as u32).wrapping_add(0xfdc));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xf88));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf74), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf78), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf7c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf80), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf84), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf90), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xf94), 0u64);
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0xf98));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfa8), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfa4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfb0), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0xfb4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfbc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfc0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfc8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xfd4), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0xfe0));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xfe4));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3528));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xfe8));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xfec));
    c.swc1(31, (c.g[V0][0] as u32).wrapping_add(0x1000));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xff0), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0xff4), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0xff8));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xffc));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1004));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1008), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x100c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1010), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1014), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1018), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x101c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1020), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1024), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1028), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x102c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1030), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1034), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1038), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x103c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1040), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1044), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1048), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x104c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1050), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1054), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1058), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x105c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1060), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1064), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1068), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x106c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1070), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1074));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1078));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x107c));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1080));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1084));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1088));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x108c));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1090));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1094), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1098), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x109c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10a4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10b8), c.g[V1][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10b4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10c0), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x10c4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10c8), 0u64);
    c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x3524));
    c.lwc1(23, (c.g[GP][0] as u32).wrapping_sub(0x3520));
    c.lwc1(24, (c.g[GP][0] as u32).wrapping_sub(0x351c));
    c.lwc1(27, (c.g[GP][0] as u32).wrapping_sub(0x3518));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x10ec), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x10f0));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x10f4));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x10f8));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0x1110));
    c.swc1(23, (c.g[V0][0] as u32).wrapping_add(0x1114));
    c.swc1(24, (c.g[V0][0] as u32).wrapping_add(0x1118));
    c.swc1(27, (c.g[V0][0] as u32).wrapping_add(0x111c));
    c.swc1(25, (c.g[V0][0] as u32).wrapping_add(0x1130));
    c.swc1(25, (c.g[V0][0] as u32).wrapping_add(0x1140));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x10fc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1100), c.g[A3][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1104), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x1108));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x110c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1120), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1124), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1128), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x112c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1134), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1138), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x113c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1144), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1148), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x114c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1150), 0u64);
    c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x3514));
    c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x3510));
    c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x350c));
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x1154));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1158), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x115c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1160), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x1164));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1168), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x116c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1170), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x1174));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1178), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x117c), 0u64);
    c.g[AT][0] = 0x40c80000;
    c.mtc1(1, 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1180), 0u64);
    c.g[AT][0] = 0x41100000;
    c.mtc1(1, 15);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x11e8));
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x11a0));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x1184));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1188), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x118c), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1190));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1194), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1198));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0x119c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11a4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11ac), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11c4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11d0), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x11d4));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3508));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x11d8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11e0), 0u64);
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x11e4));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x11ec));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11f0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11f4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x11fc), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x1200));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1204));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3504));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3500));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x34fc));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x34f8));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x1208));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x120c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1220));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x1228));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x122c));
    c.swc1(26, (c.g[V0][0] as u32).wrapping_add(0x1250));
    c.swc1(25, (c.g[V0][0] as u32).wrapping_add(0x1224));
    c.swc1(26, (c.g[V0][0] as u32).wrapping_add(0x1240));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1210), c.g[T2][0]);
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x1284));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1214), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x1218));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x121c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1230), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1234), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1238), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x123c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1244), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1248), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x124c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1254), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1258), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x125c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1260), 0u64);
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x1264));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1268), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x126c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1270), 0u64);
    c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x1274));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1278), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x127c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1280), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1288), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x128c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x34f4));
    c.g[AT][0] = 0x3fe00000;
    c.mtc1(1, 1);
    c.g[AT][0] = 0x42100000;
    c.mtc1(1, 3);
    c.g[AT][0] = 0xffffffffc1740000;
    c.mtc1(1, 4);
    c.g[AT][0] = 0xffffffffc1400000;
    c.mtc1(1, 5);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1290), 0u64);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1294));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x12dc));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x12f4));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x12f8));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x12fc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12d4), c.g[T3][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1298), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x129c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12a0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12a4), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x12a8));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x12ac));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x12b0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12b4), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x12b8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12bc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12c8), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x12cc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12d0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12d8), 0u64);
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x34f0));
    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x34ec));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12e0), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x12e4));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x12e8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x12f0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1300), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1304), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1308), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x130c), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x1310));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1314));
    c.g[AT][0] = 0x40880000;
    c.mtc1(1, 0);
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x34e8));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x34e4));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x34e0));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x34dc));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x34d8));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x1318));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x131c));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1330));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x1334));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x1338));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x133c));
    c.swc1(22, (c.g[V0][0] as u32).wrapping_add(0x1360));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x1374));
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x1394));
    c.swc1(22, (c.g[V0][0] as u32).wrapping_add(0x1358));
    c.swc1(22, (c.g[V0][0] as u32).wrapping_add(0x135c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1320), c.g[A3][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1324), 0u64);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x1328));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x132c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1340), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1344), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1348), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x134c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1350), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1354), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1364), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1368), 0u64);
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x1384));
    c.g[AT][0] = 0xffffffffc3700000;
    c.mtc1(1, 12);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x136c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1370), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1378), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x137c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1380), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1388), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x138c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1390), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1398), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x139c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x34d4));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x34d0));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x34cc));
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x34c8));
    c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x34c4));
    c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x34c0));
    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x34bc));
    c.g[AT][0] = 0x3fc00000;
    c.mtc1(1, 8);
    c.g[AT][0] = 0x41740000;
    c.mtc1(1, 9);
    c.g[AT][0] = 0xffffffffc1140000;
    c.mtc1(1, 10);
    c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x13a0));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x13a8));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x13ac));
    c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x13b8));
    c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x13bc));
    c.swc1(5, (c.g[V0][0] as u32).wrapping_add(0x13d0));
    c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x13d8));
    c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0x13ec));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13e4), c.g[A1][0]);
    c.swc1(8, (c.g[V0][0] as u32).wrapping_add(0x13f8));
    c.swc1(9, (c.g[V0][0] as u32).wrapping_add(0x1404));
    c.swc1(10, (c.g[V0][0] as u32).wrapping_add(0x1408));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x13a4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13b0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13b4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13c0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13c4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13c8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13cc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13e8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13f0), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x13f4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x13fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1400), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x140c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1410), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1414), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1418), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x141c), 0u64);
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x1420));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1424));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1428));
    c.swc1(21, (c.g[V0][0] as u32).wrapping_add(0x1440));
    c.swc1(23, (c.g[V0][0] as u32).wrapping_add(0x1444));
    c.swc1(24, (c.g[V0][0] as u32).wrapping_add(0x1448));
    c.swc1(27, (c.g[V0][0] as u32).wrapping_add(0x144c));
    c.swc1(25, (c.g[V0][0] as u32).wrapping_add(0x1470));
    c.swc1(25, (c.g[V0][0] as u32).wrapping_add(0x1460));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1430), c.g[A3][0]);
    c.swc1(13, (c.g[V0][0] as u32).wrapping_add(0x1438));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x142c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1434), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x143c));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1450), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1454), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1458), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x145c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1464), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1468), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x146c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1474), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1478), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x147c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1480), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x1484));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1488), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x148c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1490), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x1494));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1498), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x149c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14a0), 0u64);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x14a4));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14a8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14ac), 0u64);
    c.g[AT][0] = 0xffffffffc0c80000;
    c.mtc1(1, 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14b0), 0u64);
    c.swc1(20, (c.g[V0][0] as u32).wrapping_add(0x14d0));
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x1518));
    c.swc1(16, (c.g[V0][0] as u32).wrapping_add(0x14b4));
    c.swc1(19, (c.g[V0][0] as u32).wrapping_add(0x14cc));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14f4), c.g[A0][0]);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1500), c.g[V1][0]);
    c.swc1(11, (c.g[V0][0] as u32).wrapping_add(0x1504));
    c.swc1(14, (c.g[V0][0] as u32).wrapping_add(0x1508));
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x151c));
    c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x1530));
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x1534));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14b8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14bc), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x14c0));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14c4), 0u64);
    c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0x14c8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14d4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14d8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14dc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14e0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14e4), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14e8), 0u64);
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x34b8));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14ec), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14f0), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14f8), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x14fc), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x150c), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1510), 0u64);
    c.swc1(15, (c.g[V0][0] as u32).wrapping_add(0x1514));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1520), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1524), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1528), 0u64);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x152c), 0u64);
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x34b4));
    c.swc1(1, (c.g[V0][0] as u32).wrapping_add(0x1538));
    c.lwc1(31, (c.g[SP][0] as u32).wrapping_add(0x58));
    c.lwc1(30, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x28));
    c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x18));
    c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.lwc1(20, c.g[SP][0] as u32);
    c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x153c));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t;
}

/// `WScriptMan_LoadState(self, stream)` (0x0030bd20): restore the script
/// manager from a replay frame. Reads the 652-byte header and the collect
/// pool (`CollectSetLoad`), moves every process of the active and add lists
/// back to the idle list (`PopFront` / `PushFront`), loads both process lists
/// from the stream, empties the global Luno tables, then reads `count`
/// `(id, table)` pairs (`cWScriptCache_GetData`, `cLunoTable_Read`) and clears
/// the current-call fields. Resumable body: the normal entry is hand-ported,
/// any other `pc` (resume after an unwound callee or event) runs the
/// generated body.
pub fn wscript_man_load_state(c: &mut Cpu) {
    if c.pc != 0x0030bd20 {
        return ee::s10::f_0030bd20__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0030bd20;
        loop {
            match pc {
                0x0030bd20 => {
                    c.cycles += 20;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                    c.g[A2][0] = 0x28c;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2c0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S3][0] = c.g[A1][0];
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = c.g[S1][0];
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.g[S2][0] = 0u64;
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0030bd6c);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0030bd6c)?;
                    pc = 0x0030bd70;
                    continue;
                }
                0x0030bd70 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0030bd7c);
                    c.g[A1][0] = c.g[S3][0];
                    Callee {
                        addr: 0x0030c700,
                        f: ee::s10::WScriptMan_CollectSetLoad,
                    }
                    .call(c, 0x0030bd7c)?;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    let t = (c.g[S2][0] as i64) >= 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10c));
                    if t {
                        if back_edge_unwinds(c, 0x0030bd70) {
                            return Ok(());
                        }
                        pc = 0x0030bd70;
                        continue;
                    }
                    pc = 0x0030bd88;
                    continue;
                }
                0x0030bd88 => {
                    c.cycles += 4;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b8));
                    c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2bc));
                    let t = true;
                    c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3cc));
                    if t {
                        pc = 0x0030bda4;
                        continue;
                    }
                    pc = 0x0030bd98;
                    continue;
                }
                0x0030bd98 => {
                    c.cycles += 3;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[RA][0] = sx(0x0030bda4);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b4));
                    Callee {
                        addr: 0x0030b260,
                        f: ee::s10::WScriptProcessList_PushFront,
                    }
                    .call(c, 0x0030bda4)?;
                    pc = 0x0030bda4;
                    continue;
                }
                0x0030bda4 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0030bdac);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x0030b208,
                        f: ee::s10::WScriptProcessList_PopFront,
                    }
                    .call(c, 0x0030bdac)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        if back_edge_unwinds(c, 0x0030bd98) {
                            return Ok(());
                        }
                        pc = 0x0030bd98;
                        continue;
                    }
                    pc = 0x0030bdb4;
                    continue;
                }
                0x0030bdb4 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x0030bdcc;
                        continue;
                    }
                    pc = 0x0030bdbc;
                    continue;
                }
                0x0030bdbc => {
                    c.cycles += 1;
                    pc = 0x0030bdc0;
                    continue;
                }
                0x0030bdc0 => {
                    c.cycles += 3;
                    c.sw(c.g[V0][0] as u32, 0u64);
                    c.g[RA][0] = sx(0x0030bdcc);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2b4));
                    Callee {
                        addr: 0x0030b260,
                        f: ee::s10::WScriptProcessList_PushFront,
                    }
                    .call(c, 0x0030bdcc)?;
                    pc = 0x0030bdcc;
                    continue;
                }
                0x0030bdcc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0030bdd4);
                    c.g[A0][0] = c.g[S4][0];
                    Callee {
                        addr: 0x0030b208,
                        f: ee::s10::WScriptProcessList_PopFront,
                    }
                    .call(c, 0x0030bdd4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        if back_edge_unwinds(c, 0x0030bdc0) {
                            return Ok(());
                        }
                        pc = 0x0030bdc0;
                        continue;
                    }
                    pc = 0x0030bddc;
                    continue;
                }
                0x0030bddc => {
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0030bdf0);
                    c.g[S0][0] = c.g[S5][0];
                    Callee {
                        addr: 0x0030bee8,
                        f: ee::s10::WScriptMan_LoadProcessList,
                    }
                    .call(c, 0x0030bdf0)?;
                    c.g[S2][0] = 0x31;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0030be04);
                    c.g[A2][0] = c.g[S3][0];
                    Callee {
                        addr: 0x0030bee8,
                        f: ee::s10::WScriptMan_LoadProcessList,
                    }
                    .call(c, 0x0030be04)?;
                    pc = 0x0030be08;
                    continue;
                }
                0x0030be08 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                    let t = c.g[A0][0] == 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    if t {
                        pc = 0x0030be1c;
                        continue;
                    }
                    pc = 0x0030be14;
                    continue;
                }
                0x0030be14 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0030be1c);
                    call_ptr(c, 0x00224f68, 0x0030be1c)?;
                    pc = 0x0030be1c;
                    continue;
                }
                0x0030be1c => {
                    c.cycles += 2;
                    let t = (c.g[S2][0] as i64) >= 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                    if t {
                        if back_edge_unwinds(c, 0x0030be08) {
                            return Ok(());
                        }
                        pc = 0x0030be08;
                        continue;
                    }
                    pc = 0x0030be24;
                    continue;
                }
                0x0030be24 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = 0x4;
                    c.g[S0][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0030be44);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0030be44)?;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);

                    if (c.g[V0][0] as i64) <= 0 {
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x2a8), 0u64);
                        pc = 0x0030beb4;
                        continue;
                    }
                    pc = 0x0030be50;
                    continue;
                }
                0x0030be50 => {
                    c.cycles += 2;
                    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                    pc = 0x0030be58;
                    continue;
                }
                0x0030be58 => {
                    c.cycles += 22;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = 0x4;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0030be74);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0030be74)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28c));
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[RA][0] = sx(0x0030be84);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    call_ptr(c, 0x003aca38, 0x0030be84)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[RA][0] = sx(0x0030bea0);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x002259a8, 0x0030bea0)?;
                    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
                        if back_edge_unwinds(c, 0x0030be58) {
                            return Ok(());
                        }
                        pc = 0x0030be58;
                        continue;
                    }
                    pc = 0x0030beb0;
                    continue;
                }
                0x0030beb0 => {
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x2a8), 0u64);
                    pc = 0x0030beb4;
                    continue;
                }
                0x0030beb4 => {
                    c.cycles += 13;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x290), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x294), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x298), 0u64);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x2a4), 0u64);
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0030bd20, 0x0030bee8);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{self, fakes, fresh, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Base of the type table the initialisers fill (`*(gp+0xac0)`).
    const TYPE_TABLE: u32 = DATA + 0x4000;

    /// Both initialisers write constants into the type table through the
    /// pointer at `gp+0xac0`; the seeds scramble every register and put junk
    /// in the table and the stack so every store is visible.
    fn init_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0xa77a_c4ed ^ (seed as u64) << 12);
        difftest::scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP + 0xac0, TYPE_TABLE);
        difftest::fill(c, &mut r, TYPE_TABLE, 0x580);
        difftest::fill(c, &mut r, difftest::SP - 0x100, 0x40);
    }

    #[test]
    fn attached_emitter_type_data_init_part1_matches() {
        difftest::check(
            ee::s09::f_002f83b0__generated,
            attached_emitter_type_data_init_part1,
            8,
            &init_setup,
        );
    }

    #[test]
    fn attached_emitter_type_data_init_part2_matches() {
        difftest::check(
            ee::s09::f_002f9838__generated,
            attached_emitter_type_data_init_part2,
            8,
            &init_setup,
        );
    }

    // ---- WScriptMan_LoadState -------------------------------------------

    const LS_ENTRY: u32 = 0x0030_bd20;
    const LS_SELF: u32 = DATA + 0x2000;
    const LS_STREAM: u32 = DATA + 0x6000;
    const LS_VTABLE: u32 = DATA + 0x7000;
    const LS_WORLD: u32 = DATA + 0x8000;
    const LS_TABLES: u32 = DATA + 0x4_0000;
    const LS_NODES: u32 = DATA + 0x5_0000;
    /// Gate for the fakes (in the machine's RAM): only the tests that write
    /// the magic word see them.
    const LS_CTRL: u32 = DATA + 0x2f_fff0;
    const LS_MAGIC: u32 = 0x5c21_9a10;
    /// What the stream's `read` hands out: the `count` word and the table id.
    const LS_COUNT_WORD: u32 = DATA + 0x2f_ffe0;
    const LS_ID_WORD: u32 = DATA + 0x2f_ffe4;
    /// Remaining nodes of each list, keyed by the list address.
    const LS_POPS: u32 = DATA + 0x30_0000;

    fn ls_pops(list: u32) -> u32 {
        LS_POPS + (list & 0xf_fffc)
    }

    /// The gated callee fake: logs like `fakes::fake_call`; `PopFront` hands out
    /// the list's remaining nodes (a counter in RAM), then 0.
    fn oc810_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0] as u32;
        let ra = c.g[RA][0] as u32;
        let pops = addr == 0x0030_b208;
        let n = if pops { c.bus.read32(ls_pops(a0)) } else { 0 };
        fakes::fake_call(c, addr);
        if pops && c.pc == ra {
            if n > 0 {
                c.bus.write32(ls_pops(a0), n - 1);
                c.g[V0][0] = (LS_NODES + 0x10 * n) as u64;
            } else {
                c.g[V0][0] = 0;
            }
        }
    }

    /// The stream's `read(buf, len)`: a 4-byte read to `sp` delivers the
    /// count, to `sp+4` the table id; anything else only logs.
    fn oc810_read_miss(c: &mut Cpu, target: u32) {
        if target == fakes::FAKE && c.g[A2][0] as u32 == 4 {
            let dest = c.g[A1][0] as u32;
            let v = c.bus.read32(if dest & 4 == 0 {
                LS_COUNT_WORD
            } else {
                LS_ID_WORD
            });
            c.bus.write32(dest, v);
        }
        fakes::fake_call(c, target)
    }

    macro_rules! oc810_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(LS_CTRL) != LS_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc810_fake_call(c, $addr);
                }
            )*
            fn oc810_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc810_fakes!(
        (
            oc810_fake_pop_front,
            OC810_PREV_POP_FRONT,
            ee::s10::OVERRIDE_0030B208,
            ee::s10::f_0030b208__generated,
            0x0030_b208
        ),
        (
            oc810_fake_push_front,
            OC810_PREV_PUSH_FRONT,
            ee::s10::OVERRIDE_0030B260,
            ee::s10::f_0030b260__generated,
            0x0030_b260
        ),
        (
            oc810_fake_load_process_list,
            OC810_PREV_LOAD_PROCESS_LIST,
            ee::s10::OVERRIDE_0030BEE8,
            ee::s10::f_0030bee8__generated,
            0x0030_bee8
        ),
        (
            oc810_fake_collect_set_load,
            OC810_PREV_COLLECT_SET_LOAD,
            ee::s10::OVERRIDE_0030C700,
            ee::s10::f_0030c700__generated,
            0x0030_c700
        ),
        (
            oc810_fake_luno_clear,
            OC810_PREV_LUNO_CLEAR,
            ee::s05::OVERRIDE_00224F68,
            ee::s05::f_00224f68__generated,
            0x0022_4f68
        ),
        (
            oc810_fake_luno_read,
            OC810_PREV_LUNO_READ,
            ee::s05::OVERRIDE_002259A8,
            ee::s05::f_002259a8__generated,
            0x0022_59a8
        ),
        (
            oc810_fake_cache_get_data,
            OC810_PREV_CACHE_GET_DATA,
            ee::s13::OVERRIDE_003ACA38,
            ee::s13::f_003aca38__generated,
            0x003a_ca38
        ),
    );

    /// A script manager with a stream: `self->World`, the 50 global table
    /// pointers (some null), the lists' node counts, and the stream reads.
    fn ls_setup(c: &mut Cpu, seed: u32) {
        oc810_install_fakes();
        let mut r = fakes::machine(c, seed, 0x0810_5c21, 14);
        c.hooks.miss = oc810_read_miss;
        c.bus.write32(LS_CTRL, LS_MAGIC);
        difftest::fill(c, &mut r, LS_SELF, 0x100);
        c.bus.write32(LS_SELF + 0x28c, LS_WORLD);
        c.bus.write32(LS_WORLD + 4, DATA + 0x9000);
        for i in 0..50u32 {
            let t = if r.next_u32() % 3 == 0 {
                0
            } else {
                LS_TABLES + 0x10 * i
            };
            c.bus.write32(LS_SELF + 0x3cc + 4 * i, t);
        }
        c.bus.write32(ls_pops(LS_SELF + 0x2b8), seed % 3);
        c.bus.write32(ls_pops(LS_SELF + 0x2bc), (seed / 3) % 3);
        c.bus.write32(
            LS_COUNT_WORD,
            [0, 1, 2, 3, 0x8000_0000][(seed / 9 % 5) as usize],
        );
        c.bus.write32(LS_ID_WORD, r.next_u32() % 50);
        let vt = LS_VTABLE;
        c.bus
            .write16(vt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(vt + 0x14, fakes::FAKE);
        c.bus.write32(LS_STREAM, vt);
        c.g[A0][0] = LS_SELF as u64;
        c.g[A1][0] = LS_STREAM as u64;
        c.pc = LS_ENTRY;
    }

    /// Restoring the script manager from a replay frame: every combination of
    /// list lengths, read counts (none, some, negative) and unwinds in the
    /// stream reads and the table loads.
    #[test]
    fn wscript_man_load_state_matches() {
        difftest::check(
            ee::s10::f_0030bd20__generated,
            wscript_man_load_state,
            180,
            &ls_setup,
        );
    }

    /// The seeds reach the list loops and the counted loop: several distinct
    /// cycle totals, and some runs that return normally.
    #[test]
    fn wscript_man_load_state_paths() {
        let mut seen = std::collections::BTreeSet::new();
        let mut returned = 0;
        for seed in 0..180 {
            let mut c = fresh();
            ls_setup(&mut c, seed);
            let before = c.cycles;
            difftest::run_generated(&mut c, ee::s10::f_0030bd20__generated);
            seen.insert(c.cycles - before);
            returned += (c.pc == difftest::RA) as u32;
        }
        assert!(seen.len() >= 8, "paths {seen:?}");
        assert!(returned >= 20, "returned {returned}");
    }
}
