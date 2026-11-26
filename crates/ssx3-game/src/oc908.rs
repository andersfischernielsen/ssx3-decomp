//! Staging module oc908 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc908`.

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
        0x001a_b178,
        net_fe_state_show_challenge_error_popup,
        "NetFEState_ShowChallengeErrorPopup",
    ),
    (
        0x001a_b288,
        net_fe_state_show_session_lost_popup,
        "NetFEState_ShowSessionLostPopup",
    ),
    (
        0x001a_c5c0,
        c_fe_use_existing_account_on_widget_action,
        "cFEUseExistingAccount_OnWidgetAction",
    ),
    (
        0x001a_d5e8,
        c_fe_state_select_persona_on_widget_action,
        "cFEStateSelectPersona_OnWidgetAction",
    ),
];

/// `NetFEState_ShowChallengeErrorPopup` (0x001ab178): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn net_fe_state_show_challenge_error_popup(c: &mut Cpu) {
    if c.pc != 0x001ab178 {
        return ee::s03::f_001ab178__generated(c);
    }
    guest(c, |c| {
        'b_1ab25c: {
            c.cycles += 28;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x001ab19c);
            c.g[S0][0] = c.g[A1][0];
            Callee {
                addr: 0x001a9c50,
                f: ee::s03::NetFEState_GetPopup,
            }
            .call(c, 0x001ab19c)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = 0x460000;
            c.g[V0][0] = 0x1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
            c.sw((c.g[S2][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
            c.g[RA][0] = sx(0x001ab1b8);
            c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xbc));
            call_ptr(c, 0x00317670, 0x001ab1b8)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001ab1c8);
            c.g[A1][0] = 0u64;
            Callee {
                addr: 0x001c57c0,
                f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
            }
            .call(c, 0x001ab1c8)?;
            c.g[V0][0] = 0x835;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xed));
            c.g[V1][0] = 0x6;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x150), c.g[V0][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x14c), c.g[V1][0]);
            c.g[V0][0] = (c.g[S0][0] < 0x27u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x170), 0u64);
            if t {
                break 'b_1ab25c;
            } else {
                'b_1ab248: {
                    'b_1ab240: {
                        'b_1ab234: {
                            'b_1ab228: {
                                'b_1ab21c: {
                                    'b_1ab210: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2800));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001ab204 => {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x460000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x2768));
                                                if t {
                                                    break 'b_1ab248;
                                                } else {
                                                    break 'b_1ab210;
                                                }
                                            }
                                            0x001ab210 => {
                                                break 'b_1ab210;
                                            }
                                            0x001ab21c => {
                                                break 'b_1ab21c;
                                            }
                                            0x001ab228 => {
                                                break 'b_1ab228;
                                            }
                                            0x001ab234 => {
                                                break 'b_1ab234;
                                            }
                                            0x001ab240 => {
                                                break 'b_1ab240;
                                            }
                                            0x001ab25c => {
                                                break 'b_1ab25c;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001ab178, 0x001ab284);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2780));
                                    if t {
                                        break 'b_1ab248;
                                    } else {
                                        break 'b_1ab21c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A0][0] = 0x460000;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x27a0));
                                if t {
                                    break 'b_1ab248;
                                } else {
                                    break 'b_1ab228;
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = 0x460000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2158));
                            if t {
                                break 'b_1ab248;
                            } else {
                                break 'b_1ab234;
                            }
                        }
                        c.cycles += 3;
                        c.g[A0][0] = 0x460000;
                        let t = true;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x27b8));
                        if t {
                            break 'b_1ab248;
                        } else {
                            break 'b_1ab240;
                        }
                    }
                    c.cycles += 2;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x27d8));
                    break 'b_1ab248;
                }
                c.cycles += 5;
                c.g[RA][0] = sx(0x001ab250);
                call_ptr(c, 0x00317670, 0x001ab250)?;
                c.g[V1][0] = 0x3;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                break 'b_1ab25c;
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x001ab268);
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x001a9bc0,
            f: ee::s03::NetFEState_ShowPopup,
        }
        .call(c, 0x001ab268)?;
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

/// `NetFEState_ShowSessionLostPopup` (0x001ab288): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn net_fe_state_show_session_lost_popup(c: &mut Cpu) {
    if c.pc != 0x001ab288 {
        return ee::s03::f_001ab288__generated(c);
    }
    guest(c, |c| {
        'b_1ab348: {
            c.cycles += 28;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x001ab2ac);
            c.g[S0][0] = c.g[A1][0];
            Callee {
                addr: 0x001a9c50,
                f: ee::s03::NetFEState_GetPopup,
            }
            .call(c, 0x001ab2ac)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = 0x460000;
            c.g[V0][0] = 0x1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
            c.sw((c.g[S2][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
            c.g[RA][0] = sx(0x001ab2c8);
            c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xbc));
            call_ptr(c, 0x00317670, 0x001ab2c8)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001ab2d8);
            c.g[A1][0] = 0u64;
            Callee {
                addr: 0x001c57c0,
                f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
            }
            .call(c, 0x001ab2d8)?;
            c.g[V0][0] = 0x837;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xee));
            c.g[V1][0] = 0x6;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x150), c.g[V0][0]);
            c.sw((c.g[S2][0] as u32).wrapping_add(0x14c), c.g[V1][0]);
            c.g[V0][0] = (c.g[S0][0] < 0x26u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x170), 0u64);
            if t {
                break 'b_1ab348;
            } else {
                'b_1ab334: {
                    'b_1ab32c: {
                        'b_1ab320: {
                            c.cycles += 7;
                            c.g[V0][0] = 0x460000;
                            c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28a0));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                            let t = c.g[A0][0] as u32;
                            match t {
                                0x001ab314 => {
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2780));
                                    if t {
                                        break 'b_1ab334;
                                    } else {
                                        break 'b_1ab320;
                                    }
                                }
                                0x001ab320 => {
                                    break 'b_1ab320;
                                }
                                0x001ab32c => {
                                    break 'b_1ab32c;
                                }
                                0x001ab348 => {
                                    break 'b_1ab348;
                                }
                                _ => {
                                    c.jump_out(t, 0x001ab288, 0x001ab370);
                                    return Ok(());
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[A0][0] = 0x460000;
                        let t = true;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x27a0));
                        if t {
                            break 'b_1ab334;
                        } else {
                            break 'b_1ab32c;
                        }
                    }
                    c.cycles += 2;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x27d8));
                    break 'b_1ab334;
                }
                c.cycles += 5;
                c.g[RA][0] = sx(0x001ab33c);
                call_ptr(c, 0x00317670, 0x001ab33c)?;
                c.g[V1][0] = 0x3;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                break 'b_1ab348;
            }
        }
        c.cycles += 10;
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x001ab354);
        c.g[A1][0] = c.g[S2][0];
        Callee {
            addr: 0x001a9bc0,
            f: ee::s03::NetFEState_ShowPopup,
        }
        .call(c, 0x001ab354)?;
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

/// `cFEUseExistingAccount_OnWidgetAction` (0x001ac5c0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_use_existing_account_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001ac5c0 {
        return ee::s03::f_001ac5c0__generated(c);
    }
    guest(c, |c| 'l_1ac5c0: loop {
        'b_1ac904: {
            'b_1ac900: {
                'b_1ac8f8: {
                    'b_1ac8e8: {
                        'b_1ac88c: {
                            c.cycles += 9;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                            c.g[V0][0] = 0x6;
                            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.g[S3][0] = c.g[A0][0];
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                            let t = c.g[A2][0] == c.g[V0][0];
                            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                            if t {
                                break 'b_1ac88c;
                            } else {
                                'b_1ac614: {
                                    'b_1ac600: {
                                        c.cycles += 3;
                                        c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x5;
                                        if t {
                                            break 'b_1ac600;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[A2][0] == c.g[V0][0] {
                                                c.g[V1][0] =
                                                    c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                                                break 'b_1ac614;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_1ac8f8;
                                                } else {
                                                    break 'b_1ac600;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0xd;
                                    let t = c.g[A2][0] == c.g[V0][0];

                                    if t {
                                        break 'b_1ac8e8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;

                                        if t {
                                            break 'b_1ac8f8;
                                        } else {
                                            break 'b_1ac614;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x460000;
                                if t {
                                    break 'b_1ac900;
                                } else {
                                    'b_1ac874: {
                                        'b_1ac81c: {
                                            'b_1ac7a0: {
                                                'b_1ac71c: {
                                                    'b_1ac704: {
                                                        'b_1ac6b0: {
                                                            c.cycles += 6;
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x2d30));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                            let t = c.g[A0][0] as u32;
                                                            match t {
                                                                0x001ac5c0 => {
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x001ac5c0,
                                                                        );
                                                                    }
                                                                    continue 'l_1ac5c0;
                                                                }
                                                                0x001ac638 => {
                                                                    c.cycles += 4;
                                                                    c.g[RA][0] = sx(0x001ac640);
                                                                    c.g[A0][0] = c.g[S3][0];
                                                                    Callee { addr: 0x001ac408, f: ee::s03::cFEUseExistingAccount_ValidateFields }.call(c, 0x001ac640)?;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.lq(
                                                                        16,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x40),
                                                                    );
                                                                    if t {
                                                                        break 'b_1ac904;
                                                                    } else {
                                                                        c.cycles += 26;
                                                                        c.g[RA][0] = sx(0x001ac650);
                                                                        c.g[A0][0] = c.g[S3][0];
                                                                        Callee { addr: 0x001a9c50, f: ee::s03::NetFEState_GetPopup }.call(c, 0x001ac650)?;
                                                                        c.g[S0][0] = c.g[V0][0];
                                                                        c.g[A0][0] = 0x460000;
                                                                        c.g[V0][0] = 0x1;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x2c70,
                                                                                ));
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0xbc),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[RA][0] = sx(0x001ac66c);
                                                                        c.g[S1][0] =
                                                                            sx((c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0xbc,
                                                                                ));
                                                                        call_ptr(
                                                                            c, 0x00317670,
                                                                            0x001ac66c,
                                                                        )?;
                                                                        c.g[V1][0] = 0x3;
                                                                        c.g[A0][0] = 0x460000;
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0xc0),
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x2c88,
                                                                                ));
                                                                        c.g[RA][0] = sx(0x001ac684);
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0xc4),
                                                                            c.g[V0][0],
                                                                        );
                                                                        call_ptr(
                                                                            c, 0x00317670,
                                                                            0x001ac684,
                                                                        )?;
                                                                        c.g[A2][0] = c.g[V0][0];
                                                                        c.g[A0][0] = c.g[S1][0];
                                                                        c.g[RA][0] = sx(0x001ac694);
                                                                        c.g[A1][0] = 0u64;
                                                                        Callee { addr: 0x001c57c0, f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId }.call(c, 0x001ac694)?;
                                                                        c.g[V0][0] = 0x80e;
                                                                        c.g[A1][0] = c.g[S0][0];
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x150,
                                                                                ),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[RA][0] = sx(0x001ac6a8);
                                                                        c.g[A0][0] = c.g[S3][0];
                                                                        Callee { addr: 0x001a9bc0, f: ee::s03::NetFEState_ShowPopup }.call(c, 0x001ac6a8)?;
                                                                        let t = true;
                                                                        c.g[V0][0] = 0x4;
                                                                        if t {
                                                                            break 'b_1ac704;
                                                                        } else {
                                                                            break 'b_1ac6b0;
                                                                        }
                                                                    }
                                                                }
                                                                0x001ac6b0 => {
                                                                    break 'b_1ac6b0;
                                                                }
                                                                0x001ac71c => {
                                                                    break 'b_1ac71c;
                                                                }
                                                                0x001ac7a0 => {
                                                                    break 'b_1ac7a0;
                                                                }
                                                                0x001ac81c => {
                                                                    break 'b_1ac81c;
                                                                }
                                                                _ => {
                                                                    c.jump_out(
                                                                        t, 0x001ac5c0, 0x001ac91c,
                                                                    );
                                                                    return Ok(());
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x001ac6b8);
                                                        c.g[A0][0] = c.g[S3][0];
                                                        Callee { addr: 0x001ac360, f: ee::s03::cFEUseExistingAccount_CheckAccountName }.call(c, 0x001ac6b8)?;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x40),
                                                        );
                                                        if t {
                                                            break 'b_1ac904;
                                                        } else {
                                                            c.cycles += 17;
                                                            c.g[RA][0] = sx(0x001ac6c8);
                                                            c.g[A0][0] = c.g[S3][0];
                                                            Callee {
                                                                addr: 0x001a9c50,
                                                                f: ee::s03::NetFEState_GetPopup,
                                                            }
                                                            .call(c, 0x001ac6c8)?;
                                                            c.g[S0][0] = c.g[V0][0];
                                                            c.g[A0][0] = 0x460000;
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0xbc),
                                                                0u64,
                                                            );
                                                            c.g[RA][0] = sx(0x001ac6dc);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x2c98));
                                                            call_ptr(c, 0x00317670, 0x001ac6dc)?;
                                                            c.g[A2][0] = 0x3;
                                                            c.g[V1][0] = 0x812;
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x164),
                                                                0u64,
                                                            );
                                                            c.g[A1][0] = c.g[S0][0];
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0xc4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A0][0] = c.g[S3][0];
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0xc0),
                                                                c.g[A2][0],
                                                            );
                                                            c.g[RA][0] = sx(0x001ac700);
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x150),
                                                                c.g[V1][0],
                                                            );
                                                            Callee {
                                                                addr: 0x001a9bc0,
                                                                f: ee::s03::NetFEState_ShowPopup,
                                                            }
                                                            .call(c, 0x001ac700)?;
                                                            c.g[V0][0] = 0x2;
                                                            break 'b_1ac704;
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A0][0] = c
                                                        .lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                                    c.sw(
                                                        (c.g[S3][0] as u32).wrapping_add(0x6d8),
                                                        c.g[V0][0],
                                                    );
                                                    c.g[RA][0] = sx(0x001ac714);
                                                    c.g[A1][0] = 0u64;
                                                    call_ptr(c, 0x0025b6d8, 0x001ac714)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x40),
                                                    );
                                                    if t {
                                                        break 'b_1ac904;
                                                    } else {
                                                        break 'b_1ac71c;
                                                    }
                                                }
                                                c.cycles += 33;
                                                c.g[A0][0] = c.g[S3][0];
                                                c.g[RA][0] = sx(0x001ac728);
                                                c.g[S1][0] = 0x3;
                                                Callee {
                                                    addr: 0x001a9c50,
                                                    f: ee::s03::NetFEState_GetPopup,
                                                }
                                                .call(c, 0x001ac728)?;
                                                c.g[S0][0] = c.g[V0][0];
                                                c.g[V1][0] = 0x32;
                                                c.g[V0][0] = 0xffffffffffffffff;
                                                c.g[A0][0] = 0x460000;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x180),
                                                    c.g[V1][0],
                                                );
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x2cb8));
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xbc),
                                                    c.g[V0][0],
                                                );
                                                c.g[RA][0] = sx(0x001ac74c);
                                                c.g[S2][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                                call_ptr(c, 0x00317670, 0x001ac74c)?;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xc0),
                                                    c.g[S1][0],
                                                );
                                                c.g[A0][0] = 0x460000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x2cd0));
                                                c.g[RA][0] = sx(0x001ac760);
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xc4),
                                                    c.g[V0][0],
                                                );
                                                call_ptr(c, 0x00317670, 0x001ac760)?;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xd0),
                                                    c.g[S1][0],
                                                );
                                                c.g[A0][0] = 0x460000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                                                c.g[RA][0] = sx(0x001ac774);
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xd4),
                                                    c.g[V0][0],
                                                );
                                                call_ptr(c, 0x00317670, 0x001ac774)?;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[A2][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x001ac784);
                                                c.g[A1][0] = 0u64;
                                                Callee {
                                                    addr: 0x001c57c0,
                                                    f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                                }
                                                .call(c, 0x001ac784)?;
                                                c.g[V0][0] = 0x80f;
                                                c.g[A0][0] = c.g[S3][0];
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x150),
                                                    c.g[V0][0],
                                                );
                                                c.g[RA][0] = sx(0x001ac798);
                                                c.g[A1][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x001a9bc0,
                                                    f: ee::s03::NetFEState_ShowPopup,
                                                }
                                                .call(c, 0x001ac798)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                                if t {
                                                    break 'b_1ac904;
                                                } else {
                                                    break 'b_1ac7a0;
                                                }
                                            }
                                            c.cycles += 9;
                                            c.g[A0][0] = 0x460000;
                                            c.g[RA][0] = sx(0x001ac7ac);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x2b88));
                                            call_ptr(c, 0x00317670, 0x001ac7ac)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                            c.g[RA][0] = sx(0x001ac7b8);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x0039d860, 0x001ac7b8)?;
                                            c.g[S1][0] = c.g[V0][0];
                                            let t = c.g[S1][0] == 0u64;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                            if t {
                                                break 'b_1ac900;
                                            } else {
                                                c.cycles += 22;
                                                c.g[A0][0] = 0x460000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x2ce8));
                                                c.g[RA][0] = sx(0x001ac7d4);
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                                                call_ptr(c, 0x00317670, 0x001ac7d4)?;
                                                c.g[A2][0] = c.g[V0][0];
                                                c.g[A1][0] = c.g[S1][0];
                                                c.g[A3][0] = c.g[S0][0];
                                                c.g[A0][0] = c.g[S3][0];
                                                c.g[T0][0] = 0u64;
                                                c.g[T1][0] = 0x6;
                                                c.g[RA][0] = sx(0x001ac7f4);
                                                c.g[T2][0] = 0x10;
                                                Callee {
                                                    addr: 0x001a9150,
                                                    f: ee::s03::NetFEState_OpenKeyboardTextId,
                                                }
                                                .call(c, 0x001ac7f4)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x69c));
                                                c.g[V0][0] = 0x1;
                                                c.g[A1][0] = 0x4b;
                                                c.g[A2][0] = 0x1;
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x58),
                                                    c.g[V0][0],
                                                );
                                                c.g[A3][0] = 0x2;
                                                c.g[RA][0] = sx(0x001ac814);
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x69c));
                                                call_ptr(c, 0x001ce3c8, 0x001ac814)?;
                                                let t = true;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x69c));
                                                if t {
                                                    break 'b_1ac874;
                                                } else {
                                                    break 'b_1ac81c;
                                                }
                                            }
                                        }
                                        c.cycles += 9;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x001ac828);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ba0));
                                        call_ptr(c, 0x00317670, 0x001ac828)?;
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                        c.g[RA][0] = sx(0x001ac834);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x001ac834)?;
                                        c.g[S1][0] = c.g[V0][0];
                                        let t = c.g[S1][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                        if t {
                                            break 'b_1ac900;
                                        } else {
                                            c.cycles += 13;
                                            c.g[A0][0] = 0x460000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x2d00));
                                            c.g[RA][0] = sx(0x001ac850);
                                            c.g[S0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                            call_ptr(c, 0x00317670, 0x001ac850)?;
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[A2][0] = c.g[V0][0];
                                            c.g[A3][0] = c.g[S0][0];
                                            c.g[A0][0] = c.g[S3][0];
                                            c.g[T0][0] = 0x1;
                                            c.g[T1][0] = 0x7;
                                            c.g[RA][0] = sx(0x001ac870);
                                            c.g[T2][0] = 0x10;
                                            Callee {
                                                addr: 0x001a9150,
                                                f: ee::s03::NetFEState_OpenKeyboardTextId,
                                            }
                                            .call(c, 0x001ac870)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x69c));
                                            break 'b_1ac874;
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x35;
                                    c.g[A2][0] = 0u64;
                                    c.g[RA][0] = sx(0x001ac884);
                                    c.g[A3][0] = 0x2;
                                    call_ptr(c, 0x001ce3c8, 0x001ac884)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                    if t {
                                        break 'b_1ac904;
                                    } else {
                                        break 'b_1ac88c;
                                    }
                                }
                            }
                        }
                        c.cycles += 23;
                        c.g[A1][0] = 0x460000;
                        c.g[A0][0] = 0xb10;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d10));
                        c.g[A2][0] = 0u64;
                        c.g[RA][0] = sx(0x001ac8a4);
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x001ac8a4)?;
                        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[A3][0] = 0u64;
                        c.g[T0][0] = 0u64;
                        c.g[RA][0] = sx(0x001ac8c0);
                        c.g[T1][0] = 0u64;
                        Callee {
                            addr: 0x001b6ed0,
                            f: ee::s03::cFEStateNetAccountMem_ctor,
                        }
                        .call(c, 0x001ac8c0)?;
                        c.g[V1][0] = 0x832;
                        c.g[A1][0] = c.g[V0][0];
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x001ac8e0);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f290, 0x001ac8e0)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        if t {
                            break 'b_1ac904;
                        } else {
                            break 'b_1ac8e8;
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001ac8f0);
                    c.g[A0][0] = c.g[S3][0];
                    Callee {
                        addr: 0x001acdd0,
                        f: ee::s03::cFEUseExistingAccount_OnKeyboardDone,
                    }
                    .call(c, 0x001ac8f0)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_1ac904;
                    } else {
                        break 'b_1ac8f8;
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x001ac900);
                c.g[A0][0] = c.g[S3][0];
                Callee {
                    addr: 0x001a8918,
                    f: ee::s03::NetFEState_OnWidgetAction,
                }
                .call(c, 0x001ac900)?;
                break 'b_1ac900;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1ac904;
        }
        c.cycles += 6;
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

/// `cFEStateSelectPersona_OnWidgetAction` (0x001ad5e8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_select_persona_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001ad5e8 {
        return ee::s03::f_001ad5e8__generated(c);
    }
    guest(c, |c| 'l_1ad5e8: loop {
        'b_1ad8a8: {
            'b_1ad8a4: {
                'b_1ad898: {
                    c.cycles += 10;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S2][0] = c.g[A0][0];
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    if t {
                        break 'b_1ad898;
                    } else {
                        'b_1ad880: {
                            'b_1ad7b4: {
                                'b_1ad708: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x460000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2f30));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x001ad5e8 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001ad5e8);
                                            }
                                            continue 'l_1ad5e8;
                                        }
                                        0x001ad62c => {
                                            'b_1ad678: {
                                                c.cycles += 4;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                                c.g[V0][0] = 0xffffffffffffffff;
                                                let t = c.g[A1][0] != c.g[V0][0];
                                                c.g[A0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                                if t {
                                                    break 'b_1ad678;
                                                } else {
                                                    c.cycles += 15;
                                                    c.g[A1][0] = 0x460000;
                                                    c.g[A0][0] = 0x6f4;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x2ed0));
                                                    c.g[A2][0] = 0u64;
                                                    c.g[RA][0] = sx(0x001ad654);
                                                    c.g[A3][0] = 0u64;
                                                    call_ptr(c, 0x00317d70, 0x001ad654)?;
                                                    c.g[A1][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x001ad664);
                                                    c.g[A2][0] = 0u64;
                                                    Callee {
                                                        addr: 0x001b88f0,
                                                        f: ee::s03::cFEStateCreateStep1_ctor,
                                                    }
                                                    .call(c, 0x001ad664)?;
                                                    c.g[A0][0] = c.g[S2][0];
                                                    c.g[RA][0] = sx(0x001ad670);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    Callee {
                                                        addr: 0x001a8a40,
                                                        f: ee::s03::NetFEState_GotoNextState,
                                                    }
                                                    .call(c, 0x001ad670)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_1ad8a8;
                                                    } else {
                                                        break 'b_1ad678;
                                                    }
                                                }
                                            }
                                            'b_1ad6f4: {
                                                c.cycles += 4;
                                                c.g[V0][0] =
                                                    c.lw((c.g[A0][0] as u32).wrapping_add(0x6c));
                                                c.g[V0][0] = ((c.g[A1][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;

                                                if t {
                                                    break 'b_1ad6f4;
                                                } else {
                                                    c.cycles += 27;
                                                    c.g[A0][0] = 0x460000;
                                                    c.g[RA][0] = sx(0x001ad694);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x2e58));
                                                    call_ptr(c, 0x00317670, 0x001ad694)?;
                                                    c.g[A3][0] = 0x4a0000;
                                                    c.g[A2][0] = c.g[V0][0];
                                                    c.g[T2][0] = 0xc;
                                                    c.g[T0][0] = 0u64;
                                                    c.g[T1][0] = 0xffffffffffffffff;
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[A3][0] =
                                                        sx((c.g[A3][0] as u32)
                                                            .wrapping_add(0x1408));
                                                    c.g[RA][0] = sx(0x001ad6b8);
                                                    c.g[A0][0] = c.g[S2][0];
                                                    Callee {
                                                        addr: 0x001a9150,
                                                        f: ee::s03::NetFEState_OpenKeyboardTextId,
                                                    }
                                                    .call(c, 0x001ad6b8)?;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x69c)
                                                        );
                                                    c.g[V0][0] = 0x1;
                                                    c.g[A1][0] = 0x4b;
                                                    c.g[A2][0] = 0x1;
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x58),
                                                        c.g[V0][0],
                                                    );
                                                    c.g[A3][0] = 0x2;
                                                    c.g[RA][0] = sx(0x001ad6d8);
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x69c)
                                                        );
                                                    call_ptr(c, 0x001ce3c8, 0x001ad6d8)?;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x69c)
                                                        );
                                                    c.g[A1][0] = 0x35;
                                                    c.g[A2][0] = 0u64;
                                                    c.g[RA][0] = sx(0x001ad6ec);
                                                    c.g[A3][0] = 0x2;
                                                    call_ptr(c, 0x001ce3c8, 0x001ad6ec)?;
                                                    let t = true;
                                                    c.sw(
                                                        (c.g[S2][0] as u32).wrapping_add(0x6d0),
                                                        0u64,
                                                    );
                                                    if t {
                                                        break 'b_1ad8a4;
                                                    } else {
                                                        break 'b_1ad6f4;
                                                    }
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[RA][0] = sx(0x001ad6fc);
                                            call_ptr(c, 0x0025cd28, 0x001ad6fc)?;
                                            c.g[V0][0] = 0x1;
                                            let t = true;
                                            c.sw(
                                                (c.g[S2][0] as u32).wrapping_add(0x6d0),
                                                c.g[V0][0],
                                            );
                                            if t {
                                                break 'b_1ad8a4;
                                            } else {
                                                break 'b_1ad708;
                                            }
                                        }
                                        0x001ad708 => {
                                            break 'b_1ad708;
                                        }
                                        0x001ad7b4 => {
                                            break 'b_1ad7b4;
                                        }
                                        0x001ad880 => {
                                            break 'b_1ad880;
                                        }
                                        0x001ad898 => {
                                            break 'b_1ad898;
                                        }
                                        0x001ad8a4 => {
                                            break 'b_1ad8a4;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x001ad5e8, 0x001ad8bc);
                                            return Ok(());
                                        }
                                    }
                                }
                                'b_1ad71c: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    let t = c.g[V0][0] != 0u64;

                                    if t {
                                        break 'b_1ad71c;
                                    } else {
                                        c.cycles += 2;
                                        c.g[RA][0] = sx(0x001ad71c);
                                        call_ptr(c, 0x00260f80, 0x001ad71c)?;
                                        break 'b_1ad71c;
                                    }
                                }
                                'b_1ad738: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001ad724);
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    call_ptr(c, 0x00261460, 0x001ad724)?;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    if t {
                                        break 'b_1ad738;
                                    } else {
                                        c.cycles += 3;
                                        c.g[RA][0] = sx(0x001ad734);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        call_ptr(c, 0x002613c8, 0x001ad734)?;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        break 'b_1ad738;
                                    }
                                }
                                'b_1ad748: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != 0u64;

                                    if t {
                                        break 'b_1ad748;
                                    } else {
                                        c.cycles += 2;
                                        c.g[RA][0] = sx(0x001ad748);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                        call_ptr(c, 0x0025b800, 0x001ad748)?;
                                        break 'b_1ad748;
                                    }
                                }
                                c.cycles += 27;
                                c.g[RA][0] = sx(0x001ad750);
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                call_ptr(c, 0x00261408, 0x001ad750)?;
                                c.g[RA][0] = sx(0x001ad758);
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                call_ptr(c, 0x0025b800, 0x001ad758)?;
                                c.g[A1][0] = 0x460000;
                                c.g[A0][0] = 0xb10;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d10));
                                c.g[A2][0] = 0u64;
                                c.g[RA][0] = sx(0x001ad770);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x001ad770)?;
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A2][0] = c.g[S2][0];
                                c.g[A3][0] = 0u64;
                                c.g[T0][0] = 0u64;
                                c.g[RA][0] = sx(0x001ad78c);
                                c.g[T1][0] = 0u64;
                                Callee {
                                    addr: 0x001b6ed0,
                                    f: ee::s03::cFEStateNetAccountMem_ctor,
                                }
                                .call(c, 0x001ad78c)?;
                                c.g[V1][0] = 0x832;
                                c.g[A1][0] = c.g[V0][0];
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x001ad7ac);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f290, 0x001ad7ac)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1ad8a8;
                                } else {
                                    break 'b_1ad7b4;
                                }
                            }
                            c.cycles += 3;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                            let t = (c.g[A1][0] as i64) < 0;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                            if t {
                                break 'b_1ad8a4;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x6c));
                                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1ad8a8;
                                } else {
                                    c.cycles += 44;
                                    c.sw((c.g[S2][0] as u32).wrapping_add(0x6d4), c.g[A1][0]);
                                    c.g[RA][0] = sx(0x001ad7dc);
                                    c.g[A0][0] = c.g[S2][0];
                                    Callee {
                                        addr: 0x001a9c50,
                                        f: ee::s03::NetFEState_GetPopup,
                                    }
                                    .call(c, 0x001ad7dc)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A0][0] = 0x460000;
                                    c.g[V0][0] = 0x2;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ee8));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x001ad7f8);
                                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                    call_ptr(c, 0x00317670, 0x001ad7f8)?;
                                    c.g[V1][0] = 0x3;
                                    c.g[A0][0] = 0x460000;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2f08));
                                    c.g[RA][0] = sx(0x001ad810);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                    call_ptr(c, 0x00317670, 0x001ad810)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A1][0] = 0u64;
                                    c.g[RA][0] = sx(0x001ad820);
                                    c.g[A0][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x001c57c0,
                                        f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                    }
                                    .call(c, 0x001ad820)?;
                                    c.g[A0][0] = 0x460000;
                                    c.g[RA][0] = sx(0x001ad82c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c88));
                                    call_ptr(c, 0x00317670, 0x001ad82c)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A1][0] = 0x1;
                                    c.g[RA][0] = sx(0x001ad83c);
                                    c.g[A0][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x001c57c0,
                                        f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                    }
                                    .call(c, 0x001ad83c)?;
                                    c.g[A0][0] = 0x460000;
                                    c.g[RA][0] = sx(0x001ad848);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2f18));
                                    call_ptr(c, 0x00317670, 0x001ad848)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A1][0] = 0u64;
                                    c.g[RA][0] = sx(0x001ad85c);
                                    c.g[A3][0] = 0x1;
                                    Callee {
                                        addr: 0x001c5878,
                                        f: ee::s03::cFEPopupConfirm_CfgSetOptionConfirmTextId,
                                    }
                                    .call(c, 0x001ad85c)?;
                                    c.g[V0][0] = 0x1;
                                    c.g[V1][0] = 0x816;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x168), c.g[V0][0]);
                                    c.g[A0][0] = c.g[S2][0];
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V1][0]);
                                    c.g[RA][0] = sx(0x001ad878);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001a9bc0,
                                        f: ee::s03::NetFEState_ShowPopup,
                                    }
                                    .call(c, 0x001ad878)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_1ad8a8;
                                    } else {
                                        break 'b_1ad880;
                                    }
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x69c));
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001ad890);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
                        Callee {
                            addr: 0x001ad208,
                            f: ee::s03::cFEStateSelectPersona_SubmitPersona,
                        }
                        .call(c, 0x001ad890)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_1ad8a8;
                        } else {
                            break 'b_1ad898;
                        }
                    }
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001ad8a4);
                c.g[A1][0] = c.g[S0][0];
                Callee {
                    addr: 0x001a8918,
                    f: ee::s03::NetFEState_OnWidgetAction,
                }
                .call(c, 0x001ad8a4)?;
                break 'b_1ad8a4;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1ad8a8;
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
    use crate::difftest::{check, fresh, run_generated, DATA, GAME_GP};
    use std::collections::BTreeSet;
    use std::ptr::addr_of_mut;

    /// Gate word of this module's callee fakes (clear of the fakes' call log
    /// at DATA+0x100000).
    pub(crate) const OC908_CTRL: u32 = DATA + 0x4e_0000;
    pub(crate) const OC908_MAGIC: u32 = 0x0908_5eed;
    /// Result mailboxes: `{mode, value}` per callee, indexed by its address.
    pub(crate) const OC908_MAIL: u32 = DATA + 0x5a_0000;

    /// Make the gated fake of `addr` return `v` (sign-extended) in `v0`.
    pub(crate) fn oc908_force(c: &mut Cpu, addr: u32, v: u32) {
        let mail = OC908_MAIL + (addr & 0xffff) * 8;
        c.bus.write32(mail, 1);
        c.bus.write32(mail + 4, v);
    }

    /// The gated fake callee: like `fakes::fake_call`, then the forced result
    /// of its mailbox unless it unwound.
    pub(crate) fn oc908_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = OC908_MAIL + (addr & 0xffff) * 8;
        let mode = c.bus.read32(mail);
        let val = c.bus.read32(mail + 4);
        fakes::fake_call(c, addr);
        if c.pc == ra && mode == 1 {
            c.g[V0][0] = sx(val);
        }
    }

    macro_rules! oc908_fakes {
        ($(($fake:ident, $slot:expr, $gen:path, $addr:expr)),* $(,)?) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC908_CTRL) != OC908_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                    }
                    oc908_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc908_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc908_fakes!(
        (
            oc908_fake_get_popup,
            ee::s03::OVERRIDE_001A9C50,
            ee::s03::f_001a9c50__generated,
            0x001a_9c50
        ),
        (
            oc908_fake_show_popup,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a_9bc0
        ),
        (
            oc908_fake_cfg_option_text,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c_57c0
        ),
        (
            oc908_fake_cfg_confirm_text,
            ee::s03::OVERRIDE_001C5878,
            ee::s03::f_001c5878__generated,
            0x001c_5878
        ),
        (
            oc908_fake_open_keyboard,
            ee::s03::OVERRIDE_001A9150,
            ee::s03::f_001a9150__generated,
            0x001a_9150
        ),
        (
            oc908_fake_check_account_name,
            ee::s03::OVERRIDE_001AC360,
            ee::s03::f_001ac360__generated,
            0x001a_c360
        ),
        (
            oc908_fake_validate_fields,
            ee::s03::OVERRIDE_001AC408,
            ee::s03::f_001ac408__generated,
            0x001a_c408
        ),
        (
            oc908_fake_on_keyboard_done,
            ee::s03::OVERRIDE_001ACDD0,
            ee::s03::f_001acdd0__generated,
            0x001a_cdd0
        ),
        (
            oc908_fake_account_mem_ctor,
            ee::s03::OVERRIDE_001B6ED0,
            ee::s03::f_001b6ed0__generated,
            0x001b_6ed0
        ),
        (
            oc908_fake_create_step1_ctor,
            ee::s03::OVERRIDE_001B88F0,
            ee::s03::f_001b88f0__generated,
            0x001b_88f0
        ),
        (
            oc908_fake_net_on_widget_action,
            ee::s03::OVERRIDE_001A8918,
            ee::s03::f_001a8918__generated,
            0x001a_8918
        ),
        (
            oc908_fake_goto_next_state,
            ee::s03::OVERRIDE_001A8A40,
            ee::s03::f_001a8a40__generated,
            0x001a_8a40
        ),
        (
            oc908_fake_submit_persona,
            ee::s03::OVERRIDE_001AD208,
            ee::s03::f_001ad208__generated,
            0x001a_d208
        ),
        (
            oc908_fake_str_hash,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc908_fake_alloc_named,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc908_fake_key_enabled,
            ee::s04::OVERRIDE_001CE3C8,
            ee::s04::f_001ce3c8__generated,
            0x001c_e3c8
        ),
        (
            oc908_fake_lobby_connect,
            ee::s06::OVERRIDE_0025B6D8,
            ee::s06::f_0025b6d8__generated,
            0x0025_b6d8
        ),
        (
            oc908_fake_lobby_disconnect,
            ee::s06::OVERRIDE_0025B800,
            ee::s06::f_0025b800__generated,
            0x0025_b800
        ),
        (
            oc908_fake_lobby_select_persona,
            ee::s06::OVERRIDE_0025CD28,
            ee::s06::f_0025cd28__generated,
            0x0025_cd28
        ),
        (
            oc908_fake_buddy_create,
            ee::s06::OVERRIDE_00260F80,
            ee::s06::f_00260f80__generated,
            0x0026_0f80
        ),
        (
            oc908_fake_buddy_reconnect,
            ee::s06::OVERRIDE_002613C8,
            ee::s06::f_002613c8__generated,
            0x0026_13c8
        ),
        (
            oc908_fake_buddy_disconnect,
            ee::s06::OVERRIDE_00261408,
            ee::s06::f_00261408__generated,
            0x0026_1408
        ),
        (
            oc908_fake_buddy_is_connected,
            ee::s06::OVERRIDE_00261460,
            ee::s06::f_00261460__generated,
            0x0026_1460
        ),
        (
            oc908_fake_find_object,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc908_fake_list_first,
            ee::s13::OVERRIDE_00397840,
            ee::s13::f_00397840__generated,
            0x0039_7840
        ),
        (
            oc908_fake_list_push_front,
            ee::s13::OVERRIDE_003978F0,
            ee::s13::f_003978f0__generated,
            0x0039_78f0
        ),
    );

    /// Run `body` for every seed on the generated function and require that
    /// at least `min` different cycle totals (paths) came out.
    fn oc908_paths(generated: fn(&mut Cpu), seeds: u32, min: usize, setup: &dyn Fn(&mut Cpu, u32)) {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        assert!(
            seen.len() >= min,
            "only {} paths taken: {seen:?}",
            seen.len()
        );
    }

    /// Write a jump table of `n` words at `base`, entry `i` = `targets[map(i)]`.
    fn oc908_table(c: &mut Cpu, base: u32, n: u32, targets: &[u32]) {
        for i in 0..n {
            let t = targets[((i * 3 + 1) as usize) % targets.len()];
            c.bus.write32(base + 4 * i, t);
        }
    }

    const OC908_SELF: u32 = DATA + 0x100;
    const OC908_POPUP: u32 = DATA + 0x2000;

    /// Fixture of the two popup functions: popup object forced from
    /// `GetPopup`, the error code `code`, the jump table at `table`.
    fn oc908_popup_setup(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        base: u32,
        bias: u32,
        count: u32,
        targets: &[u32],
    ) {
        oc908_install_fakes();
        let _r = machine(c, seed, salt, 5);
        c.bus.write32(OC908_CTRL, OC908_MAGIC);
        oc908_force(c, 0x001a_9c50, OC908_POPUP);
        oc908_table(c, base, count, targets);
        // Codes below, inside and above the table's range.
        let k = (seed % (count + 6)) as i32 - 3;
        c.g[A0][0] = sx(OC908_SELF);
        c.g[A1][0] = sx((bias as i32 + k) as u32);
        c.pc = 0;
    }

    /// `NetFEState_ShowChallengeErrorPopup`: fills the popup with the
    /// challenge error's text (picked through a jump table by error code
    /// 0xed..0x113) and shows it.
    #[test]
    fn net_fe_state_show_challenge_error_popup_matches() {
        let targets = [
            0x001a_b204u32,
            0x001a_b210,
            0x001a_b21c,
            0x001a_b228,
            0x001a_b234,
            0x001a_b240,
            0x001a_b25c,
        ];
        let setup = |c: &mut Cpu, seed: u32| {
            oc908_popup_setup(c, seed, 0x001a_b178, 0x0046_2800, 0xed, 0x27, &targets);
            c.pc = 0x001a_b178;
        };
        check(
            ee::s03::f_001ab178__generated,
            net_fe_state_show_challenge_error_popup,
            120,
            &setup,
        );
        oc908_paths(ee::s03::f_001ab178__generated, 120, 3, &setup);
    }

    /// `NetFEState_ShowSessionLostPopup`: same shape, error codes
    /// 0xee..0x113 and four table targets.
    #[test]
    fn net_fe_state_show_session_lost_popup_matches() {
        let targets = [0x001a_b314u32, 0x001a_b320, 0x001a_b32c, 0x001a_b348];
        let setup = |c: &mut Cpu, seed: u32| {
            oc908_popup_setup(c, seed, 0x001a_b288, 0x0046_28a0, 0xee, 0x26, &targets);
            c.pc = 0x001a_b288;
        };
        check(
            ee::s03::f_001ab288__generated,
            net_fe_state_show_session_lost_popup,
            120,
            &setup,
        );
        oc908_paths(ee::s03::f_001ab288__generated, 120, 3, &setup);
    }

    /// Objects shared by the two widget-action fixtures: the state (`self`)
    /// with its sub-objects, the lobby manager at `gp-0xc8`, the popup and
    /// the account-memory object the constructor returns.
    fn oc908_widget_objects(c: &mut Cpu, seed: u32) {
        let this = OC908_SELF;
        let (obj40, objd0, objd0_10, kb) =
            (DATA + 0x1000, DATA + 0x1100, DATA + 0x1180, DATA + 0x1200);
        c.bus.write32(this + 0x10, DATA + 0x1500);
        c.bus.write32(this + 0x40, obj40);
        c.bus.write32(obj40 + 0xd0, objd0);
        c.bus.write32(objd0 + 0x10, objd0_10);
        c.bus.write32(this + 0x69c, kb);
        let lobby = DATA + 0x1400;
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), lobby);
        c.bus.write32(lobby + 0x48, 0x1111_0000 + seed);
        c.bus.write32(lobby + 0x4c, 0x2222_0000 + seed);
        c.bus.write32(lobby + 0x6c, 4);
        c.bus.write32(
            GAME_GP + 0x238,
            if seed & 4 != 0 { DATA + 0x1600 } else { 0 },
        );
        oc908_force(c, 0x001a_9c50, OC908_POPUP);
        // The account-memory object: with or without a first list entry.
        let obj = DATA + 0x2400;
        c.bus
            .write32(obj + 0x20, if seed & 1 != 0 { DATA + 0x2800 } else { 0 });
        oc908_force(c, 0x001b_6ed0, obj);
        oc908_force(c, 0x001b_88f0, obj);
        oc908_force(
            c,
            0x0039_7840,
            if seed & 2 != 0 { 0 } else { DATA + 0x2800 },
        );
    }

    /// `cFEUseExistingAccount_OnWidgetAction`: events 5 (a button: ok,
    /// back, name/password field, via a five-entry jump table), 6 (open the
    /// account list), 0xd (keyboard done) and the rest (forwarded to the base
    /// state), with the validators passing or failing.
    #[test]
    fn c_fe_use_existing_account_on_widget_action_matches() {
        let targets = [
            0x001a_c638u32,
            0x001a_c6b0,
            0x001a_c71c,
            0x001a_c7a0,
            0x001a_c81c,
        ];
        const EVENTS: [u64; 7] = [5, 6, 0xd, 0, 1, 7, 0x20];
        let setup = |c: &mut Cpu, seed: u32| {
            oc908_install_fakes();
            let _r = machine(c, seed, 0x001a_c5c0, 6);
            c.bus.write32(OC908_CTRL, OC908_MAGIC);
            oc908_widget_objects(c, seed);
            for (i, t) in targets.iter().enumerate() {
                c.bus.write32(0x0046_2d30 + 4 * i as u32, *t);
            }
            let child = DATA + 0x1300;
            // The button index: inside the table or beyond it.
            c.bus.write32(child + 0x18, (seed / 7) % 7);
            oc908_force(c, 0x001a_c408, (seed / 14) & 1);
            oc908_force(c, 0x001a_c360, (seed / 28) & 1);
            oc908_force(
                c,
                0x0039_d860,
                if (seed / 56) & 1 != 0 {
                    0
                } else {
                    DATA + 0x1700
                },
            );
            c.g[A0][0] = sx(OC908_SELF);
            c.g[A1][0] = sx(child);
            c.g[A2][0] = EVENTS[(seed % 7) as usize];
            c.pc = 0x001a_c5c0;
        };
        check(
            ee::s03::f_001ac5c0__generated,
            c_fe_use_existing_account_on_widget_action,
            450,
            &setup,
        );
        oc908_paths(ee::s03::f_001ac5c0__generated, 450, 6, &setup);
    }

    /// `cFEStateSelectPersona_OnWidgetAction`: events 5..14 through a
    /// ten-entry jump table (create persona, pick one of the list, rename
    /// with the keyboard, log out of the lobby/buddy managers, submit), the
    /// other events forwarded to the base state.
    #[test]
    fn c_fe_state_select_persona_on_widget_action_matches() {
        let targets = [
            0x001a_d62cu32,
            0x001a_d708,
            0x001a_d7b4,
            0x001a_d880,
            0x001a_d898,
            0x001a_d8a4,
        ];
        const EVENTS: [u64; 14] = [5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 4, 15, 0, 0x20];
        const CHILD: [u32; 5] = [0xffff_ffff, 0, 3, 4, 9];
        let setup = |c: &mut Cpu, seed: u32| {
            oc908_install_fakes();
            let _r = machine(c, seed, 0x001a_d5e8, 8);
            c.bus.write32(OC908_CTRL, OC908_MAGIC);
            oc908_widget_objects(c, seed);
            oc908_table(c, 0x0046_2f30, 10, &targets);
            let child = DATA + 0x1300;
            c.bus
                .write32(child + 0x18, CHILD[((seed / 14) % 5) as usize]);
            oc908_force(c, 0x0026_1460, (seed / 70) & 1);
            c.g[A0][0] = sx(OC908_SELF);
            c.g[A1][0] = sx(child);
            c.g[A2][0] = EVENTS[(seed % 14) as usize];
            c.pc = 0x001a_d5e8;
        };
        check(
            ee::s03::f_001ad5e8__generated,
            c_fe_state_select_persona_on_widget_action,
            560,
            &setup,
        );
        oc908_paths(ee::s03::f_001ad5e8__generated, 560, 6, &setup);
    }
}
