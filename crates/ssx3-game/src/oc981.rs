//! Staging module oc981 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc981`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0020_fb40,
    c_ov_state_hud1_p_draw_progress_meter_mp,
    "cOVStateHUD1P_DrawProgressMeterMP",
)];

/// `void cOVStateHUD1P::DrawProgressMeterMP(cRider* rider, int right)` (0x0020fb40):
/// the multiplayer course-progress strip of the 1P HUD. Draws a vertical
/// meter (`this` = `a0`: X/Y/W/H at +0x20..+0x2c, pixels per course unit at
/// +0x30, last drawn rider index at +0x34; `a1` the rider, `a2` which side)
/// with one icon per racer (own, human, AI: draw prims 12/13/14) and per
/// course marker, fading at both ends. Early outs: HUD-off flag (gp-0x990),
/// no radar header, no rider. The racers' relative positions are rate-limited
/// towards their targets (`limitStep`, 46.296 per call, clamp +-2000) and
/// kept in the six-entry table at 0x4c8bc8. Every register, cycle and call is
/// kept as in the generated body (the body is its straight conversion with
/// the calls through `call_ptr` and the loop event checks).
pub fn c_ov_state_hud1_p_draw_progress_meter_mp(c: &mut Cpu) {
    guest(c, |c| {
        'b_210558: {
            'b_210554: {
                c.cycles += 25;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1d0));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x990));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.g[S3][0] = c.g[A2][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                c.swc1(29, (c.g[SP][0] as u32).wrapping_add(0x1c8));
                c.swc1(28, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.swc1(27, (c.g[SP][0] as u32).wrapping_add(0x1b8));
                c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x198));
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x188));
                let t = c.g[V0][0] != 0u64;
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x180));
                if t {
                    break 'b_210554;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x994));
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
                    if t {
                        break 'b_210558;
                    } else {
                        c.cycles += 2;
                        let t = c.g[A1][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x998));
                        if t {
                            break 'b_210558;
                        } else {
                            c.cycles += 3;
                            c.g[S7][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9a0));
                            let t = c.g[S7][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), c.g[V0][0]);
                            if t {
                                break 'b_210558;
                            } else {
                                c.cycles += 29;
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x6c0));
                                c.g[FP][0] = 0x4d0000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x0020fbe0);
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x0020fbe0)?;
                                c.g[S5][0] = c.g[V0][0];
                                c.g[A0][0] = 0x18;
                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.mult(0, 4, 21, 4);
                                c.g[V0][0] = 0x4d0000;
                                c.g[AT][0] = 0xffffffffc4fa0000;
                                c.mtc1(1, 7);
                                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7438));
                                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                                c.g[V1][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                c.g[S6][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                                c.g[V0][0] = c.g[V1][0];
                                c.g[AT][0] = 0x44fa0000;
                                c.mtc1(1, 6);
                                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x54c8));
                                c.g[T1][0] = 0x1;
                                c.lwc1(4, (c.g[V1][0] as u32).wrapping_add(0xc));
                                c.g[T0][0] = 0u64;
                                c.g[S4][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x78));
                                c.lwc1(22, (c.g[V0][0] as u32).wrapping_add(0x8));
                                'l_20fc38: loop {
                                    'b_20fc44: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x34));
                                        if c.g[V0][0] != c.g[S5][0] {
                                            c.sw((c.g[A2][0] as u32), 0u64);
                                            break 'b_20fc44;
                                        } else {
                                            break 'b_20fc44;
                                        }
                                    }
                                    'b_20fcd0: {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[T0][0] as i64) < (c.g[S4][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                            break 'b_20fcd0;
                                        } else {
                                            'b_20fc8c: {
                                                'b_20fc74: {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[A2][0] as u32).wrapping_sub(0x8),
                                                    );
                                                    c.sub_s(0, 0, 4);
                                                    c.c_lt_s(0, 7);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_20fc74;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[AT][0] = 0xffffffffc4fa0000;
                                                        c.mtc1(1, 0);
                                                        let t = true;
                                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                                                        if t {
                                                            break 'b_20fc8c;
                                                        } else {
                                                            break 'b_20fc74;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.c_lt_s(6, 0);
                                                let t = !c.fcc();
                                                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                                                if t {
                                                    break 'b_20fc8c;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[AT][0] = 0x44fa0000;
                                                    c.mtc1(1, 0);
                                                    break 'b_20fc8c;
                                                }
                                            }
                                            'b_20fcc8: {
                                                c.cycles += 2;
                                                if c.g[V0][0] == 0u64 {
                                                    c.swc1(0, (c.g[A3][0] as u32));
                                                    break 'b_20fcc8;
                                                } else {
                                                    'b_20fcc4: {
                                                        'b_20fcb4: {
                                                            c.cycles += 6;
                                                            c.lwc1(2, (c.g[A3][0] as u32));
                                                            c.lwc1(
                                                                3,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54c4),
                                                            );
                                                            c.sub_s(1, 0, 2);
                                                            c.c_lt_s(3, 1);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_20fcb4;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.add_s(0, 2, 3);
                                                                if t {
                                                                    break 'b_20fcc4;
                                                                } else {
                                                                    break 'b_20fcb4;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.c_lt_s(1, 5);
                                                        if c.fcc() {
                                                            c.sub_s(0, 2, 3);
                                                            break 'b_20fcc4;
                                                        } else {
                                                            break 'b_20fcc4;
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.swc1(0, (c.g[A3][0] as u32));
                                                    break 'b_20fcc8;
                                                }
                                            }
                                            c.cycles += 2;
                                            c.sw((c.g[A2][0] as u32), c.g[T1][0]);
                                            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
                                            break 'b_20fcd0;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = ((c.g[T0][0] as i64) < 6i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x18));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0020fc38);
                                        }
                                        continue 'l_20fc38;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V0][0] = 0x3;
                                        c.g[V1][0] = 0xffffffffffffffff;
                                        'l_20fce8: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                            let t = c.g[V0][0] != c.g[V1][0];
                                            c.g[A0][0] = 0x80;
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0020fce8);
                                                }
                                                continue 'l_20fce8;
                                            } else {
                                                c.cycles += 5;
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 0);
                                                c.g[V1][0] = c.g[SP][0];
                                                c.g[A1][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    c.g[A0][0],
                                                );
                                                'l_20fd18: loop {
                                                    c.cycles += 8;
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x14),
                                                        c.g[A0][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                        c.g[A0][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x1c),
                                                        c.g[A0][0],
                                                    );
                                                    c.swc1(
                                                        0,
                                                        (c.g[V1][0] as u32).wrapping_add(0x8),
                                                    );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x30));
                                                    c.g[V0][0] = ((c.g[V1][0] as i64)
                                                        < (c.g[A1][0] as i64))
                                                        as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.sw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x10),
                                                            c.g[A0][0],
                                                        );
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0020fd18);
                                                        }
                                                        continue 'l_20fd18;
                                                    } else {
                                                        'b_20fd48: {
                                                            c.cycles += 2;
                                                            let t = c.g[S3][0] != 0u64;
                                                            c.lwc1(
                                                                21,
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            if t {
                                                                break 'b_20fd48;
                                                            } else {
                                                                c.cycles += 2;
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[S2][0] as u32)
                                                                        .wrapping_add(0x28),
                                                                );
                                                                c.add_s(21, 21, 0);
                                                                break 'b_20fd48;
                                                            }
                                                        }
                                                        'b_210240: {
                                                            c.cycles += 146;
                                                            c.g[A1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[S0][0] = 0u64;
                                                            c.g[AT][0] = 0x3f800000;
                                                            c.mtc1(1, 20);
                                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32)
                                                                .wrapping_add(0x10d8));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0xf8));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xfc));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x0020fd6c);
                                                            c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x0020fd6c)?;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[T0][0] = 0xfffffffffe7f0000;
                                                            c.g[T0][0] = c.g[T0][0] | 0xffff;
                                                            c.g[T4][0] = 0x1000000;
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[T1][0] = 0xffffffffffcf0000;
                                                            c.g[T1][0] = c.g[T1][0] | 0xffff;
                                                            c.g[T3][0] = 0xfffffffffff00000;
                                                            c.ldl(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(3, (c.g[V0][0] as u32));
                                                            c.ldl(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A2][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.sdl(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x1b),
                                                            );
                                                            c.sdr(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x14),
                                                            );
                                                            c.sdl(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x23),
                                                            );
                                                            c.sdr(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x1c),
                                                            );
                                                            c.sw(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x24),
                                                                c.g[A2][0],
                                                            );
                                                            c.g[T3][0] = c.g[T3][0] | 0xfff;
                                                            c.g[T2][0] = 0x10000;
                                                            c.g[T5][0] = 0xffffffffffffff83;
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[T2][0] = c.g[T2][0] | 0x4000;
                                                            c.g[A2][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[A1][0] = 0x500000;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0xe60));
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xe84),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V0][0] = 0x500000;
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[A3][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1420));
                                                            c.ldl(
                                                                14,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(14, (c.g[A3][0] as u32));
                                                            c.ldl(
                                                                4,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                4,
                                                                (c.g[A3][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A2][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.sdl(
                                                                14,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(14, (c.g[V1][0] as u32));
                                                            c.sdl(
                                                                4,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                4,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x10),
                                                                c.g[A2][0],
                                                            );
                                                            c.g[A3][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[A0][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                                                            c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A0][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                                                            c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A2][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
                                                            c.g[V0][0] = c.g[V0][0] | 0x14;
                                                            c.sw(
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x10d8));
                                                            c.g[A0][0] = c.lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x110));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x114));
                                                            let t = c.g[V0][0] as u32;
                                                            c.g[RA][0] = sx(0x0020fe74);
                                                            c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x0020fe74)?;
                                                            c.g[V0][0] = sx((c.g[FP][0] as u32)
                                                                .wrapping_sub(0x73a8));
                                                            c.lwc1(
                                                                27,
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0x24),
                                                            );
                                                            c.g[T0][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc));
                                                            c.g[T2][0] = 0xfffffffffffffc1f;
                                                            c.lwc1(
                                                                0,
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0x2c),
                                                            );
                                                            c.g[A1][0] = 0x4;
                                                            c.lwc1(
                                                                1,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.g[A2][0] = c.g[SP][0];
                                                            c.add_s(23, 27, 0);
                                                            c.lwc1(
                                                                4,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54c0),
                                                            );
                                                            c.swc1(1, (c.g[SP][0] as u32));
                                                            c.g[A3][0] = 0u64;
                                                            c.lwc1(
                                                                0,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54bc),
                                                            );
                                                            c.sub_s(5, 21, 4);
                                                            c.lwc1(
                                                                3,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.add_s(4, 21, 4);
                                                            c.sub_s(26, 22, 0);
                                                            c.lwc1(
                                                                2,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54b8),
                                                            );
                                                            c.swc1(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.lwc1(
                                                                1,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x14),
                                                            );
                                                            c.add_s(29, 26, 2);
                                                            c.swc1(
                                                                1,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x34),
                                                            );
                                                            c.lwc1(
                                                                1,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.swc1(
                                                                1,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x60),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x64),
                                                            );
                                                            c.lwc1(
                                                                1,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x14),
                                                            );
                                                            c.swc1(
                                                                1,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x90),
                                                            );
                                                            c.lwc1(
                                                                0,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.swc1(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.swc1(
                                                                23,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc4),
                                                            );
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc8),
                                                                0u64,
                                                            );
                                                            c.swc1(
                                                                20,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xcc),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x94),
                                                            );
                                                            c.lq(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.swc1(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc8),
                                                                0u64,
                                                            );
                                                            c.sq(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.swc1(
                                                                5,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x50),
                                                            );
                                                            c.swc1(
                                                                27,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc4),
                                                            );
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc8),
                                                                0u64,
                                                            );
                                                            c.lq(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.g[T1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.swc1(
                                                                4,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc8),
                                                                0u64,
                                                            );
                                                            c.sq(
                                                                3,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x80),
                                                            );
                                                            c.lq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xb0),
                                                            );
                                                            c.g[A0][0] = c.lw((c.g[T1][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
                                                            c.g[V0][0] = c.g[V0][0] | 0x120;
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[T0][0] as u32));
                                                            c.g[V0][0] = c.lw((c.g[T1][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.sh(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x10),
                                                                c.g[V1][0],
                                                            );
                                                            c.g[T0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[V0][0] = c.lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0x10d8));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x238));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x23c));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x0020ff7c);
                                                            c.g[A0][0] = sx((c.g[T0][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x0020ff7c)?;
                                                            c.g[AT][0] = 0x42700000;
                                                            c.mtc1(1, 0);
                                                            c.sub_s(25, 23, 0);
                                                            let t = (c.g[S4][0] as i64) <= 0;
                                                            c.add_s(24, 27, 0);
                                                            if t {
                                                                break 'b_210240;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.lwc1(
                                                                    28,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x54b4),
                                                                );
                                                                c.mov_s(22, 20);
                                                                c.g[V1][0] = 0x4d0000;
                                                                'l_20ffa0: loop {
                                                                    'b_210234: {
                                                                        'b_210230: {
                                                                            c.cycles += 9;
                                                                            c.g[V0][0] = 0x18;
                                                                            c.g[V1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x7430,
                                                                                    ));
                                                                            c.lo[0] = c.g[V1][0];
                                                                            c.madd(0, 2, 16, 2);
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.c_lt_s(0, 26);
                                                                            let t = c.fcc();
                                                                            c.g[S1][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x1,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_210230;
                                                                            } else {
                                                                                c.cycles += 4;
                                                                                c.c_lt_s(29, 0);
                                                                                if c.fcc() {
                                                                                    c.g[S0][0] =
                                                                                        c.g[S1][0];
                                                                                    break 'b_210234;
                                                                                } else {
                                                                                    'b_21002c: {
                                                                                        c.cycles +=
                                                                                            7;
                                                                                        c.sub_s(
                                                                                            0, 0,
                                                                                            26,
                                                                                        );
                                                                                        c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x30));
                                                                                        c.g[A0]
                                                                                            [0] =
                                                                                            0xc;
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x8;
                                                                                        c.mul_s(
                                                                                            0, 0, 1,
                                                                                        );
                                                                                        let t = c.g[S5][0] == c.g[S0][0];
                                                                                        c.sub_s(
                                                                                            20, 23,
                                                                                            0,
                                                                                        );
                                                                                        if t {
                                                                                            break 'b_21002c;
                                                                                        } else {
                                                                                            'b_210024: {
                                                                                                c.cycles += 11;
                                                                                                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                                                                                                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                                                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                                                                                                c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                                                                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                                                                                                let t = c.g[V0][0] as u32;
                                                                                                c.g[RA][0] = sx(0x00210014);
                                                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                                                                call_ptr(c, t, 0x00210014)?;
                                                                                                let t = c.g[V0][0] == 0u64;
                                                                                                c.g[A0][0] = 0xd;
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_210024;
                                                                                                } else {
                                                                                                    c.cycles += 2;
                                                                                                    let t = true;
                                                                                                    c.g[V1][0] = 0xa;
                                                                                                    if t {
break 'b_21002c;
} else {
break 'b_210024;
}
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 2;
                                                                                            c.g[A0][0] = 0xe;
                                                                                            c.g[V1][0] = 0x9;
                                                                                            break 'b_21002c;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 14;
                                                                                    c.g[V1][0] = sx(
                                                                                        (c.g[V1][0]
                                                                                            as u32)
                                                                                            << 4,
                                                                                    );
                                                                                    c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_sub(0x73a8));
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                    c.g[AT][0] =
                                                                                        0x3f000000;
                                                                                    c.mtc1(1, 1);
                                                                                    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                                                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                                                    c.g[T1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                                                                    c.mul_s(
                                                                                        0, 0, 1,
                                                                                    );
                                                                                    c.sub_s(
                                                                                        4, 20, 0,
                                                                                    );
                                                                                    c.c_le_s(4, 27);
                                                                                    let t = c.fcc();
                                                                                    c.add_s(
                                                                                        5, 20, 0,
                                                                                    );
                                                                                    if t {
                                                                                        break 'b_210230;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.c_le_s(
                                                                                            23, 5,
                                                                                        );
                                                                                        if c.fcc() {
                                                                                            c.g[S0][0] = c.g[S1][0];
                                                                                            break 'b_210234;
                                                                                        } else {
                                                                                            'b_2100a8: {
                                                                                                'b_2100a0: {
                                                                                                    'b_210090: {
                                                                                                        c.cycles += 5;
                                                                                                        c.c_lt_s(4, 24);
                                                                                                        c.g[AT][0] = 0x3f800000;
                                                                                                        c.mtc1(1, 1);
                                                                                                        let t = !c.fcc();

                                                                                                        if t {
break 'b_210090;
} else {
c.cycles += 2;
let t = true;
c.sub_s(0, 24, 4);
if t {
break 'b_2100a0;
} else {
break 'b_210090;
}
}
                                                                                                    }
                                                                                                    c.cycles += 3;
                                                                                                    c.c_lt_s(25, 5);
                                                                                                    let t = !c.fcc();

                                                                                                    if t {
break 'b_2100a8;
} else {
c.cycles += 1;
c.sub_s(0, 5, 25);
break 'b_2100a0;
}
                                                                                                }
                                                                                                c.cycles += 2;
                                                                                                c.mul_s(0, 0, 28);
                                                                                                c.sub_s(1, 1, 0);
                                                                                                break 'b_2100a8;
                                                                                            }
                                                                                            'b_2100bc: {
                                                                                                c.cycles += 5;
                                                                                                c.mtc1(0, 0);
                                                                                                c.c_lt_s(1, 0);
                                                                                                if c.fcc() {
c.mov_s(1, 0);
break 'b_2100bc;
} else {
break 'b_2100bc;
}
                                                                                            }
                                                                                            'b_2100d4: {
                                                                                                c.cycles += 4;
                                                                                                c.lwc1(2, (c.g[T1][0] as u32).wrapping_add(0x10));
                                                                                                c.lwc1(3, (c.g[T1][0] as u32).wrapping_add(0x14));
                                                                                                let t = c.g[S3][0] != 0u64;
                                                                                                c.mov_s(0, 2);
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_2100d4;
                                                                                                } else {
                                                                                                    c.cycles += 2;
                                                                                                    c.mov_s(2, 3);
                                                                                                    c.mov_s(3, 0);
                                                                                                    break 'b_2100d4;
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 9;
                                                                                            c.g[AT][0] = 0x43000000;
                                                                                            c.mtc1(
                                                                                                1,
                                                                                                0,
                                                                                            );
                                                                                            c.g[T2][0] = sx((c.g[A0][0] as u32) << 5);
                                                                                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                                                            c.g[A0][0] = 0x3;
                                                                                            c.mul_s(0, 1, 0);
                                                                                            c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xac));
                                                                                            c.cvt_w_s(1, 0);
                                                                                            c.mfc1(
                                                                                                5,
                                                                                                1,
                                                                                            );
                                                                                            'l_2100f8: loop {
                                                                                                c.cycles += 7;
                                                                                                c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                                                                                                let t = (c.g[A0][0] as i64) >= 0;
                                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
                                                                                                if t
                                                                                                {
                                                                                                    if event_due(c) { deliver_event_here(c, 0x002100f8); }
                                                                                                    continue 'l_2100f8;
                                                                                                } else {
                                                                                                    'b_2101d4: {
                                                                                                        'b_210190: {
                                                                                                            c.cycles += 13;
                                                                                                            c.swc1(2, (c.g[SP][0] as u32));
                                                                                                            c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
                                                                                                            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                                                                            c.lwc1(1, (c.g[T1][0] as u32).wrapping_add(0xc));
                                                                                                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                                                                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
                                                                                                            c.lwc1(2, (c.g[T1][0] as u32).wrapping_add(0x18));
                                                                                                            c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x90));
                                                                                                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x64));
                                                                                                            c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x18));
                                                                                                            let t = c.g[S3][0] == 0u64;
                                                                                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
                                                                                                            if t {
break 'b_210190;
} else {
c.cycles += 18;
c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.add_s(0, 21, 0);
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xc4));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xcc));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc4));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
let t = true;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
if t {
break 'b_2101d4;
} else {
break 'b_210190;
}
}
                                                                                                        }
                                                                                                        c.cycles += 17;
                                                                                                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
                                                                                                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
                                                                                                        c.sub_s(0, 21, 0);
                                                                                                        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xc4));
                                                                                                        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xcc));
                                                                                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
                                                                                                        c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                                        c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
                                                                                                        c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc4));
                                                                                                        c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                                                                        c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                        break 'b_2101d4;
                                                                                                    }
                                                                                                    c.cycles += 23;
                                                                                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
                                                                                                    c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x80));
                                                                                                    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                                                                    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                                                                                    c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                                                                                    c.g[V1][0] = 0xfffffffffffffc1f;
                                                                                                    c.g[A1][0] = 0x4;
                                                                                                    c.g[A2][0] = c.g[SP][0];
                                                                                                    c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
                                                                                                    c.g[A3][0] = 0u64;
                                                                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                                                                                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                                                                                    c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                                                                                                    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                                                                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
                                                                                                    c.g[V1][0] = c.lw((c.g[T1][0] as u32));
                                                                                                    c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                                                                                                    c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                                                                                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
                                                                                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
                                                                                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
                                                                                                    let t = c.g[V1][0] as u32;
                                                                                                    c.g[RA][0] = sx(0x00210230);
                                                                                                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                                    call_ptr(c, t, 0x00210230)?;
                                                                                                    break 'b_210230;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 1;
                                                                        c.g[S0][0] = c.g[S1][0];
                                                                        break 'b_210234;
                                                                    }
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = ((c.g[S0][0]
                                                                        as i64)
                                                                        < (c.g[S4][0] as i64))
                                                                        as u64;
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.g[V1][0] = 0x4d0000;
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x0020ffa0,
                                                                            );
                                                                        }
                                                                        continue 'l_20ffa0;
                                                                    } else {
                                                                        break 'b_210240;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[A0][0] = 0x80;
                                                        c.g[V1][0] = 0x3;
                                                        c.g[V0][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xac));
                                                        'l_210250: loop {
                                                            c.cycles += 7;
                                                            c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            let t = (c.g[V1][0] as i64) >= 0;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x30));
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x00210250,
                                                                    );
                                                                }
                                                                continue 'l_210250;
                                                            } else {
                                                                'b_21052c: {
                                                                    c.cycles += 3;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[S7][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    let t = c.g[A1][0] == 0u64;
                                                                    c.g[V1][0] = 0x530000;
                                                                    if t {
                                                                        break 'b_21052c;
                                                                    } else {
                                                                        'b_21029c: {
                                                                            c.cycles += 5;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x8d0,
                                                                                ));
                                                                            c.g[V0][0] = sx(((c.g
                                                                                [V0][0]
                                                                                as u32)
                                                                                as i32
                                                                                >> 9)
                                                                                as u32);
                                                                            c.g[V0][0] =
                                                                                c.g[V0][0] & 0x1;
                                                                            let t =
                                                                                c.g[V0][0] == 0u64;
                                                                            c.g[S1][0] = 0x1;
                                                                            if t {
                                                                                break 'b_21029c;
                                                                            } else {
                                                                                c.cycles += 4;
                                                                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x88));
                                                                                if c.g[A0][0] != 0 {
                                                                                    c.g[S1][0] =
                                                                                        0u64;
                                                                                }
                                                                                break 'b_21029c;
                                                                            }
                                                                        }
                                                                        c.cycles += 2;
                                                                        let t = (c.g[A1][0] as i64)
                                                                            <= 0;
                                                                        c.g[A0][0] = 0u64;
                                                                        if t {
                                                                            break 'b_21052c;
                                                                        } else {
                                                                            c.cycles += 9;
                                                                            c.g[V0][0] = 0x4d0000;
                                                                            c.g[AT][0] = 0x3f000000;
                                                                            c.mtc1(1, 22);
                                                                            c.lwc1(
                                                                                28,
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0x54b0,
                                                                                    ),
                                                                            );
                                                                            c.g[S4][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x7358,
                                                                                    ));
                                                                            c.g[AT][0] = 0x3f800000;
                                                                            c.mtc1(1, 20);
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [SP][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0xd0,
                                                                                ));
                                                                            'l_2102c8: loop {
                                                                                'b_21051c: {
                                                                                    'b_210518: {
                                                                                        c.cycles +=
                                                                                            7;
                                                                                        c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
                                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                        c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                                        c.c_lt_s(
                                                                                            1, 26,
                                                                                        );
                                                                                        let t =
                                                                                            c.fcc();
                                                                                        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                                                                        if t {
                                                                                            break 'b_210518;
                                                                                        } else {
                                                                                            c.cycles += 4;
                                                                                            c.c_lt_s(29, 1);
                                                                                            if c.fcc() {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_21051c;
} else {
'b_210310: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[V0][0] as u32));
c.g[V0][0] = 0x1;
if c.g[V1][0] != c.g[V0][0] {
c.sub_s(1, 1, 26);
break 'b_210310;
} else {
c.cycles += 2;
if c.g[S1][0] == 0u64 {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_21051c;
} else {
c.cycles += 1;
c.sub_s(1, 1, 26);
break 'b_210310;
}
}
}
c.cycles += 16;
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x30));
c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
c.mul_s(1, 1, 0);
c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
c.g[T1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
c.mul_s(7, 2, 22);
c.mul_s(0, 0, 22);
c.sub_s(1, 23, 1);
c.sub_s(5, 1, 0);
c.c_le_s(5, 27);
let t = c.fcc();
c.add_s(6, 1, 0);
if t {
break 'b_210518;
} else {
c.cycles += 4;
c.c_le_s(23, 6);
if c.fcc() {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_21051c;
} else {
'b_210394: {
'b_21038c: {
'b_21037c: {
c.cycles += 5;
c.c_lt_s(5, 24);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 2);
let t = !c.fcc();

if t {
break 'b_21037c;
} else {
c.cycles += 2;
let t = true;
c.sub_s(0, 24, 5);
if t {
break 'b_21038c;
} else {
break 'b_21037c;
}
}
}
c.cycles += 3;
c.c_lt_s(25, 6);
let t = !c.fcc();

if t {
break 'b_210394;
} else {
c.cycles += 1;
c.sub_s(0, 6, 25);
break 'b_21038c;
}
}
c.cycles += 2;
c.mul_s(0, 0, 28);
c.sub_s(2, 2, 0);
break 'b_210394;
}
'b_2103a8: {
c.cycles += 5;
c.mtc1(0, 0);
c.c_lt_s(2, 0);
if c.fcc() {
c.mov_s(2, 0);
break 'b_2103a8;
} else {
break 'b_2103a8;
}
}
'b_2103c8: {
'b_2103c0: {
c.cycles += 3;
c.lwc1(3, (c.g[T1][0] as u32).wrapping_add(0x10));
let t = c.g[S3][0] == 0u64;
c.lwc1(4, (c.g[T1][0] as u32).wrapping_add(0x14));
if t {
break 'b_2103c0;
} else {
c.cycles += 3;
c.add_s(0, 3, 4);
let t = true;
c.mul_s(3, 0, 22);
if t {
break 'b_2103c8;
} else {
break 'b_2103c0;
}
}
}
c.cycles += 2;
c.add_s(0, 3, 4);
c.mul_s(4, 0, 22);
break 'b_2103c8;
}
c.cycles += 20;
c.swc1(3, (c.g[SP][0] as u32));
c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.g[AT][0] = 0x43000000;
c.mtc1(1, 0);
c.g[V1][0] = 0x3;
c.lwc1(1, (c.g[T1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xac));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mul_s(0, 2, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.lwc1(2, (c.g[T1][0] as u32).wrapping_add(0xc));
c.cvt_w_s(1, 0);
c.mfc1(4, 1);
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x60));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
c.lwc1(1, (c.g[T1][0] as u32).wrapping_add(0x18));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x90));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x64));
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x18));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
'l_210418: loop {
c.cycles += 7;
c.sw((c.g[V0][0] as u32), c.g[A0][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
let t = (c.g[V1][0] as i64) >= 0;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x30));
if t {
if event_due(c) { deliver_event_here(c, 0x00210418); } continue 'l_210418;
} else {
'b_2104bc: {
'b_210480: {
c.cycles += 2;
if c.g[S3][0] == 0u64 {
c.sub_s(0, 21, 7);
break 'b_210480;
} else {
c.cycles += 17;
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.add_s(0, 21, 7);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc4));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xc4));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
let t = true;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
if t {
break 'b_2104bc;
} else {
break 'b_210480;
}
}
}
c.cycles += 15;
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xc4));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xcc));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xc4));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc0));
break 'b_2104bc;
}
c.cycles += 23;
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V1][0] = 0xfffffffffffffc1f;
c.g[A1][0] = 0x4;
c.g[A2][0] = c.g[SP][0];
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[A3][0] = 0u64;
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | 0x160;
c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = c.lw((c.g[T1][0] as u32));
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x00210518);
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x00210518)?;
break 'b_210518;
}
}
}
}
}
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 1;
                                                                                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
                                                                                    break 'b_21051c;
                                                                                }
                                                                                c.cycles += 4;
                                                                                c.g[A0][0] =
                                                                                    c.g[S0][0];
                                                                                c.g[V0][0] = ((c.g
                                                                                    [A0][0]
                                                                                    as i64)
                                                                                    < (c.g[V0][0]
                                                                                        as i64))
                                                                                    as u64;
                                                                                let t = c.g[V0][0]
                                                                                    != 0u64;
                                                                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xd0));
                                                                                if t {
                                                                                    if event_due(c)
                                                                                    {
                                                                                        deliver_event_here(c, 0x002102c8);
                                                                                    }
                                                                                    continue 'l_2102c8;
                                                                                } else {
                                                                                    break 'b_21052c;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 10;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x854));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0xe84));
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x10d8));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x14));
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0xe84),
                                                                    c.g[V0][0],
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lh((c.g[A1][0] as u32)
                                                                        .wrapping_add(0x100));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A1][0] as u32)
                                                                        .wrapping_add(0x104));
                                                                let t = c.g[V0][0] as u32;
                                                                c.g[RA][0] = sx(0x00210550);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V1][0] as u32),
                                                                    ));
                                                                call_ptr(c, t, 0x00210550)?;
                                                                c.sw(
                                                                    (c.g[S2][0] as u32)
                                                                        .wrapping_add(0x34),
                                                                    c.g[S5][0],
                                                                );
                                                                break 'b_210554;
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
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
            break 'b_210558;
        }
        c.cycles += 21;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lwc1(29, (c.g[SP][0] as u32).wrapping_add(0x1c8));
        c.lwc1(28, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lwc1(27, (c.g[SP][0] as u32).wrapping_add(0x1b8));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x1a8));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x198));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x188));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x180));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{check_reaching, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{elf_image, DATA, GAME_GP};

    fn oc981_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// Fill a row of random floats at `a` (`n` words, range +-`range`).
    fn oc981_floats(c: &mut Cpu, r: &mut crate::difftest::Rng, a: u32, n: u32, range: f32) {
        for k in 0..n {
            let v = r.f32(range);
            c.bus.write32(a + 4 * k, v);
        }
    }

    /// `cOVStateHUD1P::DrawProgressMeterMP`: early outs (HUD off, no radar
    /// header, no rider, no course), own rider index 0 (the real leaf at
    /// 0x1406c0) or 1..5 (forced fake result), racers of both kinds (human
    /// or AI through the vtable at +0x44: fake or the leaf), 0..6 racers
    /// near and far from the meter range, 0..6 course markers of types 0..3
    /// (type 1 hidden in timed events), both sides, the fade at either end
    /// and the clipped icons. The renderer's vtable slots are fakes, the draw
    /// state stack and the rate-limit table are real memory.
    #[test]
    fn c_ov_state_hud1_p_draw_progress_meter_mp_matches() {
        check_reaching(
            ee::s05::f_0020fb40__generated,
            c_ov_state_hud1_p_draw_progress_meter_mp,
            300,
            &[
                0x0020_fbe0,
                0x0020_fd6c,
                0x0020_fe74,
                0x0020_ff7c,
                0x0021_0014,
                0x0021_0230,
                0x0021_0518,
                0x0021_0550,
            ],
            &|c, seed| {
                let mut r = machine(c, seed, 0x20fb40, 24);
                elf_image(c);
                c.pc = 0x0020_fb40;
                let gp = GAME_GP;
                let this = DATA + 0x9000;
                let rider = DATA + 0x1000;
                let world = DATA + 0x100;
                let race = DATA + 0x200;
                let list = DATA + 0x300;
                let my_vt = DATA + 0x8800;
                let course = DATA + 0xb800;
                let markers = DATA + 0xb000;
                let renderer = DATA + 0xc000;
                let rvt = DATA + 0xd000;
                let table = 0x004c_8bc8u32;
                let icons = 0x004c_8c58u32;
                // Early-out flags.
                oc981_w(c, gp - 0x990, (seed % 11 == 0) as u32);
                oc981_w(c, gp - 0x994, (seed % 13 != 0) as u32);
                oc981_w(c, gp - 0x998, markers);
                oc981_w(c, gp - 0x9a0, if seed % 19 == 0 { 0 } else { course });
                oc981_w(c, gp - 0x848, world);
                oc981_w(c, gp - 0x854, renderer);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 17 == 0 { 0 } else { sx(rider) };
                c.g[A2][0] = (seed % 2) as u64;
                // The player's rider: getAIIndex (vtable +0x38/+0x3c).
                oc981_w(c, rider + 0x6c0, my_vt);
                c.bus
                    .write16(my_vt + 0x38, ((r.next_u32() % 16) * 4) as u16);
                let me = if seed % 7 == 0 { 0 } else { 1 + seed % 5 };
                if me == 0 {
                    oc981_w(c, my_vt + 0x3c, 0x0014_06c0);
                } else {
                    oc981_w(c, my_vt + 0x3c, FAKE);
                    oc981_w(c, LOG_RESULTS, me);
                }
                // The world: race object, rider list, racer count.
                let n = r.next_u32() % 7;
                oc981_w(c, world + 0x84, race);
                oc981_w(c, race + 0xc, list);
                oc981_w(c, list + 0x78, n);
                oc981_w(c, world + 0xc0, DATA + 0x400);
                oc981_w(c, DATA + 0x400 + 0x88, (r.next_u32() % 2) as u32);
                oc981_w(
                    c,
                    0x0053_08d0,
                    if r.next_u32() % 3 == 0 { 0 } else { 1 << 9 },
                );
                for i in 0..6u32 {
                    let rd = DATA + 0x2000 + 0x800 * i;
                    let vt = DATA + 0x8000 + 0x100 * i;
                    oc981_w(c, list + 0x28 + 4 * i, rd);
                    oc981_w(c, rd + 0x6c0, vt);
                    c.bus.write16(vt + 0x40, ((r.next_u32() % 16) * 4) as u16);
                    // isHuman: pointer (fake) or 0 (the leaf).
                    let f = if r.next_u32() % 2 == 0 {
                        FAKE + 0x10
                    } else {
                        0x0014_06c0
                    };
                    oc981_w(c, vt + 0x44, f);
                }
                // The meter's geometry and last rider index.
                let top = 50.0 + (r.next_u32() % 100) as f32;
                let height = 150.0 + (r.next_u32() % 100) as f32;
                oc981_w(c, this + 0x20, ((r.next_u32() % 400) as f32).to_bits());
                oc981_w(c, this + 0x24, top.to_bits());
                oc981_w(
                    c,
                    this + 0x28,
                    (20.0f32 + (r.next_u32() % 20) as f32).to_bits(),
                );
                oc981_w(c, this + 0x2c, height.to_bits());
                let scale = height / 10000.0 * (0.7 + (r.next_u32() % 7) as f32 * 0.1);
                oc981_w(c, this + 0x30, scale.to_bits());
                oc981_w(c, this + 0x34, if r.next_u32() % 3 == 0 { me } else { 9 });
                // Racer table: dist +8, pos +0xc, rel +0x10, relValid +0x14.
                let my_dist = 3000.0f32 + (r.next_u32() % 4000) as f32;
                let my_pos = (r.next_u32() % 4000) as f32;
                for i in 0..6u32 {
                    let row = table + 0x18 * i;
                    let (dist, pos) = if i == me {
                        (my_dist, my_pos)
                    } else {
                        let a = r.f32(6000.0);
                        let b = r.f32(3000.0);
                        (my_dist + f32::from_bits(a), my_pos + f32::from_bits(b))
                    };
                    oc981_w(c, row + 8, dist.to_bits());
                    oc981_w(c, row + 0xc, pos.to_bits());
                    oc981_floats(c, &mut r, row + 0x10, 1, 2500.0);
                    oc981_w(c, row + 0x14, (r.next_u32() % 2) as u32);
                }
                // Icon table: w +4, h +8, texture +0xc (handle, then uv rect).
                for k in 0..11u32 {
                    let row = icons + 0x10 * k;
                    let tex = DATA + 0xa000 + 0x40 * k;
                    oc981_w(c, row + 4, (4.0f32 + (r.next_u32() % 20) as f32).to_bits());
                    oc981_w(c, row + 8, (4.0f32 + (r.next_u32() % 20) as f32).to_bits());
                    oc981_w(c, row + 0xc, tex);
                    oc981_w(c, tex, r.next_u32() % 64);
                    oc981_floats(c, &mut r, tex + 4, 7, 64.0);
                }
                // Course markers: type +0, dist +4.
                oc981_w(c, course + 8, r.next_u32() % 7);
                for i in 0..6u32 {
                    let m = markers + 8 * i;
                    oc981_w(c, m, r.next_u32() % 4);
                    let d = my_dist + f32::from_bits(r.f32(6000.0));
                    oc981_w(c, m + 4, d.to_bits());
                }
                // The renderer: draw-state stack and the vtable of fakes.
                oc981_w(c, renderer + 0xe84, DATA + 0xc800);
                oc981_w(c, DATA + 0xc800 + 4, r.next_u32());
                oc981_w(c, DATA + 0xc800 + 8, r.next_u32());
                oc981_w(c, renderer + 0x10d8, rvt);
                for (i, off) in [0xf8u32, 0x110, 0x238, 0x100].into_iter().enumerate() {
                    c.bus.write16(rvt + off, ((r.next_u32() % 16) * 4) as u16);
                    oc981_w(c, rvt + off + 4, FAKE + 0x20 + 0x10 * i as u32);
                }
            },
        );
    }
}
