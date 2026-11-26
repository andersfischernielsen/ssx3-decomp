//! Staging module oc850 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc850`.

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
        0x0019_38f8,
        c_fe_state_bragging_rights_on_widget_action,
        "cFEStateBraggingRights_OnWidgetAction",
    ),
    (
        0x0019_3c00,
        c_fe_state_bragging_rights_v25,
        "cFEStateBraggingRights_v25",
    ),
    (
        0x0019_3cf8,
        c_fe_state_bragging_rights_on_widget_notify,
        "cFEStateBraggingRights_OnWidgetNotify",
    ),
    (
        0x0019_3de0,
        c_fe_state_bragging_rights_dispatch_row_action,
        "cFEStateBraggingRights_DispatchRowAction",
    ),
    (
        0x0019_3f38,
        c_fe_state_bragging_rights_get_row_default,
        "cFEStateBraggingRights_GetRowDefault",
    ),
];

/// `int cFEStateBraggingRights_OnWidgetAction(self, widget, event)` (0x1938f8): the
/// bragging-rights screen's widget events (1..9, jump table at 0x45fdc0). 1/2
/// recolour the widget's menu rows, 5 (a row confirmed) marks the menu confirmed
/// and, once both are, asks the engine object for the next state and switches to
/// it, 6 (cancel) switches state (or opens the "cancel challenge" popup online),
/// 7 forwards to `NetFEState_OnWidgetAction`, 9 stores the selected value in the
/// BE rules; the other events do nothing. `gp-0x1680` is the online flag.
/// Resumes at any point other than the entry through the generated body.
pub fn c_fe_state_bragging_rights_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001938f8 {
        return ee::s02::f_001938f8__entries(c);
    }
    guest(c, |c| 'l_1938f8: loop {
        'b_193be0: {
            'b_193bdc: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S3][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S4][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_193bdc;
                } else {
                    'b_193b4c: {
                        'b_193b28: {
                            'b_193b08: {
                                'b_193a1c: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x460000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x240));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x001938f8 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001938f8);
                                            }
                                            continue 'l_1938f8;
                                        }
                                        0x00193948 => {
                                            c.cycles += 4;
                                            c.g[S0][0] =
                                                sx((c.g[S3][0] as u32).wrapping_add(0x6d0));
                                            c.g[S1][0] = 0u64;
                                            c.g[S5][0] = 0xffffffffffffffff;
                                            c.g[S2][0] = c.g[S0][0];
                                            'l_193958: loop {
                                                'b_1939bc: {
                                                    'b_1939a4: {
                                                        c.cycles += 5;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x00193964);
                                                        c.g[A1][0] = c.g[S4][0];
                                                        Callee { addr: 0x00192918, f: ee::s02::cFEStateBraggingRights_FindRowIndex }.call(c, 0x00193964)?;
                                                        let t = c.g[V0][0] == c.g[S5][0];
                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                            .wrapping_sub(0x1680));
                                                        if t {
                                                            break 'b_1939a4;
                                                        } else {
                                                            'b_19398c: {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                                let t = c.g[V0][0] != 0u64;
                                                                c.g[A0][0] = c.g[S2][0];
                                                                if t {
                                                                    break 'b_19398c;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = c.g[S1][0] != 0u64;

                                                                    if t {
                                                                        break 'b_19398c;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[RA][0] = sx(0x00193988);
                                                                        c.g[A0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x2dc));
                                                                        call_ptr(
                                                                            c, 0x00265688,
                                                                            0x00193988,
                                                                        )?;
                                                                        c.g[A0][0] = c.g[S2][0];
                                                                        break 'b_19398c;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 6;
                                                            c.g[RA][0] = sx(0x00193994);
                                                            c.g[A1][0] = 0x1;
                                                            Callee { addr: 0x00192740, f: ee::s02::cFEStateBraggingRights_SetRowsVisible }.call(c, 0x00193994)?;
                                                            c.g[RA][0] = sx(0x0019399c);
                                                            c.g[A0][0] = c.g[S3][0];
                                                            Callee { addr: 0x00194080, f: ee::s02::cFEStateBraggingRights_StartWaitAnim }.call(c, 0x0019399c)?;
                                                            let t = true;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x1680));
                                                            if t {
                                                                break 'b_1939bc;
                                                            } else {
                                                                break 'b_1939a4;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 5;
                                                    c.g[S1][0] =
                                                        sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                                    c.g[S2][0] =
                                                        sx((c.g[S2][0] as u32).wrapping_add(0x34));
                                                    c.g[V0][0] =
                                                        ((c.g[S1][0] as i64) < 2i64) as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[S0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x34));
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x00193958);
                                                        }
                                                        continue 'l_193958;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                            .wrapping_sub(0x1680));
                                                        break 'b_1939bc;
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                                if c.g[V0][0] == 0u64 {
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x60),
                                                    );
                                                    break 'b_193be0;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x6f8)
                                                        );
                                                    if c.g[V0][0] == 0u64 {
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x60),
                                                        );
                                                        break 'b_193be0;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x72c));
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A1][0] = c.g[S3][0];
                                                        if t {
                                                            break 'b_193bdc;
                                                        } else {
                                                            c.cycles += 10;
                                                            c.g[V0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A2][0] = c.lw((c.g[S4][0] as u32)
                                                                .wrapping_add(0x18));
                                                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[A0][0] = c.lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x24));
                                                            let t = c.g[V0][0] as u32;
                                                            c.g[RA][0] = sx(0x00193a00);
                                                            c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x00193a00)?;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A1][0] = c.g[V0][0];
                                                            if t {
                                                                break 'b_193bdc;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x10));
                                                                c.g[RA][0] = sx(0x00193a14);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x18));
                                                                call_ptr(
                                                                    c, 0x0039f400, 0x00193a14,
                                                                )?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x60),
                                                                );
                                                                if t {
                                                                    break 'b_193be0;
                                                                } else {
                                                                    break 'b_193a1c;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        0x00193a1c => {
                                            break 'b_193a1c;
                                        }
                                        0x00193b08 => {
                                            break 'b_193b08;
                                        }
                                        0x00193b28 => {
                                            break 'b_193b28;
                                        }
                                        0x00193b4c => {
                                            break 'b_193b4c;
                                        }
                                        0x00193bdc => {
                                            break 'b_193bdc;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x001938f8, 0x00193c00);
                                            return Ok(());
                                        }
                                    }
                                }
                                'b_193ab8: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                                        break 'b_193ab8;
                                    } else {
                                        c.cycles += 35;
                                        c.g[RA][0] = sx(0x00193a34);
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x001a9c50, 0x00193a34)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A0][0] = 0x460000;
                                        c.g[V0][0] = 0x2;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x268));
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x00193a50);
                                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                        call_ptr(c, 0x00317670, 0x00193a50)?;
                                        c.g[V1][0] = 0x3;
                                        c.g[A0][0] = 0x460000;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                        c.g[RA][0] = sx(0x00193a68);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                        call_ptr(c, 0x00317670, 0x00193a68)?;
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0u64;
                                        c.g[RA][0] = sx(0x00193a78);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x001c57c0, 0x00193a78)?;
                                        c.g[A0][0] = 0x460000;
                                        c.g[RA][0] = sx(0x00193a84);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                        call_ptr(c, 0x00317670, 0x00193a84)?;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x00193a94);
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x001c57c0, 0x00193a94)?;
                                        c.g[V0][0] = 0x838;
                                        c.g[V1][0] = 0x1;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                                        c.g[A0][0] = c.g[S3][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x168), c.g[V1][0]);
                                        c.g[RA][0] = sx(0x00193ab0);
                                        c.g[A1][0] = c.g[S0][0];
                                        call_ptr(c, 0x001a9bc0, 0x00193ab0)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                        if t {
                                            break 'b_193be0;
                                        } else {
                                            break 'b_193ab8;
                                        }
                                    }
                                }
                                'b_193aec: {
                                    c.cycles += 10;
                                    c.g[A1][0] = c.g[S3][0];
                                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18));
                                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00193ad8);
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00193ad8)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.g[V0][0];
                                    if t {
                                        break 'b_193aec;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = sx(0x00193aec);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                        call_ptr(c, 0x0039f400, 0x00193aec)?;
                                        break 'b_193aec;
                                    }
                                }
                                c.cycles += 7;
                                c.g[A0][0] = 0x530000;
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8b8));
                                c.g[RA][0] = sx(0x00193b00);
                                c.g[A2][0] = 0x20;
                                call_ptr(c, 0x003e6448, 0x00193b00)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                if t {
                                    break 'b_193be0;
                                } else {
                                    break 'b_193b08;
                                }
                            }
                            c.cycles += 8;
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18));
                            c.g[A0][0] = 0x34;
                            c.mult(0, 4, 16, 4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d0));
                            c.g[RA][0] = sx(0x00193b20);
                            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            Callee {
                                addr: 0x001927b0,
                                f: ee::s02::cFEStateBraggingRights_UpdateRowColors,
                            }
                            .call(c, 0x00193b20)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            if t {
                                break 'b_193be0;
                            } else {
                                break 'b_193b28;
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                        let t = c.g[V0][0] != 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        if t {
                            break 'b_193be0;
                        } else {
                            c.cycles += 5;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[RA][0] = sx(0x00193b44);
                            c.g[A1][0] = c.g[S4][0];
                            call_ptr(c, 0x001a8918, 0x00193b44)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            if t {
                                break 'b_193be0;
                            } else {
                                break 'b_193b4c;
                            }
                        }
                    }
                    'b_193b90: {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                        c.g[V0][0] = c.g[V0][0] ^ 0x1;
                        if c.g[V0][0] != 0u64 {
                            c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x18));
                            break 'b_193b90;
                        } else {
                            c.cycles += 13;
                            c.g[A1][0] = c.g[S4][0];
                            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6d0));
                            c.g[RA][0] = sx(0x00193b6c);
                            c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                            Callee {
                                addr: 0x00192918,
                                f: ee::s02::cFEStateBraggingRights_FindRowIndex,
                            }
                            .call(c, 0x00193b6c)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00193b78);
                            c.g[A0][0] = c.g[S4][0];
                            call_ptr(c, 0x0039a738, 0x00193b78)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00193b88);
                            c.g[A2][0] = c.g[V0][0];
                            call_ptr(c, 0x00265568, 0x00193b88)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                            if t {
                                break 'b_193be0;
                            } else {
                                break 'b_193b90;
                            }
                        }
                    }
                    c.cycles += 12;
                    c.g[A0][0] = 0x34;
                    c.g[A1][0] = c.g[S4][0];
                    c.mult(0, 4, 16, 4);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6d0));
                    c.g[RA][0] = sx(0x00193ba8);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Callee {
                        addr: 0x00192918,
                        f: ee::s02::cFEStateBraggingRights_FindRowIndex,
                    }
                    .call(c, 0x00193ba8)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x738));
                    c.g[A0][0] = 0xffffffffffffffff;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S1][0] = c.lb(c.g[V1][0] as u32);
                    if c.g[S1][0] == c.g[A0][0] {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        break 'b_193be0;
                    } else {
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x00193bc8);
                        c.g[A0][0] = c.g[S4][0];
                        call_ptr(c, 0x0039a738, 0x00193bc8)?;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x00193bdc);
                        c.g[A3][0] = c.g[V0][0];
                        Callee {
                            addr: 0x00193de0,
                            f: ee::s02::cFEStateBraggingRights_DispatchRowAction,
                        }
                        .call(c, 0x00193bdc)?;
                        break 'b_193bdc;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_193be0;
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateBraggingRights_v25(self, popup)` (0x193c00): fill the help
/// popup of the bragging-rights screen: four options, the screen name hash and
/// two description lines, then the four option button texts and their button ids
/// (hash of each string from the game data at 0x45fde8..0x45fe98).
pub fn c_fe_state_bragging_rights_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 61;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A1][0] = 0x4;
        c.g[RA][0] = sx(0x00193c1c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001da528, 0x00193c1c)?;
        c.g[A0][0] = 0x460000;
        c.g[RA][0] = sx(0x00193c28);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x218));
        call_ptr(c, 0x00317670, 0x00193c28)?;
        c.g[A0][0] = 0x460000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x54), c.g[V0][0]);
        c.g[RA][0] = sx(0x00193c38);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x200));
        call_ptr(c, 0x00317670, 0x00193c38)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00193c48);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x001da530, 0x00193c48)?;
        c.g[A0][0] = 0x460000;
        c.g[RA][0] = sx(0x00193c54);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1e0));
        call_ptr(c, 0x00317670, 0x00193c54)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x00193c64);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x001da530, 0x00193c64)?;
        c.g[A0][0] = 0x460000;
        c.g[RA][0] = sx(0x00193c70);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c0));
        call_ptr(c, 0x00317670, 0x00193c70)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x00193c84);
        c.g[A3][0] = 0x4;
        call_ptr(c, 0x001da510, 0x00193c84)?;
        c.g[A0][0] = 0x460000;
        c.g[RA][0] = sx(0x00193c90);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1a0));
        call_ptr(c, 0x00317670, 0x00193c90)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        c.g[RA][0] = sx(0x00193ca4);
        c.g[A3][0] = 0x3;
        call_ptr(c, 0x001da510, 0x00193ca4)?;
        c.g[A0][0] = 0x460000;
        c.g[RA][0] = sx(0x00193cb0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x180));
        call_ptr(c, 0x00317670, 0x00193cb0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0x2;
        c.g[RA][0] = sx(0x00193cc4);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x001da510, 0x00193cc4)?;
        c.g[A0][0] = 0x460000;
        c.g[RA][0] = sx(0x00193cd0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x168));
        call_ptr(c, 0x00317670, 0x00193cd0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0x3;
        c.g[RA][0] = sx(0x00193ce4);
        c.g[A3][0] = 0x1;
        call_ptr(c, 0x001da510, 0x00193ce4)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `int cFEStateBraggingRights_OnWidgetNotify(self, ctrl, msg)` (0x193cf8): the
/// notify messages 5..9 (jump table at 0x45feb0) of the screen's two menus
/// (`self+0x6d0`, `self+0x704`, found by their control pointer): 5 recolours the
/// menu (and sends the highlight message when online), 6 and 7 answer whether the
/// screen may leave, 8/9 and anything else answer 0x100/0x101.
/// Resumes at any point other than the entry through the generated body.
pub fn c_fe_state_bragging_rights_on_widget_notify(c: &mut Cpu) {
    if c.pc != 0x00193cf8 {
        return ee::s02::f_00193cf8__entries(c);
    }
    guest(c, |c| 'l_193cf8: loop {
        'b_193d34: {
            'b_193d24: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[A3][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x6d0));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x6d0));
                let t = c.g[V0][0] != c.g[A1][0];
                c.g[T0][0] = 0u64;
                if t {
                    break 'b_193d24;
                } else {
                    c.cycles += 3;
                    c.g[S0][0] = c.g[A0][0];
                    let t = true;
                    c.g[T0][0] = 0x1;
                    if t {
                        break 'b_193d34;
                    } else {
                        break 'b_193d24;
                    }
                }
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x704));
            c.g[S0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x704));
            c.g[V0][0] = c.g[V0][0] ^ c.g[A1][0];
            if c.g[V0][0] != 0 {
                c.g[S0][0] = 0u64;
            }
            break 'b_193d34;
        }
        'b_193dcc: {
            'b_193dc8: {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x460000;
                if t {
                    break 'b_193dc8;
                } else {
                    'b_193db8: {
                        'b_193d98: {
                            'b_193d64: {
                                c.cycles += 6;
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x150));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                let t = c.g[A0][0] as u32;
                                match t {
                                    0x00193cf8 => {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00193cf8);
                                        }
                                        continue 'l_193cf8;
                                    }
                                    0x00193d5c => {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0x100;
                                        if t {
                                            break 'b_193dcc;
                                        } else {
                                            break 'b_193d64;
                                        }
                                    }
                                    0x00193d64 => {
                                        break 'b_193d64;
                                    }
                                    0x00193d98 => {
                                        break 'b_193d98;
                                    }
                                    0x00193db8 => {
                                        break 'b_193db8;
                                    }
                                    _ => {
                                        c.jump_out(t, 0x00193cf8, 0x00193ddc);
                                        return Ok(());
                                    }
                                }
                            }
                            'b_193d88: {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_193d88;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[T0][0] == 0u64;

                                    if t {
                                        break 'b_193d88;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A1][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x95));
                                        c.g[RA][0] = sx(0x00193d88);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                                        call_ptr(c, 0x00265458, 0x00193d88)?;
                                        break 'b_193d88;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00193d90);
                            c.g[A0][0] = c.g[S0][0];
                            Callee {
                                addr: 0x001927b0,
                                f: ee::s02::cFEStateBraggingRights_UpdateRowColors,
                            }
                            .call(c, 0x00193d90)?;
                            let t = true;
                            c.g[V0][0] = 0x101;
                            if t {
                                break 'b_193dcc;
                            } else {
                                break 'b_193d98;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x6f8));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_193dcc;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x72c));
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = 0x101;
                            if t {
                                break 'b_193dcc;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_193dcc;
                                } else {
                                    break 'b_193db8;
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1680));
                    c.g[V1][0] = c.g[V1][0] ^ 0x1;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_193dcc;
                    } else {
                        break 'b_193dc8;
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x101;
            break 'b_193dcc;
        }
        c.cycles += 4;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateBraggingRights_DispatchRowAction(self, player, kind, value)`
/// (0x193de0): store a menu row's value (kinds 0..7, jump table at 0x45fed0) in
/// the BE rules entry (`0x5308b8 + 12 * index`) as a bit or word, for `player` or
/// the other player (index 1 when `player` is 0). Other kinds do nothing.
/// Resumes at any point other than the entry through the generated body.
pub fn c_fe_state_bragging_rights_dispatch_row_action(c: &mut Cpu) {
    if c.pc != 0x00193de0 {
        return ee::s02::f_00193de0__entries(c);
    }
    guest(c, |c| {
        'b_193f30: {
            c.cycles += 3;
            c.g[V0][0] = (c.g[A2][0] < 0x8u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[T0][0] = c.g[A1][0];
            if t {
                break 'b_193f30;
            } else {
                'b_193e48: {
                    'b_193f14: {
                        'b_193ef4: {
                            'b_193ed4: {
                                'b_193eb4: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x460000;
                                    c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x130));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x00193e08 => {
                                            c.cycles += 8;
                                            c.g[A0][0] = (c.g[T0][0] < 0x1u64) as u64;
                                            c.g[V1][0] = 0xc;
                                            c.mult(0, 4, 4, 3);
                                            c.g[V0][0] = 0x530000;
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            let t = c.g[RA][0] as u32;
                                            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                                            c.pc = t;
                                            return Ok(());
                                        }
                                        0x00193e28 => {
                                            c.cycles += 8;
                                            c.g[V0][0] = 0xc;
                                            c.g[A1][0] = (c.g[T0][0] < 0x1u64) as u64;
                                            c.mult(0, 5, 5, 2);
                                            c.g[V1][0] = 0x530000;
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                                            c.g[A0][0] = c.g[A3][0] & 0x1;
                                            c.g[A2][0] = 0xfffffffffffffff7;
                                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                            break 'b_193e48;
                                        }
                                        0x00193e60 => {
                                            c.cycles += 13;
                                            c.g[V0][0] = 0xc;
                                            c.g[A0][0] = (c.g[T0][0] < 0x1u64) as u64;
                                            c.mult(0, 4, 4, 2);
                                            c.g[V1][0] = 0x530000;
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                                            c.g[A1][0] = 0xfffffffffffffffe;
                                            c.g[A2][0] = c.g[A3][0] & 0x1;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32)
                                                    .wrapping_add(c.g[V1][0] as u32));
                                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                                            c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                                            let t = c.g[RA][0] as u32;
                                            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                                            c.pc = t;
                                            return Ok(());
                                        }
                                        0x00193e94 => {
                                            c.cycles += 8;
                                            c.g[A1][0] = 0xc;
                                            c.g[V1][0] = 0x530000;
                                            c.mult(0, 5, 8, 5);
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                                            c.g[A0][0] = c.g[A3][0] & 0x1;
                                            c.g[A2][0] = 0xffffffffffffffef;
                                            let t = true;
                                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
                                            if t {
                                                break 'b_193e48;
                                            } else {
                                                break 'b_193eb4;
                                            }
                                        }
                                        0x00193eb4 => {
                                            break 'b_193eb4;
                                        }
                                        0x00193ed4 => {
                                            break 'b_193ed4;
                                        }
                                        0x00193ef4 => {
                                            break 'b_193ef4;
                                        }
                                        0x00193f14 => {
                                            break 'b_193f14;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x00193de0, 0x00193f38);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[A1][0] = 0xc;
                                c.g[V1][0] = 0x530000;
                                c.mult(0, 5, 8, 5);
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                                c.g[A0][0] = c.g[A3][0] & 0x1;
                                c.g[A2][0] = 0xffffffffffffffdf;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32) << 5);
                                if t {
                                    break 'b_193e48;
                                } else {
                                    break 'b_193ed4;
                                }
                            }
                            c.cycles += 8;
                            c.g[A1][0] = 0xc;
                            c.g[V1][0] = 0x530000;
                            c.mult(0, 5, 8, 5);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                            c.g[A0][0] = c.g[A3][0] & 0x1;
                            c.g[A2][0] = 0xfffffffffffffffd;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
                            if t {
                                break 'b_193e48;
                            } else {
                                break 'b_193ef4;
                            }
                        }
                        c.cycles += 8;
                        c.g[A1][0] = 0xc;
                        c.g[V1][0] = 0x530000;
                        c.mult(0, 5, 8, 5);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                        c.g[A0][0] = c.g[A3][0] & 0x1;
                        c.g[A2][0] = 0xfffffffffffffffb;
                        let t = true;
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                        if t {
                            break 'b_193e48;
                        } else {
                            break 'b_193f14;
                        }
                    }
                    c.cycles += 7;
                    c.g[A0][0] = (c.g[T0][0] < 0x1u64) as u64;
                    c.g[V1][0] = 0xc;
                    c.mult(0, 4, 4, 3);
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
                    break 'b_193f30;
                }
                c.cycles += 6;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
                c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
                let t = c.g[RA][0] as u32;
                c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                c.pc = t;
                return Ok(());
            }
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.pc = t;
        return Ok(());
    });
}

/// `int cFEStateBraggingRights_GetRowDefault(self, player, kind)` (0x193f38):
/// read back the BE rules value of a menu row (kinds 0..7, jump table at
/// 0x45fef0); -1 for other kinds.
/// Resumes at any point other than the entry through the generated body.
pub fn c_fe_state_bragging_rights_get_row_default(c: &mut Cpu) {
    if c.pc != 0x00193f38 {
        return ee::s02::f_00193f38__entries(c);
    }
    guest(c, |c| {
        c.cycles += 3;
        c.g[V0][0] = (c.g[A2][0] < 0x8u64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x460000;
        if t {
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = 0xffffffffffffffff;
            c.pc = t;
            return Ok(());
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[A2][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x110));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            let t = c.g[A0][0] as u32;
            match t {
                0x00193f5c => {
                    c.cycles += 8;
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = (c.g[A1][0] < 0x1u64) as u64;
                    c.mult(0, 4, 4, 2);
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.pc = t;
                    return Ok(());
                }
                0x00193f7c => {
                    c.cycles += 10;
                    c.g[V1][0] = (c.g[A1][0] < 0x1u64) as u64;
                    c.g[A0][0] = 0xc;
                    c.mult(0, 3, 3, 4);
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00193fa4 => {
                    c.cycles += 9;
                    c.g[V1][0] = (c.g[A1][0] < 0x1u64) as u64;
                    c.g[A0][0] = 0xc;
                    c.mult(0, 3, 3, 4);
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00193fc8 => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[V0][0] = 0x530000;
                    c.mult(0, 3, 5, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 4);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00193fec => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[V0][0] = 0x530000;
                    c.mult(0, 3, 5, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 5);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00194010 => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[V0][0] = 0x530000;
                    c.mult(0, 3, 5, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00194034 => {
                    c.cycles += 9;
                    c.g[V1][0] = 0xc;
                    c.g[V0][0] = 0x530000;
                    c.mult(0, 3, 5, 3);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 2);
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    c.pc = t;
                    return Ok(());
                }
                0x00194058 => {
                    c.cycles += 8;
                    c.g[V0][0] = 0xc;
                    c.g[A0][0] = (c.g[A1][0] < 0x1u64) as u64;
                    c.mult(0, 4, 4, 2);
                    c.g[V1][0] = 0x530000;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x8b8));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(t, 0x00193f38, 0x00194080);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    const OC850_CTRL: u32 = DATA + 0x28_0000;
    const OC850_MAGIC: u32 = 0x4f43_3835;
    const OC850_SELF: u32 = DATA + 0x20_0000;
    const OC850_ENGINE: u32 = DATA + 0x20_1000;
    const OC850_POPUP: u32 = DATA + 0x20_2000;
    const OC850_MAP: u32 = DATA + 0x20_3000;
    const OC850_OBJ: u32 = DATA + 0x20_4000;
    const OC850_VT: u32 = DATA + 0x20_5000;
    const OC850_SETUP: u32 = DATA + 0x20_6000;
    const OC850_WIDGET: u32 = DATA + 0x21_0000;
    const OC850_CTRLPTR: u32 = DATA + 0x21_1000;
    /// A real leaf returning 0 (a virtual method that answers "no state").
    /// The online flag word (`gp-0x1680`).
    const OC850_ONLINE_FLAG: u32 = crate::difftest::GAME_GP - 0x1680;
    const OC850_ZERO_LEAF: u32 = 0x0014_06c0;

    macro_rules! oc850_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC850_CTRL) != OC850_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fakes::fake_call(c, $addr);
                }
            )*
            fn oc850_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc850_fakes!(
        (
            oc850_fake_192740,
            OC850_PREV_192740,
            ee::s02::OVERRIDE_00192740,
            ee::s02::f_00192740__generated,
            0x0019_2740
        ),
        (
            oc850_fake_194080,
            OC850_PREV_194080,
            ee::s02::OVERRIDE_00194080,
            ee::s02::f_00194080__generated,
            0x0019_4080
        ),
        (
            oc850_fake_265458,
            OC850_PREV_265458,
            ee::s07::OVERRIDE_00265458,
            ee::s07::f_00265458__generated,
            0x0026_5458
        ),
        (
            oc850_fake_265568,
            OC850_PREV_265568,
            ee::s07::OVERRIDE_00265568,
            ee::s07::f_00265568__generated,
            0x0026_5568
        ),
        (
            oc850_fake_265688,
            OC850_PREV_265688,
            ee::s07::OVERRIDE_00265688,
            ee::s07::f_00265688__generated,
            0x0026_5688
        ),
        (
            oc850_fake_39f400,
            OC850_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc850_fake_39a738,
            OC850_PREV_39A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738
        ),
        (
            oc850_fake_1a8918,
            OC850_PREV_1A8918,
            ee::s03::OVERRIDE_001A8918,
            ee::s03::f_001a8918__generated,
            0x001a_8918
        ),
        (
            oc850_fake_1a9bc0,
            OC850_PREV_1A9BC0,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a_9bc0
        ),
        (
            oc850_fake_1c57c0,
            OC850_PREV_1C57C0,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c_57c0
        ),
        (
            oc850_fake_3e6448,
            OC850_PREV_3E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
    );

    /// A bragging-rights screen object with its two menus (`+0x6d0`, `+0x704`,
    /// 0x34 bytes each: control pointer, 8 row widgets from `+8`, confirmed flag
    /// at `+0x28`), the engine/vtable chain behind `self+0x10` and the online
    /// flag at `gp-0x1680`. `rows` puts the widget into random row slots.
    fn oc850_screen(c: &mut Cpu, r: &mut Rng, rows: bool) {
        oc850_tables(c);
        c.bus.write32(OC850_CTRL, OC850_MAGIC);
        c.bus.write32(OC850_SELF + 0x10, OC850_ENGINE);
        c.bus.write32(OC850_SELF + 0x6a0, OC850_POPUP);
        c.bus.write32(OC850_SELF + 0x738, OC850_MAP);
        c.bus.write32(OC850_ENGINE, OC850_OBJ);
        c.bus.write32(OC850_OBJ + 4, OC850_VT);
        for off in [0x20u32, 0x28] {
            c.bus
                .write16(OC850_VT + off, (r.next_u32() as i16 as i32 & !3) as u16);
        }
        for off in [0x24u32, 0x2c] {
            let target = if r.next_u32() % 3 == 0 {
                OC850_ZERO_LEAF
            } else {
                FAKE + off
            };
            c.bus.write32(OC850_VT + off, target);
        }
        for i in 0..8 {
            let v = match r.next_u32() % 4 {
                0 => 0xffff_ffffu32,
                _ => r.next_u32() % 8,
            };
            c.bus.write8(OC850_MAP + i, v as u8);
        }
        c.bus.write32(
            OC850_ONLINE_FLAG,
            [0u32, 1, 1, 2][(r.next_u32() % 4) as usize],
        );
        c.bus.write32(crate::difftest::GAME_GP + 0x2dc, OC850_SETUP);
        for menu in 0..2u32 {
            let base = OC850_SELF + 0x6d0 + 0x34 * menu;
            c.bus.write32(base, OC850_CTRLPTR + 0x100 * menu);
            c.bus.write32(base + 0x28, r.next_u32() % 2);
            if rows {
                for k in 0..8u32 {
                    if r.next_u32() % 3 == 0 {
                        c.bus.write32(base + 8 + 4 * k, OC850_WIDGET);
                    }
                }
            }
        }
        c.bus.write32(OC850_WIDGET + 0x18, r.next_u32() % 2);
        c.bus.write32(OC850_WIDGET + 0x8, OC850_VT);
    }

    /// The jump tables of the five functions (game data the test RAM lacks).
    fn oc850_tables(c: &mut Cpu) {
        let tables: [(u32, &[u32]); 4] = [
            (
                0x0045_fdc0,
                &[
                    0x0019_3b08,
                    0x0019_3b08,
                    0x0019_3bdc,
                    0x0019_3bdc,
                    0x0019_3948,
                    0x0019_3a1c,
                    0x0019_3b28,
                    0x0019_3bdc,
                    0x0019_3b4c,
                ],
            ),
            (
                0x0045_feb0,
                &[
                    0x0019_3d64,
                    0x0019_3d98,
                    0x0019_3db8,
                    0x0019_3d5c,
                    0x0019_3d5c,
                ],
            ),
            (
                0x0045_fed0,
                &[
                    0x0019_3e08,
                    0x0019_3e28,
                    0x0019_3e60,
                    0x0019_3e94,
                    0x0019_3eb4,
                    0x0019_3ed4,
                    0x0019_3ef4,
                    0x0019_3f14,
                ],
            ),
            (
                0x0045_fef0,
                &[
                    0x0019_3f5c,
                    0x0019_3f7c,
                    0x0019_3fa4,
                    0x0019_3fc8,
                    0x0019_3fec,
                    0x0019_4010,
                    0x0019_4034,
                    0x0019_4058,
                ],
            ),
        ];
        for (base, words) in tables {
            crate::difftest::put_words(c, base, words);
        }
    }

    /// Events 0..=10 (1..=9 take the jump table), online or not, rows with and
    /// without the widget, virtual methods answering a state or nothing.
    #[test]
    fn bragging_rights_on_widget_action_matches() {
        oc850_install_fakes();
        check(
            ee::s02::f_001938f8__generated,
            c_fe_state_bragging_rights_on_widget_action,
            11 * 24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1938f8, 6);
                c.pc = 0x0019_38f8;
                let event = seed % 11;
                oc850_screen(c, &mut r, matches!(event, 5 | 9) && seed % 5 != 0);
                c.g[A0][0] = OC850_SELF as u64;
                c.g[A1][0] = OC850_WIDGET as u64;
                c.g[A2][0] = event as u64;
            },
        );
    }

    /// The help popup is filled through real setters and the string hashes.
    #[test]
    fn bragging_rights_v25_matches() {
        check(
            ee::s02::f_00193c00__generated,
            c_fe_state_bragging_rights_v25,
            8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x193c00, 0);
                c.pc = 0x0019_3c00;
                c.g[A0][0] = OC850_SELF as u64;
                c.g[A1][0] = (DATA + 0x30_0000 + (r.next_u32() & 0xf_fff0)) as u64;
            },
        );
    }

    /// Messages 3..=11 on the first menu, the second menu and neither, online or
    /// not, with the "confirmed" flags both ways.
    #[test]
    fn bragging_rights_on_widget_notify_matches() {
        oc850_install_fakes();
        check(
            ee::s02::f_00193cf8__generated,
            c_fe_state_bragging_rights_on_widget_notify,
            9 * 3 * 12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x193cf8, 4);
                c.pc = 0x0019_3cf8;
                oc850_screen(c, &mut r, false);
                let msg = 3 + seed % 9;
                let which = (seed / 9) % 3;
                let ctrl = OC850_CTRLPTR + 0x40;
                c.bus.write32(
                    OC850_SELF + 0x6d0,
                    if which == 0 { ctrl } else { 0x1111_0000 },
                );
                c.bus.write32(
                    OC850_SELF + 0x704,
                    if which == 1 { ctrl } else { 0x2222_0000 },
                );
                c.bus.write32(OC850_SELF + 0x72c, r.next_u32() % 2);
                c.g[A0][0] = OC850_SELF as u64;
                c.g[A1][0] = ctrl as u64;
                c.g[A2][0] = msg as u64;
            },
        );
    }

    /// Every kind 0..=9 (8 and 9 do nothing) for player 0, 1 and others.
    #[test]
    fn bragging_rights_dispatch_row_action_matches() {
        check_lohi(
            ee::s02::f_00193de0__generated,
            c_fe_state_bragging_rights_dispatch_row_action,
            10 * 4 * 6,
            &|c, seed| {
                let mut r = Rng(0x193d_e000 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0019_3de0;
                oc850_tables(c);
                crate::difftest::fill(c, &mut r, 0x0053_08b8, 6);
                c.g[A1][0] = [0u64, 1, 2, r.next_u32() as u64][(seed / 10 % 4) as usize];
                c.g[A2][0] = (seed % 10) as u64;
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    /// Every kind 0..=9 (8 and 9 answer -1) for player 0, 1 and others.
    #[test]
    fn bragging_rights_get_row_default_matches() {
        check_lohi(
            ee::s02::f_00193f38__generated,
            c_fe_state_bragging_rights_get_row_default,
            10 * 4 * 6,
            &|c, seed| {
                let mut r = Rng(0x193f_3800 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0019_3f38;
                oc850_tables(c);
                crate::difftest::fill(c, &mut r, 0x0053_08b8, 6);
                c.g[A1][0] = [0u64, 1, 2, r.next_u32() as u64][(seed / 10 % 4) as usize];
                c.g[A2][0] = (seed % 10) as u64;
            },
        );
    }
}
