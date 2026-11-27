//! Staging module oc929 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc929`.

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
        0x001c_a180,
        c_fe_popup_confirm_on_widget_notify,
        "cFEPopupConfirm_OnWidgetNotify",
    ),
    (
        0x001c_a298,
        c_fe_popup_confirm_on_widget_action,
        "cFEPopupConfirm_OnWidgetAction",
    ),
    (
        0x001c_a598,
        c_fe_popup_confirm_on_child_event,
        "cFEPopupConfirm_OnChildEvent",
    ),
    (
        0x001c_aa18,
        c_ui_state_buy_popup_set_reward_item_params,
        "cUIStateBuyPopup_SetRewardItemParams",
    ),
];

/// `cFEPopupConfirm_OnWidgetNotify` (0x001ca180): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_popup_confirm_on_widget_notify(c: &mut Cpu) {
    if c.pc != 0x001ca180 {
        return ee::s03::f_001ca180__generated(c);
    }
    guest(c, |c| 'l_1ca180: loop {
        'b_1ca278: {
            'b_1ca274: {
                c.cycles += 11;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] == 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_1ca274;
                } else {
                    'b_1ca264: {
                        'b_1ca254: {
                            'b_1ca24c: {
                                c.cycles += 7;
                                c.g[V0][0] = 0x460000;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6130));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                let t = c.g[A0][0] as u32;
                                match t {
                                    0x001ca180 => {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001ca180);
                                        }
                                        continue 'l_1ca180;
                                    }
                                    0x001ca1c8 => {
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x96));
                                        c.g[S0][0] = 0u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[S3][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x95));
                                        if t {
                                            break 'b_1ca274;
                                        } else {
                                            c.cycles += 2;
                                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x74));
                                            'l_1ca1e0: loop {
                                                'b_1ca1f8: {
                                                    c.cycles += 6;
                                                    c.g[RA][0] = sx(0x001ca1e8);
                                                    c.g[A1][0] = c.g[S0][0];
                                                    call_ptr(c, 0x00397870, 0x001ca1e8)?;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x88));
                                                    c.g[A2][0] = c.g[V0][0];
                                                    if c.g[A2][0] == c.g[V1][0] {
                                                        c.g[A2][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x84));
                                                        break 'b_1ca1f8;
                                                    } else {
                                                        break 'b_1ca1f8;
                                                    }
                                                }
                                                'b_1ca234: {
                                                    'b_1ca21c: {
                                                        c.cycles += 2;
                                                        let t = c.g[S0][0] != c.g[S3][0];
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x8));
                                                        if t {
                                                            break 'b_1ca21c;
                                                        } else {
                                                            c.cycles += 7;
                                                            c.g[A1][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x32c));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x58));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x5c));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x001ca214);
                                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x001ca214)?;
                                                            let t = true;
                                                            c.g[V0][0] = c.lbu(
                                                                (c.g[S2][0] as u32)
                                                                    .wrapping_add(0x96),
                                                            );
                                                            if t {
                                                                break 'b_1ca234;
                                                            } else {
                                                                break 'b_1ca21c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A1][0] =
                                                        sx((c.g[S1][0] as u32).wrapping_add(0x33c));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x58));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x5c));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x001ca230);
                                                    c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x001ca230)?;
                                                    c.g[V0][0] = c.lbu(
                                                        (c.g[S2][0] as u32).wrapping_add(0x96),
                                                    );
                                                    break 'b_1ca234;
                                                }
                                                c.cycles += 4;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V0][0] = ((c.g[S0][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[A0][0] =
                                                    sx((c.g[S2][0] as u32).wrapping_add(0x74));
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x001ca1e0);
                                                    }
                                                    continue 'l_1ca1e0;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[V0][0] = 0x101;
                                                    if t {
                                                        break 'b_1ca278;
                                                    } else {
                                                        break 'b_1ca24c;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    0x001ca24c => {
                                        break 'b_1ca24c;
                                    }
                                    0x001ca254 => {
                                        break 'b_1ca254;
                                    }
                                    0x001ca264 => {
                                        break 'b_1ca264;
                                    }
                                    _ => {
                                        c.jump_out(t, 0x001ca180, 0x001ca294);
                                        return Ok(());
                                    }
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_1ca278;
                            } else {
                                break 'b_1ca254;
                            }
                        }
                        c.cycles += 4;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x310));
                        c.g[V0][0] = 0x101;
                        let t = true;
                        if c.g[V1][0] == 0 {
                            c.g[V0][0] = 0u64;
                        }
                        if t {
                            break 'b_1ca278;
                        } else {
                            break 'b_1ca264;
                        }
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb8));
                    c.g[V0][0] = 0x101;
                    let t = true;
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = 0u64;
                    }
                    if t {
                        break 'b_1ca278;
                    } else {
                        break 'b_1ca274;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x101;
            break 'b_1ca278;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEPopupConfirm_OnWidgetAction` (0x001ca298): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_popup_confirm_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001ca298 {
        return ee::s03::f_001ca298__generated(c);
    }
    guest(c, |c| 'l_1ca298: loop {
        'b_1ca478: {
            'b_1ca474: {
                'b_1ca458: {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A0][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = c.g[A1][0];
                    if t {
                        break 'b_1ca458;
                    } else {
                        'b_1ca41c: {
                            'b_1ca3ec: {
                                c.cycles += 7;
                                c.g[V0][0] = 0x460000;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6150));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                let t = c.g[A0][0] as u32;
                                match t {
                                    0x001ca298 => {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001ca298);
                                        }
                                        continue 'l_1ca298;
                                    }
                                    0x001ca2d8 => {
                                        'b_1ca3e0: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x30c));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A2][0] = 0u64;
                                            if t {
                                                break 'b_1ca3e0;
                                            } else {
                                                c.cycles += 5;
                                                c.sw((c.g[S0][0] as u32).wrapping_add(0xb8), 0u64);
                                                c.g[A1][0] = 0u64;
                                                c.g[A0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x2d0));
                                                c.g[V1][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x90));
                                                'l_1ca2f8: loop {
                                                    'b_1ca354: {
                                                        'b_1ca344: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                            if c.g[S1][0] != c.g[V0][0] {
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1));
                                                                break 'b_1ca344;
                                                            } else {
                                                                'b_1ca320: {
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] =
                                                                        c.lw((c.g[A0][0] as u32));
                                                                    if c.g[V0][0] == 0u64 {
                                                                        c.g[A3][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        break 'b_1ca320;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[RA][0] = sx(0x001ca318);
                                                                        c.g[A0][0] = c.g[S0][0];
                                                                        Callee { addr: 0x001c9b98, f: ee::s03::cFEPopupConfirm_OpenConfirmPopup }.call(c, 0x001ca318)?;
                                                                        let t = true;
                                                                        c.g[A2][0] = 0x1;
                                                                        if t {
                                                                            break 'b_1ca354;
                                                                        } else {
                                                                            break 'b_1ca320;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 9;
                                                                c.g[A2][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0xf));
                                                                c.g[A1][0] = c.g[S0][0];
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A3][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x88));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x8c));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[RA][0] = sx(0x001ca33c);
                                                                c.g[A0][0] = sx((c.g[A3][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                call_ptr(c, t, 0x001ca33c)?;
                                                                let t = true;
                                                                c.g[A2][0] = 0x1;
                                                                if t {
                                                                    break 'b_1ca354;
                                                                } else {
                                                                    break 'b_1ca344;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[V0][0] =
                                                            ((c.g[A1][0] as i64) < 5i64) as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[V1][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x4));
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x001ca2f8);
                                                            }
                                                            continue 'l_1ca2f8;
                                                        } else {
                                                            break 'b_1ca354;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    if c.g[A2][0] != 0u64 {
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        break 'b_1ca478;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x88));
                                                        if c.g[S1][0] != c.g[V0][0] {
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            break 'b_1ca478;
                                                        } else {
                                                            c.cycles += 25;
                                                            c.g[A1][0] = 0x460000;
                                                            c.g[A0][0] = 0x444;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0x2728));
                                                            c.g[A2][0] = 0x100;
                                                            c.g[RA][0] = sx(0x001ca380);
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317d70, 0x001ca380)?;
                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A2][0] = c.g[S0][0];
                                                            c.g[T0][0] = 0xf;
                                                            c.g[A3][0] = 0u64;
                                                            c.g[RA][0] = sx(0x001ca398);
                                                            c.g[A0][0] = c.g[V0][0];
                                                            call_ptr(c, 0x001cb030, 0x001ca398)?;
                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x328));
                                                            c.g[A0][0] = c.g[V0][0];
                                                            c.g[RA][0] = sx(0x001ca3a8);
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x2a8),
                                                                c.g[V0][0],
                                                            );
                                                            call_ptr(c, 0x001cd088, 0x001ca3a8)?;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x2a8));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xd0));
                                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[RA][0] = sx(0x001ca3c0);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            call_ptr(c, 0x0039f290, 0x001ca3c0)?;
                                                            c.g[A2][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x84));
                                                            if c.g[A2][0] == 0u64 {
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                break 'b_1ca478;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                c.g[RA][0] = sx(0x001ca3d8);
                                                                c.g[A1][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x2b4));
                                                                Callee { addr: 0x001c8258, f: ee::s03::cFEPopupConfirm_GetWidgetText }.call(c, 0x001ca3d8)?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                if t {
                                                                    break 'b_1ca478;
                                                                } else {
                                                                    break 'b_1ca3e0;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x1;
                                        let t = true;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xb8), c.g[V0][0]);
                                        if t {
                                            break 'b_1ca474;
                                        } else {
                                            break 'b_1ca3ec;
                                        }
                                    }
                                    0x001ca3ec => {
                                        break 'b_1ca3ec;
                                    }
                                    0x001ca41c => {
                                        break 'b_1ca41c;
                                    }
                                    0x001ca458 => {
                                        break 'b_1ca458;
                                    }
                                    0x001ca474 => {
                                        break 'b_1ca474;
                                    }
                                    _ => {
                                        c.jump_out(t, 0x001ca298, 0x001ca488);
                                        return Ok(());
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x310));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = c.g[S0][0];
                            if t {
                                break 'b_1ca474;
                            } else {
                                c.cycles += 9;
                                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                c.g[A2][0] = 0x14;
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001ca414);
                                c.g[A0][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001ca414)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1ca478;
                                } else {
                                    break 'b_1ca41c;
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2a8));
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2b0));
                        c.g[RA][0] = sx(0x001ca42c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
                        call_ptr(c, 0x003184e8, 0x001ca42c)?;
                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
                        if c.g[A1][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            break 'b_1ca478;
                        } else {
                            c.cycles += 4;
                            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2b0));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_sub(0x8));
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                break 'b_1ca478;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001ca450);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001ca6a0,
                                    f: ee::s03::cFEPopupConfirm_CheckAndFixTextEntry,
                                }
                                .call(c, 0x001ca450)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1ca478;
                                } else {
                                    break 'b_1ca458;
                                }
                            }
                        }
                    }
                }
                c.cycles += 7;
                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = c.g[S1][0];
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x98));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001ca474);
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001ca474)?;
                break 'b_1ca474;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1ca478;
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

/// `cFEPopupConfirm_OnChildEvent` (0x001ca598): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_popup_confirm_on_child_event(c: &mut Cpu) {
    if c.pc != 0x001ca598 {
        return ee::s03::f_001ca598__generated(c);
    }
    guest(c, |c| 'l_1ca598: loop {
        'b_1ca690: {
            'b_1ca68c: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xf));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = c.g[A1][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_1ca68c;
                } else {
                    'b_1ca648: {
                        'b_1ca634: {
                            'b_1ca630: {
                                'b_1ca620: {
                                    'b_1ca600: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6180));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001ca598 => {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x001ca598);
                                                }
                                                continue 'l_1ca598;
                                            }
                                            0x001ca5d4 => {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x2a8));
                                                if c.g[A3][0] != c.g[V0][0] {
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    );
                                                    break 'b_1ca690;
                                                } else {
                                                    c.cycles += 8;
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x2b0)
                                                        );
                                                    c.g[RA][0] = sx(0x001ca5ec);
                                                    c.g[A0][0] = c.g[A3][0];
                                                    call_ptr(c, 0x001cb418, 0x001ca5ec)?;
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x2b4)
                                                        );
                                                    c.g[RA][0] = sx(0x001ca5f8);
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x2a8)
                                                        );
                                                    call_ptr(c, 0x001ccf00, 0x001ca5f8)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    );
                                                    if t {
                                                        break 'b_1ca690;
                                                    } else {
                                                        break 'b_1ca600;
                                                    }
                                                }
                                            }
                                            0x001ca600 => {
                                                break 'b_1ca600;
                                            }
                                            0x001ca620 => {
                                                break 'b_1ca620;
                                            }
                                            0x001ca630 => {
                                                break 'b_1ca630;
                                            }
                                            0x001ca648 => {
                                                break 'b_1ca648;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001ca598, 0x001ca69c);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = 0x1;
                                    c.g[RA][0] = sx(0x001ca610);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f190, 0x001ca610)?;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x324));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xf));
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x320), c.g[V1][0]);
                                    if t {
                                        break 'b_1ca68c;
                                    } else {
                                        break 'b_1ca620;
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[V0][0] = 0xffffffffffffffff;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x320), c.g[V0][0]);
                                if t {
                                    break 'b_1ca634;
                                } else {
                                    break 'b_1ca630;
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            break 'b_1ca634;
                        }
                        c.cycles += 5;
                        c.g[A1][0] = 0x1;
                        c.g[RA][0] = sx(0x001ca640);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f190, 0x001ca640)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_1ca690;
                        } else {
                            break 'b_1ca648;
                        }
                    }
                    'b_1ca654: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2a8));
                        if c.g[A3][0] == c.g[V0][0] {
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x2a8), 0u64);
                            break 'b_1ca654;
                        } else {
                            break 'b_1ca654;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2ac));
                    if c.g[A3][0] != c.g[V0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        break 'b_1ca690;
                    } else {
                        c.cycles += 4;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x320));
                        c.g[V0][0] = 0xffffffffffffffff;
                        let t = c.g[A2][0] == c.g[V0][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x2ac), 0u64);
                        if t {
                            break 'b_1ca68c;
                        } else {
                            c.cycles += 7;
                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001ca68c);
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001ca68c)?;
                            break 'b_1ca68c;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1ca690;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `cUIStateBuyPopup_SetRewardItemParams` (0x001caa18): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_ui_state_buy_popup_set_reward_item_params(c: &mut Cpu) {
    if c.pc != 0x001caa18 {
        return ee::s04::f_001caa18__generated(c);
    }
    guest(c, |c| {
        'b_1caaf4: {
            c.cycles += 26;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.g[V0][0] = 0x2;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S3][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = 0x460000;
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S0][0] = c.g[A1][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = c.g[A3][0];
            c.g[S2][0] = c.g[T0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x61f8));
            c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[A2][0]);
            c.g[RA][0] = sx(0x001caa58);
            c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            call_ptr(c, 0x00317670, 0x001caa58)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x3));
            c.g[V1][0] = 0x3;
            c.g[A0][0] = 0xffffffffffffffff;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x20), c.g[V0][0]);
            c.sw((c.g[S3][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
            c.g[V0][0] = (c.g[S0][0] < 0x6u64) as u64;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.sw((c.g[S3][0] as u32), c.g[S1][0]);
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x4), c.g[S2][0]);
            if t {
                break 'b_1caaf4;
            } else {
                'b_1caae0: {
                    'b_1caad8: {
                        'b_1caacc: {
                            'b_1caac0: {
                                'b_1caab4: {
                                    'b_1caaa8: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x62a0));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001caa9c => {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x460000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x6210));
                                                if t {
                                                    break 'b_1caae0;
                                                } else {
                                                    break 'b_1caaa8;
                                                }
                                            }
                                            0x001caaa8 => {
                                                break 'b_1caaa8;
                                            }
                                            0x001caab4 => {
                                                break 'b_1caab4;
                                            }
                                            0x001caac0 => {
                                                break 'b_1caac0;
                                            }
                                            0x001caacc => {
                                                break 'b_1caacc;
                                            }
                                            0x001caad8 => {
                                                break 'b_1caad8;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001caa18, 0x001cab10);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6228));
                                    if t {
                                        break 'b_1caae0;
                                    } else {
                                        break 'b_1caab4;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = 0x460000;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6240));
                                if t {
                                    break 'b_1caae0;
                                } else {
                                    break 'b_1caac0;
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = 0x460000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6258));
                            if t {
                                break 'b_1caae0;
                            } else {
                                break 'b_1caacc;
                            }
                        }
                        c.cycles += 3;
                        c.g[A0][0] = 0x460000;
                        let t = true;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6270));
                        if t {
                            break 'b_1caae0;
                        } else {
                            break 'b_1caad8;
                        }
                    }
                    c.cycles += 2;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6288));
                    break 'b_1caae0;
                }
                c.cycles += 5;
                c.g[RA][0] = sx(0x001caae8);
                call_ptr(c, 0x00317670, 0x001caae8)?;
                c.g[V1][0] = 0x3;
                c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                c.sw((c.g[S3][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                break 'b_1caaf4;
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{check, elf_image, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Control word of this module's gated fakes (own gate, own magic).
    pub(crate) const OC929_CTRL: u32 = DATA + 0xe_0000;
    pub(crate) const OC929_MAGIC: u32 = 0x0929_f00d;
    /// What the `UIList_GetAt` fake returns for index 0..7.
    pub(crate) const OC929_GETAT: u32 = OC929_CTRL + 0x20;
    pub(crate) const OC929_THIS: u32 = DATA + 0x1000;
    pub(crate) const OC929_ARG: u32 = DATA + 0x2000;
    pub(crate) const OC929_OBJ: u32 = DATA + 0x3000;
    pub(crate) const OC929_VT: u32 = DATA + 0x4000;
    /// Empty UI list: its anchor (+4) is a self-linked sentinel.
    pub(crate) const OC929_SENT: u32 = DATA + 0x5100;
    pub(crate) const OC929_ALLOC: u32 = DATA + 0x6000;
    pub(crate) const OC929_KBD: u32 = DATA + 0x6100;

    /// Logged fake with scripted results: `UIList_GetAt` answers from the
    /// table at `OC929_GETAT`, the allocator and the keyboard constructor
    /// return fixed blocks.
    pub(crate) fn oc929_fake(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let a1 = c.g[A1][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        match target {
            0x0039_7870 => {
                let v = c.bus.read32(OC929_GETAT + 4 * a1.min(7));
                c.g[V0][0] = v as u64;
            }
            0x0031_7d70 => c.g[V0][0] = OC929_ALLOC as u64,
            0x001c_b030 => c.g[V0][0] = OC929_KBD as u64,
            _ => {}
        }
    }

    macro_rules! oc929_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC929_CTRL) != OC929_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                oc929_fake(c, $addr);
            }
        };
    }

    oc929_gated!(
        oc929_fake_get_at,
        OC929_PREV_GET_AT,
        0x0039_7870,
        ee::s13::f_00397870__generated
    );
    oc929_gated!(
        oc929_fake_alloc,
        OC929_PREV_ALLOC,
        0x0031_7d70,
        ee::s10::f_00317d70__generated
    );
    oc929_gated!(
        oc929_fake_kbd_ctor,
        OC929_PREV_KBD_CTOR,
        0x001c_b030,
        ee::s04::f_001cb030__generated
    );
    oc929_gated!(
        oc929_fake_max_len,
        OC929_PREV_MAX_LEN,
        0x001c_d088,
        ee::s04::f_001cd088__generated
    );
    oc929_gated!(
        oc929_fake_pop,
        OC929_PREV_POP,
        0x0039_f190,
        ee::s13::f_0039f190__generated
    );
    oc929_gated!(
        oc929_fake_set_text,
        OC929_PREV_SET_TEXT,
        0x001c_b418,
        ee::s04::f_001cb418__generated
    );
    oc929_gated!(
        oc929_fake_set_static,
        OC929_PREV_SET_STATIC,
        0x001c_cf00,
        ee::s04::f_001ccf00__generated
    );
    oc929_gated!(
        oc929_fake_assign,
        OC929_PREV_ASSIGN,
        0x0031_84e8,
        ee::s10::f_003184e8__generated
    );
    oc929_gated!(
        oc929_fake_widget_text,
        OC929_PREV_WIDGET_TEXT,
        0x001c_8258,
        ee::s03::f_001c8258__generated
    );
    oc929_gated!(
        oc929_fake_open_confirm,
        OC929_PREV_OPEN_CONFIRM,
        0x001c_9b98,
        ee::s03::f_001c9b98__generated
    );
    oc929_gated!(
        oc929_fake_check_text,
        OC929_PREV_CHECK_TEXT,
        0x001c_a6a0,
        ee::s03::f_001ca6a0__generated
    );

    pub(crate) fn oc929_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397870),
            oc929_fake_get_at,
            &OC929_PREV_GET_AT,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            oc929_fake_alloc,
            &OC929_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001CB030),
            oc929_fake_kbd_ctor,
            &OC929_PREV_KBD_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001CD088),
            oc929_fake_max_len,
            &OC929_PREV_MAX_LEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039F190),
            oc929_fake_pop,
            &OC929_PREV_POP,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001CB418),
            oc929_fake_set_text,
            &OC929_PREV_SET_TEXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001CCF00),
            oc929_fake_set_static,
            &OC929_PREV_SET_STATIC,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003184E8),
            oc929_fake_assign,
            &OC929_PREV_ASSIGN,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C8258),
            oc929_fake_widget_text,
            &OC929_PREV_WIDGET_TEXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001C9B98),
            oc929_fake_open_confirm,
            &OC929_PREV_OPEN_CONFIRM,
        );
        fakes::hook(
            addr_of_mut!(ee::s03::OVERRIDE_001CA6A0),
            oc929_fake_check_text,
            &OC929_PREV_CHECK_TEXT,
        );
    }

    fn oc929_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// Machine with the game image (jump tables, strings), the gated fakes on
    /// and the shared object layout: a vtable at `OC929_VT` whose methods
    /// (`+0x58/0x5c`, `+0x88/0x8c`, `+0x98/0x9c`) are machine fakes, three
    /// objects pointing at it, an empty UI list behind the popup's state chain.
    fn oc929_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        oc929_install_fakes();
        fakes::machine(c, seed, salt, calls);
        elf_image(c);
        oc929_w(c, OC929_CTRL, OC929_MAGIC);
        for i in 0..3 {
            oc929_w(c, OC929_OBJ + 0x100 * i + 8, OC929_VT);
        }
        for (k, off) in [0x58u32, 0x88, 0x98].into_iter().enumerate() {
            c.bus.write16(OC929_VT + off, 0);
            oc929_w(c, OC929_VT + off + 4, FAKE + 0x10 * (k as u32 + 1));
        }
        // this -> +0x40 -> +0xd0 -> +0x10 -> list at +0x18 (empty).
        oc929_w(c, OC929_THIS + 0x40, DATA + 0x5000);
        oc929_w(c, DATA + 0x5000 + 0xd0, DATA + 0x5040);
        oc929_w(c, DATA + 0x5040 + 0x10, DATA + 0x5080);
        oc929_w(c, DATA + 0x5080 + 0x18 + 4, OC929_SENT);
        oc929_w(c, OC929_SENT, OC929_SENT);
        oc929_w(c, OC929_SENT + 4, OC929_SENT);
        oc929_w(c, OC929_THIS + 0x10, DATA + 0x5080);
        oc929_w(c, OC929_THIS + 0x20, OC929_OBJ);
    }

    /// `cFEPopupConfirm_OnWidgetNotify(this, menu, event)`: the scroll-change
    /// event recolours the menu's rows (the selected one highlighted), the
    /// option/misc/previous/next events answer 0 or 0x101 from the popup's
    /// flags.
    #[test]
    fn c_fe_popup_confirm_on_widget_notify_matches() {
        check(
            ee::s03::f_001ca180__generated,
            c_fe_popup_confirm_on_widget_notify,
            216,
            &|c, seed| {
                oc929_base(c, seed, 0x1ca180, 6);
                c.pc = 0x001c_a180;
                let ev = 3 + seed % 9;
                c.g[A0][0] = OC929_THIS as u64;
                c.g[A1][0] = OC929_ARG as u64;
                c.g[A2][0] = ev as u64;
                oc929_w(c, OC929_THIS + 0x84, OC929_OBJ + 0x200);
                oc929_w(c, OC929_THIS + 0x88, OC929_OBJ + 0x100);
                oc929_w(c, OC929_THIS + 0x310, seed / 9 & 1);
                oc929_w(c, OC929_THIS + 0xb8, seed / 18 & 1);
                c.bus.write8(OC929_ARG + 0x96, (seed / 9 % 4) as u8);
                c.bus.write8(OC929_ARG + 0x95, (seed / 36 % 4) as u8);
                for i in 0..8 {
                    let o = OC929_OBJ + 0x100 * (seed >> (i + 2) & 1);
                    oc929_w(c, OC929_GETAT + 4 * i, o);
                }
            },
        );
    }

    /// `cFEPopupConfirm_OnWidgetAction(this, id, event)`: button actions
    /// (open the confirm popup or the keyboard popup, edit text, forward to
    /// the child, ...).
    #[test]
    fn c_fe_popup_confirm_on_widget_action_matches() {
        check(
            ee::s03::f_001ca298__generated,
            c_fe_popup_confirm_on_widget_action,
            420,
            &|c, seed| {
                oc929_base(c, seed, 0x1ca298, 5);
                c.pc = 0x001c_a298;
                let ev = 3 + seed % 12;
                let id = 0x1234u32;
                c.g[A0][0] = OC929_THIS as u64;
                c.g[A1][0] = id as u64;
                c.g[A2][0] = ev as u64;
                let hit = seed / 12 % 7;
                for k in 0..5u32 {
                    let v = if k == hit { id } else { id + 1 + k };
                    oc929_w(c, OC929_THIS + 0x90 + 4 * k, v);
                    oc929_w(c, OC929_THIS + 0x2d0 + 12 * k, seed >> (k + 3) & 1);
                }
                oc929_w(c, OC929_THIS + 0x30c, seed / 84 & 1);
                oc929_w(
                    c,
                    OC929_THIS + 0x88,
                    if seed / 7 & 1 == 0 { id } else { id + 0x40 },
                );
                oc929_w(c, OC929_THIS + 0x84, seed / 3 & 1);
                oc929_w(c, OC929_THIS + 0x80, seed / 5 & 1);
                oc929_w(c, OC929_THIS + 0x310, seed / 11 & 1);
                oc929_w(c, OC929_THIS + 0x328, 12);
                oc929_w(c, OC929_THIS + 0x2a8, OC929_KBD);
                oc929_w(c, OC929_THIS + 0x2b0, DATA + 0x7010);
                oc929_w(c, DATA + 0x7008, seed / 13 & 1);
            },
        );
    }

    /// `cFEPopupConfirm_OnChildEvent(this, child, event)`: the keyboard
    /// popup's events (text entered, closed, cancelled ...).
    #[test]
    fn c_fe_popup_confirm_on_child_event_matches() {
        check(
            ee::s03::f_001ca598__generated,
            c_fe_popup_confirm_on_child_event,
            240,
            &|c, seed| {
                oc929_base(c, seed, 0x1ca598, 3);
                c.pc = 0x001c_a598;
                let ev = 13 + seed % 12;
                let child = DATA + 0x6800;
                c.g[A0][0] = OC929_THIS as u64;
                c.g[A1][0] = child as u64;
                c.g[A2][0] = ev as u64;
                oc929_w(
                    c,
                    OC929_THIS + 0x2a8,
                    if seed / 12 & 1 == 0 { child } else { child + 8 },
                );
                oc929_w(
                    c,
                    OC929_THIS + 0x2ac,
                    if seed / 24 & 1 == 0 { child } else { child + 8 },
                );
                oc929_w(c, OC929_THIS + 0x2b0, DATA + 0x7010);
                oc929_w(c, OC929_THIS + 0x2b4, DATA + 0x7020);
                oc929_w(
                    c,
                    OC929_THIS + 0x320,
                    if seed / 48 & 1 == 0 { 0xffff_ffff } else { 3 },
                );
                oc929_w(c, OC929_THIS + 0x324, 7);
            },
        );
    }

    /// `cUIStateBuyPopup_SetRewardItemParams(this, kind, id, a, b)`: fills
    /// the popup's item record; the image name comes from the kind (3..8)
    /// through a jump table, hashed by the real `StrHash`.
    #[test]
    fn c_ui_state_buy_popup_set_reward_item_params_matches() {
        check(
            ee::s04::f_001caa18__generated,
            c_ui_state_buy_popup_set_reward_item_params,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x1caa18, 0);
                elf_image(c);
                c.pc = 0x001c_aa18;
                c.g[A0][0] = OC929_THIS as u64;
                c.g[A1][0] = (seed % 10) as u64;
                c.g[A2][0] = 0x55 + seed as u64;
                c.g[A3][0] = 0x66 + seed as u64;
                c.g[T0][0] = 0x77 + seed as u64;
            },
        );
    }
}
