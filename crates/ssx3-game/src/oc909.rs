//! Staging module oc909 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc909`.

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
        0x001a_da30,
        c_fe_state_select_persona_v31,
        "cFEStateSelectPersona_v31",
    ),
    (
        0x001a_dfe8,
        c_fe_state_challenge_popup_on_activate,
        "cFEStateChallengePopup_OnActivate",
    ),
    (
        0x001a_e7a0,
        c_fe_state_challenge_popup_on_widget_action,
        "cFEStateChallengePopup_OnWidgetAction",
    ),
    (
        0x001a_f470,
        c_fe_state_challenge_popup_v31,
        "cFEStateChallengePopup_v31",
    ),
];

/// `cFEStateSelectPersona_v31` (0x001ada30): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_select_persona_v31(c: &mut Cpu) {
    if c.pc != 0x001ada30 {
        return ee::s03::f_001ada30__generated(c);
    }
    guest(c, |c| 'l_1ada30: loop {
        'b_1adc1c: {
            'b_1adc18: {
                'b_1adc14: {
                    'b_1adc04: {
                        c.cycles += 9;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd6));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = (c.g[V1][0] < 0x3cu64) as u64;
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S2][0] = c.g[A0][0];
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        let t = c.g[V0][0] == 0u64;
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        if t {
                            break 'b_1adc04;
                        } else {
                            'b_1adbcc: {
                                'b_1adbb8: {
                                    'b_1adb7c: {
                                        'b_1adb6c: {
                                            'b_1adab8: {
                                                c.cycles += 7;
                                                c.g[V0][0] = 0x460000;
                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x2fa0));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x001ada30 => {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001ada30);
                                                        }
                                                        continue 'l_1ada30;
                                                    }
                                                    0x001ada70 => {
                                                        c.cycles += 18;
                                                        c.g[RA][0] = sx(0x001ada78);
                                                        c.g[A0][0] = c.g[S2][0];
                                                        Callee {
                                                            addr: 0x001a9c50,
                                                            f: ee::s03::NetFEState_GetPopup,
                                                        }
                                                        .call(c, 0x001ada78)?;
                                                        c.g[S0][0] = c.g[V0][0];
                                                        c.g[A0][0] = 0x460000;
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0xbc),
                                                            0u64,
                                                        );
                                                        c.g[RA][0] = sx(0x001ada8c);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x2f58));
                                                        call_ptr(c, 0x00317670, 0x001ada8c)?;
                                                        c.g[V1][0] = 0x3;
                                                        c.g[A2][0] = 0x817;
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x164),
                                                            0u64,
                                                        );
                                                        c.g[A0][0] = c.g[S2][0];
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0xc4),
                                                            c.g[V0][0],
                                                        );
                                                        c.g[A1][0] = c.g[S0][0];
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0xc0),
                                                            c.g[V1][0],
                                                        );
                                                        c.g[RA][0] = sx(0x001adab0);
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x150),
                                                            c.g[A2][0],
                                                        );
                                                        Callee {
                                                            addr: 0x001a9bc0,
                                                            f: ee::s03::NetFEState_ShowPopup,
                                                        }
                                                        .call(c, 0x001adab0)?;
                                                        let t = true;
                                                        c.g[V0][0] = 0xffffffffffffffff;
                                                        if t {
                                                            break 'b_1adc18;
                                                        } else {
                                                            break 'b_1adab8;
                                                        }
                                                    }
                                                    0x001adab8 => {
                                                        break 'b_1adab8;
                                                    }
                                                    0x001adb6c => {
                                                        break 'b_1adb6c;
                                                    }
                                                    0x001adb7c => {
                                                        break 'b_1adb7c;
                                                    }
                                                    0x001adbb8 => {
                                                        break 'b_1adbb8;
                                                    }
                                                    0x001adbcc => {
                                                        break 'b_1adbcc;
                                                    }
                                                    0x001adc04 => {
                                                        break 'b_1adc04;
                                                    }
                                                    0x001adc14 => {
                                                        break 'b_1adc14;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x001ada30, 0x001adc30);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            'b_1adacc: {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                                let t = c.g[V0][0] != 0u64;

                                                if t {
                                                    break 'b_1adacc;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[RA][0] = sx(0x001adacc);
                                                    call_ptr(c, 0x00260f80, 0x001adacc)?;
                                                    break 'b_1adacc;
                                                }
                                            }
                                            'b_1adae8: {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001adad4);
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                                call_ptr(c, 0x00261460, 0x001adad4)?;
                                                let t = c.g[V0][0] != 0u64;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                                if t {
                                                    break 'b_1adae8;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[RA][0] = sx(0x001adae4);
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x238)
                                                        );
                                                    call_ptr(c, 0x002613c8, 0x001adae4)?;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x238)
                                                        );
                                                    break 'b_1adae8;
                                                }
                                            }
                                            'b_1adaf8: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] != 0u64;

                                                if t {
                                                    break 'b_1adaf8;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[RA][0] = sx(0x001adaf8);
                                                    c.g[A0][0] = c
                                                        .lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                                    call_ptr(c, 0x0025b800, 0x001adaf8)?;
                                                    break 'b_1adaf8;
                                                }
                                            }
                                            c.cycles += 29;
                                            c.g[RA][0] = sx(0x001adb00);
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                            call_ptr(c, 0x00261408, 0x001adb00)?;
                                            c.g[RA][0] = sx(0x001adb08);
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                            call_ptr(c, 0x0025b800, 0x001adb08)?;
                                            c.g[RA][0] = sx(0x001adb10);
                                            c.g[A0][0] = c.g[S2][0];
                                            Callee {
                                                addr: 0x001a9c50,
                                                f: ee::s03::NetFEState_GetPopup,
                                            }
                                            .call(c, 0x001adb10)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A0][0] = 0x460000;
                                            c.g[V0][0] = 0x1;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x2f78));
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xbc),
                                                c.g[V0][0],
                                            );
                                            c.g[RA][0] = sx(0x001adb2c);
                                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                            call_ptr(c, 0x00317670, 0x001adb2c)?;
                                            c.g[V1][0] = 0x3;
                                            c.g[A0][0] = 0x460000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xc0),
                                                c.g[V1][0],
                                            );
                                            c.g[RA][0] = sx(0x001adb44);
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xc4),
                                                c.g[V0][0],
                                            );
                                            call_ptr(c, 0x00317670, 0x001adb44)?;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A2][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x001adb54);
                                            c.g[A1][0] = 0u64;
                                            Callee {
                                                addr: 0x001c57c0,
                                                f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                            }
                                            .call(c, 0x001adb54)?;
                                            c.g[V0][0] = 0x819;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x164), 0u64);
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x150),
                                                c.g[V0][0],
                                            );
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x001adb6c);
                                            c.g[A0][0] = c.g[S2][0];
                                            Callee {
                                                addr: 0x001a9bc0,
                                                f: ee::s03::NetFEState_ShowPopup,
                                            }
                                            .call(c, 0x001adb6c)?;
                                            break 'b_1adb6c;
                                        }
                                        c.cycles += 4;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                        c.g[A1][0] = 0x1;
                                        c.g[RA][0] = sx(0x001adb7c);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039f190, 0x001adb7c)?;
                                        break 'b_1adb7c;
                                    }
                                    c.cycles += 15;
                                    c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2e00));
                                    c.g[S0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x90));
                                    c.g[RA][0] = sx(0x001adb94);
                                    c.g[S0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                                    call_ptr(c, 0x00317670, 0x001adb94)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001adba0);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001adba0)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x94));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001adbb0);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x001adbb0)?;
                                    let t = true;
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    if t {
                                        break 'b_1adc18;
                                    } else {
                                        break 'b_1adbb8;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x001adbc4);
                                c.g[A1][0] = c.g[A2][0];
                                Callee {
                                    addr: 0x001adc30,
                                    f: ee::s03::cFEStateSelectPersona_OnSubmitResult,
                                }
                                .call(c, 0x001adbc4)?;
                                let t = true;
                                c.g[V0][0] = 0xffffffffffffffff;
                                if t {
                                    break 'b_1adc18;
                                } else {
                                    break 'b_1adbcc;
                                }
                            }
                            c.cycles += 14;
                            c.g[A1][0] = 0x460000;
                            c.g[A0][0] = 0x6d0;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd8));
                            c.g[A2][0] = 0u64;
                            c.g[RA][0] = sx(0x001adbe4);
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x00317d70, 0x001adbe4)?;
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                            c.g[RA][0] = sx(0x001adbf0);
                            c.g[A0][0] = c.g[V0][0];
                            Callee {
                                addr: 0x001bfbb8,
                                f: ee::s03::cFEStateOnlineMainMenu_ctor,
                            }
                            .call(c, 0x001adbf0)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001adbfc);
                            c.g[A1][0] = c.g[V0][0];
                            Callee {
                                addr: 0x001a8a40,
                                f: ee::s03::NetFEState_GotoNextState,
                            }
                            .call(c, 0x001adbfc)?;
                            let t = true;
                            c.g[V0][0] = 0xffffffffffffffff;
                            if t {
                                break 'b_1adc18;
                            } else {
                                break 'b_1adc04;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001adc0c);
                    c.g[A0][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001a8e40,
                        f: ee::s03::NetFEState_HandleNetEvent,
                    }
                    .call(c, 0x001adc0c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1adc1c;
                    } else {
                        break 'b_1adc14;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0xffffffffffffffff;
                break 'b_1adc18;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1adc1c;
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

/// `cFEStateChallengePopup_OnActivate` (0x001adfe8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_challenge_popup_on_activate(c: &mut Cpu) {
    if c.pc != 0x001adfe8 {
        return ee::s03::f_001adfe8__generated(c);
    }
    guest(c, |c| 'l_1adfe8: loop {
        'b_1ae0ac: {
            'b_1ae0a4: {
                c.cycles += 26;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[RA][0] = sx(0x001ae000);
                c.g[S1][0] = c.g[A0][0];
                Callee {
                    addr: 0x001a87d0,
                    f: ee::s03::NetFEState_OnActivate,
                }
                .call(c, 0x001ae000)?;
                c.g[S0][0] = 0x530000;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A1][0] = 0x7;
                c.g[RA][0] = sx(0x001ae014);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                call_ptr(c, 0x0014dd58, 0x001ae014)?;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4b30));
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6dc));
                c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.g[RA][0] = sx(0x001ae028);
                c.g[A1][0] = c.g[A1][0] ^ 0x1;
                call_ptr(c, 0x0039a7a8, 0x001ae028)?;
                c.g[RA][0] = sx(0x001ae030);
                call_ptr(c, 0x0014dc80, 0x001ae030)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001ae03c);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x001ae03c)?;
                c.g[V0][0] = 0x530000;
                c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c12));
                c.g[V1][0] = (c.g[A0][0] < 0x6u64) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x460000;
                if t {
                    break 'b_1ae0a4;
                } else {
                    'b_1ae098: {
                        'b_1ae08c: {
                            'b_1ae080: {
                                'b_1ae074: {
                                    c.cycles += 6;
                                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30f0));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x001adfe8 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001adfe8);
                                            }
                                            continue 'l_1adfe8;
                                        }
                                        0x001ae068 => {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x440000;
                                            let t = true;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x1288));
                                            if t {
                                                break 'b_1ae0ac;
                                            } else {
                                                break 'b_1ae074;
                                            }
                                        }
                                        0x001ae074 => {
                                            break 'b_1ae074;
                                        }
                                        0x001ae080 => {
                                            break 'b_1ae080;
                                        }
                                        0x001ae08c => {
                                            break 'b_1ae08c;
                                        }
                                        0x001ae098 => {
                                            break 'b_1ae098;
                                        }
                                        0x001ae0a4 => {
                                            break 'b_1ae0a4;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x001adfe8, 0x001ae0e0);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x440000;
                                let t = true;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1278));
                                if t {
                                    break 'b_1ae0ac;
                                } else {
                                    break 'b_1ae080;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = 0x440000;
                            let t = true;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1298));
                            if t {
                                break 'b_1ae0ac;
                            } else {
                                break 'b_1ae08c;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x440000;
                        let t = true;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12a8));
                        if t {
                            break 'b_1ae0ac;
                        } else {
                            break 'b_1ae098;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x440000;
                    let t = true;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b8));
                    if t {
                        break 'b_1ae0ac;
                    } else {
                        break 'b_1ae0a4;
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x440000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1268));
            break 'b_1ae0ac;
        }
        c.cycles += 13;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x6d8), c.g[V0][0]);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4b30));
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001ae0c0);
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        Callee {
            addr: 0x001af098,
            f: ee::s03::cFEStateChallengePopup_InitSettingLists,
        }
        .call(c, 0x001ae0c0)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001ae0cc);
        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6d0));
        Callee {
            addr: 0x001aec60,
            f: ee::s03::cFEStateChallengePopup_SetSettingsLocked,
        }
        .call(c, 0x001ae0cc)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateChallengePopup_OnWidgetAction` (0x001ae7a0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_challenge_popup_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001ae7a0 {
        return ee::s03::f_001ae7a0__generated(c);
    }
    guest(c, |c| 'l_1ae7a0: loop {
        'b_1aebe4: {
            'b_1aebe0: {
                'b_1aebdc: {
                    c.cycles += 6;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    let t = c.g[A1][0] == 0u64;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    if t {
                        break 'b_1aebdc;
                    } else {
                        'b_1aebd4: {
                            c.cycles += 4;
                            c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                            c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x460000;
                            if t {
                                break 'b_1aebd4;
                            } else {
                                'b_1aebc0: {
                                    'b_1aebac: {
                                        'b_1aea50: {
                                            'b_1aea40: {
                                                'b_1aea3c: {
                                                    'b_1aea2c: {
                                                        c.cycles += 6;
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x3280));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                        let t = c.g[A0][0] as u32;
                                                        match t {
                                                            0x001ae7a0 => {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x001ae7a0,
                                                                    );
                                                                }
                                                                continue 'l_1ae7a0;
                                                            }
                                                            0x001ae7e0 => {
                                                                'b_1aea0c: {
                                                                    'b_1ae890: {
                                                                        c.cycles += 4;
                                                                        c.g[A0][0] = c.lw((c.g[A1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x18));
                                                                        c.g[V0][0] = 0x9;
                                                                        let t = c.g[A0][0]
                                                                            == c.g[V0][0];
                                                                        c.g[V0][0] = ((c.g[A0][0]
                                                                            as i64)
                                                                            < 10i64)
                                                                            as u64;
                                                                        if t {
                                                                            break 'b_1ae890;
                                                                        } else {
                                                                            'b_1ae81c: {
                                                                                'b_1ae808: {
                                                                                    c.cycles += 2;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        == 0u64;
                                                                                    c.g[V0][0] =
                                                                                        0x7;
                                                                                    if t {
                                                                                        break 'b_1ae808;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t = c.g[A0][0] == c.g[V0][0];
                                                                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                        if t {
                                                                                            break 'b_1ae81c;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            let t =
                                                                                                true;
                                                                                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                                                            if t {
                                                                                                break 'b_1aebe4;
                                                                                            } else {
                                                                                                break 'b_1ae808;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                c.cycles += 3;
                                                                                c.g[V0][0] = 0xa;
                                                                                let t = c.g[A0][0]
                                                                                    == c.g[V0][0];
                                                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                                                if t {
                                                                                    break 'b_1aea0c;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    let t = true;
                                                                                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                                                    if t {
                                                                                        break 'b_1aebe4;
                                                                                    } else {
                                                                                        break 'b_1ae81c;
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 29;
                                                                            c.g[RA][0] =
                                                                                sx(0x001ae824);
                                                                            c.g[A0][0] = c.g[S1][0];
                                                                            Callee { addr: 0x001ae680, f: ee::s03::cFEStateChallengePopup_ApplySettings }.call(c, 0x001ae824)?;
                                                                            c.g[V0][0] = 0x1;
                                                                            c.g[A1][0] = 0x460000;
                                                                            c.g[A3][0] = 0u64;
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0xc38,
                                                                                    ));
                                                                            c.g[A0][0] = 0x33c;
                                                                            c.g[A2][0] = 0x100;
                                                                            c.sw(
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0xb4c,
                                                                                    ),
                                                                                c.g[V0][0],
                                                                            );
                                                                            c.sw(
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0xb5c,
                                                                                    ),
                                                                                0u64,
                                                                            );
                                                                            c.g[RA][0] =
                                                                                sx(0x001ae84c);
                                                                            c.sw(
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0xb58,
                                                                                    ),
                                                                                0u64,
                                                                            );
                                                                            call_ptr(
                                                                                c, 0x00317d70,
                                                                                0x001ae84c,
                                                                            )?;
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [S1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x10,
                                                                                ));
                                                                            c.g[A0][0] = c.g[V0][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x001ae85c);
                                                                            c.g[A2][0] = 0x1;
                                                                            call_ptr(
                                                                                c, 0x00186610,
                                                                                0x001ae85c,
                                                                            )?;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [V0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1c,
                                                                                ));
                                                                            c.g[A0][0] =
                                                                                0xffffffffffffc0ff;
                                                                            c.g[A1][0] = c.g[V0][0];
                                                                            c.g[A2][0] = 0u64;
                                                                            c.g[V1][0] = c.g[V1][0]
                                                                                & c.g[A0][0];
                                                                            c.g[A3][0] = 0u64;
                                                                            c.g[V1][0] =
                                                                                c.g[V1][0] | 0x100;
                                                                            c.sw(
                                                                                (c.g[V0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x1c,
                                                                                    ),
                                                                                c.g[V1][0],
                                                                            );
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x10,
                                                                                ));
                                                                            c.g[RA][0] =
                                                                                sx(0x001ae888);
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x18,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, 0x0039f320,
                                                                                0x001ae888,
                                                                            )?;
                                                                            let t = true;
                                                                            c.lq(
                                                                                16,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x20,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_1aebe0;
                                                                            } else {
                                                                                break 'b_1ae890;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x6d0));
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.lq(
                                                                        16,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x20),
                                                                    );
                                                                    if t {
                                                                        break 'b_1aebe0;
                                                                    } else {
                                                                        'b_1ae9f8: {
                                                                            'b_1ae944: {
                                                                                c.cycles += 23;
                                                                                c.g[RA][0] =
                                                                                    sx(0x001ae8a4);
                                                                                c.g[A0][0] =
                                                                                    c.g[S1][0];
                                                                                Callee { addr: 0x001ae680, f: ee::s03::cFEStateChallengePopup_ApplySettings }.call(c, 0x001ae8a4)?;
                                                                                c.g[RA][0] =
                                                                                    sx(0x001ae8ac);
                                                                                c.g[A0][0] =
                                                                                    c.g[S1][0];
                                                                                Callee { addr: 0x001af568, f: ee::s03::cFEStateChallengePopup_ShowWaitingForChallenge }.call(c, 0x001ae8ac)?;
                                                                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                c.g[A1][0] = 0x7;
                                                                                c.g[RA][0] =
                                                                                    sx(0x001ae8bc);
                                                                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                                                                call_ptr(
                                                                                    c, 0x0014dd58,
                                                                                    0x001ae8bc,
                                                                                )?;
                                                                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6dc));
                                                                                c.g[RA][0] =
                                                                                    sx(0x001ae8c8);
                                                                                c.g[S0][0] =
                                                                                    c.g[V0][0];
                                                                                call_ptr(
                                                                                    c, 0x0039a738,
                                                                                    0x001ae8c8,
                                                                                )?;
                                                                                c.g[V0][0] = (0u64
                                                                                    < c.g[V0][0])
                                                                                    as u64;
                                                                                c.g[V1][0] =
                                                                                    0x530000;
                                                                                c.sw((c.g[V1][0] as u32).wrapping_add(0x4b34), c.g[V0][0]);
                                                                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                                                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                                                let t = c.g[V1][0]
                                                                                    as u32;
                                                                                c.g[RA][0] =
                                                                                    sx(0x001ae8e8);
                                                                                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                call_ptr(
                                                                                    c, t,
                                                                                    0x001ae8e8,
                                                                                )?;
                                                                                c.g[RA][0] =
                                                                                    sx(0x001ae8f0);
                                                                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6dc));
                                                                                call_ptr(
                                                                                    c, 0x0039a738,
                                                                                    0x001ae8f0,
                                                                                )?;
                                                                                let t = c.g[V0][0]
                                                                                    == 0u64;
                                                                                c.g[V1][0] =
                                                                                    0x530000;
                                                                                if t {
                                                                                    break 'b_1ae944;
                                                                                } else {
                                                                                    c.cycles += 19;
                                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                                                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        | 0x1c0;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001ae90c,
                                                                                    );
                                                                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x0014dc80,
                                                                                        0x001ae90c,
                                                                                    )?;
                                                                                    c.g[A0][0] =
                                                                                        c.g[V0][0];
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001ae918,
                                                                                    );
                                                                                    c.g[A1][0] =
                                                                                        0u64;
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x0014dd58,
                                                                                        0x001ae918,
                                                                                    )?;
                                                                                    c.g[S0][0] =
                                                                                        c.g[V0][0];
                                                                                    c.g[A1][0] =
                                                                                        0u64;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001ae928,
                                                                                    );
                                                                                    c.g[A0][0] =
                                                                                        c.g[S0][0];
                                                                                    call_ptr(
                                                                                        c,
                                                                                        0x00144d98,
                                                                                        0x001ae928,
                                                                                    )?;
                                                                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                                                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        as u32;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001ae93c,
                                                                                    );
                                                                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                    call_ptr(
                                                                                        c, t,
                                                                                        0x001ae93c,
                                                                                    )?;
                                                                                    let t = true;
                                                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                                                                    if t {
                                                                                        break 'b_1ae9f8;
                                                                                    } else {
                                                                                        break 'b_1ae944;
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 45;
                                                                            c.g[RA][0] =
                                                                                sx(0x001ae94c);
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x6e8,
                                                                                ));
                                                                            call_ptr(
                                                                                c, 0x0039a738,
                                                                                0x001ae94c,
                                                                            )?;
                                                                            c.g[S0][0] = 0x530000;
                                                                            c.g[V0][0] =
                                                                                c.g[V0][0] & 0x1;
                                                                            c.g[S0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x8b8,
                                                                                    ));
                                                                            c.g[A0][0] =
                                                                                0xffffffffffffffbf;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x18,
                                                                                ));
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    << 6);
                                                                            c.g[V1][0] = c.g[V1][0]
                                                                                & c.g[A0][0];
                                                                            c.g[V1][0] = c.g[V1][0]
                                                                                | c.g[V0][0];
                                                                            c.sw(
                                                                                (c.g[S0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x18,
                                                                                    ),
                                                                                c.g[V1][0],
                                                                            );
                                                                            c.g[RA][0] =
                                                                                sx(0x001ae978);
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x6e4,
                                                                                ));
                                                                            call_ptr(
                                                                                c, 0x0039a738,
                                                                                0x001ae978,
                                                                            )?;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x18,
                                                                                ));
                                                                            c.g[V0][0] =
                                                                                c.g[V0][0] & 0x1;
                                                                            c.g[A0][0] =
                                                                                0xfffffffffffffeff;
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    << 8);
                                                                            c.g[V1][0] = c.g[V1][0]
                                                                                & c.g[A0][0];
                                                                            c.g[V1][0] = c.g[V1][0]
                                                                                | c.g[V0][0];
                                                                            c.sw(
                                                                                (c.g[S0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x18,
                                                                                    ),
                                                                                c.g[V1][0],
                                                                            );
                                                                            c.g[RA][0] =
                                                                                sx(0x001ae99c);
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x6ec,
                                                                                ));
                                                                            call_ptr(
                                                                                c, 0x0039a738,
                                                                                0x001ae99c,
                                                                            )?;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x18,
                                                                                ));
                                                                            c.g[A0][0] =
                                                                                0xffffffffffffff7f;
                                                                            c.g[V0][0] =
                                                                                c.g[V0][0] & 0x1;
                                                                            c.g[V1][0] = c.g[V1][0]
                                                                                & c.g[A0][0];
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    << 7);
                                                                            c.g[V1][0] = c.g[V1][0]
                                                                                | c.g[V0][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x001ae9bc);
                                                                            c.sw(
                                                                                (c.g[S0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x18,
                                                                                    ),
                                                                                c.g[V1][0],
                                                                            );
                                                                            call_ptr(
                                                                                c, 0x0014dc80,
                                                                                0x001ae9bc,
                                                                            )?;
                                                                            c.g[A1][0] = 0u64;
                                                                            c.g[RA][0] =
                                                                                sx(0x001ae9c8);
                                                                            c.g[A0][0] = c.g[V0][0];
                                                                            call_ptr(
                                                                                c, 0x0014dd58,
                                                                                0x001ae9c8,
                                                                            )?;
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [S1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x6e0,
                                                                                ));
                                                                            c.g[RA][0] =
                                                                                sx(0x001ae9d4);
                                                                            c.g[S0][0] = c.g[V0][0];
                                                                            call_ptr(
                                                                                c, 0x0039a738,
                                                                                0x001ae9d4,
                                                                            )?;
                                                                            c.g[A0][0] = c.g[S0][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x001ae9e0);
                                                                            c.g[A1][0] = c.g[V0][0];
                                                                            call_ptr(
                                                                                c, 0x00144d98,
                                                                                0x001ae9e0,
                                                                            )?;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            c.g[A0][0] = c.lh((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [V1][0]
                                                                                as u32)
                                                                                .wrapping_add(0xc));
                                                                            let t =
                                                                                c.g[V0][0] as u32;
                                                                            c.g[RA][0] =
                                                                                sx(0x001ae9f4);
                                                                            c.g[A0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[A0][0]
                                                                                            as u32),
                                                                                    ));
                                                                            call_ptr(
                                                                                c, t, 0x001ae9f4,
                                                                            )?;
                                                                            c.g[A0][0] = c.lw((c.g
                                                                                [GP][0]
                                                                                as u32)
                                                                                .wrapping_sub(
                                                                                    0xc8,
                                                                                ));
                                                                            break 'b_1ae9f8;
                                                                        }
                                                                        c.cycles += 5;
                                                                        c.g[A2][0] = c.lb((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x14));
                                                                        c.g[RA][0] = sx(0x001aea04);
                                                                        c.g[A1][0] = c.lw((c.g[A0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x98));
                                                                        call_ptr(
                                                                            c, 0x0025f4c0,
                                                                            0x001aea04,
                                                                        )?;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x20),
                                                                        );
                                                                        if t {
                                                                            break 'b_1aebe0;
                                                                        } else {
                                                                            break 'b_1aea0c;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 3;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x6d0));
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[A1][0] = 0u64;
                                                                if t {
                                                                    break 'b_1aea3c;
                                                                } else {
                                                                    c.cycles += 5;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0xc8));
                                                                    c.g[RA][0] = sx(0x001aea24);
                                                                    c.g[A2][0] = 0x1;
                                                                    call_ptr(
                                                                        c, 0x0025f628, 0x001aea24,
                                                                    )?;
                                                                    let t = true;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    if t {
                                                                        break 'b_1aea40;
                                                                    } else {
                                                                        break 'b_1aea2c;
                                                                    }
                                                                }
                                                            }
                                                            0x001aea2c => {
                                                                break 'b_1aea2c;
                                                            }
                                                            0x001aea50 => {
                                                                break 'b_1aea50;
                                                            }
                                                            0x001aebac => {
                                                                break 'b_1aebac;
                                                            }
                                                            0x001aebc0 => {
                                                                break 'b_1aebc0;
                                                            }
                                                            0x001aebd4 => {
                                                                break 'b_1aebd4;
                                                            }
                                                            0x001aebdc => {
                                                                break 'b_1aebdc;
                                                            }
                                                            _ => {
                                                                c.jump_out(
                                                                    t, 0x001ae7a0, 0x001aebf0,
                                                                );
                                                                return Ok(());
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                                    c.g[A1][0] = 0u64;
                                                    c.g[RA][0] = sx(0x001aea3c);
                                                    c.g[A2][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x6d0)
                                                        );
                                                    call_ptr(c, 0x0025f628, 0x001aea3c)?;
                                                    break 'b_1aea3c;
                                                }
                                                c.cycles += 1;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                                break 'b_1aea40;
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001aea48);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            call_ptr(c, 0x0039f600, 0x001aea48)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_1aebe0;
                                            } else {
                                                break 'b_1aea50;
                                            }
                                        }
                                        'b_1aeac0: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x6dc));
                                            if c.g[A1][0] != c.g[V0][0] {
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6e8));
                                                break 'b_1aeac0;
                                            } else {
                                                c.cycles += 25;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                c.g[A1][0] = 0x7;
                                                c.g[RA][0] = sx(0x001aea6c);
                                                c.g[A0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                                call_ptr(c, 0x0014dd58, 0x001aea6c)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6dc));
                                                c.g[RA][0] = sx(0x001aea78);
                                                c.g[S0][0] = c.g[V0][0];
                                                call_ptr(c, 0x0039a738, 0x001aea78)?;
                                                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                                                c.g[V1][0] = 0x530000;
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x4b34),
                                                    c.g[V0][0],
                                                );
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001aea98);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001aea98)?;
                                                c.g[RA][0] = sx(0x001aeaa0);
                                                c.g[A0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6dc));
                                                call_ptr(c, 0x0039a738, 0x001aeaa0)?;
                                                c.g[A1][0] = (0u64 < c.g[V0][0]) as u64;
                                                c.g[RA][0] = sx(0x001aeaac);
                                                c.g[A0][0] = c.g[S1][0];
                                                Callee { addr: 0x001af098, f: ee::s03::cFEStateChallengePopup_InitSettingLists }.call(c, 0x001aeaac)?;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[RA][0] = sx(0x001aeab8);
                                                c.g[A1][0] =
                                                    c.lw((c.g[A0][0] as u32).wrapping_add(0x6d0));
                                                Callee { addr: 0x001aec60, f: ee::s03::cFEStateChallengePopup_SetSettingsLocked }.call(c, 0x001aeab8)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                                if t {
                                                    break 'b_1aebe0;
                                                } else {
                                                    break 'b_1aeac0;
                                                }
                                            }
                                        }
                                        'b_1aeb50: {
                                            'b_1aeb40: {
                                                'b_1aeaec: {
                                                    c.cycles += 2;
                                                    if c.g[A1][0] != c.g[V0][0] {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x6e4));
                                                        break 'b_1aeaec;
                                                    } else {
                                                        c.cycles += 9;
                                                        c.g[RA][0] = sx(0x001aead0);
                                                        c.g[A0][0] = c.g[A1][0];
                                                        call_ptr(c, 0x0039a738, 0x001aead0)?;
                                                        c.g[A0][0] = 0x530000;
                                                        c.g[V0][0] = c.g[V0][0] & 0x1;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x8b8));
                                                        c.g[A1][0] = 0xffffffffffffffbf;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                        let t = true;
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                                                        if t {
                                                            break 'b_1aeb40;
                                                        } else {
                                                            break 'b_1aeaec;
                                                        }
                                                    }
                                                }
                                                'b_1aeb18: {
                                                    c.cycles += 2;
                                                    if c.g[A1][0] != c.g[V0][0] {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x6ec));
                                                        break 'b_1aeb18;
                                                    } else {
                                                        c.cycles += 9;
                                                        c.g[RA][0] = sx(0x001aeafc);
                                                        c.g[A0][0] = c.g[A1][0];
                                                        call_ptr(c, 0x0039a738, 0x001aeafc)?;
                                                        c.g[A0][0] = 0x530000;
                                                        c.g[V0][0] = c.g[V0][0] & 0x1;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x8b8));
                                                        c.g[A1][0] = 0xfffffffffffffeff;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                        let t = true;
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                                                        if t {
                                                            break 'b_1aeb40;
                                                        } else {
                                                            break 'b_1aeb18;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                if c.g[A1][0] != c.g[V0][0] {
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x6e0)
                                                        );
                                                    break 'b_1aeb50;
                                                } else {
                                                    c.cycles += 8;
                                                    c.g[RA][0] = sx(0x001aeb28);
                                                    c.g[A0][0] = c.g[A1][0];
                                                    call_ptr(c, 0x0039a738, 0x001aeb28)?;
                                                    c.g[A0][0] = 0x530000;
                                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                                                    c.g[A1][0] = 0xffffffffffffff7f;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[A0][0] as u32).wrapping_add(0x18));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 7);
                                                    break 'b_1aeb40;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                                            c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                                            let t = true;
                                            c.sw(
                                                (c.g[A0][0] as u32).wrapping_add(0x18),
                                                c.g[V1][0],
                                            );
                                            if t {
                                                break 'b_1aebdc;
                                            } else {
                                                break 'b_1aeb50;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[A1][0] != c.g[V0][0];
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                        if t {
                                            break 'b_1aebe0;
                                        } else {
                                            c.cycles += 21;
                                            c.g[RA][0] = sx(0x001aeb60);
                                            call_ptr(c, 0x0014dc80, 0x001aeb60)?;
                                            c.g[A1][0] = 0u64;
                                            c.g[RA][0] = sx(0x001aeb6c);
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x0014dd58, 0x001aeb6c)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x6e0));
                                            c.g[RA][0] = sx(0x001aeb78);
                                            c.g[S0][0] = c.g[V0][0];
                                            call_ptr(c, 0x0039a738, 0x001aeb78)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x001aeb84);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x00144d98, 0x001aeb84)?;
                                            c.g[RA][0] = sx(0x001aeb8c);
                                            c.g[A0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x6e0));
                                            call_ptr(c, 0x0039a738, 0x001aeb8c)?;
                                            c.sw(
                                                (c.g[GP][0] as u32).wrapping_add(0x1d9c),
                                                c.g[V0][0],
                                            );
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001aeba4);
                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001aeba4)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_1aebe0;
                                            } else {
                                                break 'b_1aebac;
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x001aebb8);
                                    c.g[A2][0] = 0x1;
                                    Callee {
                                        addr: 0x001af208,
                                        f: ee::s03::cFEStateChallengePopup_UpdateInformArrows,
                                    }
                                    .call(c, 0x001aebb8)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1aebe0;
                                    } else {
                                        break 'b_1aebc0;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x001aebcc);
                                c.g[A2][0] = 0u64;
                                Callee {
                                    addr: 0x001af208,
                                    f: ee::s03::cFEStateChallengePopup_UpdateInformArrows,
                                }
                                .call(c, 0x001aebcc)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1aebe0;
                                } else {
                                    break 'b_1aebd4;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x001aebdc);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x001a8918,
                            f: ee::s03::NetFEState_OnWidgetAction,
                        }
                        .call(c, 0x001aebdc)?;
                        break 'b_1aebdc;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_1aebe0;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1aebe4;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateChallengePopup_v31` (0x001af470): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_challenge_popup_v31(c: &mut Cpu) {
    if c.pc != 0x001af470 {
        return ee::s03::f_001af470__generated(c);
    }
    guest(c, |c| 'l_1af470: loop {
        'b_1af558: {
            'b_1af554: {
                'b_1af550: {
                    'b_1af53c: {
                        c.cycles += 9;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S1][0] = c.g[A1][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xe3));
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        c.g[V0][0] = (c.g[V1][0] < 0x31u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[S0][0] = c.g[A0][0];
                        if t {
                            break 'b_1af53c;
                        } else {
                            'b_1af528: {
                                'b_1af514: {
                                    'b_1af504: {
                                        'b_1af4e8: {
                                            c.cycles += 7;
                                            c.g[V0][0] = 0x460000;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x33a0));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                            let t = c.g[A0][0] as u32;
                                            match t {
                                                0x001af470 => {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x001af470);
                                                    }
                                                    continue 'l_1af470;
                                                }
                                                0x001af4b0 => {
                                                    c.cycles += 14;
                                                    c.g[A1][0] = 0x460000;
                                                    c.g[A0][0] = 0x6e4;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x1c60));
                                                    c.g[A2][0] = 0x100;
                                                    c.g[RA][0] = sx(0x001af4c8);
                                                    c.g[A3][0] = 0u64;
                                                    call_ptr(c, 0x00317d70, 0x001af4c8)?;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                    c.g[RA][0] = sx(0x001af4d4);
                                                    c.g[A0][0] = c.g[V0][0];
                                                    Callee { addr: 0x001b0f60, f: ee::s03::cFEStateOnlineCharacterSelect_ctor }.call(c, 0x001af4d4)?;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x001af4e0);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    Callee {
                                                        addr: 0x001a8a40,
                                                        f: ee::s03::NetFEState_GotoNextState,
                                                    }
                                                    .call(c, 0x001af4e0)?;
                                                    let t = true;
                                                    c.g[V0][0] = 0u64;
                                                    if t {
                                                        break 'b_1af554;
                                                    } else {
                                                        break 'b_1af4e8;
                                                    }
                                                }
                                                0x001af4e8 => {
                                                    break 'b_1af4e8;
                                                }
                                                0x001af504 => {
                                                    break 'b_1af504;
                                                }
                                                0x001af514 => {
                                                    break 'b_1af514;
                                                }
                                                0x001af528 => {
                                                    break 'b_1af528;
                                                }
                                                0x001af53c => {
                                                    break 'b_1af53c;
                                                }
                                                0x001af550 => {
                                                    break 'b_1af550;
                                                }
                                                _ => {
                                                    c.jump_out(t, 0x001af470, 0x001af568);
                                                    return Ok(());
                                                }
                                            }
                                        }
                                        c.cycles += 7;
                                        c.g[RA][0] = sx(0x001af4f0);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x001af688,
                                            f: ee::s03::cFEStateChallengePopup_ShowSetOptions,
                                        }
                                        .call(c, 0x001af4f0)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001af4fc);
                                        c.g[A1][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001ab178,
                                            f: ee::s03::NetFEState_ShowChallengeErrorPopup,
                                        }
                                        .call(c, 0x001af4fc)?;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_1af554;
                                        } else {
                                            break 'b_1af504;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001af50c);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001ab108,
                                        f: ee::s03::NetFEState_ShowWaitPlayerPopup,
                                    }
                                    .call(c, 0x001af50c)?;
                                    let t = true;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_1af554;
                                    } else {
                                        break 'b_1af514;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001af520);
                                c.g[A1][0] = c.g[A2][0];
                                Callee {
                                    addr: 0x001afa60,
                                    f: ee::s03::NetFE_ShowPlayerStats,
                                }
                                .call(c, 0x001af520)?;
                                let t = true;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_1af554;
                                } else {
                                    break 'b_1af528;
                                }
                            }
                            c.cycles += 5;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001af534);
                            c.g[A1][0] = c.g[A2][0];
                            Callee {
                                addr: 0x001afd88,
                                f: ee::s03::NetFE_ShowPlayerConnQuality,
                            }
                            .call(c, 0x001af534)?;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_1af554;
                            } else {
                                break 'b_1af53c;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001af548);
                    c.g[A1][0] = c.g[S1][0];
                    Callee {
                        addr: 0x001a8e40,
                        f: ee::s03::NetFEState_HandleNetEvent,
                    }
                    .call(c, 0x001af548)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1af558;
                    } else {
                        break 'b_1af550;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_1af554;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1af558;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::{check, DATA, GAME_GP};

    /// Test object of the challenge/persona states.
    const OC909_OBJ: u32 = DATA + 0x1000;
    /// Zeroed sub-objects the state's pointer fields point at.
    const OC909_SUB: u32 = DATA + 0x4000;

    /// Fill the jump table at `base` (`n` entries) from `labels` (some
    /// entries a foreign address, which leaves through `jump_out`).
    fn oc909_table(c: &mut Cpu, seed: u32, base: u32, n: u32, labels: &[u32]) {
        for i in 0..n {
            let k = (seed / 3).wrapping_add(i.wrapping_mul(7)) % (labels.len() as u32 + 1);
            let v = labels
                .get(k as usize)
                .copied()
                .unwrap_or(0x001a_0000 + 4 * i);
            c.bus.write32(base + 4 * i, v);
        }
    }

    /// The state object: pointer fields at `+0x10`, `+0x6d0`, `+0x6d8`, `+0x6dc`
    /// (and their targets), the world at `gp-0x848`.
    fn oc909_object(c: &mut Cpu, seed: u32) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x100);
        c.bus.write32(DATA + 0x100 + 0x78, DATA + 0x200);
        for (i, off) in [0x10u32, 0x6d0, 0x6d8, 0x6dc, 0x6e0, 0x6e4, 0x6e8, 0x6ec]
            .into_iter()
            .enumerate()
        {
            c.bus.write32(OC909_OBJ + off, OC909_SUB + 0x200 * i as u32);
        }
        c.bus.write32(OC909_OBJ + 0x14, seed & 0xff);
        c.g[A0][0] = sx(OC909_OBJ);
    }

    /// `cFEStateSelectPersona_v31(self, event)`: event dispatch through the
    /// jump table at `0x462fa0` (events `0xd6..0x112`; out-of-range and foreign
    /// table entries included). Runs the real net-state callees.
    #[test]
    fn c_fe_state_select_persona_v31_matches() {
        check(
            ee::s03::f_001ada30__generated,
            c_fe_state_select_persona_v31,
            120,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x909_a30, 0);
                oc909_object(c, seed);
                oc909_table(
                    c,
                    seed,
                    0x0046_2fa0,
                    0x3c,
                    &[
                        0x001a_da70,
                        0x001a_dab8,
                        0x001a_db6c,
                        0x001a_db7c,
                        0x001a_dbb8,
                        0x001a_dbcc,
                        0x001a_dc04,
                        0x001a_dc14,
                    ],
                );
                c.g[A1][0] = sx(0xd6 + (seed * 5) % 0x44);
                c.pc = 0x001a_da30;
            },
        );
    }

    /// `cFEStateChallengePopup_OnActivate(self)`: the base activation, then a
    /// byte at `0x535c12` (0..6, or above) picks a message id through the jump
    /// table at `0x4630f0`, stored at `+0x6d8`; then the list setup.
    #[test]
    fn c_fe_state_challenge_popup_on_activate_matches() {
        check(
            ee::s03::f_001adfe8__generated,
            c_fe_state_challenge_popup_on_activate,
            120,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x909_fe8, 0);
                oc909_object(c, seed);
                c.bus.write32(0x0053_4b30, DATA + 0x300);
                c.bus.write32(DATA + 0x304, seed & 1);
                c.bus.write8(0x0053_5c12, (seed % 8) as u8);
                oc909_table(
                    c,
                    seed,
                    0x0046_30f0,
                    6,
                    &[
                        0x001a_e068,
                        0x001a_e074,
                        0x001a_e080,
                        0x001a_e08c,
                        0x001a_e098,
                        0x001a_e0a4,
                    ],
                );
                c.pc = 0x001a_dfe8;
            },
        );
    }

    /// `cFEStateChallengePopup_OnWidgetAction(self, widget, action)`: action
    /// `1..10` through the jump table at `0x463280`; `widget == 0` leaves early.
    #[test]
    fn c_fe_state_challenge_popup_on_widget_action_matches() {
        check(
            ee::s03::f_001ae7a0__generated,
            c_fe_state_challenge_popup_on_widget_action,
            120,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x909_7a0, 0);
                oc909_object(c, seed);
                oc909_table(
                    c,
                    seed,
                    0x0046_3280,
                    9,
                    &[
                        0x001a_e7e0,
                        0x001a_ea2c,
                        0x001a_ea50,
                        0x001a_ebac,
                        0x001a_ebc0,
                        0x001a_ebd4,
                        0x001a_ebdc,
                    ],
                );
                let widget = DATA + 0x800;
                c.bus.write32(widget + 0x18, seed % 12);
                c.bus.write32(widget + 0xc, DATA + 0x900);
                c.bus.write32(DATA + 0x900 + 0xc, DATA + 0xa00);
                c.g[A1][0] = if seed % 11 == 0 { 0 } else { sx(widget) };
                c.g[A2][0] = sx(seed % 12);
                c.pc = 0x001a_e7a0;
            },
        );
    }

    /// `cFEStateChallengePopup_v31(self, event)`: event `0xe3..0x114` through
    /// the jump table at `0x4633a0`.
    #[test]
    fn c_fe_state_challenge_popup_v31_matches() {
        check(
            ee::s03::f_001af470__generated,
            c_fe_state_challenge_popup_v31,
            120,
            &|c, seed| {
                let _ = machine_rt(c, seed, 0x909_470, 0);
                oc909_object(c, seed);
                oc909_table(
                    c,
                    seed,
                    0x0046_33a0,
                    0x31,
                    &[
                        0x001a_f4b0,
                        0x001a_f4e8,
                        0x001a_f504,
                        0x001a_f514,
                        0x001a_f528,
                        0x001a_f53c,
                        0x001a_f550,
                    ],
                );
                c.g[A1][0] = sx(0xe3 + (seed * 5) % 0x38);
                c.pc = 0x001a_f470;
            },
        );
    }
}
