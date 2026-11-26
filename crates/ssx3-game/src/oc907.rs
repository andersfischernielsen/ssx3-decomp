//! Staging module oc907 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc907`.

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
        0x001a_8b88,
        net_fe_state_on_net_event,
        "NetFEState_OnNetEvent",
    ),
    (
        0x001a_8e40,
        net_fe_state_handle_net_event,
        "NetFEState_HandleNetEvent",
    ),
    (
        0x001a_93d0,
        net_fe_state_on_child_event_default,
        "NetFEState_OnChildEventDefault",
    ),
    (
        0x001a_97b8,
        net_fe_state_on_child_event,
        "NetFEState_OnChildEvent",
    ),
];

/// `NetFEState_OnNetEvent` (0x001a8b88): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[allow(unused_labels, unused_assignments, unused_variables)]
pub fn net_fe_state_on_net_event(c: &mut Cpu) {
    if c.pc != 0x001a8b88 {
        return ee::s03::f_001a8b88__generated(c);
    }
    guest(c, |c| 'l_1a8b88: loop {
        'b_1a8e24: {
            'b_1a8e20: {
                'b_1a8e08: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xc8));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = (c.g[V0][0] < 0x4fu64) as u64;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S0][0] = c.g[A2][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                    if t {
                        break 'b_1a8e08;
                    } else {
                        'b_1a8c70: {
                            'b_1a8bd4: {
                                c.cycles += 5;
                                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x115));
                                c.g[S2][0] = c.g[V0][0];
                                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x115;
                                if t {
                                    break 'b_1a8bd4;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6b4));
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x115;
                                    if t {
                                        break 'b_1a8c70;
                                    } else {
                                        break 'b_1a8bd4;
                                    }
                                }
                            }
                            'b_1a8bfc: {
                                c.cycles += 2;
                                if c.g[A1][0] != c.g[V0][0] {
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_1a8bfc;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    if c.g[A0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_1a8bfc;
                                    } else {
                                        c.cycles += 5;
                                        c.g[RA][0] = sx(0x001a8bf0);
                                        call_ptr(c, 0x00261058, 0x001a8bf0)?;
                                        c.g[RA][0] = sx(0x001a8bf8);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        call_ptr(c, 0x00261408, 0x001a8bf8)?;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_1a8bfc;
                                    }
                                }
                            }
                            'b_1a8c68: {
                                'b_1a8c64: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xf0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xf4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001a8c0c);
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001a8c0c)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_1a8c64;
                                    } else {
                                        'b_1a8c40: {
                                            c.cycles += 3;
                                            c.g[V0][0] = (c.g[S2][0] < 0x2u64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A2][0] = c.g[S0][0];
                                            if t {
                                                break 'b_1a8c40;
                                            } else {
                                                c.cycles += 8;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                c.g[A1][0] = c.lw((c.g[S0][0] as u32));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0xf8));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0xfc));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001a8c38);
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001a8c38)?;
                                                let t = true;
                                                c.sw((c.g[S1][0] as u32).wrapping_add(0x6b4), 0u64);
                                                if t {
                                                    break 'b_1a8c68;
                                                } else {
                                                    break 'b_1a8c40;
                                                }
                                            }
                                        }
                                        c.cycles += 9;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x115;
                                        c.g[A2][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xf8));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xfc));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001a8c5c);
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001a8c5c)?;
                                        let t = true;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x6b4), 0u64);
                                        if t {
                                            break 'b_1a8c68;
                                        } else {
                                            break 'b_1a8c64;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x6b4), c.g[V0][0]);
                                break 'b_1a8c68;
                            }
                            c.cycles += 2;
                            let t = true;
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_1a8e24;
                            } else {
                                break 'b_1a8c70;
                            }
                        }
                        'b_1a8c84: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x114;
                            let t = c.g[A1][0] == c.g[V0][0];
                            c.g[A1][0] = c.g[S0][0];
                            if t {
                                break 'b_1a8c84;
                            } else {
                                c.cycles += 2;
                                c.g[RA][0] = sx(0x001a8c84);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001a8ae8,
                                    f: ee::s03::NetFEState_QueueNetEvent,
                                }
                                .call(c, 0x001a8c84)?;
                                break 'b_1a8c84;
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c8));
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6cc));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                        let t = c.g[V1][0] == c.g[A1][0];
                        c.g[S3][0] = c.g[A0][0];
                        if t {
                            break 'b_1a8e20;
                        } else {
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xf0));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xf4));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001a8cb0);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001a8cb0)?;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_1a8e24;
                            } else {
                                'b_1a8cec: {
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                                    c.g[V0][0] = c.g[V0][0] & 0x6;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                        break 'b_1a8cec;
                                    } else {
                                        'b_1a8ce8: {
                                            c.cycles += 3;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x6a0));
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0xbc));
                                            if t {
                                                break 'b_1a8ce8;
                                            } else {
                                                c.cycles += 2;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                                    break 'b_1a8cec;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x170)
                                                        );
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V0][0] = 0u64;
                                                    if t {
                                                        break 'b_1a8e24;
                                                    } else {
                                                        break 'b_1a8ce8;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                        break 'b_1a8cec;
                                    }
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6cc));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                let t = c.g[V1][0] == c.g[A0][0];
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_1a8e24;
                                } else {
                                    c.cycles += 1;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                    'l_1a8d00: loop {
                                        'b_1a8dd0: {
                                            c.cycles += 24;
                                            c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                            c.sw((c.g[SP][0] as u32), c.g[V1][0]);
                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                            c.g[S2][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                                            c.sw(
                                                (c.g[SP][0] as u32).wrapping_add(0x20),
                                                c.g[V0][0],
                                            );
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                            c.g[S0][0] = c.lw((c.g[V0][0] as u32));
                                            c.sw((c.g[V1][0] as u32), c.g[S0][0]);
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                                            c.g[RA][0] = sx(0x001a8d2c);
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                            call_ptr(c, 0x00317e50, 0x001a8d2c)?;
                                            c.sw(
                                                (c.g[SP][0] as u32).wrapping_add(0x10),
                                                c.g[S0][0],
                                            );
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = c.lw((c.g[S2][0] as u32));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0xf8));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0xfc));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001a8d4c);
                                            c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001a8d4c)?;
                                            c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xec));
                                            c.g[V0][0] = (c.g[V1][0] < 0x28u64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x460000;
                                            if t {
                                                break 'b_1a8dd0;
                                            } else {
                                                'b_1a8da8: {
                                                    'b_1a8d8c: {
                                                        c.cycles += 6;
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x1a90));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                        let t = c.g[A0][0] as u32;
                                                        match t {
                                                            0x001a8b88 => {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x001a8b88,
                                                                    );
                                                                }
                                                                continue 'l_1a8b88;
                                                            }
                                                            0x001a8d78 => {
                                                                c.cycles += 5;
                                                                c.g[RA][0] = sx(0x001a8d80);
                                                                call_ptr(
                                                                    c, 0x0028b180, 0x001a8d80,
                                                                )?;
                                                                c.g[A0][0] = c.g[V0][0];
                                                                c.g[RA][0] = sx(0x001a8d8c);
                                                                c.g[A1][0] = 0x7;
                                                                call_ptr(
                                                                    c, 0x00294f78, 0x001a8d8c,
                                                                )?;
                                                                break 'b_1a8d8c;
                                                            }
                                                            0x001a8d8c => {
                                                                break 'b_1a8d8c;
                                                            }
                                                            0x001a8da8 => {
                                                                break 'b_1a8da8;
                                                            }
                                                            0x001a8dd0 => {
                                                                break 'b_1a8dd0;
                                                            }
                                                            _ => {
                                                                c.jump_out(
                                                                    t, 0x001a8b88, 0x001a8e40,
                                                                );
                                                                return Ok(());
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 7;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0xd8));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0xdc));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x001a8da0);
                                                    c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x001a8da0)?;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1a8dd0;
                                                    } else {
                                                        break 'b_1a8da8;
                                                    }
                                                }
                                                c.cycles += 10;
                                                c.g[RA][0] = sx(0x001a8db0);
                                                call_ptr(c, 0x0028b180, 0x001a8db0)?;
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x001a8dbc);
                                                c.g[A1][0] = 0x8;
                                                call_ptr(c, 0x00294f78, 0x001a8dbc)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0xd8));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xdc));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001a8dd0);
                                                c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001a8dd0)?;
                                                break 'b_1a8dd0;
                                            }
                                        }
                                        'b_1a8dec: {
                                            c.cycles += 2;
                                            let t = c.g[S2][0] == 0u64;
                                            c.g[A1][0] = 0x3;
                                            if t {
                                                break 'b_1a8dec;
                                            } else {
                                                c.cycles += 5;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001a8dec);
                                                c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                call_ptr(c, t, 0x001a8dec)?;
                                                break 'b_1a8dec;
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6cc));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                        if c.g[V1][0] != c.g[A0][0] {
                                            c.g[A0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001a8d00);
                                            }
                                            continue 'l_1a8d00;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_1a8e24;
                                            } else {
                                                break 'b_1a8e08;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 6;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = c.g[S0][0];
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xf8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xfc));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001a8e20);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001a8e20)?;
                break 'b_1a8e20;
            }
            c.cycles += 1;
            c.g[V0][0] = 0u64;
            break 'b_1a8e24;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
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

/// `NetFEState_HandleNetEvent` (0x001a8e40): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[allow(unused_labels, unused_assignments, unused_variables)]
pub fn net_fe_state_handle_net_event(c: &mut Cpu) {
    if c.pc != 0x001a8e40 {
        return ee::s03::f_001a8e40__generated(c);
    }
    guest(c, |c| 'l_1a8e40: loop {
        'b_1a8f80: {
            'b_1a8f7c: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xdc));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = (c.g[V1][0] < 0x3bu64) as u64;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                let t = c.g[V0][0] == 0u64;
                c.g[S1][0] = c.g[A2][0];
                if t {
                    break 'b_1a8f7c;
                } else {
                    'b_1a8f5c: {
                        'b_1a8f04: {
                            'b_1a8ef4: {
                                'b_1a8ecc: {
                                    'b_1a8e90: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1b50));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001a8e40 => {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x001a8e40);
                                                }
                                                continue 'l_1a8e40;
                                            }
                                            0x001a8e80 => {
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001a8e88);
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x001a9930,
                                                    f: ee::s03::NetFEState_ShowDisconnectedPopup,
                                                }
                                                .call(c, 0x001a8e88)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_1a8f80;
                                                } else {
                                                    break 'b_1a8e90;
                                                }
                                            }
                                            0x001a8e90 => {
                                                break 'b_1a8e90;
                                            }
                                            0x001a8ecc => {
                                                break 'b_1a8ecc;
                                            }
                                            0x001a8f04 => {
                                                break 'b_1a8f04;
                                            }
                                            0x001a8f5c => {
                                                break 'b_1a8f5c;
                                            }
                                            0x001a8f7c => {
                                                break 'b_1a8f7c;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001a8e40, 0x001a8f94);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001a8e98);
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                    call_ptr(c, 0x0025da90, 0x001a8e98)?;
                                    if c.g[V0][0] != 0u64 {
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        break 'b_1a8f80;
                                    } else {
                                        c.cycles += 11;
                                        c.g[A1][0] = 0x460000;
                                        c.g[A0][0] = 0xb84;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1b30));
                                        c.g[A2][0] = 0u64;
                                        c.g[RA][0] = sx(0x001a8eb8);
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, 0x00317d70, 0x001a8eb8)?;
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                        c.g[RA][0] = sx(0x001a8ec4);
                                        c.g[A0][0] = c.g[V0][0];
                                        Callee {
                                            addr: 0x001bcb58,
                                            f: ee::s03::cFEStateLobbyMenu_ctor,
                                        }
                                        .call(c, 0x001a8ec4)?;
                                        let t = true;
                                        c.g[A0][0] = c.g[S0][0];
                                        if t {
                                            break 'b_1a8ef4;
                                        } else {
                                            break 'b_1a8ecc;
                                        }
                                    }
                                }
                                c.cycles += 10;
                                c.g[A1][0] = 0x460000;
                                c.g[A0][0] = 0x6d0;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd8));
                                c.g[A2][0] = 0u64;
                                c.g[RA][0] = sx(0x001a8ee4);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x001a8ee4)?;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x001a8ef0);
                                c.g[A0][0] = c.g[V0][0];
                                Callee {
                                    addr: 0x001bfbb8,
                                    f: ee::s03::cFEStateOnlineMainMenu_ctor,
                                }
                                .call(c, 0x001a8ef0)?;
                                c.g[A0][0] = c.g[S0][0];
                                break 'b_1a8ef4;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001a8efc);
                            c.g[A1][0] = c.g[V0][0];
                            Callee {
                                addr: 0x001a8a40,
                                f: ee::s03::NetFEState_GotoNextState,
                            }
                            .call(c, 0x001a8efc)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1a8f80;
                            } else {
                                break 'b_1a8f04;
                            }
                        }
                        'b_1a8f28: {
                            c.cycles += 5;
                            c.g[V0][0] = 0x74690000;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[V0][0] = c.g[V0][0] | 0x6d65;
                            let t = c.g[V1][0] != c.g[V0][0];

                            if t {
                                break 'b_1a8f28;
                            } else {
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001a8f20);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001a8f98,
                                    f: ee::s03::NetFEState_OnNetTimeout,
                                }
                                .call(c, 0x001a8f20)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1a8f80;
                                } else {
                                    break 'b_1a8f28;
                                }
                            }
                        }
                        c.cycles += 13;
                        c.g[RA][0] = sx(0x001a8f30);
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001a9c50,
                            f: ee::s03::NetFEState_GetPopup,
                        }
                        .call(c, 0x001a8f30)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[SP][0];
                        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
                        c.g[A3][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001a8f48);
                        c.g[A2][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001a9cc0,
                            f: ee::s03::NetFEState_ShowNetErrorPopup,
                        }
                        .call(c, 0x001a8f48)?;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32));
                        c.g[RA][0] = sx(0x001a8f54);
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001a9bc0,
                            f: ee::s03::NetFEState_ShowPopup,
                        }
                        .call(c, 0x001a8f54)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_1a8f80;
                        } else {
                            break 'b_1a8f5c;
                        }
                    }
                    c.cycles += 3;
                    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
                    let t = c.g[A3][0] == 0u64;
                    c.g[A2][0] = c.g[S1][0];
                    if t {
                        break 'b_1a8f7c;
                    } else {
                        c.cycles += 5;
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001a8f7c);
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001a8f7c)?;
                        break 'b_1a8f7c;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1a8f80;
        }
        c.cycles += 5;
        c.g[V0][0] = 0u64;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `NetFEState_OnChildEventDefault` (0x001a93d0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[allow(unused_labels, unused_assignments, unused_variables)]
pub fn net_fe_state_on_child_event_default(c: &mut Cpu) {
    if c.pc != 0x001a93d0 {
        return ee::s03::f_001a93d0__generated(c);
    }
    guest(c, |c| {
        'b_1a942c: {
            'b_1a9428: {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0xf));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A3][0] = c.g[A0][0];
                if t {
                    break 'b_1a9428;
                } else {
                    'b_1a941c: {
                        c.cycles += 7;
                        c.g[V0][0] = 0x460000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c80));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                        let t = c.g[A0][0] as u32;
                        match t {
                            0x001a9404 => {
                                c.cycles += 6;
                                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = 0x1;
                                c.g[RA][0] = sx(0x001a9414);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f190, 0x001a9414)?;
                                let t = true;
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                if t {
                                    break 'b_1a942c;
                                } else {
                                    break 'b_1a941c;
                                }
                            }
                            0x001a941c => {
                                break 'b_1a941c;
                            }
                            0x001a9428 => {
                                break 'b_1a9428;
                            }
                            _ => {
                                c.jump_out(t, 0x001a93d0, 0x001a9434);
                                return Ok(());
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x6a0));
                    if c.g[A1][0] == c.g[V0][0] {
                        c.sw((c.g[A3][0] as u32).wrapping_add(0x6a0), 0u64);
                        break 'b_1a9428;
                    } else {
                        break 'b_1a9428;
                    }
                }
            }
            c.cycles += 1;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            break 'b_1a942c;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `NetFEState_OnChildEvent` (0x001a97b8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[allow(unused_labels, unused_assignments, unused_variables)]
pub fn net_fe_state_on_child_event(c: &mut Cpu) {
    if c.pc != 0x001a97b8 {
        return ee::s03::f_001a97b8__generated(c);
    }
    guest(c, |c| 'l_1a97b8: loop {
        'b_1a9884: {
            'b_1a97e0: {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.g[A3][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x69c));
                if c.g[A1][0] != c.g[V0][0] {
                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                    break 'b_1a97e0;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001a97d8);
                    Callee {
                        addr: 0x001a94e0,
                        f: ee::s03::NetFEState_OnKeyboardPopupEvent,
                    }
                    .call(c, 0x001a97d8)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    if t {
                        break 'b_1a9884;
                    } else {
                        break 'b_1a97e0;
                    }
                }
            }
            'b_1a9878: {
                c.cycles += 4;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x832));
                c.g[V1][0] = (c.g[A0][0] < 0xdu64) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x460000;
                if t {
                    break 'b_1a9878;
                } else {
                    'b_1a9868: {
                        'b_1a9858: {
                            'b_1a9848: {
                                'b_1a9838: {
                                    'b_1a9828: {
                                        'b_1a9818: {
                                            c.cycles += 6;
                                            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x1cf0));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                            let t = c.g[A0][0] as u32;
                                            match t {
                                                0x001a97b8 => {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x001a97b8);
                                                    }
                                                    continue 'l_1a97b8;
                                                }
                                                0x001a9808 => {
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x001a9810);
                                                    c.g[A0][0] = c.g[A3][0];
                                                    Callee { addr: 0x001a9438, f: ee::s03::NetFEState_OnMemCardLoginPopupEvent }.call(c, 0x001a9810)?;
                                                    let t = true;
                                                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                                    if t {
                                                        break 'b_1a9884;
                                                    } else {
                                                        break 'b_1a9818;
                                                    }
                                                }
                                                0x001a9818 => {
                                                    break 'b_1a9818;
                                                }
                                                0x001a9828 => {
                                                    break 'b_1a9828;
                                                }
                                                0x001a9838 => {
                                                    break 'b_1a9838;
                                                }
                                                0x001a9848 => {
                                                    break 'b_1a9848;
                                                }
                                                0x001a9858 => {
                                                    break 'b_1a9858;
                                                }
                                                0x001a9868 => {
                                                    break 'b_1a9868;
                                                }
                                                0x001a9878 => {
                                                    break 'b_1a9878;
                                                }
                                                _ => {
                                                    c.jump_out(t, 0x001a97b8, 0x001a988c);
                                                    return Ok(());
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001a9820);
                                        c.g[A0][0] = c.g[A3][0];
                                        Callee {
                                            addr: 0x001a9268,
                                            f: ee::s03::NetFEState_OnMessengerPopupEvent,
                                        }
                                        .call(c, 0x001a9820)?;
                                        let t = true;
                                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                        if t {
                                            break 'b_1a9884;
                                        } else {
                                            break 'b_1a9828;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001a9830);
                                    c.g[A0][0] = c.g[A3][0];
                                    Callee {
                                        addr: 0x001a92d8,
                                        f: ee::s03::NetFEState_OnChallengePopupEvent,
                                    }
                                    .call(c, 0x001a9830)?;
                                    let t = true;
                                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                    if t {
                                        break 'b_1a9884;
                                    } else {
                                        break 'b_1a9838;
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001a9840);
                                c.g[A0][0] = c.g[A3][0];
                                Callee {
                                    addr: 0x001a9578,
                                    f: ee::s03::NetFEState_OnSessionLostPopupEvent,
                                }
                                .call(c, 0x001a9840)?;
                                let t = true;
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                if t {
                                    break 'b_1a9884;
                                } else {
                                    break 'b_1a9848;
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001a9850);
                            c.g[A0][0] = c.g[A3][0];
                            Callee {
                                addr: 0x001a9638,
                                f: ee::s03::NetFEState_OnChildEventIgnoreClose,
                            }
                            .call(c, 0x001a9850)?;
                            let t = true;
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                            if t {
                                break 'b_1a9884;
                            } else {
                                break 'b_1a9858;
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001a9860);
                        c.g[A0][0] = c.g[A3][0];
                        Callee {
                            addr: 0x001a9668,
                            f: ee::s03::NetFEState_OnDisconnectedPopupEvent,
                        }
                        .call(c, 0x001a9860)?;
                        let t = true;
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        if t {
                            break 'b_1a9884;
                        } else {
                            break 'b_1a9868;
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001a9870);
                    c.g[A0][0] = c.g[A3][0];
                    Callee {
                        addr: 0x001a9710,
                        f: ee::s03::NetFEState_OnNetInterruptedPopupEvent,
                    }
                    .call(c, 0x001a9870)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    if t {
                        break 'b_1a9884;
                    } else {
                        break 'b_1a9878;
                    }
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x001a9880);
            c.g[A0][0] = c.g[A3][0];
            Callee {
                addr: 0x001a93d0,
                f: ee::s03::NetFEState_OnChildEventDefault,
            }
            .call(c, 0x001a9880)?;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            break 'b_1a9884;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA};

    const SELF: u32 = DATA + 0x1000;
    const LIST_A: u32 = DATA + 0x3400;
    const LIST_V: u32 = DATA + 0x3800;
    const LIST_EV: u32 = DATA + 0x3c00;

    const GATE: u32 = DATA + 0x3f00;
    const MAGIC: u32 = 0x0907_f00d;

    /// Gated fake of `__builtin_delete` (0x317e50): the real one takes a
    /// semaphore and waits forever on a test machine.
    fn fake_delete(c: &mut Cpu) {
        if c.bus.read32(GATE) != MAGIC {
            return crate::difftest::fakes::fall_back(
                c,
                crate::difftest::fakes::prev_cell(fake_delete),
                ee::s10::f_00317e50__generated,
            );
        }
        c.cycles += 5;
        c.g[V0][0] = 0;
        crate::abi::ret(c);
    }

    fn install_fake_delete() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            fake_delete,
            crate::difftest::fakes::prev_cell(fake_delete),
        );
    }

    /// A state object at `SELF` with a vtable (slots `0xc0 0xd8 0xf0 0xf8`
    /// call the fake; the 0xf0 slot sometimes runs a real leaf returning 0).
    fn net_state(c: &mut Cpu, r: &mut crate::difftest::Rng, seed: u32) {
        let vt = fakes::vtable(c, r, &[0xc0, 0xd8, 0xf0, 0xf8]);
        c.bus.write32(SELF + 8, vt);
        if seed % 2 == 1 {
            c.bus.write32(vt + 0xf4, 0x0014_06c0);
        }
        c.bus.write32(SELF + 0x10, DATA + 0x6000);
        // Child state at +0x20 (default branch of HandleNetEvent) and the
        // popup slot at +0x69c / +0x6a0.
        c.bus
            .write32(SELF + 0x20, if seed % 5 == 0 { 0 } else { SELF + 0x400 });
        c.bus.write32(SELF + 0x408, vt);
        c.bus.write32(crate::difftest::GAME_GP + 0x238, 0);
    }

    /// `NetFEState_OnNetEvent`: dispatches a network event: the login events
    /// (0x115/0x116), the queue case (0x114), a pending-event list drained
    /// through the jump table, or a plain forward to the state's handler.
    #[test]
    fn net_fe_state_on_net_event_matches() {
        install_fake_delete();
        check(
            ee::s03::f_001a8b88__generated,
            net_fe_state_on_net_event,
            160,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x0907_0001, 3);
                c.pc = 0x001a_8b88;
                c.bus.write32(GATE, MAGIC);
                net_state(c, &mut r, seed);
                let ev = match seed % 8 {
                    0 => 0x115,
                    1 => 0x116,
                    2 => 0x114,
                    3 => 0x10,
                    _ => 0xc8 + (seed / 8) % 0x4f,
                };
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = ev as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                c.bus.write32(SELF + 0x6b4, (seed / 3) % 2);
                c.bus
                    .write32(SELF + 0x1c, if seed % 3 == 0 { 0 } else { 6 });
                c.bus
                    .write32(SELF + 0x6a0, if seed % 4 == 0 { 0 } else { SELF + 0x800 });
                // Pending list: empty (seed even) or one item whose event
                // code walks the 0xec.. jump table.
                // SELF+0x6c8 is the list header, SELF+0x6cc its circular
                // sentinel node (LIST_A): empty when the first node is the
                // sentinel itself, else one item V (next and prev: the
                // sentinel) whose unlink makes it empty again.
                c.bus.write32(SELF + 0x6cc, LIST_A);
                if seed % 2 == 0 {
                    c.bus.write32(LIST_A, LIST_A);
                    c.bus.write32(LIST_A + 4, LIST_A);
                } else {
                    c.bus.write32(LIST_A, LIST_V);
                    c.bus.write32(LIST_A + 4, LIST_V);
                    c.bus.write32(LIST_V, LIST_A);
                    c.bus.write32(LIST_V + 4, LIST_A);
                    c.bus.write32(LIST_V + 8, LIST_EV);
                    c.bus.write32(LIST_EV, 0xec + (seed / 2) % 0x2c);
                }
            },
        );
    }

    /// `NetFEState_HandleNetEvent`: the 0xdc.. network event jump table
    /// (popups, lobby and online menus, state change, timeout) with a
    /// forward to the child state for everything else.
    #[test]
    fn net_fe_state_handle_net_event_matches() {
        check(
            ee::s03::f_001a8e40__generated,
            net_fe_state_handle_net_event,
            160,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x0907_0002, 2);
                c.pc = 0x001a_8e40;
                net_state(c, &mut r, seed);
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = (0xdc + seed % 0x40) as u64;
                c.g[A2][0] = (DATA + 0x2000) as u64;
                // The "time" tag of the timeout case.
                c.bus.write32(
                    DATA + 0x2008,
                    if seed % 3 == 0 { 0x7469_6d65 } else { 0x1234 },
                );
            },
        );
    }

    /// `NetFEState_OnChildEventDefault`: the default child event: event
    /// 0xf..0x16 jump table (one case closes the top states), otherwise
    /// clears the popup slot when it holds the finished child.
    #[test]
    fn net_fe_state_on_child_event_default_matches() {
        check(
            ee::s03::f_001a93d0__generated,
            net_fe_state_on_child_event_default,
            80,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x0907_0003, 0);
                c.pc = 0x001a_93d0;
                net_state(c, &mut r, seed);
                c.g[A0][0] = SELF as u64;
                c.g[A2][0] = (0xf + seed % 12) as u64;
                let child = DATA + 0x2000;
                c.g[A1][0] = child as u64;
                c.bus
                    .write32(SELF + 0x6a0, if seed % 2 == 0 { child } else { 0 });
            },
        );
    }

    /// `NetFEState_OnChildEvent`: a popup child event goes to its handler
    /// (keyboard, mem-card login, messenger, challenge, session lost, ...)
    /// by the child's id (0x832..0x83e); anything else is the default.
    #[test]
    fn net_fe_state_on_child_event_matches() {
        check(
            ee::s03::f_001a97b8__generated,
            net_fe_state_on_child_event,
            160,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x0907_0004, 2);
                c.pc = 0x001a_97b8;
                net_state(c, &mut r, seed);
                let child = DATA + 0x2000;
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = child as u64;
                c.bus.write32(
                    SELF + 0x69c,
                    if seed % 9 == 0 { child } else { DATA + 0x2400 },
                );
                c.bus.write32(child + 0x18, 0x832 + seed % 0x10);
            },
        );
    }
}
