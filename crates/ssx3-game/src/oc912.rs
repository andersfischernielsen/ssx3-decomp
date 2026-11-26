//! Staging module oc912 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc912`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `cFEStateNetConfig_OnChildEvent` (0x001b5d58): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_net_config_on_child_event(c: &mut Cpu) {
    if c.pc != 0x001b5d58 {
        return ee::s03::f_001b5d58__generated(c);
    }
    guest(c, |c| 'l_1b5d58: loop {
        'b_1b5e1c: {
            'b_1b5e10: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x80e));
                c.g[V0][0] = (c.g[V1][0] < 0x37u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A3][0] = c.g[A0][0];
                if t {
                    break 'b_1b5e10;
                } else {
                    'b_1b5e00: {
                        'b_1b5df0: {
                            'b_1b5de0: {
                                'b_1b5dd0: {
                                    'b_1b5dc0: {
                                        'b_1b5db0: {
                                            'b_1b5da0: {
                                                c.cycles += 7;
                                                c.g[V0][0] = 0x460000;
                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x40d0));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x001b5d58 => {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001b5d58);
                                                        }
                                                        continue 'l_1b5d58;
                                                    }
                                                    0x001b5d90 => {
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x001b5d98);
                                                        c.g[A0][0] = c.g[A3][0];
                                                        Callee { addr: 0x001b5cf8, f: ee::s03::cFEStateNetConfig_OnInfoPopupClosed }.call(c, 0x001b5d98)?;
                                                        let t = true;
                                                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                                        if t {
                                                            break 'b_1b5e1c;
                                                        } else {
                                                            break 'b_1b5da0;
                                                        }
                                                    }
                                                    0x001b5da0 => {
                                                        break 'b_1b5da0;
                                                    }
                                                    0x001b5db0 => {
                                                        break 'b_1b5db0;
                                                    }
                                                    0x001b5dc0 => {
                                                        break 'b_1b5dc0;
                                                    }
                                                    0x001b5dd0 => {
                                                        break 'b_1b5dd0;
                                                    }
                                                    0x001b5de0 => {
                                                        break 'b_1b5de0;
                                                    }
                                                    0x001b5df0 => {
                                                        break 'b_1b5df0;
                                                    }
                                                    0x001b5e00 => {
                                                        break 'b_1b5e00;
                                                    }
                                                    0x001b5e10 => {
                                                        break 'b_1b5e10;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x001b5d58, 0x001b5e24);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001b5da8);
                                            c.g[A0][0] = c.g[A3][0];
                                            Callee { addr: 0x001b55b0, f: ee::s03::cFEStateNetConfig_OnConnectingPopupEvent }.call(c, 0x001b5da8)?;
                                            let t = true;
                                            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                            if t {
                                                break 'b_1b5e1c;
                                            } else {
                                                break 'b_1b5db0;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001b5db8);
                                        c.g[A0][0] = c.g[A3][0];
                                        Callee {
                                            addr: 0x001b5740,
                                            f: ee::s03::cFEStateNetConfig_OnDisconnectPopupEvent,
                                        }
                                        .call(c, 0x001b5db8)?;
                                        let t = true;
                                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                        if t {
                                            break 'b_1b5e1c;
                                        } else {
                                            break 'b_1b5dc0;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001b5dc8);
                                    c.g[A0][0] = c.g[A3][0];
                                    Callee {
                                        addr: 0x001b5830,
                                        f: ee::s03::cFEStateNetConfig_OnAuthPopupEvent,
                                    }
                                    .call(c, 0x001b5dc8)?;
                                    let t = true;
                                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                    if t {
                                        break 'b_1b5e1c;
                                    } else {
                                        break 'b_1b5dd0;
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001b5dd8);
                                c.g[A0][0] = c.g[A3][0];
                                Callee {
                                    addr: 0x001b5a58,
                                    f: ee::s03::cFEStateNetConfig_OnLoginPopupEvent,
                                }
                                .call(c, 0x001b5dd8)?;
                                let t = true;
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                if t {
                                    break 'b_1b5e1c;
                                } else {
                                    break 'b_1b5de0;
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001b5de8);
                            c.g[A0][0] = c.g[A3][0];
                            Callee {
                                addr: 0x001b4c40,
                                f: ee::s03::cFEStateNetConfig_OnRetryPopupEvent,
                            }
                            .call(c, 0x001b5de8)?;
                            let t = true;
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                            if t {
                                break 'b_1b5e1c;
                            } else {
                                break 'b_1b5df0;
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001b5df8);
                        c.g[A0][0] = c.g[A3][0];
                        Callee {
                            addr: 0x001b4c00,
                            f: ee::s03::cFEStateNetConfig_OnPopupEventPassthrough,
                        }
                        .call(c, 0x001b5df8)?;
                        let t = true;
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        if t {
                            break 'b_1b5e1c;
                        } else {
                            break 'b_1b5e00;
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001b5e08);
                    c.g[A0][0] = c.g[A3][0];
                    Callee {
                        addr: 0x001b5bd8,
                        f: ee::s03::cFEStateNetConfig_OnCreateConfigPopupEvent,
                    }
                    .call(c, 0x001b5e08)?;
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    if t {
                        break 'b_1b5e1c;
                    } else {
                        break 'b_1b5e10;
                    }
                }
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x001b5e18);
            c.g[A0][0] = c.g[A3][0];
            Callee {
                addr: 0x001a97b8,
                f: ee::s03::NetFEState_OnChildEvent,
            }
            .call(c, 0x001b5e18)?;
            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
            break 'b_1b5e1c;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateNetAccountMem_Update` (0x001b7cb8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_net_account_mem_update(c: &mut Cpu) {
    if c.pc != 0x001b7cb8 {
        return ee::s03::f_001b7cb8__generated(c);
    }
    guest(c, |c| 'l_1b7cb8: loop {
        'b_1b7cfc: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x001b7cd0);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
            call_ptr(c, 0x00265a68, 0x001b7cd0)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
            c.g[RA][0] = sx(0x001b7ce0);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x6d4), c.g[V0][0]);
            call_ptr(c, 0x00265d38, 0x001b7ce0)?;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_1b7cfc;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d4));
                let t = c.g[V1][0] != 0u64;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x6d8), c.g[V0][0]);
                if t {
                    break 'b_1b7cfc;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x2;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x6d4), c.g[V0][0]);
                    break 'b_1b7cfc;
                }
            }
        }
        'b_1b7e30: {
            'b_1b7e2c: {
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6dc));
                c.g[V0][0] = (c.g[V1][0] < 0x1eu64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = 0x460000;
                if t {
                    break 'b_1b7e2c;
                } else {
                    'b_1b7e24: {
                        'b_1b7e14: {
                            'b_1b7e04: {
                                'b_1b7df4: {
                                    'b_1b7de4: {
                                        'b_1b7dd0: {
                                            'b_1b7dc0: {
                                                'b_1b7db0: {
                                                    'b_1b7da0: {
                                                        'b_1b7d8c: {
                                                            'b_1b7d7c: {
                                                                'b_1b7d6c: {
                                                                    'b_1b7d58: {
                                                                        'b_1b7d48: {
                                                                            'b_1b7d38: {
                                                                                c.cycles += 6;
                                                                                c.g[V1][0] =
                                                                                    sx((c.g[V1][0]
                                                                                        as u32)
                                                                                        << 2);
                                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4430));
                                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                                c.g[A0][0] = c.lw(
                                                                                    (c.g[V1][0]
                                                                                        as u32),
                                                                                );
                                                                                let t = c.g[A0][0]
                                                                                    as u32;
                                                                                match t {
                                                                                    0x001b7cb8 => {
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x001b7cb8);
                                                                                        }
                                                                                        continue 'l_1b7cb8;
                                                                                    }
                                                                                    0x001b7d24 => {
                                                                                        c.cycles +=
                                                                                            5;
                                                                                        c.g[A0]
                                                                                            [0] = c
                                                                                            .g[S0]
                                                                                            [0];
                                                                                        c.g[RA][0] = sx(0x001b7d30);
                                                                                        c.g[A1]
                                                                                            [0] =
                                                                                            0x1;
                                                                                        Callee { addr: 0x001b70d0, f: ee::s03::cFEStateNetAccountMem_SetState }.call(c, 0x001b7d30)?;
                                                                                        let t =
                                                                                            true;
                                                                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                                                        if t {
                                                                                            break 'b_1b7e30;
                                                                                        } else {
                                                                                            break 'b_1b7d38;
                                                                                        }
                                                                                    }
                                                                                    0x001b7d38 => {
                                                                                        break 'b_1b7d38;
                                                                                    }
                                                                                    0x001b7d48 => {
                                                                                        break 'b_1b7d48;
                                                                                    }
                                                                                    0x001b7d58 => {
                                                                                        break 'b_1b7d58;
                                                                                    }
                                                                                    0x001b7d6c => {
                                                                                        break 'b_1b7d6c;
                                                                                    }
                                                                                    0x001b7d7c => {
                                                                                        break 'b_1b7d7c;
                                                                                    }
                                                                                    0x001b7d8c => {
                                                                                        break 'b_1b7d8c;
                                                                                    }
                                                                                    0x001b7da0 => {
                                                                                        break 'b_1b7da0;
                                                                                    }
                                                                                    0x001b7db0 => {
                                                                                        break 'b_1b7db0;
                                                                                    }
                                                                                    0x001b7dc0 => {
                                                                                        break 'b_1b7dc0;
                                                                                    }
                                                                                    0x001b7dd0 => {
                                                                                        break 'b_1b7dd0;
                                                                                    }
                                                                                    0x001b7de4 => {
                                                                                        break 'b_1b7de4;
                                                                                    }
                                                                                    0x001b7df4 => {
                                                                                        break 'b_1b7df4;
                                                                                    }
                                                                                    0x001b7e04 => {
                                                                                        break 'b_1b7e04;
                                                                                    }
                                                                                    0x001b7e14 => {
                                                                                        break 'b_1b7e14;
                                                                                    }
                                                                                    0x001b7e24 => {
                                                                                        break 'b_1b7e24;
                                                                                    }
                                                                                    0x001b7e2c => {
                                                                                        break 'b_1b7e2c;
                                                                                    }
                                                                                    _ => {
                                                                                        c.jump_out(t, 0x001b7cb8, 0x001b7e3c);
                                                                                        return Ok(
                                                                                            (),
                                                                                        );
                                                                                    }
                                                                                }
                                                                            }
                                                                            c.cycles += 4;
                                                                            c.g[RA][0] =
                                                                                sx(0x001b7d40);
                                                                            c.g[A0][0] = c.g[S0][0];
                                                                            Callee { addr: 0x001b77a8, f: ee::s03::cFEStateNetAccountMem_UpdateState01 }.call(c, 0x001b7d40)?;
                                                                            let t = true;
                                                                            c.lq(
                                                                                16,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x10,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_1b7e30;
                                                                            } else {
                                                                                break 'b_1b7d48;
                                                                            }
                                                                        }
                                                                        c.cycles += 4;
                                                                        c.g[RA][0] = sx(0x001b7d50);
                                                                        c.g[A0][0] = c.g[S0][0];
                                                                        Callee { addr: 0x001b77c8, f: ee::s03::cFEStateNetAccountMem_UpdateState02 }.call(c, 0x001b7d50)?;
                                                                        let t = true;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                        );
                                                                        if t {
                                                                            break 'b_1b7e30;
                                                                        } else {
                                                                            break 'b_1b7d58;
                                                                        }
                                                                    }
                                                                    c.cycles += 5;
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    c.g[RA][0] = sx(0x001b7d64);
                                                                    c.g[A1][0] = 0x6;
                                                                    Callee { addr: 0x001b70d0, f: ee::s03::cFEStateNetAccountMem_SetState }.call(c, 0x001b7d64)?;
                                                                    let t = true;
                                                                    c.lq(
                                                                        16,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x10),
                                                                    );
                                                                    if t {
                                                                        break 'b_1b7e30;
                                                                    } else {
                                                                        break 'b_1b7d6c;
                                                                    }
                                                                }
                                                                c.cycles += 4;
                                                                c.g[RA][0] = sx(0x001b7d74);
                                                                c.g[A0][0] = c.g[S0][0];
                                                                Callee { addr: 0x001b7820, f: ee::s03::cFEStateNetAccountMem_UpdateState06 }.call(c, 0x001b7d74)?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                if t {
                                                                    break 'b_1b7e30;
                                                                } else {
                                                                    break 'b_1b7d7c;
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x001b7d84);
                                                            c.g[A0][0] = c.g[S0][0];
                                                            Callee { addr: 0x001b78f0, f: ee::s03::cFEStateNetAccountMem_UpdateState08 }.call(c, 0x001b7d84)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            if t {
                                                                break 'b_1b7e30;
                                                            } else {
                                                                break 'b_1b7d8c;
                                                            }
                                                        }
                                                        c.cycles += 5;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x001b7d98);
                                                        c.g[A1][0] = 0xc;
                                                        Callee { addr: 0x001b70d0, f: ee::s03::cFEStateNetAccountMem_SetState }.call(c, 0x001b7d98)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x10),
                                                        );
                                                        if t {
                                                            break 'b_1b7e30;
                                                        } else {
                                                            break 'b_1b7da0;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x001b7da8);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    Callee { addr: 0x001b7938, f: ee::s03::cFEStateNetAccountMem_UpdateState0C }.call(c, 0x001b7da8)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x10),
                                                    );
                                                    if t {
                                                        break 'b_1b7e30;
                                                    } else {
                                                        break 'b_1b7db0;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001b7db8);
                                                c.g[A0][0] = c.g[S0][0];
                                                Callee {
                                                    addr: 0x001b7a08,
                                                    f: ee::s03::cFEStateNetAccountMem_UpdateState0E,
                                                }
                                                .call(c, 0x001b7db8)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                                if t {
                                                    break 'b_1b7e30;
                                                } else {
                                                    break 'b_1b7dc0;
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001b7dc8);
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x001b7a50,
                                                f: ee::s03::cFEStateNetAccountMem_UpdateState10,
                                            }
                                            .call(c, 0x001b7dc8)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                            if t {
                                                break 'b_1b7e30;
                                            } else {
                                                break 'b_1b7dd0;
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001b7ddc);
                                        c.g[A1][0] = 0x14;
                                        Callee {
                                            addr: 0x001b70d0,
                                            f: ee::s03::cFEStateNetAccountMem_SetState,
                                        }
                                        .call(c, 0x001b7ddc)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        if t {
                                            break 'b_1b7e30;
                                        } else {
                                            break 'b_1b7de4;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001b7dec);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001b7a98,
                                        f: ee::s03::cFEStateNetAccountMem_UpdateState14,
                                    }
                                    .call(c, 0x001b7dec)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                    if t {
                                        break 'b_1b7e30;
                                    } else {
                                        break 'b_1b7df4;
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001b7dfc);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001b7bc0,
                                    f: ee::s03::cFEStateNetAccountMem_UpdateState16,
                                }
                                .call(c, 0x001b7dfc)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                if t {
                                    break 'b_1b7e30;
                                } else {
                                    break 'b_1b7e04;
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001b7e0c);
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                            call_ptr(c, 0x002659e8, 0x001b7e0c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_1b7e30;
                            } else {
                                break 'b_1b7e14;
                            }
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001b7e1c);
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001b7c28,
                            f: ee::s03::cFEStateNetAccountMem_UpdateState1A,
                        }
                        .call(c, 0x001b7e1c)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_1b7e30;
                        } else {
                            break 'b_1b7e24;
                        }
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001b7e2c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001b7c70,
                        f: ee::s03::cFEStateNetAccountMem_UpdateState1B,
                    }
                    .call(c, 0x001b7e2c)?;
                    break 'b_1b7e2c;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1b7e30;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateNetAccountMem_ShowErrorPopup` (0x001b7e40): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_net_account_mem_show_error_popup(c: &mut Cpu) {
    if c.pc != 0x001b7e40 {
        return ee::s03::f_001b7e40__generated(c);
    }
    guest(c, |c| {
        'b_1b814c: {
            c.cycles += 26;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S5][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A0][0] = 0x460000;
            c.g[S2][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S1][0] = 0u64;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S3][0] = 0u64;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[RA][0] = sx(0x001b7e84);
            c.g[S6][0] = 0x1;
            call_ptr(c, 0x00317670, 0x001b7e84)?;
            c.g[A1][0] = 0x4a0000;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
            c.g[A0][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x001b7e9c);
            c.g[A2][0] = 0x1;
            call_ptr(c, 0x002c2718, 0x001b7e9c)?;
            c.g[V0][0] = (c.g[S2][0] < 0x19u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x460000;
            if t {
                break 'b_1b814c;
            } else {
                'b_1b8130: {
                    'b_1b812c: {
                        'b_1b8128: {
                            'b_1b8124: {
                                'b_1b8070: {
                                    'b_1b805c: {
                                        'b_1b7fb4: {
                                            'b_1b7f5c: {
                                                c.cycles += 6;
                                                c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x4520));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x001b7ec0 => {
                                                        'b_1b7f4c: {
                                                            'b_1b7f3c: {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S5][0] as u32)
                                                                        .wrapping_add(0xb04));
                                                                let t = c.g[V0][0] != 0u64;

                                                                if t {
                                                                    break 'b_1b7f3c;
                                                                } else {
                                                                    'b_1b7f28: {
                                                                        c.cycles += 4;
                                                                        c.g[RA][0] = sx(0x001b7ed4);
                                                                        c.g[A0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x2e0));
                                                                        call_ptr(
                                                                            c, 0x00265cb0,
                                                                            0x001b7ed4,
                                                                        )?;
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        if t {
                                                                            break 'b_1b7f28;
                                                                        } else {
                                                                            c.cycles += 19;
                                                                            c.g[A0][0] = 0x460000;
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x2370,
                                                                                    ));
                                                                            c.g[S2][0] =
                                                                                sx((c.g[S5][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        0x702,
                                                                                    ));
                                                                            c.g[S0][0] = c.lw((c.g
                                                                                [V0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x8c,
                                                                                ));
                                                                            c.g[S4][0] = c.g[S2][0];
                                                                            c.g[S1][0] = c.lw((c.g
                                                                                [S0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                            c.g[V0][0] = c.lh((c.g
                                                                                [S1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x20,
                                                                                ));
                                                                            c.g[RA][0] =
                                                                                sx(0x001b7f00);
                                                                            c.g[S0][0] =
                                                                                sx((c.g[S0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[V0][0]
                                                                                            as u32),
                                                                                    ));
                                                                            call_ptr(
                                                                                c, 0x00317670,
                                                                                0x001b7f00,
                                                                            )?;
                                                                            c.g[V1][0] = c.lw((c.g
                                                                                [S1][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x24,
                                                                                ));
                                                                            c.g[A0][0] = c.g[S0][0];
                                                                            let t =
                                                                                c.g[V1][0] as u32;
                                                                            c.g[RA][0] =
                                                                                sx(0x001b7f10);
                                                                            c.g[A1][0] = c.g[V0][0];
                                                                            call_ptr(
                                                                                c, t, 0x001b7f10,
                                                                            )?;
                                                                            c.g[A1][0] = c.g[V0][0];
                                                                            c.g[A0][0] = c.g[S2][0];
                                                                            c.g[RA][0] =
                                                                                sx(0x001b7f20);
                                                                            c.g[A2][0] = c.g[SP][0];
                                                                            call_ptr(
                                                                                c, 0x002c26d0,
                                                                                0x001b7f20,
                                                                            )?;
                                                                            let t = true;
                                                                            c.g[A0][0] = 0x460000;
                                                                            if t {
                                                                                break 'b_1b7f4c;
                                                                            } else {
                                                                                break 'b_1b7f28;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 5;
                                                                    c.g[A0][0] = c.g[S5][0];
                                                                    c.g[RA][0] = sx(0x001b7f34);
                                                                    c.g[S4][0] = sx((c.g[S5][0]
                                                                        as u32)
                                                                        .wrapping_add(0x702));
                                                                    Callee { addr: 0x001b8238, f: ee::s03::cFEStateNetAccountMem_FormatNotDetectedText }.call(c, 0x001b7f34)?;
                                                                    let t = true;
                                                                    c.g[A0][0] = 0x460000;
                                                                    if t {
                                                                        break 'b_1b7f4c;
                                                                    } else {
                                                                        break 'b_1b7f3c;
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 4;
                                                            c.g[RA][0] = sx(0x001b7f44);
                                                            c.g[A0][0] = c.g[S5][0];
                                                            Callee { addr: 0x001b83c0, f: ee::s03::cFEStateNetAccountMem_FormatFailedText }.call(c, 0x001b7f44)?;
                                                            c.g[S4][0] = sx((c.g[S5][0] as u32)
                                                                .wrapping_add(0x702));
                                                            c.g[A0][0] = 0x460000;
                                                            break 'b_1b7f4c;
                                                        }
                                                        c.cycles += 4;
                                                        c.g[S3][0] = c.g[S4][0];
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x2328));
                                                        let t = true;
                                                        c.g[S6][0] = 0u64;
                                                        if t {
                                                            break 'b_1b8130;
                                                        } else {
                                                            break 'b_1b7f5c;
                                                        }
                                                    }
                                                    0x001b7f5c => {
                                                        break 'b_1b7f5c;
                                                    }
                                                    0x001b7fb4 => {
                                                        break 'b_1b7fb4;
                                                    }
                                                    0x001b805c => {
                                                        break 'b_1b805c;
                                                    }
                                                    0x001b8070 => {
                                                        break 'b_1b8070;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x001b7e40, 0x001b8234);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 22;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] = 0x460000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x44a8));
                                            c.g[S2][0] =
                                                sx((c.g[S5][0] as u32).wrapping_add(0x702));
                                            c.g[S0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                            c.g[S3][0] = c.g[S2][0];
                                            c.g[S6][0] = 0u64;
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[RA][0] = sx(0x001b7f88);
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            call_ptr(c, 0x00317670, 0x001b7f88)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A0][0] = c.g[S0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001b7f98);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, t, 0x001b7f98)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A2][0] = c.g[SP][0];
                                            c.g[A3][0] = 0x11;
                                            c.g[RA][0] = sx(0x001b7fac);
                                            c.g[A0][0] = c.g[S2][0];
                                            call_ptr(c, 0x002c26d0, 0x001b7fac)?;
                                            let t = true;
                                            c.g[A0][0] = 0x460000;
                                            if t {
                                                break 'b_1b812c;
                                            } else {
                                                break 'b_1b7fb4;
                                            }
                                        }
                                        'b_1b804c: {
                                            c.cycles += 4;
                                            c.g[V1][0] =
                                                c.lw((c.g[S5][0] as u32).wrapping_add(0x6dc));
                                            c.g[V0][0] = 0x3;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            if t {
                                                break 'b_1b804c;
                                            } else {
                                                c.cycles += 34;
                                                c.g[A0][0] = 0x460000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x42e0));
                                                c.g[S3][0] =
                                                    sx((c.g[S5][0] as u32).wrapping_add(0x702));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                c.g[S4][0] = c.g[S3][0];
                                                c.g[S6][0] = 0u64;
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[RA][0] = sx(0x001b7fec);
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                call_ptr(c, 0x00317670, 0x001b7fec)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A1][0] = c.g[V0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001b7ffc);
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, t, 0x001b7ffc)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                c.g[S2][0] = c.g[V0][0];
                                                c.g[A0][0] = 0x460000;
                                                c.g[S0][0] =
                                                    c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x44c0));
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[RA][0] = sx(0x001b8020);
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                call_ptr(c, 0x00317670, 0x001b8020)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001b8030);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, t, 0x001b8030)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[A2][0] = c.g[S2][0];
                                                c.g[A0][0] = c.g[S3][0];
                                                c.g[RA][0] = sx(0x001b8044);
                                                c.g[A3][0] = c.g[SP][0];
                                                call_ptr(c, 0x002c26d0, 0x001b8044)?;
                                                let t = true;
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_1b8128;
                                                } else {
                                                    break 'b_1b804c;
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001b8054);
                                        c.g[A0][0] = c.g[S5][0];
                                        Callee {
                                            addr: 0x001b83c0,
                                            f: ee::s03::cFEStateNetAccountMem_FormatFailedText,
                                        }
                                        .call(c, 0x001b8054)?;
                                        let t = true;
                                        c.g[S4][0] = sx((c.g[S5][0] as u32).wrapping_add(0x702));
                                        if t {
                                            break 'b_1b8124;
                                        } else {
                                            break 'b_1b805c;
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S5][0];
                                    c.g[RA][0] = sx(0x001b8068);
                                    c.g[S3][0] = sx((c.g[S5][0] as u32).wrapping_add(0x702));
                                    Callee {
                                        addr: 0x001b83c0,
                                        f: ee::s03::cFEStateNetAccountMem_FormatFailedText,
                                    }
                                    .call(c, 0x001b8068)?;
                                    let t = true;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1b812c;
                                    } else {
                                        break 'b_1b8070;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e0));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                c.g[V0][0] = c.g[V0][0] ^ 0x16;
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_1b814c;
                                } else {
                                    'b_1b80e0: {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001b808c);
                                        call_ptr(c, 0x00265d68, 0x001b808c)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        if t {
                                            break 'b_1b80e0;
                                        } else {
                                            c.cycles += 19;
                                            c.g[A0][0] = 0x460000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x44d8));
                                            c.g[S2][0] =
                                                sx((c.g[S5][0] as u32).wrapping_add(0x702));
                                            c.g[S0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                            c.g[S4][0] = c.g[S2][0];
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[RA][0] = sx(0x001b80b8);
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            call_ptr(c, 0x00317670, 0x001b80b8)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A0][0] = c.g[S0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001b80c8);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, t, 0x001b80c8)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x001b80d8);
                                            c.g[A2][0] = c.g[SP][0];
                                            call_ptr(c, 0x002c26d0, 0x001b80d8)?;
                                            let t = true;
                                            c.g[A0][0] = 0x460000;
                                            if t {
                                                break 'b_1b8128;
                                            } else {
                                                break 'b_1b80e0;
                                            }
                                        }
                                    }
                                    c.cycles += 17;
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x44f0));
                                    c.g[S2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x702));
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    c.g[S4][0] = c.g[S2][0];
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[RA][0] = sx(0x001b8104);
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    call_ptr(c, 0x00317670, 0x001b8104)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001b8114);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x001b8114)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001b8124);
                                    c.g[A2][0] = c.g[SP][0];
                                    call_ptr(c, 0x002c26d0, 0x001b8124)?;
                                    break 'b_1b8124;
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = 0x460000;
                            break 'b_1b8128;
                        }
                        c.cycles += 1;
                        c.g[S3][0] = c.g[S4][0];
                        break 'b_1b812c;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2328));
                    break 'b_1b8130;
                }
                c.cycles += 7;
                c.g[RA][0] = sx(0x001b8138);
                call_ptr(c, 0x00317670, 0x001b8138)?;
                c.g[A0][0] = 0x460000;
                c.g[S0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001b8148);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x820));
                call_ptr(c, 0x00317670, 0x001b8148)?;
                c.g[S1][0] = c.g[V0][0];
                break 'b_1b814c;
            }
        }
        'b_1b81c8: {
            c.cycles += 2;
            let t = c.g[S6][0] == 0u64;
            c.g[S2][0] = 0xffffffffffffffff;
            if t {
                break 'b_1b81c8;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x6d8));
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if t {
                    break 'b_1b81c8;
                } else {
                    c.cycles += 26;
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4508));
                    c.g[S2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x702));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S3][0] = c.g[S2][0];
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001b8184);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    call_ptr(c, 0x00317670, 0x001b8184)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001b8194);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x001b8194)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001b81a4);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x002c26d0, 0x001b81a4)?;
                    c.g[A0][0] = 0x460000;
                    c.g[RA][0] = sx(0x001b81b0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2328));
                    call_ptr(c, 0x00317670, 0x001b81b0)?;
                    c.g[A0][0] = 0x460000;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001b81c0);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x820));
                    call_ptr(c, 0x00317670, 0x001b81c0)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[S2][0] = 0xffffffffffffffff;
                    break 'b_1b81c8;
                }
            }
        }
        'b_1b81ec: {
            c.cycles += 4;
            c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x6dc));
            c.g[V0][0] = 0x17;
            let t = c.g[V1][0] != c.g[V0][0];
            if c.g[S1][0] == 0 {
                c.g[S2][0] = 0u64;
            }
            if t {
                break 'b_1b81ec;
            } else {
                c.cycles += 5;
                c.g[A0][0] = 0x460000;
                c.g[S1][0] = 0u64;
                c.g[RA][0] = sx(0x001b81e8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x820));
                call_ptr(c, 0x00317670, 0x001b81e8)?;
                c.g[S0][0] = c.g[V0][0];
                break 'b_1b81ec;
            }
        }
        c.cycles += 18;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[S1][0];
        c.g[T2][0] = c.g[S3][0];
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x001b820c);
        c.g[T1][0] = 0u64;
        Callee {
            addr: 0x001b7650,
            f: ee::s03::cFEStateNetAccountMem_ShowPopup,
        }
        .call(c, 0x001b820c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateNetAccountMem_FormatNotDetectedText` (0x001b8238): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_net_account_mem_format_not_detected_text(c: &mut Cpu) {
    if c.pc != 0x001b8238 {
        return ee::s03::f_001b8238__generated(c);
    }
    guest(c, |c| {
        'b_1b83a4: {
            'b_1b83a0: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.g[A1][0] = 0x4a0000;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
                c.g[S3][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A0][0] = c.g[SP][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[RA][0] = sx(0x001b8268);
                c.g[A2][0] = 0x1;
                call_ptr(c, 0x002c2718, 0x001b8268)?;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6dc));
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3));
                c.g[V1][0] = (c.g[A0][0] < 0x1au64) as u64;
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x460000;
                if t {
                    break 'b_1b83a0;
                } else {
                    'b_1b8360: {
                        'b_1b8314: {
                            c.cycles += 6;
                            c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x45a0));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                            let t = c.g[A0][0] as u32;
                            match t {
                                0x001b8294 => {
                                    c.cycles += 32;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x42e0));
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[RA][0] = sx(0x001b82b4);
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    call_ptr(c, 0x00317670, 0x001b82b4)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A1][0] = c.g[V0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001b82c4);
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, t, 0x001b82c4)?;
                                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[S2][0] = c.g[V0][0];
                                    c.g[A0][0] = 0x460000;
                                    c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f20));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[RA][0] = sx(0x001b82e8);
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    call_ptr(c, 0x00317670, 0x001b82e8)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001b82f8);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x001b82f8)?;
                                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x702));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A3][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001b830c);
                                    c.g[A2][0] = 0x1;
                                    call_ptr(c, 0x002c26d0, 0x001b830c)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                                    if t {
                                        break 'b_1b83a4;
                                    } else {
                                        break 'b_1b8314;
                                    }
                                }
                                0x001b8314 => {
                                    break 'b_1b8314;
                                }
                                0x001b8360 => {
                                    break 'b_1b8360;
                                }
                                0x001b83a0 => {
                                    break 'b_1b83a0;
                                }
                                _ => {
                                    c.jump_out(t, 0x001b8238, 0x001b83bc);
                                    return Ok(());
                                }
                            }
                        }
                        c.cycles += 19;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4588));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x001b8334);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        call_ptr(c, 0x00317670, 0x001b8334)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001b8344);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, t, 0x001b8344)?;
                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x702));
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A2][0] = 0x1;
                        c.g[RA][0] = sx(0x001b8358);
                        c.g[A3][0] = 0x11;
                        call_ptr(c, 0x002c26d0, 0x001b8358)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                        if t {
                            break 'b_1b83a4;
                        } else {
                            break 'b_1b8360;
                        }
                    }
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x460000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x44d8));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001b8380);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    call_ptr(c, 0x00317670, 0x001b8380)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001b8390);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x001b8390)?;
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x702));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001b83a0);
                    c.g[A2][0] = c.g[SP][0];
                    call_ptr(c, 0x002c26d0, 0x001b83a0)?;
                    break 'b_1b83a0;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_1b83a4;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001b_5d58,
        c_fe_state_net_config_on_child_event,
        "cFEStateNetConfig_OnChildEvent",
    ),
    (
        0x001b_7cb8,
        c_fe_state_net_account_mem_update,
        "cFEStateNetAccountMem_Update",
    ),
    (
        0x001b_7e40,
        c_fe_state_net_account_mem_show_error_popup,
        "cFEStateNetAccountMem_ShowErrorPopup",
    ),
    (
        0x001b_8238,
        c_fe_state_net_account_mem_format_not_detected_text,
        "cFEStateNetAccountMem_FormatNotDetectedText",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the fakes below (a per-machine magic: fakes installed in
    /// the shared override slots run only for fixtures that set it).
    const OC912_CTRL: u32 = DATA + 0x20_0000;
    const OC912_MAGIC: u32 = 0x0c91_2a5e;
    /// The net-account-mem state object (`self`).
    const OC912_SELF: u32 = DATA + 0x1000;
    /// The object `*(gp + 0x2e0)` points at (the popup manager).
    const OC912_POPUP: u32 = DATA + 0x700;

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// the forced result of its mailbox (mode 1: `v0 = value`) unless it
    /// unwound.
    fn oc912_fake_call(c: &mut Cpu, idx: u32, addr: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc == ra {
            let mail = OC912_CTRL + 0x10 + 8 * idx;
            if c.bus.read32(mail) == 1 {
                let v = c.bus.read32(mail + 4);
                c.g[V0][0] = sx(v);
            }
        }
    }

    macro_rules! oc912_fakes {
        ($(($idx:expr, $fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC912_CTRL) != OC912_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc912_fake_call(c, $idx, $addr);
                }
            )*
            fn oc912_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc912_fakes!(
        (
            0,
            oc912_fake_001b5cf8,
            OC912_PREV_001B5CF8,
            ee::s03::OVERRIDE_001B5CF8,
            ee::s03::f_001b5cf8__generated,
            0x001b_5cf8
        ),
        (
            1,
            oc912_fake_001b55b0,
            OC912_PREV_001B55B0,
            ee::s03::OVERRIDE_001B55B0,
            ee::s03::f_001b55b0__generated,
            0x001b_55b0
        ),
        (
            2,
            oc912_fake_001b5740,
            OC912_PREV_001B5740,
            ee::s03::OVERRIDE_001B5740,
            ee::s03::f_001b5740__generated,
            0x001b_5740
        ),
        (
            3,
            oc912_fake_001b5830,
            OC912_PREV_001B5830,
            ee::s03::OVERRIDE_001B5830,
            ee::s03::f_001b5830__generated,
            0x001b_5830
        ),
        (
            4,
            oc912_fake_001b5a58,
            OC912_PREV_001B5A58,
            ee::s03::OVERRIDE_001B5A58,
            ee::s03::f_001b5a58__generated,
            0x001b_5a58
        ),
        (
            5,
            oc912_fake_001b4c40,
            OC912_PREV_001B4C40,
            ee::s03::OVERRIDE_001B4C40,
            ee::s03::f_001b4c40__generated,
            0x001b_4c40
        ),
        (
            6,
            oc912_fake_001b4c00,
            OC912_PREV_001B4C00,
            ee::s03::OVERRIDE_001B4C00,
            ee::s03::f_001b4c00__generated,
            0x001b_4c00
        ),
        (
            7,
            oc912_fake_001b5bd8,
            OC912_PREV_001B5BD8,
            ee::s03::OVERRIDE_001B5BD8,
            ee::s03::f_001b5bd8__generated,
            0x001b_5bd8
        ),
        (
            8,
            oc912_fake_001a97b8,
            OC912_PREV_001A97B8,
            ee::s03::OVERRIDE_001A97B8,
            ee::s03::f_001a97b8__generated,
            0x001a_97b8
        ),
        (
            9,
            oc912_fake_00265a68,
            OC912_PREV_00265A68,
            ee::s07::OVERRIDE_00265A68,
            ee::s07::f_00265a68__generated,
            0x0026_5a68
        ),
        (
            10,
            oc912_fake_00265d38,
            OC912_PREV_00265D38,
            ee::s07::OVERRIDE_00265D38,
            ee::s07::f_00265d38__generated,
            0x0026_5d38
        ),
        (
            11,
            oc912_fake_001b70d0,
            OC912_PREV_001B70D0,
            ee::s03::OVERRIDE_001B70D0,
            ee::s03::f_001b70d0__generated,
            0x001b_70d0
        ),
        (
            12,
            oc912_fake_001b77a8,
            OC912_PREV_001B77A8,
            ee::s03::OVERRIDE_001B77A8,
            ee::s03::f_001b77a8__generated,
            0x001b_77a8
        ),
        (
            13,
            oc912_fake_001b77c8,
            OC912_PREV_001B77C8,
            ee::s03::OVERRIDE_001B77C8,
            ee::s03::f_001b77c8__generated,
            0x001b_77c8
        ),
        (
            14,
            oc912_fake_001b7820,
            OC912_PREV_001B7820,
            ee::s03::OVERRIDE_001B7820,
            ee::s03::f_001b7820__generated,
            0x001b_7820
        ),
        (
            15,
            oc912_fake_001b78f0,
            OC912_PREV_001B78F0,
            ee::s03::OVERRIDE_001B78F0,
            ee::s03::f_001b78f0__generated,
            0x001b_78f0
        ),
        (
            16,
            oc912_fake_001b7938,
            OC912_PREV_001B7938,
            ee::s03::OVERRIDE_001B7938,
            ee::s03::f_001b7938__generated,
            0x001b_7938
        ),
        (
            17,
            oc912_fake_001b7a08,
            OC912_PREV_001B7A08,
            ee::s03::OVERRIDE_001B7A08,
            ee::s03::f_001b7a08__generated,
            0x001b_7a08
        ),
        (
            18,
            oc912_fake_001b7a50,
            OC912_PREV_001B7A50,
            ee::s03::OVERRIDE_001B7A50,
            ee::s03::f_001b7a50__generated,
            0x001b_7a50
        ),
        (
            19,
            oc912_fake_001b7a98,
            OC912_PREV_001B7A98,
            ee::s03::OVERRIDE_001B7A98,
            ee::s03::f_001b7a98__generated,
            0x001b_7a98
        ),
        (
            20,
            oc912_fake_001b7bc0,
            OC912_PREV_001B7BC0,
            ee::s03::OVERRIDE_001B7BC0,
            ee::s03::f_001b7bc0__generated,
            0x001b_7bc0
        ),
        (
            21,
            oc912_fake_002659e8,
            OC912_PREV_002659E8,
            ee::s07::OVERRIDE_002659E8,
            ee::s07::f_002659e8__generated,
            0x0026_59e8
        ),
        (
            22,
            oc912_fake_001b7c28,
            OC912_PREV_001B7C28,
            ee::s03::OVERRIDE_001B7C28,
            ee::s03::f_001b7c28__generated,
            0x001b_7c28
        ),
        (
            23,
            oc912_fake_001b7c70,
            OC912_PREV_001B7C70,
            ee::s03::OVERRIDE_001B7C70,
            ee::s03::f_001b7c70__generated,
            0x001b_7c70
        ),
        (
            24,
            oc912_fake_00317670,
            OC912_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            25,
            oc912_fake_002c2718,
            OC912_PREV_002C2718,
            ee::s08::OVERRIDE_002C2718,
            ee::s08::f_002c2718__generated,
            0x002c_2718
        ),
        (
            26,
            oc912_fake_00265cb0,
            OC912_PREV_00265CB0,
            ee::s07::OVERRIDE_00265CB0,
            ee::s07::f_00265cb0__generated,
            0x0026_5cb0
        ),
        (
            27,
            oc912_fake_002c26d0,
            OC912_PREV_002C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0
        ),
        (
            28,
            oc912_fake_001b8238,
            OC912_PREV_001B8238,
            ee::s03::OVERRIDE_001B8238,
            ee::s03::f_001b8238__generated,
            0x001b_8238
        ),
        (
            29,
            oc912_fake_001b83c0,
            OC912_PREV_001B83C0,
            ee::s03::OVERRIDE_001B83C0,
            ee::s03::f_001b83c0__generated,
            0x001b_83c0
        ),
        (
            30,
            oc912_fake_00265d68,
            OC912_PREV_00265D68,
            ee::s07::OVERRIDE_00265D68,
            ee::s07::f_00265d68__generated,
            0x0026_5d68
        ),
        (
            31,
            oc912_fake_001b7650,
            OC912_PREV_001B7650,
            ee::s03::OVERRIDE_001B7650,
            ee::s03::f_001b7650__generated,
            0x001b_7650
        ),
    );

    /// Mailbox index of a faked callee.
    fn oc912_mail_index(addr: u32) -> u32 {
        match addr {
            0x001b5cf8 => 0,
            0x001b55b0 => 1,
            0x001b5740 => 2,
            0x001b5830 => 3,
            0x001b5a58 => 4,
            0x001b4c40 => 5,
            0x001b4c00 => 6,
            0x001b5bd8 => 7,
            0x001a97b8 => 8,
            0x00265a68 => 9,
            0x00265d38 => 10,
            0x001b70d0 => 11,
            0x001b77a8 => 12,
            0x001b77c8 => 13,
            0x001b7820 => 14,
            0x001b78f0 => 15,
            0x001b7938 => 16,
            0x001b7a08 => 17,
            0x001b7a50 => 18,
            0x001b7a98 => 19,
            0x001b7bc0 => 20,
            0x002659e8 => 21,
            0x001b7c28 => 22,
            0x001b7c70 => 23,
            0x00317670 => 24,
            0x002c2718 => 25,
            0x00265cb0 => 26,
            0x002c26d0 => 27,
            0x001b8238 => 28,
            0x001b83c0 => 29,
            0x00265d68 => 30,
            0x001b7650 => 31,
            _ => panic!("no fake for {addr:#x}"),
        }
    }

    /// Make the fake of `addr` return `value` in `v0`.
    fn oc912_force(c: &mut Cpu, addr: u32, value: u32) {
        let mail = OC912_CTRL + 0x10 + 8 * oc912_mail_index(addr);
        c.bus.write32(mail, 1);
        c.bus.write32(mail + 4, value);
    }

    /// Scrambled machine with the game image (jump tables), the gated fakes
    /// on, and the world/list/interface chain the string lookups walk:
    /// `*(gp-0x848)+0x8c -> list`, `list+4 -> interface` (halfword at +0x20,
    /// method pointer at +0x24 left null: a call through the miss hook).
    fn oc912_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> crate::difftest::Rng {
        elf_image(c);
        let r = machine(c, seed, salt, calls);
        oc912_install_fakes();
        c.bus.write32(OC912_CTRL, OC912_MAGIC);
        let (world, list, iface) = (DATA + 0x400, DATA + 0x500, DATA + 0x600);
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x8c, list);
        c.bus.write32(list + 4, iface);
        c.bus.write32(iface + 0x20, 0x10);
        c.bus.write32(iface + 0x24, 0);
        c.bus.write32(GAME_GP + 0x2e0, OC912_POPUP);
        c.bus.write32(OC912_SELF + 0x6dc, 0);
        r
    }

    /// The popup-event switch of the net-config state: every `miMiscData`
    /// 0x80e..0x844 (nine handled ids, the rest falls to the base state)
    /// plus ids outside the table.
    #[test]
    fn c_fe_state_net_config_on_child_event_matches() {
        covered(
            ee::s03::f_001b5d58__generated,
            c_fe_state_net_config_on_child_event,
            120,
            &|c, seed| {
                let _r = oc912_machine(c, seed, 0x912_5d58, 2);
                let popup = DATA + 0x1100;
                let k = seed % 0x3c;
                let misc = if k >= 0x3a { 0x800 + k } else { 0x80e + k };
                c.bus.write32(popup + 0x18, misc);
                c.g[A0][0] = sx(OC912_SELF);
                c.g[A1][0] = sx(popup);
                c.g[A2][0] = sx(seed);
                c.pc = 0x001b_5d58;
            },
        );
    }

    /// The per-frame state switch of the account-memory state: every state
    /// 0..0x1f, the popup manager's state word zero and non-zero, and the
    /// "popup done" call returning zero and non-zero.
    #[test]
    fn c_fe_state_net_account_mem_update_matches() {
        covered(
            ee::s03::f_001b7cb8__generated,
            c_fe_state_net_account_mem_update,
            128,
            &|c, seed| {
                let _r = oc912_machine(c, seed, 0x912_7cb8, 3);
                c.bus.write32(OC912_SELF + 0x6dc, seed % 0x20);
                c.bus
                    .write32(OC912_POPUP + 4, [0, 5][(seed / 32 % 2) as usize]);
                oc912_force(c, 0x0026_5d38, [0, 1][(seed / 64 % 2) as usize]);
                c.g[A0][0] = sx(OC912_SELF);
                c.pc = 0x001b_7cb8;
            },
        );
    }

    /// The error-popup text chooser: every error code 0..0x1a, with the
    /// state flags, the popup manager's answers and the string-lookup
    /// results drawn from the seed.
    #[test]
    fn c_fe_state_net_account_mem_show_error_popup_matches() {
        covered(
            ee::s03::f_001b7e40__generated,
            c_fe_state_net_account_mem_show_error_popup,
            600,
            &|c, seed| {
                let mut r = oc912_machine(c, seed, 0x912_7e40, 6);
                let w = r.next_u32();
                c.bus.write32(OC912_SELF + 0xb04, w & 1);
                oc912_force(c, 0x0026_5cb0, if w & 2 != 0 { 0 } else { 1 });
                oc912_force(c, 0x0026_5d68, if w & 4 != 0 { 0 } else { 1 });
                c.bus
                    .write32(OC912_POPUP, if w & 8 != 0 { 0x16 } else { 7 });
                c.bus
                    .write32(OC912_SELF + 0x6dc, [3, 0x17, 0, 5][(w >> 5 & 3) as usize]);
                c.bus.write32(OC912_SELF + 0x6d8, w >> 7 & 1);
                oc912_force(c, 0x0031_7670, if w & 0x100 != 0 { 0 } else { 0x0012_3450 });
                c.g[A0][0] = sx(OC912_SELF);
                c.g[A1][0] = sx(seed % 27);
                c.pc = 0x001b_7e40;
            },
        );
    }

    /// The "memory card not detected" text: every state 0..33 (three text
    /// variants, the empty case and states outside the table).
    #[test]
    fn c_fe_state_net_account_mem_format_not_detected_text_matches() {
        covered(
            ee::s03::f_001b8238__generated,
            c_fe_state_net_account_mem_format_not_detected_text,
            68,
            &|c, seed| {
                let _r = oc912_machine(c, seed, 0x912_8238, 4);
                c.bus.write32(OC912_SELF + 0x6dc, seed % 34);
                c.g[A0][0] = sx(OC912_SELF);
                c.pc = 0x001b_8238;
            },
        );
    }
}
