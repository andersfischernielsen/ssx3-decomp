//! Staging module oc751 (wave wave244): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc751`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x001e_9ad0, c_ov_state_hud1p_init, "cOVStateHUD1P_Init")];

/// `cOVStateHUD1P_onCreateScreen` (0x1e9ad0, `cOVStateHUD1P_Init`): sets up the
/// single-player HUD. Fills the `gTextPictDB` token table (0x4c8980, 0x24-byte
/// entries) with each texture's handle and UVs from the engine's texture bank,
/// resets the timers and the two rider panels, gives each local rider
/// (`+0x87c` set) the next panel with its AI index, places the trophy objects
/// from the layout tables 0x477000 (one rider) or 0x477b40 (two), looks up the
/// fonts, the countdown/place/icon elements, sizes the vertical bars, picks the
/// time separator by the language bits 22..24 of the option word (0x535610),
/// measures "00", "-" and "11", and builds the hint boxes (a random Uber trick,
/// RECOVER and the skip hint). The body is kept block for block: every
/// register and cycle charge follows the generated code.
pub fn c_ov_state_hud1p_init(c: &mut Cpu) {
    'b_1e9bb4: {
        c.cycles += 31;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x440));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x3d0));
        c.g[S4][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
        c.g[A0][0] = 0x4a0000;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x3b0));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x3a0));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20d0));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x390));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x400));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x3f0));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x3e0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x3c0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x380), c.g[RA][0]);
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x438));
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x430));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x428));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x420));
        if call_ptr(c, 0x00317670, 0x001e9b1c).is_err() {
            return;
        }
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x360), c.g[V1][0]);
        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x368), c.g[A0][0]);
        c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw((c.g[S4][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V0][0] = 0x4d0000;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_sub(0x7680));
        let t = c.g[V1][0] == 0u64;
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7680));
        if t {
            break 'b_1e9bb4;
        } else {
            c.cycles += 1;
            c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
            'l_1e9b50: loop {
                c.cycles += 25;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58));
                if call_ptr(c, 0x00398380, 0x001e9b5c).is_err() {
                    return;
                }
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x10));
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.swc1(0, c.g[SP][0] as u32);
                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(5, c.g[SP][0] as u32);
                c.sdl(5, (c.g[S0][0] as u32).wrapping_add(0x1b));
                c.sdr(5, (c.g[S0][0] as u32).wrapping_add(0x14));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x18));
                c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                c.swc1(1, c.g[SP][0] as u32);
                c.ldl(12, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.ldr(12, c.g[SP][0] as u32);
                c.sdl(12, (c.g[S0][0] as u32).wrapping_add(0x23));
                c.sdr(12, (c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                c.g[V0][0] = c.lw(c.g[S0][0] as u32);
                if c.g[V0][0] != 0u64 {
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    if event_due(c) {
                        deliver_event_here(c, 0x001e9b50);
                    }
                    continue 'l_1e9b50;
                } else {
                    break 'b_1e9bb4;
                }
            }
        }
    }
    c.cycles += 71;
    c.g[V0][0] = 0x50;
    c.g[V1][0] = 0xfffffffff0000000;
    c.sw((c.g[S4][0] as u32).wrapping_add(0x3c4), c.g[V0][0]);
    c.g[T9][0] = sx((c.g[S4][0] as u32).wrapping_add(0x48));
    c.sw((c.g[S4][0] as u32).wrapping_add(0x3b4), c.g[V1][0]);
    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4c));
    c.g[AT][0] = 0xffffffffbf800000;
    c.mtc1(1, 0);
    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x3ec));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x370), c.g[T9][0]);
    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x50));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x374), c.g[V0][0]);
    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0xdc));
    c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x3d0));
    c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x494));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x330), c.g[V1][0]);
    c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x478));
    c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x3c0));
    c.g[T0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x47c));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x334), c.g[A0][0]);
    c.g[T4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4fc));
    c.sw((c.g[S4][0] as u32).wrapping_add(0x3c8), 0u64);
    c.g[T5][0] = sx((c.g[S4][0] as u32).wrapping_add(0x53c));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x344), c.g[A1][0]);
    c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(0x544));
    c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x190));
    c.g[T8][0] = sx((c.g[S4][0] as u32).wrapping_add(0x560));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x34c), c.g[A2][0]);
    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0xb48));
    c.sw((c.g[S4][0] as u32).wrapping_add(0x194), 0u64);
    c.mov_s(1, 0);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x340), c.g[A3][0]);
    c.g[T9][0] = sx((c.g[S4][0] as u32).wrapping_add(0x6ac));
    c.sw((c.g[S4][0] as u32).wrapping_add(0x198), 0u64);
    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x7f8));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x348), c.g[T0][0]);
    c.g[S3][0] = 0xffffffffffffffff;
    c.sw((c.g[S4][0] as u32).wrapping_add(0x3a4), 0u64);
    c.g[S2][0] = sx((c.g[S4][0] as u32).wrapping_add(0xd0));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x354), c.g[T4][0]);
    c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(0xcc));
    c.sw((c.g[S4][0] as u32).wrapping_add(0x3a8), 0u64);
    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0xc8));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x358), c.g[T5][0]);
    c.g[T7][0] = sx((c.g[S4][0] as u32).wrapping_add(0xc0));
    c.sw((c.g[S4][0] as u32).wrapping_add(0xa44), 0u64);
    c.g[T6][0] = sx((c.g[S4][0] as u32).wrapping_add(0xbc));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x35c), c.g[S5][0]);
    c.g[T5][0] = sx((c.g[S4][0] as u32).wrapping_add(0xb8));
    c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0xc94));
    c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1dc));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x364), c.g[T8][0]);
    c.g[T4][0] = sx((c.g[S4][0] as u32).wrapping_add(0xb4));
    c.sw((c.g[S4][0] as u32).wrapping_add(0x3cc), 0u64);
    c.g[T3][0] = sx((c.g[S4][0] as u32).wrapping_add(0xb0));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x338), c.g[V1][0]);
    c.g[T2][0] = sx((c.g[S4][0] as u32).wrapping_add(0xac));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x350), c.g[T9][0]);
    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0xc4));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    c.g[T1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x9c));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x33c), c.g[V0][0]);
    c.g[T0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x94));
    c.g[AT][0] = 0x3f000000;
    c.mtc1(1, 2);
    c.g[A3][0] = sx((c.g[S4][0] as u32).wrapping_add(0x90));
    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x374));
    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x8c));
    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x370));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x378), c.g[S5][0]);
    'l_1e9cd0: loop {
        c.cycles += 46;
        c.sw(c.g[A0][0] as u32, c.g[S3][0]);
        c.sw(c.g[A1][0] as u32, c.g[S3][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8c));
        c.sw(c.g[A2][0] as u32, c.g[S3][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8c));
        c.swc1(0, c.g[A3][0] as u32);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8c));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8c));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
        c.swc1(1, c.g[T0][0] as u32);
        c.ldl(24, (c.g[SP][0] as u32).wrapping_add(0x17));
        c.ldr(24, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.ldl(25, (c.g[SP][0] as u32).wrapping_add(0x1f));
        c.ldr(25, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.sdl(24, (c.g[T1][0] as u32).wrapping_add(0x7));
        c.sdr(24, c.g[T1][0] as u32);
        c.sdl(25, (c.g[T1][0] as u32).wrapping_add(0xf));
        c.sdr(25, (c.g[T1][0] as u32).wrapping_add(0x8));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x8c));
        c.swc1(1, c.g[T2][0] as u32);
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x8c));
        c.swc1(2, c.g[T3][0] as u32);
        c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x8c));
        c.sw(c.g[T4][0] as u32, c.g[S3][0]);
        c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x8c));
        c.swc1(1, c.g[T5][0] as u32);
        c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x8c));
        c.sw(c.g[T6][0] as u32, 0u64);
        c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x8c));
        c.swc1(1, c.g[T7][0] as u32);
        c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x8c));
        c.sw(c.g[S0][0] as u32, 0u64);
        c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x8c));
        c.sw(c.g[S1][0] as u32, 0u64);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
        c.sw(c.g[S2][0] as u32, 0u64);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8c));
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8c));
        c.g[T8][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x378));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T8][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8c));
        if t {
            if event_due(c) {
                deliver_event_here(c, 0x001e9cd0);
            }
            continue 'l_1e9cd0;
        } else {
            c.cycles += 8;
            c.sw((c.g[S4][0] as u32).wrapping_add(0x160), 0u64);
            c.g[A0][0] = 0xffffffffffffffff;
            c.g[V1][0] = 0x2;
            c.g[T9][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x330));
            c.sw((c.g[S4][0] as u32).wrapping_add(0x164), 0u64);
            c.sw((c.g[S4][0] as u32).wrapping_add(0x168), 0u64);
            c.g[V0][0] = sx((c.g[T9][0] as u32).wrapping_add(0x30));
            'l_1e9da8: loop {
                c.cycles += 7;
                c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                let t = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x18));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x001e9da8);
                    }
                    continue 'l_1e9da8;
                } else {
                    'b_1e9e74: {
                        c.cycles += 7;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[S1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                        let t = (c.g[A0][0] as i64) <= 0;
                        c.g[A2][0] = 0u64;
                        if t {
                            break 'b_1e9e74;
                        } else {
                            c.cycles += 6;
                            c.g[S2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x370));
                            c.g[S3][0] = 0x1000000;
                            c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x374));
                            c.g[S5][0] = 0x1000000;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            'l_1e9df8: loop {
                                'b_1e9e58: {
                                    c.cycles += 9;
                                    c.g[A1][0] = sx((c.g[S1][0] as u32) << 2);
                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x87c));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_1e9e58;
                                    } else {
                                        c.cycles += 15;
                                        c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
                                        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                        if call_ptr(c, t, 0x001e9e40).is_err() {
                                            return;
                                        }
                                        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                                        c.g[V0][0] = c.g[S3][0];
                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8c));
                                        c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                        c.g[S3][0] =
                                            sx((c.g[S3][0] as u32).wrapping_add(c.g[S5][0] as u32));
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        break 'b_1e9e58;
                                    }
                                }
                                c.cycles += 7;
                                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x7c));
                                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x001e9df8);
                                    }
                                    continue 'l_1e9df8;
                                } else {
                                    break 'b_1e9e74;
                                }
                            }
                        }
                    }
                    'b_1ea09c: {
                        'b_1e9f78: {
                            c.cycles += 10;
                            c.g[AT][0] = 0x44200000;
                            c.mtc1(1, 0);
                            c.g[V0][0] = 0x1;
                            c.g[AT][0] = 0x43f00000;
                            c.mtc1(1, 1);
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
                            c.sw(c.g[SP][0] as u32, 0u64);
                            let t = c.g[A2][0] != c.g[V0][0];
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                            if t {
                                break 'b_1e9f78;
                            } else {
                                c.cycles += 55;
                                c.g[S0][0] = 0x470000;
                                c.g[A1][0] = c.g[S6][0];
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x7000));
                                c.g[A0][0] = c.g[S0][0];
                                if call_ptr(c, 0x001e91f8, 0x001e9eb0).is_err() {
                                    return;
                                }
                                c.g[A2][0] = 0u64;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[S7][0];
                                if call_ptr(c, 0x001e9220, 0x001e9ec0).is_err() {
                                    return;
                                }
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[FP][0];
                                if call_ptr(c, 0x001e9290, 0x001e9ecc).is_err() {
                                    return;
                                }
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                                c.g[A3][0] = c.g[S7][0];
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.g[T0][0] = c.g[FP][0];
                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x44));
                                c.g[A0][0] = c.g[S4][0];
                                c.mul_s(0, 0, 2);
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 4);
                                c.mul_s(1, 1, 3);
                                c.g[T2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x21));
                                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x54));
                                c.g[A2][0] = c.g[S6][0];
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[T3][0] = 0u64;
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                                c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x57));
                                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x47));
                                c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                                c.g[T1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x20));
                                if call_ptr(c, 0x001f10f8, 0x001e9f30).is_err() {
                                    return;
                                }
                                c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                c.g[A1][0] = c.g[SP][0];
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x54));
                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x334));
                                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                                c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x60));
                                c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x64));
                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x68));
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x6c));
                                if call_ptr(c, 0x0020eb50, 0x001e9f60).is_err() {
                                    return;
                                }
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x344));
                                c.g[A1][0] = c.g[SP][0];
                                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                                if call_ptr(c, 0x0020eb50, 0x001e9f70).is_err() {
                                    return;
                                }
                                break 'b_1ea09c;
                            }
                        }
                        c.cycles += 73;
                        c.g[S0][0] = 0x470000;
                        c.g[A1][0] = c.g[S6][0];
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x7b40));
                        c.g[A0][0] = c.g[S0][0];
                        if call_ptr(c, 0x001e91f8, 0x001e9f8c).is_err() {
                            return;
                        }
                        c.g[A2][0] = 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S7][0];
                        if call_ptr(c, 0x001e9220, 0x001e9f9c).is_err() {
                            return;
                        }
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[FP][0];
                        if call_ptr(c, 0x001e9290, 0x001e9fa8).is_err() {
                            return;
                        }
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.g[A3][0] = c.g[S7][0];
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[T0][0] = c.g[FP][0];
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x44));
                        c.g[T3][0] = 0u64;
                        c.mul_s(0, 0, 2);
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 4);
                        c.mul_s(1, 1, 3);
                        c.g[S2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x21));
                        c.g[S1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x20));
                        c.g[A0][0] = c.g[S4][0];
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[T2][0] = c.g[S2][0];
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.g[T1][0] = c.g[S1][0];
                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x74));
                        c.g[A1][0] = c.g[S3][0];
                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.ldl(21, (c.g[SP][0] as u32).wrapping_add(0x77));
                        c.ldr(21, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sdl(21, (c.g[SP][0] as u32).wrapping_add(0x47));
                        c.sdr(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[A2][0] = c.g[S6][0];
                        if call_ptr(c, 0x001f10f8, 0x001ea018).is_err() {
                            return;
                        }
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[A1][0] = c.g[SP][0];
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x74));
                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x334));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x84));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x88));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8c));
                        if call_ptr(c, 0x0020eb50, 0x001ea048).is_err() {
                            return;
                        }
                        c.g[A1][0] = c.g[S3][0];
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A2][0] = c.g[S6][0];
                        c.g[A3][0] = c.g[S7][0];
                        c.g[T0][0] = c.g[FP][0];
                        c.g[T1][0] = c.g[S1][0];
                        c.g[T2][0] = c.g[S2][0];
                        c.g[T3][0] = 0x1;
                        if call_ptr(c, 0x001f10f8, 0x001ea06c).is_err() {
                            return;
                        }
                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[A1][0] = c.g[SP][0];
                        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x74));
                        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x344));
                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x94));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x98));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x9c));
                        if call_ptr(c, 0x0020eb50, 0x001ea09c).is_err() {
                            return;
                        }
                        break 'b_1ea09c;
                    }
                    c.cycles += 67;
                    c.g[S2][0] = 0x19;
                    if call_ptr(c, 0x0014dc80, 0x001ea0a4).is_err() {
                        return;
                    }
                    c.g[S1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34c));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x4;
                    if call_ptr(c, 0x0014dd58, 0x001ea0b4).is_err() {
                        return;
                    }
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = 0x4a0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21f0));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    if call_ptr(c, 0x00317670, 0x001ea0c8).is_err() {
                        return;
                    }
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    if call_ptr(c, 0x003983f0, 0x001ea0d4).is_err() {
                        return;
                    }
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x428), c.g[V0][0]);
                    c.g[A0][0] = 0x4a0000;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21f8));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    if call_ptr(c, 0x00317670, 0x001ea0ec).is_err() {
                        return;
                    }
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    if call_ptr(c, 0x003983f0, 0x001ea0f8).is_err() {
                        return;
                    }
                    c.g[V1][0] = 0x4a0000;
                    c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2200));
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x42c), c.g[V0][0]);
                    if call_ptr(c, 0x00317670, 0x001ea10c).is_err() {
                        return;
                    }
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    if call_ptr(c, 0x00398380, 0x001ea118).is_err() {
                        return;
                    }
                    c.g[A1][0] = 0x4a0000;
                    c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2208));
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x440), c.g[V0][0]);
                    if call_ptr(c, 0x00317670, 0x001ea12c).is_err() {
                        return;
                    }
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    if call_ptr(c, 0x00398380, 0x001ea138).is_err() {
                        return;
                    }
                    c.g[A2][0] = 0x4a0000;
                    c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2210));
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x43c), c.g[V0][0]);
                    if call_ptr(c, 0x00317670, 0x001ea14c).is_err() {
                        return;
                    }
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[A1][0] = c.g[V0][0];
                    if call_ptr(c, 0x00398380, 0x001ea158).is_err() {
                        return;
                    }
                    c.g[A0][0] = 0x4a0000;
                    c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2218));
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x438), c.g[V0][0]);
                    if call_ptr(c, 0x00317670, 0x001ea16c).is_err() {
                        return;
                    }
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[A1][0] = c.g[V0][0];
                    if call_ptr(c, 0x00398380, 0x001ea178).is_err() {
                        return;
                    }
                    c.g[A0][0] = 0x4a0000;
                    c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2220));
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x434), c.g[V0][0]);
                    if call_ptr(c, 0x00317670, 0x001ea18c).is_err() {
                        return;
                    }
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                    c.g[A1][0] = c.g[V0][0];
                    if call_ptr(c, 0x00398380, 0x001ea198).is_err() {
                        return;
                    }
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x470), c.g[V0][0]);
                    c.g[V0][0] = 0x4d0000;
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7728));
                    'l_1ea1a8: loop {
                        c.cycles += 9;
                        c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                        if call_ptr(c, 0x00398380, 0x001ea1c0).is_err() {
                            return;
                        }
                        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                        let t = (c.g[S2][0] as i64) >= 0;
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001ea1a8);
                            }
                            continue 'l_1ea1a8;
                        } else {
                            'b_1ea22c: {
                                'b_1ea210: {
                                    c.cycles += 13;
                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                    c.g[V0][0] = 0xffffffffdeed0000;
                                    c.g[V0][0] = c.g[V0][0] | 0xdeed;
                                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xf50));
                                    c.sw((c.g[S4][0] as u32).wrapping_add(0x490), c.g[V0][0]);
                                    c.sw((c.g[S4][0] as u32).wrapping_add(0x474), c.g[A1][0]);
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1c0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    if call_ptr(c, t, 0x001ea1f8).is_err() {
                                        return;
                                    }
                                    let t = (c.g[V0][0] as i64) < 0;
                                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x340));
                                    if t {
                                        break 'b_1ea210;
                                    } else {
                                        c.cycles += 4;
                                        c.mtc1(2, 0);
                                        c.cvt_s_w(0, 0);
                                        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                        break 'b_1ea22c;
                                    }
                                }
                                c.cycles += 7;
                                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 0, 0);
                                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                break 'b_1ea22c;
                            }
                            'b_1ea27c: {
                                'b_1ea260: {
                                    c.cycles += 9;
                                    c.swc1(0, c.g[A3][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x10d8));
                                    c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x474));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1c8));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1cc));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    if call_ptr(c, t, 0x001ea248).is_err() {
                                        return;
                                    }
                                    let t = (c.g[V0][0] as i64) < 0;
                                    c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x348));
                                    if t {
                                        break 'b_1ea260;
                                    } else {
                                        c.cycles += 4;
                                        c.mtc1(2, 0);
                                        c.cvt_s_w(0, 0);
                                        c.g[T4][0] = 0x4a0000;
                                        break 'b_1ea27c;
                                    }
                                }
                                c.cycles += 7;
                                c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.mtc1(2, 0);
                                c.cvt_s_w(0, 0);
                                c.add_s(0, 0, 0);
                                c.g[T4][0] = 0x4a0000;
                                break 'b_1ea27c;
                            }
                            c.cycles += 93;
                            c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add(0x2210));
                            c.g[S5][0] = 0x4a0000;
                            c.swc1(0, c.g[T0][0] as u32);
                            c.g[S7][0] = 0xc;
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                            if call_ptr(c, 0x00317670, 0x001ea294).is_err() {
                                return;
                            }
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            if call_ptr(c, 0x00398380, 0x001ea2a0).is_err() {
                                return;
                            }
                            c.g[T5][0] = 0x4a0000;
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = sx((c.g[T5][0] as u32).wrapping_add(0x2208));
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x444), c.g[V0][0]);
                            if call_ptr(c, 0x00317670, 0x001ea2b4).is_err() {
                                return;
                            }
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            if call_ptr(c, 0x00398380, 0x001ea2c0).is_err() {
                                return;
                            }
                            c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x2200));
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x448), c.g[V0][0]);
                            if call_ptr(c, 0x00317670, 0x001ea2d0).is_err() {
                                return;
                            }
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[A1][0] = c.g[V0][0];
                            if call_ptr(c, 0x00398380, 0x001ea2dc).is_err() {
                                return;
                            }
                            c.g[A0][0] = 0x4a0000;
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2228));
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x44c), c.g[V0][0]);
                            if call_ptr(c, 0x00317670, 0x001ea2f0).is_err() {
                                return;
                            }
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[A1][0] = c.g[V0][0];
                            if call_ptr(c, 0x00398380, 0x001ea2fc).is_err() {
                                return;
                            }
                            c.g[A0][0] = 0x4a0000;
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2230));
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x450), c.g[V0][0]);
                            if call_ptr(c, 0x00317670, 0x001ea310).is_err() {
                                return;
                            }
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[A1][0] = c.g[V0][0];
                            if call_ptr(c, 0x00398380, 0x001ea31c).is_err() {
                                return;
                            }
                            c.g[A0][0] = 0x4a0000;
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2238));
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x454), c.g[V0][0]);
                            if call_ptr(c, 0x00317670, 0x001ea330).is_err() {
                                return;
                            }
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[A1][0] = c.g[V0][0];
                            if call_ptr(c, 0x00398380, 0x001ea33c).is_err() {
                                return;
                            }
                            c.g[A0][0] = 0x4a0000;
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2240));
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x458), c.g[V0][0]);
                            if call_ptr(c, 0x00317670, 0x001ea350).is_err() {
                                return;
                            }
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[A1][0] = c.g[V0][0];
                            if call_ptr(c, 0x00398380, 0x001ea35c).is_err() {
                                return;
                            }
                            c.g[A0][0] = 0x4a0000;
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2248));
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x45c), c.g[V0][0]);
                            if call_ptr(c, 0x00317670, 0x001ea370).is_err() {
                                return;
                            }
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[A1][0] = c.g[V0][0];
                            if call_ptr(c, 0x00398380, 0x001ea37c).is_err() {
                                return;
                            }
                            c.g[A0][0] = 0x4a0000;
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2250));
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x460), c.g[V0][0]);
                            if call_ptr(c, 0x00317670, 0x001ea390).is_err() {
                                return;
                            }
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[A1][0] = c.g[V0][0];
                            if call_ptr(c, 0x00398380, 0x001ea39c).is_err() {
                                return;
                            }
                            c.g[A0][0] = 0x4a0000;
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2258));
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x464), c.g[V0][0]);
                            if call_ptr(c, 0x00317670, 0x001ea3b0).is_err() {
                                return;
                            }
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[A1][0] = c.g[V0][0];
                            if call_ptr(c, 0x00398380, 0x001ea3bc).is_err() {
                                return;
                            }
                            c.g[A0][0] = 0x4a0000;
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2260));
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x468), c.g[V0][0]);
                            if call_ptr(c, 0x00317670, 0x001ea3d0).is_err() {
                                return;
                            }
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x58));
                            c.g[A1][0] = c.g[V0][0];
                            if call_ptr(c, 0x00398380, 0x001ea3dc).is_err() {
                                return;
                            }
                            c.sw((c.g[S4][0] as u32).wrapping_add(0x46c), c.g[V0][0]);
                            c.g[V1][0] = 0u64;
                            c.g[V0][0] = 0x4d0000;
                            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x76c0));
                            c.mult(0, 2, 3, 23);
                            'l_1ea3f0: loop {
                                c.cycles += 6;
                                c.g[T8][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x354));
                                c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                                c.g[S2][0] = 0x2;
                                c.g[S1][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                                c.g[S0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[T8][0] as u32));
                                'l_1ea408: loop {
                                    c.cycles += 9;
                                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                                    c.g[A1][0] = c.lw(c.g[S1][0] as u32);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58));
                                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                                    if call_ptr(c, 0x00398380, 0x001ea420).is_err() {
                                        return;
                                    }
                                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                                    let t = (c.g[S2][0] as i64) >= 0;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001ea408);
                                        }
                                        continue 'l_1ea408;
                                    } else {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.g[S3][0];
                                        c.g[V0][0] = ((c.g[V1][0] as i64) < 5i64) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.mult(0, 2, 3, 23);
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001ea3f0);
                                            }
                                            continue 'l_1ea3f0;
                                        } else {
                                            'b_1ea4d8: {
                                                'b_1ea4d4: {
                                                    c.cycles += 23;
                                                    c.g[A0][0] = 0x4a0000;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S4][0] as u32).wrapping_add(0x10));
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x2268));
                                                    if call_ptr(c, 0x00317670, 0x001ea44c).is_err()
                                                    {
                                                        return;
                                                    }
                                                    c.g[A0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x58));
                                                    c.g[A1][0] = c.g[V0][0];
                                                    if call_ptr(c, 0x00398380, 0x001ea458).is_err()
                                                    {
                                                        return;
                                                    }
                                                    c.g[A0][0] = 0x470000;
                                                    c.sw(
                                                        (c.g[S4][0] as u32).wrapping_add(0x538),
                                                        c.g[V0][0],
                                                    );
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x6940));
                                                    c.g[A1][0] = c.g[S6][0];
                                                    c.g[A2][0] = 0u64;
                                                    if call_ptr(c, 0x001e9220, 0x001ea470).is_err()
                                                    {
                                                        return;
                                                    }
                                                    c.lwc1(
                                                        1,
                                                        (c.g[SP][0] as u32).wrapping_add(0x24),
                                                    );
                                                    c.g[AT][0] = 0x42580000;
                                                    c.mtc1(1, 0);
                                                    c.g[AT][0] = 0x41b80000;
                                                    c.mtc1(1, 4);
                                                    c.sub_s(3, 1, 0);
                                                    c.c_lt_s(4, 3);
                                                    let t = !c.fcc();
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_1ea4d4;
                                                    } else {
                                                        c.cycles += 14;
                                                        c.lwc1(
                                                            0,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x577c),
                                                        );
                                                        c.g[AT][0] = 0x3f000000;
                                                        c.mtc1(1, 2);
                                                        c.mul_s(0, 3, 0);
                                                        c.cvt_w_s(1, 0);
                                                        c.mfc1(2, 1);
                                                        c.mtc1(2, 1);
                                                        c.cvt_s_w(1, 1);
                                                        c.add_s(1, 1, 2);
                                                        c.mul_s(1, 1, 4);
                                                        c.c_lt_s(1, 3);
                                                        let t = !c.fcc();
                                                        c.sw(
                                                            (c.g[S4][0] as u32).wrapping_add(0x540),
                                                            c.g[V0][0],
                                                        );
                                                        if t {
                                                            break 'b_1ea4d8;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            break 'b_1ea4d4;
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.sw(
                                                    (c.g[S4][0] as u32).wrapping_add(0x540),
                                                    c.g[V0][0],
                                                );
                                                break 'b_1ea4d8;
                                            }
                                            'b_1ea574: {
                                                'b_1ea570: {
                                                    c.cycles += 23;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x358)
                                                        );
                                                    c.g[A0][0] = 0x470000;
                                                    c.g[AT][0] = 0x41b80000;
                                                    c.mtc1(1, 21);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x7480));
                                                    c.lwc1(
                                                        1,
                                                        (c.g[V0][0] as u32).wrapping_add(0x4),
                                                    );
                                                    c.cvt_s_w(1, 1);
                                                    c.g[AT][0] = 0x42580000;
                                                    c.mtc1(1, 20);
                                                    c.lwc1(
                                                        0,
                                                        (c.g[SP][0] as u32).wrapping_add(0x24),
                                                    );
                                                    c.g[A1][0] = c.g[S6][0];
                                                    c.g[A2][0] = 0u64;
                                                    c.mul_s(1, 1, 21);
                                                    c.add_s(1, 1, 20);
                                                    c.div_s(0, 0, 1);
                                                    c.swc1(0, c.g[V0][0] as u32);
                                                    if call_ptr(c, 0x001e9220, 0x001ea51c).is_err()
                                                    {
                                                        return;
                                                    }
                                                    c.lwc1(
                                                        0,
                                                        (c.g[SP][0] as u32).wrapping_add(0x24),
                                                    );
                                                    c.sub_s(3, 0, 20);
                                                    c.c_lt_s(21, 3);
                                                    let t = !c.fcc();
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_1ea570;
                                                    } else {
                                                        c.cycles += 14;
                                                        c.lwc1(
                                                            0,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x5778),
                                                        );
                                                        c.g[AT][0] = 0x3f000000;
                                                        c.mtc1(1, 2);
                                                        c.mul_s(0, 3, 0);
                                                        c.cvt_w_s(1, 0);
                                                        c.mfc1(2, 1);
                                                        c.mtc1(2, 1);
                                                        c.cvt_s_w(1, 1);
                                                        c.add_s(1, 1, 2);
                                                        c.mul_s(1, 1, 21);
                                                        c.c_lt_s(1, 3);
                                                        let t = !c.fcc();
                                                        c.sw(
                                                            (c.g[S4][0] as u32).wrapping_add(0x548),
                                                            c.g[V0][0],
                                                        );
                                                        if t {
                                                            break 'b_1ea574;
                                                        } else {
                                                            c.cycles += 1;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            break 'b_1ea570;
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.sw(
                                                    (c.g[S4][0] as u32).wrapping_add(0x548),
                                                    c.g[V0][0],
                                                );
                                                break 'b_1ea574;
                                            }
                                            'b_1ea604: {
                                                c.cycles += 32;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x35c));
                                                c.g[A0][0] = 0x470000;
                                                c.g[AT][0] = 0x41b80000;
                                                c.mtc1(1, 20);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x7048));
                                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                c.cvt_s_w(0, 0);
                                                c.g[AT][0] = 0x42580000;
                                                c.mtc1(1, 2);
                                                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
                                                c.g[A1][0] = c.g[S6][0];
                                                c.g[A2][0] = 0u64;
                                                c.g[AT][0] = 0x41900000;
                                                c.mtc1(1, 22);
                                                c.mul_s(0, 0, 20);
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 21);
                                                c.mtc1(0, 23);
                                                c.g[S0][0] = 0x1;
                                                c.add_s(0, 0, 2);
                                                c.div_s(1, 1, 0);
                                                c.swc1(1, c.g[V0][0] as u32);
                                                if call_ptr(c, 0x001e9220, 0x001ea5d0).is_err() {
                                                    return;
                                                }
                                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.sw(
                                                    (c.g[S4][0] as u32).wrapping_add(0x550),
                                                    c.g[S0][0],
                                                );
                                                c.sub_s(1, 0, 20);
                                                c.swc1(21, (c.g[S4][0] as u32).wrapping_add(0x54c));
                                                c.sub_s(0, 1, 22);
                                                c.c_lt_s(0, 23);
                                                let t = !c.fcc();
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_1ea604;
                                                } else {
                                                    c.cycles += 4;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x5774),
                                                    );
                                                    c.sw(
                                                        (c.g[S4][0] as u32).wrapping_add(0x550),
                                                        0u64,
                                                    );
                                                    c.mul_s(0, 1, 0);
                                                    c.swc1(
                                                        0,
                                                        (c.g[S4][0] as u32).wrapping_add(0x54c),
                                                    );
                                                    break 'b_1ea604;
                                                }
                                            }
                                            'b_1ea64c: {
                                                c.cycles += 14;
                                                c.g[A0][0] = 0x470000;
                                                c.g[A1][0] = c.g[S6][0];
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x7b88));
                                                c.g[A2][0] = 0u64;
                                                if call_ptr(c, 0x001e9220, 0x001ea618).is_err() {
                                                    return;
                                                }
                                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                c.sw(
                                                    (c.g[S4][0] as u32).wrapping_add(0x558),
                                                    c.g[S0][0],
                                                );
                                                c.sub_s(1, 0, 20);
                                                c.swc1(21, (c.g[S4][0] as u32).wrapping_add(0x554));
                                                c.sub_s(0, 1, 22);
                                                c.c_lt_s(0, 23);
                                                let t = !c.fcc();
                                                c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_1ea64c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_sub(0x5770),
                                                    );
                                                    c.sw(
                                                        (c.g[S4][0] as u32).wrapping_add(0x558),
                                                        0u64,
                                                    );
                                                    c.mul_s(0, 1, 0);
                                                    c.swc1(
                                                        0,
                                                        (c.g[S4][0] as u32).wrapping_add(0x554),
                                                    );
                                                    break 'b_1ea64c;
                                                }
                                            }
                                            'b_1ea6e8: {
                                                'b_1ea6b8: {
                                                    c.cycles += 5;
                                                    c.g[V0][0] = 0x530000;
                                                    c.g[A0][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x5610));
                                                    c.g[V0][0] = c.g[A0][0] & 0x7;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x368)
                                                        );
                                                    if t {
                                                        break 'b_1ea6b8;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x280));
                                                        'l_1ea664: loop {
                                                            c.cycles += 19;
                                                            c.ldl(
                                                                5,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(5, c.g[A0][0] as u32);
                                                            c.ldl(
                                                                6,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                6,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.ldl(
                                                                7,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.ldr(
                                                                7,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.ldl(
                                                                8,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.ldr(
                                                                8,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.sdl(
                                                                5,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(5, c.g[V1][0] as u32);
                                                            c.sdl(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                7,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.sdr(
                                                                7,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.sdl(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x1f),
                                                            );
                                                            c.sdr(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x20));
                                                            let t = c.g[A0][0] != c.g[V0][0];
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x001ea664,
                                                                    );
                                                                }
                                                                continue 'l_1ea664;
                                                            } else {
                                                                c.cycles += 2;
                                                                break 'b_1ea6e8;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x280));
                                                'l_1ea6bc: loop {
                                                    c.cycles += 11;
                                                    c.g[T4][0] = c.ld(c.g[A0][0] as u32);
                                                    c.g[T5][0] =
                                                        c.ld((c.g[A0][0] as u32).wrapping_add(0x8));
                                                    c.g[S5][0] = c
                                                        .ld((c.g[A0][0] as u32).wrapping_add(0x10));
                                                    c.g[T8][0] = c
                                                        .ld((c.g[A0][0] as u32).wrapping_add(0x18));
                                                    c.sd(c.g[V1][0] as u32, c.g[T4][0]);
                                                    c.sd(
                                                        (c.g[V1][0] as u32).wrapping_add(0x8),
                                                        c.g[T5][0],
                                                    );
                                                    c.sd(
                                                        (c.g[V1][0] as u32).wrapping_add(0x10),
                                                        c.g[S5][0],
                                                    );
                                                    c.sd(
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                        c.g[T8][0],
                                                    );
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x20));
                                                    let t = c.g[A0][0] != c.g[V0][0];
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x20));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001ea6bc);
                                                        }
                                                        continue 'l_1ea6bc;
                                                    } else {
                                                        break 'b_1ea6e8;
                                                    }
                                                }
                                            }
                                            'b_1ea738: {
                                                'b_1ea734: {
                                                    c.cycles += 10;
                                                    c.ldl(
                                                        25,
                                                        (c.g[A0][0] as u32).wrapping_add(0x7),
                                                    );
                                                    c.ldr(25, c.g[A0][0] as u32);
                                                    c.sdl(
                                                        25,
                                                        (c.g[V1][0] as u32).wrapping_add(0x7),
                                                    );
                                                    c.sdr(25, c.g[V1][0] as u32);
                                                    c.g[V0][0] = c
                                                        .lw((c.g[SP][0] as u32).wrapping_add(0xa0));
                                                    c.g[V1][0] = 0x1;
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 22);
                                                    c.g[A0][0] = c.g[V0][0] & 0x7;
                                                    let t = c.g[A0][0] == c.g[V1][0];
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0xf04)
                                                        );
                                                    if t {
                                                        break 'b_1ea734;
                                                    } else {
                                                        'b_1ea730: {
                                                            c.cycles += 2;
                                                            let t = c.g[A0][0] == 0u64;
                                                            c.g[V0][0] = 0x2;
                                                            if t {
                                                                break 'b_1ea730;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = c.g[A0][0] == c.g[V0][0];
                                                                c.g[V0][0] = 0x3;
                                                                if t {
                                                                    break 'b_1ea730;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t =
                                                                        c.g[A0][0] == c.g[V0][0];
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0xf04));
                                                                    if t {
                                                                        break 'b_1ea734;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        break 'b_1ea738;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0xf0c));
                                                        break 'b_1ea734;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.sw(
                                                    (c.g[S4][0] as u32).wrapping_add(0x430),
                                                    c.g[V0][0],
                                                );
                                                break 'b_1ea738;
                                            }
                                            c.cycles += 125;
                                            c.g[AT][0] = 0x3f800000;
                                            c.mtc1(1, 20);
                                            c.g[A2][0] = 0u64;
                                            c.g[A1][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x430));
                                            c.g[A3][0] = 0u64;
                                            c.g[A0][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x42c));
                                            c.mov_s(12, 20);
                                            c.mov_s(13, 20);
                                            if call_ptr(c, 0x00391fb0, 0x001ea75c).is_err() {
                                                return;
                                            }
                                            c.g[A0][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x42c));
                                            c.g[A1][0] = 0x4a0000;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x2270));
                                            c.mov_s(12, 20);
                                            c.mov_s(13, 20);
                                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x16c));
                                            c.g[A2][0] = 0u64;
                                            c.g[A3][0] = 0u64;
                                            if call_ptr(c, 0x00391fb0, 0x001ea780).is_err() {
                                                return;
                                            }
                                            c.g[A0][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x42c));
                                            c.g[A1][0] = 0x4a0000;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x2108));
                                            c.mov_s(12, 20);
                                            c.mov_s(13, 20);
                                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x170));
                                            c.g[A2][0] = 0u64;
                                            c.g[A3][0] = 0u64;
                                            if call_ptr(c, 0x00391fb0, 0x001ea7a4).is_err() {
                                                return;
                                            }
                                            c.mov_s(12, 20);
                                            c.g[A0][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x42c));
                                            c.g[A1][0] = 0x4a0000;
                                            c.mov_s(13, 12);
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x2278));
                                            c.g[A2][0] = 0u64;
                                            c.g[A3][0] = 0u64;
                                            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x174));
                                            if call_ptr(c, 0x00391fb0, 0x001ea7c8).is_err() {
                                                return;
                                            }
                                            c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x170));
                                            c.g[V0][0] = 0xffffffffffff0000;
                                            c.g[AT][0] = 0x3e800000;
                                            c.mtc1(1, 2);
                                            c.g[V0][0] = c.g[V0][0] | 0xffff;
                                            c.sub_s(1, 1, 0);
                                            c.sw((c.g[S4][0] as u32).wrapping_add(0x17c), 0u64);
                                            c.sw((c.g[S4][0] as u32).wrapping_add(0x180), 0u64);
                                            c.sw((c.g[S4][0] as u32).wrapping_add(0x184), 0u64);
                                            c.mul_s(1, 1, 2);
                                            c.sw((c.g[S4][0] as u32).wrapping_add(0x188), 0u64);
                                            c.sw(
                                                (c.g[S4][0] as u32).wrapping_add(0x18c),
                                                c.g[V0][0],
                                            );
                                            c.swc1(1, (c.g[S4][0] as u32).wrapping_add(0x178));
                                            if call_ptr(c, 0x003177f0, 0x001ea800).is_err() {
                                                return;
                                            }
                                            c.g[V0][0] = c.g[V0][0] & 0x1;
                                            c.g[A2][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                                            c.sw(
                                                (c.g[S4][0] as u32).wrapping_add(0x55c),
                                                c.g[V0][0],
                                            );
                                            c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[GP][0] as u32).wrapping_sub(0xf20));
                                            c.g[A3][0] =
                                                sx((c.g[GP][0] as u32).wrapping_add(0x1e60));
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                                            c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                                            if call_ptr(c, 0x001e94e0, 0x001ea828).is_err() {
                                                return;
                                            }
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x350));
                                            c.g[A3][0] =
                                                sx((c.g[GP][0] as u32).wrapping_add(0x1e60));
                                            c.g[A2][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                                            c.g[A1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0xf18));
                                            if call_ptr(c, 0x001e94e0, 0x001ea83c).is_err() {
                                                return;
                                            }
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x33c));
                                            c.g[A3][0] =
                                                sx((c.g[GP][0] as u32).wrapping_add(0x1e60));
                                            c.g[A2][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                                            c.g[A1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0xf18));
                                            if call_ptr(c, 0x001e94e0, 0x001ea850).is_err() {
                                                return;
                                            }
                                            c.g[A1][0] = 0x470000;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_sub(0x1548));
                                            if call_ptr(c, 0x003dca40, 0x001ea860).is_err() {
                                                return;
                                            }
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] = 0x470000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x1538));
                                            c.g[S0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            if call_ptr(c, 0x00317670, 0x001ea880).is_err() {
                                                return;
                                            }
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A1][0] = c.g[V0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] = c.g[S0][0];
                                            if call_ptr(c, t, 0x001ea890).is_err() {
                                                return;
                                            }
                                            c.g[A1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[S2][0] = c.g[V0][0];
                                            c.g[A0][0] = 0x470000;
                                            c.g[S0][0] =
                                                c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x1528));
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            if call_ptr(c, 0x00317670, 0x001ea8b4).is_err() {
                                                return;
                                            }
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A0][0] = c.g[S0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[A1][0] = c.g[V0][0];
                                            if call_ptr(c, t, 0x001ea8c4).is_err() {
                                                return;
                                            }
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                                            c.g[A1][0] = c.g[S6][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A3][0] = c.g[V0][0];
                                            if call_ptr(c, 0x003dc9f8, 0x001ea8d8).is_err() {
                                                return;
                                            }
                                            c.g[A2][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                                            c.g[A3][0] =
                                                sx((c.g[GP][0] as u32).wrapping_add(0x1e68));
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x338));
                                            c.g[A1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                                            if call_ptr(c, 0x001e9540, 0x001ea8ec).is_err() {
                                                return;
                                            }
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x400));
                                            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x3f0));
                                            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x3e0));
                                            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x3d0));
                                            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x3c0));
                                            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x3b0));
                                            c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                                            c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x390));
                                            c.g[RA][0] =
                                                c.ld((c.g[SP][0] as u32).wrapping_add(0x380));
                                            c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x438));
                                            c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x430));
                                            c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x428));
                                            c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x420));
                                            let t = c.g[RA][0] as u32;
                                            c.g[SP][0] =
                                                sx((c.g[SP][0] as u32).wrapping_add(0x440));
                                            c.pc = t;
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{elf_image, fresh, run_generated, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Control words of the `cOVStateHUD1P_Init` fixture (per machine): the
    /// gate for the callee fakes, and the "return -1" flags of the two
    /// texture-size calls.
    pub(crate) const OC751_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC751_MAGIC: u32 = 0x0c75_1d0c;

    /// Values the layout fakes (`HUD_GetElemPos`/`Size`/`Scale`) write.
    const OC751_FLOATS: [f32; 12] = [
        0.0, 1.0, 20.0, 30.0, 54.0, 60.0, 77.0, 100.0, 240.0, 400.0, 640.0, 0.5,
    ];

    /// A fake callee: logs the call like `fakes::fake_call`; the three layout
    /// getters also write a pair of floats through `a1`, so the bar-size
    /// arithmetic of the caller sees varied sizes.
    fn oc751_fake_call(c: &mut Cpu, addr: u32) {
        let out = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        let h = fakes::mix(c.cycles ^ (addr as u64) << 20 ^ out as u64);
        fakes::fake_call(c, addr);
        if c.pc == ra && matches!(addr, 0x001e_91f8 | 0x001e_9220 | 0x001e_9290) {
            let x = OC751_FLOATS[(h % 12) as usize];
            let y = OC751_FLOATS[((h >> 8) % 12) as usize];
            c.bus.write32(out, x.to_bits());
            c.bus.write32(out + 4, y.to_bits());
        }
    }

    /// The `miss` hook of the fixture: the texture-size method pointers
    /// (`FAKE + 0x10` width, `FAKE + 0x20` height) return -1 when the flag
    /// word asks for it, to reach the unsigned-conversion paths.
    fn oc751_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        let flags = c.bus.read32(OC751_CTRL + 4);
        let neg =
            (target == FAKE + 0x10 && flags & 1 != 0) || (target == FAKE + 0x20 && flags & 2 != 0);
        if c.pc == ra && neg {
            c.g[V0][0] = sx(0xffff_fff0);
        }
    }

    macro_rules! oc751_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC751_CTRL) != OC751_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc751_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc751_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc751_fakes!(
        (
            oc751_fake_317670,
            OC751_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc751_fake_3177f0,
            OC751_PREV_3177F0,
            ee::s10::OVERRIDE_003177F0,
            ee::s10::f_003177f0__generated,
            0x0031_77f0
        ),
        (
            oc751_fake_398380,
            OC751_PREV_398380,
            ee::s13::OVERRIDE_00398380,
            ee::s13::f_00398380__generated,
            0x0039_8380
        ),
        (
            oc751_fake_3983f0,
            OC751_PREV_3983F0,
            ee::s13::OVERRIDE_003983F0,
            ee::s13::f_003983f0__generated,
            0x0039_83f0
        ),
        (
            oc751_fake_20eb50,
            OC751_PREV_20EB50,
            ee::s05::OVERRIDE_0020EB50,
            ee::s05::f_0020eb50__generated,
            0x0020_eb50
        ),
        (
            oc751_fake_14dc80,
            OC751_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc751_fake_14dd58,
            OC751_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc751_fake_391fb0,
            OC751_PREV_391FB0,
            ee::s12::OVERRIDE_00391FB0,
            ee::s12::f_00391fb0__generated,
            0x0039_1fb0
        ),
        (
            oc751_fake_3dca40,
            OC751_PREV_3DCA40,
            ee::s14::OVERRIDE_003DCA40,
            ee::s14::f_003dca40__generated,
            0x003d_ca40
        ),
        (
            oc751_fake_3dc9f8,
            OC751_PREV_3DC9F8,
            ee::s14::OVERRIDE_003DC9F8,
            ee::s14::f_003dc9f8__generated,
            0x003d_c9f8
        ),
        (
            oc751_fake_1e9220,
            OC751_PREV_1E9220,
            ee::s04::OVERRIDE_001E9220,
            ee::s04::f_001e9220__generated,
            0x001e_9220
        ),
        (
            oc751_fake_1e9290,
            OC751_PREV_1E9290,
            ee::s04::OVERRIDE_001E9290,
            ee::s04::f_001e9290__generated,
            0x001e_9290
        ),
        (
            oc751_fake_1e91f8,
            OC751_PREV_1E91F8,
            ee::s04::OVERRIDE_001E91F8,
            ee::s04::f_001e91f8__generated,
            0x001e_91f8
        ),
        (
            oc751_fake_1f10f8,
            OC751_PREV_1F10F8,
            ee::s04::OVERRIDE_001F10F8,
            ee::s04::f_001f10f8__generated,
            0x001f_10f8
        ),
        (
            oc751_fake_1e94e0,
            OC751_PREV_1E94E0,
            ee::s04::OVERRIDE_001E94E0,
            ee::s04::f_001e94e0__generated,
            0x001e_94e0
        ),
        (
            oc751_fake_1e9540,
            OC751_PREV_1E9540,
            ee::s04::OVERRIDE_001E9540,
            ee::s04::f_001e9540__generated,
            0x001e_9540
        ),
    );

    /// Fixture objects (all inside `DATA`, clear of the fakes' log).
    pub(crate) const OC751_HUD: u32 = DATA + 0x5_0000;
    pub(crate) const OC751_ENGINE: u32 = DATA + 0x5_2000;
    pub(crate) const OC751_WORLD: u32 = DATA + 0x6_0000;
    pub(crate) const OC751_RM: u32 = DATA + 0x6_1000;
    pub(crate) const OC751_RACE: u32 = DATA + 0x6_2000;
    pub(crate) const OC751_RIDER: u32 = DATA + 0x6_3000;
    pub(crate) const OC751_RIDER_CHAR: u32 = DATA + 0x6_4000;
    pub(crate) const OC751_BEING_VT: u32 = DATA + 0x6_8000;
    pub(crate) const OC751_LOCALE: u32 = DATA + 0x6_9000;
    pub(crate) const OC751_LOCALE_VT: u32 = DATA + 0x6_9100;
    pub(crate) const OC751_APP: u32 = DATA + 0x7_0000;
    pub(crate) const OC751_GFX: u32 = DATA + 0x7_2000;
    /// `gTextPictDB`: 0x24-byte entries, hash 0 ends the table.
    pub(crate) const OC751_PICT_DB: u32 = 0x004c_8980;
    /// The option word (language in bits 22..24).
    pub(crate) const OC751_OPTION: u32 = 0x0053_5610;

    /// The `cOVStateHUD1P_Init` fixture for `seed`: a HUD state with its
    /// engine, a texture table of 0..6 entries, a race of 0..3 riders (each
    /// local or not), the app's graphics object (texture sizes, sometimes
    /// negative), the locale object and a random language.
    fn oc751_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x001e_9ad0, 100);
        elf_image(c);
        oc751_install_fakes();
        c.bus.write32(OC751_CTRL, OC751_MAGIC);
        c.hooks.miss = oc751_miss;
        let pick = |r: &mut Rng, n: u32| r.next_u32() % n;
        let gp_at = |off: i32| (GAME_GP as i32 + off) as u32;
        c.bus.write32(OC751_CTRL + 4, pick(&mut r, 4));
        // Globals: the world, the app, the strings and the pointer table.
        c.bus.write32(gp_at(-0x848), OC751_WORLD);
        c.bus.write32(gp_at(-0x854), OC751_APP);
        for off in [-0xf04, -0xf0c, -0xf18, -0xf20, -0xf1c] {
            c.bus.write32(gp_at(off), r.next_u32());
        }
        // The texture table.
        let entries = pick(&mut r, 7);
        for i in 0..8u32 {
            let e = OC751_PICT_DB + 0x24 * i;
            let hash = if i < entries { r.next_u32() | 1 } else { 0 };
            c.bus.write32(e, hash);
            c.bus.write32(e + 4, r.next_u32());
        }
        // The HUD state and its engine.
        c.bus.write32(OC751_HUD + 0x10, OC751_ENGINE);
        c.bus.write32(OC751_ENGINE + 8, DATA + 0x5_3000);
        // World: race manager (+0x84) -> race (+0xc); the locale (+0x8c).
        c.bus.write32(OC751_WORLD + 0x84, OC751_RM);
        c.bus.write32(OC751_RM + 0xc, OC751_RACE);
        let riders = pick(&mut r, 4);
        c.bus.write32(OC751_RACE + 0x7c, riders);
        for i in 0..3u32 {
            let rider = OC751_RIDER + 0x100 * i;
            let chr = OC751_RIDER_CHAR + 0x1000 * i;
            c.bus.write32(OC751_RACE + 0x40 + 4 * i, rider);
            c.bus.write32(rider + 0x18, chr);
            c.bus.write32(chr + 0x87c, pick(&mut r, 2));
            c.bus.write32(chr + 0x6c0, OC751_BEING_VT);
        }
        c.bus.write16(OC751_BEING_VT + 0x38, 0);
        c.bus.write32(OC751_BEING_VT + 0x3c, FAKE + 0x40);
        c.bus.write32(OC751_WORLD + 0x8c, OC751_LOCALE);
        c.bus.write32(OC751_LOCALE + 4, OC751_LOCALE_VT);
        c.bus.write16(OC751_LOCALE_VT + 0x20, 0);
        c.bus.write32(OC751_LOCALE_VT + 0x24, FAKE + 0x50);
        // The app: the graphics object whose methods return the texture sizes.
        c.bus.write32(OC751_APP + 0xf50, r.next_u32());
        c.bus.write32(OC751_APP + 0x10d8, OC751_GFX);
        c.bus.write16(OC751_GFX + 0x1c0, 0);
        c.bus.write32(OC751_GFX + 0x1c4, FAKE + 0x10);
        c.bus.write16(OC751_GFX + 0x1c8, 0);
        c.bus.write32(OC751_GFX + 0x1cc, FAKE + 0x20);
        // The option word: a random language (0..5) in bits 22..24.
        let lang = pick(&mut r, 6);
        c.bus
            .write32(OC751_OPTION, (r.next_u32() & !(7 << 22)) | (lang << 22));
        c.g[A0][0] = sx(OC751_HUD);
        c.pc = 0x001e_9ad0;
    }

    /// `cOVStateHUD1P_Init` against the generated body: the table walk, the
    /// panel setup for 0..3 local riders, the bar-size arithmetic over varied
    /// element sizes, the language switch and the unsigned texture-size
    /// conversions, with every callee faked (so the call log and argument
    /// registers are compared too) and some calls unwinding.
    #[test]
    fn c_ov_state_hud1p_init_matches() {
        covered(
            ee::s04::f_001e9ad0__generated,
            c_ov_state_hud1p_init,
            96,
            &oc751_fixture,
        );
    }

    /// The fixture must reach several distinct paths of the function.
    #[test]
    fn c_ov_state_hud1p_init_covers_paths() {
        let mut costs = std::collections::BTreeSet::new();
        for seed in 0..48 {
            let mut c = fresh();
            oc751_fixture(&mut c, seed);
            run_generated(&mut c, ee::s04::f_001e9ad0__generated);
            costs.insert(c.cycles);
        }
        assert!(costs.len() >= 6, "fixture reaches only {costs:?}");
    }
}
