//! Staging module oc651 (wave 234, list oc651): hand-written replacements, each
//! with its differential test against the generated body. Moved into the
//! source-tree units by `tools/reorg_modules.py apply oc651`.
//!
//! The bodies are transcriptions of the generated code: the same stores, loads,
//! cycle charges and register writes in the same order, with each call made
//! through `call_ptr` (the unwind check) and the `ra` set that `call_ptr` makes
//! left out.

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
    (0x0024_8970, visual_effects_test_menu_ctor, "VisualEffectsTestMenu_ctor"),
    (0x0024_8dc8, plants_menu_ctor, "PlantsMenu_ctor"),
];

/// `VisualEffectsTestMenu_ctor(this)` (0x248970): the debug menu for the
/// visual-effects test. Runs the `cMenu` base constructor, sets the vtable,
/// builds the title and spacer items, then the 17 int items (HUD toggles,
/// the current element, position, size, scale, colour, layer and the world
/// painter enable and layer id), and appends all 19 items to the menu.
/// Returns `this`. Transcribed from the generated body; each call goes through
/// `call_ptr` so an unwinding callee returns at once, as the original does.
pub fn visual_effects_test_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 278;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3a4));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x404));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x344));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3d4));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2e4));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
        c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x374));
        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x434));
        call_ptr(c, 0x002caa58, 0x002489c8)?;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x464));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
        c.g[A1][0] = 0x480000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1fc8));
        c.g[V0][0] = 0x480000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x378));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[SP][0] as u32));
        call_ptr(c, 0x002cd2a0, 0x002489f0)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002cd350, 0x00248a04)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c4));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1fa8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1388));
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x00248a34)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x224));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f90));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        c.g[A1][0] = 0x1;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x138c));
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x00248a64)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x254));
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x284));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f78));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
        c.g[A1][0] = 0x2;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1390));
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x00248a94)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b4));
        c.g[T1][0] = 0x480000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x24), c.g[V0][0]);
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f68));
        c.g[A1][0] = 0x3;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1394));
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x3e8;
        call_ptr(c, 0x002cd8f8, 0x00248abc)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x314));
        c.g[T1][0] = 0x480000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x28), c.g[V0][0]);
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f50));
        c.g[A1][0] = 0x4;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1398));
        c.g[A3][0] = 0xfffffffffffffd80;
        c.g[T0][0] = 0x280;
        call_ptr(c, 0x002cd8f8, 0x00248ae4)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[T1][0] = 0x480000;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f40));
        c.g[A1][0] = 0x5;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x139c));
        c.g[A3][0] = 0xfffffffffffffe20;
        c.g[T0][0] = 0x1e0;
        call_ptr(c, 0x002cd8f8, 0x00248b04)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[T1][0] = 0x4a0000;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2cf0));
        c.g[A1][0] = 0x6;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13a0));
        c.g[A3][0] = 0xfffffffffffffd80;
        c.g[T0][0] = 0x280;
        call_ptr(c, 0x002cd8f8, 0x00248b24)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
        c.g[T1][0] = 0x4a0000;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2cf8));
        c.g[A1][0] = 0x7;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13a4));
        c.g[A3][0] = 0xfffffffffffffd80;
        c.g[T0][0] = 0x280;
        call_ptr(c, 0x002cd8f8, 0x00248b44)?;
        c.g[T1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S7][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d00));
        c.g[A1][0] = 0x8;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13a8));
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[T0][0] = 0x3e8;
        call_ptr(c, 0x002cd8f8, 0x00248b64)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
        c.g[T1][0] = 0x4a0000;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d08));
        c.g[A1][0] = 0x9;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13ac));
        c.g[A3][0] = 0xffffffffffffffff;
        c.g[T0][0] = 0x3e8;
        call_ptr(c, 0x002cd8f8, 0x00248b84)?;
        c.g[T1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S5][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d10));
        c.g[A1][0] = 0xa;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13b0));
        c.g[A3][0] = 0xffffffffffffff01;
        c.g[T0][0] = 0xff;
        call_ptr(c, 0x002cd8f8, 0x00248ba4)?;
        c.g[T1][0] = 0x4a0000;
        c.g[A0][0] = c.g[FP][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d18));
        c.g[A1][0] = 0xb;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13b4));
        c.g[A3][0] = 0xffffffffffffff01;
        c.g[T0][0] = 0xff;
        call_ptr(c, 0x002cd8f8, 0x00248bc4)?;
        c.g[T1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S3][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d20));
        c.g[A1][0] = 0xc;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13b8));
        c.g[A3][0] = 0xffffffffffffff01;
        c.g[T0][0] = 0xff;
        call_ptr(c, 0x002cd8f8, 0x00248be4)?;
        c.g[T1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S6][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d28));
        c.g[A1][0] = 0xd;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13bc));
        c.g[A3][0] = 0xffffffffffffff01;
        c.g[T0][0] = 0xff;
        call_ptr(c, 0x002cd8f8, 0x00248c04)?;
        c.g[T1][0] = 0x480000;
        c.g[A0][0] = c.g[S2][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f30));
        c.g[A1][0] = 0xe;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13c0));
        c.g[A3][0] = 0xfffffffffffffff0;
        c.g[T0][0] = 0x10;
        call_ptr(c, 0x002cd8f8, 0x00248c24)?;
        c.g[T1][0] = 0x480000;
        c.g[A0][0] = c.g[S4][0];
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f20));
        c.g[A1][0] = 0xf;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13c4));
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0x1;
        call_ptr(c, 0x002cd8f8, 0x00248c44)?;
        c.g[T1][0] = 0x480000;
        c.g[A3][0] = 0u64;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1f00));
        c.g[T0][0] = 0x64;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x13c8));
        c.g[A1][0] = 0x10;
        call_ptr(c, 0x002cd8f8, 0x00248c64)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248c74)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248c84)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248c94)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248ca4)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248cb4)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248cc4)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248cd4)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x1c));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248ce4)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248cf4)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x24));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d04)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d14)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d24)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d34)?;
        c.g[A1][0] = c.g[FP][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d44)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d54)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d64)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d74)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d84)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00248d94)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        Ok(())
    });
}

/// `PlantsMenu_ctor(this)` (0x248dc8): the debug menu for the plant sway and
/// branch parameters. Runs the `cMenu` base constructor, sets the vtable,
/// builds the title and spacer items, the enable int item and the float
/// items (sway, branch and rotation amplitudes and frequencies, direction),
/// then appends all 14 items to the menu. Returns `this`. Transcribed from
/// the generated body; each call goes through `call_ptr`.
pub fn plants_menu_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 223;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x130));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fc));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x330));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S5][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c8));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S6][0] = sx((c.g[S0][0] as u32).wrapping_add(0x264));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1fc));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        c.g[FP][0] = sx((c.g[S0][0] as u32).wrapping_add(0x298));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x364));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x398));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
        call_ptr(c, 0x002caa58, 0x00248e30)?;
        c.mtc1(0, 20);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14c));
        c.g[A1][0] = 0x480000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1ee8));
        c.g[V0][0] = 0x480000;
        c.g[AT][0] = 0x40000000;
        c.mtc1(1, 21);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3d0));
        c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x5004));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x12c), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[SP][0] as u32));
        call_ptr(c, 0x002cd2a0, 0x00248e64)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x164));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A1][0] = 0x28;
        call_ptr(c, 0x002cd350, 0x00248e78)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x194));
        c.g[T1][0] = 0x4a0000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2d30));
        c.g[T0][0] = 0x2;
        c.g[A3][0] = 0u64;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1354));
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x002cd8f8, 0x00248ea0)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c8));
        c.g[A3][0] = 0x480000;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1ed8));
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1358));
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x002cdce8, 0x00248ec8)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x230));
        c.g[AT][0] = 0xffffffffc0a00000;
        c.mtc1(1, 12);
        c.g[A3][0] = 0x480000;
        c.g[AT][0] = 0x40a00000;
        c.mtc1(1, 13);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1ec8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[V0][0]);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x135c));
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002cdce8, 0x00248ef8)?;
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[S7][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1eb8));
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1360));
        c.g[A1][0] = 0x3;
        call_ptr(c, 0x002cdce8, 0x00248f18)?;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[A3][0] = 0x480000;
        c.mov_s(13, 21);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1ea0));
        c.mov_s(12, 20);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1364));
        c.g[AT][0] = 0x43b40000;
        c.mtc1(1, 21);
        c.g[A1][0] = 0x4;
        call_ptr(c, 0x002cdce8, 0x00248f40)?;
        c.g[AT][0] = 0x447a0000;
        c.mtc1(1, 13);
        c.g[A3][0] = 0x480000;
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S6][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1e88));
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1368));
        c.g[A1][0] = 0x5;
        call_ptr(c, 0x002cdce8, 0x00248f64)?;
        c.g[T1][0] = 0x480000;
        c.g[T0][0] = 0x1770;
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x1e78));
        c.g[A0][0] = c.g[FP][0];
        c.g[A3][0] = 0u64;
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x136c));
        c.g[A1][0] = 0x6;
        call_ptr(c, 0x002cd8f8, 0x00248f84)?;
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[S5][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1e58));
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1370));
        c.g[A1][0] = 0x7;
        call_ptr(c, 0x002cdce8, 0x00248fa4)?;
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[S3][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1e40));
        c.mov_s(12, 20);
        c.mov_s(13, 22);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1374));
        c.g[A1][0] = 0x8;
        call_ptr(c, 0x002cdce8, 0x00248fc4)?;
        c.g[A3][0] = 0x480000;
        c.mov_s(13, 22);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1e20));
        c.g[A0][0] = c.g[S4][0];
        c.mov_s(12, 20);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1378));
        c.g[A1][0] = 0x9;
        call_ptr(c, 0x002cdce8, 0x00248fe4)?;
        c.g[A3][0] = 0x480000;
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1e00));
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x137c));
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x002cdce8, 0x00249004)?;
        c.g[A3][0] = 0x480000;
        c.mov_s(12, 20);
        c.mov_s(13, 21);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1df0));
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x1380));
        c.g[A1][0] = 0xb;
        call_ptr(c, 0x002cdce8, 0x00249024)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249034)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249044)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249054)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249064)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249074)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249084)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249094)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002490a4)?;
        c.g[A1][0] = c.g[FP][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002490b4)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002490c4)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002490d4)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002490e4)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x002490f4)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0xffffffffffffffff;
        call_ptr(c, 0x002cbcc8, 0x00249104)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA};

    /// The menu object both constructors fill (zeroed memory; the constructors
    /// write every field they read back).
    pub(crate) const OC651_MENU: u32 = DATA + 0x1000;

    /// A machine with `this` in a0 and the entry in `pc`.
    fn oc651_menu_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let _ = fakes::machine(c, seed, 0x651_0001, 0);
        c.pc = entry;
        c.g[A0][0] = OC651_MENU as u64;
    }

    #[test]
    fn visual_effects_test_menu_ctor_matches() {
        check(ssx3_ee::s06::f_00248970__generated, visual_effects_test_menu_ctor, 200, &|c, seed| {
            oc651_menu_setup(c, seed, 0x0024_8970)
        });
    }

    #[test]
    fn plants_menu_ctor_matches() {
        check(ssx3_ee::s06::f_00248dc8__generated, plants_menu_ctor, 200, &|c, seed| {
            oc651_menu_setup(c, seed, 0x0024_8dc8)
        });
    }
}
