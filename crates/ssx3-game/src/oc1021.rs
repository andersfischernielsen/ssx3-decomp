//! Staging module oc1021 (wave wave262): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1021`.

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
    0x0021_1bc8,
    ov_screen_profile_save_pda_on_widget_action,
    "OVScreenProfileSavePDA_OnWidgetAction",
)];

/// `OVScreenProfileSavePDA_OnWidgetAction` (0x00211bc8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_profile_save_pda_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x00211bc8 {
        return ee::s05::f_00211bc8__generated(c);
    }
    guest(c, |c| 'l_211bc8: loop {
        'b_212064: {
            'b_212060: {
                'b_21205c: {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                    if t {
                        break 'b_21205c;
                    } else {
                        'b_211fa8: {
                            'b_211f40: {
                                'b_211d14: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x470000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1e00));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x00211bc8 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x00211bc8);
                                            }
                                            continue 'l_211bc8;
                                        }
                                        0x00211c14 => {
                                            'b_211cd4: {
                                                'b_211c6c: {
                                                    c.cycles += 17;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x1a4)
                                                        );
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[RA][0] = sx(0x00211c24);
                                                    c.g[S0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x74));
                                                    Callee {
                                                        addr: 0x00227f80,
                                                        f: ee::s05::MCManager_Get,
                                                    }
                                                    .call(c, 0x00211c24)?;
                                                    c.g[A0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x276));
                                                    c.g[RA][0] = sx(0x00211c30);
                                                    c.g[A1][0] = c.g[S0][0];
                                                    call_ptr(c, 0x002c2540, 0x00211c30)?;
                                                    c.g[RA][0] = sx(0x00211c38);
                                                    call_ptr(c, 0x0014dc80, 0x00211c38)?;
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x00211c44);
                                                    c.g[A1][0] = 0x1;
                                                    call_ptr(c, 0x0014dd58, 0x00211c44)?;
                                                    c.g[S4][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x00211c50);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x00416810, 0x00211c50)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A2][0] = c.g[S0][0];
                                                    if t {
                                                        break 'b_211c6c;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1cc));
                                                        c.g[RA][0] = sx(0x00211c64);
                                                        c.g[A0][0] = c.g[S4][0];
                                                        call_ptr(c, 0x00147138, 0x00211c64)?;
                                                        let t = true;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S4][0] as u32)
                                                                .wrapping_add(0xc));
                                                        if t {
                                                            break 'b_211cd4;
                                                        } else {
                                                            break 'b_211c6c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 26;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                c.g[A0][0] = 0x470000;
                                                c.g[S2][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x70));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1dc0));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[RA][0] = sx(0x00211c90);
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                call_ptr(c, 0x00317670, 0x00211c90)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00211ca0);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, t, 0x00211ca0)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x00211cac);
                                                c.g[A0][0] = c.g[S2][0];
                                                call_ptr(c, 0x002c2580, 0x00211cac)?;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                                                c.g[A1][0] = c.g[S2][0];
                                                c.g[A0][0] = c.g[SP][0];
                                                c.g[RA][0] = sx(0x00211cc0);
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                                call_ptr(c, 0x00417828, 0x00211cc0)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[RA][0] = sx(0x00211cd0);
                                                c.g[A2][0] = c.g[SP][0];
                                                call_ptr(c, 0x00147138, 0x00211cd0)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                                                break 'b_211cd4;
                                            }
                                            c.cycles += 16;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x00211ce4);
                                            c.g[A0][0] =
                                                sx((c.g[S4][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00211ce4)?;
                                            c.g[RA][0] = sx(0x00211cec);
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            Callee {
                                                addr: 0x00227f80,
                                                f: ee::s05::MCManager_Get,
                                            }
                                            .call(c, 0x00211cec)?;
                                            c.g[A2][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x00211cfc);
                                            c.g[A1][0] = 0x1;
                                            call_ptr(c, 0x0023cae8, 0x00211cfc)?;
                                            c.g[V1][0] = 0x1;
                                            c.g[V0][0] = 0x3;
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1c0),
                                                c.g[V0][0],
                                            );
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1dc),
                                                c.g[V1][0],
                                            );
                                            let t = true;
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1a8),
                                                c.g[V1][0],
                                            );
                                            if t {
                                                break 'b_21205c;
                                            } else {
                                                break 'b_211d14;
                                            }
                                        }
                                        0x00211d14 => {
                                            break 'b_211d14;
                                        }
                                        0x00211f40 => {
                                            break 'b_211f40;
                                        }
                                        0x00211fa8 => {
                                            break 'b_211fa8;
                                        }
                                        0x0021205c => {
                                            break 'b_21205c;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x00211bc8, 0x0021207c);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S1][0] == 0u64;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                                if t {
                                    break 'b_212060;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                                    if c.g[V0][0] != 0u64 {
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                                        break 'b_212064;
                                    } else {
                                        c.cycles += 7;
                                        c.g[RA][0] = sx(0x00211d30);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        Callee {
                                            addr: 0x00227f80,
                                            f: ee::s05::MCManager_Get,
                                        }
                                        .call(c, 0x00211d30)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
                                        c.g[V0][0] = 0x3;
                                        if c.g[V1][0] != c.g[V0][0] {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                                            break 'b_212060;
                                        } else {
                                            c.cycles += 7;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                            c.g[V1][0] = 0x1;
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1c4),
                                                c.g[V0][0],
                                            );
                                            c.sw(
                                                (c.g[A0][0] as u32).wrapping_add(0xf8),
                                                c.g[V0][0],
                                            );
                                            c.g[S2][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                                            if c.g[S2][0] != c.g[V1][0] {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                                                break 'b_212060;
                                            } else {
                                                'b_211eb4: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x1a4)
                                                        );
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    if t {
                                                        break 'b_211eb4;
                                                    } else {
                                                        c.cycles += 82;
                                                        c.g[A1][0] = 0x470000;
                                                        c.g[A0][0] = 0x444;
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x1dd8));
                                                        c.g[A2][0] = 0x100;
                                                        c.g[RA][0] = sx(0x00211d84);
                                                        c.g[A3][0] = 0u64;
                                                        call_ptr(c, 0x00317d70, 0x00211d84)?;
                                                        c.g[T0][0] = c.lbu(
                                                            (c.g[S3][0] as u32).wrapping_add(0x15),
                                                        );
                                                        c.g[A2][0] = c.g[S3][0];
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[A3][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211d9c);
                                                        c.g[A0][0] = c.g[V0][0];
                                                        call_ptr(c, 0x001cb030, 0x00211d9c)?;
                                                        c.g[V1][0] = 0x37;
                                                        c.sw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x1a4),
                                                            c.g[V0][0],
                                                        );
                                                        c.sw(
                                                            (c.g[V0][0] as u32).wrapping_add(0x140),
                                                            c.g[V1][0],
                                                        );
                                                        c.g[A1][0] = 0x8;
                                                        c.g[RA][0] = sx(0x00211db4);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        call_ptr(c, 0x001cd088, 0x00211db4)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A2][0] = 0x1;
                                                        c.g[A3][0] = 0x2;
                                                        c.g[RA][0] = sx(0x00211dc8);
                                                        c.g[A1][0] = 0x4b;
                                                        call_ptr(c, 0x001ce3c8, 0x00211dc8)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x1;
                                                        c.sw(
                                                            (c.g[V0][0] as u32).wrapping_add(0x58),
                                                            c.g[S2][0],
                                                        );
                                                        c.g[RA][0] = sx(0x00211ddc);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        call_ptr(c, 0x001ce408, 0x00211ddc)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0xe;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211df0);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211df0)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x27;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e04);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e04)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x28;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e18);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e18)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x34;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e2c);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e2c)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x1a;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e40);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e40)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x19;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e54);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e54)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x35;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e68);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e68)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x40;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e7c);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e7c)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x41;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211e90);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211e90)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[A1][0] = 0x42;
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x00211ea4);
                                                        c.g[A3][0] = 0x2;
                                                        call_ptr(c, 0x001ce3c8, 0x00211ea4)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                        c.g[RA][0] = sx(0x00211eb0);
                                                        c.g[A1][0] = 0x1;
                                                        call_ptr(c, 0x001cd0b0, 0x00211eb0)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        break 'b_211eb4;
                                                    }
                                                }
                                                c.cycles += 35;
                                                c.g[A0][0] = 0x470000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1de8));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[RA][0] = sx(0x00211ed0);
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                call_ptr(c, 0x00317670, 0x00211ed0)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00211ee0);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, t, 0x00211ee0)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                                                c.g[RA][0] = sx(0x00211eec);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, 0x001ccf70, 0x00211eec)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                                                c.g[RA][0] = sx(0x00211ef8);
                                                c.sw(
                                                    (c.g[V0][0] as u32).wrapping_add(0x54),
                                                    c.g[S2][0],
                                                );
                                                call_ptr(c, 0x0014dc80, 0x00211ef8)?;
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x00211f04);
                                                c.g[A1][0] = 0x1;
                                                call_ptr(c, 0x0014dd58, 0x00211f04)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                                                c.g[RA][0] = sx(0x00211f10);
                                                c.g[A0][0] = c.g[V0][0];
                                                call_ptr(c, 0x00147170, 0x00211f10)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                                                c.g[RA][0] = sx(0x00211f1c);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, 0x001cb418, 0x00211f1c)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                c.g[A1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1a4));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
                                                c.g[RA][0] = sx(0x00211f34);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                                call_ptr(c, 0x0039f290, 0x00211f34)?;
                                                c.sw(
                                                    (c.g[S3][0] as u32).wrapping_add(0x1a0),
                                                    c.g[S2][0],
                                                );
                                                let t = true;
                                                c.sw((c.g[S3][0] as u32).wrapping_add(0x19c), 0u64);
                                                if t {
                                                    break 'b_21205c;
                                                } else {
                                                    break 'b_211f40;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x00211f48);
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            Callee {
                                addr: 0x00227f80,
                                f: ee::s05::MCManager_Get,
                            }
                            .call(c, 0x00211f48)?;
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A0][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                            let t = c.g[V0][0] == c.g[A0][0];
                            c.g[V0][0] = 0x6;
                            if t {
                                break 'b_21205c;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                                if t {
                                    break 'b_212060;
                                } else {
                                    'b_211fa0: {
                                        'b_211f88: {
                                            'b_211f84: {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == 0u64;
                                                c.sw(
                                                    (c.g[S3][0] as u32).wrapping_add(0x1e4),
                                                    c.g[A0][0],
                                                );
                                                if t {
                                                    break 'b_211f84;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x2;
                                                    if c.g[V1][0] == c.g[V0][0] {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x748));
                                                        break 'b_211f88;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] != c.g[A0][0];
                                                        c.g[V0][0] = 0x6;
                                                        if t {
                                                            break 'b_211fa0;
                                                        } else {
                                                            break 'b_211f84;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x748));
                                            break 'b_211f88;
                                        }
                                        c.cycles += 6;
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00211f9c);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x00211f9c)?;
                                        c.g[V0][0] = 0x6;
                                        break 'b_211fa0;
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                                    if t {
                                        break 'b_21205c;
                                    } else {
                                        break 'b_211fa8;
                                    }
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                        c.g[S2][0] = 0x1;
                        let t = c.g[V0][0] == c.g[S2][0];
                        c.g[V0][0] = 0x6;
                        if t {
                            break 'b_21205c;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                            if t {
                                break 'b_212060;
                            } else {
                                c.cycles += 12;
                                c.g[RA][0] = sx(0x00211fcc);
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                Callee {
                                    addr: 0x00227f80,
                                    f: ee::s05::MCManager_Get,
                                }
                                .call(c, 0x00211fcc)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x428));
                                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x00211fec);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00211fec)?;
                                if c.g[V0][0] == 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                                    break 'b_212060;
                                } else {
                                    c.cycles += 6;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c4), c.g[V0][0]);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
                                    c.g[S1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                                    if c.g[S1][0] != c.g[S2][0] {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        break 'b_212060;
                                    } else {
                                        c.cycles += 9;
                                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                                        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c4));
                                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x00212028);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x00212028)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x4;
                                        if t {
                                            break 'b_21205c;
                                        } else {
                                            c.cycles += 11;
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1a8),
                                                c.g[S1][0],
                                            );
                                            c.g[A0][0] = c.g[S0][0];
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1c0),
                                                c.g[V0][0],
                                            );
                                            c.g[RA][0] = sx(0x00212044);
                                            c.g[A1][0] = 0u64;
                                            call_ptr(c, 0x0023d570, 0x00212044)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                                            c.g[A1][0] = 0x18;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x0021205c);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0021205c)?;
                                            break 'b_21205c;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                break 'b_212060;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
            break 'b_212064;
        }
        c.cycles += 6;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine_rt, FAKE};
    use crate::difftest::{check, elf_image, fresh, run_generated, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (the machine's RAM, never
    /// `DATA+0x100000`).
    pub(crate) const OC1021_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC1021_MAGIC: u32 = 0x1021_c0de;
    /// Per-callee result mailbox: 0 keeps the fake's own result, 1 returns 0,
    /// anything else is returned as it is.
    pub(crate) const OC1021_MAIL: u32 = DATA + 0x20_0100;
    pub(crate) const OC1021_ZERO: u32 = 1;

    // Fixture layout.
    /// The state (`this`, a0).
    pub(crate) const OC1021_SELF: u32 = DATA + 0x1000;
    /// The MC manager returned by `MCManager_Get`.
    pub(crate) const OC1021_MC: u32 = DATA + 0x2000;
    pub(crate) const OC1021_WORLD: u32 = DATA + 0x3000;
    pub(crate) const OC1021_WOBJ: u32 = DATA + 0x3100;
    pub(crate) const OC1021_WOBJ2: u32 = DATA + 0x3200;
    pub(crate) const OC1021_A2OBJ: u32 = DATA + 0x3400;
    pub(crate) const OC1021_A2VT: u32 = DATA + 0x3500;
    pub(crate) const OC1021_WV: u32 = DATA + 0x3600;
    /// The widget event (a1).
    pub(crate) const OC1021_EV: u32 = DATA + 0x3800;
    pub(crate) const OC1021_CHAIN: u32 = DATA + 0x3900;
    pub(crate) const OC1021_CHAIN2: u32 = DATA + 0x3a00;
    /// Objects the faked constructor and allocator return.
    pub(crate) const OC1021_KEYBOARD: u32 = DATA + 0x5000;
    pub(crate) const OC1021_ALLOC: u32 = DATA + 0x5800;
    /// A real leaf that returns 0.
    pub(crate) const OC1021_LEAF0: u32 = 0x0014_06c0;

    /// Every direct callee: index, address.
    pub(crate) const OC1021_CALLEES: [u32; 21] = [
        0x002c_2540,
        0x0014_dc80,
        0x0014_dd58,
        0x0041_6810,
        0x0014_7138,
        0x002c_2580,
        0x0041_7828,
        0x0023_cae8,
        0x001c_b030,
        0x0031_7d70,
        0x001c_d088,
        0x001c_e3c8,
        0x001c_e408,
        0x001c_d0b0,
        0x001c_cf70,
        0x0014_7170,
        0x001c_b418,
        0x0039_f290,
        0x0023_d570,
        0x0031_7670,
        0x0022_7f80,
    ];
    pub(crate) const OC1021_IDX_STRLEN: u32 = 3;
    pub(crate) const OC1021_IDX_CTOR: u32 = 8;
    pub(crate) const OC1021_IDX_ALLOC: u32 = 9;
    pub(crate) const OC1021_IDX_MCGET: u32 = 20;

    pub(crate) fn oc1021_apply_mail(c: &mut Cpu, idx: u32) {
        match c.bus.read32(OC1021_MAIL + 4 * idx) {
            0 => {}
            OC1021_ZERO => c.g[V0][0] = 0,
            m => c.g[V0][0] = sx(m),
        }
    }

    /// Gated fakes in the override slots of the direct callees: each logs its
    /// call like `fakes::fake`, then answers from its mailbox; gate off, the
    /// slot runs the body it held.
    macro_rules! oc1021_gated {
        ($($fake:ident => $idx:literal, $slot:path, $generated:path;)*) => {
            $(
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1021_CTRL) != OC1021_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, OC1021_CALLEES[$idx], ra);
                    if c.pc == ra {
                        oc1021_apply_mail(c, $idx);
                    }
                }
            )*

            pub(crate) fn oc1021_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc1021_gated! {
        oc1021_fake_2c2540 => 0, ee::s08::OVERRIDE_002C2540, ee::s08::f_002c2540__generated;
        oc1021_fake_14dc80 => 1, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        oc1021_fake_14dd58 => 2, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        oc1021_fake_416810 => 3, ee::s15::OVERRIDE_00416810, ee::s15::f_00416810__generated;
        oc1021_fake_147138 => 4, ee::s01::OVERRIDE_00147138, ee::s01::f_00147138__generated;
        oc1021_fake_2c2580 => 5, ee::s08::OVERRIDE_002C2580, ee::s08::f_002c2580__generated;
        oc1021_fake_417828 => 6, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated;
        oc1021_fake_23cae8 => 7, ee::s06::OVERRIDE_0023CAE8, ee::s06::f_0023cae8__generated;
        oc1021_fake_1cb030 => 8, ee::s04::OVERRIDE_001CB030, ee::s04::f_001cb030__generated;
        oc1021_fake_317d70 => 9, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated;
        oc1021_fake_1cd088 => 10, ee::s04::OVERRIDE_001CD088, ee::s04::f_001cd088__generated;
        oc1021_fake_1ce3c8 => 11, ee::s04::OVERRIDE_001CE3C8, ee::s04::f_001ce3c8__generated;
        oc1021_fake_1ce408 => 12, ee::s04::OVERRIDE_001CE408, ee::s04::f_001ce408__generated;
        oc1021_fake_1cd0b0 => 13, ee::s04::OVERRIDE_001CD0B0, ee::s04::f_001cd0b0__generated;
        oc1021_fake_1ccf70 => 14, ee::s04::OVERRIDE_001CCF70, ee::s04::f_001ccf70__generated;
        oc1021_fake_147170 => 15, ee::s01::OVERRIDE_00147170, ee::s01::f_00147170__generated;
        oc1021_fake_1cb418 => 16, ee::s04::OVERRIDE_001CB418, ee::s04::f_001cb418__generated;
        oc1021_fake_39f290 => 17, ee::s13::OVERRIDE_0039F290, ee::s13::f_0039f290__generated;
        oc1021_fake_23d570 => 18, ee::s06::OVERRIDE_0023D570, ee::s06::f_0023d570__generated;
        oc1021_fake_317670 => 19, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated;
        oc1021_fake_227f80 => 20, ee::s05::OVERRIDE_00227F80, ee::s05::f_00227f80__generated;
    }

    fn oc1021_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// `OVScreenProfileSavePDA_OnWidgetAction(this, event, action)`: the PDA
    /// profile-save screen's widget handler. `action - 5` (0..9) picks one of
    /// five arms through the jump table (name edit, keyboard popup, MC result
    /// polling), anything else does nothing. The seed varies the action (also
    /// out of range), the state machine fields (`+0x1a8`, `+0x1c0`, `+0x214`),
    /// whether the keyboard popup exists, and what the memory-card manager's
    /// vtable calls and `strlen` answer.
    pub(crate) fn oc1021_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine_rt(c, seed, 0x211bc8, 16);
        elf_image(c);
        oc1021_install_fakes();
        c.bus.write32(OC1021_CTRL, OC1021_MAGIC);
        c.pc = 0x0021_1bc8;
        let gp = GAME_GP;
        let pick =
            |r: &mut crate::difftest::Rng, set: &[u32]| set[(r.next_u32() as usize) % set.len()];

        // Action 3..=16: the ten table entries and both out-of-range sides.
        let action = 3 + seed % 14;
        let event = if r.next_u32() % 5 == 0 { 0 } else { OC1021_EV };
        oc1021_w(c, gp - 0x848, OC1021_WORLD);
        oc1021_w(c, OC1021_WORLD + 0x8c, OC1021_WOBJ);
        oc1021_w(c, OC1021_WOBJ + 4, OC1021_WOBJ2);
        c.bus.write16(OC1021_WOBJ2 + 0x20, 0);
        oc1021_w(c, OC1021_WOBJ2 + 0x24, FAKE + 0x10);

        let this = OC1021_SELF;
        oc1021_w(c, this + 0x10, DATA + 0x6000);
        c.bus.write8(this + 0x15, r.next_u32() as u8);
        oc1021_w(c, this + 0x19c, r.next_u32());
        oc1021_w(c, this + 0x1a0, r.next_u32());
        let keyboard = if r.next_u32() % 2 == 0 {
            0
        } else {
            OC1021_KEYBOARD
        };
        oc1021_w(c, this + 0x1a4, keyboard);
        oc1021_w(c, this + 0x1a8, pick(&mut r, &[0, 1, 1, 2]));
        oc1021_w(c, this + 0x1c0, pick(&mut r, &[0, 1, 1, 2, 3, 6, 6]));
        oc1021_w(c, this + 0x1c4, r.next_u32());
        oc1021_w(c, this + 0x1cc, r.next_u32() % 16);
        oc1021_w(c, this + 0x1dc, r.next_u32());
        oc1021_w(c, this + 0x214, pick(&mut r, &[3, 3, 0]));
        oc1021_w(c, this + 0x40, OC1021_CHAIN);
        oc1021_w(c, OC1021_CHAIN + 0xd0, OC1021_CHAIN2);
        oc1021_w(c, OC1021_KEYBOARD + 0x58, 0);

        // The memory-card manager and the objects its vtable calls reach.
        oc1021_w(c, OC1021_MC + 0x434, OC1021_A2OBJ);
        oc1021_w(c, OC1021_MC + 0x428, r.next_u32());
        oc1021_w(c, OC1021_MC + 0x748, OC1021_WV);
        oc1021_w(c, OC1021_WV + 0xc, FAKE + 0x20);
        oc1021_w(c, OC1021_A2OBJ, OC1021_A2VT);
        oc1021_w(c, OC1021_EV + 0x18, r.next_u32() % 4);
        for off in [0x1b4, 0x21c] {
            let f = if r.next_u32() % 2 == 0 {
                OC1021_LEAF0
            } else {
                FAKE + 0x30
            };
            oc1021_w(c, OC1021_A2VT + off, f);
        }

        // Callee answers.
        let mail = |c: &mut Cpu, idx: u32, v: u32| oc1021_w(c, OC1021_MAIL + 4 * idx, v);
        mail(c, OC1021_IDX_MCGET, OC1021_MC);
        mail(c, OC1021_IDX_ALLOC, OC1021_ALLOC);
        mail(c, OC1021_IDX_CTOR, OC1021_KEYBOARD);
        mail(
            c,
            OC1021_IDX_STRLEN,
            if r.next_u32() % 2 == 0 {
                OC1021_ZERO
            } else {
                5
            },
        );

        c.g[A0][0] = sx(this);
        c.g[A1][0] = sx(event);
        c.g[A2][0] = sx(action);
    }

    /// The port matches the generated body: every action, state, popup and
    /// MC-manager outcome, and unwinds inside the call chains.
    #[test]
    fn ov_screen_profile_save_pda_on_widget_action_matches() {
        check(
            ee::s05::f_00211bc8__generated,
            ov_screen_profile_save_pda_on_widget_action,
            420,
            &oc1021_setup,
        );
    }

    /// The fixture reaches many distinct paths (total cycle costs of the
    /// generated body over the seeds).
    #[test]
    fn ov_screen_profile_save_pda_on_widget_action_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..420u32 {
            let mut c = fresh();
            oc1021_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s05::f_00211bc8__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 12, "only {} paths: {seen:?}", seen.len());
    }
}
