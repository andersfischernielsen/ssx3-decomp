//! Staging module oc959 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc959`.

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
        0x001e4200,
        ov_screen_message_center_on_widget_action,
        "OVScreenMessageCenter_OnWidgetAction",
    ),
    (
        0x001e4578,
        ov_screen_message_center_on_widget_notify,
        "OVScreenMessageCenter_OnWidgetNotify",
    ),
    (
        0x001e4f70,
        ov_screen_message_center_set_row_subject,
        "OVScreenMessageCenter_SetRowSubject",
    ),
    (
        0x001e5638,
        c_ov_state_view_popup_set_variant_subject,
        "cOVState_ViewPopup_SetVariantSubject",
    ),
];

/// `OVScreenMessageCenter_OnWidgetAction` (0x001e4200): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_message_center_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001e4200 {
        return ee::s04::f_001e4200__generated(c);
    }
    guest(c, |c| {
        'b_1e4438: {
            'b_1e4434: {
                'b_1e4430: {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S0][0] = c.g[A2][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S4][0] = c.g[A1][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[S4][0] == 0u64;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    if t {
                        break 'b_1e4430;
                    } else {
                        'b_1e43f8: {
                            'b_1e43ac: {
                                'b_1e439c: {
                                    'b_1e436c: {
                                        'b_1e4294: {
                                            c.cycles += 12;
                                            c.g[RA][0] = sx(0x001e4238);
                                            Callee {
                                                addr: 0x001e39f8,
                                                f: ee::s04::MessageCenter_GetInbox,
                                            }
                                            .call(c, 0x001e4238)?;
                                            c.g[A0][0] = 0x4;
                                            c.g[A1][0] = 0x1;
                                            c.g[RA][0] = sx(0x001e4248);
                                            c.g[S5][0] = c.g[V0][0];
                                            Callee {
                                                addr: 0x001e2bb8,
                                                f: ee::s04::MessageCenter_CountGroupMessages,
                                            }
                                            .call(c, 0x001e4248)?;
                                            c.g[RA][0] = sx(0x001e4250);
                                            c.g[S2][0] = c.g[V0][0];
                                            Callee {
                                                addr: 0x001e33e8,
                                                f: ee::s04::MessageCenter_GetMessageCount,
                                            }
                                            .call(c, 0x001e4250)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[V0][0] = 0x5;
                                            let t = c.g[S0][0] == c.g[V0][0];
                                            c.g[V0][0] = (c.g[S0][0] < 0x6u64) as u64;
                                            if t {
                                                break 'b_1e4294;
                                            } else {
                                                'b_1e4278: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x1;
                                                    if t {
                                                        break 'b_1e4278;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[S0][0] == c.g[V0][0];
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x60),
                                                        );
                                                        if t {
                                                            break 'b_1e43f8;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.lq(
                                                                17,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x50),
                                                            );
                                                            if t {
                                                                break 'b_1e4438;
                                                            } else {
                                                                break 'b_1e4278;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x6;
                                                let t = c.g[S0][0] == c.g[V0][0];
                                                c.g[V0][0] = 0x7;
                                                if t {
                                                    break 'b_1e43ac;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[S0][0] == c.g[V0][0];
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x60),
                                                    );
                                                    if t {
                                                        break 'b_1e436c;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.lq(
                                                            17,
                                                            (c.g[SP][0] as u32).wrapping_add(0x50),
                                                        );
                                                        if t {
                                                            break 'b_1e4438;
                                                        } else {
                                                            break 'b_1e4294;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            sx((c.g[S2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        let t = (c.g[V0][0] as i64) <= 0;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                        if t {
                                            break 'b_1e4434;
                                        } else {
                                            'b_1e435c: {
                                                'b_1e42e0: {
                                                    c.cycles += 6;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x144)
                                                        );
                                                    c.g[S3][0] = c.lbu(
                                                        (c.g[V0][0] as u32).wrapping_add(0x95),
                                                    );
                                                    c.g[V1][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_sub((c.g[S3][0] as u32)));
                                                    c.g[S0][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                                    let t = (c.g[S0][0] as i64) < 0;
                                                    c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[S2][0] as u32)));
                                                    if t {
                                                        break 'b_1e42e0;
                                                    } else {
                                                        c.cycles += 10;
                                                        c.g[RA][0] = sx(0x001e42c0);
                                                        c.g[A0][0] = c.g[S0][0];
                                                        Callee {
                                                            addr: 0x001e3620,
                                                            f: ee::s04::MessageCenter_GetMessageId,
                                                        }
                                                        .call(c, 0x001e42c0)?;
                                                        c.g[A2][0] = c
                                                            .lw((c.g[S5][0] as u32)
                                                                .wrapping_add(0xc8));
                                                        c.g[V1][0] = 0x1;
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            << ((c.g[S0][0] as u32) & 31));
                                                        c.g[A0][0] = c.g[S1][0];
                                                        c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
                                                        c.g[A1][0] = c.g[V0][0];
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[S5][0] as u32).wrapping_add(0xc8),
                                                            c.g[A2][0],
                                                        );
                                                        if t {
                                                            break 'b_1e435c;
                                                        } else {
                                                            break 'b_1e42e0;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = (c.g[V0][0] as i64) < 0;
                                                c.g[V0][0] = 0x4c0000;
                                                if t {
                                                    break 'b_1e4430;
                                                } else {
                                                    'b_1e4358: {
                                                        c.cycles += 14;
                                                        c.g[A2][0] = c
                                                            .lw((c.g[S4][0] as u32)
                                                                .wrapping_add(0x18));
                                                        c.g[A0][0] = 0x18;
                                                        c.g[A1][0] = 0x14;
                                                        c.mult(0, 4, 6, 4);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x6c08));
                                                        c.g[V1][0] = 0x440000;
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add(0x1630));
                                                        c.g[S0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.mult(0, 5, 4, 5);
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add((c.g[V1][0] as u32)));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A1][0] as u32)
                                                                .wrapping_add(0x10));
                                                        if c.g[V0][0] == 0u64 {
                                                            c.g[A1][0] = c.g[A2][0];
                                                            break 'b_1e4358;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32)
                                                                .wrapping_add(0x4));
                                                            let t = c.g[A2][0] != c.g[V0][0];
                                                            c.g[A1][0] = c.g[A2][0];
                                                            if t {
                                                                break 'b_1e4358;
                                                            } else {
                                                                c.cycles += 11;
                                                                c.g[RA][0] = sx(0x001e4334);
                                                                Callee { addr: 0x001e3a30, f: ee::s04::MessageCenter_IsFlagSet }.call(c, 0x001e4334)?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                c.g[RA][0] = sx(0x001e4340);
                                                                c.g[A1][0] = c.g[V0][0] ^ 0x1;
                                                                Callee { addr: 0x001e3a78, f: ee::s04::MessageCenter_SetFlag }.call(c, 0x001e4340)?;
                                                                c.g[RA][0] = sx(0x001e4348);
                                                                c.g[A0][0] = c.g[S1][0];
                                                                Callee { addr: 0x001e48b0, f: ee::s04::OVScreenMessageCenter_RefreshList }.call(c, 0x001e4348)?;
                                                                c.g[RA][0] = sx(0x001e4350);
                                                                c.g[A0][0] = c.g[S1][0];
                                                                Callee { addr: 0x001e4658, f: ee::s04::OVScreenMessageCenter_UpdateScrollArrows }.call(c, 0x001e4350)?;
                                                                let t = true;

                                                                if t {
                                                                    break 'b_1e439c;
                                                                } else {
                                                                    break 'b_1e4358;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.g[A0][0] = c.g[S1][0];
                                                    break 'b_1e435c;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001e4364);
                                            c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                                            Callee {
                                                addr: 0x001e50c8,
                                                f: ee::s04::OVScreenMessageCenter_OpenViewPopup,
                                            }
                                            .call(c, 0x001e4364)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                            if t {
                                                break 'b_1e4434;
                                            } else {
                                                break 'b_1e436c;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = (c.g[A0][0] as i64) <= 0;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x170), 0u64);
                                    if t {
                                        break 'b_1e4430;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x144));
                                        c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x95));
                                        c.g[V0][0] =
                                            sx((c.g[A0][0] as u32)
                                                .wrapping_sub((c.g[V0][0] as u32)));
                                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        let t = (c.g[A1][0] as i64) < 0;

                                        if t {
                                            break 'b_1e439c;
                                        } else {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001e4394);
                                            c.g[A0][0] = c.g[S1][0];
                                            Callee {
                                                addr: 0x001e5148,
                                                f: ee::s04::OVScreenMessageCenter_DeleteMessage,
                                            }
                                            .call(c, 0x001e4394)?;
                                            c.g[V0][0] = 0x1;
                                            c.sw(
                                                (c.g[S1][0] as u32).wrapping_add(0x170),
                                                c.g[V0][0],
                                            );
                                            break 'b_1e439c;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001e43a4);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001e4458,
                                    f: ee::s04::OVScreenMessageCenter_UpdateButtonText,
                                }
                                .call(c, 0x001e43a4)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                if t {
                                    break 'b_1e4434;
                                } else {
                                    break 'b_1e43ac;
                                }
                            }
                            'b_1e43e0: {
                                c.cycles += 10;
                                c.g[A0][0] = 0x470000;
                                c.g[RA][0] = sx(0x001e43b8);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                                call_ptr(c, 0x00317670, 0x001e43b8)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x17c));
                                c.g[RA][0] = sx(0x001e43c4);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039c778, 0x001e43c4)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = 0xffff;
                                let t = c.g[A1][0] == c.g[V0][0];
                                c.g[A1][0] = c.g[A1][0] & 0xffff;
                                if t {
                                    break 'b_1e43e0;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x17c));
                                    c.g[RA][0] = sx(0x001e43e0);
                                    c.g[A2][0] = 0x1;
                                    call_ptr(c, 0x0039c870, 0x001e43e0)?;
                                    break 'b_1e43e0;
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[A1][0] = 0x1;
                            c.g[RA][0] = sx(0x001e43f0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039f190, 0x001e43f0)?;
                            let t = true;
                            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1030), 0u64);
                            if t {
                                break 'b_1e4430;
                            } else {
                                break 'b_1e43f8;
                            }
                        }
                        c.cycles += 4;
                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xfc));
                        c.g[S1][0] = 0x7;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                        'l_1e4408: loop {
                            c.cycles += 10;
                            c.g[A1][0] = 0u64;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001e4428);
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001e4428)?;
                            if (c.g[S1][0] as i64) >= 0 {
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                                if event_due(c) {
                                    deliver_event_here(c, 0x001e4408);
                                }
                                continue 'l_1e4408;
                            } else {
                                break 'b_1e4430;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                break 'b_1e4434;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_1e4438;
        }
        c.cycles += 7;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenMessageCenter_OnWidgetNotify` (0x001e4578): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_message_center_on_widget_notify(c: &mut Cpu) {
    if c.pc != 0x001e4578 {
        return ee::s04::f_001e4578__generated(c);
    }
    guest(c, |c| 'l_1e4578: loop {
        'b_1e45fc: {
            'b_1e45f8: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x4));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_1e45f8;
                } else {
                    'b_1e45f0: {
                        'b_1e45e4: {
                            'b_1e45dc: {
                                'b_1e45cc: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x470000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1f70));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x001e4578 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001e4578);
                                            }
                                            continue 'l_1e4578;
                                        }
                                        0x001e45b0 => {
                                            c.cycles += 7;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x001e45bc);
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x174),
                                                c.g[A3][0],
                                            );
                                            Callee { addr: 0x001e4658, f: ee::s04::OVScreenMessageCenter_UpdateScrollArrows }.call(c, 0x001e45bc)?;
                                            c.g[RA][0] = sx(0x001e45c4);
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x001e48b0,
                                                f: ee::s04::OVScreenMessageCenter_RefreshList,
                                            }
                                            .call(c, 0x001e45c4)?;
                                            let t = true;
                                            c.g[V0][0] = 0x101;
                                            if t {
                                                break 'b_1e45fc;
                                            } else {
                                                break 'b_1e45cc;
                                            }
                                        }
                                        0x001e45cc => {
                                            break 'b_1e45cc;
                                        }
                                        0x001e45dc => {
                                            break 'b_1e45dc;
                                        }
                                        0x001e45e4 => {
                                            break 'b_1e45e4;
                                        }
                                        0x001e45f0 => {
                                            break 'b_1e45f0;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x001e4578, 0x001e460c);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001e45d4);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001e4458,
                                    f: ee::s04::OVScreenMessageCenter_UpdateButtonText,
                                }
                                .call(c, 0x001e45d4)?;
                                let t = true;
                                c.g[V0][0] = 0x101;
                                if t {
                                    break 'b_1e45fc;
                                } else {
                                    break 'b_1e45dc;
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_1e45fc;
                            } else {
                                break 'b_1e45e4;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x170));
                        let t = true;
                        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                        if t {
                            break 'b_1e45fc;
                        } else {
                            break 'b_1e45f0;
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_1e45fc;
                    } else {
                        break 'b_1e45f8;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x101;
            break 'b_1e45fc;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenMessageCenter_SetRowSubject` (0x001e4f70): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_message_center_set_row_subject(c: &mut Cpu) {
    if c.pc != 0x001e4f70 {
        return ee::s04::f_001e4f70__generated(c);
    }
    guest(c, |c| {
        'b_1e5064: {
            'b_1e5060: {
                'b_1e503c: {
                    'b_1e502c: {
                        'b_1e4ff0: {
                            c.cycles += 15;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.g[S0][0] = c.g[A2][0];
                            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.g[S2][0] = c.g[A1][0];
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.g[S3][0] = c.g[A0][0];
                            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                            c.g[S1][0] = c.g[A3][0] & 0xff;
                            c.g[A0][0] = 0x3;
                            c.g[RA][0] = sx(0x001e4fa4);
                            c.g[A1][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001e2d58,
                                f: ee::s04::MessageCenter_IsMessageOfType,
                            }
                            .call(c, 0x001e4fa4)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x73));
                            if t {
                                break 'b_1e4ff0;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[V1][0] < 0x18u64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x470000;
                                if t {
                                    break 'b_1e5060;
                                } else {
                                    'b_1e4fe0: {
                                        c.cycles += 6;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1e20));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001e4fd0 => {
                                                c.cycles += 4;
                                                c.g[A1][0] = 0x470000;
                                                c.g[A2][0] = c.g[S1][0];
                                                let t = true;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x1ec0));
                                                if t {
                                                    break 'b_1e502c;
                                                } else {
                                                    break 'b_1e4fe0;
                                                }
                                            }
                                            0x001e4fe0 => {
                                                break 'b_1e4fe0;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001e4f70, 0x001e50c4);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A1][0] = 0x470000;
                                    c.g[A2][0] = c.g[S1][0];
                                    let t = true;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1ea0));
                                    if t {
                                        break 'b_1e502c;
                                    } else {
                                        break 'b_1e4ff0;
                                    }
                                }
                            }
                        }
                        'b_1e5010: {
                            c.cycles += 5;
                            c.g[A0][0] = 0x5;
                            c.g[RA][0] = sx(0x001e4ffc);
                            c.g[A1][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001e2d58,
                                f: ee::s04::MessageCenter_IsMessageOfType,
                            }
                            .call(c, 0x001e4ffc)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0x470000;
                            if t {
                                break 'b_1e5010;
                            } else {
                                c.cycles += 3;
                                c.g[A2][0] = c.g[S1][0];
                                let t = true;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1e80));
                                if t {
                                    break 'b_1e502c;
                                } else {
                                    break 'b_1e5010;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[A0][0] = 0x4;
                        c.g[RA][0] = sx(0x001e501c);
                        c.g[A1][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001e2d58,
                            f: ee::s04::MessageCenter_IsMessageOfType,
                        }
                        .call(c, 0x001e501c)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0x470000;
                        if t {
                            break 'b_1e503c;
                        } else {
                            c.cycles += 2;
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1e60));
                            break 'b_1e502c;
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e5034);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00417828, 0x001e5034)?;
                    let t = true;
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                    if t {
                        break 'b_1e5064;
                    } else {
                        break 'b_1e503c;
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001e5048);
                c.g[A0][0] = 0x8;
                Callee {
                    addr: 0x001e2d58,
                    f: ee::s04::MessageCenter_IsMessageOfType,
                }
                .call(c, 0x001e5048)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x470000;
                if t {
                    break 'b_1e5060;
                } else {
                    c.cycles += 4;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1e40));
                    c.g[RA][0] = sx(0x001e5060);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00417828, 0x001e5060)?;
                    break 'b_1e5060;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
            break 'b_1e5064;
        }
        'b_1e50ac: {
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0xfc));
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32));
            if c.g[A0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                break 'b_1e50ac;
            } else {
                c.cycles += 13;
                c.g[RA][0] = sx(0x001e5080);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x001e5080)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32));
                c.g[RA][0] = sx(0x001e508c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001e508c)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001e50a8);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001e50a8)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                break 'b_1e50ac;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

/// `cOVState_ViewPopup_SetVariantSubject` (0x001e5638): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_ov_state_view_popup_set_variant_subject(c: &mut Cpu) {
    if c.pc != 0x001e5638 {
        return ee::s04::f_001e5638__generated(c);
    }
    guest(c, |c| {
        'b_1e5680: {
            'b_1e566c: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x102c));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A0][0];
                let t = c.g[V0][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                if t {
                    break 'b_1e566c;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e5664);
                    Callee {
                        addr: 0x001e5aa0,
                        f: ee::s04::cOVState_ViewPopup_GetGroupVariant,
                    }
                    .call(c, 0x001e5664)?;
                    let t = true;
                    c.g[S1][0] = c.g[V0][0];
                    if t {
                        break 'b_1e5680;
                    } else {
                        break 'b_1e566c;
                    }
                }
            }
            c.cycles += 5;
            c.g[RA][0] = sx(0x001e5674);
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x001e3388,
                f: ee::s04::MessageCenter_FindMessage,
            }
            .call(c, 0x001e5674)?;
            c.g[RA][0] = sx(0x001e567c);
            c.g[A0][0] = c.g[V0][0];
            Callee {
                addr: 0x001e3650,
                f: ee::s04::MessageCenter_GetMessageVariant,
            }
            .call(c, 0x001e567c)?;
            c.g[S1][0] = c.g[V0][0];
            break 'b_1e5680;
        }
        'b_1e574c: {
            'b_1e5748: {
                'b_1e5724: {
                    'b_1e5714: {
                        'b_1e56d8: {
                            c.cycles += 5;
                            c.g[A0][0] = 0x3;
                            c.g[RA][0] = sx(0x001e568c);
                            c.g[A1][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001e2d58,
                                f: ee::s04::MessageCenter_IsMessageOfType,
                            }
                            .call(c, 0x001e568c)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x73));
                            if t {
                                break 'b_1e56d8;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[V1][0] < 0x18u64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x470000;
                                if t {
                                    break 'b_1e5748;
                                } else {
                                    'b_1e56c8: {
                                        c.cycles += 6;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1d20));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001e56b8 => {
                                                c.cycles += 4;
                                                c.g[A1][0] = 0x470000;
                                                c.g[A2][0] = c.g[S1][0];
                                                let t = true;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0x1ec0));
                                                if t {
                                                    break 'b_1e5714;
                                                } else {
                                                    break 'b_1e56c8;
                                                }
                                            }
                                            0x001e56c8 => {
                                                break 'b_1e56c8;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001e5638, 0x001e5780);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A1][0] = 0x470000;
                                    c.g[A2][0] = c.g[S1][0];
                                    let t = true;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1ea0));
                                    if t {
                                        break 'b_1e5714;
                                    } else {
                                        break 'b_1e56d8;
                                    }
                                }
                            }
                        }
                        'b_1e56f8: {
                            c.cycles += 5;
                            c.g[A0][0] = 0x5;
                            c.g[RA][0] = sx(0x001e56e4);
                            c.g[A1][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001e2d58,
                                f: ee::s04::MessageCenter_IsMessageOfType,
                            }
                            .call(c, 0x001e56e4)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0x470000;
                            if t {
                                break 'b_1e56f8;
                            } else {
                                c.cycles += 3;
                                c.g[A2][0] = c.g[S1][0];
                                let t = true;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1e80));
                                if t {
                                    break 'b_1e5714;
                                } else {
                                    break 'b_1e56f8;
                                }
                            }
                        }
                        c.cycles += 5;
                        c.g[A0][0] = 0x4;
                        c.g[RA][0] = sx(0x001e5704);
                        c.g[A1][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001e2d58,
                            f: ee::s04::MessageCenter_IsMessageOfType,
                        }
                        .call(c, 0x001e5704)?;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A1][0] = 0x470000;
                        if t {
                            break 'b_1e5724;
                        } else {
                            c.cycles += 2;
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1e60));
                            break 'b_1e5714;
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001e571c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00417828, 0x001e571c)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                    if t {
                        break 'b_1e574c;
                    } else {
                        break 'b_1e5724;
                    }
                }
                c.cycles += 5;
                c.g[A1][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001e5730);
                c.g[A0][0] = 0x8;
                Callee {
                    addr: 0x001e2d58,
                    f: ee::s04::MessageCenter_IsMessageOfType,
                }
                .call(c, 0x001e5730)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x470000;
                if t {
                    break 'b_1e5748;
                } else {
                    c.cycles += 4;
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1e40));
                    c.g[RA][0] = sx(0x001e5748);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00417828, 0x001e5748)?;
                    break 'b_1e5748;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
            break 'b_1e574c;
        }
        'b_1e576c: {
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_1e576c;
            } else {
                c.cycles += 6;
                c.g[RA][0] = sx(0x001e575c);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x001e575c)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x54));
                c.g[RA][0] = sx(0x001e5768);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001e5768)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'b_1e576c;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine_rt as machine;
    use crate::difftest::{check, elf_image, fresh, run_generated, Rng, DATA};

    use crate::difftest::fakes;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (in each machine's own RAM), the
    /// per-fake call counters (+0x20) and result mailboxes (+0x100).
    const OC959_CTRL: u32 = DATA + 0x2e_0000;
    const OC959_MAGIC: u32 = 0x0959_0959;

    /// A fake callee: logs the call like `fakes::fake`, then returns `mail - 1`
    /// when the fixture put `mail` (non-zero) in this fake's mailbox for this
    /// call number (16 calls, then wraps).
    fn oc959_fake_call(c: &mut Cpu, addr: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(OC959_CTRL + 0x20 + 4 * idx);
        c.bus.write32(OC959_CTRL + 0x20 + 4 * idx, n + 1);
        let mail = c.bus.read32(OC959_CTRL + 0x100 + 0x40 * idx + 4 * (n & 15));
        fakes::fake(c, addr, ra);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 1) as u64;
        }
    }

    macro_rules! oc959_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC959_CTRL) != OC959_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc959_fake_call(c, $addr, $idx);
                }
            )*
            fn oc959_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc959_fakes!(
        (
            oc959_fake_is_type,
            OC959_PREV_IS_TYPE,
            ee::s04::OVERRIDE_001E2D58,
            ee::s04::f_001e2d58__generated,
            0x001e2d58,
            0
        ),
        (
            oc959_fake_msg_id,
            OC959_PREV_MSG_ID,
            ee::s04::OVERRIDE_001E3620,
            ee::s04::f_001e3620__generated,
            0x001e3620,
            1
        ),
        (
            oc959_fake_msg_variant,
            OC959_PREV_MSG_VARIANT,
            ee::s04::OVERRIDE_001E3650,
            ee::s04::f_001e3650__generated,
            0x001e3650,
            2
        ),
        (
            oc959_fake_inbox,
            OC959_PREV_INBOX,
            ee::s04::OVERRIDE_001E39F8,
            ee::s04::f_001e39f8__generated,
            0x001e39f8,
            3
        ),
        (
            oc959_fake_is_flag,
            OC959_PREV_IS_FLAG,
            ee::s04::OVERRIDE_001E3A30,
            ee::s04::f_001e3a30__generated,
            0x001e3a30,
            4
        ),
        (
            oc959_fake_set_flag,
            OC959_PREV_SET_FLAG,
            ee::s04::OVERRIDE_001E3A78,
            ee::s04::f_001e3a78__generated,
            0x001e3a78,
            5
        ),
        (
            oc959_fake_find_msg,
            OC959_PREV_FIND_MSG,
            ee::s04::OVERRIDE_001E3388,
            ee::s04::f_001e3388__generated,
            0x001e3388,
            6
        ),
        (
            oc959_fake_open_popup,
            OC959_PREV_OPEN_POPUP,
            ee::s04::OVERRIDE_001E50C8,
            ee::s04::f_001e50c8__generated,
            0x001e50c8,
            7
        ),
        (
            oc959_fake_delete_msg,
            OC959_PREV_DELETE_MSG,
            ee::s04::OVERRIDE_001E5148,
            ee::s04::f_001e5148__generated,
            0x001e5148,
            8
        ),
        (
            oc959_fake_count_group,
            OC959_PREV_COUNT_GROUP,
            ee::s04::OVERRIDE_001E2BB8,
            ee::s04::f_001e2bb8__generated,
            0x001e2bb8,
            9
        ),
        (
            oc959_fake_group_variant,
            OC959_PREV_GROUP_VARIANT,
            ee::s04::OVERRIDE_001E5AA0,
            ee::s04::f_001e5aa0__generated,
            0x001e5aa0,
            10
        ),
        (
            oc959_fake_msg_count,
            OC959_PREV_MSG_COUNT,
            ee::s04::OVERRIDE_001E33E8,
            ee::s04::f_001e33e8__generated,
            0x001e33e8,
            11
        ),
        (
            oc959_fake_update_text,
            OC959_PREV_UPDATE_TEXT,
            ee::s04::OVERRIDE_001E4458,
            ee::s04::f_001e4458__generated,
            0x001e4458,
            12
        ),
        (
            oc959_fake_scroll_arrows,
            OC959_PREV_SCROLL_ARROWS,
            ee::s04::OVERRIDE_001E4658,
            ee::s04::f_001e4658__generated,
            0x001e4658,
            13
        ),
        (
            oc959_fake_refresh_list,
            OC959_PREV_REFRESH_LIST,
            ee::s04::OVERRIDE_001E48B0,
            ee::s04::f_001e48b0__generated,
            0x001e48b0,
            14
        ),
    );

    /// Mailbox of fake `idx` for call number `n`: the fake returns `value`.
    fn oc959_mail(c: &mut Cpu, idx: u32, n: u32, value: u32) {
        c.bus
            .write32(OC959_CTRL + 0x100 + 0x40 * idx + 4 * n, value + 1);
    }

    /// Fakes on, and a seed-dependent result for each faked callee call.
    fn oc959_gate(c: &mut Cpu, seed: u32) {
        oc959_install_fakes();
        c.bus.write32(OC959_CTRL, OC959_MAGIC);
        for idx in 0..15u32 {
            for n in 0..8u32 {
                let bit = (crate::difftest::fakes::mix((seed * 31 + idx * 7 + n) as u64) >> 5) & 1;
                oc959_mail(c, idx, n, bit as u32);
            }
        }
    }

    const OC959_SELF: u32 = DATA + 0x1000;
    const OC959_EVENT: u32 = DATA + 0x2000;
    const OC959_SUB: u32 = DATA + 0x3000;
    const OC959_VT: u32 = DATA + 0x4000;
    const OC959_WIDGET: u32 = DATA + 0x5000;
    const OC959_OUT: u32 = DATA + 0x6000;
    const FAKE: u32 = 0x0060_0000;

    /// The message-center screen object: sub-object at +0x144 (inbox view with
    /// the first-row offset byte at +0x95), eight widgets at +0xfc (each with a
    /// vtable whose slots 0x28/0x48 hold a call target), flags and sound ids.
    fn oc959_screen(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let s = OC959_SELF;
        c.bus.write32(s + 0x144, OC959_SUB);
        c.bus.write8(OC959_SUB + 0x95, (r.next_u32() % 4) as u8);
        c.bus.write32(s + 0x10, OC959_SUB + 0x100);
        c.bus.write32(s + 0x17c, OC959_SUB + 0x200);
        c.bus.write32(s + 0x170, seed % 2);
        c.bus.write32(s + 0x174, seed % 3);
        c.bus.write32(OC959_EVENT + 0x18, r.next_u32() % 40);
        c.bus.write32(OC959_VT + 0x28, FAKE);
        c.bus.write32(OC959_VT + 0x2c, FAKE + 0x10);
        c.bus.write32(OC959_VT + 0x48, FAKE + 0x20);
        c.bus.write32(OC959_VT + 0x4c, FAKE + 0x30);
        for i in 0..8u32 {
            let w = OC959_WIDGET + 0x100 * i;
            c.bus.write32(s + 0xfc + 4 * i, w);
            c.bus.write32(w + 8, OC959_VT);
        }
    }

    /// OnWidgetAction: widget ids 0..12 (buttons: toggle, delete, text update,
    /// open popup, the clear loop) with and without an event.
    #[test]
    fn ov_screen_message_center_on_widget_action_matches() {
        check(
            ee::s04::f_001e4200__generated,
            ov_screen_message_center_on_widget_action,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e4200, 0);
                elf_image(c);
                oc959_screen(c, &mut r, seed);
                oc959_gate(c, seed);
                c.pc = 0x001e_4200;
                c.g[A0][0] = sx(OC959_SELF);
                c.g[A1][0] = if seed % 11 == 0 { 0 } else { sx(OC959_EVENT) };
                c.g[A2][0] = (seed % 12) as u64;
            },
        );
    }

    /// OnWidgetNotify: ids 3..11 cover the jump table, the default and the
    /// out-of-range exit.
    #[test]
    fn ov_screen_message_center_on_widget_notify_matches() {
        check(
            ee::s04::f_001e4578__generated,
            ov_screen_message_center_on_widget_notify,
            96,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e4578, 0);
                elf_image(c);
                oc959_screen(c, &mut r, seed);
                oc959_gate(c, seed);
                c.pc = 0x001e_4578;
                c.g[A0][0] = sx(OC959_SELF);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (seed % 12) as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// Distinct cycle totals of the generated body (no unwinds).
    fn oc959_paths(g: fn(&mut Cpu), seeds: u32, min: usize, setup: &dyn Fn(&mut Cpu, u32)) {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, g);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= min, "only {} paths: {seen:?}", seen.len());
    }

    #[test]
    fn ov_screen_message_center_on_widget_notify_paths() {
        oc959_paths(ee::s04::f_001e4578__generated, 96, 4, &|c, seed| {
            let mut r = machine(c, seed, 0x1e4578, 0);
            elf_image(c);
            oc959_screen(c, &mut r, seed);
            oc959_gate(c, seed);
            c.pc = 0x001e_4578;
            c.g[A0][0] = sx(OC959_SELF);
            c.g[A2][0] = (seed % 12) as u64;
        });
    }

    /// Message ids around the 0x73..0x8a table (and outside it) with the four
    /// type checks (3, 5, 4, 8) answered by the real `IsMessageOfType`.
    fn oc959_subject_fixture(c: &mut Cpu, seed: u32, entry: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        elf_image(c);
        oc959_screen(c, &mut r, seed);
        oc959_gate(c, seed);
        c.pc = entry;
        r
    }

    #[test]
    fn ov_screen_message_center_set_row_subject_matches() {
        check(
            ee::s04::f_001e4f70__generated,
            ov_screen_message_center_set_row_subject,
            192,
            &|c, seed| {
                oc959_subject_fixture(c, seed, 0x001e_4f70, 0x1e4f70);
                c.g[A0][0] = sx(OC959_SELF);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (0x6c + seed / 8 % 32) as u64;
                c.g[A3][0] = (seed % 5) as u64;
            },
        );
    }

    #[test]
    fn c_ov_state_view_popup_set_variant_subject_matches() {
        check(
            ee::s04::f_001e5638__generated,
            c_ov_state_view_popup_set_variant_subject,
            192,
            &|c, seed| {
                oc959_subject_fixture(c, seed, 0x001e_5638, 0x1e5638);
                c.bus
                    .write32(OC959_SELF + 0x54, if seed % 3 == 0 { 0 } else { OC959_OUT });
                c.g[A0][0] = sx(OC959_SELF);
                c.g[A1][0] = (0x6c + seed / 2 % 32) as u64;
            },
        );
    }
}
