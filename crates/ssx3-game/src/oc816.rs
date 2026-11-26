//! Staging module oc816 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc816`.

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
    &[(0x0017_c5f8, debug_menu_render, "DebugMenu_Render")];

/// `void DebugMenu_Render(cDebugMenuScreen *self)` (0x17c5f8): draws the debug
/// front-end menu: the background image (unless the active image is negative),
/// then rows of text with the current row highlighted, the event, path, mode,
/// location, rider and controller names, audio and network info. A converted
/// labelled-block body: every register, cycle and call kept. It has no resume
/// entries, so no `__entries` fallback.
pub fn debug_menu_render(c: &mut Cpu) {
    guest(c, |c| {
        'b_17cfcc: {
            'b_17c65c: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1d0));
                c.g[V0][0] = 0x8c10;
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.g[S6][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1c8));
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x1b8));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                let t = (c.g[V1][0] as i64) < 0;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_17c65c;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0017c654);
                    Callee {
                        addr: 0x0017ae00,
                        f: ee::s02::DebugMenu_DrawBackground,
                    }
                    .call(c, 0x0017c654)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                    if t {
                        break 'b_17cfcc;
                    } else {
                        break 'b_17c65c;
                    }
                }
            }
            'b_17c95c: {
                'b_17c920: {
                    c.cycles += 158;
                    c.g[S2][0] = 0x4a0000;
                    c.g[AT][0] = 0x42700000;
                    c.mtc1(1, 20);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x12c0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x80));
                    c.g[AT][0] = 0x41a00000;
                    c.mtc1(1, 22);
                    c.g[RA][0] = sx(0x0017c680);
                    c.g[S4][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0014dc80, 0x0017c680)?;
                    c.g[AT][0] = 0x44110000;
                    c.mtc1(1, 21);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0017c694);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x0017c694)?;
                    c.g[RA][0] = sx(0x0017c69c);
                    c.g[S5][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x0017c69c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0017c6a8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0014dd58, 0x0017c6a8)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[RA][0] = sx(0x0017c6bc);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, 0x0014dd58, 0x0017c6bc)?;
                    c.lwc1(4, (c.g[S4][0] as u32).wrapping_add(0x34));
                    c.lwc1(3, (c.g[S4][0] as u32).wrapping_add(0x14));
                    c.cvt_s_w(3, 3);
                    c.lwc1(5, (c.g[S4][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = 0x8000;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5b1c));
                    c.g[S1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 1);
                    c.g[FP][0] = c.g[V0][0];
                    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x5b18));
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x38));
                    c.g[V1][0] = c.g[SP][0];
                    c.swc1(4, (c.g[S4][0] as u32).wrapping_add(0x3c));
                    c.g[A1][0] = 0x460000;
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x1c));
                    c.mul_s(23, 3, 4);
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.mov_s(13, 22);
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x2c));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3140));
                    c.swc1(1, c.g[SP][0] as u32);
                    c.mov_s(12, 20);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x24));
                    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017c76c);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00391cb0, 0x0017c76c)?;
                    c.g[RA][0] = sx(0x0017c774);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00145378, 0x0017c774)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.mov_s(13, 22);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017c788);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x00392a60, 0x0017c788)?;
                    c.add_s(22, 23, 22);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = c.g[SP][0];
                    c.g[A1][0] = 0x460000;
                    c.mov_s(12, 20);
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    c.mov_s(13, 22);
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3130));
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017c7d4);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00391cb0, 0x0017c7d4)?;
                    c.g[RA][0] = sx(0x0017c7dc);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00145398, 0x0017c7dc)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.mov_s(13, 22);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017c7f0);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x00392a60, 0x0017c7f0)?;
                    c.add_s(22, 22, 23);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = c.g[SP][0];
                    c.g[A1][0] = 0x460000;
                    c.mov_s(12, 20);
                    c.g[V0][0] = c.g[V0][0] ^ 0x2;
                    c.mov_s(13, 22);
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[S0][0];
                    }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3120));
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017c83c);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00391cb0, 0x0017c83c)?;
                    c.g[RA][0] = sx(0x0017c844);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00145340, 0x0017c844)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.mov_s(13, 22);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017c858);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x00392a60, 0x0017c858)?;
                    c.add_s(22, 22, 23);
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                    c.g[A1][0] = 0x460000;
                    c.mov_s(12, 20);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3110));
                    c.g[V0][0] = c.g[V0][0] ^ 0x3;
                    c.mov_s(13, 22);
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[SP][0];
                    }
                    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[S0][0] as u32);
                    c.ldl(3, (c.g[S0][0] as u32).wrapping_add(0xf));
                    c.ldr(3, (c.g[S0][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(3, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(3, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017c8a0);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00391cb0, 0x0017c8a0)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x0017c8ac);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00145c38, 0x0017c8ac)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.mov_s(13, 22);
                    c.mov_s(12, 21);
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017c8c8);
                    c.add_s(22, 22, 23);
                    call_ptr(c, 0x001dca80, 0x0017c8c8)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f38));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = 0x460000;
                    if t {
                        break 'b_17c920;
                    } else {
                        c.cycles += 19;
                        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x30f8));
                        c.g[RA][0] = sx(0x0017c8e4);
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x00144bc0, 0x0017c8e4)?;
                        c.g[S7][0] = c.g[S3][0];
                        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                        c.g[RA][0] = sx(0x0017c8f4);
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x00144bc0, 0x0017c8f4)?;
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x44));
                        c.g[RA][0] = sx(0x0017c900);
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x00144bc0, 0x0017c900)?;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S2][0];
                        c.g[A3][0] = c.g[S1][0];
                        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
                        c.g[RA][0] = sx(0x0017c918);
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x00417828, 0x0017c918)?;
                        let t = true;
                        c.g[S0][0] = 0x8000;
                        if t {
                            break 'b_17c95c;
                        } else {
                            break 'b_17c920;
                        }
                    }
                }
                c.cycles += 15;
                c.g[S0][0] = 0x4a0000;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x12c8));
                c.g[RA][0] = sx(0x0017c934);
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x00144bc0, 0x0017c934)?;
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
                c.g[RA][0] = sx(0x0017c940);
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x00144bc0, 0x0017c940)?;
                c.g[S7][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x34));
                c.g[RA][0] = sx(0x0017c958);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00417828, 0x0017c958)?;
                c.g[S0][0] = 0x8000;
                break 'b_17c95c;
            }
            'b_17cc74: {
                'b_17cc6c: {
                    c.cycles += 194;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[AT][0] = 0x42700000;
                    c.mtc1(1, 20);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[A1][0] = 0x4a0000;
                    c.g[AT][0] = 0x44110000;
                    c.mtc1(1, 21);
                    c.mov_s(13, 22);
                    c.g[V0][0] = c.g[V0][0] ^ 0x4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12d0));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.g[A0][0] = c.g[S4][0];
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(8, c.g[V1][0] as u32);
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(2, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(8, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(8, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.mov_s(12, 20);
                    c.g[T0][0] = 0x4a0000;
                    c.g[S1][0] = 0x4a0000;
                    c.g[S2][0] = sx((c.g[T0][0] as u32).wrapping_add(0x12c0));
                    c.g[RA][0] = sx(0x0017c9c8);
                    c.g[S3][0] = c.g[S1][0];
                    call_ptr(c, 0x00391cb0, 0x0017c9c8)?;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x12f0));
                    c.g[A1][0] = c.g[S7][0];
                    c.mov_s(13, 22);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017c9e0);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x00392a60, 0x0017c9e0)?;
                    c.add_s(22, 22, 23);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x460000;
                    c.mov_s(12, 20);
                    c.g[V0][0] = c.g[V0][0] ^ 0x5;
                    c.mov_s(13, 22);
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30e8));
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017ca2c);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00391cb0, 0x0017ca2c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x104));
                    c.g[RA][0] = sx(0x0017ca38);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014ee28, 0x0017ca38)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.mov_s(13, 22);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017ca4c);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x00392a60, 0x0017ca4c)?;
                    c.add_s(22, 22, 23);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x460000;
                    c.mov_s(12, 20);
                    c.g[V0][0] = c.g[V0][0] ^ 0x6;
                    c.mov_s(13, 22);
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30d8));
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017ca98);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00391cb0, 0x0017ca98)?;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x0017caa4);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00147618, 0x0017caa4)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.mov_s(13, 22);
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.mov_s(12, 21);
                    c.g[RA][0] = sx(0x0017cac0);
                    c.add_s(22, 22, 23);
                    call_ptr(c, 0x001dca80, 0x0017cac0)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x4a0000;
                    c.mov_s(13, 22);
                    c.g[V0][0] = c.g[V0][0] ^ 0x7;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12d8));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.g[A0][0] = c.g[S4][0];
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017cb08);
                    c.mov_s(12, 20);
                    call_ptr(c, 0x00391cb0, 0x0017cb08)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc24));
                    c.g[V1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0x1e10));
                    c.mov_s(13, 22);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.mov_s(12, 21);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.add_s(22, 22, 23);
                    c.g[RA][0] = sx(0x0017cb30);
                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                    call_ptr(c, 0x00392a60, 0x0017cb30)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x460000;
                    c.mov_s(13, 22);
                    c.g[V0][0] = c.g[V0][0] ^ 0x8;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30c8));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.g[A0][0] = c.g[S4][0];
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017cb78);
                    c.mov_s(12, 20);
                    call_ptr(c, 0x00391cb0, 0x0017cb78)?;
                    c.g[V0][0] = 0x530000;
                    c.mov_s(13, 22);
                    c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5c04));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017cb94);
                    c.mov_s(12, 21);
                    call_ptr(c, 0x001dca80, 0x0017cb94)?;
                    c.add_s(22, 22, 23);
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x460000;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V0][0] = c.g[V0][0] ^ 0x9;
                    c.mov_s(13, 22);
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30b0));
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017cbe0);
                    c.mov_s(12, 20);
                    call_ptr(c, 0x00391cb0, 0x0017cbe0)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc30));
                    c.g[A1][0] = c.g[S2][0];
                    c.mov_s(13, 22);
                    c.mov_s(12, 21);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x0017cbfc);
                    c.add_s(22, 22, 23);
                    call_ptr(c, 0x001dca80, 0x0017cbfc)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V0][0] = c.g[V0][0] ^ 0xa;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12e8));
                    if c.g[V0][0] == 0 {
                        c.g[V1][0] = c.g[SP][0];
                    }
                    c.mov_s(12, 20);
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017cc44);
                    c.mov_s(13, 22);
                    call_ptr(c, 0x00391cb0, 0x0017cc44)?;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0017cc5c);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x0017cc5c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        break 'b_17cc6c;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3098));
                        if t {
                            break 'b_17cc74;
                        } else {
                            break 'b_17cc6c;
                        }
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = 0x460000;
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3088));
                break 'b_17cc74;
            }
            'b_17cd34: {
                c.cycles += 46;
                c.g[AT][0] = 0x44110000;
                c.mtc1(1, 12);
                c.g[A1][0] = c.g[S1][0];
                c.mov_s(13, 22);
                c.g[RA][0] = sx(0x0017cc8c);
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x001dca80, 0x0017cc8c)?;
                c.add_s(22, 22, 23);
                c.g[V0][0] = 0x8000;
                c.g[AT][0] = 0x42700000;
                c.mtc1(1, 12);
                c.g[S1][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A1][0] = 0x460000;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.mov_s(13, 22);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3078));
                c.g[V1][0] = c.g[V1][0] ^ 0xb;
                c.g[A0][0] = c.g[S4][0];
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = c.g[SP][0];
                }
                c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
                c.ldr(3, c.g[V0][0] as u32);
                c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0xf));
                c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.sdl(3, (c.g[S4][0] as u32).wrapping_add(0x47));
                c.sdr(3, (c.g[S4][0] as u32).wrapping_add(0x40));
                c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                c.g[RA][0] = sx(0x0017cce8);
                c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x12f0));
                call_ptr(c, 0x00391cb0, 0x0017cce8)?;
                c.g[RA][0] = sx(0x0017ccf0);
                call_ptr(c, 0x0028b180, 0x0017ccf0)?;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc34));
                c.g[V1][0] = 0x14;
                c.g[AT][0] = 0x44110000;
                c.mtc1(1, 12);
                c.mov_s(13, 22);
                c.mult(0, 6, 6, 3);
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S0][0];
                c.add_s(22, 22, 23);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x60a4));
                c.g[RA][0] = sx(0x0017cd20);
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                call_ptr(c, 0x001dca80, 0x0017cd20)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                let t = c.g[A0][0] == 0u64;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x100), 0u64);
                if t {
                    break 'b_17cd34;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0017cd34);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    call_ptr(c, 0x00256038, 0x0017cd34)?;
                    break 'b_17cd34;
                }
            }
            'b_17ce04: {
                'b_17cdd0: {
                    c.cycles += 25;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.g[A3][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x102));
                    c.g[A1][0] = 0x460000;
                    c.g[T0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x101));
                    c.g[T1][0] = c.g[A2][0] & 0xff;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3068));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0017cd5c);
                    c.g[A2][0] = sx((c.g[A2][0] as u32) >> 24);
                    call_ptr(c, 0x00417828, 0x0017cd5c)?;
                    c.g[V0][0] = 0x460000;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3048));
                    c.ldl(4, (c.g[T0][0] as u32).wrapping_add(0x7));
                    c.ldr(4, c.g[T0][0] as u32);
                    c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
                    c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xd7));
                    c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0xdf));
                    c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0xd8));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[A3][0]);
                    let t = c.g[V1][0] == 0u64;
                    c.g[A2][0] = c.g[S0][0];
                    if t {
                        break 'b_17cdd0;
                    } else {
                        c.cycles += 14;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = c.g[V0][0] ^ 0xc;
                        if c.g[V0][0] == 0 {
                            c.g[V1][0] = c.g[SP][0];
                        }
                        c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                        c.ldr(2, c.g[V1][0] as u32);
                        c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
                        c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                        c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
                        c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
                        c.sdl(4, (c.g[S4][0] as u32).wrapping_add(0x4f));
                        c.sdr(4, (c.g[S4][0] as u32).wrapping_add(0x48));
                        let t = true;
                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_17ce04;
                        } else {
                            break 'b_17cdd0;
                        }
                    }
                }
                c.cycles += 13;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc1c));
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[V1][0];
                c.g[V0][0] = c.g[V0][0] ^ 0xc;
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = c.g[SP][0];
                }
                c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(8, c.g[V1][0] as u32);
                c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(2, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(8, (c.g[S4][0] as u32).wrapping_add(0x47));
                c.sdr(8, (c.g[S4][0] as u32).wrapping_add(0x40));
                c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x4f));
                c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x48));
                break 'b_17ce04;
            }
            'b_17cf60: {
                c.cycles += 66;
                c.g[AT][0] = 0x42700000;
                c.mtc1(1, 21);
                c.g[A1][0] = 0x460000;
                c.mov_s(13, 22);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3030));
                c.g[A0][0] = c.g[S4][0];
                c.g[RA][0] = sx(0x0017ce24);
                c.mov_s(12, 21);
                call_ptr(c, 0x001dc990, 0x0017ce24)?;
                c.g[AT][0] = 0x43a00000;
                c.mtc1(1, 20);
                c.g[S0][0] = 0x8000;
                c.g[AT][0] = 0x44110000;
                c.mtc1(1, 12);
                c.g[S0][0] = sx((c.g[S6][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x12f0));
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc44));
                c.mov_s(13, 22);
                c.g[A0][0] = c.g[S4][0];
                c.add_s(22, 22, 23);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[RA][0] = sx(0x0017ce60);
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                call_ptr(c, 0x001dca80, 0x0017ce60)?;
                c.g[AT][0] = 0x40a00000;
                c.mtc1(1, 0);
                c.g[V1][0] = c.g[SP][0];
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                c.g[A1][0] = 0x4a0000;
                c.add_s(22, 22, 0);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1318));
                c.g[V0][0] = c.g[V0][0] ^ 0xd;
                c.g[A0][0] = c.g[S4][0];
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.g[S1][0];
                }
                c.mov_s(12, 20);
                c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(6, c.g[V1][0] as u32);
                c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x47));
                c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x40));
                c.sdl(7, (c.g[S4][0] as u32).wrapping_add(0x4f));
                c.sdr(7, (c.g[S4][0] as u32).wrapping_add(0x48));
                c.g[RA][0] = sx(0x0017ceb4);
                c.mov_s(13, 22);
                call_ptr(c, 0x001dca40, 0x0017ceb4)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc1c));
                c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.add_s(13, 22, 23);
                c.g[A1][0] = 0x460000;
                c.g[V0][0] = c.g[V0][0] ^ 0xe;
                c.g[A0][0] = c.g[S4][0];
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = c.g[SP][0];
                }
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3020));
                c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.ldr(6, c.g[V1][0] as u32);
                c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x47));
                c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x40));
                c.sdl(7, (c.g[S4][0] as u32).wrapping_add(0x4f));
                c.sdr(7, (c.g[S4][0] as u32).wrapping_add(0x48));
                c.g[RA][0] = sx(0x0017cefc);
                c.mov_s(12, 20);
                call_ptr(c, 0x001dca40, 0x0017cefc)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                let t = c.g[V1][0] != 0u64;
                c.g[A1][0] = 0x460000;
                if t {
                    break 'b_17cf60;
                } else {
                    c.cycles += 21;
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x5b14));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 1);
                    c.mov_s(12, 20);
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3010));
                    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf4));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xfc), 0u64);
                    c.g[AT][0] = 0x43d20000;
                    c.mtc1(1, 13);
                    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf7));
                    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.ldl(6, (c.g[SP][0] as u32).wrapping_add(0xff));
                    c.ldr(6, (c.g[SP][0] as u32).wrapping_add(0xf8));
                    c.sdl(3, (c.g[S4][0] as u32).wrapping_add(0x47));
                    c.sdr(3, (c.g[S4][0] as u32).wrapping_add(0x40));
                    c.sdl(6, (c.g[S4][0] as u32).wrapping_add(0x4f));
                    c.sdr(6, (c.g[S4][0] as u32).wrapping_add(0x48));
                    c.g[RA][0] = sx(0x0017cf60);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x00392908, 0x0017cf60)?;
                    break 'b_17cf60;
                }
            }
            c.cycles += 27;
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 0);
            c.g[A1][0] = 0x460000;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 1);
            c.g[A0][0] = c.g[S4][0];
            c.mov_s(14, 0);
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf4));
            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.mov_s(12, 21);
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xf8));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2ff8));
            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xfc));
            c.mov_s(15, 14);
            c.g[AT][0] = 0x43d20000;
            c.mtc1(1, 13);
            c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xf7));
            c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xff));
            c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0xf8));
            c.sdl(2, (c.g[S4][0] as u32).wrapping_add(0x47));
            c.sdr(2, (c.g[S4][0] as u32).wrapping_add(0x40));
            c.sdl(3, (c.g[S4][0] as u32).wrapping_add(0x4f));
            c.sdr(3, (c.g[S4][0] as u32).wrapping_add(0x48));
            c.g[RA][0] = sx(0x0017cfc8);
            c.g[A2][0] = 0x7;
            call_ptr(c, 0x001dc9d0, 0x0017cfc8)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1a0));
            break 'b_17cfcc;
        }
        c.cycles += 15;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1c8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x1b8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Fixture objects, in DATA+0x200000 and up (clear of the fake log at
    /// DATA+0x100000 and of the pointers the fakes return).
    pub(crate) const OC816_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC816_MAGIC: u32 = 0x4f43_3831;
    pub(crate) const OC816_SELF: u32 = DATA + 0x21_0000;
    pub(crate) const OC816_WORLD: u32 = DATA + 0x23_0000;
    pub(crate) const OC816_OBJ: u32 = DATA + 0x24_0000;
    pub(crate) const OC816_FONT: u32 = DATA + 0x25_0000;
    pub(crate) const OC816_GRMAN: u32 = DATA + 0x26_0000;
    pub(crate) const OC816_GRVT: u32 = DATA + 0x27_0000;
    /// Offsets in the menu object (self + 0x8000 + ..): current row and
    /// the values the rows print; `self + 0x8c10 + 8` is the active image.
    pub(crate) const OC816_SEL_ROW: u32 = 0x8000 + 0xc1c;
    pub(crate) const OC816_IMAGE: u32 = 0x8c10 + 8;

    /// Gated fakes for the callees of `DebugMenu_Render`: with the gate word
    /// written they log the call like `fakes::fake_call`; otherwise they run
    /// the body they replaced.
    macro_rules! oc816_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC816_CTRL) != OC816_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc816_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc816_fakes!(
        (
            oc816_fake_17ae00,
            OC816_PREV_17AE00,
            ee::s02::OVERRIDE_0017AE00,
            ee::s02::f_0017ae00__generated,
            0x0017ae00
        ),
        (
            oc816_fake_14dc80,
            OC816_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc816_fake_14dd58,
            OC816_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc816_fake_391cb0,
            OC816_PREV_391CB0,
            ee::s12::OVERRIDE_00391CB0,
            ee::s12::f_00391cb0__generated,
            0x00391cb0
        ),
        (
            oc816_fake_392a60,
            OC816_PREV_392A60,
            ee::s13::OVERRIDE_00392A60,
            ee::s13::f_00392a60__generated,
            0x00392a60
        ),
        (
            oc816_fake_145378,
            OC816_PREV_145378,
            ee::s01::OVERRIDE_00145378,
            ee::s01::f_00145378__generated,
            0x00145378
        ),
        (
            oc816_fake_145398,
            OC816_PREV_145398,
            ee::s01::OVERRIDE_00145398,
            ee::s01::f_00145398__generated,
            0x00145398
        ),
        (
            oc816_fake_145340,
            OC816_PREV_145340,
            ee::s01::OVERRIDE_00145340,
            ee::s01::f_00145340__generated,
            0x00145340
        ),
        (
            oc816_fake_1dca80,
            OC816_PREV_1DCA80,
            ee::s04::OVERRIDE_001DCA80,
            ee::s04::f_001dca80__generated,
            0x001dca80
        ),
        (
            oc816_fake_145c38,
            OC816_PREV_145C38,
            ee::s01::OVERRIDE_00145C38,
            ee::s01::f_00145c38__generated,
            0x00145c38
        ),
        (
            oc816_fake_417828,
            OC816_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
        (
            oc816_fake_144bc0,
            OC816_PREV_144BC0,
            ee::s01::OVERRIDE_00144BC0,
            ee::s01::f_00144bc0__generated,
            0x00144bc0
        ),
        (
            oc816_fake_14ee28,
            OC816_PREV_14EE28,
            ee::s01::OVERRIDE_0014EE28,
            ee::s01::f_0014ee28__generated,
            0x0014ee28
        ),
        (
            oc816_fake_147618,
            OC816_PREV_147618,
            ee::s01::OVERRIDE_00147618,
            ee::s01::f_00147618__generated,
            0x00147618
        ),
        (
            oc816_fake_28b180,
            OC816_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028b180
        ),
        (
            oc816_fake_1dc990,
            OC816_PREV_1DC990,
            ee::s04::OVERRIDE_001DC990,
            ee::s04::f_001dc990__generated,
            0x001dc990
        ),
        (
            oc816_fake_1dca40,
            OC816_PREV_1DCA40,
            ee::s04::OVERRIDE_001DCA40,
            ee::s04::f_001dca40__generated,
            0x001dca40
        ),
        (
            oc816_fake_392908,
            OC816_PREV_392908,
            ee::s12::OVERRIDE_00392908,
            ee::s12::f_00392908__generated,
            0x00392908
        ),
        (
            oc816_fake_1dc9d0,
            OC816_PREV_1DC9D0,
            ee::s04::OVERRIDE_001DC9D0,
            ee::s04::f_001dc9d0__generated,
            0x001dc9d0
        ),
        (
            oc816_fake_3f3cc0,
            OC816_PREV_3F3CC0,
            ee::s14::OVERRIDE_003F3CC0,
            ee::s14::f_003f3cc0__generated,
            0x003f3cc0
        ),
    );

    /// `DebugMenu_Render(self)`: the active image negative (no background call)
    /// or not (only the background is drawn), each of the 15 highlighted
    /// rows (and one past them), the multiplayer and world flags, the local
    /// address call, and fake unwinds at random calls.
    #[test]
    fn oc816_debug_menu_render_matches() {
        oc816_install_fakes();
        check(
            ee::s02::f_0017c5f8__generated,
            debug_menu_render,
            480,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0017_c5f8, 70);
                c.bus.write32(OC816_CTRL, OC816_MAGIC);
                let bits = r.next_u32();
                fill(c, &mut r, OC816_FONT, 0x20);
                c.bus.write32(OC816_FONT + 0x14, r.next_u32() & 0xff);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), OC816_WORLD);
                c.bus.write32(OC816_WORLD + 0x80, OC816_OBJ);
                c.bus.write32(OC816_WORLD + 0x78, r.next_u32() & 0xfffff0);
                c.bus.write32(OC816_WORLD + 4, bits & 1);
                c.bus.write32(OC816_OBJ + 0x18, OC816_FONT);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f38), (bits >> 1) & 1);
                c.bus.write32(GAME_GP.wrapping_sub(0x238), (bits >> 2) & 1);
                // The graphics manager's vtable slot 0x4c: a fake method, or a
                // real leaf that returns 0.
                c.bus.write32(GAME_GP.wrapping_sub(0x854), OC816_GRMAN);
                c.bus.write32(OC816_GRMAN + 0x10d8, OC816_GRVT);
                c.bus.write16(OC816_GRVT + 0x48, 0x10 * (seed % 3) as u16);
                c.bus.write32(
                    OC816_GRVT + 0x4c,
                    if (bits >> 3) & 1 == 0 {
                        fakes::FAKE + 0x4c
                    } else {
                        0x0014_06c0
                    },
                );
                fill(c, &mut r, OC816_SELF + 0x8c00, 0x10);
                let image = if (bits >> 4) % 8 == 0 {
                    0u32
                } else {
                    0xffff_ffff
                };
                c.bus.write32(OC816_SELF + OC816_IMAGE, image);
                c.bus.write32(OC816_SELF + OC816_SEL_ROW, seed % 17);
                c.bus.write32(OC816_SELF + 0x8000 + 0xc24, (bits >> 8) % 6);
                c.bus.write32(OC816_SELF + 0x8000 + 0xc30, r.next_u32());
                c.bus
                    .write32(OC816_SELF + 0x8000 + 0xc34, r.next_u32() & 0xffff);
                c.bus.write32(OC816_SELF + 0x8000 + 0xc44, (bits >> 12) % 4);
                c.g[A0][0] = sx(OC816_SELF);
            },
        );
    }
}
