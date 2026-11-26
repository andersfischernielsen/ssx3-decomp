//! Staging module oc915 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc915`.

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
        0x001bd738,
        c_fe_state_lobby_menu_v31,
        "cFEStateLobbyMenu_v31",
    ),
    (
        0x001bfe70,
        c_fe_state_online_main_menu_on_widget_action,
        "cFEStateOnlineMainMenu_OnWidgetAction",
    ),
    (
        0x001c0d28,
        c_fe_state_online_agreement_on_widget_action,
        "cFEStateOnlineAgreement_OnWidgetAction",
    ),
    (
        0x001c19c8,
        c_fe_state_net_quick_play_popup_v31,
        "cFEStateNetQuickPlayPopup_v31",
    ),
];

/// `cFEStateLobbyMenu_v31` (0x001bd738): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_lobby_menu_v31(c: &mut Cpu) {
    if c.pc != 0x001bd738 {
        return ee::s03::f_001bd738__generated(c);
    }
    guest(c, |c| 'l_1bd738: loop {
        'b_1bd834: {
            'b_1bd830: {
                'b_1bd820: {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xdc));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = (c.g[V1][0] < 0x27u64) as u64;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    if t {
                        break 'b_1bd820;
                    } else {
                        'b_1bd810: {
                            'b_1bd7d8: {
                                'b_1bd788: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x460000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5000));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x001bd738 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001bd738);
                                            }
                                            continue 'l_1bd738;
                                        }
                                        0x001bd774 => {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[RA][0] = sx(0x001bd780);
                                            c.g[A1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x774));
                                            Callee {
                                                addr: 0x001be510,
                                                f: ee::s03::cFEStateLobbyMenu_RefreshPlayerList,
                                            }
                                            .call(c, 0x001bd780)?;
                                            let t = true;
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_1bd830;
                                            } else {
                                                break 'b_1bd788;
                                            }
                                        }
                                        0x001bd788 => {
                                            break 'b_1bd788;
                                        }
                                        0x001bd7d8 => {
                                            break 'b_1bd7d8;
                                        }
                                        0x001bd810 => {
                                            break 'b_1bd810;
                                        }
                                        0x001bd820 => {
                                            break 'b_1bd820;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x001bd738, 0x001bd844);
                                            return Ok(());
                                        }
                                    }
                                }
                                'b_1bd7c0: {
                                    c.cycles += 6;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d4));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x764), c.g[V1][0]);
                                    if t {
                                        break 'b_1bd7c0;
                                    } else {
                                        c.cycles += 8;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x76c));
                                        c.g[V0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[RA][0] = sx(0x001bd7b4);
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x708));
                                        call_ptr(c, 0x0039a708, 0x001bd7b4)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001bd7c0);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x003184e8, 0x001bd7c0)?;
                                        break 'b_1bd7c0;
                                    }
                                }
                                c.cycles += 6;
                                c.g[RA][0] = sx(0x001bd7c8);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001be0a8,
                                    f: ee::s03::cFEStateLobbyMenu_RefreshRoomLists,
                                }
                                .call(c, 0x001bd7c8)?;
                                c.g[RA][0] = sx(0x001bd7d0);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001be038,
                                    f: ee::s03::cFEStateLobbyMenu_ClampRoomSelections,
                                }
                                .call(c, 0x001bd7d0)?;
                                let t = true;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_1bd830;
                                } else {
                                    break 'b_1bd7d8;
                                }
                            }
                            c.cycles += 14;
                            c.g[A1][0] = 0x460000;
                            c.g[A0][0] = 0x73c;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xf0));
                            c.g[A2][0] = 0u64;
                            c.g[RA][0] = sx(0x001bd7f0);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x001bd7f0)?;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[RA][0] = sx(0x001bd7fc);
                            c.g[A0][0] = c.g[V0][0];
                            Callee {
                                addr: 0x001b1f88,
                                f: ee::s03::cFEStateChatRoom_ctor,
                            }
                            .call(c, 0x001bd7fc)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001bd808);
                            c.g[A1][0] = c.g[V0][0];
                            Callee {
                                addr: 0x001a8a40,
                                f: ee::s03::NetFEState_GotoNextState,
                            }
                            .call(c, 0x001bd808)?;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_1bd830;
                            } else {
                                break 'b_1bd810;
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001bd818);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x001bd848,
                            f: ee::s03::cFEStateLobbyMenu_OnNetResult,
                        }
                        .call(c, 0x001bd818)?;
                        let t = true;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_1bd830;
                        } else {
                            break 'b_1bd820;
                        }
                    }
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x001bd828);
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x001a8e40,
                    f: ee::s03::NetFEState_HandleNetEvent,
                }
                .call(c, 0x001bd828)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                if t {
                    break 'b_1bd834;
                } else {
                    break 'b_1bd830;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1bd834;
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

/// `cFEStateOnlineMainMenu_OnWidgetAction` (0x001bfe70): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_online_main_menu_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001bfe70 {
        return ee::s03::f_001bfe70__generated(c);
    }
    guest(c, |c| 'l_1bfe70: loop {
        'b_1c0120: {
            'b_1c011c: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[A1][0] == 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_1c011c;
                } else {
                    'b_1c0114: {
                        'b_1c00f8: {
                            'b_1c00e8: {
                                'b_1c00e0: {
                                    'b_1c0074: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x6;
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                        if t {
                                            break 'b_1c0074;
                                        } else {
                                            'b_1bfec4: {
                                                'b_1bfeb0: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x5;
                                                    if t {
                                                        break 'b_1bfeb0;
                                                    } else {
                                                        c.cycles += 2;
                                                        if c.g[A2][0] == c.g[V0][0] {
                                                            c.g[V1][0] = c.lw((c.g[A1][0] as u32)
                                                                .wrapping_add(0x18));
                                                            break 'b_1bfec4;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            if t {
                                                                break 'b_1c0114;
                                                            } else {
                                                                break 'b_1bfeb0;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x8;
                                                if c.g[A2][0] == c.g[V0][0] {
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                                                    break 'b_1c00f8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    if t {
                                                        break 'b_1c0114;
                                                    } else {
                                                        break 'b_1bfec4;
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x460000;
                                            if t {
                                                break 'b_1c011c;
                                            } else {
                                                'b_1c001c: {
                                                    'b_1c0004: {
                                                        'b_1bfff4: {
                                                            'b_1bffcc: {
                                                                'b_1bff98: {
                                                                    'b_1bff6c: {
                                                                        'b_1bff40: {
                                                                            c.cycles += 6;
                                                                            c.g[V1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    << 2);
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x5550,
                                                                                    ));
                                                                            c.g[V1][0] =
                                                                                sx((c.g[V1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[V0][0]
                                                                                            as u32),
                                                                                    ));
                                                                            c.g[A0][0] = c
                                                                                .lw((c.g[V1][0]
                                                                                    as u32));
                                                                            let t =
                                                                                c.g[A0][0] as u32;
                                                                            match t {
                                                                                0x001bfe70 => {
                                                                                    if event_due(c)
                                                                                    {
                                                                                        deliver_event_here(c, 0x001bfe70);
                                                                                    }
                                                                                    continue 'l_1bfe70;
                                                                                }
                                                                                0x001bfee8 => {
                                                                                    c.cycles += 22;
                                                                                    c.g[V0][0] =
                                                                                        0x1;
                                                                                    c.g[A1][0] =
                                                                                        0x460000;
                                                                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x54e0));
                                                                                    c.g[A0][0] =
                                                                                        0x734;
                                                                                    c.g[A2][0] =
                                                                                        0u64;
                                                                                    c.g[A3][0] =
                                                                                        0u64;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001bff08,
                                                                                    );
                                                                                    c.sw((c.g[GP][0] as u32).wrapping_add(0x1d98), c.g[V0][0]);
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x00317d70,
                                                                                        0x001bff08,
                                                                                    )?;
                                                                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                                                                    c.g[A0][0] =
                                                                                        c.g[V0][0];
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001bff18,
                                                                                    );
                                                                                    c.g[A2][0] =
                                                                                        c.g[S2][0];
                                                                                    Callee { addr: 0x001c0f40, f: ee::s03::cFEStateNetQuickPlayPopup_ctor }.call(c, 0x001bff18)?;
                                                                                    c.g[V1][0] =
                                                                                        0x82e;
                                                                                    c.g[A1][0] =
                                                                                        c.g[V0][0];
                                                                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                                                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                                                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                                                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001bff38,
                                                                                    );
                                                                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x0039f290,
                                                                                        0x001bff38,
                                                                                    )?;
                                                                                    let t = true;
                                                                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                    if t {
                                                                                        break 'b_1c0120;
                                                                                    } else {
                                                                                        break 'b_1bff40;
                                                                                    }
                                                                                }
                                                                                0x001bff40 => {
                                                                                    break 'b_1bff40;
                                                                                }
                                                                                0x001bff6c => {
                                                                                    break 'b_1bff6c;
                                                                                }
                                                                                0x001bff98 => {
                                                                                    break 'b_1bff98;
                                                                                }
                                                                                0x001bffcc => {
                                                                                    break 'b_1bffcc;
                                                                                }
                                                                                0x001c0004 => {
                                                                                    break 'b_1c0004;
                                                                                }
                                                                                0x001c001c => {
                                                                                    break 'b_1c001c;
                                                                                }
                                                                                0x001c00e8 => {
                                                                                    break 'b_1c00e8;
                                                                                }
                                                                                0x001c00f4 => {
                                                                                    c.cycles += 1;
                                                                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                    break 'b_1c00f8;
                                                                                }
                                                                                _ => {
                                                                                    c.jump_out(
                                                                                        t,
                                                                                        0x001bfe70,
                                                                                        0x001c0134,
                                                                                    );
                                                                                    return Ok(());
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 11;
                                                                        c.g[A1][0] = 0x460000;
                                                                        c.g[A0][0] = 0xb84;
                                                                        c.g[A1][0] =
                                                                            sx((c.g[A1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1b30,
                                                                                ));
                                                                        c.g[A2][0] = 0u64;
                                                                        c.g[RA][0] = sx(0x001bff58);
                                                                        c.g[A3][0] = 0u64;
                                                                        call_ptr(
                                                                            c, 0x00317d70,
                                                                            0x001bff58,
                                                                        )?;
                                                                        c.g[A1][0] = c.lw((c.g[S2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10));
                                                                        c.g[RA][0] = sx(0x001bff64);
                                                                        c.g[A0][0] = c.g[V0][0];
                                                                        Callee { addr: 0x001bcb58, f: ee::s03::cFEStateLobbyMenu_ctor }.call(c, 0x001bff64)?;
                                                                        let t = true;
                                                                        c.g[A0][0] = c.g[S2][0];
                                                                        if t {
                                                                            break 'b_1bfff4;
                                                                        } else {
                                                                            break 'b_1bff6c;
                                                                        }
                                                                    }
                                                                    c.cycles += 11;
                                                                    c.g[A1][0] = 0x460000;
                                                                    c.g[A0][0] = 0x6f0;
                                                                    c.g[A1][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x5500));
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[RA][0] = sx(0x001bff84);
                                                                    c.g[A3][0] = 0u64;
                                                                    call_ptr(
                                                                        c, 0x00317d70, 0x001bff84,
                                                                    )?;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[S2][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    c.g[RA][0] = sx(0x001bff90);
                                                                    c.g[A0][0] = c.g[V0][0];
                                                                    Callee { addr: 0x001c2d18, f: ee::s03::cFEStateStats_ctor }.call(c, 0x001bff90)?;
                                                                    let t = true;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    if t {
                                                                        break 'b_1bfff4;
                                                                    } else {
                                                                        break 'b_1bff98;
                                                                    }
                                                                }
                                                                c.cycles += 11;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S2][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                c.g[A1][0] = c.g[S2][0];
                                                                c.g[A2][0] = 0u64;
                                                                c.g[A3][0] =
                                                                    c.lw((c.g[V0][0] as u32));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[A3][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x20));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x24));
                                                                let t = c.g[V0][0] as u32;
                                                                c.g[RA][0] = sx(0x001bffbc);
                                                                c.g[A0][0] = sx((c.g[A3][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                call_ptr(c, t, 0x001bffbc)?;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                if t {
                                                                    break 'b_1c0120;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    if t {
                                                                        break 'b_1bfff4;
                                                                    } else {
                                                                        break 'b_1bffcc;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 10;
                                                            c.g[A1][0] = 0x460000;
                                                            c.g[A0][0] = 0x6d0;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x11a8));
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x001bffe4);
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317d70, 0x001bffe4)?;
                                                            c.g[A1][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[RA][0] = sx(0x001bfff0);
                                                            c.g[A0][0] = c.g[V0][0];
                                                            Callee { addr: 0x001c1c50, f: ee::s03::cFEStateNetSSXWorld_ctor }.call(c, 0x001bfff0)?;
                                                            c.g[A0][0] = c.g[S2][0];
                                                            break 'b_1bfff4;
                                                        }
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x001bfffc);
                                                        c.g[A1][0] = c.g[V0][0];
                                                        Callee {
                                                            addr: 0x001a8a40,
                                                            f: ee::s03::NetFEState_GotoNextState,
                                                        }
                                                        .call(c, 0x001bfffc)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x30),
                                                        );
                                                        if t {
                                                            break 'b_1c0120;
                                                        } else {
                                                            break 'b_1c0004;
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[RA][0] = sx(0x001c000c);
                                                    c.g[A0][0] = c.g[S2][0];
                                                    Callee {
                                                        addr: 0x001a9c50,
                                                        f: ee::s03::NetFEState_GetPopup,
                                                    }
                                                    .call(c, 0x001c000c)?;
                                                    c.g[S0][0] = c.g[V0][0];
                                                    c.g[A0][0] = 0x460000;
                                                    c.g[V0][0] = 0x2;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5510));
                                                    break 'b_1c001c;
                                                }
                                                c.cycles += 22;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xbc),
                                                    c.g[V0][0],
                                                );
                                                c.g[RA][0] = sx(0x001c0028);
                                                c.g[S1][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                                call_ptr(c, 0x00317670, 0x001c0028)?;
                                                c.g[V1][0] = 0x3;
                                                c.g[A0][0] = 0x460000;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xc0),
                                                    c.g[V1][0],
                                                );
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                                                c.g[RA][0] = sx(0x001c0040);
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xc4),
                                                    c.g[V0][0],
                                                );
                                                call_ptr(c, 0x00317670, 0x001c0040)?;
                                                c.g[A2][0] = c.g[V0][0];
                                                c.g[A1][0] = 0u64;
                                                c.g[RA][0] = sx(0x001c0050);
                                                c.g[A0][0] = c.g[S1][0];
                                                Callee {
                                                    addr: 0x001c57c0,
                                                    f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                                }
                                                .call(c, 0x001c0050)?;
                                                c.g[A0][0] = 0x460000;
                                                c.g[RA][0] = sx(0x001c005c);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x2c88));
                                                call_ptr(c, 0x00317670, 0x001c005c)?;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[A2][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x001c006c);
                                                c.g[A1][0] = 0x1;
                                                Callee {
                                                    addr: 0x001c57c0,
                                                    f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                                }
                                                .call(c, 0x001c006c)?;
                                                let t = true;
                                                c.g[V0][0] = 0x82d;
                                                if t {
                                                    break 'b_1c00e0;
                                                } else {
                                                    break 'b_1c0074;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 27;
                                    c.g[RA][0] = sx(0x001c007c);
                                    c.g[A0][0] = c.g[S2][0];
                                    Callee {
                                        addr: 0x001a9c50,
                                        f: ee::s03::NetFEState_GetPopup,
                                    }
                                    .call(c, 0x001c007c)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A0][0] = 0x460000;
                                    c.g[V0][0] = 0x2;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5530));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x001c0098);
                                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                    call_ptr(c, 0x00317670, 0x001c0098)?;
                                    c.g[V1][0] = 0x3;
                                    c.g[A0][0] = 0x460000;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                    c.g[RA][0] = sx(0x001c00b0);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                    call_ptr(c, 0x00317670, 0x001c00b0)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A1][0] = 0u64;
                                    c.g[RA][0] = sx(0x001c00c0);
                                    c.g[A0][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x001c57c0,
                                        f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                    }
                                    .call(c, 0x001c00c0)?;
                                    c.g[A0][0] = 0x460000;
                                    c.g[RA][0] = sx(0x001c00cc);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                    call_ptr(c, 0x00317670, 0x001c00cc)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001c00dc);
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x001c57c0,
                                        f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                    }
                                    .call(c, 0x001c00dc)?;
                                    c.g[V0][0] = 0x846;
                                    break 'b_1c00e0;
                                }
                                c.cycles += 2;
                                c.g[A0][0] = c.g[S2][0];
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                                break 'b_1c00e8;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001c00f0);
                            c.g[A1][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001a9bc0,
                                f: ee::s03::NetFEState_ShowPopup,
                            }
                            .call(c, 0x001c00f0)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1c0120;
                            } else {
                                break 'b_1c00f8;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V0][0] & 0x6;
                        let t = c.g[V0][0] != 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_1c0120;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001c010c);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x001a9b58,
                                f: ee::s03::NetFEState_OpenMessenger,
                            }
                            .call(c, 0x001c010c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1c0120;
                            } else {
                                break 'b_1c0114;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001c011c);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001a8918,
                        f: ee::s03::NetFEState_OnWidgetAction,
                    }
                    .call(c, 0x001c011c)?;
                    break 'b_1c011c;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1c0120;
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

/// `cFEStateOnlineAgreement_OnWidgetAction` (0x001c0d28): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_online_agreement_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001c0d28 {
        return ee::s03::f_001c0d28__generated(c);
    }
    guest(c, |c| 'l_1c0d28: loop {
        'b_1c0e78: {
            'b_1c0e74: {
                c.cycles += 5;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                let t = c.g[A1][0] == 0u64;
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_1c0e74;
                } else {
                    'b_1c0e6c: {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = (c.g[A2][0] < 0x5u64) as u64;
                        if t {
                            break 'b_1c0e6c;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;
                            if t {
                                break 'b_1c0e6c;
                            } else {
                                'b_1c0e5c: {
                                    'b_1c0e34: {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x12));
                                        c.g[V1][0] = (c.g[A0][0] < 0x5u64) as u64;
                                        let t = c.g[V1][0] == 0u64;
                                        c.g[V0][0] = 0x460000;
                                        if t {
                                            break 'b_1c0e34;
                                        } else {
                                            'b_1c0e08: {
                                                'b_1c0dd8: {
                                                    'b_1c0da8: {
                                                        c.cycles += 6;
                                                        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x5760));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                        let t = c.g[A0][0] as u32;
                                                        match t {
                                                            0x001c0d28 => {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x001c0d28,
                                                                    );
                                                                }
                                                                continue 'l_1c0d28;
                                                            }
                                                            0x001c0d7c => {
                                                                c.cycles += 11;
                                                                c.g[A1][0] = 0x460000;
                                                                c.g[A0][0] = 0x6d4;
                                                                c.g[A1][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x5740));
                                                                c.g[A2][0] = 0u64;
                                                                c.g[RA][0] = sx(0x001c0d94);
                                                                c.g[A3][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x00317d70, 0x001c0d94,
                                                                )?;
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                c.g[RA][0] = sx(0x001c0da0);
                                                                c.g[A0][0] = c.g[V0][0];
                                                                Callee {
                                                                    addr: 0x001bee30,
                                                                    f: ee::s03::cFEStateLogin_ctor,
                                                                }
                                                                .call(c, 0x001c0da0)?;
                                                                let t = true;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                if t {
                                                                    break 'b_1c0e5c;
                                                                } else {
                                                                    break 'b_1c0da8;
                                                                }
                                                            }
                                                            0x001c0da8 => {
                                                                break 'b_1c0da8;
                                                            }
                                                            0x001c0dd8 => {
                                                                break 'b_1c0dd8;
                                                            }
                                                            0x001c0e08 => {
                                                                break 'b_1c0e08;
                                                            }
                                                            0x001c0e34 => {
                                                                break 'b_1c0e34;
                                                            }
                                                            _ => {
                                                                c.jump_out(
                                                                    t, 0x001c0d28, 0x001c0e84,
                                                                );
                                                                return Ok(());
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 12;
                                                    c.g[A1][0] = 0x460000;
                                                    c.g[A0][0] = 0x6f4;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x2ed0));
                                                    c.g[A2][0] = 0u64;
                                                    c.g[RA][0] = sx(0x001c0dc0);
                                                    c.g[A3][0] = 0u64;
                                                    call_ptr(c, 0x00317d70, 0x001c0dc0)?;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x001c0dd0);
                                                    c.g[A2][0] = 0x1;
                                                    Callee {
                                                        addr: 0x001b88f0,
                                                        f: ee::s03::cFEStateCreateStep1_ctor,
                                                    }
                                                    .call(c, 0x001c0dd0)?;
                                                    let t = true;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    if t {
                                                        break 'b_1c0e5c;
                                                    } else {
                                                        break 'b_1c0dd8;
                                                    }
                                                }
                                                c.cycles += 12;
                                                c.g[A1][0] = 0x460000;
                                                c.g[A0][0] = 0x6f4;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x2ed0));
                                                c.g[A2][0] = 0u64;
                                                c.g[RA][0] = sx(0x001c0df0);
                                                c.g[A3][0] = 0u64;
                                                call_ptr(c, 0x00317d70, 0x001c0df0)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x001c0e00);
                                                c.g[A2][0] = 0u64;
                                                Callee {
                                                    addr: 0x001b88f0,
                                                    f: ee::s03::cFEStateCreateStep1_ctor,
                                                }
                                                .call(c, 0x001c0e00)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_1c0e5c;
                                                } else {
                                                    break 'b_1c0e08;
                                                }
                                            }
                                            c.cycles += 11;
                                            c.g[A1][0] = 0x460000;
                                            c.g[A0][0] = 0x6e0;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x4848));
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x001c0e20);
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x001c0e20)?;
                                            c.g[A1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                            c.g[RA][0] = sx(0x001c0e2c);
                                            c.g[A0][0] = c.g[V0][0];
                                            Callee {
                                                addr: 0x001ba3e0,
                                                f: ee::s03::cFEStateCreateStep2_ctor,
                                            }
                                            .call(c, 0x001c0e2c)?;
                                            let t = true;
                                            c.g[A0][0] = c.g[S0][0];
                                            if t {
                                                break 'b_1c0e5c;
                                            } else {
                                                break 'b_1c0e34;
                                            }
                                        }
                                    }
                                    c.cycles += 10;
                                    c.g[A1][0] = 0x460000;
                                    c.g[A0][0] = 0x6d0;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x11a8));
                                    c.g[A2][0] = 0u64;
                                    c.g[RA][0] = sx(0x001c0e4c);
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x001c0e4c)?;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x001c0e58);
                                    c.g[A0][0] = c.g[V0][0];
                                    Callee {
                                        addr: 0x001c1c50,
                                        f: ee::s03::cFEStateNetSSXWorld_ctor,
                                    }
                                    .call(c, 0x001c0e58)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    break 'b_1c0e5c;
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001c0e64);
                                c.g[A1][0] = c.g[V0][0];
                                Callee {
                                    addr: 0x001a8a40,
                                    f: ee::s03::NetFEState_GotoNextState,
                                }
                                .call(c, 0x001c0e64)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_1c0e78;
                                } else {
                                    break 'b_1c0e6c;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001c0e74);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001a8918,
                        f: ee::s03::NetFEState_OnWidgetAction,
                    }
                    .call(c, 0x001c0e74)?;
                    break 'b_1c0e74;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1c0e78;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateNetQuickPlayPopup_v31` (0x001c19c8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_net_quick_play_popup_v31(c: &mut Cpu) {
    if c.pc != 0x001c19c8 {
        return ee::s03::f_001c19c8__generated(c);
    }
    guest(c, |c| 'l_1c19c8: loop {
        'b_1c1b30: {
            'b_1c1b2c: {
                'b_1c1b28: {
                    'b_1c1b14: {
                        c.cycles += 10;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S0][0] = c.g[A1][0];
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xe6));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        c.g[V0][0] = (c.g[V1][0] < 0x2du64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[S2][0] = c.g[A0][0];
                        if t {
                            break 'b_1c1b14;
                        } else {
                            'b_1c1adc: {
                                'b_1c1a38: {
                                    'b_1c1a18: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5980));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001c19c8 => {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x001c19c8);
                                                }
                                                continue 'l_1c19c8;
                                            }
                                            0x001c1a0c => {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x1;
                                                let t = true;
                                                c.sw(
                                                    (c.g[S2][0] as u32).wrapping_add(0x6f0),
                                                    c.g[V0][0],
                                                );
                                                if t {
                                                    break 'b_1c1b28;
                                                } else {
                                                    break 'b_1c1a18;
                                                }
                                            }
                                            0x001c1a18 => {
                                                break 'b_1c1a18;
                                            }
                                            0x001c1a38 => {
                                                break 'b_1c1a38;
                                            }
                                            0x001c1adc => {
                                                break 'b_1c1adc;
                                            }
                                            0x001c1b14 => {
                                                break 'b_1c1b14;
                                            }
                                            0x001c1b28 => {
                                                break 'b_1c1b28;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001c19c8, 0x001c1b44);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x001c1a24);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f698, 0x001c1a24)?;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001c1a30);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001ab178,
                                        f: ee::s03::NetFEState_ShowChallengeErrorPopup,
                                    }
                                    .call(c, 0x001c1a30)?;
                                    let t = true;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_1c1b2c;
                                    } else {
                                        break 'b_1c1a38;
                                    }
                                }
                                'b_1c1ab4: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6f0));
                                    if c.g[V0][0] == 0u64 {
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x6ec), 0u64);
                                        break 'b_1c1ab4;
                                    } else {
                                        c.cycles += 28;
                                        c.g[RA][0] = sx(0x001c1a4c);
                                        c.g[A0][0] = c.g[S2][0];
                                        Callee {
                                            addr: 0x001a9c50,
                                            f: ee::s03::NetFEState_GetPopup,
                                        }
                                        .call(c, 0x001c1a4c)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A0][0] = 0x460000;
                                        c.g[V0][0] = 0x1;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001c1a68);
                                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                        call_ptr(c, 0x00317670, 0x001c1a68)?;
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x001c1a78);
                                        c.g[A2][0] = c.g[V0][0];
                                        Callee {
                                            addr: 0x001c57c0,
                                            f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                        }
                                        .call(c, 0x001c1a78)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x001c1a84);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5958));
                                        call_ptr(c, 0x00317670, 0x001c1a84)?;
                                        c.g[A1][0] = 0x82f;
                                        c.g[V1][0] = 0x3;
                                        c.g[A2][0] = 0x6;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[A1][0]);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                        c.g[A0][0] = c.g[S2][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                        c.g[A1][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001c1aac);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x14c), c.g[A2][0]);
                                        Callee {
                                            addr: 0x001a9bc0,
                                            f: ee::s03::NetFEState_ShowPopup,
                                        }
                                        .call(c, 0x001c1aac)?;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_1c1b2c;
                                        } else {
                                            break 'b_1c1ab4;
                                        }
                                    }
                                }
                                c.cycles += 10;
                                c.g[A1][0] = c.g[S2][0];
                                c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                                c.g[A2][0] = 0x14;
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001c1ad4);
                                c.g[A0][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001c1ad4)?;
                                let t = true;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_1c1b2c;
                                } else {
                                    break 'b_1c1adc;
                                }
                            }
                            c.cycles += 14;
                            c.g[A1][0] = 0x460000;
                            c.g[A0][0] = 0x6e4;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c60));
                            c.g[A2][0] = 0x100;
                            c.g[RA][0] = sx(0x001c1af4);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x001c1af4)?;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[RA][0] = sx(0x001c1b00);
                            c.g[A0][0] = c.g[V0][0];
                            Callee {
                                addr: 0x001b0f60,
                                f: ee::s03::cFEStateOnlineCharacterSelect_ctor,
                            }
                            .call(c, 0x001c1b00)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001c1b0c);
                            c.g[A1][0] = c.g[V0][0];
                            Callee {
                                addr: 0x001a8a40,
                                f: ee::s03::NetFEState_GotoNextState,
                            }
                            .call(c, 0x001c1b0c)?;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_1c1b2c;
                            } else {
                                break 'b_1c1b14;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001c1b20);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001a8e40,
                        f: ee::s03::NetFEState_HandleNetEvent,
                    }
                    .call(c, 0x001c1b20)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1c1b30;
                    } else {
                        break 'b_1c1b28;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_1c1b2c;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1c1b30;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, elf_image, fresh, run_generated, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate word: the callee fakes below act only on machines that set it.
    pub(crate) const OC915_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC915_MAGIC: u32 = 0x4f43_3931;
    /// The state object under test.
    pub(crate) const OC915_OBJ: u32 = DATA + 0x21_0000;
    /// The widget / message source.
    pub(crate) const OC915_SRC: u32 = DATA + 0x21_1000;
    /// Object chain behind the virtual calls (`obj+0x10`, `obj+0x20`).
    pub(crate) const OC915_CHAIN: u32 = DATA + 0x21_2000;
    /// Popup list owner chain (`obj+0x40`).
    pub(crate) const OC915_UI: u32 = DATA + 0x21_3000;
    pub(crate) const OC915_LEAF_ZERO: u32 = 0x0014_06c0;

    /// The fake callee: logs like `fakes::fake_call`.
    pub(crate) fn oc915_fake_call(c: &mut Cpu, addr: u32) {
        fakes::fake_call(c, addr);
    }

    macro_rules! oc915_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC915_CTRL) != OC915_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc915_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc915_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc915_fakes!(
        (
            oc915_fake_001a8918,
            OC915_PREV_001A8918,
            ee::s03::OVERRIDE_001A8918,
            ee::s03::f_001a8918__generated,
            0x001a8918
        ),
        (
            oc915_fake_001a8a40,
            OC915_PREV_001A8A40,
            ee::s03::OVERRIDE_001A8A40,
            ee::s03::f_001a8a40__generated,
            0x001a8a40
        ),
        (
            oc915_fake_001a8e40,
            OC915_PREV_001A8E40,
            ee::s03::OVERRIDE_001A8E40,
            ee::s03::f_001a8e40__generated,
            0x001a8e40
        ),
        (
            oc915_fake_001a9b58,
            OC915_PREV_001A9B58,
            ee::s03::OVERRIDE_001A9B58,
            ee::s03::f_001a9b58__generated,
            0x001a9b58
        ),
        (
            oc915_fake_001a9bc0,
            OC915_PREV_001A9BC0,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a9bc0
        ),
        (
            oc915_fake_001a9c50,
            OC915_PREV_001A9C50,
            ee::s03::OVERRIDE_001A9C50,
            ee::s03::f_001a9c50__generated,
            0x001a9c50
        ),
        (
            oc915_fake_001ab178,
            OC915_PREV_001AB178,
            ee::s03::OVERRIDE_001AB178,
            ee::s03::f_001ab178__generated,
            0x001ab178
        ),
        (
            oc915_fake_001b0f60,
            OC915_PREV_001B0F60,
            ee::s03::OVERRIDE_001B0F60,
            ee::s03::f_001b0f60__generated,
            0x001b0f60
        ),
        (
            oc915_fake_001b1f88,
            OC915_PREV_001B1F88,
            ee::s03::OVERRIDE_001B1F88,
            ee::s03::f_001b1f88__generated,
            0x001b1f88
        ),
        (
            oc915_fake_001b88f0,
            OC915_PREV_001B88F0,
            ee::s03::OVERRIDE_001B88F0,
            ee::s03::f_001b88f0__generated,
            0x001b88f0
        ),
        (
            oc915_fake_001ba3e0,
            OC915_PREV_001BA3E0,
            ee::s03::OVERRIDE_001BA3E0,
            ee::s03::f_001ba3e0__generated,
            0x001ba3e0
        ),
        (
            oc915_fake_001bcb58,
            OC915_PREV_001BCB58,
            ee::s03::OVERRIDE_001BCB58,
            ee::s03::f_001bcb58__generated,
            0x001bcb58
        ),
        (
            oc915_fake_001bd848,
            OC915_PREV_001BD848,
            ee::s03::OVERRIDE_001BD848,
            ee::s03::f_001bd848__generated,
            0x001bd848
        ),
        (
            oc915_fake_001be038,
            OC915_PREV_001BE038,
            ee::s03::OVERRIDE_001BE038,
            ee::s03::f_001be038__generated,
            0x001be038
        ),
        (
            oc915_fake_001be0a8,
            OC915_PREV_001BE0A8,
            ee::s03::OVERRIDE_001BE0A8,
            ee::s03::f_001be0a8__generated,
            0x001be0a8
        ),
        (
            oc915_fake_001be510,
            OC915_PREV_001BE510,
            ee::s03::OVERRIDE_001BE510,
            ee::s03::f_001be510__generated,
            0x001be510
        ),
        (
            oc915_fake_001bee30,
            OC915_PREV_001BEE30,
            ee::s03::OVERRIDE_001BEE30,
            ee::s03::f_001bee30__generated,
            0x001bee30
        ),
        (
            oc915_fake_001c0f40,
            OC915_PREV_001C0F40,
            ee::s03::OVERRIDE_001C0F40,
            ee::s03::f_001c0f40__generated,
            0x001c0f40
        ),
        (
            oc915_fake_001c1c50,
            OC915_PREV_001C1C50,
            ee::s03::OVERRIDE_001C1C50,
            ee::s03::f_001c1c50__generated,
            0x001c1c50
        ),
        (
            oc915_fake_001c2d18,
            OC915_PREV_001C2D18,
            ee::s03::OVERRIDE_001C2D18,
            ee::s03::f_001c2d18__generated,
            0x001c2d18
        ),
        (
            oc915_fake_001c57c0,
            OC915_PREV_001C57C0,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c57c0
        ),
        (
            oc915_fake_00317670,
            OC915_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc915_fake_00317d70,
            OC915_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc915_fake_003184e8,
            OC915_PREV_003184E8,
            ee::s10::OVERRIDE_003184E8,
            ee::s10::f_003184e8__generated,
            0x003184e8
        ),
        (
            oc915_fake_0039a708,
            OC915_PREV_0039A708,
            ee::s13::OVERRIDE_0039A708,
            ee::s13::f_0039a708__generated,
            0x0039a708
        ),
        (
            oc915_fake_0039f698,
            OC915_PREV_0039F698,
            ee::s13::OVERRIDE_0039F698,
            ee::s13::f_0039f698__generated,
            0x0039f698
        ),
    );

    /// Arm the machine: the game image (the jump tables live in its
    /// rodata) and the fake gate.
    fn oc915_arm(c: &mut Cpu) {
        elf_image(c);
        c.bus.write32(OC915_CTRL, OC915_MAGIC);
    }

    /// An empty `UIList` (what `UIList_Reset` leaves) at `l`.
    fn oc915_list_reset(c: &mut Cpu, l: u32) {
        c.bus.write32(l, l);
        c.bus.write32(l + 0x4, l + 0xc);
        c.bus.write32(l + 0xc, l);
        c.bus.write32(l + 0x10, l + 0xc);
    }

    /// The object chain the state's virtual calls walk: `obj+0x10`,
    /// `obj+0x20`, `obj+0x40` (popup list owner) all point into fixtures
    /// whose method slots hold `method`.
    fn oc915_chain(c: &mut Cpu, method: u32) {
        let o = OC915_OBJ;
        let (a, b, cc) = (OC915_CHAIN, OC915_CHAIN + 0x400, OC915_CHAIN + 0x800);
        c.bus.write32(o + 0x10, a);
        c.bus.write32(a, b);
        c.bus.write32(b + 0x4, cc);
        c.bus.write32(cc + 0x24, method);
        // obj+0x20 -> x, [x+8] -> y, [y+0x8c] = method.
        let (x, y) = (OC915_CHAIN + 0xc00, OC915_CHAIN + 0x1000);
        c.bus.write32(o + 0x20, x);
        c.bus.write32(x + 0x8, y);
        c.bus.write32(y + 0x8c, method);
        // obj+0x40 -> p, [p+0xd0] -> q, [q+0x10] -> r, list at r+0x18.
        let (p, q, r) = (OC915_UI, OC915_UI + 0x400, OC915_UI + 0x800);
        c.bus.write32(o + 0x40, p);
        c.bus.write32(p + 0xd0, q);
        c.bus.write32(q + 0x10, r);
        oc915_list_reset(c, r + 0x18);
    }

    /// Number of distinct cycle totals (paths) the generated body takes.
    fn oc915_paths(f: fn(&mut Cpu), seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) -> usize {
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

    fn oc915_method(seed: u32) -> u32 {
        if (seed / 7) % 2 == 0 {
            OC915_LEAF_ZERO
        } else {
            fakes::FAKE
        }
    }

    /// `cFEStateLobbyMenu_v31(self, event)`: events 0xdc..0x102 through the
    /// jump table (player list refresh, room lists, chat room, net result),
    /// anything else to `NetFEState_HandleNetEvent`.
    fn oc915_lobby_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1b_d738, 6);
        oc915_arm(c);
        c.pc = 0x001b_d738;
        let k = seed % 43;
        let round = seed / 43;
        let (p1, p2) = (DATA + 0x21_4000, DATA + 0x21_4400);
        c.bus.write32(OC915_OBJ + 0x10, DATA + 0x21_4800);
        c.bus.write32(OC915_OBJ + 0x774, r.next_u32());
        c.bus.write32(OC915_OBJ + 0x6d4, p1);
        c.bus.write32(p1 + 0xa0, p2);
        c.bus.write32(p2 + 0x18, (3 * round + 2 * k) % 7);
        for i in 0..4 {
            c.bus.write32(OC915_OBJ + 0x708 + 4 * i, r.next_u32());
        }
        c.g[A0][0] = OC915_OBJ as u64;
        c.g[A1][0] = (0xdc + k) as u64;
    }

    #[test]
    fn c_fe_state_lobby_menu_v31_matches() {
        oc915_install_fakes();
        check(
            ee::s03::f_001bd738__generated,
            c_fe_state_lobby_menu_v31,
            129,
            &oc915_lobby_setup,
        );
        let n = oc915_paths(ee::s03::f_001bd738__generated, 129, &oc915_lobby_setup);
        assert!(n >= 5, "only {n} distinct paths");
    }

    /// `cFEStateOnlineMainMenu_OnWidgetAction(self, source, action)`:
    /// action 5 dispatches on `source+0x18` (quick play popup, lobby, stats,
    /// world, virtual call, popups), 6 and 8 configure popups or open the
    /// messenger, anything else (or no source) goes to the base class.
    fn oc915_main_menu_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1b_fe70, 8);
        oc915_arm(c);
        c.pc = 0x001b_fe70;
        oc915_chain(c, oc915_method(seed));
        let sel = seed % 9;
        let round = seed / 9;
        let a2 = [5, 5, 5, 6, 8, 3, 7, 0, 9][sel as usize];
        c.bus.write32(OC915_SRC + 0x18, round % 8);
        c.bus.write32(
            OC915_OBJ + 0x1c,
            [0, 2, 4, 6, 1][(round % 5) as usize] | (r.next_u32() & !7),
        );
        c.g[A0][0] = OC915_OBJ as u64;
        c.g[A1][0] = if seed % 11 == 0 { 0 } else { OC915_SRC as u64 };
        c.g[A2][0] = a2;
    }

    #[test]
    fn c_fe_state_online_main_menu_on_widget_action_matches() {
        oc915_install_fakes();
        check(
            ee::s03::f_001bfe70__generated,
            c_fe_state_online_main_menu_on_widget_action,
            144,
            &oc915_main_menu_setup,
        );
        let n = oc915_paths(ee::s03::f_001bfe70__generated, 144, &oc915_main_menu_setup);
        assert!(n >= 6, "only {n} distinct paths");
    }

    /// `cFEStateOnlineAgreement_OnWidgetAction(self, source, action)`:
    /// action 5 or 6 pushes the screen chosen by `self+0x18` (0x12..0x16:
    /// login, create steps 1 and 2, world), anything else the base class.
    fn oc915_agreement_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x1c_0d28, 6);
        oc915_arm(c);
        c.pc = 0x001c_0d28;
        c.bus.write32(OC915_OBJ + 0x10, DATA + 0x21_4800);
        c.bus.write32(OC915_OBJ + 0x18, 0x10 + (seed / 8) % 9);
        c.g[A0][0] = OC915_OBJ as u64;
        c.g[A1][0] = if seed % 13 == 0 { 0 } else { OC915_SRC as u64 };
        c.g[A2][0] = [5, 6, 3, 8, 5, 5, 6, 0][(seed % 8) as usize];
    }

    #[test]
    fn c_fe_state_online_agreement_on_widget_action_matches() {
        oc915_install_fakes();
        check(
            ee::s03::f_001c0d28__generated,
            c_fe_state_online_agreement_on_widget_action,
            96,
            &oc915_agreement_setup,
        );
        let n = oc915_paths(ee::s03::f_001c0d28__generated, 96, &oc915_agreement_setup);
        assert!(n >= 5, "only {n} distinct paths");
    }

    /// `cFEStateNetQuickPlayPopup_v31(self, event)`: events 0xe6..0x112
    /// through the jump table (challenge error, popups, a virtual call,
    /// character select), anything else to `NetFEState_HandleNetEvent`.
    fn oc915_quick_play_setup(c: &mut Cpu, seed: u32) {
        machine(c, seed, 0x1c_19c8, 6);
        oc915_arm(c);
        c.pc = 0x001c_19c8;
        oc915_chain(c, fakes::FAKE);
        let k = seed % 48;
        c.bus.write32(OC915_OBJ + 0x6f0, seed / 48 % 2);
        c.bus.write32(OC915_OBJ + 0x6ec, 5);
        c.g[A0][0] = OC915_OBJ as u64;
        c.g[A1][0] = (0xe6 + k) as u64;
    }

    #[test]
    fn c_fe_state_net_quick_play_popup_v31_matches() {
        oc915_install_fakes();
        check(
            ee::s03::f_001c19c8__generated,
            c_fe_state_net_quick_play_popup_v31,
            96,
            &oc915_quick_play_setup,
        );
        let n = oc915_paths(ee::s03::f_001c19c8__generated, 96, &oc915_quick_play_setup);
        assert!(n >= 6, "only {n} distinct paths");
    }
}
