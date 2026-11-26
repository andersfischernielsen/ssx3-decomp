//! Staging module oc914 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc914`.

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
        0x001b_aa88,
        c_fe_state_create_step2_on_child_event,
        "cFEStateCreateStep2_OnChildEvent",
    ),
    (
        0x001b_aed8,
        c_fe_state_create_step2_on_created_popup_event,
        "cFEStateCreateStep2_OnCreatedPopupEvent",
    ),
    (
        0x001b_c990,
        c_fe_state_create_duplicate_on_created_popup_event,
        "cFEStateCreateDuplicate_OnCreatedPopupEvent",
    ),
    (
        0x001b_d1b8,
        c_fe_state_lobby_menu_on_widget_action,
        "cFEStateLobbyMenu_OnWidgetAction",
    ),
];

/// `cFEStateCreateStep2_OnChildEvent` (0x001baa88): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_create_step2_on_child_event(c: &mut Cpu) {
    if c.pc != 0x001baa88 {
        return ee::s03::f_001baa88__generated(c);
    }
    guest(c, |c| 'l_1baa88: loop {
        'b_1bab1c: {
            'b_1bab10: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x80e));
                c.g[V0][0] = (c.g[V1][0] < 0x1fu64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A3][0] = c.g[A0][0];
                if t {
                    break 'b_1bab10;
                } else {
                    'b_1bab00: {
                        'b_1baaf0: {
                            'b_1baae0: {
                                'b_1baad0: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x460000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4a50));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x001baa88 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001baa88);
                                            }
                                            continue 'l_1baa88;
                                        }
                                        0x001baac0 => {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001baac8);
                                            c.g[A0][0] = c.g[A3][0];
                                            Callee {
                                                addr: 0x001bb158,
                                                f: ee::s03::cFEStateCreateStep2_OnSaveAccountAnswer,
                                            }
                                            .call(c, 0x001baac8)?;
                                            let t = true;
                                            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                            if t {
                                                break 'b_1bab1c;
                                            } else {
                                                break 'b_1baad0;
                                            }
                                        }
                                        0x001baad0 => {
                                            break 'b_1baad0;
                                        }
                                        0x001baae0 => {
                                            break 'b_1baae0;
                                        }
                                        0x001baaf0 => {
                                            break 'b_1baaf0;
                                        }
                                        0x001bab00 => {
                                            break 'b_1bab00;
                                        }
                                        0x001bab10 => {
                                            break 'b_1bab10;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x001baa88, 0x001bab24);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001baad8);
                                c.g[A0][0] = c.g[A3][0];
                                Callee {
                                    addr: 0x001bb0e8,
                                    f: ee::s03::cFEStateCreateStep2_OnParentEmailEntered,
                                }
                                .call(c, 0x001baad8)?;
                                let t = true;
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                if t {
                                    break 'b_1bab1c;
                                } else {
                                    break 'b_1baae0;
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001baae8);
                            c.g[A0][0] = c.g[A3][0];
                            Callee {
                                addr: 0x001bb098,
                                f: ee::s03::cFEStateCreateStep2_OnAgePopupEvent,
                            }
                            .call(c, 0x001baae8)?;
                            let t = true;
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                            if t {
                                break 'b_1bab1c;
                            } else {
                                break 'b_1baaf0;
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001baaf8);
                        c.g[A0][0] = c.g[A3][0];
                        Callee {
                            addr: 0x001bade8,
                            f: ee::s03::cFEStateCreateStep2_OnErrorPopupEvent,
                        }
                        .call(c, 0x001baaf8)?;
                        let t = true;
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        if t {
                            break 'b_1bab1c;
                        } else {
                            break 'b_1bab00;
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001bab08);
                    c.g[A0][0] = c.g[A3][0];
                    Callee {
                        addr: 0x001baed8,
                        f: ee::s03::cFEStateCreateStep2_OnCreatedPopupEvent,
                    }
                    .call(c, 0x001bab08)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    if t {
                        break 'b_1bab1c;
                    } else {
                        break 'b_1bab10;
                    }
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x001bab18);
            c.g[A0][0] = c.g[A3][0];
            Callee {
                addr: 0x001a97b8,
                f: ee::s03::NetFEState_OnChildEvent,
            }
            .call(c, 0x001bab18)?;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            break 'b_1bab1c;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateCreateStep2_OnCreatedPopupEvent` (0x001baed8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_create_step2_on_created_popup_event(c: &mut Cpu) {
    if c.pc != 0x001baed8 {
        return ee::s03::f_001baed8__generated(c);
    }
    guest(c, |c| {
        'b_1bb084: {
            'b_1bb080: {
                'b_1bb078: {
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
                        break 'b_1bb078;
                    } else {
                        'b_1baf44: {
                            'b_1baf2c: {
                                c.cycles += 7;
                                c.g[V0][0] = 0x460000;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4af0));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                let t = c.g[A0][0] as u32;
                                match t {
                                    0x001baf18 => {
                                        c.cycles += 5;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x6d8), 0u64);
                                        c.g[RA][0] = sx(0x001baf24);
                                        c.g[A0][0] = c.g[S2][0];
                                        Callee {
                                            addr: 0x001a97b8,
                                            f: ee::s03::NetFEState_OnChildEvent,
                                        }
                                        .call(c, 0x001baf24)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_1bb084;
                                        } else {
                                            break 'b_1baf2c;
                                        }
                                    }
                                    0x001baf2c => {
                                        break 'b_1baf2c;
                                    }
                                    0x001baf44 => {
                                        break 'b_1baf44;
                                    }
                                    0x001bb078 => {
                                        break 'b_1bb078;
                                    }
                                    _ => {
                                        c.jump_out(t, 0x001baed8, 0x001bb098);
                                        return Ok(());
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[V0][0] = 0x1;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001baf3c);
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x6d8), c.g[V0][0]);
                            Callee {
                                addr: 0x001a97b8,
                                f: ee::s03::NetFEState_OnChildEvent,
                            }
                            .call(c, 0x001baf3c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1bb084;
                            } else {
                                break 'b_1baf44;
                            }
                        }
                        'b_1bafe0: {
                            c.cycles += 5;
                            c.g[RA][0] = sx(0x001baf4c);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x001a97b8,
                                f: ee::s03::NetFEState_OnChildEvent,
                            }
                            .call(c, 0x001baf4c)?;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d8));
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_1bafe0;
                            } else {
                                c.cycles += 34;
                                c.g[RA][0] = sx(0x001baf60);
                                c.g[A0][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x001a9c50,
                                    f: ee::s03::NetFEState_GetPopup,
                                }
                                .call(c, 0x001baf60)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A0][0] = 0x460000;
                                c.g[V0][0] = 0x2;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2d80));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                                c.g[RA][0] = sx(0x001baf7c);
                                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                call_ptr(c, 0x00317670, 0x001baf7c)?;
                                c.g[V1][0] = 0x3;
                                c.g[A0][0] = 0x460000;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                c.g[RA][0] = sx(0x001baf94);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                call_ptr(c, 0x00317670, 0x001baf94)?;
                                c.g[A2][0] = c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                c.g[RA][0] = sx(0x001bafa4);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001c57c0,
                                    f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                }
                                .call(c, 0x001bafa4)?;
                                c.g[A0][0] = 0x460000;
                                c.g[RA][0] = sx(0x001bafb0);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                call_ptr(c, 0x00317670, 0x001bafb0)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A2][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001bafc0);
                                c.g[A1][0] = 0x1;
                                Callee {
                                    addr: 0x001c57c0,
                                    f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                }
                                .call(c, 0x001bafc0)?;
                                c.g[V0][0] = 0x81f;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x164), 0u64);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001bafd8);
                                c.g[A0][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x001a9bc0,
                                    f: ee::s03::NetFEState_ShowPopup,
                                }
                                .call(c, 0x001bafd8)?;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x6d8), 0u64);
                                if t {
                                    break 'b_1bb080;
                                } else {
                                    break 'b_1bafe0;
                                }
                            }
                        }
                        c.cycles += 38;
                        c.g[RA][0] = sx(0x001bafe8);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001a9c50,
                            f: ee::s03::NetFEState_GetPopup,
                        }
                        .call(c, 0x001bafe8)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A0][0] = 0x460000;
                        c.g[V0][0] = 0x1;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c70));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                        c.g[RA][0] = sx(0x001bb004);
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                        call_ptr(c, 0x00317670, 0x001bb004)?;
                        c.g[V1][0] = 0x3;
                        c.g[A0][0] = 0x460000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c88));
                        c.g[RA][0] = sx(0x001bb01c);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x001bb01c)?;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001bb02c);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x001c57c0,
                            f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                        }
                        .call(c, 0x001bb02c)?;
                        c.g[V0][0] = 0x80e;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001bb040);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                        Callee {
                            addr: 0x001a9bc0,
                            f: ee::s03::NetFEState_ShowPopup,
                        }
                        .call(c, 0x001bb040)?;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                        c.g[RA][0] = sx(0x001bb054);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
                        call_ptr(c, 0x003184e8, 0x001bb054)?;
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                        c.g[RA][0] = sx(0x001bb068);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4c));
                        call_ptr(c, 0x003184e8, 0x001bb068)?;
                        c.g[RA][0] = sx(0x001bb070);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        call_ptr(c, 0x0025c610, 0x001bb070)?;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x6d8), 0u64);
                        if t {
                            break 'b_1bb080;
                        } else {
                            break 'b_1bb078;
                        }
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x001bb080);
                c.g[A0][0] = c.g[S2][0];
                Callee {
                    addr: 0x001a97b8,
                    f: ee::s03::NetFEState_OnChildEvent,
                }
                .call(c, 0x001bb080)?;
                break 'b_1bb080;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1bb084;
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

/// `cFEStateCreateDuplicate_OnCreatedPopupEvent` (0x001bc990): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_create_duplicate_on_created_popup_event(c: &mut Cpu) {
    if c.pc != 0x001bc990 {
        return ee::s03::f_001bc990__generated(c);
    }
    guest(c, |c| {
        'b_1bcb40: {
            'b_1bcb3c: {
                'b_1bcb34: {
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
                        break 'b_1bcb34;
                    } else {
                        'b_1bc9fc: {
                            'b_1bc9e4: {
                                c.cycles += 7;
                                c.g[V0][0] = 0x460000;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4d80));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                let t = c.g[A0][0] as u32;
                                match t {
                                    0x001bc9d0 => {
                                        c.cycles += 5;
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x6d4), 0u64);
                                        c.g[RA][0] = sx(0x001bc9dc);
                                        c.g[A0][0] = c.g[S2][0];
                                        Callee {
                                            addr: 0x001a97b8,
                                            f: ee::s03::NetFEState_OnChildEvent,
                                        }
                                        .call(c, 0x001bc9dc)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_1bcb40;
                                        } else {
                                            break 'b_1bc9e4;
                                        }
                                    }
                                    0x001bc9e4 => {
                                        break 'b_1bc9e4;
                                    }
                                    0x001bc9fc => {
                                        break 'b_1bc9fc;
                                    }
                                    0x001bcb34 => {
                                        break 'b_1bcb34;
                                    }
                                    _ => {
                                        c.jump_out(t, 0x001bc990, 0x001bcb54);
                                        return Ok(());
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[V0][0] = 0x1;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001bc9f4);
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x6d4), c.g[V0][0]);
                            Callee {
                                addr: 0x001a97b8,
                                f: ee::s03::NetFEState_OnChildEvent,
                            }
                            .call(c, 0x001bc9f4)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1bcb40;
                            } else {
                                break 'b_1bc9fc;
                            }
                        }
                        'b_1bca9c: {
                            c.cycles += 5;
                            c.g[RA][0] = sx(0x001bca04);
                            c.g[A0][0] = c.g[S2][0];
                            Callee {
                                addr: 0x001a97b8,
                                f: ee::s03::NetFEState_OnChildEvent,
                            }
                            .call(c, 0x001bca04)?;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d4));
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_1bca9c;
                            } else {
                                c.cycles += 35;
                                c.g[RA][0] = sx(0x001bca18);
                                c.g[A0][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x001a9c50,
                                    f: ee::s03::NetFEState_GetPopup,
                                }
                                .call(c, 0x001bca18)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A0][0] = 0x460000;
                                c.g[V0][0] = 0x2;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2d80));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                                c.g[RA][0] = sx(0x001bca34);
                                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                call_ptr(c, 0x00317670, 0x001bca34)?;
                                c.g[V1][0] = 0x3;
                                c.g[A0][0] = 0x460000;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                c.g[RA][0] = sx(0x001bca4c);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                call_ptr(c, 0x00317670, 0x001bca4c)?;
                                c.g[A2][0] = c.g[V0][0];
                                c.g[A1][0] = 0u64;
                                c.g[RA][0] = sx(0x001bca5c);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001c57c0,
                                    f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                }
                                .call(c, 0x001bca5c)?;
                                c.g[A0][0] = 0x460000;
                                c.g[RA][0] = sx(0x001bca68);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                call_ptr(c, 0x00317670, 0x001bca68)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A2][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001bca78);
                                c.g[A1][0] = 0x1;
                                Callee {
                                    addr: 0x001c57c0,
                                    f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                }
                                .call(c, 0x001bca78)?;
                                c.g[V0][0] = 0x81f;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x168), 0u64);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x164), 0u64);
                                c.g[A1][0] = c.g[S0][0];
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                                c.g[RA][0] = sx(0x001bca94);
                                c.g[A0][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x001a9bc0,
                                    f: ee::s03::NetFEState_ShowPopup,
                                }
                                .call(c, 0x001bca94)?;
                                let t = true;
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x6d4), 0u64);
                                if t {
                                    break 'b_1bcb3c;
                                } else {
                                    break 'b_1bca9c;
                                }
                            }
                        }
                        c.cycles += 38;
                        c.g[RA][0] = sx(0x001bcaa4);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001a9c50,
                            f: ee::s03::NetFEState_GetPopup,
                        }
                        .call(c, 0x001bcaa4)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A0][0] = 0x460000;
                        c.g[V0][0] = 0x1;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c70));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                        c.g[RA][0] = sx(0x001bcac0);
                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                        call_ptr(c, 0x00317670, 0x001bcac0)?;
                        c.g[V1][0] = 0x3;
                        c.g[A0][0] = 0x460000;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c88));
                        c.g[RA][0] = sx(0x001bcad8);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x001bcad8)?;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001bcae8);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x001c57c0,
                            f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                        }
                        .call(c, 0x001bcae8)?;
                        c.g[V0][0] = 0x80e;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001bcafc);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                        Callee {
                            addr: 0x001a9bc0,
                            f: ee::s03::NetFEState_ShowPopup,
                        }
                        .call(c, 0x001bcafc)?;
                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d8));
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32));
                        c.g[RA][0] = sx(0x001bcb10);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
                        call_ptr(c, 0x003184e8, 0x001bcb10)?;
                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d8));
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                        c.g[RA][0] = sx(0x001bcb24);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4c));
                        call_ptr(c, 0x003184e8, 0x001bcb24)?;
                        c.g[RA][0] = sx(0x001bcb2c);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        call_ptr(c, 0x0025c610, 0x001bcb2c)?;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0x6d4), 0u64);
                        if t {
                            break 'b_1bcb3c;
                        } else {
                            break 'b_1bcb34;
                        }
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x001bcb3c);
                c.g[A0][0] = c.g[S2][0];
                Callee {
                    addr: 0x001a97b8,
                    f: ee::s03::NetFEState_OnChildEvent,
                }
                .call(c, 0x001bcb3c)?;
                break 'b_1bcb3c;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1bcb40;
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

/// `cFEStateLobbyMenu_OnWidgetAction` (0x001bd1b8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_lobby_menu_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001bd1b8 {
        return ee::s03::f_001bd1b8__generated(c);
    }
    guest(c, |c| 'l_1bd1b8: loop {
        'b_1bd414: {
            'b_1bd404: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S2][0] = c.g[A1][0];
                let t = c.g[V0][0] == 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_1bd404;
                } else {
                    'b_1bd3a8: {
                        'b_1bd398: {
                            'b_1bd378: {
                                'b_1bd330: {
                                    'b_1bd2f8: {
                                        'b_1bd22c: {
                                            c.cycles += 7;
                                            c.g[V0][0] = 0x460000;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x4f80));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                            let t = c.g[A0][0] as u32;
                                            match t {
                                                0x001bd1b8 => {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x001bd1b8);
                                                    }
                                                    continue 'l_1bd1b8;
                                                }
                                                0x001bd1fc => {
                                                    c.cycles += 3;
                                                    c.g[A0][0] = 0u64;
                                                    c.g[V1][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x708));
                                                    'l_1bd208: loop {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                        let t = c.g[S2][0] == c.g[V0][0];
                                                        c.g[S1][0] = sx((c.g[A0][0] as u32) << 2);
                                                        if t {
                                                            break 'b_1bd3a8;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[V0][0] =
                                                                ((c.g[A0][0] as i64) < 4i64) as u64;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0x4));
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x001bd208,
                                                                    );
                                                                }
                                                                continue 'l_1bd208;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x30),
                                                                );
                                                                if t {
                                                                    break 'b_1bd414;
                                                                } else {
                                                                    break 'b_1bd22c;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                0x001bd22c => {
                                                    break 'b_1bd22c;
                                                }
                                                0x001bd2f8 => {
                                                    break 'b_1bd2f8;
                                                }
                                                0x001bd330 => {
                                                    break 'b_1bd330;
                                                }
                                                0x001bd378 => {
                                                    break 'b_1bd378;
                                                }
                                                0x001bd398 => {
                                                    break 'b_1bd398;
                                                }
                                                0x001bd404 => {
                                                    break 'b_1bd404;
                                                }
                                                _ => {
                                                    c.jump_out(t, 0x001bd1b8, 0x001bd428);
                                                    return Ok(());
                                                }
                                            }
                                        }
                                        'b_1bd280: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x700));
                                            let t = c.g[S2][0] != c.g[V0][0];
                                            c.g[S1][0] = 0u64;
                                            if t {
                                                break 'b_1bd280;
                                            } else {
                                                c.cycles += 18;
                                                c.g[A1][0] = 0x460000;
                                                c.g[A0][0] = 0x6e8;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x4f60));
                                                c.g[A2][0] = 0u64;
                                                c.g[RA][0] = sx(0x001bd250);
                                                c.g[A3][0] = 0u64;
                                                call_ptr(c, 0x00317d70, 0x001bd250)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x001bd260);
                                                c.g[A2][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x001be7d8,
                                                    f: ee::s03::cFEStateNetCreateRoomPopup_ctor,
                                                }
                                                .call(c, 0x001bd260)?;
                                                c.g[V1][0] = 0x823;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.sw(
                                                    (c.g[V0][0] as u32).wrapping_add(0x18),
                                                    c.g[V1][0],
                                                );
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                                                c.g[A0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                                c.g[RA][0] = sx(0x001bd280);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                                call_ptr(c, 0x0039f290, 0x001bd280)?;
                                                break 'b_1bd280;
                                            }
                                        }
                                        'b_1bd2bc: {
                                            'b_1bd298: {
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x718));
                                                let t = c.g[S2][0] != c.g[V0][0];
                                                c.g[A1][0] = 0u64;
                                                if t {
                                                    break 'b_1bd298;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[S1][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x708)
                                                        );
                                                    if t {
                                                        break 'b_1bd2bc;
                                                    } else {
                                                        c.cycles += 1;
                                                        break 'b_1bd298;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            'l_1bd29c: loop {
                                                c.cycles += 3;
                                                c.g[V0][0] = ((c.g[A1][0] as i64) < 4i64) as u64;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = sx((c.g[A1][0] as u32) << 2);
                                                if t {
                                                    break 'b_1bd2bc;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[V0][0] as u32).wrapping_add(0x718)
                                                        );
                                                    if c.g[S2][0] != c.g[V1][0] {
                                                        c.g[A1][0] =
                                                            sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1));
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001bd29c);
                                                        }
                                                        continue 'l_1bd29c;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[S1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x708));
                                                        break 'b_1bd2bc;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        if c.g[S1][0] == 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            break 'b_1bd414;
                                        } else {
                                            c.cycles += 5;
                                            c.g[RA][0] = sx(0x001bd2cc);
                                            c.g[A0][0] = c.g[S1][0];
                                            call_ptr(c, 0x0039a738, 0x001bd2cc)?;
                                            c.g[V1][0] = 0xffffffffffffffff;
                                            if c.g[V0][0] == c.g[V1][0] {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                break 'b_1bd414;
                                            } else {
                                                c.cycles += 8;
                                                c.g[RA][0] = sx(0x001bd2e0);
                                                c.g[A0][0] = c.g[S1][0];
                                                call_ptr(c, 0x0039a738, 0x001bd2e0)?;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x001bd2f0);
                                                c.g[A2][0] = 0u64;
                                                Callee {
                                                    addr: 0x001bd460,
                                                    f: ee::s03::cFEStateLobbyMenu_EnterRoom,
                                                }
                                                .call(c, 0x001bd2f0)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_1bd414;
                                                } else {
                                                    break 'b_1bd2f8;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 14;
                                    c.g[A1][0] = 0x460000;
                                    c.g[A0][0] = 0x6d0;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd8));
                                    c.g[A2][0] = 0u64;
                                    c.g[RA][0] = sx(0x001bd310);
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x001bd310)?;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x001bd31c);
                                    c.g[A0][0] = c.g[V0][0];
                                    Callee {
                                        addr: 0x001bfbb8,
                                        f: ee::s03::cFEStateOnlineMainMenu_ctor,
                                    }
                                    .call(c, 0x001bd31c)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x001bd328);
                                    c.g[A1][0] = c.g[V0][0];
                                    Callee {
                                        addr: 0x001a8a40,
                                        f: ee::s03::NetFEState_GotoNextState,
                                    }
                                    .call(c, 0x001bd328)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_1bd414;
                                    } else {
                                        break 'b_1bd330;
                                    }
                                }
                                'b_1bd364: {
                                    c.cycles += 8;
                                    c.g[V1][0] = 0x1;
                                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d4));
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x760), c.g[V1][0]);
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa0));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x764), c.g[V1][0]);
                                    if t {
                                        break 'b_1bd364;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x708));
                                        c.g[A0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x319));
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x768), c.g[A0][0]);
                                        break 'b_1bd364;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001bd370);
                                c.g[A1][0] = c.g[S2][0];
                                Callee {
                                    addr: 0x001a8918,
                                    f: ee::s03::NetFEState_OnWidgetAction,
                                }
                                .call(c, 0x001bd370)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1bd414;
                                } else {
                                    break 'b_1bd378;
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = c.g[V0][0] & 0x6;
                            if c.g[V0][0] != 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                break 'b_1bd414;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001bd390);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001a9b58,
                                    f: ee::s03::NetFEState_OpenMessenger,
                                }
                                .call(c, 0x001bd390)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1bd414;
                                } else {
                                    break 'b_1bd398;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001bd3a0);
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001bd428,
                            f: ee::s03::cFEStateLobbyMenu_OnKeyboardDone,
                        }
                        .call(c, 0x001bd3a0)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_1bd414;
                        } else {
                            break 'b_1bd3a8;
                        }
                    }
                    c.cycles += 23;
                    c.g[V1][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x319));
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    c.g[A0][0] = c.g[S2][0];
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x750), c.g[V1][0]);
                    c.g[RA][0] = sx(0x001bd3c0);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x778), 0u64);
                    call_ptr(c, 0x0039a738, 0x001bd3c0)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001bd3d0);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x774), c.g[V0][0]);
                    call_ptr(c, 0x0025f180, 0x001bd3d0)?;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x740));
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001bd3f0);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001bd3f0)?;
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x774));
                    c.g[RA][0] = sx(0x001bd3fc);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    call_ptr(c, 0x0025d770, 0x001bd3fc)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1bd414;
                    } else {
                        break 'b_1bd404;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x001bd410);
            c.g[A1][0] = c.g[S2][0];
            Callee {
                addr: 0x001a8918,
                f: ee::s03::NetFEState_OnWidgetAction,
            }
            .call(c, 0x001bd410)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1bd414;
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
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of the oc914 fakes: they act only while it holds the magic.
    const OC914_CTRL: u32 = DATA + 0x4f_0100;
    const OC914_MAGIC: u32 = 0x0914_5eed;
    /// Nonzero: the fake of `0x0039a738` returns -1.
    const OC914_MODE: u32 = OC914_CTRL + 4;
    const OC914_SELF: u32 = DATA + 0x100;

    /// Like `fakes::fake_call`; the fake of `0x0039a738` returns -1 when the
    /// fixture asks for it.
    fn oc914_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mode = c.bus.read32(OC914_MODE);
        fakes::fake_call(c, addr);
        if addr == 0x0039_a738 && c.pc == ra && mode != 0 {
            c.g[V0][0] = u64::MAX;
        }
    }

    macro_rules! oc914_fakes {
        ($(($fake:ident, $slot:expr, $gen:path, $addr:expr)),* $(,)?) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC914_CTRL) != OC914_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                    }
                    oc914_fake_call(c, $addr);
                }
            )*
            fn oc914_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    oc914_fakes!(
        (
            oc914_f_1a97b8,
            ee::s03::OVERRIDE_001A97B8,
            ee::s03::f_001a97b8__generated,
            0x001a_97b8
        ),
        (
            oc914_f_1a9c50,
            ee::s03::OVERRIDE_001A9C50,
            ee::s03::f_001a9c50__generated,
            0x001a_9c50
        ),
        (
            oc914_f_1a9bc0,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a_9bc0
        ),
        (
            oc914_f_1c57c0,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c_57c0
        ),
        (
            oc914_f_1bb158,
            ee::s03::OVERRIDE_001BB158,
            ee::s03::f_001bb158__generated,
            0x001b_b158
        ),
        (
            oc914_f_1bb0e8,
            ee::s03::OVERRIDE_001BB0E8,
            ee::s03::f_001bb0e8__generated,
            0x001b_b0e8
        ),
        (
            oc914_f_1bb098,
            ee::s03::OVERRIDE_001BB098,
            ee::s03::f_001bb098__generated,
            0x001b_b098
        ),
        (
            oc914_f_1bade8,
            ee::s03::OVERRIDE_001BADE8,
            ee::s03::f_001bade8__generated,
            0x001b_ade8
        ),
        (
            oc914_f_1baed8,
            ee::s03::OVERRIDE_001BAED8,
            ee::s03::f_001baed8__generated,
            0x001b_aed8
        ),
        (
            oc914_f_1be7d8,
            ee::s03::OVERRIDE_001BE7D8,
            ee::s03::f_001be7d8__generated,
            0x001b_e7d8
        ),
        (
            oc914_f_1bd460,
            ee::s03::OVERRIDE_001BD460,
            ee::s03::f_001bd460__generated,
            0x001b_d460
        ),
        (
            oc914_f_1bd428,
            ee::s03::OVERRIDE_001BD428,
            ee::s03::f_001bd428__generated,
            0x001b_d428
        ),
        (
            oc914_f_1bfbb8,
            ee::s03::OVERRIDE_001BFBB8,
            ee::s03::f_001bfbb8__generated,
            0x001b_fbb8
        ),
        (
            oc914_f_1a8a40,
            ee::s03::OVERRIDE_001A8A40,
            ee::s03::f_001a8a40__generated,
            0x001a_8a40
        ),
        (
            oc914_f_1a8918,
            ee::s03::OVERRIDE_001A8918,
            ee::s03::f_001a8918__generated,
            0x001a_8918
        ),
        (
            oc914_f_1a9b58,
            ee::s03::OVERRIDE_001A9B58,
            ee::s03::f_001a9b58__generated,
            0x001a_9b58
        ),
        (
            oc914_f_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc914_f_3184e8,
            ee::s10::OVERRIDE_003184E8,
            ee::s10::f_003184e8__generated,
            0x0031_84e8
        ),
        (
            oc914_f_317d70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc914_f_25c610,
            ee::s06::OVERRIDE_0025C610,
            ee::s06::f_0025c610__generated,
            0x0025_c610
        ),
        (
            oc914_f_25f180,
            ee::s06::OVERRIDE_0025F180,
            ee::s06::f_0025f180__generated,
            0x0025_f180
        ),
        (
            oc914_f_25d770,
            ee::s06::OVERRIDE_0025D770,
            ee::s06::f_0025d770__generated,
            0x0025_d770
        ),
        (
            oc914_f_39a738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738
        ),
    );

    /// Fixture common to the four handlers: the machine with the runtime, the
    /// gate on, and the jump table `base` (`n` words) filled with `cases`
    /// rotated by `rot`.
    fn oc914_setup(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        base: u32,
        n: u32,
        cases: &[u32],
        rot: u32,
    ) -> Rng {
        oc914_install_fakes();
        let r = machine_rt(c, seed, salt, 14);
        c.bus.write32(OC914_CTRL, OC914_MAGIC);
        c.bus.write32(OC914_MODE, (seed >> 3) & 1);
        for i in 0..n {
            c.bus
                .write32(base + 4 * i, cases[((i + rot) as usize) % cases.len()]);
        }
        r
    }

    /// `cFEStateCreateStep2_OnChildEvent`: the account-creation step 2 screen
    /// dispatches its child popup's event (`child + 0x18`, 0x80e + n) through
    /// a 0x1f-entry jump table to one of five handlers, or to the base
    /// `NetFEState_OnChildEvent`.
    #[test]
    fn c_fe_state_create_step2_on_child_event_matches() {
        check(
            ee::s03::f_001baa88__generated,
            c_fe_state_create_step2_on_child_event,
            240,
            &|c, seed| {
                let _r = oc914_setup(
                    c,
                    seed,
                    0x001b_aa88,
                    0x0046_4a50,
                    0x1f,
                    &[
                        0x001b_aac0,
                        0x001b_aad0,
                        0x001b_aae0,
                        0x001b_aaf0,
                        0x001b_ab00,
                        0x001b_ab10,
                    ],
                    seed / 36,
                );
                let child = DATA + 0x3000;
                // 36 events: 31 in the table, 5 beyond it.
                c.bus.write32(child + 0x18, 0x80e + seed % 36);
                c.g[A0][0] = sx(OC914_SELF);
                c.g[A1][0] = sx(child);
                c.pc = 0x001b_aa88;
            },
        );
    }

    /// The popup-created handlers of step 2 (`+0x6d8` flag) and of the
    /// duplicate screen (`+0x6d4` flag): events 0xf..0x16 jump through an
    /// 8-entry table to "clear the flag", "set the flag", the error/confirm
    /// popup builders or the base handler.
    fn oc914_popup_fixture(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        table: u32,
        cases: &[u32],
        flag: u32,
    ) {
        let _r = oc914_setup(c, seed, salt, table, 8, cases, seed / 11);
        c.bus.write32(OC914_SELF + flag, (seed >> 1) & 1);
        // The pointers behind `+0x6d0` / `+0x6d8` and the global at gp-0xc8.
        c.bus.write32(OC914_SELF + 0x6d0, DATA + 0x1000);
        if flag == 0x6d4 {
            c.bus.write32(OC914_SELF + 0x6d8, DATA + 0x1000);
        }
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x1100);
        // 11 events: 0xf..0x16 in the table, three outside.
        c.g[A0][0] = sx(OC914_SELF);
        c.g[A2][0] = (0xf + seed % 11) as u64;
        c.pc = 0;
    }

    #[test]
    fn c_fe_state_create_step2_on_created_popup_event_matches() {
        check(
            ee::s03::f_001baed8__generated,
            c_fe_state_create_step2_on_created_popup_event,
            220,
            &|c, seed| {
                oc914_popup_fixture(
                    c,
                    seed,
                    0x001b_aed8,
                    0x0046_4af0,
                    &[0x001b_af18, 0x001b_af2c, 0x001b_af44, 0x001b_b078],
                    0x6d8,
                );
                c.pc = 0x001b_aed8;
            },
        );
    }

    #[test]
    fn c_fe_state_create_duplicate_on_created_popup_event_matches() {
        check(
            ee::s03::f_001bc990__generated,
            c_fe_state_create_duplicate_on_created_popup_event,
            220,
            &|c, seed| {
                oc914_popup_fixture(
                    c,
                    seed,
                    0x001b_c990,
                    0x0046_4d80,
                    &[0x001b_c9d0, 0x001b_c9e4, 0x001b_c9fc, 0x001b_cb34],
                    0x6d4,
                );
                c.pc = 0x001b_c990;
            },
        );
    }

    /// `cFEStateLobbyMenu_OnWidgetAction`: the lobby menu's widget handler;
    /// events 5..13 jump through a 9-entry table (item pick by pointer
    /// compare, create-room popup, join, back to the online menu, selected
    /// player, messenger, keyboard done) or fall to the base handler.
    #[test]
    fn c_fe_state_lobby_menu_on_widget_action_matches() {
        check(
            ee::s03::f_001bd1b8__generated,
            c_fe_state_lobby_menu_on_widget_action,
            600,
            &|c, seed| {
                let mut r = oc914_setup(
                    c,
                    seed,
                    0x001b_d1b8,
                    0x0046_4f80,
                    9,
                    &[
                        0x001b_d1fc,
                        0x001b_d22c,
                        0x001b_d2f8,
                        0x001b_d330,
                        0x001b_d378,
                        0x001b_d398,
                        0x001b_d404,
                    ],
                    seed / 12,
                );
                let this = OC914_SELF;
                c.bus.write32(this + 0x10, DATA + 0x1000);
                c.bus.write32(this + 0x40, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100 + 0xd0, DATA + 0x1200);
                c.bus.write32(DATA + 0x1200 + 0x10, DATA + 0x1300);
                c.bus
                    .write32(this + 0x1c, if (seed >> 2) & 1 != 0 { 0x2 } else { 0 });
                // Screen item `+0x6d4`: its `+0xa0` block holds the selected index.
                c.bus.write32(this + 0x6d4, DATA + 0x1400);
                c.bus.write32(DATA + 0x1400 + 0xa0, DATA + 0x1500);
                c.bus.write32(DATA + 0x1500 + 0x18, (seed / 5) % 6);
                // Room list items (+0x708..), create button (+0x700), join
                // buttons (+0x718..), per-item widgets (+0x740..).
                c.bus.write32(this + 0x700, DATA + 0x1800);
                for i in 0..4u32 {
                    c.bus.write32(
                        this + 0x708 + 4 * i,
                        if (seed >> (4 + i)) & 1 != 0 {
                            DATA + 0x2000 + 0x100 * i
                        } else {
                            0
                        },
                    );
                    c.bus
                        .write32(this + 0x718 + 4 * i, DATA + 0x2400 + 0x100 * i);
                    let p = DATA + 0x2800 + 0x40 * i;
                    c.bus.write32(this + 0x740 + 4 * i, p);
                    let q = DATA + 0x2c00 + 0x80 * i;
                    c.bus.write32(p + 8, q);
                    c.bus.write32(q + 0x4c, 0x001a_9b58);
                }
                // The widget that acted: one of the lists, or a stranger.
                let k = r.next_u32() % 4;
                let widget = match r.next_u32() % 6 {
                    0 => DATA + 0x2000 + 0x100 * k,
                    1 => DATA + 0x1800,
                    2 => DATA + 0x2400 + 0x100 * k,
                    3 => DATA + 0x2000 + 0x100 * k,
                    _ => DATA + 0x3000,
                };
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(widget);
                // 14 events: 9 in the table, 5 outside.
                c.g[A2][0] = (5 + r.next_u32() % 14) as u64;
                c.pc = 0x001b_d1b8;
            },
        );
    }
}
