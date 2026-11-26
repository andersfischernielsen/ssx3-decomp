//! Staging module oc913 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc913`.

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
        0x001b83c0,
        c_fe_state_net_account_mem_format_failed_text,
        "cFEStateNetAccountMem_FormatFailedText",
    ),
    (
        0x001b84d8,
        c_fe_state_net_account_mem_on_child_event,
        "cFEStateNetAccountMem_OnChildEvent",
    ),
    (
        0x001b8c38,
        c_fe_state_create_step1_on_widget_action,
        "cFEStateCreateStep1_OnWidgetAction",
    ),
    (
        0x001b92c8,
        c_fe_state_create_step1_on_edit_done_popup_event,
        "cFEStateCreateStep1_OnEditDonePopupEvent",
    ),
];

/// `cFEStateNetAccountMem_FormatFailedText` (0x001b83c0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_net_account_mem_format_failed_text(c: &mut Cpu) {
    if c.pc != 0x001b83c0 {
        return ee::s03::f_001b83c0__generated(c);
    }
    guest(c, |c| {
        'b_1b84c4: {
            'b_1b84c0: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.g[A1][0] = 0x4a0000;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A0][0] = c.g[SP][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[RA][0] = sx(0x001b83ec);
                c.g[A2][0] = 0x1;
                call_ptr(c, 0x002c2718, 0x001b83ec)?;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6dc));
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                c.g[V1][0] = (c.g[A0][0] < 0x1au64) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x460000;
                if t {
                    break 'b_1b84c0;
                } else {
                    'b_1b8480: {
                        'b_1b8444: {
                            'b_1b8438: {
                                'b_1b8428: {
                                    c.cycles += 6;
                                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4660));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x001b8418 => {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] = 0x460000;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x4608));
                                            if t {
                                                break 'b_1b8444;
                                            } else {
                                                break 'b_1b8428;
                                            }
                                        }
                                        0x001b8428 => {
                                            break 'b_1b8428;
                                        }
                                        0x001b8438 => {
                                            break 'b_1b8438;
                                        }
                                        0x001b8480 => {
                                            break 'b_1b8480;
                                        }
                                        0x001b84c0 => {
                                            break 'b_1b84c0;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x001b83c0, 0x001b84d8);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = 0x460000;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4628));
                                if t {
                                    break 'b_1b8444;
                                } else {
                                    break 'b_1b8438;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A0][0] = 0x460000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4640));
                            break 'b_1b8444;
                        }
                        c.cycles += 15;
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x001b8458);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        call_ptr(c, 0x00317670, 0x001b8458)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001b8468);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, t, 0x001b8468)?;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x702));
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001b8478);
                        c.g[A2][0] = c.g[SP][0];
                        call_ptr(c, 0x002c26d0, 0x001b8478)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        if t {
                            break 'b_1b84c4;
                        } else {
                            break 'b_1b8480;
                        }
                    }
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x44f0));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001b84a0);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    call_ptr(c, 0x00317670, 0x001b84a0)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001b84b0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x001b84b0)?;
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x702));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001b84c0);
                    c.g[A2][0] = c.g[SP][0];
                    call_ptr(c, 0x002c26d0, 0x001b84c0)?;
                    break 'b_1b84c0;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_1b84c4;
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

/// `cFEStateNetAccountMem_OnChildEvent` (0x001b84d8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_net_account_mem_on_child_event(c: &mut Cpu) {
    if c.pc != 0x001b84d8 {
        return ee::s03::f_001b84d8__generated(c);
    }
    guest(c, |c| 'l_1b84d8: loop {
        'b_1b884c: {
            'b_1b8848: {
                'b_1b883c: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S0][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6dc));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3));
                    c.g[V0][0] = (c.g[V1][0] < 0x1bu64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = c.g[A1][0];
                    if t {
                        break 'b_1b883c;
                    } else {
                        'b_1b882c: {
                            'b_1b8810: {
                                'b_1b87ec: {
                                    'b_1b875c: {
                                        'b_1b8734: {
                                            'b_1b871c: {
                                                'b_1b8710: {
                                                    'b_1b86c4: {
                                                        'b_1b8620: {
                                                            c.cycles += 7;
                                                            c.g[V0][0] = 0x460000;
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x46d0));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                            let t = c.g[A0][0] as u32;
                                                            match t {
                                                                0x001b84d8 => {
                                                                    if event_due(c) {
                                                                        deliver_event_here(
                                                                            c, 0x001b84d8,
                                                                        );
                                                                    }
                                                                    continue 'l_1b84d8;
                                                                }
                                                                0x001b8520 => {
                                                                    'b_1b8604: {
                                                                        'b_1b85d0: {
                                                                            c.cycles += 3;
                                                                            c.g[V0][0] = 0x10;
                                                                            let t = c.g[A2][0]
                                                                                == c.g[V0][0];
                                                                            c.g[V0][0] = (c.g[A2]
                                                                                [0]
                                                                                < 0x11u64)
                                                                                as u64;
                                                                            if t {
                                                                                break 'b_1b85d0;
                                                                            } else {
                                                                                'b_1b8560: {
                                                                                    'b_1b8544: {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t = c.g
                                                                                            [V0][0]
                                                                                            == 0u64;
                                                                                        c.g[V0]
                                                                                            [0] =
                                                                                            0xf;
                                                                                        if t {
                                                                                            break 'b_1b8544;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            let t = c.g[A2][0] == c.g[V0][0];
                                                                                            c.g[A0][0] = c.g[S0][0];
                                                                                            if t {
                                                                                                break 'b_1b8560;
                                                                                            } else {
                                                                                                c.cycles += 2;
                                                                                                let t = true;

                                                                                                if t
                                                                                                {
                                                                                                    break 'b_1b882c;
                                                                                                } else {
                                                                                                    break 'b_1b8544;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 3;
                                                                                    c.g[V0][0] =
                                                                                        0x14;
                                                                                    let t = c.g[A2]
                                                                                        [0]
                                                                                        == c.g[V0]
                                                                                            [0];
                                                                                    c.g[V0][0] =
                                                                                        0x16;
                                                                                    if t {
                                                                                        break 'b_1b85d0;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            2;
                                                                                        let t = c.g[A2][0] == c.g[V0][0];
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S0]
                                                                                            [0];
                                                                                        if t {
                                                                                            break 'b_1b8604;
                                                                                        } else {
                                                                                            c.cycles += 2;
                                                                                            let t =
                                                                                                true;

                                                                                            if t {
                                                                                                break 'b_1b882c;
                                                                                            } else {
                                                                                                break 'b_1b8560;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                'b_1b85b4: {
                                                                                    'b_1b8594: {
                                                                                        'b_1b8584: {
                                                                                            c.cycles += 7;
                                                                                            c.g[A1][0] = c.g[S1][0];
                                                                                            c.g[RA][0] = sx(0x001b856c);
                                                                                            c.g[A2][0] = 0xf;
                                                                                            Callee { addr: 0x001a97b8, f: ee::s03::NetFEState_OnChildEvent }.call(c, 0x001b856c)?;
                                                                                            c.g[RA][0] = sx(0x001b8574);
                                                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                                                                                            call_ptr(c, 0x00265d68, 0x001b8574)?;
                                                                                            let t = c.g[V0][0] == 0u64;
                                                                                            c.g[V0][0] = 0x2;
                                                                                            if t {
                                                                                                break 'b_1b8584;
                                                                                            } else {
                                                                                                c.cycles += 2;
                                                                                                let t = true;
                                                                                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6dc));
                                                                                                if t
                                                                                                {
                                                                                                    break 'b_1b8594;
                                                                                                } else {
                                                                                                    break 'b_1b8584;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        c.cycles +=
                                                                                            3;
                                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d8));
                                                                                        let t = c.g
                                                                                            [V0][0]
                                                                                            == 0u64;
                                                                                        c.g[V0]
                                                                                            [0] =
                                                                                            0x6;
                                                                                        if t {
                                                                                            break 'b_1b85b4;
                                                                                        } else {
                                                                                            c.cycles += 1;
                                                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6dc));
                                                                                            break 'b_1b8594;
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 8;
                                                                                    c.g[A0][0] =
                                                                                        c.g[S0][0];
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001b85a0,
                                                                                    );
                                                                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x6d4), c.g[V0][0]);
                                                                                    Callee { addr: 0x001b8890, f: ee::s03::cFEStateNetAccountMem_PopupIdToState }.call(c, 0x001b85a0)?;
                                                                                    c.g[A0][0] =
                                                                                        c.g[S0][0];
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001b85ac,
                                                                                    );
                                                                                    c.g[A1][0] =
                                                                                        c.g[V0][0];
                                                                                    Callee { addr: 0x001b70d0, f: ee::s03::cFEStateNetAccountMem_SetState }.call(c, 0x001b85ac)?;
                                                                                    let t = true;
                                                                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                    if t {
                                                                                        break 'b_1b884c;
                                                                                    } else {
                                                                                        break 'b_1b85b4;
                                                                                    }
                                                                                }
                                                                                c.cycles += 3;
                                                                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                                                                if c.g[A1][0]
                                                                                    == 0u64
                                                                                {
                                                                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                    break 'b_1b884c;
                                                                                } else {
                                                                                    c.cycles += 4;
                                                                                    c.g[RA][0] = sx(
                                                                                        0x001b85c8,
                                                                                    );
                                                                                    c.g[A0][0] =
                                                                                        c.g[S0][0];
                                                                                    Callee { addr: 0x001b70d0, f: ee::s03::cFEStateNetAccountMem_SetState }.call(c, 0x001b85c8)?;
                                                                                    let t = true;
                                                                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                                                    if t {
                                                                                        break 'b_1b884c;
                                                                                    } else {
                                                                                        break 'b_1b85d0;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 7;
                                                                        c.g[A1][0] = c.g[S1][0];
                                                                        c.g[RA][0] = sx(0x001b85dc);
                                                                        c.g[A0][0] = c.g[S0][0];
                                                                        Callee { addr: 0x001a97b8, f: ee::s03::NetFEState_OnChildEvent }.call(c, 0x001b85dc)?;
                                                                        c.g[V1][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x6dc));
                                                                        c.g[V0][0] = 0x19;
                                                                        let t = c.g[V1][0]
                                                                            == c.g[V0][0];
                                                                        c.g[V0][0] = 0xd;
                                                                        if t {
                                                                            break 'b_1b8710;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = c.g[V1][0]
                                                                                == c.g[V0][0];
                                                                            c.g[V0][0] = 0x7;
                                                                            if t {
                                                                                break 'b_1b8710;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = c.g[V1][0]
                                                                                    == c.g[V0][0];
                                                                                c.g[V0][0] = 0x10;
                                                                                if t {
                                                                                    break 'b_1b8710;
                                                                                } else {
                                                                                    c.cycles += 2;
                                                                                    let t = true;
                                                                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x6d0), c.g[V0][0]);
                                                                                    if t {
                                                                                        break 'b_1b8848;
                                                                                    } else {
                                                                                        break 'b_1b8604;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 7;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    c.g[RA][0] = sx(0x001b8610);
                                                                    c.g[A2][0] = 0x16;
                                                                    Callee { addr: 0x001a97b8, f: ee::s03::NetFEState_OnChildEvent }.call(c, 0x001b8610)?;
                                                                    c.g[A3][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x20));
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    let t = true;
                                                                    c.g[A2][0] = c
                                                                        .lw((c.g[A1][0] as u32)
                                                                            .wrapping_add(0x6d0));
                                                                    if t {
                                                                        break 'b_1b8810;
                                                                    } else {
                                                                        break 'b_1b8620;
                                                                    }
                                                                }
                                                                0x001b8620 => {
                                                                    break 'b_1b8620;
                                                                }
                                                                0x001b86c4 => {
                                                                    break 'b_1b86c4;
                                                                }
                                                                0x001b875c => {
                                                                    break 'b_1b875c;
                                                                }
                                                                0x001b87ec => {
                                                                    break 'b_1b87ec;
                                                                }
                                                                0x001b883c => {
                                                                    break 'b_1b883c;
                                                                }
                                                                _ => {
                                                                    c.jump_out(
                                                                        t, 0x001b84d8, 0x001b8860,
                                                                    );
                                                                    return Ok(());
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.g[S2][0] = 0x10;
                                                        let t = c.g[A2][0] == c.g[S2][0];
                                                        c.g[V0][0] = (c.g[A2][0] < 0x11u64) as u64;
                                                        if t {
                                                            break 'b_1b871c;
                                                        } else {
                                                            'b_1b869c: {
                                                                'b_1b8660: {
                                                                    'b_1b8644: {
                                                                        c.cycles += 2;
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[V0][0] = 0xf;
                                                                        if t {
                                                                            break 'b_1b8644;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = c.g[A2][0]
                                                                                == c.g[V0][0];
                                                                            c.g[A0][0] = c.g[S0][0];
                                                                            if t {
                                                                                break 'b_1b8660;
                                                                            } else {
                                                                                c.cycles += 2;
                                                                                let t = true;

                                                                                if t {
                                                                                    break 'b_1b882c;
                                                                                } else {
                                                                                    break 'b_1b8644;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = 0x14;
                                                                    let t =
                                                                        c.g[A2][0] == c.g[V0][0];
                                                                    c.g[V0][0] = 0x16;
                                                                    if t {
                                                                        break 'b_1b871c;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = c.g[A2][0]
                                                                            == c.g[V0][0];
                                                                        c.g[A0][0] = c.g[S0][0];
                                                                        if t {
                                                                            break 'b_1b869c;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = true;

                                                                            if t {
                                                                                break 'b_1b882c;
                                                                            } else {
                                                                                break 'b_1b8660;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                'b_1b8694: {
                                                                    c.cycles += 6;
                                                                    c.g[A1][0] = c.g[S1][0];
                                                                    c.g[RA][0] = sx(0x001b866c);
                                                                    c.g[A2][0] = 0xf;
                                                                    Callee { addr: 0x001a97b8, f: ee::s03::NetFEState_OnChildEvent }.call(c, 0x001b866c)?;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S0][0] as u32)
                                                                            .wrapping_add(0x18));
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    if t {
                                                                        break 'b_1b8694;
                                                                    } else {
                                                                        c.cycles += 7;
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x6d0,
                                                                                ),
                                                                            0u64,
                                                                        );
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x6d4,
                                                                                ),
                                                                            0u64,
                                                                        );
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x6d8,
                                                                                ),
                                                                            0u64,
                                                                        );
                                                                        c.g[RA][0] = sx(0x001b868c);
                                                                        c.g[A1][0] = c.lw((c.g[A0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xb0c));
                                                                        Callee { addr: 0x001b70d0, f: ee::s03::cFEStateNetAccountMem_SetState }.call(c, 0x001b868c)?;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x30),
                                                                        );
                                                                        if t {
                                                                            break 'b_1b884c;
                                                                        } else {
                                                                            break 'b_1b8694;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.sw(
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x6d0),
                                                                    c.g[S2][0],
                                                                );
                                                                if t {
                                                                    break 'b_1b8848;
                                                                } else {
                                                                    break 'b_1b869c;
                                                                }
                                                            }
                                                            c.cycles += 7;
                                                            c.g[A2][0] = 0x16;
                                                            c.g[A1][0] = c.g[S1][0];
                                                            c.g[RA][0] = sx(0x001b86ac);
                                                            c.g[A0][0] = c.g[S0][0];
                                                            Callee {
                                                                addr: 0x001a97b8,
                                                                f: ee::s03::NetFEState_OnChildEvent,
                                                            }
                                                            .call(c, 0x001b86ac)?;
                                                            c.g[A2][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x6d0));
                                                            if c.g[A2][0] == 0u64 {
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                break 'b_1b884c;
                                                            } else {
                                                                c.cycles += 3;
                                                                c.g[A3][0] = c
                                                                    .lw((c.g[S0][0] as u32)
                                                                        .wrapping_add(0x20));
                                                                let t = true;
                                                                c.g[A1][0] = c.g[S0][0];
                                                                if t {
                                                                    break 'b_1b8810;
                                                                } else {
                                                                    break 'b_1b86c4;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x10;
                                                    let t = c.g[A2][0] == c.g[V0][0];
                                                    c.g[V0][0] = (c.g[A2][0] < 0x11u64) as u64;
                                                    if t {
                                                        break 'b_1b871c;
                                                    } else {
                                                        'b_1b8704: {
                                                            'b_1b86e8: {
                                                                c.cycles += 2;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V0][0] = 0xf;
                                                                if t {
                                                                    break 'b_1b86e8;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t =
                                                                        c.g[A2][0] == c.g[V0][0];
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    if t {
                                                                        break 'b_1b8704;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = true;

                                                                        if t {
                                                                            break 'b_1b882c;
                                                                        } else {
                                                                            break 'b_1b86e8;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V0][0] = 0x14;
                                                            let t = c.g[A2][0] == c.g[V0][0];
                                                            c.g[V0][0] = 0x16;
                                                            if t {
                                                                break 'b_1b871c;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = c.g[A2][0] == c.g[V0][0];
                                                                c.g[A0][0] = c.g[S0][0];
                                                                if t {
                                                                    break 'b_1b8734;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;

                                                                    if t {
                                                                        break 'b_1b882c;
                                                                    } else {
                                                                        break 'b_1b8704;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 3;
                                                        c.g[A1][0] = c.g[S1][0];
                                                        c.g[RA][0] = sx(0x001b8710);
                                                        c.g[A2][0] = 0xf;
                                                        Callee {
                                                            addr: 0x001a97b8,
                                                            f: ee::s03::NetFEState_OnChildEvent,
                                                        }
                                                        .call(c, 0x001b8710)?;
                                                        break 'b_1b8710;
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x11;
                                                let t = true;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x6d0),
                                                    c.g[V0][0],
                                                );
                                                if t {
                                                    break 'b_1b8848;
                                                } else {
                                                    break 'b_1b871c;
                                                }
                                            }
                                            c.cycles += 6;
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[RA][0] = sx(0x001b8728);
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x001a97b8,
                                                f: ee::s03::NetFEState_OnChildEvent,
                                            }
                                            .call(c, 0x001b8728)?;
                                            c.g[V0][0] = 0xf;
                                            let t = true;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x6d0),
                                                c.g[V0][0],
                                            );
                                            if t {
                                                break 'b_1b8848;
                                            } else {
                                                break 'b_1b8734;
                                            }
                                        }
                                        c.cycles += 7;
                                        c.g[A2][0] = 0x16;
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x001b8744);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x001a97b8,
                                            f: ee::s03::NetFEState_OnChildEvent,
                                        }
                                        .call(c, 0x001b8744)?;
                                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d0));
                                        if c.g[A2][0] == 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            break 'b_1b884c;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A3][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                            let t = true;
                                            c.g[A1][0] = c.g[S0][0];
                                            if t {
                                                break 'b_1b8810;
                                            } else {
                                                break 'b_1b875c;
                                            }
                                        }
                                    }
                                    'b_1b87c4: {
                                        'b_1b8778: {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0xf;
                                            let t = c.g[A2][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x16;
                                            if t {
                                                break 'b_1b8778;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.g[A0][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_1b87c4;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1b882c;
                                                    } else {
                                                        break 'b_1b8778;
                                                    }
                                                }
                                            }
                                        }
                                        'b_1b87a0: {
                                            c.cycles += 7;
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x001b8788);
                                            c.g[A2][0] = 0xf;
                                            Callee {
                                                addr: 0x001a97b8,
                                                f: ee::s03::NetFEState_OnChildEvent,
                                            }
                                            .call(c, 0x001b8788)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0xb08));
                                            if c.g[V0][0] != 0u64 {
                                                c.sw((c.g[S0][0] as u32).wrapping_add(0x6d0), 0u64);
                                                break 'b_1b87a0;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x10;
                                                let t = true;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x6d0),
                                                    c.g[V0][0],
                                                );
                                                if t {
                                                    break 'b_1b8848;
                                                } else {
                                                    break 'b_1b87a0;
                                                }
                                            }
                                        }
                                        c.cycles += 9;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x6d4), 0u64);
                                        c.g[A1][0] = 0u64;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x6d8), 0u64);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xb08), 0u64);
                                        c.g[RA][0] = sx(0x001b87bc);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xb0c), 0u64);
                                        Callee {
                                            addr: 0x001b70d0,
                                            f: ee::s03::cFEStateNetAccountMem_SetState,
                                        }
                                        .call(c, 0x001b87bc)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_1b884c;
                                        } else {
                                            break 'b_1b87c4;
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A2][0] = 0x16;
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x001b87d4);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001a97b8,
                                        f: ee::s03::NetFEState_OnChildEvent,
                                    }
                                    .call(c, 0x001b87d4)?;
                                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d0));
                                    if c.g[A2][0] == 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        break 'b_1b884c;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                        let t = true;
                                        c.g[A1][0] = c.g[S0][0];
                                        if t {
                                            break 'b_1b8810;
                                        } else {
                                            break 'b_1b87ec;
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x16;
                                let t = c.g[A2][0] != c.g[V0][0];
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_1b882c;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x001b8804);
                                    c.g[A2][0] = 0x16;
                                    Callee {
                                        addr: 0x001a97b8,
                                        f: ee::s03::NetFEState_OnChildEvent,
                                    }
                                    .call(c, 0x001b8804)?;
                                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                                    c.g[A1][0] = c.g[S0][0];
                                    c.g[A2][0] = 0xf;
                                    break 'b_1b8810;
                                }
                            }
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x88));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001b8824);
                            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001b8824)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1b884c;
                            } else {
                                break 'b_1b882c;
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001b8834);
                        c.g[A1][0] = c.g[S1][0];
                        Callee {
                            addr: 0x001a97b8,
                            f: ee::s03::NetFEState_OnChildEvent,
                        }
                        .call(c, 0x001b8834)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_1b884c;
                        } else {
                            break 'b_1b883c;
                        }
                    }
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001b8848);
                c.g[A1][0] = c.g[S1][0];
                Callee {
                    addr: 0x001a97b8,
                    f: ee::s03::NetFEState_OnChildEvent,
                }
                .call(c, 0x001b8848)?;
                break 'b_1b8848;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1b884c;
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

/// `cFEStateCreateStep1_OnWidgetAction` (0x001b8c38): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_create_step1_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001b8c38 {
        return ee::s03::f_001b8c38__generated(c);
    }
    guest(c, |c| 'l_1b8c38: loop {
        'b_1b9074: {
            'b_1b9070: {
                'b_1b906c: {
                    'b_1b9060: {
                        c.cycles += 10;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S1][0] = c.g[A0][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.g[S2][0] = c.g[A1][0];
                        let t = c.g[V0][0] == 0u64;
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        if t {
                            break 'b_1b9060;
                        } else {
                            'b_1b9014: {
                                'b_1b8fd8: {
                                    'b_1b8fc8: {
                                        'b_1b8f34: {
                                            'b_1b8cf0: {
                                                c.cycles += 7;
                                                c.g[V0][0] = 0x460000;
                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x4880));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x001b8c38 => {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001b8c38);
                                                        }
                                                        continue 'l_1b8c38;
                                                    }
                                                    0x001b8c7c => {
                                                        'b_1b8cb8: {
                                                            c.cycles += 6;
                                                            c.g[A0][0] = 0x460000;
                                                            c.g[S0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                            c.g[RA][0] = sx(0x001b8c8c);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4780));
                                                            call_ptr(c, 0x00317670, 0x001b8c8c)?;
                                                            if c.g[S0][0] != c.g[V0][0] {
                                                                c.g[A0][0] = 0x460000;
                                                                break 'b_1b8cb8;
                                                            } else {
                                                                c.cycles += 2;
                                                                if c.g[S2][0] == 0u64 {
                                                                    c.g[A0][0] = 0x460000;
                                                                    break 'b_1b8cb8;
                                                                } else {
                                                                    c.cycles += 7;
                                                                    c.g[RA][0] = sx(0x001b8ca4);
                                                                    c.g[A0][0] = c.g[S2][0];
                                                                    call_ptr(
                                                                        c, 0x0039a738, 0x001b8ca4,
                                                                    )?;
                                                                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x6d0));
                                                                    c.g[V0][0] = (c.g[V0][0]
                                                                        < 0x1u64)
                                                                        as u64;
                                                                    c.sw(
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x38),
                                                                        c.g[V0][0],
                                                                    );
                                                                    c.g[A0][0] = 0x460000;
                                                                    break 'b_1b8cb8;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 5;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                        c.g[RA][0] = sx(0x001b8cc4);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x4790));
                                                        call_ptr(c, 0x00317670, 0x001b8cc4)?;
                                                        let t = c.g[S0][0] != c.g[V0][0];
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x30),
                                                        );
                                                        if t {
                                                            break 'b_1b9070;
                                                        } else {
                                                            c.cycles += 2;
                                                            if c.g[S2][0] == 0u64 {
                                                                c.lq(
                                                                    17,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x20),
                                                                );
                                                                break 'b_1b9074;
                                                            } else {
                                                                c.cycles += 7;
                                                                c.g[RA][0] = sx(0x001b8cdc);
                                                                c.g[A0][0] = c.g[S2][0];
                                                                call_ptr(
                                                                    c, 0x0039a738, 0x001b8cdc,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x6d0));
                                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                                c.g[V0][0] =
                                                                    (c.g[V0][0] < 0x1u64) as u64;
                                                                let t = true;
                                                                c.sw(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x3c),
                                                                    c.g[V0][0],
                                                                );
                                                                if t {
                                                                    break 'b_1b906c;
                                                                } else {
                                                                    break 'b_1b8cf0;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    0x001b8cf0 => {
                                                        break 'b_1b8cf0;
                                                    }
                                                    0x001b8f34 => {
                                                        break 'b_1b8f34;
                                                    }
                                                    0x001b8fd8 => {
                                                        break 'b_1b8fd8;
                                                    }
                                                    0x001b9014 => {
                                                        break 'b_1b9014;
                                                    }
                                                    0x001b9060 => {
                                                        break 'b_1b9060;
                                                    }
                                                    0x001b906c => {
                                                        break 'b_1b906c;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x001b8c38, 0x001b9084);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            'b_1b8ecc: {
                                                'b_1b8eb4: {
                                                    'b_1b8d80: {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = 0x460000;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                        c.g[RA][0] = sx(0x001b8d00);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x4768));
                                                        call_ptr(c, 0x00317670, 0x001b8d00)?;
                                                        if c.g[S0][0] != c.g[V0][0] {
                                                            c.g[A0][0] = 0x460000;
                                                            break 'b_1b8d80;
                                                        } else {
                                                            c.cycles += 9;
                                                            c.g[A0][0] = 0x460000;
                                                            c.g[RA][0] = sx(0x001b8d14);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x2b88));
                                                            call_ptr(c, 0x00317670, 0x001b8d14)?;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[RA][0] = sx(0x001b8d20);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x0039d860, 0x001b8d20)?;
                                                            c.g[S0][0] = c.g[V0][0];
                                                            let t = c.g[S0][0] == 0u64;
                                                            c.g[A0][0] = 0x460000;
                                                            if t {
                                                                break 'b_1b906c;
                                                            } else {
                                                                c.cycles += 21;
                                                                c.g[RA][0] = sx(0x001b8d34);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x47b8));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001b8d34,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x6d0));
                                                                c.g[A2][0] = c.g[V0][0];
                                                                c.g[A1][0] = c.g[S0][0];
                                                                c.g[A0][0] = c.g[S1][0];
                                                                c.g[A3][0] =
                                                                    c.lw((c.g[V1][0] as u32));
                                                                c.g[T0][0] = 0u64;
                                                                c.g[T1][0] = 0xa;
                                                                c.g[RA][0] = sx(0x001b8d58);
                                                                c.g[T2][0] = 0x10;
                                                                Callee { addr: 0x001a9150, f: ee::s03::NetFEState_OpenKeyboardTextId }.call(c, 0x001b8d58)?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x69c));
                                                                c.g[V0][0] = 0x1;
                                                                c.g[A1][0] = 0x4b;
                                                                c.g[A2][0] = 0x1;
                                                                c.sw(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x58),
                                                                    c.g[V0][0],
                                                                );
                                                                c.g[A3][0] = 0x2;
                                                                c.g[RA][0] = sx(0x001b8d78);
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x69c));
                                                                call_ptr(
                                                                    c, 0x001ce3c8, 0x001b8d78,
                                                                )?;
                                                                let t = true;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x69c));
                                                                if t {
                                                                    break 'b_1b8eb4;
                                                                } else {
                                                                    break 'b_1b8d80;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    'b_1b8ea8: {
                                                        'b_1b8de4: {
                                                            c.cycles += 5;
                                                            c.g[S0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                            c.g[RA][0] = sx(0x001b8d8c);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x47d0));
                                                            call_ptr(c, 0x00317670, 0x001b8d8c)?;
                                                            if c.g[S0][0] != c.g[V0][0] {
                                                                c.g[A0][0] = 0x460000;
                                                                break 'b_1b8de4;
                                                            } else {
                                                                c.cycles += 9;
                                                                c.g[A0][0] = 0x460000;
                                                                c.g[RA][0] = sx(0x001b8da0);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x2ba0));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001b8da0,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x40));
                                                                c.g[RA][0] = sx(0x001b8dac);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x0039d860, 0x001b8dac,
                                                                )?;
                                                                c.g[S0][0] = c.g[V0][0];
                                                                let t = c.g[S0][0] == 0u64;
                                                                c.g[A0][0] = 0x460000;
                                                                if t {
                                                                    break 'b_1b906c;
                                                                } else {
                                                                    c.cycles += 11;
                                                                    c.g[RA][0] = sx(0x001b8dc0);
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x2d00));
                                                                    call_ptr(
                                                                        c, 0x00317670, 0x001b8dc0,
                                                                    )?;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x6d0));
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    c.g[A2][0] = c.g[V0][0];
                                                                    c.g[A0][0] = c.g[S1][0];
                                                                    c.g[A3][0] = c
                                                                        .lw((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    c.g[T0][0] = 0x1;
                                                                    c.g[T1][0] = 0xb;
                                                                    let t = true;
                                                                    c.g[T2][0] = 0x10;
                                                                    if t {
                                                                        break 'b_1b8ea8;
                                                                    } else {
                                                                        break 'b_1b8de4;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        'b_1b8e48: {
                                                            c.cycles += 5;
                                                            c.g[S0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                            c.g[RA][0] = sx(0x001b8df0);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x47e0));
                                                            call_ptr(c, 0x00317670, 0x001b8df0)?;
                                                            if c.g[S0][0] != c.g[V0][0] {
                                                                c.g[A0][0] = 0x460000;
                                                                break 'b_1b8e48;
                                                            } else {
                                                                c.cycles += 9;
                                                                c.g[A0][0] = 0x460000;
                                                                c.g[RA][0] = sx(0x001b8e04);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x47f8));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001b8e04,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x40));
                                                                c.g[RA][0] = sx(0x001b8e10);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x0039d860, 0x001b8e10,
                                                                )?;
                                                                c.g[S0][0] = c.g[V0][0];
                                                                let t = c.g[S0][0] == 0u64;
                                                                c.g[A0][0] = 0x460000;
                                                                if t {
                                                                    break 'b_1b906c;
                                                                } else {
                                                                    c.cycles += 11;
                                                                    c.g[RA][0] = sx(0x001b8e24);
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(0x4810));
                                                                    call_ptr(
                                                                        c, 0x00317670, 0x001b8e24,
                                                                    )?;
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x6d0));
                                                                    c.g[A1][0] = c.g[S0][0];
                                                                    c.g[A2][0] = c.g[V0][0];
                                                                    c.g[A0][0] = c.g[S1][0];
                                                                    c.g[A3][0] = c
                                                                        .lw((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x8));
                                                                    c.g[T0][0] = 0x1;
                                                                    c.g[T1][0] = 0xc;
                                                                    let t = true;
                                                                    c.g[T2][0] = 0x10;
                                                                    if t {
                                                                        break 'b_1b8ea8;
                                                                    } else {
                                                                        break 'b_1b8e48;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 5;
                                                        c.g[S0][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x38));
                                                        c.g[RA][0] = sx(0x001b8e54);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x4828));
                                                        call_ptr(c, 0x00317670, 0x001b8e54)?;
                                                        if c.g[S0][0] != c.g[V0][0] {
                                                            c.g[A0][0] = 0x460000;
                                                            break 'b_1b8ecc;
                                                        } else {
                                                            c.cycles += 9;
                                                            c.g[A0][0] = 0x460000;
                                                            c.g[RA][0] = sx(0x001b8e68);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4838));
                                                            call_ptr(c, 0x00317670, 0x001b8e68)?;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[RA][0] = sx(0x001b8e74);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x0039d860, 0x001b8e74)?;
                                                            c.g[S0][0] = c.g[V0][0];
                                                            let t = c.g[S0][0] == 0u64;
                                                            c.g[A0][0] = 0x460000;
                                                            if t {
                                                                break 'b_1b906c;
                                                            } else {
                                                                c.cycles += 10;
                                                                c.g[RA][0] = sx(0x001b8e88);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x2cd0));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001b8e88,
                                                                )?;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x6d0));
                                                                c.g[A1][0] = c.g[S0][0];
                                                                c.g[A2][0] = c.g[V0][0];
                                                                c.g[A0][0] = c.g[S1][0];
                                                                c.g[A3][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0xc));
                                                                c.g[T0][0] = 0u64;
                                                                c.g[T1][0] = 0xd;
                                                                c.g[T2][0] = 0x32;
                                                                break 'b_1b8ea8;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 3;
                                                    c.g[RA][0] = sx(0x001b8eb0);
                                                    Callee {
                                                        addr: 0x001a9150,
                                                        f: ee::s03::NetFEState_OpenKeyboardTextId,
                                                    }
                                                    .call(c, 0x001b8eb0)?;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x69c)
                                                        );
                                                    break 'b_1b8eb4;
                                                }
                                                c.cycles += 6;
                                                c.g[A1][0] = 0x35;
                                                c.g[A2][0] = 0u64;
                                                c.g[RA][0] = sx(0x001b8ec4);
                                                c.g[A3][0] = 0x2;
                                                call_ptr(c, 0x001ce3c8, 0x001b8ec4)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_1b9070;
                                                } else {
                                                    break 'b_1b8ecc;
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[S0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                            c.g[RA][0] = sx(0x001b8ed8);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x47a8));
                                            call_ptr(c, 0x00317670, 0x001b8ed8)?;
                                            if c.g[S0][0] != c.g[V0][0] {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                break 'b_1b9070;
                                            } else {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001b8ee8);
                                                c.g[A0][0] = c.g[S1][0];
                                                Callee {
                                                    addr: 0x001ba110,
                                                    f: ee::s03::cFEStateCreateStep1_ValidateFields,
                                                }
                                                .call(c, 0x001b8ee8)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_1b9070;
                                                } else {
                                                    'b_1b8f24: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x6ec));
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A1][0] = 0x460000;
                                                        if t {
                                                            break 'b_1b8f24;
                                                        } else {
                                                            c.cycles += 10;
                                                            c.g[A0][0] = 0x6e0;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x4848));
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x001b8f10);
                                                            c.g[A3][0] = 0u64;
                                                            call_ptr(c, 0x00317d70, 0x001b8f10)?;
                                                            c.g[A1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[RA][0] = sx(0x001b8f1c);
                                                            c.g[A0][0] = c.g[V0][0];
                                                            Callee { addr: 0x001ba3e0, f: ee::s03::cFEStateCreateStep2_ctor }.call(c, 0x001b8f1c)?;
                                                            let t = true;
                                                            c.g[A0][0] = c.g[S1][0];
                                                            if t {
                                                                break 'b_1b8fc8;
                                                            } else {
                                                                break 'b_1b8f24;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x001b8f2c);
                                                    c.g[A0][0] = c.g[S1][0];
                                                    Callee {
                                                        addr: 0x001ba280,
                                                        f: ee::s03::cFEStateCreateStep1_SubmitEdits,
                                                    }
                                                    .call(c, 0x001b8f2c)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_1b9070;
                                                    } else {
                                                        break 'b_1b8f34;
                                                    }
                                                }
                                            }
                                        }
                                        'b_1b8f98: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x6ec));
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A0][0] = 0xb10;
                                            if t {
                                                break 'b_1b8f98;
                                            } else {
                                                c.cycles += 22;
                                                c.g[A1][0] = 0x460000;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x2d10));
                                                c.g[A2][0] = 0u64;
                                                c.g[RA][0] = sx(0x001b8f54);
                                                c.g[A3][0] = 0u64;
                                                call_ptr(c, 0x00317d70, 0x001b8f54)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[A2][0] = c.g[S1][0];
                                                c.g[A3][0] = 0u64;
                                                c.g[T0][0] = 0u64;
                                                c.g[RA][0] = sx(0x001b8f70);
                                                c.g[T1][0] = 0u64;
                                                Callee {
                                                    addr: 0x001b6ed0,
                                                    f: ee::s03::cFEStateNetAccountMem_ctor,
                                                }
                                                .call(c, 0x001b8f70)?;
                                                c.g[V1][0] = 0x832;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.sw(
                                                    (c.g[V0][0] as u32).wrapping_add(0x18),
                                                    c.g[V1][0],
                                                );
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                                                c.g[A0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                                c.g[RA][0] = sx(0x001b8f90);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                                call_ptr(c, 0x0039f290, 0x001b8f90)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_1b9070;
                                                } else {
                                                    break 'b_1b8f98;
                                                }
                                            }
                                        }
                                        c.cycles += 12;
                                        c.g[RA][0] = sx(0x001b8fa0);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001ba218,
                                            f: ee::s03::cFEStateCreateStep1_RestoreAccountInfo,
                                        }
                                        .call(c, 0x001b8fa0)?;
                                        c.g[A1][0] = 0x460000;
                                        c.g[A0][0] = 0x6d8;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d68));
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001b8fb8);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x001b8fb8)?;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = sx(0x001b8fc4);
                                        c.g[A0][0] = c.g[V0][0];
                                        Callee {
                                            addr: 0x001ad130,
                                            f: ee::s03::cFEStateSelectPersona_ctor,
                                        }
                                        .call(c, 0x001b8fc4)?;
                                        c.g[A0][0] = c.g[S1][0];
                                        break 'b_1b8fc8;
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001b8fd0);
                                    c.g[A1][0] = c.g[V0][0];
                                    Callee {
                                        addr: 0x001a8a40,
                                        f: ee::s03::NetFEState_GotoNextState,
                                    }
                                    .call(c, 0x001b8fd0)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_1b9070;
                                    } else {
                                        break 'b_1b8fd8;
                                    }
                                }
                                c.cycles += 5;
                                c.g[RA][0] = sx(0x001b8fe0);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001b9820,
                                    f: ee::s03::cFEStateCreateStep1_OnKeyboardDone,
                                }
                                .call(c, 0x001b8fe0)?;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6f0));
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = c.g[S1][0];
                                if t {
                                    break 'b_1b906c;
                                } else {
                                    c.cycles += 10;
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.g[S1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001b8ffc);
                                    c.g[S1][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                                    Callee {
                                        addr: 0x001b9558,
                                        f: ee::s03::cFEStateCreateStep1_AllFieldsFilled,
                                    }
                                    .call(c, 0x001b8ffc)?;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x44));
                                    c.g[A0][0] = c.g[S1][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001b900c);
                                    c.g[A1][0] = c.g[V0][0] ^ 0x1;
                                    call_ptr(c, t, 0x001b900c)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_1b9070;
                                    } else {
                                        break 'b_1b9014;
                                    }
                                }
                            }
                            'b_1b904c: {
                                c.cycles += 13;
                                c.g[A1][0] = 0x460000;
                                c.g[A0][0] = 0x6dc;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4860));
                                c.g[A2][0] = 0x100;
                                c.g[RA][0] = sx(0x001b902c);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x001b902c)?;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x001b9038);
                                c.g[A0][0] = c.g[V0][0];
                                Callee {
                                    addr: 0x001c0418,
                                    f: ee::s03::cFEStateOnlineAgreement_ctor,
                                }
                                .call(c, 0x001b9038)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6ec));
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x14;
                                if t {
                                    break 'b_1b904c;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = 0x15;
                                    break 'b_1b904c;
                                }
                            }
                            c.cycles += 5;
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                            c.g[RA][0] = sx(0x001b9058);
                            c.g[A0][0] = c.g[S1][0];
                            Callee {
                                addr: 0x001a8a40,
                                f: ee::s03::NetFEState_GotoNextState,
                            }
                            .call(c, 0x001b9058)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1b9070;
                            } else {
                                break 'b_1b9060;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001b906c);
                    c.g[A1][0] = c.g[S2][0];
                    Callee {
                        addr: 0x001a8918,
                        f: ee::s03::NetFEState_OnWidgetAction,
                    }
                    .call(c, 0x001b906c)?;
                    break 'b_1b906c;
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_1b9070;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1b9074;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateCreateStep1_OnEditDonePopupEvent` (0x001b92c8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_create_step1_on_edit_done_popup_event(c: &mut Cpu) {
    if c.pc != 0x001b92c8 {
        return ee::s03::f_001b92c8__generated(c);
    }
    guest(c, |c| {
        'b_1b9414: {
            'b_1b9410: {
                'b_1b9408: {
                    c.cycles += 9;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xf));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    if t {
                        break 'b_1b9408;
                    } else {
                        'b_1b9334: {
                            'b_1b931c: {
                                c.cycles += 7;
                                c.g[V0][0] = 0x460000;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4940));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                let t = c.g[A0][0] as u32;
                                match t {
                                    0x001b9308 => {
                                        c.cycles += 5;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x6e8), 0u64);
                                        c.g[RA][0] = sx(0x001b9314);
                                        c.g[A0][0] = c.g[S2][0];
                                        Callee {
                                            addr: 0x001a97b8,
                                            f: ee::s03::NetFEState_OnChildEvent,
                                        }
                                        .call(c, 0x001b9314)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_1b9414;
                                        } else {
                                            break 'b_1b931c;
                                        }
                                    }
                                    0x001b931c => {
                                        break 'b_1b931c;
                                    }
                                    0x001b9334 => {
                                        break 'b_1b9334;
                                    }
                                    0x001b9408 => {
                                        break 'b_1b9408;
                                    }
                                    _ => {
                                        c.jump_out(t, 0x001b92c8, 0x001b9428);
                                        return Ok(());
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[V0][0] = 0x1;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001b932c);
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x6e8), c.g[V0][0]);
                            Callee {
                                addr: 0x001a97b8,
                                f: ee::s03::NetFEState_OnChildEvent,
                            }
                            .call(c, 0x001b932c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1b9414;
                            } else {
                                break 'b_1b9334;
                            }
                        }
                        'b_1b93d4: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6e8));
                            if c.g[V0][0] == 0u64 {
                                c.g[A1][0] = 0x460000;
                                break 'b_1b93d4;
                            } else {
                                c.cycles += 37;
                                c.g[RA][0] = sx(0x001b9348);
                                c.g[A0][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x001a97b8,
                                    f: ee::s03::NetFEState_OnChildEvent,
                                }
                                .call(c, 0x001b9348)?;
                                c.g[RA][0] = sx(0x001b9350);
                                c.g[A0][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x001a9c50,
                                    f: ee::s03::NetFEState_GetPopup,
                                }
                                .call(c, 0x001b9350)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A0][0] = 0x460000;
                                c.g[V0][0] = 0x2;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2d80));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                                c.g[RA][0] = sx(0x001b936c);
                                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                call_ptr(c, 0x00317670, 0x001b936c)?;
                                c.g[V1][0] = 0x3;
                                c.g[A0][0] = 0x460000;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                c.g[RA][0] = sx(0x001b9384);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                call_ptr(c, 0x00317670, 0x001b9384)?;
                                c.g[A2][0] = c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                c.g[RA][0] = sx(0x001b9394);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001c57c0,
                                    f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                }
                                .call(c, 0x001b9394)?;
                                c.g[A0][0] = 0x460000;
                                c.g[RA][0] = sx(0x001b93a0);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                call_ptr(c, 0x00317670, 0x001b93a0)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A2][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001b93b0);
                                c.g[A1][0] = 0x1;
                                Callee {
                                    addr: 0x001c57c0,
                                    f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                }
                                .call(c, 0x001b93b0)?;
                                c.g[V0][0] = 0x81e;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x168), 0u64);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x164), 0u64);
                                c.g[A1][0] = c.g[S0][0];
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                                c.g[RA][0] = sx(0x001b93cc);
                                c.g[A0][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x001a9bc0,
                                    f: ee::s03::NetFEState_ShowPopup,
                                }
                                .call(c, 0x001b93cc)?;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x6e8), 0u64);
                                if t {
                                    break 'b_1b9410;
                                } else {
                                    break 'b_1b93d4;
                                }
                            }
                        }
                        c.cycles += 13;
                        c.g[A0][0] = 0x6d8;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d68));
                        c.g[A2][0] = 0u64;
                        c.g[RA][0] = sx(0x001b93e8);
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x001b93e8)?;
                        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x001b93f4);
                        c.g[A0][0] = c.g[V0][0];
                        Callee {
                            addr: 0x001ad130,
                            f: ee::s03::cFEStateSelectPersona_ctor,
                        }
                        .call(c, 0x001b93f4)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001b9400);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001a8a40,
                            f: ee::s03::NetFEState_GotoNextState,
                        }
                        .call(c, 0x001b9400)?;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x6e8), 0u64);
                        if t {
                            break 'b_1b9410;
                        } else {
                            break 'b_1b9408;
                        }
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x001b9410);
                c.g[A0][0] = c.g[S2][0];
                Callee {
                    addr: 0x001a97b8,
                    f: ee::s03::NetFEState_OnChildEvent,
                }
                .call(c, 0x001b9410)?;
                break 'b_1b9410;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1b9414;
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
    use crate::difftest::{check, fakes, fill, fresh, run_generated, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's slot fakes (the fakes fall back to the
    /// generated bodies when it is not set), the result mask word, the value
    /// the `EAString` lookup fake returns.
    const CTRL: u32 = DATA + 0x7e000;
    const MAGIC: u32 = 0x0913_c0de;
    const KEY: u32 = DATA + 0x7d000;

    /// Gated fakes for every callee that has an override slot. Mode 1: a
    /// call whose bit (`log index & 15`) is set in the mask word returns 0;
    /// mode 2: returns `KEY` (the string-table lookup).
    macro_rules! oc913_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $mode:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(CTRL) != MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    let n = c.bus.read32(fakes::LOG_COUNT);
                    fakes::fake_call(c, $addr);
                    if c.pc >> 16 != 0xdead {
                        match $mode {
                            1 => {
                                if c.bus.read32(CTRL + 4) >> (n & 15) & 1 != 0 {
                                    c.g[V0][0] = 0;
                                }
                            }
                            2 => c.g[V0][0] = c.bus.read32(CTRL + 8) as u64,
                            _ => {}
                        }
                    }
                }
            )*
            fn oc913_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc913_fakes!(
        (
            oc913_fake_001a8918,
            OC913_PREV_001A8918,
            ee::s03::OVERRIDE_001A8918,
            ee::s03::f_001a8918__generated,
            0x001a_8918,
            1
        ),
        (
            oc913_fake_001a8a40,
            OC913_PREV_001A8A40,
            ee::s03::OVERRIDE_001A8A40,
            ee::s03::f_001a8a40__generated,
            0x001a_8a40,
            1
        ),
        (
            oc913_fake_001a9150,
            OC913_PREV_001A9150,
            ee::s03::OVERRIDE_001A9150,
            ee::s03::f_001a9150__generated,
            0x001a_9150,
            1
        ),
        (
            oc913_fake_001a97b8,
            OC913_PREV_001A97B8,
            ee::s03::OVERRIDE_001A97B8,
            ee::s03::f_001a97b8__generated,
            0x001a_97b8,
            1
        ),
        (
            oc913_fake_001a9bc0,
            OC913_PREV_001A9BC0,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a_9bc0,
            1
        ),
        (
            oc913_fake_001a9c50,
            OC913_PREV_001A9C50,
            ee::s03::OVERRIDE_001A9C50,
            ee::s03::f_001a9c50__generated,
            0x001a_9c50,
            0
        ),
        (
            oc913_fake_001ad130,
            OC913_PREV_001AD130,
            ee::s03::OVERRIDE_001AD130,
            ee::s03::f_001ad130__generated,
            0x001a_d130,
            0
        ),
        (
            oc913_fake_001b6ed0,
            OC913_PREV_001B6ED0,
            ee::s03::OVERRIDE_001B6ED0,
            ee::s03::f_001b6ed0__generated,
            0x001b_6ed0,
            0
        ),
        (
            oc913_fake_001b70d0,
            OC913_PREV_001B70D0,
            ee::s03::OVERRIDE_001B70D0,
            ee::s03::f_001b70d0__generated,
            0x001b_70d0,
            1
        ),
        (
            oc913_fake_001b8890,
            OC913_PREV_001B8890,
            ee::s03::OVERRIDE_001B8890,
            ee::s03::f_001b8890__generated,
            0x001b_8890,
            0
        ),
        (
            oc913_fake_001b9558,
            OC913_PREV_001B9558,
            ee::s03::OVERRIDE_001B9558,
            ee::s03::f_001b9558__generated,
            0x001b_9558,
            1
        ),
        (
            oc913_fake_001b9820,
            OC913_PREV_001B9820,
            ee::s03::OVERRIDE_001B9820,
            ee::s03::f_001b9820__generated,
            0x001b_9820,
            1
        ),
        (
            oc913_fake_001ba110,
            OC913_PREV_001BA110,
            ee::s03::OVERRIDE_001BA110,
            ee::s03::f_001ba110__generated,
            0x001b_a110,
            1
        ),
        (
            oc913_fake_001ba218,
            OC913_PREV_001BA218,
            ee::s03::OVERRIDE_001BA218,
            ee::s03::f_001ba218__generated,
            0x001b_a218,
            1
        ),
        (
            oc913_fake_001ba280,
            OC913_PREV_001BA280,
            ee::s03::OVERRIDE_001BA280,
            ee::s03::f_001ba280__generated,
            0x001b_a280,
            1
        ),
        (
            oc913_fake_001ba3e0,
            OC913_PREV_001BA3E0,
            ee::s03::OVERRIDE_001BA3E0,
            ee::s03::f_001ba3e0__generated,
            0x001b_a3e0,
            0
        ),
        (
            oc913_fake_001c0418,
            OC913_PREV_001C0418,
            ee::s03::OVERRIDE_001C0418,
            ee::s03::f_001c0418__generated,
            0x001c_0418,
            0
        ),
        (
            oc913_fake_001c57c0,
            OC913_PREV_001C57C0,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c_57c0,
            1
        ),
        (
            oc913_fake_001ce3c8,
            OC913_PREV_001CE3C8,
            ee::s04::OVERRIDE_001CE3C8,
            ee::s04::f_001ce3c8__generated,
            0x001c_e3c8,
            1
        ),
        (
            oc913_fake_00265d68,
            OC913_PREV_00265D68,
            ee::s07::OVERRIDE_00265D68,
            ee::s07::f_00265d68__generated,
            0x0026_5d68,
            1
        ),
        (
            oc913_fake_002c26d0,
            OC913_PREV_002C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0,
            1
        ),
        (
            oc913_fake_002c2718,
            OC913_PREV_002C2718,
            ee::s08::OVERRIDE_002C2718,
            ee::s08::f_002c2718__generated,
            0x002c_2718,
            1
        ),
        (
            oc913_fake_00317670,
            OC913_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            2
        ),
        (
            oc913_fake_00317d70,
            OC913_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70,
            0
        ),
        (
            oc913_fake_0039a738,
            OC913_PREV_0039A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738,
            1
        ),
        (
            oc913_fake_0039d860,
            OC913_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860,
            1
        ),
    );

    /// The gate, the mask and the lookup value, after `machine`.
    fn gate(c: &mut Cpu, r: &mut Rng) {
        oc913_install_fakes();
        c.bus.write32(CTRL, MAGIC);
        c.bus.write32(CTRL + 4, r.next_u32() & 0xffff);
        c.bus.write32(CTRL + 8, KEY);
    }

    /// A jump table of `n` words at `base`: the block addresses `arms` in
    /// turn (rotated by `seed`), now and then a foreign address that leaves
    /// through `jump_out`.
    fn table(c: &mut Cpu, base: u32, n: u32, arms: &[u32], seed: u32) {
        for i in 0..n {
            let v = if (i + seed) % 7 == 3 {
                0x0040_0000 + 8 * i
            } else {
                arms[((i + seed / 3) as usize) % arms.len()]
            };
            c.bus.write32(base + 4 * i, v);
        }
    }

    const SELF: u32 = DATA + 0x1000;
    const WIDGET: u32 = DATA + 0x2000;

    fn objects(c: &mut Cpu, r: &mut Rng) {
        for base in [SELF, WIDGET, DATA + 0x3000, DATA + 0x4000, DATA + 0x5000] {
            fill(c, r, base, 0x400);
        }
        for base in [DATA + 0x6000, DATA + 0x7000, DATA + 0x8000, DATA + 0x9000] {
            for i in 0..0x400 {
                c.bus.write32(base + 4 * i, 0);
            }
        }
    }

    // ---------------------------------------------------------------- 0x1b83c0

    fn setup_format_failed_text(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x913_83c0, 6);
        gate(c, &mut r);
        objects(c, &mut r);
        c.bus.write32(SELF + 0x6dc, seed % 32);
        // gp-0x848: the front-end manager, +0x8c its list, +4 the entry.
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x3000);
        c.bus.write32(DATA + 0x3000 + 0x8c, DATA + 0x4000);
        c.bus.write32(DATA + 0x4000 + 4, DATA + 0x5000);
        c.bus
            .write16(DATA + 0x5000 + 0x20, 0x10 * (seed % 3) as u16);
        c.bus.write32(DATA + 0x5000 + 0x24, fakes::FAKE + 0xc0);
        table(
            c,
            0x0046_4660,
            0x1a,
            &[
                0x001b_8418,
                0x001b_8428,
                0x001b_8438,
                0x001b_8480,
                0x001b_84c0,
            ],
            seed,
        );
        c.g[A0][0] = SELF as u64;
        c.pc = 0x001b_83c0;
    }

    #[test]
    fn c_fe_state_net_account_mem_format_failed_text_matches() {
        check(
            ee::s03::f_001b83c0__generated,
            c_fe_state_net_account_mem_format_failed_text,
            96,
            &setup_format_failed_text,
        );
    }

    // ---------------------------------------------------------------- 0x1b84d8

    fn setup_on_child_event(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x913_84d8, 8);
        gate(c, &mut r);
        objects(c, &mut r);
        let msgs = [15u32, 16, 20, 22, 15, 16, 23, 1];
        c.bus.write32(SELF + 0x6dc, seed % 32);
        c.bus.write32(SELF + 0x18, DATA + 0x3000);
        c.bus.write32(SELF + 0x20, DATA + 0x4000);
        c.bus.write32(DATA + 0x4000 + 8, DATA + 0x5000);
        c.bus
            .write16(DATA + 0x5000 + 0x88, 0x10 * (seed % 3) as u16);
        c.bus.write32(DATA + 0x5000 + 0x8c, fakes::FAKE + 0x40);
        if (seed / 32) % 2 == 0 {
            c.bus.write32(SELF + 0x6d8, 0);
        }
        if (seed / 64) % 3 == 0 {
            c.bus.write32(WIDGET + 0x18, 0);
        }
        table(
            c,
            0x0046_46d0,
            0x1b,
            &[
                0x001b_8520,
                0x001b_8620,
                0x001b_86c4,
                0x001b_875c,
                0x001b_87ec,
                0x001b_883c,
            ],
            seed / 32,
        );
        c.g[A0][0] = SELF as u64;
        c.g[A1][0] = WIDGET as u64;
        c.g[A2][0] = msgs[((seed / 32) % 8) as usize] as u64;
        c.pc = 0x001b_84d8;
    }

    #[test]
    fn c_fe_state_net_account_mem_on_child_event_matches() {
        check(
            ee::s03::f_001b84d8__generated,
            c_fe_state_net_account_mem_on_child_event,
            256,
            &setup_on_child_event,
        );
    }

    /// The fixture must reach many different paths of the dispatcher.
    #[test]
    fn c_fe_state_net_account_mem_on_child_event_covers_paths() {
        let mut costs = std::collections::BTreeSet::new();
        for seed in 0..256 {
            let mut c = fresh();
            setup_on_child_event(&mut c, seed);
            run_generated(&mut c, ee::s03::f_001b84d8__generated);
            costs.insert(c.cycles);
        }
        assert!(
            costs.len() >= 15,
            "fixture reaches only {} distinct paths: {costs:?}",
            costs.len()
        );
    }

    // ---------------------------------------------------------------- 0x1b8c38

    fn setup_widget_action(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x913_8c38, 8);
        gate(c, &mut r);
        objects(c, &mut r);
        c.bus.write32(SELF + 0x6d0, DATA + 0x3000);
        c.bus.write32(SELF + 0x40, DATA + 0x4000);
        c.bus.write32(DATA + 0x4000 + 0xd0, DATA + 0x6000);
        c.bus.write32(DATA + 0x6000 + 0x10, DATA + 0x9000);
        let keyboard = (seed / 12) % 4;
        if keyboard == 0 {
            c.bus.write32(SELF + 0x6f0, 0);
        } else {
            c.bus.write32(SELF + 0x6f0, DATA + 0x7000);
            c.bus.write32(DATA + 0x7000 + 8, DATA + 0x8000);
            c.bus
                .write16(DATA + 0x8000 + 0x40, 0x10 * (seed % 3) as u16);
            c.bus.write32(DATA + 0x8000 + 0x44, fakes::FAKE + 0x80);
        }
        c.bus.write32(
            WIDGET + 0x38,
            if (seed / 12) % 2 == 0 { KEY } else { 0x1234 },
        );
        c.bus.write32(SELF + 0x6ec, (seed / 24) % 2);
        table(
            c,
            0x0046_4880,
            10,
            &[
                0x001b_8c7c,
                0x001b_8cf0,
                0x001b_8f34,
                0x001b_8fd8,
                0x001b_9014,
                0x001b_9060,
                0x001b_906c,
            ],
            seed / 12,
        );
        c.g[A0][0] = SELF as u64;
        c.g[A1][0] = WIDGET as u64;
        c.g[A2][0] = (4 + seed % 12) as u64;
        c.pc = 0x001b_8c38;
    }

    #[test]
    fn c_fe_state_create_step1_on_widget_action_matches() {
        check(
            ee::s03::f_001b8c38__generated,
            c_fe_state_create_step1_on_widget_action,
            240,
            &setup_widget_action,
        );
    }

    #[test]
    fn c_fe_state_create_step1_on_widget_action_covers_paths() {
        let mut costs = std::collections::BTreeSet::new();
        for seed in 0..240 {
            let mut c = fresh();
            setup_widget_action(&mut c, seed);
            run_generated(&mut c, ee::s03::f_001b8c38__generated);
            costs.insert(c.cycles);
        }
        assert!(
            costs.len() >= 12,
            "fixture reaches only {} distinct paths: {costs:?}",
            costs.len()
        );
    }

    // ---------------------------------------------------------------- 0x1b92c8

    fn setup_edit_done(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x913_92c8, 8);
        gate(c, &mut r);
        objects(c, &mut r);
        c.bus.write32(SELF + 0x6e8, (seed / 10) % 2);
        table(
            c,
            0x0046_4940,
            8,
            &[0x001b_9308, 0x001b_931c, 0x001b_9334, 0x001b_9408],
            seed / 10,
        );
        c.g[A0][0] = SELF as u64;
        c.g[A2][0] = (0xe + seed % 10) as u64;
        c.pc = 0x001b_92c8;
    }

    #[test]
    fn c_fe_state_create_step1_on_edit_done_popup_event_matches() {
        check(
            ee::s03::f_001b92c8__generated,
            c_fe_state_create_step1_on_edit_done_popup_event,
            120,
            &setup_edit_done,
        );
    }

    #[test]
    fn c_fe_state_create_step1_on_edit_done_popup_event_covers_paths() {
        let mut costs = std::collections::BTreeSet::new();
        for seed in 0..120 {
            let mut c = fresh();
            setup_edit_done(&mut c, seed);
            run_generated(&mut c, ee::s03::f_001b92c8__generated);
            costs.insert(c.cycles);
        }
        assert!(
            costs.len() >= 5,
            "fixture reaches only {} distinct paths: {costs:?}",
            costs.len()
        );
    }
}
