//! Staging module oc984 (wave wave259): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc984`.

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
        0x0020_df38,
        ov_screen_replay_menu_handle_input,
        "OVScreenReplayMenu_HandleInput",
    ),
    (
        0x0020_e6b0,
        ov_screen_replay_menu_update_camera_text,
        "OVScreenReplayMenu_UpdateCameraText",
    ),
];

/// `OVScreenReplayMenu_HandleInput` (0x0020df38): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_replay_menu_handle_input(c: &mut Cpu) {
    if c.pc != 0x0020df38 {
        return ee::s05::f_0020df38__generated(c);
    }
    guest(c, |c| {
        'b_20e518: {
            'b_20e514: {
                'b_20e4f0: {
                    'b_20e4ec: {
                        'b_20e4e4: {
                            c.cycles += 17;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                            c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[S0][0] = c.g[A0][0];
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[S1][0] = c.g[A1][0];
                            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                            c.g[S2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x84));
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x14));
                            c.g[A1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
                            let t = c.g[A1][0] != c.g[V0][0];
                            if c.g[V1][0] != 0 {
                                c.g[S2][0] = c.g[V1][0];
                            }
                            if t {
                                break 'b_20e4e4;
                            } else {
                                'b_20e050: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x9c));
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_20e050;
                                    } else {
                                        'b_20e044: {
                                            'b_20dfe0: {
                                                c.cycles += 7;
                                                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[A3][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[A1][0] = 0xca;
                                                c.g[RA][0] = sx(0x0020df9c);
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xb0));
                                                call_ptr(c, 0x00320c48, 0x0020df9c)?;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x9c));
                                                    break 'b_20dfe0;
                                                } else {
                                                    'b_20dfc0: {
                                                        c.cycles += 4;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xa4));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xb0));
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[V0][0] = 0x4;
                                                        if t {
                                                            break 'b_20dfc0;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xa8));
                                                            if c.g[V1][0] != c.g[V0][0] {
                                                                c.g[V1][0] = 0xfffffffffffffffc;
                                                                break 'b_20e044;
                                                            } else {
                                                                break 'b_20dfc0;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 8;
                                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                                    c.g[A1][0] = 0x2;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x10));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x0020dfd8);
                                                    c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x0020dfd8)?;
                                                    let t = true;
                                                    c.g[V1][0] = 0xfffffffffffffffc;
                                                    if t {
                                                        break 'b_20e044;
                                                    } else {
                                                        break 'b_20dfe0;
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_20e050;
                                            } else {
                                                c.cycles += 9;
                                                c.g[V1][0] =
                                                    c.lbu((c.g[S1][0] as u32).wrapping_add(0x4));
                                                c.g[A1][0] = 0xcb;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                    .wrapping_add((c.g[V1][0] as u32)));
                                                c.g[RA][0] = sx(0x0020e004);
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xb0));
                                                call_ptr(c, 0x00320c48, 0x0020e004)?;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    break 'b_20e050;
                                                } else {
                                                    'b_20e028: {
                                                        c.cycles += 4;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xa4));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xac));
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[V0][0] = 0xfffffffffffffffc;
                                                        if t {
                                                            break 'b_20e028;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xa8));
                                                            let t = c.g[V1][0] != c.g[V0][0];
                                                            c.g[V1][0] = 0x4;
                                                            if t {
                                                                break 'b_20e044;
                                                            } else {
                                                                break 'b_20e028;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 7;
                                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                                    c.g[A1][0] = 0x2;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x10));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x0020e040);
                                                    c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x0020e040)?;
                                                    c.g[V1][0] = 0x4;
                                                    break 'b_20e044;
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x1;
                                        let t = true;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xa8), c.g[V1][0]);
                                        if t {
                                            break 'b_20e514;
                                        } else {
                                            break 'b_20e050;
                                        }
                                    }
                                }
                                'b_20e438: {
                                    'b_20e380: {
                                        'b_20e100: {
                                            'b_20e0f4: {
                                                c.cycles += 6;
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x0020e060);
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x0020e060)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x9c));
                                                if t {
                                                    break 'b_20e0f4;
                                                } else {
                                                    c.cycles += 2;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x8));
                                                        break 'b_20e100;
                                                    } else {
                                                        c.cycles += 33;
                                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                                        c.g[A1][0] = 0x6;
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x14));
                                                        let t = c.g[V1][0] as u32;
                                                        c.g[RA][0] = sx(0x0020e088);
                                                        c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        call_ptr(c, t, 0x0020e088)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x69c));
                                                        c.g[A1][0] = 0x440000;
                                                        c.g[A2][0] = c.lw((c.g[A1][0] as u32)
                                                            .wrapping_add(0x28f8));
                                                        c.g[V1][0] = 0x530000;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add(0x66e8));
                                                        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                                                        c.sw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x6a0),
                                                            c.g[A2][0],
                                                        );
                                                        c.g[A1][0] = 0x2;
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[V1][0] as u32)));
                                                        c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                        c.g[V1][0] = 0x1;
                                                        c.sw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x69c),
                                                            c.g[A0][0],
                                                        );
                                                        c.g[A0][0] = 0x470000;
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x9c),
                                                            c.g[V1][0],
                                                        );
                                                        c.g[RA][0] = sx(0x0020e0cc);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x1be0));
                                                        call_ptr(c, 0x00317670, 0x0020e0cc)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x0020e0d8);
                                                        c.g[A1][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0039d860, 0x0020e0d8)?;
                                                        c.g[A1][0] = c.g[V0][0];
                                                        c.g[A0][0] = 0xfffffffffffffff7;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[A1][0] as u32)
                                                                .wrapping_add(0x90));
                                                        c.g[V0][0] = 0x1;
                                                        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[A1][0] as u32).wrapping_add(0x90),
                                                            c.g[V1][0],
                                                        );
                                                        if t {
                                                            break 'b_20e514;
                                                        } else {
                                                            break 'b_20e0f4;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[V1][0] =
                                                    c.lbu((c.g[S1][0] as u32).wrapping_add(0x4));
                                                break 'b_20e380;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_20e100;
                                            }
                                        }
                                        'b_20e2c8: {
                                            c.cycles += 6;
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x0020e110);
                                            c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x0020e110)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x6;
                                            if t {
                                                break 'b_20e2c8;
                                            } else {
                                                'b_20e2bc: {
                                                    'b_20e288: {
                                                        'b_20e1c8: {
                                                            c.cycles += 16;
                                                            c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x0020e12c);
                                                            c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x0020e12c)?;
                                                            c.g[A0][0] = 0x470000;
                                                            c.g[RA][0] = sx(0x0020e138);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1be0));
                                                            call_ptr(c, 0x00317670, 0x0020e138)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[RA][0] = sx(0x0020e144);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x0039d860, 0x0020e144)?;
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xa0));
                                                            c.g[V0][0] = 0x1;
                                                            c.g[V1][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x18));
                                                            let t = c.g[V1][0] == c.g[V0][0];
                                                            c.g[V0][0] =
                                                                ((c.g[V1][0] as i64) < 2i64) as u64;
                                                            if t {
                                                                break 'b_20e1c8;
                                                            } else {
                                                                'b_20e180: {
                                                                    'b_20e170: {
                                                                        c.cycles += 2;
                                                                        if c.g[V0][0] == 0u64 {
                                                                            c.g[V0][0] = 0x2;
                                                                            break 'b_20e170;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t =
                                                                                c.g[V1][0] == 0u64;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0x848,
                                                                                ));
                                                                            if t {
                                                                                break 'b_20e180;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = true;
                                                                                c.g[V0][0] = 0x1;
                                                                                if t {
                                                                                    break 'b_20e514;
                                                                                } else {
                                                                                    break 'b_20e170;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 2;
                                                                    let t =
                                                                        c.g[V1][0] == c.g[V0][0];
                                                                    c.g[V0][0] = 0x1;
                                                                    if t {
                                                                        break 'b_20e288;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x30),
                                                                        );
                                                                        if t {
                                                                            break 'b_20e518;
                                                                        } else {
                                                                            break 'b_20e180;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 18;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x84));
                                                                c.g[RA][0] = sx(0x0020e18c);
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                call_ptr(
                                                                    c, 0x0026f980, 0x0020e18c,
                                                                )?;
                                                                c.g[RA][0] = sx(0x0020e194);
                                                                c.g[A0][0] = 0x12;
                                                                Callee {
                                                                    addr: 0x0020ab50,
                                                                    f: ee::s05::GameUI_OpenScreen,
                                                                }
                                                                .call(c, 0x0020e194)?;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x69c));
                                                                c.g[A0][0] = 0x530000;
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x66e8));
                                                                c.g[A1][0] = 0x440000;
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x1));
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x28f0));
                                                                c.g[V1][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x69c),
                                                                    c.g[V0][0],
                                                                );
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                c.g[V0][0] =
                                                                    c.lw((c.g[V1][0] as u32));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                let t = true;
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A1][0] as u32),
                                                                    ));
                                                                if t {
                                                                    break 'b_20e2bc;
                                                                } else {
                                                                    break 'b_20e1c8;
                                                                }
                                                            }
                                                        }
                                                        'b_20e210: {
                                                            c.cycles += 14;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A1][0] = 0x1;
                                                            c.g[RA][0] = sx(0x0020e1d8);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            call_ptr(c, 0x0039f190, 0x0020e1d8)?;
                                                            c.g[A0][0] = 0x470000;
                                                            c.g[RA][0] = sx(0x0020e1e4);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x1fb0));
                                                            call_ptr(c, 0x00317670, 0x0020e1e4)?;
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[RA][0] = sx(0x0020e1f0);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x0039c778, 0x0020e1f0)?;
                                                            c.g[A1][0] = c.g[V0][0];
                                                            c.g[V0][0] = 0xffff;
                                                            let t = c.g[A1][0] == c.g[V0][0];
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                            if t {
                                                                break 'b_20e210;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x40));
                                                                c.g[RA][0] = sx(0x0020e20c);
                                                                c.g[A2][0] = 0x1;
                                                                call_ptr(
                                                                    c, 0x0039c870, 0x0020e20c,
                                                                )?;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                break 'b_20e210;
                                                            }
                                                        }
                                                        'b_20e238: {
                                                            c.cycles += 6;
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x84));
                                                            c.g[RA][0] = sx(0x0020e21c);
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x28));
                                                            call_ptr(c, 0x0026f980, 0x0020e21c)?;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xb4));
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                            if t {
                                                                break 'b_20e238;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x84));
                                                                c.g[RA][0] = sx(0x0020e234);
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x28));
                                                                call_ptr(
                                                                    c, 0x002706b8, 0x0020e234,
                                                                )?;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                break 'b_20e238;
                                                            }
                                                        }
                                                        'b_20e258: {
                                                            c.cycles += 5;
                                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x84));
                                                            c.g[V1][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x28));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x608));
                                                            if c.g[V0][0] == 0u64 {
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x69c));
                                                                break 'b_20e258;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[RA][0] = sx(0x0020e254);
                                                                c.g[A1][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x00230488, 0x0020e254,
                                                                )?;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x69c));
                                                                break 'b_20e258;
                                                            }
                                                        }
                                                        c.cycles += 12;
                                                        c.g[A0][0] = 0x530000;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x66e8));
                                                        c.g[A1][0] = 0x440000;
                                                        c.g[V0][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x1));
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x28f0));
                                                        c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.sw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x69c),
                                                            c.g[V0][0],
                                                        );
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        let t = true;
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add((c.g[A1][0] as u32)));
                                                        if t {
                                                            break 'b_20e2bc;
                                                        } else {
                                                            break 'b_20e288;
                                                        }
                                                    }
                                                    c.cycles += 13;
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x69c)
                                                        );
                                                    c.g[V0][0] = 0x530000;
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x9c),
                                                        0u64,
                                                    );
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x66e8));
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                                                    c.g[A0][0] = 0x440000;
                                                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x28f0));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.sw(
                                                        (c.g[GP][0] as u32).wrapping_sub(0x69c),
                                                        c.g[A1][0],
                                                    );
                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    break 'b_20e2bc;
                                                }
                                                c.cycles += 3;
                                                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                let t = true;
                                                c.sw(
                                                    (c.g[GP][0] as u32).wrapping_sub(0x6a0),
                                                    c.g[V1][0],
                                                );
                                                if t {
                                                    break 'b_20e4e4;
                                                } else {
                                                    break 'b_20e2c8;
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x9c));
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V1][0] =
                                                c.lbu((c.g[S1][0] as u32).wrapping_add(0x4));
                                            break 'b_20e380;
                                        } else {
                                            'b_20e370: {
                                                c.cycles += 7;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x30));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x34));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x0020e2e8);
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x0020e2e8)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = 0x3;
                                                if t {
                                                    break 'b_20e370;
                                                } else {
                                                    c.cycles += 32;
                                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x10));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x0020e304);
                                                    c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x0020e304)?;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x69c)
                                                        );
                                                    c.g[A0][0] = 0x530000;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x66e8));
                                                    c.g[A1][0] = 0x440000;
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x28f0));
                                                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.sw(
                                                        (c.g[GP][0] as u32).wrapping_sub(0x69c),
                                                        c.g[V0][0],
                                                    );
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                    c.g[A0][0] = 0x470000;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x1be0));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add((c.g[A1][0] as u32)));
                                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                    c.sw(
                                                        (c.g[GP][0] as u32).wrapping_sub(0x6a0),
                                                        c.g[V1][0],
                                                    );
                                                    c.g[RA][0] = sx(0x0020e34c);
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x9c),
                                                        0u64,
                                                    );
                                                    call_ptr(c, 0x00317670, 0x0020e34c)?;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x0020e358);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, 0x0039d860, 0x0020e358)?;
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[V1][0] = c
                                                        .lw((c.g[A0][0] as u32).wrapping_add(0x90));
                                                    c.g[V0][0] = 0x1;
                                                    c.g[V1][0] = c.g[V1][0] | 0x8;
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[A0][0] as u32).wrapping_add(0x90),
                                                        c.g[V1][0],
                                                    );
                                                    if t {
                                                        break 'b_20e514;
                                                    } else {
                                                        break 'b_20e370;
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x9c));
                                            if c.g[V0][0] != 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                break 'b_20e438;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V1][0] =
                                                    c.lbu((c.g[S1][0] as u32).wrapping_add(0x4));
                                                break 'b_20e380;
                                            }
                                        }
                                    }
                                    'b_20e434: {
                                        c.cycles += 8;
                                        c.g[A1][0] = 0xcc;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                        c.g[RA][0] = sx(0x0020e398);
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xb0));
                                        call_ptr(c, 0x00320c48, 0x0020e398)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A1][0] = 0x6;
                                        if t {
                                            break 'b_20e434;
                                        } else {
                                            'b_20e3fc: {
                                                c.cycles += 9;
                                                c.g[V0][0] = c.lw((c.g[S2][0] as u32));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x0020e3b4);
                                                c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x0020e3b4)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                let t = c.g[V0][0] == 0u64;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xa0),
                                                    c.g[V0][0],
                                                );
                                                if t {
                                                    break 'b_20e3fc;
                                                } else {
                                                    c.cycles += 14;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x0020e3d4);
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x2720));
                                                    call_ptr(c, 0x0039e8b8, 0x0020e3d4)?;
                                                    c.g[A1][0] = 0x470000;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x0020e3e4);
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x1bc0));
                                                    call_ptr(c, 0x0039e8b8, 0x0020e3e4)?;
                                                    c.g[A1][0] = 0x470000;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x0020e3f4);
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x1bd0));
                                                    call_ptr(c, 0x0039e948, 0x0020e3f4)?;
                                                    let t = true;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_20e514;
                                                    } else {
                                                        break 'b_20e3fc;
                                                    }
                                                }
                                            }
                                            c.cycles += 14;
                                            c.g[A1][0] = 0x4a0000;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0020e40c);
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x2720));
                                            call_ptr(c, 0x0039e948, 0x0020e40c)?;
                                            c.g[A1][0] = 0x470000;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0020e41c);
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x1bc0));
                                            call_ptr(c, 0x0039e948, 0x0020e41c)?;
                                            c.g[A1][0] = 0x470000;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0020e42c);
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x1bd0));
                                            call_ptr(c, 0x0039e8b8, 0x0020e42c)?;
                                            let t = true;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_20e514;
                                            } else {
                                                break 'b_20e434;
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_20e438;
                                }
                                'b_20e494: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x0020e448);
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x0020e448)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_20e494;
                                    } else {
                                        c.cycles += 9;
                                        c.g[A0][0] = 0x470000;
                                        c.g[RA][0] = sx(0x0020e45c);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1be0));
                                        call_ptr(c, 0x00317670, 0x0020e45c)?;
                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                        c.g[RA][0] = sx(0x0020e468);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x0020e468)?;
                                        c.g[V1][0] = c.g[V0][0];
                                        if c.g[V1][0] == 0u64 {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x9c));
                                            break 'b_20e4f0;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xb4));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x1;
                                            if t {
                                                break 'b_20e4ec;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V1][0] =
                                                    c.lbu((c.g[V1][0] as u32).wrapping_add(0x95));
                                                if c.g[V1][0] != c.g[V0][0] {
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x9c));
                                                    break 'b_20e4f0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_20e518;
                                                    } else {
                                                        break 'b_20e494;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x0020e4a4);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x0020e4a4)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = 0x470000;
                                if t {
                                    break 'b_20e4ec;
                                } else {
                                    c.cycles += 8;
                                    c.g[RA][0] = sx(0x0020e4b4);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1be0));
                                    call_ptr(c, 0x00317670, 0x0020e4b4)?;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x0020e4c0);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x0020e4c0)?;
                                    c.g[V1][0] = c.g[V0][0];
                                    if c.g[V1][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x9c));
                                        break 'b_20e4f0;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb4));
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x2;
                                        if t {
                                            break 'b_20e4ec;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V1][0] =
                                                c.lbu((c.g[V1][0] as u32).wrapping_add(0x95));
                                            if c.g[V1][0] != c.g[V0][0] {
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x9c));
                                                break 'b_20e4f0;
                                            } else {
                                                break 'b_20e4e4;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = true;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_20e514;
                        } else {
                            break 'b_20e4ec;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x9c));
                    break 'b_20e4f0;
                }
                c.cycles += 2;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_20e514;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[RA][0] = sx(0x0020e50c);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    call_ptr(c, 0x0026fb88, 0x0020e50c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_20e518;
                    } else {
                        break 'b_20e514;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_20e518;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenReplayMenu_UpdateCameraText` (0x0020e6b0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_replay_menu_update_camera_text(c: &mut Cpu) {
    if c.pc != 0x0020e6b0 {
        return ee::s05::f_0020e6b0__generated(c);
    }
    guest(c, |c| {
        'b_20e804: {
            c.cycles += 13;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
            c.g[A0][0] = 0x4a0000;
            c.g[RA][0] = sx(0x0020e6cc);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2730));
            call_ptr(c, 0x00317670, 0x0020e6cc)?;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            c.g[RA][0] = sx(0x0020e6d8);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x0020e6d8)?;
            c.g[S0][0] = c.g[V0][0];
            let t = c.g[S0][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_20e804;
            } else {
                'b_20e7f8: {
                    'b_20e7e4: {
                        c.cycles += 6;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x630));
                        c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x470000;
                        if t {
                            break 'b_20e7e4;
                        } else {
                            'b_20e7d4: {
                                'b_20e7c4: {
                                    'b_20e7a8: {
                                        'b_20e798: {
                                            'b_20e780: {
                                                'b_20e774: {
                                                    'b_20e764: {
                                                        'b_20e748: {
                                                            'b_20e724: {
                                                                c.cycles += 6;
                                                                c.g[V1][0] =
                                                                    sx((c.g[V1][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1ca0));
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                c.g[A0][0] =
                                                                    c.lw((c.g[V1][0] as u32));
                                                                let t = c.g[A0][0] as u32;
                                                                match t {
                                                                    0x0020e714 => {
                                                                        c.cycles += 4;
                                                                        c.g[V0][0] = 0x470000;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1c40,
                                                                                ));
                                                                        let t = true;
                                                                        c.g[A0][0] = c.ld((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c40));
                                                                        if t {
                                                                            break 'b_20e780;
                                                                        } else {
                                                                            break 'b_20e724;
                                                                        }
                                                                    }
                                                                    0x0020e724 => {
                                                                        break 'b_20e724;
                                                                    }
                                                                    0x0020e748 => {
                                                                        break 'b_20e748;
                                                                    }
                                                                    0x0020e764 => {
                                                                        break 'b_20e764;
                                                                    }
                                                                    0x0020e774 => {
                                                                        break 'b_20e774;
                                                                    }
                                                                    0x0020e798 => {
                                                                        break 'b_20e798;
                                                                    }
                                                                    0x0020e7a8 => {
                                                                        break 'b_20e7a8;
                                                                    }
                                                                    0x0020e7c4 => {
                                                                        break 'b_20e7c4;
                                                                    }
                                                                    0x0020e7d4 => {
                                                                        break 'b_20e7d4;
                                                                    }
                                                                    _ => {
                                                                        c.jump_out(
                                                                            t, 0x0020e6b0,
                                                                            0x0020e814,
                                                                        );
                                                                        return Ok(());
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 9;
                                                            c.g[V0][0] = 0x470000;
                                                            c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1c50));
                                                            c.g[A0][0] = c.ld((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1c50));
                                                            c.g[A1][0] = c.lhu(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.sd((c.g[SP][0] as u32), c.g[A0][0]);
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x8),
                                                                c.g[V0][0],
                                                            );
                                                            let t = true;
                                                            c.sh(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                c.g[A1][0],
                                                            );
                                                            if t {
                                                                break 'b_20e7f8;
                                                            } else {
                                                                break 'b_20e748;
                                                            }
                                                        }
                                                        c.cycles += 7;
                                                        c.g[V0][0] = 0x470000;
                                                        c.g[V1][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x1c60));
                                                        c.g[A0][0] = c.ld((c.g[V0][0] as u32)
                                                            .wrapping_add(0x1c60));
                                                        c.g[A1][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.sd((c.g[SP][0] as u32), c.g[A0][0]);
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x8),
                                                            c.g[A1][0],
                                                        );
                                                        if t {
                                                            break 'b_20e7f8;
                                                        } else {
                                                            break 'b_20e764;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[V0][0] = 0x470000;
                                                    c.g[V1][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x1c70));
                                                    let t = true;
                                                    c.g[A0][0] = c
                                                        .ld((c.g[V0][0] as u32)
                                                            .wrapping_add(0x1c70));
                                                    if t {
                                                        break 'b_20e780;
                                                    } else {
                                                        break 'b_20e774;
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x470000;
                                                c.g[V1][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x1c80));
                                                c.g[A0][0] =
                                                    c.ld((c.g[V0][0] as u32).wrapping_add(0x1c80));
                                                break 'b_20e780;
                                            }
                                            c.cycles += 6;
                                            c.g[A1][0] =
                                                c.lbu((c.g[V1][0] as u32).wrapping_add(0xa));
                                            c.g[V0][0] =
                                                c.lhu((c.g[V1][0] as u32).wrapping_add(0x8));
                                            c.sd((c.g[SP][0] as u32), c.g[A0][0]);
                                            c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                            let t = true;
                                            c.sb((c.g[SP][0] as u32).wrapping_add(0xa), c.g[A1][0]);
                                            if t {
                                                break 'b_20e7f8;
                                            } else {
                                                break 'b_20e798;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = 0x4a0000;
                                        c.g[V1][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x2738));
                                        let t = true;
                                        c.sd((c.g[SP][0] as u32), c.g[V1][0]);
                                        if t {
                                            break 'b_20e7f8;
                                        } else {
                                            break 'b_20e7a8;
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x470000;
                                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c90));
                                    c.g[A0][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x1c90));
                                    c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x8));
                                    c.sd((c.g[SP][0] as u32), c.g[A0][0]);
                                    let t = true;
                                    c.sb((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                    if t {
                                        break 'b_20e7f8;
                                    } else {
                                        break 'b_20e7c4;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = 0x4a0000;
                                c.g[V1][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x2740));
                                let t = true;
                                c.sd((c.g[SP][0] as u32), c.g[V1][0]);
                                if t {
                                    break 'b_20e7f8;
                                } else {
                                    break 'b_20e7d4;
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = 0x4a0000;
                            c.g[V1][0] = c.ld((c.g[V0][0] as u32).wrapping_add(0x2748));
                            let t = true;
                            c.sd((c.g[SP][0] as u32), c.g[V1][0]);
                            if t {
                                break 'b_20e7f8;
                            } else {
                                break 'b_20e7e4;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2138));
                    c.g[RA][0] = sx(0x0020e7f8);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00417828, 0x0020e7f8)?;
                    break 'b_20e7f8;
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0020e804);
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x003a0c50, 0x0020e804)?;
                break 'b_20e804;
            }
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{elf_image, Rng, DATA, GAME_GP};
    use crate::game::main::loadscreens::tests::{oc736_fake_call, oc736_force, oc736_force_zero};
    use std::sync::atomic::AtomicUsize;

    /// The fakes' gate word and its magic (each machine of a diff has its own RAM).
    pub(crate) const OC984_CTRL: u32 = DATA + 0x2c_0000;
    pub(crate) const OC984_MAGIC: u32 = 0x4f43_3938;
    /// `InputBinding_EvalBool` answers: the word for button `0xca + n` at `+0x100 + 4n`.
    pub(crate) const OC984_PAD_ANS: u32 = OC984_CTRL + 0x100;
    pub(crate) const OC984_WORLD: u32 = DATA + 0x21_0000;
    pub(crate) const OC984_W84: u32 = DATA + 0x22_0000;
    pub(crate) const OC984_ENT: u32 = DATA + 0x23_0000;
    pub(crate) const OC984_THIS: u32 = DATA + 0x24_0000;
    pub(crate) const OC984_ENG: u32 = DATA + 0x25_0000;
    pub(crate) const OC984_SND: u32 = DATA + 0x26_0000;
    pub(crate) const OC984_SNDVT: u32 = DATA + 0x27_0000;
    pub(crate) const OC984_EV: u32 = DATA + 0x28_0000;
    pub(crate) const OC984_EVVT: u32 = DATA + 0x29_0000;
    pub(crate) const OC984_OBJ: u32 = DATA + 0x2a_0000;
    pub(crate) const OC984_SEL: u32 = DATA + 0x2b_0000;
    /// A real leaf that returns 0 (a vtable slot that answers "no").
    pub(crate) const OC984_ZERO_LEAF: u32 = 0x0014_06c0;

    macro_rules! oc984_fakes {
        ($install:ident; $(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC984_CTRL) != OC984_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc736_fake_call(c, $addr);
                }
            )*
            pub(crate) fn $install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc984_fakes!(
        oc984_install_generic;
        (oc984_fake_317670, OC984_PREV_317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x0031_7670),
        (oc984_fake_39d860, OC984_PREV_39D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039_d860),
        (oc984_fake_26f980, OC984_PREV_26F980, ee::s07::OVERRIDE_0026F980, ee::s07::f_0026f980__generated, 0x0026_f980),
        (oc984_fake_39f190, OC984_PREV_39F190, ee::s13::OVERRIDE_0039F190, ee::s13::f_0039f190__generated, 0x0039_f190),
        (oc984_fake_39c778, OC984_PREV_39C778, ee::s13::OVERRIDE_0039C778, ee::s13::f_0039c778__generated, 0x0039_c778),
        (oc984_fake_39c870, OC984_PREV_39C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039_c870),
        (oc984_fake_2706b8, OC984_PREV_2706B8, ee::s07::OVERRIDE_002706B8, ee::s07::f_002706b8__generated, 0x0027_06b8),
        (oc984_fake_230488, OC984_PREV_230488, ee::s06::OVERRIDE_00230488, ee::s06::f_00230488__generated, 0x0023_0488),
        (oc984_fake_39e8b8, OC984_PREV_39E8B8, ee::s13::OVERRIDE_0039E8B8, ee::s13::f_0039e8b8__generated, 0x0039_e8b8),
        (oc984_fake_39e948, OC984_PREV_39E948, ee::s13::OVERRIDE_0039E948, ee::s13::f_0039e948__generated, 0x0039_e948),
        (oc984_fake_26fb88, OC984_PREV_26FB88, ee::s07::OVERRIDE_0026FB88, ee::s07::f_0026fb88__generated, 0x0026_fb88),
        (oc984_fake_20ab50, OC984_PREV_20AB50, ee::s05::OVERRIDE_0020AB50, ee::s05::f_0020ab50__generated, 0x0020_ab50),
        (oc984_fake_3a0c50, OC984_PREV_3A0C50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated, 0x003a_0c50),
        (oc984_fake_417828, OC984_PREV_417828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated, 0x0041_7828),
    );

    static OC984_PREV_320C48: AtomicUsize = AtomicUsize::new(0);

    /// `InputBinding_EvalBool(pad, button)`: the fake call, then the answer the
    /// fixture stored for the button (0 or 1), so a branch can see "not pressed".
    fn oc984_fake_320c48(c: &mut Cpu) {
        if c.bus.read32(OC984_CTRL) != OC984_MAGIC {
            return fakes::fall_back(c, &OC984_PREV_320C48, ee::s10::f_00320c48__generated);
        }
        let a1 = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, 0x0032_0c48);
        if c.pc == ra {
            let w = c
                .bus
                .read32(OC984_PAD_ANS + 4 * (a1.wrapping_sub(0xca) & 7));
            c.g[V0][0] = w as u64;
        }
    }

    pub(crate) fn oc984_install_fakes() {
        oc984_install_generic();
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00320C48),
            oc984_fake_320c48,
            &OC984_PREV_320C48,
        );
    }

    /// One vtable slot of the input event at `off`: a small delta (`this` adjust),
    /// and either the zero leaf or a fake method.
    fn oc984_slot(c: &mut Cpu, r: &mut Rng, off: u32, n: u32) {
        let delta = [0u16, 4, 0x10, 0xfffc][(r.next_u32() % 4) as usize];
        c.bus.write16(OC984_EVVT + off, delta);
        let f = if r.next_u32() % 2 == 0 {
            OC984_ZERO_LEAF
        } else {
            FAKE + 0x10 * n
        };
        c.bus.write32(OC984_EVVT + off + 4, f);
    }

    /// `OVScreenReplayMenu_HandleInput(this, input)`: the replay menu's input
    /// handler. The input's port against the replay's, the three pad buttons
    /// (`0xca`, `0xcb`, `0xcc`: scrub, hide the time line), the popup menu
    /// (start, next, previous, up, down events through the input's vtable) with
    /// the selected entry 0/1/2, the pause stack, and the final
    /// `Replay_HandleInput`. Sound object: the engine's, or the null one.
    fn oc984_input_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x20_df38, 24);
        c.bus.write32(OC984_CTRL, OC984_MAGIC);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC984_WORLD);
        c.bus.write32(OC984_WORLD + 0x84, OC984_W84);
        c.bus.write32(OC984_W84 + 0x28, OC984_ENT);
        let port = r.next_u32() % 4;
        c.bus.write32(OC984_ENT + 0x28, port);
        c.bus.write32(OC984_ENT + 0x608, r.next_u32() & 1);
        for p in 0..4u32 {
            c.bus
                .write32(OC984_WORLD + 0xb0 + 4 * p, OC984_ENT + 0x100 * p);
        }
        for n in 0..3u32 {
            c.bus.write32(OC984_PAD_ANS + 4 * n, r.next_u32() & 1);
        }
        // The input event: port, vtable with isStart/isNext/isPrevious/isUp/isDown.
        let ev_port = if r.next_u32() % 4 == 0 {
            (port + 1) % 4
        } else {
            port
        };
        c.bus.write8(OC984_EV + 4, ev_port as u8);
        c.bus.write32(OC984_EV + 8, OC984_EVVT);
        for (n, off) in [0x18u32, 0x28, 0x30, 0x88, 0x90].into_iter().enumerate() {
            oc984_slot(c, &mut r, off, n as u32 + 1);
        }
        // The sound object: the engine's (`+0x14`) or the null one at gp+0x2968.
        c.bus.write32(OC984_THIS + 0x10, OC984_ENG);
        c.bus.write32(
            OC984_ENG + 0x14,
            if r.next_u32() % 2 == 0 { OC984_SND } else { 0 },
        );
        c.bus.write32(OC984_SND, OC984_SNDVT);
        c.bus.write32(GAME_GP + 0x2968, OC984_SNDVT);
        c.bus
            .write16(OC984_SNDVT + 0x10, [0u16, 8][(r.next_u32() % 2) as usize]);
        c.bus.write32(OC984_SNDVT + 0x14, FAKE + 0x20);
        // The screen state.
        c.bus.write32(OC984_THIS + 0x40, DATA + 0x2d_0000);
        c.bus.write32(OC984_THIS + 0x9c, r.next_u32() & 1);
        c.bus.write32(OC984_THIS + 0xa0, r.next_u32() & 1);
        c.bus.write32(OC984_THIS + 0xa4, 1 + r.next_u32() % 2);
        c.bus.write32(
            OC984_THIS + 0xa8,
            [4u32, 0xffff_fffc, 0][(r.next_u32() % 3) as usize],
        );
        c.bus.write32(OC984_THIS + 0xac, 1 + r.next_u32() % 2);
        c.bus.write32(OC984_THIS + 0xb0, 1 + r.next_u32() % 2);
        c.bus.write32(OC984_THIS + 0xb4, r.next_u32() & 1);
        // The pause stack (gp-0x69c: index; 0x5366e8: entries) and its flag tables.
        c.bus
            .write32(GAME_GP.wrapping_sub(0x69c), 1 + r.next_u32() % 3);
        for i in 0..8u32 {
            c.bus.write32(0x0053_66e8 + 4 * i, r.next_u32() % 4);
        }
        for i in 0..4u32 {
            c.bus.write32(0x0044_28f0 + 4 * i, r.next_u32());
        }
        // The menu object (`getObjectByHashName`) with its selected entry.
        if r.next_u32() % 5 == 0 {
            oc736_force_zero(c, 0x0039_d860);
        } else {
            oc736_force(c, 0x0039_d860, OC984_OBJ);
        }
        c.bus.write32(OC984_OBJ + 0x90, r.next_u32());
        c.bus.write32(OC984_OBJ + 0xa0, OC984_SEL);
        c.bus.write32(OC984_SEL + 0x18, r.next_u32() % 4);
        c.bus.write8(OC984_OBJ + 0x95, (r.next_u32() % 3) as u8);
        if r.next_u32() % 2 == 0 {
            oc736_force(c, 0x0039_c778, 0xffff);
        } else {
            oc736_force(c, 0x0039_c778, 7);
        }
        c.g[A0][0] = sx(OC984_THIS);
        c.g[A1][0] = sx(OC984_EV);
        c.pc = 0x0020_df38;
    }

    /// The replay menu's input handler against the generated body (real
    /// callees faked; some of them unwind).
    #[test]
    fn ov_screen_replay_menu_handle_input_matches() {
        oc984_install_fakes();
        covered(
            ee::s05::f_0020df38__generated,
            ov_screen_replay_menu_handle_input,
            600,
            &oc984_input_setup,
        );
    }

    /// `OVScreenReplayMenu_UpdateCameraText(this)`: sets the camera name text of
    /// the replay menu: nine fixed names by camera mode (jump table in the game
    /// image), `sprintf` for the rest; no text object: nothing.
    fn oc984_camera_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x20_e6b0, 5);
        elf_image(c);
        c.bus.write32(OC984_CTRL, OC984_MAGIC);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC984_WORLD);
        c.bus.write32(OC984_WORLD + 0x84, OC984_W84);
        c.bus.write32(OC984_W84 + 0x28, OC984_ENT);
        let cam = if seed % 13 == 12 {
            r.next_u32()
        } else {
            seed % 13
        };
        c.bus.write32(OC984_ENT + 0x630, cam);
        c.bus.write32(OC984_THIS + 0x40, DATA + 0x2d_0000);
        if seed % 5 == 0 {
            oc736_force_zero(c, 0x0039_d860);
        } else {
            oc736_force(c, 0x0039_d860, OC984_OBJ);
        }
        c.g[A0][0] = sx(OC984_THIS);
        c.pc = 0x0020_e6b0;
    }

    #[test]
    fn ov_screen_replay_menu_update_camera_text_matches() {
        oc984_install_fakes();
        covered(
            ee::s05::f_0020e6b0__generated,
            ov_screen_replay_menu_update_camera_text,
            130,
            &oc984_camera_setup,
        );
    }
}
