//! Staging module oc916 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc916`.

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
        0x001c_1ea8,
        c_fe_state_net_ssx_world_on_widget_action,
        "cFEStateNetSSXWorld_OnWidgetAction",
    ),
    (
        0x001c_2740,
        c_fe_state_net_quick_messages_on_widget_action,
        "cFEStateNetQuickMessages_OnWidgetAction",
    ),
    (
        0x001c_3668,
        c_fe_state_stats_on_widget_action,
        "cFEStateStats_OnWidgetAction",
    ),
];

/// `cFEStateNetSSXWorld_OnWidgetAction` (0x001c1ea8): `self`, the widget `a1` and the event `a2`. Events 1/2 set the widget colour white/black through its vtable; 5 opens news (0), quick messages (1), the agreement (2) or the options screen (3, only when the state flags `&6` are clear); 6 goes to the online main menu; 8 opens the messenger; the rest go to `NetFEState_OnWidgetAction`.
pub fn c_fe_state_net_ssx_world_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001c1ea8 {
        return ee::s03::f_001c1ea8__generated(c);
    }
    guest(c, |c| 'l_1c1ea8: loop {
        'b_1c20e8: {
            'b_1c20e4: {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.g[A3][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                let t = c.g[A3][0] == 0u64;
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_1c20e4;
                } else {
                    'b_1c20d8: {
                        c.cycles += 4;
                        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x460000;
                        if t {
                            break 'b_1c20d8;
                        } else {
                            'b_1c20b8: {
                                'b_1c20a4: {
                                    'b_1c207c: {
                                        'b_1c2050: {
                                            'b_1c2040: {
                                                'b_1c2018: {
                                                    c.cycles += 6;
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x5b00));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                    let t = c.g[A0][0] as u32;
                                                    match t {
                                                        0x001c1ea8 => {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x001c1ea8);
                                                            }
                                                            continue 'l_1c1ea8;
                                                        }
                                                        0x001c1ee8 => {
                                                            'b_1c1fb0: {
                                                                'b_1c1f7c: {
                                                                    'b_1c1f50: {
                                                                        c.cycles += 4;
                                                                        c.g[A0][0] = c.lw((c.g[A3]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        c.g[V0][0] = 0x1;
                                                                        let t = c.g[A0][0]
                                                                            == c.g[V0][0];
                                                                        c.g[V0][0] = ((c.g[A0][0]
                                                                            as i64)
                                                                            < 2i64)
                                                                            as u64;
                                                                        if t {
                                                                            break 'b_1c1f50;
                                                                        } else {
                                                                            'b_1c1f28: {
                                                                                'b_1c1f10: {
                                                                                    c.cycles += 2;
                                                                                    if c.g[V0][0]
                                                                                        == 0u64
                                                                                    {
                                                                                        c.g[V0]
                                                                                            [0] =
                                                                                            0x2;
                                                                                        break 'b_1c1f10;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t = c.g
                                                                                            [A0][0]
                                                                                            == 0u64;
                                                                                        c.g[A1][0] = 0x460000;
                                                                                        if t {
                                                                                            break 'b_1c1f28;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            let t =
                                                                                                true;
                                                                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                            if t {
                                                                                                break 'b_1c20e8;
                                                                                            } else {
                                                                                                break 'b_1c1f10;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 2;
                                                                                let t = c.g[A0][0]
                                                                                    == c.g[V0][0];
                                                                                c.g[V0][0] = 0x3;
                                                                                if t {
                                                                                    break 'b_1c1f7c;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    if c.g[A0][0]
                                                                                        == c.g[V0]
                                                                                            [0]
                                                                                    {
                                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                                                                        break 'b_1c1fb0;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t =
                                                                                            true;
                                                                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                        if t {
                                                                                            break 'b_1c20e8;
                                                                                        } else {
                                                                                            break 'b_1c1f28;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 10;
                                                                            c.g[A0][0] = 0x6d0;
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x5ad0,
                                                                                    ));
                                                                            c.g[A2][0] = 0u64;
                                                                            c.g[RA][0] =
                                                                                sx(0x001c1f3c);
                                                                            c.g[A3][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x00317d70,
                                                                                0x001c1f3c,
                                                                            )?;
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x10,
                                                                                ));
                                                                            c.g[RA][0] =
                                                                                sx(0x001c1f48);
                                                                            c.g[A0][0] = c.g[V0][0];
                                                                            Callee { addr: 0x001c5018, f: ee::s03::cFEStateNetNews_ctor }.call(c, 0x001c1f48)?;
                                                                            let t = true;
                                                                            c.g[A0][0] = c.g[S0][0];
                                                                            if t {
                                                                                break 'b_1c2040;
                                                                            } else {
                                                                                break 'b_1c1f50;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 11;
                                                                    c.g[A1][0] = 0x460000;
                                                                    c.g[A0][0] = 0x6fc;
                                                                    c.g[A1][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x5ae0));
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[RA][0] = sx(0x001c1f68);
                                                                    c.g[A3][0] = 0u64;
                                                                    call_ptr(
                                                                        c, 0x00317d70, 0x001c1f68,
                                                                    )?;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    c.g[RA][0] = sx(0x001c1f74);
                                                                    c.g[A0][0] = c.g[V0][0];
                                                                    Callee { addr: 0x001c20f8, f: ee::s03::cFEStateNetQuickMessages_ctor }.call(c, 0x001c1f74)?;
                                                                    let t = true;
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    if t {
                                                                        break 'b_1c2040;
                                                                    } else {
                                                                        break 'b_1c1f7c;
                                                                    }
                                                                }
                                                                c.cycles += 13;
                                                                c.g[A1][0] = 0x460000;
                                                                c.g[A0][0] = 0x6dc;
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x4860));
                                                                c.g[A2][0] = 0x100;
                                                                c.g[RA][0] = sx(0x001c1f94);
                                                                c.g[A3][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x00317d70, 0x001c1f94,
                                                                )?;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                c.g[RA][0] = sx(0x001c1fa0);
                                                                c.g[A0][0] = c.g[V0][0];
                                                                Callee { addr: 0x001c0418, f: ee::s03::cFEStateOnlineAgreement_ctor }.call(c, 0x001c1fa0)?;
                                                                c.g[V1][0] = 0x13;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                let t = true;
                                                                c.sw(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                    c.g[V1][0],
                                                                );
                                                                if t {
                                                                    break 'b_1c2040;
                                                                } else {
                                                                    break 'b_1c1fb0;
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.g[V0][0] & 0x6;
                                                            if c.g[V0][0] != 0u64 {
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                break 'b_1c20e8;
                                                            } else {
                                                                c.cycles += 23;
                                                                c.g[A1][0] = 0x460000;
                                                                c.g[A3][0] = 0u64;
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x2968));
                                                                c.g[A0][0] = 0x54;
                                                                c.g[RA][0] = sx(0x001c1fd4);
                                                                c.g[A2][0] = 0x100;
                                                                call_ptr(
                                                                    c, 0x00317d70, 0x001c1fd4,
                                                                )?;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                c.g[A0][0] = c.g[V0][0];
                                                                c.g[RA][0] = sx(0x001c1fe4);
                                                                c.g[A2][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x001887a0, 0x001c1fe4,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x1c));
                                                                c.g[A0][0] = 0xffffffffffffc0ff;
                                                                c.g[A1][0] = c.g[V0][0];
                                                                c.g[A2][0] = 0u64;
                                                                c.g[V1][0] =
                                                                    c.g[V1][0] & c.g[A0][0];
                                                                c.g[A3][0] = 0u64;
                                                                c.g[V1][0] = c.g[V1][0] | 0x100;
                                                                c.sw(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x1c),
                                                                    c.g[V1][0],
                                                                );
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                c.g[RA][0] = sx(0x001c2010);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x18));
                                                                call_ptr(
                                                                    c, 0x0039f320, 0x001c2010,
                                                                )?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                if t {
                                                                    break 'b_1c20e8;
                                                                } else {
                                                                    break 'b_1c2018;
                                                                }
                                                            }
                                                        }
                                                        0x001c2018 => {
                                                            break 'b_1c2018;
                                                        }
                                                        0x001c2050 => {
                                                            break 'b_1c2050;
                                                        }
                                                        0x001c207c => {
                                                            break 'b_1c207c;
                                                        }
                                                        0x001c20b8 => {
                                                            break 'b_1c20b8;
                                                        }
                                                        0x001c20d8 => {
                                                            break 'b_1c20d8;
                                                        }
                                                        _ => {
                                                            c.jump_out(t, 0x001c1ea8, 0x001c20f4);
                                                            return Ok(());
                                                        }
                                                    }
                                                }
                                                c.cycles += 10;
                                                c.g[A1][0] = 0x460000;
                                                c.g[A0][0] = 0x6d0;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0xd8));
                                                c.g[A2][0] = 0u64;
                                                c.g[RA][0] = sx(0x001c2030);
                                                c.g[A3][0] = 0u64;
                                                call_ptr(c, 0x00317d70, 0x001c2030)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                c.g[RA][0] = sx(0x001c203c);
                                                c.g[A0][0] = c.g[V0][0];
                                                Callee {
                                                    addr: 0x001bfbb8,
                                                    f: ee::s03::cFEStateOnlineMainMenu_ctor,
                                                }
                                                .call(c, 0x001c203c)?;
                                                c.g[A0][0] = c.g[S0][0];
                                                break 'b_1c2040;
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001c2048);
                                            c.g[A1][0] = c.g[V0][0];
                                            Callee {
                                                addr: 0x001a8a40,
                                                f: ee::s03::NetFEState_GotoNextState,
                                            }
                                            .call(c, 0x001c2048)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_1c20e8;
                                            } else {
                                                break 'b_1c2050;
                                            }
                                        }
                                        c.cycles += 11;
                                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = c.g[SP][0];
                                        c.g[AT][0] = 0x3f800000;
                                        c.mtc1(1, 0);
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                                        c.swc1(0, (c.g[SP][0] as u32));
                                        c.g[A0][0] =
                                            sx((c.g[A3][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                        let t = true;
                                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_1c20a4;
                                        } else {
                                            break 'b_1c207c;
                                        }
                                    }
                                    c.cycles += 10;
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = c.g[SP][0];
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 0);
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                    c.swc1(0, (c.g[SP][0] as u32));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                                    break 'b_1c20a4;
                                }
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001c20b0);
                                call_ptr(c, t, 0x001c20b0)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1c20e8;
                                } else {
                                    break 'b_1c20b8;
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = c.g[V0][0] & 0x6;
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                break 'b_1c20e8;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001c20d0);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001a9b58,
                                    f: ee::s03::NetFEState_OpenMessenger,
                                }
                                .call(c, 0x001c20d0)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1c20e8;
                                } else {
                                    break 'b_1c20d8;
                                }
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001c20e4);
                    c.g[A1][0] = c.g[A3][0];
                    Callee {
                        addr: 0x001a8918,
                        f: ee::s03::NetFEState_OnWidgetAction,
                    }
                    .call(c, 0x001c20e4)?;
                    break 'b_1c20e4;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1c20e8;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateNetQuickMessages_OnWidgetAction` (0x001c2740): events 4..13 by jump table: 5 edits quick message `idx` (< 10) in the keyboard popup, 6 asks to save (changed) or goes back to the SSX World screen, 8 opens the messenger, 13 stores the edited message, 4 asks to reset the messages; others go to `NetFEState_OnWidgetAction`.
pub fn c_fe_state_net_quick_messages_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001c2740 {
        return ee::s03::f_001c2740__generated(c);
    }
    guest(c, |c| 'l_1c2740: loop {
        'b_1c29fc: {
            'b_1c29f0: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                let t = c.g[V0][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                if t {
                    break 'b_1c29f0;
                } else {
                    'b_1c29e0: {
                        'b_1c2964: {
                            'b_1c2954: {
                                'b_1c2934: {
                                    'b_1c2878: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5c20));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001c2740 => {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x001c2740);
                                                }
                                                continue 'l_1c2740;
                                            }
                                            0x001c2784 => {
                                                c.cycles += 4;
                                                c.g[S3][0] =
                                                    c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                                                c.g[V0][0] = (c.g[S3][0] < 0xau64) as u64;
                                                let t = c.g[V0][0] == 0u64;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                                if t {
                                                    break 'b_1c29fc;
                                                } else {
                                                    'b_1c2860: {
                                                        'b_1c2834: {
                                                            c.cycles += 12;
                                                            c.g[RA][0] = sx(0x001c279c);
                                                            call_ptr(c, 0x0014dc80, 0x001c279c)?;
                                                            c.g[A0][0] = c.g[V0][0];
                                                            c.g[RA][0] = sx(0x001c27a8);
                                                            c.g[A1][0] = 0x4;
                                                            call_ptr(c, 0x0014dd58, 0x001c27a8)?;
                                                            c.g[S1][0] = c.g[V0][0];
                                                            c.g[A1][0] = c.g[S3][0];
                                                            c.g[RA][0] = sx(0x001c27b8);
                                                            c.g[A0][0] = c.g[S1][0];
                                                            call_ptr(c, 0x0014f6a8, 0x001c27b8)?;
                                                            c.g[S0][0] = c.g[V0][0];
                                                            if c.g[S0][0] != 0u64 {
                                                                c.g[A0][0] = 0x460000;
                                                                break 'b_1c2834;
                                                            } else {
                                                                c.cycles += 28;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                c.g[A0][0] = c.g[S1][0];
                                                                c.g[A1][0] = c.g[S3][0];
                                                                c.g[S0][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x8c));
                                                                c.g[S1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                c.g[V0][0] = c
                                                                    .lh((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x20));
                                                                c.g[RA][0] = sx(0x001c27e4);
                                                                c.g[S0][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[V0][0] as u32),
                                                                    ));
                                                                call_ptr(
                                                                    c, 0x0014f678, 0x001c27e4,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                c.g[A0][0] = c.g[S0][0];
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[RA][0] = sx(0x001c27f4);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(c, t, 0x001c27f4)?;
                                                                c.g[A0][0] = c.g[SP][0];
                                                                c.g[RA][0] = sx(0x001c2800);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x003dcb20, 0x001c2800,
                                                                )?;
                                                                c.g[A0][0] = 0x460000;
                                                                c.g[RA][0] = sx(0x001c280c);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x5bc0));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001c280c,
                                                                )?;
                                                                c.g[A2][0] = c.g[V0][0];
                                                                c.g[T1][0] = c.g[S3][0];
                                                                c.g[A0][0] = c.g[S2][0];
                                                                c.g[A1][0] = 0u64;
                                                                c.g[A3][0] = c.g[SP][0];
                                                                c.g[T0][0] = 0u64;
                                                                c.g[RA][0] = sx(0x001c282c);
                                                                c.g[T2][0] = 0x3f;
                                                                Callee { addr: 0x001a9150, f: ee::s03::NetFEState_OpenKeyboardTextId }.call(c, 0x001c282c)?;
                                                                let t = true;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x69c));
                                                                if t {
                                                                    break 'b_1c2860;
                                                                } else {
                                                                    break 'b_1c2834;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 11;
                                                        c.g[RA][0] = sx(0x001c283c);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5bc0));
                                                        call_ptr(c, 0x00317670, 0x001c283c)?;
                                                        c.g[A2][0] = c.g[V0][0];
                                                        c.g[A3][0] = c.g[S0][0];
                                                        c.g[T1][0] = c.g[S3][0];
                                                        c.g[A0][0] = c.g[S2][0];
                                                        c.g[A1][0] = 0u64;
                                                        c.g[T0][0] = 0u64;
                                                        c.g[RA][0] = sx(0x001c285c);
                                                        c.g[T2][0] = 0x3f;
                                                        Callee { addr: 0x001a9150, f: ee::s03::NetFEState_OpenKeyboardTextId }.call(c, 0x001c285c)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x69c));
                                                        break 'b_1c2860;
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A1][0] = 0x4b;
                                                    c.g[A2][0] = 0x1;
                                                    c.g[RA][0] = sx(0x001c2870);
                                                    c.g[A3][0] = 0x2;
                                                    call_ptr(c, 0x001ce3c8, 0x001c2870)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x80),
                                                    );
                                                    if t {
                                                        break 'b_1c29fc;
                                                    } else {
                                                        break 'b_1c2878;
                                                    }
                                                }
                                            }
                                            0x001c2878 => {
                                                break 'b_1c2878;
                                            }
                                            0x001c2934 => {
                                                break 'b_1c2934;
                                            }
                                            0x001c2954 => {
                                                break 'b_1c2954;
                                            }
                                            0x001c2964 => {
                                                break 'b_1c2964;
                                            }
                                            0x001c29f0 => {
                                                break 'b_1c29f0;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001c2740, 0x001c2a14);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    'b_1c2900: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[A1][0] = 0x460000;
                                        if t {
                                            break 'b_1c2900;
                                        } else {
                                            c.cycles += 31;
                                            c.g[RA][0] = sx(0x001c288c);
                                            c.g[A0][0] = c.g[S2][0];
                                            Callee {
                                                addr: 0x001a9c50,
                                                f: ee::s03::NetFEState_GetPopup,
                                            }
                                            .call(c, 0x001c288c)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A0][0] = 0x460000;
                                            c.g[V0][0] = 0x2;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x5be0));
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xbc),
                                                c.g[V0][0],
                                            );
                                            c.g[RA][0] = sx(0x001c28a8);
                                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                            call_ptr(c, 0x00317670, 0x001c28a8)?;
                                            c.g[V1][0] = 0x3;
                                            c.g[A0][0] = 0x460000;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xc0),
                                                c.g[V1][0],
                                            );
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                            c.g[RA][0] = sx(0x001c28c0);
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xc4),
                                                c.g[V0][0],
                                            );
                                            call_ptr(c, 0x00317670, 0x001c28c0)?;
                                            c.g[A2][0] = c.g[V0][0];
                                            c.g[A1][0] = 0u64;
                                            c.g[RA][0] = sx(0x001c28d0);
                                            c.g[A0][0] = c.g[S1][0];
                                            Callee {
                                                addr: 0x001c57c0,
                                                f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                            }
                                            .call(c, 0x001c28d0)?;
                                            c.g[A0][0] = 0x460000;
                                            c.g[RA][0] = sx(0x001c28dc);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                            call_ptr(c, 0x00317670, 0x001c28dc)?;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A2][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x001c28ec);
                                            c.g[A1][0] = 0x1;
                                            Callee {
                                                addr: 0x001c57c0,
                                                f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                            }
                                            .call(c, 0x001c28ec)?;
                                            c.g[V0][0] = 0x83a;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x168), 0u64);
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x150),
                                                c.g[V0][0],
                                            );
                                            let t = true;
                                            c.g[A0][0] = c.g[S2][0];
                                            if t {
                                                break 'b_1c29e0;
                                            } else {
                                                break 'b_1c2900;
                                            }
                                        }
                                    }
                                    c.cycles += 13;
                                    c.g[A0][0] = 0x6d0;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x11a8));
                                    c.g[A2][0] = 0u64;
                                    c.g[RA][0] = sx(0x001c2914);
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x001c2914)?;
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x001c2920);
                                    c.g[A0][0] = c.g[V0][0];
                                    Callee {
                                        addr: 0x001c1c50,
                                        f: ee::s03::cFEStateNetSSXWorld_ctor,
                                    }
                                    .call(c, 0x001c2920)?;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001c292c);
                                    c.g[A1][0] = c.g[V0][0];
                                    Callee {
                                        addr: 0x001a8a40,
                                        f: ee::s03::NetFEState_GotoNextState,
                                    }
                                    .call(c, 0x001c292c)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                    if t {
                                        break 'b_1c29fc;
                                    } else {
                                        break 'b_1c2934;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                                c.g[V0][0] = c.g[V0][0] & 0x6;
                                let t = c.g[V0][0] != 0u64;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                if t {
                                    break 'b_1c29fc;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001c294c);
                                    c.g[A0][0] = c.g[S2][0];
                                    Callee {
                                        addr: 0x001a9b58,
                                        f: ee::s03::NetFEState_OpenMessenger,
                                    }
                                    .call(c, 0x001c294c)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                    if t {
                                        break 'b_1c29fc;
                                    } else {
                                        break 'b_1c2954;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001c295c);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x001c2a18,
                                f: ee::s03::cFEStateNetQuickMessages_StoreEditedMessage,
                            }
                            .call(c, 0x001c295c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                            if t {
                                break 'b_1c29fc;
                            } else {
                                break 'b_1c2964;
                            }
                        }
                        c.cycles += 31;
                        c.g[RA][0] = sx(0x001c296c);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001a9c50,
                            f: ee::s03::NetFEState_GetPopup,
                        }
                        .call(c, 0x001c296c)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A0][0] = 0x460000;
                        c.g[V0][0] = 0x2;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5c00));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                        c.g[RA][0] = sx(0x001c2988);
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                        call_ptr(c, 0x00317670, 0x001c2988)?;
                        c.g[V1][0] = 0x3;
                        c.g[A0][0] = 0x460000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                        c.g[RA][0] = sx(0x001c29a0);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x001c29a0)?;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x001c29b0);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x001c57c0,
                            f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                        }
                        .call(c, 0x001c29b0)?;
                        c.g[A0][0] = 0x460000;
                        c.g[RA][0] = sx(0x001c29bc);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                        call_ptr(c, 0x00317670, 0x001c29bc)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001c29cc);
                        c.g[A1][0] = 0x1;
                        Callee {
                            addr: 0x001c57c0,
                            f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                        }
                        .call(c, 0x001c29cc)?;
                        c.g[V0][0] = 0x83b;
                        c.g[V1][0] = 0x1;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                        c.g[A0][0] = c.g[S2][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x168), c.g[V1][0]);
                        break 'b_1c29e0;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001c29e8);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001a9bc0,
                        f: ee::s03::NetFEState_ShowPopup,
                    }
                    .call(c, 0x001c29e8)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    if t {
                        break 'b_1c29fc;
                    } else {
                        break 'b_1c29f0;
                    }
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x001c29f8);
            c.g[A0][0] = c.g[S2][0];
            Callee {
                addr: 0x001a8918,
                f: ee::s03::NetFEState_OnWidgetAction,
            }
            .call(c, 0x001c29f8)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_1c29fc;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateStats_OnWidgetAction` (0x001c3668): the stats screen; events 1..9 by jump table: 6 back to the online main menu, 9 refills the course list or results page, 1 and 2 highlight or dim the focused list or tab (and 1 moves the rank highlight onto a result row), 8 opens the messenger; the rest go to `NetFEState_OnWidgetAction`.
pub fn c_fe_state_stats_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001c3668 {
        return ee::s03::f_001c3668__generated(c);
    }
    guest(c, |c| 'l_1c3668: loop {
        'b_1c3d40: {
            'b_1c3d3c: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S2][0] = c.g[A1][0];
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S3][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.g[S6][0] = c.g[A2][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
                let t = c.g[S2][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                if t {
                    break 'b_1c3d3c;
                } else {
                    'b_1c3d2c: {
                        c.cycles += 4;
                        c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1));
                        c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x460000;
                        if t {
                            break 'b_1c3d2c;
                        } else {
                            'b_1c3d18: {
                                'b_1c3c58: {
                                    'b_1c3c38: {
                                        'b_1c39c0: {
                                            'b_1c3758: {
                                                'b_1c3754: {
                                                    'b_1c36fc: {
                                                        c.cycles += 6;
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x5e30));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                        let t = c.g[A0][0] as u32;
                                                        match t {
                                                            0x001c3668 => {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x001c3668,
                                                                    );
                                                                }
                                                                continue 'l_1c3668;
                                                            }
                                                            0x001c36c4 => {
                                                                c.cycles += 14;
                                                                c.g[A1][0] = 0x460000;
                                                                c.g[A0][0] = 0x6d0;
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_sub(0xd8));
                                                                c.g[A2][0] = 0u64;
                                                                c.g[RA][0] = sx(0x001c36dc);
                                                                c.g[A3][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x00317d70, 0x001c36dc,
                                                                )?;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                c.g[RA][0] = sx(0x001c36e8);
                                                                c.g[A0][0] = c.g[V0][0];
                                                                Callee { addr: 0x001bfbb8, f: ee::s03::cFEStateOnlineMainMenu_ctor }.call(c, 0x001c36e8)?;
                                                                c.g[A0][0] = c.g[S3][0];
                                                                c.g[RA][0] = sx(0x001c36f4);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                Callee { addr: 0x001a8a40, f: ee::s03::NetFEState_GotoNextState }.call(c, 0x001c36f4)?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xb0),
                                                                );
                                                                if t {
                                                                    break 'b_1c3d40;
                                                                } else {
                                                                    break 'b_1c36fc;
                                                                }
                                                            }
                                                            0x001c36fc => {
                                                                break 'b_1c36fc;
                                                            }
                                                            0x001c3754 => {
                                                                break 'b_1c3754;
                                                            }
                                                            0x001c39c0 => {
                                                                break 'b_1c39c0;
                                                            }
                                                            0x001c3c38 => {
                                                                break 'b_1c3c38;
                                                            }
                                                            0x001c3d2c => {
                                                                break 'b_1c3d2c;
                                                            }
                                                            0x001c3d3c => {
                                                                break 'b_1c3d3c;
                                                            }
                                                            _ => {
                                                                c.jump_out(
                                                                    t, 0x001c3668, 0x001c3d64,
                                                                );
                                                                return Ok(());
                                                            }
                                                        }
                                                    }
                                                    'b_1c3724: {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = 0x460000;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                        c.g[RA][0] = sx(0x001c370c);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5c78));
                                                        call_ptr(c, 0x00317670, 0x001c370c)?;
                                                        let t = c.g[S0][0] != c.g[V0][0];
                                                        c.g[A0][0] = 0x460000;
                                                        if t {
                                                            break 'b_1c3724;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x001c371c);
                                                            c.g[A0][0] = c.g[S3][0];
                                                            Callee { addr: 0x001c49c8, f: ee::s03::cFEStateStats_FillCourseList }.call(c, 0x001c371c)?;
                                                            let t = true;
                                                            c.g[V0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x40));
                                                            if t {
                                                                break 'b_1c3758;
                                                            } else {
                                                                break 'b_1c3724;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                                    c.g[RA][0] = sx(0x001c3730);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5c88));
                                                    call_ptr(c, 0x00317670, 0x001c3730)?;
                                                    if c.g[S0][0] != c.g[V0][0] {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x40));
                                                        break 'b_1c3758;
                                                    } else {
                                                        c.cycles += 7;
                                                        c.g[RA][0] = sx(0x001c3740);
                                                        c.g[A0][0] = c.g[S2][0];
                                                        call_ptr(c, 0x0039a738, 0x001c3740)?;
                                                        c.g[A0][0] = c.g[S3][0];
                                                        c.g[RA][0] = sx(0x001c374c);
                                                        c.g[A1][0] = c.g[V0][0];
                                                        Callee { addr: 0x001c4b78, f: ee::s03::cFEStateStats_ResetResultsPage }.call(c, 0x001c374c)?;
                                                        c.g[RA][0] = sx(0x001c3754);
                                                        c.g[A0][0] = c.g[S3][0];
                                                        Callee { addr: 0x001c3310, f: ee::s03::cFEStateStats_RefreshResults }.call(c, 0x001c3754)?;
                                                        break 'b_1c3754;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                break 'b_1c3758;
                                            }
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A0][0] = 0x460000;
                                            if t {
                                                break 'b_1c3d3c;
                                            } else {
                                                'b_1c3788: {
                                                    c.cycles += 5;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                                    c.g[RA][0] = sx(0x001c376c);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5cf0));
                                                    call_ptr(c, 0x00317670, 0x001c376c)?;
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.g[A0][0] = 0x460000;
                                                    if t {
                                                        break 'b_1c3788;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[A0][0] = c.g[S3][0];
                                                        c.g[RA][0] = sx(0x001c3780);
                                                        c.g[A1][0] = 0u64;
                                                        Callee { addr: 0x001c3ee8, f: ee::s03::cFEStateStats_SelectCategory }.call(c, 0x001c3780)?;
                                                        let t = true;
                                                        c.g[A0][0] = c.g[S3][0];
                                                        if t {
                                                            break 'b_1c3d18;
                                                        } else {
                                                            break 'b_1c3788;
                                                        }
                                                    }
                                                }
                                                'b_1c37b0: {
                                                    c.cycles += 5;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                                    c.g[RA][0] = sx(0x001c3794);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5d10));
                                                    call_ptr(c, 0x00317670, 0x001c3794)?;
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.g[A0][0] = 0x460000;
                                                    if t {
                                                        break 'b_1c37b0;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[A0][0] = c.g[S3][0];
                                                        c.g[RA][0] = sx(0x001c37a8);
                                                        c.g[A1][0] = 0x2;
                                                        Callee { addr: 0x001c3ee8, f: ee::s03::cFEStateStats_SelectCategory }.call(c, 0x001c37a8)?;
                                                        let t = true;
                                                        c.g[A0][0] = c.g[S3][0];
                                                        if t {
                                                            break 'b_1c3d18;
                                                        } else {
                                                            break 'b_1c37b0;
                                                        }
                                                    }
                                                }
                                                'b_1c37d8: {
                                                    c.cycles += 5;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                                    c.g[RA][0] = sx(0x001c37bc);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5d00));
                                                    call_ptr(c, 0x00317670, 0x001c37bc)?;
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.g[A0][0] = 0x460000;
                                                    if t {
                                                        break 'b_1c37d8;
                                                    } else {
                                                        c.cycles += 5;
                                                        c.g[A0][0] = c.g[S3][0];
                                                        c.g[RA][0] = sx(0x001c37d0);
                                                        c.g[A1][0] = 0x1;
                                                        Callee { addr: 0x001c3ee8, f: ee::s03::cFEStateStats_SelectCategory }.call(c, 0x001c37d0)?;
                                                        let t = true;
                                                        c.g[A0][0] = c.g[S3][0];
                                                        if t {
                                                            break 'b_1c3d18;
                                                        } else {
                                                            break 'b_1c37d8;
                                                        }
                                                    }
                                                }
                                                'b_1c397c: {
                                                    'b_1c3944: {
                                                        'b_1c3890: {
                                                            c.cycles += 5;
                                                            c.g[S0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                            c.g[RA][0] = sx(0x001c37e4);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x5c78));
                                                            call_ptr(c, 0x00317670, 0x001c37e4)?;
                                                            if c.g[S0][0] != c.g[V0][0] {
                                                                c.g[A0][0] = 0x460000;
                                                                break 'b_1c3890;
                                                            } else {
                                                                c.cycles += 41;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[A1][0] = c.g[SP][0];
                                                                c.g[AT][0] = 0x3f800000;
                                                                c.mtc1(1, 0);
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x58));
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0xc),
                                                                );
                                                                c.swc1(0, (c.g[SP][0] as u32));
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x5c));
                                                                let t = c.g[V0][0] as u32;
                                                                c.g[RA][0] = sx(0x001c381c);
                                                                c.g[A0][0] = sx((c.g[S2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                call_ptr(c, t, 0x001c381c)?;
                                                                c.g[A0][0] = 0x460000;
                                                                c.g[RA][0] = sx(0x001c3828);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x5ca8));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001c3828,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x40));
                                                                c.g[RA][0] = sx(0x001c3834);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x0039d860, 0x001c3834,
                                                                )?;
                                                                c.g[A2][0] = c.g[V0][0];
                                                                c.g[A1][0] = 0x1;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x48));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4c));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[RA][0] = sx(0x001c3850);
                                                                c.g[A0][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                call_ptr(c, t, 0x001c3850)?;
                                                                c.g[A0][0] = 0x460000;
                                                                c.g[RA][0] = sx(0x001c385c);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x5c98));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001c385c,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x40));
                                                                c.g[RA][0] = sx(0x001c3868);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x0039d860, 0x001c3868,
                                                                )?;
                                                                c.g[A2][0] = c.g[V0][0];
                                                                c.g[A1][0] = 0x1;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x48));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4c));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[RA][0] = sx(0x001c3884);
                                                                c.g[A0][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                call_ptr(c, t, 0x001c3884)?;
                                                                c.g[A0][0] = 0x460000;
                                                                let t = true;
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x5e10));
                                                                if t {
                                                                    break 'b_1c3944;
                                                                } else {
                                                                    break 'b_1c3890;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 5;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                        c.g[RA][0] = sx(0x001c389c);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5c88));
                                                        call_ptr(c, 0x00317670, 0x001c389c)?;
                                                        let t = c.g[S0][0] != c.g[V0][0];
                                                        c.g[S4][0] = 0u64;
                                                        if t {
                                                            break 'b_1c397c;
                                                        } else {
                                                            c.cycles += 40;
                                                            c.g[V1][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = c.g[SP][0];
                                                            c.g[AT][0] = 0x3f800000;
                                                            c.mtc1(1, 0);
                                                            c.g[A0][0] = c.lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x58));
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0xc),
                                                            );
                                                            c.swc1(0, (c.g[SP][0] as u32));
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x5c));
                                                            let t = c.g[V0][0] as u32;
                                                            c.g[RA][0] = sx(0x001c38d4);
                                                            c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x001c38d4)?;
                                                            c.g[A0][0] = 0x460000;
                                                            c.g[RA][0] = sx(0x001c38e0);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x5cc8));
                                                            call_ptr(c, 0x00317670, 0x001c38e0)?;
                                                            c.g[A0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[RA][0] = sx(0x001c38ec);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x0039d860, 0x001c38ec)?;
                                                            c.g[A2][0] = c.g[V0][0];
                                                            c.g[A1][0] = 0x1;
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x48));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x4c));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x001c3908);
                                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x001c3908)?;
                                                            c.g[A0][0] = 0x460000;
                                                            c.g[RA][0] = sx(0x001c3914);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x5cb8));
                                                            call_ptr(c, 0x00317670, 0x001c3914)?;
                                                            c.g[A0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[RA][0] = sx(0x001c3920);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x0039d860, 0x001c3920)?;
                                                            c.g[A2][0] = c.g[V0][0];
                                                            c.g[A1][0] = 0x1;
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x48));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x4c));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x001c393c);
                                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x001c393c)?;
                                                            c.g[A0][0] = 0x460000;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x5e20));
                                                            break 'b_1c3944;
                                                        }
                                                    }
                                                    c.cycles += 14;
                                                    c.g[RA][0] = sx(0x001c394c);
                                                    call_ptr(c, 0x00317670, 0x001c394c)?;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                    c.g[RA][0] = sx(0x001c3958);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, 0x0039d860, 0x001c3958)?;
                                                    c.g[A2][0] = c.g[V0][0];
                                                    c.g[A1][0] = 0x1;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x001c3974);
                                                    c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x001c3974)?;
                                                    let t = true;
                                                    c.g[A0][0] = c.g[S3][0];
                                                    if t {
                                                        break 'b_1c3d18;
                                                    } else {
                                                        break 'b_1c397c;
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[S5][0] = 0x460000;
                                                c.g[S1][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x10));
                                                c.g[A0][0] = c.g[S1][0];
                                                'l_1c3988: loop {
                                                    c.cycles += 8;
                                                    c.g[A1][0] =
                                                        sx((c.g[S5][0] as u32)
                                                            .wrapping_add(0x5d60));
                                                    c.g[RA][0] = sx(0x001c3994);
                                                    c.g[A2][0] = c.g[S4][0];
                                                    call_ptr(c, 0x00417828, 0x001c3994)?;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                                    c.g[RA][0] = sx(0x001c39a0);
                                                    c.g[A0][0] = c.g[S1][0];
                                                    call_ptr(c, 0x00317670, 0x001c39a0)?;
                                                    if c.g[S0][0] == c.g[V0][0] {
                                                        c.g[A0][0] = 0x4a0000;
                                                        break 'b_1c3c58;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[S4][0] =
                                                            sx((c.g[S4][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] =
                                                            ((c.g[S4][0] as i64) < 10i64) as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x001c3988);
                                                            }
                                                            continue 'l_1c3988;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A0][0] = c.g[S3][0];
                                                            if t {
                                                                break 'b_1c3d18;
                                                            } else {
                                                                break 'b_1c39c0;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        'b_1c3a68: {
                                            'b_1c3a34: {
                                                'b_1c39ec: {
                                                    c.cycles += 6;
                                                    c.g[A0][0] = 0x460000;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                                    c.g[RA][0] = sx(0x001c39d0);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5cf0));
                                                    call_ptr(c, 0x00317670, 0x001c39d0)?;
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.g[A0][0] = 0x460000;
                                                    if t {
                                                        break 'b_1c39ec;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x6e0));
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[A0][0] = c.g[S3][0];
                                                        if t {
                                                            break 'b_1c3d18;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[V1][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x8));
                                                            if t {
                                                                break 'b_1c3a34;
                                                            } else {
                                                                break 'b_1c39ec;
                                                            }
                                                        }
                                                    }
                                                }
                                                'b_1c3a28: {
                                                    'b_1c3a0c: {
                                                        c.cycles += 5;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                        c.g[RA][0] = sx(0x001c39f8);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5d10));
                                                        call_ptr(c, 0x00317670, 0x001c39f8)?;
                                                        let t = c.g[S0][0] != c.g[V0][0];
                                                        c.g[A0][0] = 0x460000;
                                                        if t {
                                                            break 'b_1c3a0c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V1][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x6e0));
                                                            let t = true;
                                                            c.g[V0][0] = 0x2;
                                                            if t {
                                                                break 'b_1c3a28;
                                                            } else {
                                                                break 'b_1c3a0c;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                                    c.g[RA][0] = sx(0x001c3a18);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5d00));
                                                    call_ptr(c, 0x00317670, 0x001c3a18)?;
                                                    let t = c.g[S0][0] != c.g[V0][0];
                                                    c.g[A0][0] = 0x460000;
                                                    if t {
                                                        break 'b_1c3a68;
                                                    } else {
                                                        c.cycles += 2;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x6e0));
                                                        c.g[V0][0] = 0x1;
                                                        break 'b_1c3a28;
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[V1][0] != c.g[V0][0];
                                                c.g[A0][0] = c.g[S3][0];
                                                if t {
                                                    break 'b_1c3d18;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                    break 'b_1c3a34;
                                                }
                                            }
                                            c.cycles += 13;
                                            c.g[A1][0] = c.g[SP][0];
                                            c.g[AT][0] = 0x3f800000;
                                            c.mtc1(1, 0);
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
                                            c.swc1(0, (c.g[SP][0] as u32));
                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001c3a60);
                                            c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001c3a60)?;
                                            let t = true;
                                            c.g[A0][0] = c.g[S3][0];
                                            if t {
                                                break 'b_1c3d18;
                                            } else {
                                                break 'b_1c3a68;
                                            }
                                        }
                                        'b_1c3b50: {
                                            c.cycles += 5;
                                            c.g[S0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                            c.g[RA][0] = sx(0x001c3a74);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x5c78));
                                            call_ptr(c, 0x00317670, 0x001c3a74)?;
                                            if c.g[S0][0] != c.g[V0][0] {
                                                c.g[A0][0] = 0x460000;
                                                break 'b_1c3b50;
                                            } else {
                                                c.cycles += 53;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                c.g[A1][0] = c.g[SP][0];
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 0);
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                                c.swc1(0, (c.g[SP][0] as u32));
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001c3aac);
                                                c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001c3aac)?;
                                                c.g[A0][0] = 0x460000;
                                                c.g[RA][0] = sx(0x001c3ab8);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x5ca8));
                                                call_ptr(c, 0x00317670, 0x001c3ab8)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                c.g[RA][0] = sx(0x001c3ac4);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, 0x0039d860, 0x001c3ac4)?;
                                                c.g[A2][0] = c.g[V0][0];
                                                c.g[A1][0] = 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001c3ae0);
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001c3ae0)?;
                                                c.g[A0][0] = 0x460000;
                                                c.g[RA][0] = sx(0x001c3aec);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x5c98));
                                                call_ptr(c, 0x00317670, 0x001c3aec)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                c.g[RA][0] = sx(0x001c3af8);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, 0x0039d860, 0x001c3af8)?;
                                                c.g[A2][0] = c.g[V0][0];
                                                c.g[A1][0] = 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001c3b14);
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001c3b14)?;
                                                c.g[A0][0] = 0x460000;
                                                c.g[RA][0] = sx(0x001c3b20);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x5e10));
                                                call_ptr(c, 0x00317670, 0x001c3b20)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                                c.g[RA][0] = sx(0x001c3b2c);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, 0x0039d860, 0x001c3b2c)?;
                                                c.g[A2][0] = c.g[V0][0];
                                                c.g[A1][0] = 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001c3b48);
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001c3b48)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S3][0];
                                                if t {
                                                    break 'b_1c3d18;
                                                } else {
                                                    break 'b_1c3b50;
                                                }
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                        c.g[RA][0] = sx(0x001c3b5c);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5c88));
                                        call_ptr(c, 0x00317670, 0x001c3b5c)?;
                                        if c.g[S0][0] != c.g[V0][0] {
                                            c.g[A0][0] = c.g[S3][0];
                                            break 'b_1c3d18;
                                        } else {
                                            c.cycles += 53;
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = c.g[SP][0];
                                            c.g[AT][0] = 0x3f800000;
                                            c.mtc1(1, 0);
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                            c.swc1(0, (c.g[SP][0] as u32));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001c3b94);
                                            c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001c3b94)?;
                                            c.g[A0][0] = 0x460000;
                                            c.g[RA][0] = sx(0x001c3ba0);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x5cc8));
                                            call_ptr(c, 0x00317670, 0x001c3ba0)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                            c.g[RA][0] = sx(0x001c3bac);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x0039d860, 0x001c3bac)?;
                                            c.g[A2][0] = c.g[V0][0];
                                            c.g[A1][0] = 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001c3bc8);
                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001c3bc8)?;
                                            c.g[A0][0] = 0x460000;
                                            c.g[RA][0] = sx(0x001c3bd4);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x5cb8));
                                            call_ptr(c, 0x00317670, 0x001c3bd4)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                            c.g[RA][0] = sx(0x001c3be0);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x0039d860, 0x001c3be0)?;
                                            c.g[A2][0] = c.g[V0][0];
                                            c.g[A1][0] = 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001c3bfc);
                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001c3bfc)?;
                                            c.g[A0][0] = 0x460000;
                                            c.g[RA][0] = sx(0x001c3c08);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x5e20));
                                            call_ptr(c, 0x00317670, 0x001c3c08)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                            c.g[RA][0] = sx(0x001c3c14);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x0039d860, 0x001c3c14)?;
                                            c.g[A2][0] = c.g[V0][0];
                                            c.g[A1][0] = 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001c3c30);
                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001c3c30)?;
                                            let t = true;
                                            c.g[A0][0] = c.g[S3][0];
                                            if t {
                                                break 'b_1c3d18;
                                            } else {
                                                break 'b_1c3c38;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                                    c.g[V0][0] = c.g[V0][0] & 0x6;
                                    let t = c.g[V0][0] != 0u64;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                    if t {
                                        break 'b_1c3d40;
                                    } else {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001c3c50);
                                        c.g[A0][0] = c.g[S3][0];
                                        Callee {
                                            addr: 0x001a9b58,
                                            f: ee::s03::NetFEState_OpenMessenger,
                                        }
                                        .call(c, 0x001c3c50)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                        if t {
                                            break 'b_1c3d40;
                                        } else {
                                            break 'b_1c3c58;
                                        }
                                    }
                                }
                                c.cycles += 48;
                                c.g[RA][0] = sx(0x001c3c60);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c10));
                                call_ptr(c, 0x00317670, 0x001c3c60)?;
                                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001c3c6c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001c3c6c)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = c.g[SP][0];
                                c.g[RA][0] = sx(0x001c3c7c);
                                c.g[A0][0] = c.g[S2][0];
                                call_ptr(c, 0x003a0290, 0x001c3c7c)?;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001c3c94);
                                c.g[A0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001c3c94)?;
                                c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[A1][0] = 0x1;
                                c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
                                c.lwc1(1, (c.g[SP][0] as u32));
                                c.sub_s(0, 0, 2);
                                c.g[AT][0] = 0x41600000;
                                c.mtc1(1, 3);
                                c.g[AT][0] = 0xffffffffc2200000;
                                c.mtc1(1, 2);
                                c.add_s(0, 0, 3);
                                c.add_s(1, 1, 2);
                                c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                                c.swc1(1, (c.g[SP][0] as u32));
                                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
                                c.ldr(2, (c.g[SP][0] as u32));
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x4b));
                                c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x44));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001c3cf4);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001c3cf4)?;
                                c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6e8));
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                c.g[RA][0] = sx(0x001c3d04);
                                c.g[S0][0] =
                                    sx((c.g[S4][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                                call_ptr(c, 0x0025f338, 0x001c3d04)?;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A2][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001c3d14);
                                c.g[A0][0] = c.g[S3][0];
                                Callee {
                                    addr: 0x001c3d68,
                                    f: ee::s03::cFEStateStats_UpdateScrollBar,
                                }
                                .call(c, 0x001c3d14)?;
                                c.g[A0][0] = c.g[S3][0];
                                break 'b_1c3d18;
                            }
                            c.cycles += 5;
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001c3d24);
                            c.g[A2][0] = c.g[S6][0];
                            Callee {
                                addr: 0x001a8918,
                                f: ee::s03::NetFEState_OnWidgetAction,
                            }
                            .call(c, 0x001c3d24)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
                            if t {
                                break 'b_1c3d40;
                            } else {
                                break 'b_1c3d2c;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001c3d3c);
                    c.g[A2][0] = c.g[S6][0];
                    Callee {
                        addr: 0x001a8918,
                        f: ee::s03::NetFEState_OnWidgetAction,
                    }
                    .call(c, 0x001c3d3c)?;
                    break 'b_1c3d3c;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
            break 'b_1c3d40;
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_COUNT, LOG_START};
    use crate::difftest::{fresh, run_generated, Rng, DATA, GAME_GP, SP as OC916_STACK};
    use std::sync::atomic::AtomicUsize;

    /// Gate of the oc916 fakes (a magic word in the machine's RAM) and the
    /// mailbox of forced results (one word per callee at `+0x100 + (addr & 0xffc)`).
    pub(crate) const OC916_CTRL: u32 = DATA + 0x21_0000;
    pub(crate) const OC916_MAGIC: u32 = 0x4f43_3931;
    /// Fixture objects (clear of the fakes' result pointers and of the call log).
    pub(crate) const OC916_SELF: u32 = DATA + 0x8_1000;
    pub(crate) const OC916_W: u32 = DATA + 0x8_2000;
    pub(crate) const OC916_VT: u32 = DATA + 0x8_3000;
    pub(crate) const OC916_FOBJ: u32 = DATA + 0x8_4000;
    pub(crate) const OC916_WORLD: u32 = DATA + 0x8_5000;
    pub(crate) const OC916_LOCALE: u32 = DATA + 0x8_6000;
    pub(crate) const OC916_LVT: u32 = DATA + 0x8_7000;
    pub(crate) const OC916_PARENT: u32 = DATA + 0x8_8000;
    /// The strings `isButton` compares against in `cFEStateStats_OnWidgetAction`.
    pub(crate) const OC916_NAMES: &[u32] = &[
        0x00465c78, 0x00465c88, 0x00465c98, 0x00465ca8, 0x00465cb8, 0x00465cc8, 0x00465cf0,
        0x00465d00, 0x00465d10, 0x00465e10, 0x00465e20, 0x004a1c10,
    ];

    /// The fake callee: logs like `fakes::fake_call`, then applies the
    /// per-callee rule. `StrHash` is the identity on the string's address (so
    /// the widget's name field can select any string), `UIScreen_FindObject`
    /// finds a fixture object, and `BEOptionInt_GetQuickMessage` answers
    /// "no message" when the mailbox holds 1.
    pub(crate) fn oc916_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC916_CTRL + 0x100 + (addr & 0xffc));
        fakes::fake_call(c, addr);
        if c.pc == ra {
            match addr {
                0x0031_7670 => c.g[V0][0] = a0,
                0x0039_d860 => c.g[V0][0] = OC916_FOBJ as u64,
                0x0014_f6a8 if mail == 1 => c.g[V0][0] = 0,
                _ => {}
            }
        }
    }

    macro_rules! oc916_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC916_CTRL) != OC916_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc916_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc916_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc916_fakes!(
        (
            oc916_fake_14dc80,
            OC916_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc916_fake_14dd58,
            OC916_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc916_fake_14f678,
            OC916_PREV_14F678,
            ee::s01::OVERRIDE_0014F678,
            ee::s01::f_0014f678__generated,
            0x0014f678
        ),
        (
            oc916_fake_14f6a8,
            OC916_PREV_14F6A8,
            ee::s01::OVERRIDE_0014F6A8,
            ee::s01::f_0014f6a8__generated,
            0x0014f6a8
        ),
        (
            oc916_fake_1887a0,
            OC916_PREV_1887A0,
            ee::s02::OVERRIDE_001887A0,
            ee::s02::f_001887a0__generated,
            0x001887a0
        ),
        (
            oc916_fake_1a8918,
            OC916_PREV_1A8918,
            ee::s03::OVERRIDE_001A8918,
            ee::s03::f_001a8918__generated,
            0x001a8918
        ),
        (
            oc916_fake_1a8a40,
            OC916_PREV_1A8A40,
            ee::s03::OVERRIDE_001A8A40,
            ee::s03::f_001a8a40__generated,
            0x001a8a40
        ),
        (
            oc916_fake_1a9150,
            OC916_PREV_1A9150,
            ee::s03::OVERRIDE_001A9150,
            ee::s03::f_001a9150__generated,
            0x001a9150
        ),
        (
            oc916_fake_1a9b58,
            OC916_PREV_1A9B58,
            ee::s03::OVERRIDE_001A9B58,
            ee::s03::f_001a9b58__generated,
            0x001a9b58
        ),
        (
            oc916_fake_1a9bc0,
            OC916_PREV_1A9BC0,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a9bc0
        ),
        (
            oc916_fake_1a9c50,
            OC916_PREV_1A9C50,
            ee::s03::OVERRIDE_001A9C50,
            ee::s03::f_001a9c50__generated,
            0x001a9c50
        ),
        (
            oc916_fake_1bfbb8,
            OC916_PREV_1BFBB8,
            ee::s03::OVERRIDE_001BFBB8,
            ee::s03::f_001bfbb8__generated,
            0x001bfbb8
        ),
        (
            oc916_fake_1c0418,
            OC916_PREV_1C0418,
            ee::s03::OVERRIDE_001C0418,
            ee::s03::f_001c0418__generated,
            0x001c0418
        ),
        (
            oc916_fake_1c1c50,
            OC916_PREV_1C1C50,
            ee::s03::OVERRIDE_001C1C50,
            ee::s03::f_001c1c50__generated,
            0x001c1c50
        ),
        (
            oc916_fake_1c20f8,
            OC916_PREV_1C20F8,
            ee::s03::OVERRIDE_001C20F8,
            ee::s03::f_001c20f8__generated,
            0x001c20f8
        ),
        (
            oc916_fake_1c2a18,
            OC916_PREV_1C2A18,
            ee::s03::OVERRIDE_001C2A18,
            ee::s03::f_001c2a18__generated,
            0x001c2a18
        ),
        (
            oc916_fake_1c3310,
            OC916_PREV_1C3310,
            ee::s03::OVERRIDE_001C3310,
            ee::s03::f_001c3310__generated,
            0x001c3310
        ),
        (
            oc916_fake_1c3d68,
            OC916_PREV_1C3D68,
            ee::s03::OVERRIDE_001C3D68,
            ee::s03::f_001c3d68__generated,
            0x001c3d68
        ),
        (
            oc916_fake_1c3ee8,
            OC916_PREV_1C3EE8,
            ee::s03::OVERRIDE_001C3EE8,
            ee::s03::f_001c3ee8__generated,
            0x001c3ee8
        ),
        (
            oc916_fake_1c49c8,
            OC916_PREV_1C49C8,
            ee::s03::OVERRIDE_001C49C8,
            ee::s03::f_001c49c8__generated,
            0x001c49c8
        ),
        (
            oc916_fake_1c4b78,
            OC916_PREV_1C4B78,
            ee::s03::OVERRIDE_001C4B78,
            ee::s03::f_001c4b78__generated,
            0x001c4b78
        ),
        (
            oc916_fake_1c5018,
            OC916_PREV_1C5018,
            ee::s03::OVERRIDE_001C5018,
            ee::s03::f_001c5018__generated,
            0x001c5018
        ),
        (
            oc916_fake_1c57c0,
            OC916_PREV_1C57C0,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c57c0
        ),
        (
            oc916_fake_1ce3c8,
            OC916_PREV_1CE3C8,
            ee::s04::OVERRIDE_001CE3C8,
            ee::s04::f_001ce3c8__generated,
            0x001ce3c8
        ),
        (
            oc916_fake_25f338,
            OC916_PREV_25F338,
            ee::s06::OVERRIDE_0025F338,
            ee::s06::f_0025f338__generated,
            0x0025f338
        ),
        (
            oc916_fake_317670,
            OC916_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc916_fake_317d70,
            OC916_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc916_fake_39a738,
            OC916_PREV_39A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039a738
        ),
        (
            oc916_fake_39d860,
            OC916_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc916_fake_39f320,
            OC916_PREV_39F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039f320
        ),
        (
            oc916_fake_3dcb20,
            OC916_PREV_3DCB20,
            ee::s14::OVERRIDE_003DCB20,
            ee::s14::f_003dcb20__generated,
            0x003dcb20
        ),
        (
            oc916_fake_417828,
            OC916_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
    );

    /// The machine with the fakes on, the world / locale / widget vtables the
    /// handlers walk, and the fixture objects cleared.
    pub(crate) fn oc916_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc916_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        // The jump tables (0x465b00 ...) and name strings live in the game image.
        crate::difftest::elf_image(c);
        c.bus.write32(OC916_CTRL, OC916_MAGIC);
        for obj in [OC916_SELF, OC916_W, OC916_FOBJ, OC916_PARENT] {
            for off in (0..0x800).step_by(4) {
                c.bus.write32(obj + off, 0);
            }
        }
        // The world's locale: `gp - 0x848` -> +0x8c -> +4 (vtable, slot 0x20/0x24).
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC916_WORLD);
        c.bus.write32(OC916_WORLD + 0x8c, OC916_LOCALE);
        c.bus.write32(OC916_LOCALE + 4, OC916_LVT);
        c.bus.write16(OC916_LVT + 0x20, 0);
        c.bus.write32(OC916_LVT + 0x24, FAKE);
        // The widget's vtable: methods at 0x48/0x4c, 0x58/0x5c and 0xa0/0xa4.
        for off in [0x48, 0x58, 0xa0] {
            c.bus.write16(OC916_VT + off, 0);
            c.bus.write32(OC916_VT + off + 4, FAKE);
        }
        c.bus.write32(OC916_W + 8, OC916_VT);
        c.bus.write32(OC916_FOBJ + 8, OC916_VT);
        c.bus
            .write32(OC916_W + 0x40, if seed % 4 == 3 { OC916_PARENT } else { 0 });
        // Mailbox: BEOptionInt_GetQuickMessage answers a message or none.
        c.bus
            .write32(OC916_CTRL + 0x100 + (0x0014_f6a8 & 0xffc), seed % 2);
        // The state: engine pointer and attribute flags (`& 6` clear or set).
        c.bus.write32(OC916_SELF + 0x10, r.next_u32() & !3);
        c.bus.write32(
            OC916_SELF + 0x1c,
            [0, 2, 4, 6][(seed / 2 % 4) as usize] | 0x100,
        );
        r
    }

    /// The distinct call sequences (callee addresses, in order) the generated
    /// body makes over the seeds: the path count of the fixture.
    fn oc916_paths(generated: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) -> usize {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            run_generated(&mut c, generated);
            let n = c.bus.read32(LOG_COUNT);
            let seq: Vec<u32> = (0..n)
                .map(|i| c.bus.read32(LOG_START + i * 0x220))
                .collect();
            seen.insert(seq);
        }
        seen.len()
    }

    fn oc916_ssx_world_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc916_machine(c, seed, 0x1c1ea8, 5);
        // Widget kind (event 5's menu entry) and the event.
        c.bus.write32(
            OC916_W + 0x18,
            [0, 1, 2, 3, 0, 1, 2, 3, 9][(seed / 10 % 9) as usize],
        );
        c.bus.write32(OC916_SELF + 0x40, r.next_u32());
        c.g[A0][0] = sx(OC916_SELF);
        c.g[A1][0] = if seed % 11 == 0 { 0 } else { OC916_W as u64 };
        c.g[A2][0] = [5u64, 6, 1, 2, 8, 5, 3, 4, 0, 5][(seed % 10) as usize];
        c.pc = 0x001c_1ea8;
    }

    /// `cFEStateNetSSXWorld_OnWidgetAction`: no widget, colour events 1/2,
    /// event 5 with each menu entry (flags clear or set), 6, 8 and the default.
    #[test]
    fn net_ssx_world_on_widget_action_matches() {
        fakes::covered(
            ee::s03::f_001c1ea8__generated,
            c_fe_state_net_ssx_world_on_widget_action,
            360,
            &oc916_ssx_world_setup,
        );
        let paths = oc916_paths(ee::s03::f_001c1ea8__generated, 360, &oc916_ssx_world_setup);
        assert!(paths >= 8, "only {paths} call sequences");
    }

    fn oc916_quick_messages_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc916_machine(c, seed, 0x1c2740, 9);
        c.bus.write32(OC916_W + 0x18, (seed / 13 % 12) as u32);
        c.bus.write32(OC916_SELF + 0x69c, r.next_u32() & !3);
        c.bus.write32(OC916_SELF + 0x6d0, (seed % 3 == 0) as u32);
        c.g[A0][0] = sx(OC916_SELF);
        c.g[A1][0] = sx(OC916_W);
        c.g[A2][0] = (3 + seed % 13) as u64;
        c.pc = 0x001c_2740;
    }

    /// `cFEStateNetQuickMessages_OnWidgetAction`: every event of the jump table
    /// (4..13) and the default, messages with and without a stored text, a
    /// changed message list or not.
    #[test]
    fn net_quick_messages_on_widget_action_matches() {
        fakes::covered(
            ee::s03::f_001c2740__generated,
            c_fe_state_net_quick_messages_on_widget_action,
            520,
            &oc916_quick_messages_setup,
        );
        let paths = oc916_paths(
            ee::s03::f_001c2740__generated,
            520,
            &oc916_quick_messages_setup,
        );
        assert!(paths >= 8, "only {paths} call sequences");
    }

    fn oc916_stats_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc916_machine(c, seed, 0x1c3668, 24);
        // The widget's name hash: one of the screen's strings, the sprintf'd
        // result-row name (a stack buffer), or something else.
        let n = OC916_NAMES.len() as u32;
        let pick = seed / 11 % (n + 2);
        let name = if pick < n {
            OC916_NAMES[pick as usize]
        } else if pick == n {
            OC916_STACK - 0xb0
        } else {
            0x0012_3450
        };
        c.bus.write32(OC916_W + 0x38, name);
        c.bus.write32(
            OC916_SELF + 0x40,
            if seed % 5 == 4 { 0 } else { OC916_FOBJ },
        );
        c.bus.write32(OC916_SELF + 0x6e8, r.next_u32() % 20);
        c.bus.write32(OC916_SELF + 0x6e4, r.next_u32() % 5);
        c.g[A0][0] = sx(OC916_SELF);
        c.g[A1][0] = if seed % 13 == 0 { 0 } else { OC916_W as u64 };
        c.g[A2][0] = (seed % 11) as u64;
        c.pc = 0x001c_3668;
    }

    /// `cFEStateStats_OnWidgetAction`: every event 0..10 against each of the
    /// screen's widgets (tabs, event and course lists, result rows), with the
    /// screen present or not, with and without a parent widget.
    #[test]
    fn stats_on_widget_action_matches() {
        fakes::covered(
            ee::s03::f_001c3668__generated,
            c_fe_state_stats_on_widget_action,
            800,
            &oc916_stats_setup,
        );
        let paths = oc916_paths(ee::s03::f_001c3668__generated, 800, &oc916_stats_setup);
        assert!(paths >= 12, "only {paths} call sequences");
    }
}
