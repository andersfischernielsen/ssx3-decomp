//! Staging module oc983 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc983`.

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
    (
        0x001f_df40,
        ov_screen_save_replay_v26,
        "OVScreenSaveReplay_v26",
    ),
    (
        0x0020_8c28,
        ov_screen_session_on_widget_action,
        "OVScreenSession_OnWidgetAction",
    ),
    (
        0x0020_d338,
        ov_screen_yn_dialog_init,
        "OVScreenYNDialog_Init",
    ),
    (
        0x0020_d590,
        ov_screen_yn_dialog_on_widget_action,
        "OVScreenYNDialog_OnWidgetAction",
    ),
];

/// `OVScreenSaveReplay_v26` (0x001fdf40): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_save_replay_v26(c: &mut Cpu) {
    if c.pc != 0x001fdf40 {
        return ee::s05::f_001fdf40__generated(c);
    }
    guest(c, |c| {
        'b_1fdff0: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x720));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x6e0));
            c.g[S3][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x6f0));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x710));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x700));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x6d0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x6c0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x6b0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x6a0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x690));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x680), c.g[RA][0]);
            c.g[RA][0] = sx(0x001fdf7c);
            Callee {
                addr: 0x00227f80,
                f: ee::s05::MCManager_Get,
            }
            .call(c, 0x001fdf7c)?;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001fdf94);
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x001fdf94)?;
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                break 'b_1fdff0;
            } else {
                'b_1fdfcc: {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001fdfb4);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001fdfb4)?;
                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
                        break 'b_1fdfcc;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001fdfc4);
                        call_ptr(c, 0x001d8240, 0x001fdfc4)?;
                        let t = true;
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_1fdff0;
                        } else {
                            break 'b_1fdfcc;
                        }
                    }
                }
                'b_1fdfec: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x3;
                    if t {
                        break 'b_1fdfec;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x118));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001fdfec);
                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001fdfec)?;
                        break 'b_1fdfec;
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                break 'b_1fdff0;
            }
        }
        'b_1fe3f8: {
            'b_1fe3f4: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001fe000);
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001fe000)?;
                let t = c.g[V0][0] == 0u64;
                c.g[S7][0] = 0u64;
                if t {
                    break 'b_1fe3f4;
                } else {
                    c.cycles += 60;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[S5][0] = 0x1;
                    c.g[FP][0] = 0x470000;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x158));
                    c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(0x230));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x15c));
                    c.g[S4][0] = 0u64;
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fe02c);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001fe02c)?;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x428));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0x320;
                    c.g[RA][0] = sx(0x001fe040);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00241710, 0x001fe040)?;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x001fe04c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x130));
                    call_ptr(c, 0x00317670, 0x001fe04c)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001fe058);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fe058)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001fe064);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0d00, 0x001fe064)?;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x001fe070);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf0));
                    call_ptr(c, 0x00317670, 0x001fe070)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001fe07c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fe07c)?;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x200));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x204));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fe09c);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001fe09c)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001fe0a8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0e90, 0x001fe0a8)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fe0c0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001fe0c0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x158));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001fe0e0);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    call_ptr(c, 0x00317670, 0x001fe0e0)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fe0f0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x001fe0f0)?;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
                    'l_1fe0f8: loop {
                        'b_1fe2e8: {
                            'b_1fe2b4: {
                                'b_1fe2a4: {
                                    'b_1fe21c: {
                                        c.cycles += 30;
                                        c.g[A2][0] = c.g[S5][0];
                                        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x148));
                                        c.g[RA][0] = sx(0x001fe108);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x00417828, 0x001fe108)?;
                                        c.g[RA][0] = sx(0x001fe110);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x00317670, 0x001fe110)?;
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                        c.g[RA][0] = sx(0x001fe11c);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x001fe11c)?;
                                        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.g[S6][0];
                                        c.g[A1][0] = c.g[S4][0];
                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001fe140);
                                        c.g[A0][0] =
                                            sx((c.g[A3][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001fe140)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001fe14c);
                                        c.g[A1][0] = c.g[S6][0];
                                        call_ptr(c, 0x003a0e90, 0x001fe14c)?;
                                        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                                        c.g[A1][0] = c.g[S4][0];
                                        c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001fe168);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001fe168)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                                        if t {
                                            break 'b_1fe21c;
                                        } else {
                                            c.cycles += 8;
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001fe188);
                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001fe188)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A2][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                                                break 'b_1fe21c;
                                            } else {
                                                c.cycles += 8;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                                                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x1b8));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x1bc));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001fe1a8);
                                                c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001fe1a8)?;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[A2][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x434)
                                                        );
                                                    break 'b_1fe21c;
                                                } else {
                                                    'b_1fe1f4: {
                                                        c.cycles += 2;
                                                        if c.g[S7][0] != 0u64 {
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            break 'b_1fe1f4;
                                                        } else {
                                                            c.cycles += 15;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = 0u64;
                                                            c.g[S7][0] = 0x1;
                                                            c.g[A0][0] = c.lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc8));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xcc));
                                                            let t = c.g[V0][0] as u32;
                                                            c.g[RA][0] = sx(0x001fe1d4);
                                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x001fe1d4)?;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A0][0] = c.lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x44));
                                                            let t = c.g[V0][0] as u32;
                                                            c.g[RA][0] = sx(0x001fe1ec);
                                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x001fe1ec)?;
                                                            let t = true;
                                                            c.g[S5][0] = sx((c.g[S5][0] as u32)
                                                                .wrapping_add(0x1));
                                                            if t {
                                                                break 'b_1fe2e8;
                                                            } else {
                                                                break 'b_1fe1f4;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 10;
                                                    c.g[A1][0] = 0x1;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x001fe208);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x001fe208)?;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                    let t = true;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_1fe2a4;
                                                    } else {
                                                        break 'b_1fe21c;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x428));
                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001fe234);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001fe234)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.g[S4][0];
                                    if t {
                                        break 'b_1fe2b4;
                                    } else {
                                        'b_1fe29c: {
                                            'b_1fe27c: {
                                                c.cycles += 8;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                                                c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001fe254);
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001fe254)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = 0x1;
                                                if t {
                                                    break 'b_1fe27c;
                                                } else {
                                                    c.cycles += 8;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x001fe270);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x001fe270)?;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    let t = true;
                                                    c.g[A1][0] = 0x1;
                                                    if t {
                                                        break 'b_1fe29c;
                                                    } else {
                                                        break 'b_1fe27c;
                                                    }
                                                }
                                            }
                                            c.cycles += 8;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001fe294);
                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001fe294)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0u64;
                                            break 'b_1fe29c;
                                        }
                                        c.cycles += 2;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                        break 'b_1fe2a4;
                                    }
                                }
                                c.cycles += 4;
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001fe2ac);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001fe2ac)?;
                                let t = true;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_1fe2e8;
                                } else {
                                    break 'b_1fe2b4;
                                }
                            }
                            c.cycles += 13;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001fe2cc);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001fe2cc)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001fe2e4);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001fe2e4)?;
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                            break 'b_1fe2e8;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S5][0] as i64) < 5i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001fe0f8);
                            }
                            continue 'l_1fe0f8;
                        } else {
                            'b_1fe378: {
                                'b_1fe348: {
                                    c.cycles += 8;
                                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x428));
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001fe308);
                                    c.g[A3][0] = 0x1;
                                    call_ptr(c, 0x001d97e0, 0x001fe308)?;
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    c.g[V0][0] = 0x470000;
                                    if t {
                                        break 'b_1fe348;
                                    } else {
                                        c.cycles += 13;
                                        c.g[RA][0] = sx(0x001fe31c);
                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x168));
                                        call_ptr(c, 0x00317670, 0x001fe31c)?;
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                        c.g[RA][0] = sx(0x001fe328);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x001fe328)?;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[A2][0] as u32;
                                        c.g[RA][0] = sx(0x001fe340);
                                        c.g[A0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001fe340)?;
                                        let t = true;
                                        c.g[A1][0] = c.g[S2][0];
                                        if t {
                                            break 'b_1fe378;
                                        } else {
                                            break 'b_1fe348;
                                        }
                                    }
                                }
                                c.cycles += 12;
                                c.g[RA][0] = sx(0x001fe350);
                                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x168));
                                call_ptr(c, 0x00317670, 0x001fe350)?;
                                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001fe35c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001fe35c)?;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                let t = c.g[A2][0] as u32;
                                c.g[RA][0] = sx(0x001fe374);
                                c.g[A0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001fe374)?;
                                c.g[A1][0] = c.g[S2][0];
                                break 'b_1fe378;
                            }
                            'b_1fe3c8: {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S3][0];
                                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
                                c.g[RA][0] = sx(0x001fe388);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x001d97e0, 0x001fe388)?;
                                c.g[V1][0] = 0xffffffffffffffff;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[V0][0] = 0x470000;
                                if t {
                                    break 'b_1fe3c8;
                                } else {
                                    c.cycles += 13;
                                    c.g[RA][0] = sx(0x001fe39c);
                                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x178));
                                    call_ptr(c, 0x00317670, 0x001fe39c)?;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fe3a8);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fe3a8)?;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[A2][0] as u32;
                                    c.g[RA][0] = sx(0x001fe3c0);
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001fe3c0)?;
                                    let t = true;
                                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
                                    if t {
                                        break 'b_1fe3f8;
                                    } else {
                                        break 'b_1fe3c8;
                                    }
                                }
                            }
                            c.cycles += 11;
                            c.g[RA][0] = sx(0x001fe3d0);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x178));
                            call_ptr(c, 0x00317670, 0x001fe3d0)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001fe3dc);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001fe3dc)?;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = c.g[A2][0] as u32;
                            c.g[RA][0] = sx(0x001fe3f4);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001fe3f4)?;
                            break 'b_1fe3f4;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
            break 'b_1fe3f8;
        }
        'b_1fe474: {
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[S1][0] = 0x1;
            if t {
                break 'b_1fe474;
            } else {
                c.cycles += 3;
                c.g[FP][0] = 0x470000;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
                c.g[A2][0] = c.g[S1][0];
                'l_1fe410: loop {
                    c.cycles += 25;
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x148));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001fe420);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00417828, 0x001fe420)?;
                    c.g[RA][0] = sx(0x001fe428);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00317670, 0x001fe428)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001fe434);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fe434)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fe450);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001fe450)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fe468);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001fe468)?;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = c.g[S1][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001fe410);
                        }
                        continue 'l_1fe410;
                    } else {
                        break 'b_1fe474;
                    }
                }
            }
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x710));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x700));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x6f0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x6e0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x6d0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x6c0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x690));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x680));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x720));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenSession_OnWidgetAction` (0x00208c28): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_session_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x00208c28 {
        return ee::s05::f_00208c28__generated(c);
    }
    guest(c, |c| 'l_208c28: loop {
        'b_208ee4: {
            'b_208ee0: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.g[S1][0] = c.g[A1][0];
                let t = c.g[S1][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                if t {
                    break 'b_208ee0;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V1][0] < 0x7u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        break 'b_208ee0;
                    } else {
                        'b_208ec8: {
                            'b_208dd8: {
                                'b_208da0: {
                                    'b_208d90: {
                                        'b_208d80: {
                                            c.cycles += 6;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x1870));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                            let t = c.g[A0][0] as u32;
                                            match t {
                                                0x00208c28 => {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x00208c28);
                                                    }
                                                    continue 'l_208c28;
                                                }
                                                0x00208c6c => {
                                                    'b_208d1c: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xbc));
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V1][0] = 0x1;
                                                        if t {
                                                            break 'b_208d1c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x18));
                                                            if c.g[V0][0] != 0u64 {
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0xbc),
                                                                    0u64,
                                                                );
                                                                break 'b_208d90;
                                                            } else {
                                                                c.cycles += 38;
                                                                c.g[RA][0] = sx(0x00208c8c);
                                                                call_ptr(
                                                                    c, 0x0014dc80, 0x00208c8c,
                                                                )?;
                                                                c.g[A1][0] = 0xa;
                                                                c.g[RA][0] = sx(0x00208c98);
                                                                c.g[A0][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x0014dd58, 0x00208c98,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0xd8));
                                                                c.g[RA][0] = sx(0x00208ca4);
                                                                c.sw(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    c.g[V1][0],
                                                                );
                                                                call_ptr(
                                                                    c, 0x0028b180, 0x00208ca4,
                                                                )?;
                                                                c.g[RA][0] = sx(0x00208cac);
                                                                c.g[A0][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x0028fae0, 0x00208cac,
                                                                )?;
                                                                c.g[T0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x69c));
                                                                c.g[V1][0] = 0x530000;
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x66e8));
                                                                c.g[A2][0] = 0x440000;
                                                                c.g[T0][0] = sx((c.g[T0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x1));
                                                                c.g[A2][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(0x28f0));
                                                                c.g[V0][0] =
                                                                    sx((c.g[T0][0] as u32) << 2);
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V1][0] as u32),
                                                                    ));
                                                                c.g[T1][0] = 0x3;
                                                                c.g[V1][0] =
                                                                    c.lw((c.g[V0][0] as u32));
                                                                c.g[A3][0] = 0u64;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[A1][0] as u32)
                                                                        .wrapping_add(0x84));
                                                                c.g[V1][0] =
                                                                    sx((c.g[V1][0] as u32) << 2);
                                                                c.g[A1][0] = 0xf;
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A2][0] as u32),
                                                                    ));
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x9f4),
                                                                    c.g[T1][0],
                                                                );
                                                                c.g[V0][0] =
                                                                    c.lw((c.g[V1][0] as u32));
                                                                c.g[A2][0] = 0u64;
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x69c),
                                                                    c.g[T0][0],
                                                                );
                                                                c.g[RA][0] = sx(0x00208d04);
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x6a0),
                                                                    c.g[V0][0],
                                                                );
                                                                call_ptr(
                                                                    c, 0x00231250, 0x00208d04,
                                                                )?;
                                                                c.g[RA][0] = sx(0x00208d0c);
                                                                c.g[A0][0] = c.g[S0][0];
                                                                Callee { addr: 0x0020a088, f: ee::s05::OVScreenSession_DestroyIndicators }.call(c, 0x00208d0c)?;
                                                                c.g[RA][0] = sx(0x00208d14);
                                                                c.g[A0][0] = c.g[S0][0];
                                                                Callee { addr: 0x00208bb8, f: ee::s05::OVScreenSession_HideMapPic }.call(c, 0x00208d14)?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                if t {
                                                                    break 'b_208ee4;
                                                                } else {
                                                                    break 'b_208d1c;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 25;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0xbc),
                                                        c.g[V1][0],
                                                    );
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                                    c.g[RA][0] = sx(0x00208d30);
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0xd8),
                                                        c.g[V0][0],
                                                    );
                                                    Callee { addr: 0x00208f88, f: ee::s05::OVScreenSession_UpdateConfirmPopup }.call(c, 0x00208d30)?;
                                                    c.g[A0][0] = 0x470000;
                                                    c.g[RA][0] = sx(0x00208d3c);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x6c8));
                                                    call_ptr(c, 0x00317670, 0x00208d3c)?;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0xd4));
                                                    c.g[RA][0] = sx(0x00208d48);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, 0x0039d860, 0x00208d48)?;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                                    c.g[AT][0] = 0x3f800000;
                                                    c.mtc1(1, 0);
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                    c.swc1(0, (c.g[SP][0] as u32));
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x4),
                                                        0u64,
                                                    );
                                                    c.g[A0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0x8),
                                                        0u64,
                                                    );
                                                    c.sw(
                                                        (c.g[SP][0] as u32).wrapping_add(0xc),
                                                        0u64,
                                                    );
                                                    c.g[V0][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[RA][0] = sx(0x00208d78);
                                                    c.g[A1][0] = c.g[SP][0];
                                                    call_ptr(c, t, 0x00208d78)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_208ee4;
                                                    } else {
                                                        break 'b_208d80;
                                                    }
                                                }
                                                0x00208d80 => {
                                                    break 'b_208d80;
                                                }
                                                0x00208dd8 => {
                                                    break 'b_208dd8;
                                                }
                                                0x00208ec8 => {
                                                    break 'b_208ec8;
                                                }
                                                0x00208ee0 => {
                                                    break 'b_208ee0;
                                                }
                                                _ => {
                                                    c.jump_out(t, 0x00208c28, 0x00208ef4);
                                                    return Ok(());
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                                        let t = c.g[V0][0] == 0u64;

                                        if t {
                                            break 'b_208da0;
                                        } else {
                                            c.cycles += 1;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), 0u64);
                                            break 'b_208d90;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x00208d98);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x00208f88,
                                        f: ee::s05::OVScreenSession_UpdateConfirmPopup,
                                    }
                                    .call(c, 0x00208d98)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_208ee4;
                                    } else {
                                        break 'b_208da0;
                                    }
                                }
                                c.cycles += 14;
                                c.g[RA][0] = sx(0x00208da8);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x0020a088,
                                    f: ee::s05::OVScreenSession_DestroyIndicators,
                                }
                                .call(c, 0x00208da8)?;
                                c.g[RA][0] = sx(0x00208db0);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x00208bb8,
                                    f: ee::s05::OVScreenSession_HideMapPic,
                                }
                                .call(c, 0x00208db0)?;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = 0x1;
                                c.g[RA][0] = sx(0x00208dc0);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f190, 0x00208dc0)?;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                c.g[V0][0] = 0xffffffffffffc0ff;
                                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                                c.g[V1][0] = c.g[V1][0] | 0x780;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                                if t {
                                    break 'b_208ee0;
                                } else {
                                    break 'b_208dd8;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_208ee0;
                            } else {
                                'b_208e64: {
                                    c.cycles += 6;
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x00208df0);
                                    c.g[A2][0] = 0x1;
                                    Callee {
                                        addr: 0x002095e8,
                                        f: ee::s05::OVScreenSession_SetHighlightColor,
                                    }
                                    .call(c, 0x00208df0)?;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A1][0] = 0x470000;
                                    if t {
                                        break 'b_208e64;
                                    } else {
                                        c.cycles += 26;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00208e0c);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6e8));
                                        call_ptr(c, 0x0039e948, 0x00208e0c)?;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00208e1c);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x700));
                                        call_ptr(c, 0x0039e8b8, 0x00208e1c)?;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00208e2c);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x718));
                                        call_ptr(c, 0x0039e948, 0x00208e2c)?;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00208e3c);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x730));
                                        call_ptr(c, 0x0039e8b8, 0x00208e3c)?;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00208e4c);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x748));
                                        call_ptr(c, 0x0039e948, 0x00208e4c)?;
                                        c.g[A1][0] = 0x470000;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00208e5c);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x760));
                                        call_ptr(c, 0x0039e8b8, 0x00208e5c)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_208ee4;
                                        } else {
                                            break 'b_208e64;
                                        }
                                    }
                                }
                                c.cycles += 25;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00208e70);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x700));
                                call_ptr(c, 0x0039e948, 0x00208e70)?;
                                c.g[A1][0] = 0x470000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00208e80);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6e8));
                                call_ptr(c, 0x0039e8b8, 0x00208e80)?;
                                c.g[A1][0] = 0x470000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00208e90);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x730));
                                call_ptr(c, 0x0039e948, 0x00208e90)?;
                                c.g[A1][0] = 0x470000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00208ea0);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x718));
                                call_ptr(c, 0x0039e8b8, 0x00208ea0)?;
                                c.g[A1][0] = 0x470000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00208eb0);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x760));
                                call_ptr(c, 0x0039e948, 0x00208eb0)?;
                                c.g[A1][0] = 0x470000;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x00208ec0);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x748));
                                call_ptr(c, 0x0039e8b8, 0x00208ec0)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_208ee4;
                                } else {
                                    break 'b_208ec8;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xbc));
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.g[S0][0];
                        if t {
                            break 'b_208ee0;
                        } else {
                            c.cycles += 3;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x00208ee0);
                            c.g[A2][0] = 0x1;
                            Callee {
                                addr: 0x00209648,
                                f: ee::s05::OVScreenSession_SetUnlockedColor,
                            }
                            .call(c, 0x00208ee0)?;
                            break 'b_208ee0;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_208ee4;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenYNDialog_Init` (0x0020d338): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_yn_dialog_init(c: &mut Cpu) {
    if c.pc != 0x0020d338 {
        return ee::s05::f_0020d338__generated(c);
    }
    guest(c, |c| 'l_20d338: loop {
        'b_20d388: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1af8));
            c.g[RA][0] = sx(0x0020d35c);
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x0020d35c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0020d370);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x0020d370)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_20d388;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x0020d388);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x0020d388)?;
                break 'b_20d388;
            }
        }
        'b_20d4e0: {
            'b_20d4c4: {
                c.cycles += 28;
                c.g[RA][0] = sx(0x0020d390);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0020a380,
                    f: ee::s05::cOVStateOverlay_Init,
                }
                .call(c, 0x0020d390)?;
                c.g[A0][0] = 0x4a0000;
                c.g[RA][0] = sx(0x0020d39c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23e0));
                call_ptr(c, 0x00317670, 0x0020d39c)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x0020d3a8);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x0020d3a8)?;
                c.g[A0][0] = 0x4a0000;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
                c.g[RA][0] = sx(0x0020d3b8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23e8));
                call_ptr(c, 0x00317670, 0x0020d3b8)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x0020d3c4);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x0020d3c4)?;
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x470000;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                c.g[RA][0] = sx(0x0020d3d8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b08));
                call_ptr(c, 0x00317670, 0x0020d3d8)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x0020d3e4);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x0020d3e4)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = (c.g[V1][0] < 0x7u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x470000;
                if t {
                    break 'b_20d4c4;
                } else {
                    'b_20d478: {
                        'b_20d470: {
                            'b_20d454: {
                                'b_20d44c: {
                                    'b_20d440: {
                                        'b_20d434: {
                                            'b_20d428: {
                                                'b_20d41c: {
                                                    c.cycles += 6;
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x1b60));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                    let t = c.g[A0][0] as u32;
                                                    match t {
                                                        0x0020d338 => {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x0020d338);
                                                            }
                                                            continue 'l_20d338;
                                                        }
                                                        0x0020d410 => {
                                                            c.cycles += 3;
                                                            c.g[A0][0] = 0x470000;
                                                            let t = true;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1b18));
                                                            if t {
                                                                break 'b_20d478;
                                                            } else {
                                                                break 'b_20d41c;
                                                            }
                                                        }
                                                        0x0020d41c => {
                                                            break 'b_20d41c;
                                                        }
                                                        0x0020d428 => {
                                                            break 'b_20d428;
                                                        }
                                                        0x0020d434 => {
                                                            break 'b_20d434;
                                                        }
                                                        0x0020d440 => {
                                                            break 'b_20d440;
                                                        }
                                                        0x0020d44c => {
                                                            break 'b_20d44c;
                                                        }
                                                        0x0020d470 => {
                                                            break 'b_20d470;
                                                        }
                                                        _ => {
                                                            c.jump_out(t, 0x0020d338, 0x0020d4f0);
                                                            return Ok(());
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x470000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0xa48));
                                                if t {
                                                    break 'b_20d454;
                                                } else {
                                                    break 'b_20d428;
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[A0][0] = 0x470000;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x9d0));
                                            if t {
                                                break 'b_20d454;
                                            } else {
                                                break 'b_20d434;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x9e8));
                                        if t {
                                            break 'b_20d454;
                                        } else {
                                            break 'b_20d440;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x470000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b48));
                                    if t {
                                        break 'b_20d454;
                                    } else {
                                        break 'b_20d44c;
                                    }
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 0x470000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x9f8));
                                break 'b_20d454;
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x0020d45c);
                            call_ptr(c, 0x00317670, 0x0020d45c)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x0020d468);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x0020d468)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_20d4e0;
                            } else {
                                break 'b_20d470;
                            }
                        }
                        c.cycles += 2;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10f8));
                        break 'b_20d478;
                    }
                    c.cycles += 19;
                    c.g[RA][0] = sx(0x0020d480);
                    call_ptr(c, 0x00317670, 0x0020d480)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0020d48c);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a1000, 0x0020d48c)?;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x0020d498);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b30));
                    call_ptr(c, 0x00317670, 0x0020d498)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x0020d4a4);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x0020d4a4)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[A2][0] as u32;
                    c.g[RA][0] = sx(0x0020d4bc);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0020d4bc)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_20d4e0;
                    } else {
                        break 'b_20d4c4;
                    }
                }
            }
            c.cycles += 7;
            c.g[A0][0] = 0x470000;
            c.g[RA][0] = sx(0x0020d4d0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x9b8));
            call_ptr(c, 0x00317670, 0x0020d4d0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0020d4dc);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a1000, 0x0020d4dc)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_20d4e0;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenYNDialog_OnWidgetAction` (0x0020d590): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_yn_dialog_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x0020d590 {
        return ee::s05::f_0020d590__generated(c);
    }
    guest(c, |c| 'l_20d590: loop {
        'b_20dbb8: {
            'b_20dbb4: {
                'b_20dbb0: {
                    c.cycles += 16;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S0][0] = c.g[A2][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[RA][0] = sx(0x0020d5bc);
                    c.g[S2][0] = c.g[A1][0];
                    call_ptr(c, 0x0014dc80, 0x0020d5bc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0020d5c8);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x0020d5c8)?;
                    let t = c.g[S2][0] == 0u64;
                    c.g[V0][0] = 0x5;
                    if t {
                        break 'b_20dbb0;
                    } else {
                        'b_20db64: {
                            'b_20d5e8: {
                                c.cycles += 2;
                                let t = c.g[S0][0] == c.g[V0][0];
                                c.g[V0][0] = 0x6;
                                if t {
                                    break 'b_20d5e8;
                                } else {
                                    c.cycles += 2;
                                    if c.g[S0][0] == c.g[V0][0] {
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                                        break 'b_20db64;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                        if t {
                                            break 'b_20dbb4;
                                        } else {
                                            break 'b_20d5e8;
                                        }
                                    }
                                }
                            }
                            'b_20d61c: {
                                c.cycles += 10;
                                c.g[A0][0] = 0x470000;
                                c.g[RA][0] = sx(0x0020d5f4);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                                call_ptr(c, 0x00317670, 0x0020d5f4)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x0020d600);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039c778, 0x0020d600)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = 0xffff;
                                let t = c.g[A1][0] == c.g[V0][0];
                                c.g[A1][0] = c.g[A1][0] & 0xffff;
                                if t {
                                    break 'b_20d61c;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x0020d61c);
                                    c.g[A2][0] = 0x1;
                                    call_ptr(c, 0x0039c870, 0x0020d61c)?;
                                    break 'b_20d61c;
                                }
                            }
                            'b_20db54: {
                                'b_20db50: {
                                    'b_20db24: {
                                        'b_20d638: {
                                            c.cycles += 3;
                                            c.g[A1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                            let t = c.g[A1][0] == 0u64;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_20d638;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A1][0] == c.g[V0][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                if t {
                                                    break 'b_20db24;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        17,
                                                        (c.g[SP][0] as u32).wrapping_add(0x40),
                                                    );
                                                    if t {
                                                        break 'b_20dbb8;
                                                    } else {
                                                        break 'b_20d638;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                                        c.g[V1][0] = (c.g[A0][0] < 0x7u64) as u64;
                                        let t = c.g[V1][0] == 0u64;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                                        if t {
                                            break 'b_20dbb0;
                                        } else {
                                            'b_20daf8: {
                                                'b_20dab8: {
                                                    'b_20da58: {
                                                        'b_20d954: {
                                                            'b_20d904: {
                                                                'b_20d8f4: {
                                                                    'b_20d82c: {
                                                                        c.cycles += 7;
                                                                        c.g[V0][0] = 0x470000;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1b80,
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[V0][0]
                                                                                        as u32),
                                                                                ));
                                                                        c.g[A0][0] =
                                                                            c.lw(
                                                                                (c.g[V1][0] as u32)
                                                                            );
                                                                        let t = c.g[A0][0] as u32;
                                                                        match t {
                                                                            0x0020d590 => {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x0020d590);
                                                                                }
                                                                                continue 'l_20d590;
                                                                            }
                                                                            0x0020d664 => {
                                                                                'b_20d794: {
                                                                                    c.cycles += 8;
                                                                                    c.g[V1][0] =
                                                                                        0x530000;
                                                                                    c.g[S1][0] =
                                                                                        c.g[V1][0];
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                                                                                    c.g[S3][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x4a));
                                                                                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x6));
                                                                                    c.g[V0][0] = (c
                                                                                        .g[V0][0]
                                                                                        < 0x6u64)
                                                                                        as u64;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        == 0u64;
                                                                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9e8));
                                                                                    if t {
                                                                                        break 'b_20d794;
                                                                                    } else {
                                                                                        'b_20d6e4: {
                                                                                            'b_20d6cc: {
                                                                                                c.cycles += 2;
                                                                                                let t = c.g[V0][0] == 0u64;
                                                                                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_20d6cc;
                                                                                                } else {
                                                                                                    c.cycles += 16;
                                                                                                    c.g[V0][0] = 0x530000;
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66e8));
                                                                                                    c.g[A2][0] = 0x440000;
                                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                                                                                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28f0));
                                                                                                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                                                                                                    c.g[A0][0] = 0x3;
                                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[A0][0]);
                                                                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                                                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A1][0]);
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                                                                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                                                                    let t = true;
                                                                                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
                                                                                                    if t {
break 'b_20d6e4;
} else {
break 'b_20d6cc;
}
                                                                                                }
                                                                                            }
                                                                                            c.cycles += 6;
                                                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                                                                            c.g[RA][0] = sx(0x0020d6dc);
                                                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                                                                                            call_ptr(c, 0x002706f0, 0x0020d6dc)?;
                                                                                            c.g[V0][0] = 0x3;
                                                                                            c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
                                                                                            break 'b_20d6e4;
                                                                                        }
                                                                                        'b_20d71c: {
                                                                                            'b_20d718: {
                                                                                                'b_20d704: {
                                                                                                    'b_20d6fc: {
                                                                                                        c.cycles += 3;
                                                                                                        c.g[V0][0] = 0x7;
                                                                                                        let t = c.g[S3][0] == c.g[V0][0];
                                                                                                        c.g[S4][0] = 0xe;
                                                                                                        if t {
break 'b_20d6fc;
} else {
c.cycles += 3;
c.g[V0][0] = 0xa;
let t = c.g[S3][0] != c.g[V0][0];
c.g[V0][0] = 0x8;
if t {
break 'b_20d704;
} else {
break 'b_20d6fc;
}
}
                                                                                                    }
                                                                                                    c.cycles += 2;
                                                                                                    let t = true;
                                                                                                    c.g[S4][0] = 0xf;
                                                                                                    if t {
break 'b_20d718;
} else {
break 'b_20d704;
}
                                                                                                }
                                                                                                'b_20d714: {
                                                                                                    c.cycles += 2;
                                                                                                    let t = c.g[S3][0] == c.g[V0][0];
                                                                                                    c.g[V0][0] = 0xb;
                                                                                                    if t {
break 'b_20d714;
} else {
c.cycles += 2;
let t = c.g[S3][0] != c.g[V0][0];
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
if t {
break 'b_20d71c;
} else {
break 'b_20d714;
}
}
                                                                                                }
                                                                                                c.cycles += 1;
                                                                                                c.g[S4][0] = 0x10;
                                                                                                break 'b_20d718;
                                                                                            }
                                                                                            c.cycles += 1;
                                                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                            break 'b_20d71c;
                                                                                        }
                                                                                        c.cycles +=
                                                                                            30;
                                                                                        c.g[A1]
                                                                                            [0] =
                                                                                            0u64;
                                                                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5bc8));
                                                                                        c.g[RA][0] = sx(0x0020d72c);
                                                                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                                                                        call_ptr(c, 0x0014dd58, 0x0020d72c)?;
                                                                                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                        c.g[S0]
                                                                                            [0] = c
                                                                                            .g[V0]
                                                                                            [0];
                                                                                        c.g[A0]
                                                                                            [0] =
                                                                                            0u64;
                                                                                        c.g[RA][0] = sx(0x0020d740);
                                                                                        c.g[S2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc0));
                                                                                        call_ptr(c, 0x0014de68, 0x0020d740)?;
                                                                                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x48));
                                                                                        c.g[RA][0] = sx(0x0020d74c);
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S0]
                                                                                            [0];
                                                                                        call_ptr(c, 0x00144df0, 0x0020d74c)?;
                                                                                        c.g[A1]
                                                                                            [0] = c
                                                                                            .g[S3]
                                                                                            [0];
                                                                                        c.g[RA][0] = sx(0x0020d758);
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S0]
                                                                                            [0];
                                                                                        call_ptr(c, 0x001451e8, 0x0020d758)?;
                                                                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[S4][0]);
                                                                                        c.g[V1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f34), c.g[S4][0]);
                                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[V1][0]);
                                                                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                                                        let t = c.g
                                                                                            [V1][0]
                                                                                            as u32;
                                                                                        c.g[RA][0] = sx(0x0020d77c);
                                                                                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                        call_ptr(c, t, 0x0020d77c)?;
                                                                                        c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x4a));
                                                                                        c.g[RA][0] = sx(0x0020d788);
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S2]
                                                                                            [0];
                                                                                        call_ptr(c, 0x00238160, 0x0020d788)?;
                                                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                        let t =
                                                                                            true;
                                                                                        c.g[A1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        if t {
                                                                                            break 'b_20db54;
                                                                                        } else {
                                                                                            break 'b_20d794;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_20d7ec: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        == 0u64;
                                                                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                                                                    if t {
                                                                                        break 'b_20d7ec;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            20;
                                                                                        c.g[V1][0] = 0x530000;
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
                                                                                        c.g[A1][0] = 0x440000;
                                                                                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                                                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28f0));
                                                                                        c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                                                                                        c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                                        c.g[T0]
                                                                                            [0] =
                                                                                            0x3;
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                                                        c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x84));
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[T0][0]);
                                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A2][0]);
                                                                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                                                        c.g[RA][0] = sx(0x0020d7e4);
                                                                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V0][0]);
                                                                                        call_ptr(c, 0x002302a8, 0x0020d7e4)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                                                        if t {
                                                                                            break 'b_20dbb4;
                                                                                        } else {
                                                                                            break 'b_20d7ec;
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 16;
                                                                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                                                                c.g[RA][0] =
                                                                                    sx(0x0020d7fc);
                                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                                                                                call_ptr(
                                                                                    c, 0x002706f0,
                                                                                    0x0020d7fc,
                                                                                )?;
                                                                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                c.g[RA][0] =
                                                                                    sx(0x0020d808);
                                                                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                                                                                call_ptr(
                                                                                    c, 0x00238348,
                                                                                    0x0020d808,
                                                                                )?;
                                                                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                c.g[A1][0] = 0xd;
                                                                                c.g[A2][0] = 0u64;
                                                                                c.g[A3][0] = 0x1;
                                                                                c.g[RA][0] =
                                                                                    sx(0x0020d820);
                                                                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                                                                call_ptr(
                                                                                    c, 0x00231250,
                                                                                    0x0020d820,
                                                                                )?;
                                                                                c.g[V0][0] = 0x3;
                                                                                let t = true;
                                                                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
                                                                                if t {
                                                                                    break 'b_20dbb0;
                                                                                } else {
                                                                                    break 'b_20d82c;
                                                                                }
                                                                            }
                                                                            0x0020d82c => {
                                                                                break 'b_20d82c;
                                                                            }
                                                                            0x0020d8f4 => {
                                                                                break 'b_20d8f4;
                                                                            }
                                                                            0x0020d904 => {
                                                                                break 'b_20d904;
                                                                            }
                                                                            0x0020d954 => {
                                                                                break 'b_20d954;
                                                                            }
                                                                            0x0020da58 => {
                                                                                break 'b_20da58;
                                                                            }
                                                                            0x0020dab8 => {
                                                                                break 'b_20dab8;
                                                                            }
                                                                            _ => {
                                                                                c.jump_out(
                                                                                    t, 0x0020d590,
                                                                                    0x0020dbd0,
                                                                                );
                                                                                return Ok(());
                                                                            }
                                                                        }
                                                                    }
                                                                    'b_20d868: {
                                                                        c.cycles += 12;
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        c.g[V1][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x84));
                                                                        c.g[RA][0] = sx(0x0020d83c);
                                                                        c.g[A0][0] = c.lw((c.g[V1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x28));
                                                                        call_ptr(
                                                                            c, 0x002706f0,
                                                                            0x0020d83c,
                                                                        )?;
                                                                        c.g[RA][0] = sx(0x0020d844);
                                                                        c.sw(
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x54),
                                                                            0u64,
                                                                        );
                                                                        call_ptr(
                                                                            c, 0x0014dc80,
                                                                            0x0020d844,
                                                                        )?;
                                                                        c.g[A0][0] = c.g[V0][0];
                                                                        c.g[RA][0] = sx(0x0020d850);
                                                                        c.g[A1][0] = 0x4;
                                                                        call_ptr(
                                                                            c, 0x0014dd58,
                                                                            0x0020d850,
                                                                        )?;
                                                                        c.g[A0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x204));
                                                                        let t = c.g[A0][0] == 0u64;
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        if t {
                                                                            break 'b_20d868;
                                                                        } else {
                                                                            c.cycles += 3;
                                                                            c.g[RA][0] =
                                                                                sx(0x0020d864);
                                                                            c.g[A1][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x00257610,
                                                                                0x0020d864,
                                                                            )?;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x848,
                                                                                ));
                                                                            break 'b_20d868;
                                                                        }
                                                                    }
                                                                    c.cycles += 13;
                                                                    c.g[A1][0] = 0x7;
                                                                    c.g[S0][0] = 0u64;
                                                                    c.g[RA][0] = sx(0x0020d878);
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[V0][0] as u32)
                                                                            .wrapping_add(0x78));
                                                                    call_ptr(
                                                                        c, 0x0014dd58, 0x0020d878,
                                                                    )?;
                                                                    c.g[RA][0] = sx(0x0020d880);
                                                                    call_ptr(
                                                                        c, 0x0014dc80, 0x0020d880,
                                                                    )?;
                                                                    c.g[A0][0] = c.g[V0][0];
                                                                    c.g[RA][0] = sx(0x0020d88c);
                                                                    c.g[A1][0] = 0u64;
                                                                    call_ptr(
                                                                        c, 0x0014dd58, 0x0020d88c,
                                                                    )?;
                                                                    c.g[V1][0] = 0x530000;
                                                                    c.g[V0][0] = c
                                                                        .lb((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x5c11));
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x848));
                                                                    if t {
                                                                        break 'b_20db50;
                                                                    } else {
                                                                        'b_20d8dc: {
                                                                            c.cycles += 2;
                                                                            let t =
                                                                                c.g[S0][0] == 0u64;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x848,
                                                                                ));
                                                                            if t {
                                                                                break 'b_20d8dc;
                                                                            } else {
                                                                                c.cycles += 14;
                                                                                c.g[A1][0] =
                                                                                    0x470000;
                                                                                c.g[A3][0] = 0u64;
                                                                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x640));
                                                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                                                                c.g[A0][0] = 0x234;
                                                                                c.g[A2][0] = 0x100;
                                                                                c.g[RA][0] =
                                                                                    sx(0x0020d8c4);
                                                                                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                                                                                call_ptr(
                                                                                    c, 0x00317d70,
                                                                                    0x0020d8c4,
                                                                                )?;
                                                                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                                                                c.g[A0][0] =
                                                                                    c.g[V0][0];
                                                                                c.g[RA][0] =
                                                                                    sx(0x0020d8d4);
                                                                                c.g[A2][0] = 0x1;
                                                                                Callee { addr: 0x00212138, f: ee::s05::OVScreenAutosave_ctor }.call(c, 0x0020d8d4)?;
                                                                                let t = true;
                                                                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                                                if t {
                                                                                    break 'b_20daf8;
                                                                                } else {
                                                                                    break 'b_20d8dc;
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 6;
                                                                        c.g[RA][0] = sx(0x0020d8e4);
                                                                        c.g[A0][0] = 0x26;
                                                                        Callee { addr: 0x0020ab50, f: ee::s05::GameUI_OpenScreen }.call(c, 0x0020d8e4)?;
                                                                        c.g[RA][0] = sx(0x0020d8ec);
                                                                        Callee { addr: 0x0020d110, f: ee::s05::OVScreenPDATemplate_Open }.call(c, 0x0020d8ec)?;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x50),
                                                                        );
                                                                        if t {
                                                                            break 'b_20dbb4;
                                                                        } else {
                                                                            break 'b_20d8f4;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x0020d8fc);
                                                                c.g[A0][0] = 0x1d;
                                                                Callee {
                                                                    addr: 0x0020ab50,
                                                                    f: ee::s05::GameUI_OpenScreen,
                                                                }
                                                                .call(c, 0x0020d8fc)?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x50),
                                                                );
                                                                if t {
                                                                    break 'b_20dbb4;
                                                                } else {
                                                                    break 'b_20d904;
                                                                }
                                                            }
                                                            c.cycles += 20;
                                                            c.g[A1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x69c));
                                                            c.g[V0][0] = 0x530000;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x66e8));
                                                            c.g[A2][0] = 0x440000;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0x1));
                                                            c.g[A2][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add(0x28f0));
                                                            c.g[V1][0] =
                                                                sx((c.g[A1][0] as u32) << 2);
                                                            c.g[A3][0] = 0x3;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0xce8));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                            c.sw(
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x9f4),
                                                                c.g[A3][0],
                                                            );
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.sw(
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x69c),
                                                                c.g[A1][0],
                                                            );
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add((c.g[A2][0] as u32)));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                            c.g[RA][0] = sx(0x0020d94c);
                                                            c.sw(
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x6a0),
                                                                c.g[V1][0],
                                                            );
                                                            call_ptr(c, 0x0030b758, 0x0020d94c)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x50),
                                                            );
                                                            if t {
                                                                break 'b_20dbb4;
                                                            } else {
                                                                break 'b_20d954;
                                                            }
                                                        }
                                                        c.cycles += 65;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x84));
                                                        c.g[RA][0] = sx(0x0020d964);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x28));
                                                        call_ptr(c, 0x002706f0, 0x0020d964)?;
                                                        c.g[V0][0] = 0x3;
                                                        c.g[RA][0] = sx(0x0020d970);
                                                        c.sw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x9f4),
                                                            c.g[V0][0],
                                                        );
                                                        call_ptr(c, 0x0014dc80, 0x0020d970)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x0020d97c);
                                                        c.g[A1][0] = 0u64;
                                                        call_ptr(c, 0x0014dd58, 0x0020d97c)?;
                                                        c.g[RA][0] = sx(0x0020d984);
                                                        c.g[S0][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0014dc80, 0x0020d984)?;
                                                        c.g[A1][0] = 0xa;
                                                        c.g[RA][0] = sx(0x0020d990);
                                                        c.g[A0][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0014dd58, 0x0020d990)?;
                                                        c.g[S1][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x0020d99c);
                                                        c.g[A0][0] = 0u64;
                                                        call_ptr(c, 0x0014de68, 0x0020d99c)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x0020d9a8);
                                                        c.g[A1][0] = 0xc;
                                                        call_ptr(c, 0x001451e8, 0x0020d9a8)?;
                                                        c.g[A1][0] = 0x4;
                                                        c.g[RA][0] = sx(0x0020d9b4);
                                                        c.g[A0][0] = c.g[S0][0];
                                                        call_ptr(c, 0x00144df0, 0x0020d9b4)?;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc));
                                                        let t = c.g[V0][0] as u32;
                                                        c.g[RA][0] = sx(0x0020d9c8);
                                                        c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        call_ptr(c, t, 0x0020d9c8)?;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[A1][0] = 0xc;
                                                        c.g[RA][0] = sx(0x0020d9d8);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc0));
                                                        call_ptr(c, 0x00238160, 0x0020d9d8)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x84));
                                                        c.g[RA][0] = sx(0x0020d9e8);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc));
                                                        call_ptr(c, 0x00128a10, 0x0020d9e8)?;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[V0][0] = 0x1;
                                                        c.sw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x10),
                                                            c.g[V0][0],
                                                        );
                                                        c.g[A3][0] = 0u64;
                                                        c.g[A1][0] = 0x1;
                                                        c.g[A2][0] = 0x1;
                                                        c.g[RA][0] = sx(0x0020da08);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x84));
                                                        call_ptr(c, 0x00231250, 0x0020da08)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[RA][0] = sx(0x0020da14);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x84));
                                                        call_ptr(c, 0x00230180, 0x0020da14)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[A1][0] = 0x1;
                                                        c.g[A2][0] = 0x2;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x84));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[A3][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x0020da34);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x18));
                                                        call_ptr(c, 0x0011de60, 0x0020da34)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[A1][0] = 0x1;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x84));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[A2][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x0020da50);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x18));
                                                        call_ptr(c, 0x0011df18, 0x0020da50)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x50),
                                                        );
                                                        if t {
                                                            break 'b_20dbb4;
                                                        } else {
                                                            break 'b_20da58;
                                                        }
                                                    }
                                                    c.cycles += 24;
                                                    c.g[A2][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x69c)
                                                        );
                                                    c.g[V1][0] = 0x530000;
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32)
                                                            .wrapping_add(0x66e8));
                                                    c.g[A2][0] =
                                                        sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                                                    c.g[A0][0] = 0x440000;
                                                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
                                                    c.g[T0][0] = c
                                                        .lw((c.g[A1][0] as u32).wrapping_add(0x84));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x28f0));
                                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                    c.g[A1][0] = 0x3;
                                                    c.g[A3][0] =
                                                        c.lw((c.g[T0][0] as u32).wrapping_add(0xc));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                    c.sw(
                                                        (c.g[GP][0] as u32).wrapping_sub(0x9f4),
                                                        c.g[A1][0],
                                                    );
                                                    c.g[V0][0] = c
                                                        .lw((c.g[A3][0] as u32).wrapping_add(0x40));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    c.g[A1][0] = c.lw((c.g[V1][0] as u32));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x18));
                                                    c.sw(
                                                        (c.g[GP][0] as u32).wrapping_sub(0x6a0),
                                                        c.g[A1][0],
                                                    );
                                                    c.g[RA][0] = sx(0x0020dab0);
                                                    c.sw(
                                                        (c.g[GP][0] as u32).wrapping_sub(0x69c),
                                                        c.g[A2][0],
                                                    );
                                                    call_ptr(c, 0x001253d0, 0x0020dab0)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    if t {
                                                        break 'b_20dbb4;
                                                    } else {
                                                        break 'b_20dab8;
                                                    }
                                                }
                                                c.cycles += 16;
                                                c.g[V1][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                c.g[A1][0] = 0x470000;
                                                c.sw((c.g[S1][0] as u32).wrapping_add(0x54), 0u64);
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x640));
                                                c.g[A0][0] = 0x27c;
                                                c.g[A2][0] = 0x100;
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                                                c.g[A3][0] = 0u64;
                                                c.g[RA][0] = sx(0x0020dae0);
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                                                call_ptr(c, 0x00317d70, 0x0020dae0)?;
                                                c.g[A2][0] =
                                                    c.lbu((c.g[S1][0] as u32).wrapping_add(0x14));
                                                c.g[A3][0] = 0x18;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                                c.g[RA][0] = sx(0x0020daf4);
                                                c.g[A0][0] = c.g[V0][0];
                                                Callee {
                                                    addr: 0x00211088,
                                                    f: ee::s05::OVScreenProfileSavePDA_ctor,
                                                }
                                                .call(c, 0x0020daf4)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                                break 'b_20daf8;
                                            }
                                            c.cycles += 11;
                                            c.g[A3][0] = 0xffffffffffffc0ff;
                                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[V1][0] = c.g[V1][0] & c.g[A3][0];
                                            c.g[A2][0] = 0u64;
                                            c.g[V1][0] = c.g[V1][0] | 0x100;
                                            c.g[A3][0] = 0u64;
                                            c.g[RA][0] = sx(0x0020db1c);
                                            c.sw(
                                                (c.g[V0][0] as u32).wrapping_add(0x1c),
                                                c.g[V1][0],
                                            );
                                            call_ptr(c, 0x0039f320, 0x0020db1c)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_20dbb4;
                                            } else {
                                                break 'b_20db24;
                                            }
                                        }
                                    }
                                    'b_20db4c: {
                                        c.cycles += 4;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                                        c.g[V0][0] = 0x6;
                                        if c.g[V1][0] == c.g[V0][0] {
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0x54),
                                                c.g[A1][0],
                                            );
                                            break 'b_20db4c;
                                        } else {
                                            c.cycles += 6;
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                            c.g[A1][0] = 0x1;
                                            c.g[RA][0] = sx(0x0020db44);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            call_ptr(c, 0x0039f190, 0x0020db44)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_20dbb4;
                                            } else {
                                                break 'b_20db4c;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    break 'b_20db50;
                                }
                                c.cycles += 1;
                                c.g[A1][0] = 0u64;
                                break 'b_20db54;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x0020db5c);
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                            call_ptr(c, 0x00230488, 0x0020db5c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                            if t {
                                break 'b_20dbb4;
                            } else {
                                break 'b_20db64;
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] == c.g[S0][0];
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        if t {
                            break 'b_20dbb4;
                        } else {
                            'b_20dba0: {
                                c.cycles += 10;
                                c.g[A0][0] = 0x470000;
                                c.g[RA][0] = sx(0x0020db78);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                                call_ptr(c, 0x00317670, 0x0020db78)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x0020db84);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039c778, 0x0020db84)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = 0xffff;
                                let t = c.g[A1][0] == c.g[V0][0];
                                c.g[A1][0] = c.g[A1][0] & 0xffff;
                                if t {
                                    break 'b_20dba0;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x0020dba0);
                                    c.g[A2][0] = 0x1;
                                    call_ptr(c, 0x0039c870, 0x0020dba0)?;
                                    break 'b_20dba0;
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[A1][0] = 0x1;
                            c.g[RA][0] = sx(0x0020dbb0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039f190, 0x0020dbb0)?;
                            break 'b_20dbb0;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_20dbb4;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_20dbb8;
        }
        c.cycles += 6;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_COUNT, LOG_RESULTS};
    use crate::difftest::{check, elf_image, fresh, run_generated, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (inside the machine's own RAM, away from
    /// the fakes' call log at `DATA + 0x100000`).
    const OC983_CTRL: u32 = DATA + 0x20_0000;
    const OC983_MAGIC: u32 = 0x0983_0983;
    /// Bit n set: fake call n returns 0 (the "failed / null" branches).
    const OC983_ZMASK: u32 = OC983_CTRL + 4;
    const OC983_SELF: u32 = DATA + 0x1000;
    const OC983_WIDGET: u32 = DATA + 0x2000;
    const OC983_GAME: u32 = DATA + 0x3000;
    const OC983_VT: u32 = DATA + 0x3800;

    /// A fake callee: logs like `fakes::fake_call`, then zeroes `v0` when the
    /// fixture's mask asks for it for this call index.
    fn oc983_call(c: &mut Cpu, addr: u32) {
        let n = c.bus.read32(LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc == ra && n < 32 && (c.bus.read32(OC983_ZMASK) >> n) & 1 != 0 {
            c.g[V0][0] = 0;
        }
    }

    fn oc983_miss(c: &mut Cpu, target: u32) {
        oc983_call(c, target);
    }

    macro_rules! oc983_fakes {
        ($install:ident; $(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC983_CTRL) != OC983_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc983_call(c, $addr);
                }
            )*
            fn $install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc983_fakes!(
        oc983_install;
        (oc983_fake_0011de60, OC983_PREV_0011DE60, ee::s00::OVERRIDE_0011DE60, ee::s00::f_0011de60__generated, 0x0011de60),
        (oc983_fake_0011df18, OC983_PREV_0011DF18, ee::s00::OVERRIDE_0011DF18, ee::s00::f_0011df18__generated, 0x0011df18),
        (oc983_fake_001253d0, OC983_PREV_001253D0, ee::s00::OVERRIDE_001253D0, ee::s00::f_001253d0__generated, 0x001253d0),
        (oc983_fake_00128a10, OC983_PREV_00128A10, ee::s00::OVERRIDE_00128A10, ee::s00::f_00128a10__generated, 0x00128a10),
        (oc983_fake_00144df0, OC983_PREV_00144DF0, ee::s01::OVERRIDE_00144DF0, ee::s01::f_00144df0__generated, 0x00144df0),
        (oc983_fake_001451e8, OC983_PREV_001451E8, ee::s01::OVERRIDE_001451E8, ee::s01::f_001451e8__generated, 0x001451e8),
        (oc983_fake_0014dc80, OC983_PREV_0014DC80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated, 0x0014dc80),
        (oc983_fake_0014dd58, OC983_PREV_0014DD58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated, 0x0014dd58),
        (oc983_fake_0014de68, OC983_PREV_0014DE68, ee::s01::OVERRIDE_0014DE68, ee::s01::f_0014de68__generated, 0x0014de68),
        (oc983_fake_001d8240, OC983_PREV_001D8240, ee::s04::OVERRIDE_001D8240, ee::s04::f_001d8240__generated, 0x001d8240),
        (oc983_fake_001d97e0, OC983_PREV_001D97E0, ee::s04::OVERRIDE_001D97E0, ee::s04::f_001d97e0__generated, 0x001d97e0),
        (oc983_fake_00208bb8, OC983_PREV_00208BB8, ee::s05::OVERRIDE_00208BB8, ee::s05::f_00208bb8__generated, 0x00208bb8),
        (oc983_fake_00208f88, OC983_PREV_00208F88, ee::s05::OVERRIDE_00208F88, ee::s05::f_00208f88__generated, 0x00208f88),
        (oc983_fake_002095e8, OC983_PREV_002095E8, ee::s05::OVERRIDE_002095E8, ee::s05::f_002095e8__generated, 0x002095e8),
        (oc983_fake_00209648, OC983_PREV_00209648, ee::s05::OVERRIDE_00209648, ee::s05::f_00209648__generated, 0x00209648),
        (oc983_fake_0020a088, OC983_PREV_0020A088, ee::s05::OVERRIDE_0020A088, ee::s05::f_0020a088__generated, 0x0020a088),
        (oc983_fake_0020a380, OC983_PREV_0020A380, ee::s05::OVERRIDE_0020A380, ee::s05::f_0020a380__generated, 0x0020a380),
        (oc983_fake_0020ab50, OC983_PREV_0020AB50, ee::s05::OVERRIDE_0020AB50, ee::s05::f_0020ab50__generated, 0x0020ab50),
        (oc983_fake_0020d110, OC983_PREV_0020D110, ee::s05::OVERRIDE_0020D110, ee::s05::f_0020d110__generated, 0x0020d110),
        (oc983_fake_00211088, OC983_PREV_00211088, ee::s05::OVERRIDE_00211088, ee::s05::f_00211088__generated, 0x00211088),
        (oc983_fake_00212138, OC983_PREV_00212138, ee::s05::OVERRIDE_00212138, ee::s05::f_00212138__generated, 0x00212138),
        (oc983_fake_00227f80, OC983_PREV_00227F80, ee::s05::OVERRIDE_00227F80, ee::s05::f_00227f80__generated, 0x00227f80),
        (oc983_fake_00230180, OC983_PREV_00230180, ee::s06::OVERRIDE_00230180, ee::s06::f_00230180__generated, 0x00230180),
        (oc983_fake_002302a8, OC983_PREV_002302A8, ee::s06::OVERRIDE_002302A8, ee::s06::f_002302a8__generated, 0x002302a8),
        (oc983_fake_00230488, OC983_PREV_00230488, ee::s06::OVERRIDE_00230488, ee::s06::f_00230488__generated, 0x00230488),
        (oc983_fake_00231250, OC983_PREV_00231250, ee::s06::OVERRIDE_00231250, ee::s06::f_00231250__generated, 0x00231250),
        (oc983_fake_00238160, OC983_PREV_00238160, ee::s06::OVERRIDE_00238160, ee::s06::f_00238160__generated, 0x00238160),
        (oc983_fake_00238348, OC983_PREV_00238348, ee::s06::OVERRIDE_00238348, ee::s06::f_00238348__generated, 0x00238348),
        (oc983_fake_00241710, OC983_PREV_00241710, ee::s06::OVERRIDE_00241710, ee::s06::f_00241710__generated, 0x00241710),
        (oc983_fake_00257610, OC983_PREV_00257610, ee::s06::OVERRIDE_00257610, ee::s06::f_00257610__generated, 0x00257610),
        (oc983_fake_002706f0, OC983_PREV_002706F0, ee::s07::OVERRIDE_002706F0, ee::s07::f_002706f0__generated, 0x002706f0),
        (oc983_fake_0028b180, OC983_PREV_0028B180, ee::s07::OVERRIDE_0028B180, ee::s07::f_0028b180__generated, 0x0028b180),
        (oc983_fake_0028fae0, OC983_PREV_0028FAE0, ee::s07::OVERRIDE_0028FAE0, ee::s07::f_0028fae0__generated, 0x0028fae0),
        (oc983_fake_0030b758, OC983_PREV_0030B758, ee::s10::OVERRIDE_0030B758, ee::s10::f_0030b758__generated, 0x0030b758),
        (oc983_fake_00317670, OC983_PREV_00317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x00317670),
        (oc983_fake_00317d70, OC983_PREV_00317D70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated, 0x00317d70),
        (oc983_fake_00397cc0, OC983_PREV_00397CC0, ee::s13::OVERRIDE_00397CC0, ee::s13::f_00397cc0__generated, 0x00397cc0),
        (oc983_fake_0039c778, OC983_PREV_0039C778, ee::s13::OVERRIDE_0039C778, ee::s13::f_0039c778__generated, 0x0039c778),
        (oc983_fake_0039c870, OC983_PREV_0039C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039c870),
        (oc983_fake_0039d860, OC983_PREV_0039D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039d860),
        (oc983_fake_0039e8b8, OC983_PREV_0039E8B8, ee::s13::OVERRIDE_0039E8B8, ee::s13::f_0039e8b8__generated, 0x0039e8b8),
        (oc983_fake_0039e948, OC983_PREV_0039E948, ee::s13::OVERRIDE_0039E948, ee::s13::f_0039e948__generated, 0x0039e948),
        (oc983_fake_0039f190, OC983_PREV_0039F190, ee::s13::OVERRIDE_0039F190, ee::s13::f_0039f190__generated, 0x0039f190),
        (oc983_fake_0039f320, OC983_PREV_0039F320, ee::s13::OVERRIDE_0039F320, ee::s13::f_0039f320__generated, 0x0039f320),
        (oc983_fake_003a0d00, OC983_PREV_003A0D00, ee::s13::OVERRIDE_003A0D00, ee::s13::f_003a0d00__generated, 0x003a0d00),
        (oc983_fake_003a0e90, OC983_PREV_003A0E90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated, 0x003a0e90),
        (oc983_fake_003a1000, OC983_PREV_003A1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated, 0x003a1000),
        (oc983_fake_00417828, OC983_PREV_00417828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated, 0x00417828),
    );

    /// A fixture word: 0, 1, a small number, a pointer into the filled area or a
    /// fake function address (so a call through a loaded pointer is a fake call).
    fn oc983_word(r: &mut Rng) -> u32 {
        match r.next_u32() % 8 {
            0..=2 => 0,
            3 => 1,
            4 => r.next_u32() % 16,
            5 | 6 => DATA + (r.next_u32() & 0x3ff0),
            _ => FAKE + 0x10 * (r.next_u32() % 4),
        }
    }

    /// Scrambled machine with the game image, this module's fakes (gate open, a
    /// random mask of calls that return 0), a sparse random data area
    /// (`DATA..DATA+0x4000`), some forced call results pointing into it and the
    /// game object at `gp - 0x848`.
    fn oc983_fixture(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc983_install();
        let mut r = machine(c, seed, salt, calls);
        elf_image(c);
        c.hooks.miss = oc983_miss;
        c.bus.write32(OC983_CTRL, OC983_MAGIC);
        let zero_mask = r.next_u32() & r.next_u32();
        c.bus.write32(OC983_ZMASK, zero_mask);
        for i in 0..0x1000u32 {
            let w = oc983_word(&mut r);
            c.bus.write32(DATA + 4 * i, w);
        }
        for n in 0..8u32 {
            if r.next_u32() & 1 != 0 {
                let p = DATA + (r.next_u32() & 0x3ff0);
                c.bus.write32(LOG_RESULTS + 4 * n, p);
            }
        }
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC983_GAME);
        r
    }

    /// The number of distinct cycle totals the generated body takes over the seeds.
    fn oc983_paths(f: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, f);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    /// `OVScreenYNDialog_Init(self)`: creates the dialog screen, runs the state
    /// overlay init and sets the button texts by the dialog type at `+0x9c`
    /// (a jump table of 7 types; larger types skip the texts).
    fn oc983_init_setup(c: &mut Cpu, seed: u32) {
        let _r = oc983_fixture(c, seed, 0x20d338, 14);
        c.pc = 0x0020_d338;
        c.g[A0][0] = sx(OC983_SELF);
        c.bus.write32(OC983_SELF + 0x9c, seed % 9);
    }

    #[test]
    fn ov_screen_yn_dialog_init_matches() {
        check(
            ee::s05::f_0020d338__generated,
            ov_screen_yn_dialog_init,
            300,
            &oc983_init_setup,
        );
        let n = oc983_paths(ee::s05::f_0020d338__generated, 300, &oc983_init_setup);
        assert!(n >= 6, "only {n} distinct paths");
    }

    /// `OVScreenSession_OnWidgetAction(self, widget, event)`: a map screen's
    /// next / previous / focus events, with the session popup open or closed
    /// and the widget's misc data (`+0x18`) 0 or not; events outside 1..=7 and
    /// a null widget leave at once.
    fn oc983_session_setup(c: &mut Cpu, seed: u32) {
        let _r = oc983_fixture(c, seed, 0x208c28, 30);
        c.pc = 0x0020_8c28;
        c.g[A0][0] = sx(OC983_SELF);
        let widget = if seed >= 36 && seed % 7 == 0 {
            0
        } else {
            OC983_WIDGET
        };
        c.g[A1][0] = sx(widget);
        c.g[A2][0] = sx(seed % 9);
        c.bus.write32(OC983_SELF + 0xbc, (seed / 9) % 2);
        c.bus.write32(OC983_WIDGET + 0x18, (seed / 18) % 2);
    }

    #[test]
    fn ov_screen_session_on_widget_action_matches() {
        check(
            ee::s05::f_00208c28__generated,
            ov_screen_session_on_widget_action,
            300,
            &oc983_session_setup,
        );
        let n = oc983_paths(ee::s05::f_00208c28__generated, 300, &oc983_session_setup);
        assert!(n >= 8, "only {n} distinct paths");
    }

    /// `OVScreenYNDialog_OnWidgetAction(self, widget, event)`: events 5 (a
    /// button chosen: by the widget's misc data and the dialog type at `+0x9c`
    /// restart, quit, save, ... the race) and 6 (back), with the game event
    /// kind (`gp - 0x69c`) and the pause-dialog flag (`gp - 0x9e8`) varying.
    fn oc983_yn_action_setup(c: &mut Cpu, seed: u32) {
        let _r = oc983_fixture(c, seed, 0x20d590, 40);
        c.pc = 0x0020_d590;
        c.g[A0][0] = sx(OC983_SELF);
        let widget = if seed >= 216 && seed % 5 == 0 {
            0
        } else {
            OC983_WIDGET
        };
        c.g[A1][0] = sx(widget);
        c.g[A2][0] = sx([5, 5, 5, 5, 5, 5, 6, 7, 0][(seed % 9) as usize]);
        c.bus.write32(OC983_WIDGET + 0x18, (seed / 9) % 3);
        c.bus.write32(OC983_SELF + 0x9c, (seed / 27) % 8);
        c.bus
            .write32(GAME_GP.wrapping_sub(0x69c), (seed / 5) % 8 + 5);
        c.bus.write32(GAME_GP.wrapping_sub(0x9e8), (seed / 2) % 2);
    }

    #[test]
    fn ov_screen_yn_dialog_on_widget_action_matches() {
        check(
            ee::s05::f_0020d590__generated,
            ov_screen_yn_dialog_on_widget_action,
            300,
            &oc983_yn_action_setup,
        );
        let n = oc983_paths(ee::s05::f_0020d590__generated, 300, &oc983_yn_action_setup);
        assert!(n >= 10, "only {n} distinct paths");
    }

    /// `OVScreenSaveReplay_v26(self)`: the save-replay screen's status display
    /// (memory card present / formatted / full texts through `sprintf`), reading
    /// the memory-card manager and the screen's vtable calls.
    fn oc983_save_replay_setup(c: &mut Cpu, seed: u32) {
        let _r = oc983_fixture(c, seed, 0x1fdf40, 30);
        c.pc = 0x001f_df40;
        c.g[A0][0] = sx(OC983_SELF);
        c.bus.write32(OC983_SELF + 8, OC983_VT);
    }

    #[test]
    fn ov_screen_save_replay_v26_matches() {
        check(
            ee::s05::f_001fdf40__generated,
            ov_screen_save_replay_v26,
            300,
            &oc983_save_replay_setup,
        );
        let n = oc983_paths(
            ee::s05::f_001fdf40__generated,
            300,
            &oc983_save_replay_setup,
        );
        assert!(n >= 6, "only {n} distinct paths");
    }
}
