//! Staging module oc911 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc911`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `cFEStateChatRoom_OnBuddyActionEvent` (0x001b35b0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_chat_room_on_buddy_action_event(c: &mut Cpu) {
    if c.pc != 0x001b35b0 {
        return ee::s03::f_001b35b0__generated(c);
    }
    guest(c, |c| {
        'b_1b3744: {
            'b_1b3730: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = c.g[A2][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0xf));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[S3][0] = c.g[A1][0];
                if t {
                    break 'b_1b3730;
                } else {
                    'b_1b36d4: {
                        'b_1b36c4: {
                            'b_1b36c0: {
                                'b_1b36b0: {
                                    'b_1b3688: {
                                        'b_1b3658: {
                                            'b_1b3624: {
                                                c.cycles += 7;
                                                c.g[V0][0] = 0x460000;
                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x3da0));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x001b35fc => {
                                                        c.cycles += 10;
                                                        c.g[RA][0] = sx(0x001b3604);
                                                        c.g[A0][0] = c.g[S1][0];
                                                        Callee {
                                                            addr: 0x001a98b0,
                                                            f: ee::s03::NetFEState_GetSelectedUser,
                                                        }
                                                        .call(c, 0x001b3604)?;
                                                        c.g[A0][0] = c.g[S1][0];
                                                        c.g[A1][0] = c.g[V0][0];
                                                        c.g[A2][0] = c.g[S3][0];
                                                        c.g[A3][0] = c.g[S2][0];
                                                        c.g[RA][0] = sx(0x001b361c);
                                                        c.g[T0][0] = 0u64;
                                                        Callee { addr: 0x001abcd0, f: ee::s03::NetFEState_OpenChallengePopup }.call(c, 0x001b361c)?;
                                                        let t = true;
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x50),
                                                        );
                                                        if t {
                                                            break 'b_1b3744;
                                                        } else {
                                                            break 'b_1b3624;
                                                        }
                                                    }
                                                    0x001b3624 => {
                                                        break 'b_1b3624;
                                                    }
                                                    0x001b3658 => {
                                                        break 'b_1b3658;
                                                    }
                                                    0x001b3688 => {
                                                        break 'b_1b3688;
                                                    }
                                                    0x001b36d4 => {
                                                        break 'b_1b36d4;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x001b35b0, 0x001b375c);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 13;
                                            c.g[A1][0] = c.g[S3][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x001b3634);
                                            c.g[A0][0] = c.g[S1][0];
                                            Callee {
                                                addr: 0x001a97b8,
                                                f: ee::s03::NetFEState_OnChildEvent,
                                            }
                                            .call(c, 0x001b3634)?;
                                            c.g[RA][0] = sx(0x001b363c);
                                            c.g[A0][0] = c.g[S1][0];
                                            Callee {
                                                addr: 0x001a98b0,
                                                f: ee::s03::NetFEState_GetSelectedUser,
                                            }
                                            .call(c, 0x001b363c)?;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A2][0] = c.g[S3][0];
                                            c.g[RA][0] = sx(0x001b3650);
                                            c.g[A3][0] = c.g[S2][0];
                                            Callee {
                                                addr: 0x001abb30,
                                                f: ee::s03::NetFEState_ReadOrSendMessage,
                                            }
                                            .call(c, 0x001b3650)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_1b3744;
                                            } else {
                                                break 'b_1b3658;
                                            }
                                        }
                                        c.cycles += 10;
                                        c.g[RA][0] = sx(0x001b3660);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001a98b0,
                                            f: ee::s03::NetFEState_GetSelectedUser,
                                        }
                                        .call(c, 0x001b3660)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x001b3670);
                                        c.g[A2][0] = c.g[SP][0];
                                        Callee {
                                            addr: 0x001abe58,
                                            f: ee::s03::NetFEState_ToggleMessengerBuddy,
                                        }
                                        .call(c, 0x001b3670)?;
                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32));
                                        c.g[V0][0] = 0x1;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[A0][0] = c.g[S1][0];
                                        if t {
                                            break 'b_1b36b0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A1][0] = c.g[S3][0];
                                            if t {
                                                break 'b_1b36c4;
                                            } else {
                                                break 'b_1b3688;
                                            }
                                        }
                                    }
                                    c.cycles += 10;
                                    c.g[RA][0] = sx(0x001b3690);
                                    c.g[A0][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x001a98b0,
                                        f: ee::s03::NetFEState_GetSelectedUser,
                                    }
                                    .call(c, 0x001b3690)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x001b36a0);
                                    c.g[A2][0] = c.g[SP][0] | 0x4;
                                    Callee {
                                        addr: 0x001abc50,
                                        f: ee::s03::NetFEState_ToggleBlockUser,
                                    }
                                    .call(c, 0x001b36a0)?;
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = 0x1;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[A0][0] = c.g[S1][0];
                                    if t {
                                        break 'b_1b36c0;
                                    } else {
                                        break 'b_1b36b0;
                                    }
                                }
                                c.cycles += 4;
                                c.g[RA][0] = sx(0x001b36b8);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001abed8,
                                    f: ee::s03::NetFEState_ShowMaxBuddiesPopup,
                                }
                                .call(c, 0x001b36b8)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                if t {
                                    break 'b_1b3744;
                                } else {
                                    break 'b_1b36c0;
                                }
                            }
                            c.cycles += 1;
                            c.g[A1][0] = c.g[S3][0];
                            break 'b_1b36c4;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001b36cc);
                        c.g[A2][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001a97b8,
                            f: ee::s03::NetFEState_OnChildEvent,
                        }
                        .call(c, 0x001b36cc)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        if t {
                            break 'b_1b3744;
                        } else {
                            break 'b_1b36d4;
                        }
                    }
                    c.cycles += 23;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x6c4), 0u64);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001b36e4);
                    c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    Callee {
                        addr: 0x001a98b0,
                        f: ee::s03::NetFEState_GetSelectedUser,
                    }
                    .call(c, 0x001b36e4)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001b36f0);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0025df00, 0x001b36f0)?;
                    c.g[RA][0] = sx(0x001b36f8);
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x001a9c50,
                        f: ee::s03::NetFEState_GetPopup,
                    }
                    .call(c, 0x001b36f8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 0x460000;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), 0u64);
                    c.g[RA][0] = sx(0x001b370c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3d78));
                    call_ptr(c, 0x00317670, 0x001b370c)?;
                    c.g[V1][0] = 0x3;
                    c.g[A2][0] = 0x805;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x164), 0u64);
                    c.g[A1][0] = c.g[S0][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                    c.g[A0][0] = c.g[S1][0];
                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                    c.g[RA][0] = sx(0x001b3730);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[A2][0]);
                    Callee {
                        addr: 0x001a9bc0,
                        f: ee::s03::NetFEState_ShowPopup,
                    }
                    .call(c, 0x001b3730)?;
                    break 'b_1b3730;
                }
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001b3740);
            c.g[A2][0] = c.g[S2][0];
            Callee {
                addr: 0x001a97b8,
                f: ee::s03::NetFEState_OnChildEvent,
            }
            .call(c, 0x001b3740)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_1b3744;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateChatRoom_v31` (0x001b3a30): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_chat_room_v31(c: &mut Cpu) {
    if c.pc != 0x001b3a30 {
        return ee::s03::f_001b3a30__generated(c);
    }
    guest(c, |c| 'l_1b3a30: loop {
        'b_1b3bdc: {
            'b_1b3bd8: {
                'b_1b3ba8: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S0][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xdc));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                    c.g[S2][0] = c.g[A0][0];
                    c.g[V0][0] = (c.g[V1][0] < 0x30u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = c.g[A2][0];
                    if t {
                        break 'b_1b3ba8;
                    } else {
                        'b_1b3b88: {
                            'b_1b3b78: {
                                'b_1b3b74: {
                                    'b_1b3b54: {
                                        'b_1b3aec: {
                                            'b_1b3aa4: {
                                                c.cycles += 7;
                                                c.g[V0][0] = 0x460000;
                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x3de0));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x001b3a30 => {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001b3a30);
                                                        }
                                                        continue 'l_1b3a30;
                                                    }
                                                    0x001b3a78 => {
                                                        c.cycles += 11;
                                                        c.g[A0][0] = c.g[SP][0];
                                                        c.g[RA][0] = sx(0x001b3a84);
                                                        c.g[A1][0] =
                                                            sx((c.g[S2][0] as u32)
                                                                .wrapping_add(0x70c));
                                                        call_ptr(c, 0x004166f4, 0x001b3a84)?;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        c.g[RA][0] = sx(0x001b3a90);
                                                        c.g[A1][0] = 0x1;
                                                        Callee { addr: 0x001b2378, f: ee::s03::cFEStateChatRoom_RefreshPlayerList }.call(c, 0x001b3a90)?;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        c.g[RA][0] = sx(0x001b3a9c);
                                                        c.g[A1][0] = c.g[SP][0];
                                                        Callee { addr: 0x001b3978, f: ee::s03::cFEStateChatRoom_SelectPlayerByName }.call(c, 0x001b3a9c)?;
                                                        let t = true;
                                                        c.g[V0][0] = 0u64;
                                                        if t {
                                                            break 'b_1b3bd8;
                                                        } else {
                                                            break 'b_1b3aa4;
                                                        }
                                                    }
                                                    0x001b3aa4 => {
                                                        break 'b_1b3aa4;
                                                    }
                                                    0x001b3aec => {
                                                        break 'b_1b3aec;
                                                    }
                                                    0x001b3b54 => {
                                                        break 'b_1b3b54;
                                                    }
                                                    0x001b3b74 => {
                                                        break 'b_1b3b74;
                                                    }
                                                    0x001b3b88 => {
                                                        break 'b_1b3b88;
                                                    }
                                                    0x001b3ba8 => {
                                                        break 'b_1b3ba8;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x001b3a30, 0x001b3bf0);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001b3aac);
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                            call_ptr(c, 0x0025da90, 0x001b3aac)?;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A0][0] = c.g[S2][0];
                                            if t {
                                                break 'b_1b3b78;
                                            } else {
                                                c.cycles += 14;
                                                c.g[A1][0] = 0x460000;
                                                c.g[A0][0] = 0xb84;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1b30));
                                                c.g[A2][0] = 0u64;
                                                c.g[RA][0] = sx(0x001b3acc);
                                                c.g[A3][0] = 0u64;
                                                call_ptr(c, 0x00317d70, 0x001b3acc)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                                c.g[RA][0] = sx(0x001b3ad8);
                                                c.g[A0][0] = c.g[V0][0];
                                                Callee {
                                                    addr: 0x001bcb58,
                                                    f: ee::s03::cFEStateLobbyMenu_ctor,
                                                }
                                                .call(c, 0x001b3ad8)?;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[RA][0] = sx(0x001b3ae4);
                                                c.g[A1][0] = c.g[V0][0];
                                                Callee {
                                                    addr: 0x001a8a40,
                                                    f: ee::s03::NetFEState_GotoNextState,
                                                }
                                                .call(c, 0x001b3ae4)?;
                                                let t = true;
                                                c.g[V0][0] = 0u64;
                                                if t {
                                                    break 'b_1b3bd8;
                                                } else {
                                                    break 'b_1b3aec;
                                                }
                                            }
                                        }
                                        c.cycles += 26;
                                        c.g[RA][0] = sx(0x001b3af4);
                                        c.g[A0][0] = c.g[S2][0];
                                        Callee {
                                            addr: 0x001a9c50,
                                            f: ee::s03::NetFEState_GetPopup,
                                        }
                                        .call(c, 0x001b3af4)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A0][0] = 0x460000;
                                        c.g[V0][0] = 0x1;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3dc8));
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001b3b10);
                                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                        call_ptr(c, 0x00317670, 0x001b3b10)?;
                                        c.g[V1][0] = 0x3;
                                        c.g[A0][0] = 0x460000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                        c.g[RA][0] = sx(0x001b3b28);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                        call_ptr(c, 0x00317670, 0x001b3b28)?;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001b3b38);
                                        c.g[A1][0] = 0u64;
                                        Callee {
                                            addr: 0x001c57c0,
                                            f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                        }
                                        .call(c, 0x001b3b38)?;
                                        c.g[V0][0] = 0x804;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001b3b4c);
                                        c.g[A1][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x001a9bc0,
                                            f: ee::s03::NetFEState_ShowPopup,
                                        }
                                        .call(c, 0x001b3b4c)?;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_1b3bd8;
                                        } else {
                                            break 'b_1b3b54;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6f8));
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_1b3bd8;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x001b3b6c);
                                        c.g[A1][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x001b2cd0,
                                            f: ee::s03::cFEStateChatRoom_AddChatLine,
                                        }
                                        .call(c, 0x001b3b6c)?;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_1b3bd8;
                                        } else {
                                            break 'b_1b3b74;
                                        }
                                    }
                                }
                                c.cycles += 1;
                                c.g[A0][0] = c.g[S2][0];
                                break 'b_1b3b78;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001b3b80);
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x001b2378,
                                f: ee::s03::cFEStateChatRoom_RefreshPlayerList,
                            }
                            .call(c, 0x001b3b80)?;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_1b3bd8;
                            } else {
                                break 'b_1b3b88;
                            }
                        }
                        c.cycles += 8;
                        c.g[RA][0] = sx(0x001b3b90);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001a98b0,
                            f: ee::s03::NetFEState_GetSelectedUser,
                        }
                        .call(c, 0x001b3b90)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001b3ba0);
                        c.g[A2][0] = 0x1;
                        Callee {
                            addr: 0x001ab3d0,
                            f: ee::s03::NetFEState_RefreshBuddyActionPopup,
                        }
                        .call(c, 0x001b3ba0)?;
                        let t = true;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_1b3bd8;
                        } else {
                            break 'b_1b3ba8;
                        }
                    }
                }
                c.cycles += 12;
                c.g[RA][0] = sx(0x001b3bb0);
                c.g[A0][0] = c.g[S2][0];
                Callee {
                    addr: 0x001a98b0,
                    f: ee::s03::NetFEState_GetSelectedUser,
                }
                .call(c, 0x001b3bb0)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001b3bc0);
                c.g[A2][0] = 0x1;
                Callee {
                    addr: 0x001ab3d0,
                    f: ee::s03::NetFEState_RefreshBuddyActionPopup,
                }
                .call(c, 0x001b3bc0)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001b3bd0);
                c.g[A2][0] = c.g[S1][0];
                Callee {
                    addr: 0x001a8e40,
                    f: ee::s03::NetFEState_HandleNetEvent,
                }
                .call(c, 0x001b3bd0)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                if t {
                    break 'b_1b3bdc;
                } else {
                    break 'b_1b3bd8;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_1b3bdc;
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

/// `cFEStateNetConfig_Update` (0x001b41a8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_net_config_update(c: &mut Cpu) {
    if c.pc != 0x001b41a8 {
        return ee::s03::f_001b41a8__generated(c);
    }
    guest(c, |c| 'l_1b41a8: loop {
        'b_1b41d4: {
            c.cycles += 6;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            let t = c.g[A1][0] == 0u64;
            c.g[S0][0] = c.g[A0][0];
            if t {
                break 'b_1b41d4;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001b41d4);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001b41d4)?;
                break 'b_1b41d4;
            }
        }
        'b_1b4390: {
            c.cycles += 4;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x708));
            c.g[V0][0] = (c.g[V1][0] < 0x9u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x460000;
            if t {
                break 'b_1b4390;
            } else {
                'b_1b4368: {
                    'b_1b4334: {
                        'b_1b4328: {
                            'b_1b42e8: {
                                'b_1b427c: {
                                    'b_1b426c: {
                                        'b_1b423c: {
                                            'b_1b4208: {
                                                c.cycles += 6;
                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x3ee0));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x001b41a8 => {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001b41a8);
                                                        }
                                                        continue 'l_1b41a8;
                                                    }
                                                    0x001b41fc => {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = 0x2;
                                                        let t = true;
                                                        c.sw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x708),
                                                            c.g[V0][0],
                                                        );
                                                        if t {
                                                            break 'b_1b4390;
                                                        } else {
                                                            break 'b_1b4208;
                                                        }
                                                    }
                                                    0x001b4208 => {
                                                        break 'b_1b4208;
                                                    }
                                                    0x001b423c => {
                                                        break 'b_1b423c;
                                                    }
                                                    0x001b426c => {
                                                        break 'b_1b426c;
                                                    }
                                                    0x001b427c => {
                                                        break 'b_1b427c;
                                                    }
                                                    0x001b42e8 => {
                                                        break 'b_1b42e8;
                                                    }
                                                    0x001b4328 => {
                                                        break 'b_1b4328;
                                                    }
                                                    0x001b4334 => {
                                                        break 'b_1b4334;
                                                    }
                                                    0x001b4368 => {
                                                        break 'b_1b4368;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x001b41a8, 0x001b43a8);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 8;
                                            c.g[A1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
                                            c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001b4220);
                                            c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001b4220)?;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
                                            if t {
                                                break 'b_1b4390;
                                            } else {
                                                c.cycles += 5;
                                                c.g[RA][0] = sx(0x001b4230);
                                                c.g[A1][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x1570));
                                                call_ptr(c, 0x002c67e8, 0x001b4230)?;
                                                c.g[V0][0] = 0x3;
                                                let t = true;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x708),
                                                    c.g[V0][0],
                                                );
                                                if t {
                                                    break 'b_1b4390;
                                                } else {
                                                    break 'b_1b423c;
                                                }
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
                                        c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001b4254);
                                        c.g[A0][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001b4254)?;
                                        let t = c.g[V0][0] != 0u64;

                                        if t {
                                            break 'b_1b4390;
                                        } else {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001b4264);
                                            c.g[A0][0] = c.g[S0][0];
                                            Callee {
                                                addr: 0x001b4470,
                                                f: ee::s03::cFEStateNetConfig_ScanNextCard,
                                            }
                                            .call(c, 0x001b4264)?;
                                            let t = true;

                                            if t {
                                                break 'b_1b4390;
                                            } else {
                                                break 'b_1b426c;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001b4274);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001b43a8,
                                        f: ee::s03::cFEStateNetConfig_CheckCard,
                                    }
                                    .call(c, 0x001b4274)?;
                                    let t = true;

                                    if t {
                                        break 'b_1b4390;
                                    } else {
                                        break 'b_1b427c;
                                    }
                                }
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6e0));
                                c.g[V1][0] = c.g[V0][0];
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                let t = (c.g[V1][0] as i64) >= 0;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x6e0), c.g[V0][0]);
                                if t {
                                    break 'b_1b42e8;
                                } else {
                                    'b_1b42d8: {
                                        c.cycles += 8;
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
                                        c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001b42a8);
                                        c.g[A0][0] =
                                            sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001b42a8)?;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                            break 'b_1b42d8;
                                        } else {
                                            c.cycles += 10;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x001b42bc);
                                            c.g[A1][0] = 0xffffffffffffffff;
                                            Callee {
                                                addr: 0x001b5068,
                                                f: ee::s03::cFEStateNetConfig_SetListVisible,
                                            }
                                            .call(c, 0x001b42bc)?;
                                            c.g[A3][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x1570));
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x842;
                                            c.g[RA][0] = sx(0x001b42d0);
                                            c.g[A2][0] = 0x18;
                                            Callee {
                                                addr: 0x001b4888,
                                                f: ee::s03::cFEStateNetConfig_ShowCardErrorPopup,
                                            }
                                            .call(c, 0x001b42d0)?;
                                            let t = true;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x708), 0u64);
                                            if t {
                                                break 'b_1b4390;
                                            } else {
                                                break 'b_1b42d8;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001b42e0);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f698, 0x001b42e0)?;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x708), 0u64);
                                    if t {
                                        break 'b_1b4390;
                                    } else {
                                        break 'b_1b42e8;
                                    }
                                }
                            }
                            c.cycles += 8;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
                            c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001b4300);
                            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001b4300)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
                            if t {
                                break 'b_1b4390;
                            } else {
                                c.cycles += 8;
                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1570));
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x28));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001b4320);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001b4320)?;
                                let t = true;

                                if t {
                                    break 'b_1b4390;
                                } else {
                                    break 'b_1b4328;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 0x7;
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x708), c.g[V0][0]);
                        if t {
                            break 'b_1b4390;
                        } else {
                            break 'b_1b4334;
                        }
                    }
                    c.cycles += 8;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001b434c);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001b434c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
                    if t {
                        break 'b_1b4390;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001b435c);
                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1570));
                        call_ptr(c, 0x002c67e8, 0x001b435c)?;
                        c.g[V0][0] = 0x8;
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x708), c.g[V0][0]);
                        if t {
                            break 'b_1b4390;
                        } else {
                            break 'b_1b4368;
                        }
                    }
                }
                c.cycles += 8;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1578));
                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001b4380);
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001b4380)?;
                let t = c.g[V0][0] != 0u64;

                if t {
                    break 'b_1b4390;
                } else {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x001b4390);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001b45e8,
                        f: ee::s03::cFEStateNetConfig_StartConnect,
                    }
                    .call(c, 0x001b4390)?;
                    break 'b_1b4390;
                }
            }
        }
        c.cycles += 6;
        c.g[RA][0] = sx(0x001b4398);
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x001a8770,
            f: ee::s03::NetFEState_Update,
        }
        .call(c, 0x001b4398)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
    });
}

/// `cFEStateNetConfig_OnLoginPopupEvent` (0x001b5a58): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_net_config_on_login_popup_event(c: &mut Cpu) {
    if c.pc != 0x001b5a58 {
        return ee::s03::f_001b5a58__generated(c);
    }
    guest(c, |c| {
        'b_1b5bc0: {
            'b_1b5bbc: {
                'b_1b5bac: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S2][0] = c.g[A2][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0xf));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = c.g[A1][0];
                    if t {
                        break 'b_1b5bac;
                    } else {
                        'b_1b5b94: {
                            'b_1b5b5c: {
                                'b_1b5ac4: {
                                    'b_1b5aa8: {
                                        c.cycles += 7;
                                        c.g[V0][0] = 0x460000;
                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40b0));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                        let t = c.g[A0][0] as u32;
                                        match t {
                                            0x001b5aa0 => {
                                                c.cycles += 2;
                                                let t = true;
                                                c.sw((c.g[S0][0] as u32).wrapping_add(0x6dc), 0u64);
                                                if t {
                                                    break 'b_1b5b94;
                                                } else {
                                                    break 'b_1b5aa8;
                                                }
                                            }
                                            0x001b5aa8 => {
                                                break 'b_1b5aa8;
                                            }
                                            0x001b5ac4 => {
                                                break 'b_1b5ac4;
                                            }
                                            0x001b5b5c => {
                                                break 'b_1b5b5c;
                                            }
                                            0x001b5bac => {
                                                break 'b_1b5bac;
                                            }
                                            _ => {
                                                c.jump_out(t, 0x001b5a58, 0x001b5bd4);
                                                return Ok(());
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A2][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001b5ab8);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001a97b8,
                                        f: ee::s03::NetFEState_OnChildEvent,
                                    }
                                    .call(c, 0x001b5ab8)?;
                                    c.g[V0][0] = 0x1;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x6dc), c.g[V0][0]);
                                    if t {
                                        break 'b_1b5bbc;
                                    } else {
                                        break 'b_1b5ac4;
                                    }
                                }
                                'b_1b5ae8: {
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A2][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001b5ad4);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001a97b8,
                                        f: ee::s03::NetFEState_OnChildEvent,
                                    }
                                    .call(c, 0x001b5ad4)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    let t = c.g[V0][0] != 0u64;

                                    if t {
                                        break 'b_1b5ae8;
                                    } else {
                                        c.cycles += 2;
                                        c.g[RA][0] = sx(0x001b5ae8);
                                        call_ptr(c, 0x00260f80, 0x001b5ae8)?;
                                        break 'b_1b5ae8;
                                    }
                                }
                                'b_1b5b04: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001b5af0);
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    call_ptr(c, 0x00261460, 0x001b5af0)?;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    if t {
                                        break 'b_1b5b04;
                                    } else {
                                        c.cycles += 3;
                                        c.g[RA][0] = sx(0x001b5b00);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        call_ptr(c, 0x002613c8, 0x001b5b00)?;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        break 'b_1b5b04;
                                    }
                                }
                                'b_1b5b14: {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != 0u64;

                                    if t {
                                        break 'b_1b5b14;
                                    } else {
                                        c.cycles += 2;
                                        c.g[RA][0] = sx(0x001b5b14);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                        call_ptr(c, 0x0025b800, 0x001b5b14)?;
                                        break 'b_1b5b14;
                                    }
                                }
                                c.cycles += 18;
                                c.g[RA][0] = sx(0x001b5b1c);
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                call_ptr(c, 0x00261408, 0x001b5b1c)?;
                                c.g[RA][0] = sx(0x001b5b24);
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                call_ptr(c, 0x0025b800, 0x001b5b24)?;
                                c.g[RA][0] = sx(0x001b5b2c);
                                call_ptr(c, 0x002591b8, 0x001b5b2c)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001b5b38);
                                c.g[A1][0] = 0u64;
                                call_ptr(c, 0x002636f0, 0x001b5b38)?;
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6ec));
                                c.g[A1][0] = 0u64;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001b5b54);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001b5b54)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                if t {
                                    break 'b_1b5bc0;
                                } else {
                                    break 'b_1b5b5c;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6dc));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0x460000;
                            if t {
                                break 'b_1b5b94;
                            } else {
                                c.cycles += 11;
                                c.g[A0][0] = 0x6d4;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1cd8));
                                c.g[A2][0] = 0u64;
                                c.g[RA][0] = sx(0x001b5b7c);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x001b5b7c)?;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x001b5b88);
                                c.g[A0][0] = c.g[V0][0];
                                Callee {
                                    addr: 0x001bee30,
                                    f: ee::s03::cFEStateLogin_ctor,
                                }
                                .call(c, 0x001b5b88)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001b5b94);
                                c.g[A0][0] = c.g[S0][0];
                                Callee {
                                    addr: 0x001a8a40,
                                    f: ee::s03::NetFEState_GotoNextState,
                                }
                                .call(c, 0x001b5b94)?;
                                break 'b_1b5b94;
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001b5ba4);
                        c.g[A2][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001a97b8,
                            f: ee::s03::NetFEState_OnChildEvent,
                        }
                        .call(c, 0x001b5ba4)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        if t {
                            break 'b_1b5bc0;
                        } else {
                            break 'b_1b5bac;
                        }
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001b5bbc);
                c.g[A2][0] = c.g[S2][0];
                Callee {
                    addr: 0x001a97b8,
                    f: ee::s03::NetFEState_OnChildEvent,
                }
                .call(c, 0x001b5bbc)?;
                break 'b_1b5bbc;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1b5bc0;
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

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001b_35b0,
        c_fe_state_chat_room_on_buddy_action_event,
        "cFEStateChatRoom_OnBuddyActionEvent",
    ),
    (
        0x001b_3a30,
        c_fe_state_chat_room_v31,
        "cFEStateChatRoom_v31",
    ),
    (
        0x001b_41a8,
        c_fe_state_net_config_update,
        "cFEStateNetConfig_Update",
    ),
    (
        0x001b_5a58,
        c_fe_state_net_config_on_login_popup_event,
        "cFEStateNetConfig_OnLoginPopupEvent",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Control block (gate, forced results, toggle values), away from the fake
    /// call results (`DATA..DATA+0x40000`), the log and the pools.
    const CTRL: u32 = DATA + 0x20_0000;
    const MAGIC: u32 = 0x0c91_1a11;
    /// A vtable method that is itself a gated fake (so forced results apply).
    const METHOD: u32 = 0x0026_1408;

    /// One fake call (logs every register, results as `fakes::fake`), then the
    /// test's own results: v0 forced to zero for the calls the variant picks,
    /// and the out-values of the two toggle callees.
    fn fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        let a2 = c.g[A2][0] as u32;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        if addr == 0x001a_be58 {
            let v = c.bus.read32(CTRL + 0x20);
            c.bus.write32(a2, v);
        } else if addr == 0x001a_bc50 {
            let v = c.bus.read32(CTRL + 0x24);
            c.bus.write32(a2, v);
        }
        if n < 16 && c.bus.read32(CTRL + 0x100 + 4 * n) != 0 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc911_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(CTRL) != MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    fake_call(c, $addr);
                }
            )*
            fn oc911_install() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc911_fakes!(
        (
            fk_001a8770,
            PREV_001A8770,
            ee::s03::OVERRIDE_001A8770,
            ee::s03::f_001a8770__generated,
            0x001a_8770
        ),
        (
            fk_001a8a40,
            PREV_001A8A40,
            ee::s03::OVERRIDE_001A8A40,
            ee::s03::f_001a8a40__generated,
            0x001a_8a40
        ),
        (
            fk_001a8e40,
            PREV_001A8E40,
            ee::s03::OVERRIDE_001A8E40,
            ee::s03::f_001a8e40__generated,
            0x001a_8e40
        ),
        (
            fk_001a97b8,
            PREV_001A97B8,
            ee::s03::OVERRIDE_001A97B8,
            ee::s03::f_001a97b8__generated,
            0x001a_97b8
        ),
        (
            fk_001a98b0,
            PREV_001A98B0,
            ee::s03::OVERRIDE_001A98B0,
            ee::s03::f_001a98b0__generated,
            0x001a_98b0
        ),
        (
            fk_001a9bc0,
            PREV_001A9BC0,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a_9bc0
        ),
        (
            fk_001a9c50,
            PREV_001A9C50,
            ee::s03::OVERRIDE_001A9C50,
            ee::s03::f_001a9c50__generated,
            0x001a_9c50
        ),
        (
            fk_001ab3d0,
            PREV_001AB3D0,
            ee::s03::OVERRIDE_001AB3D0,
            ee::s03::f_001ab3d0__generated,
            0x001a_b3d0
        ),
        (
            fk_001abb30,
            PREV_001ABB30,
            ee::s03::OVERRIDE_001ABB30,
            ee::s03::f_001abb30__generated,
            0x001a_bb30
        ),
        (
            fk_001abc50,
            PREV_001ABC50,
            ee::s03::OVERRIDE_001ABC50,
            ee::s03::f_001abc50__generated,
            0x001a_bc50
        ),
        (
            fk_001abcd0,
            PREV_001ABCD0,
            ee::s03::OVERRIDE_001ABCD0,
            ee::s03::f_001abcd0__generated,
            0x001a_bcd0
        ),
        (
            fk_001abe58,
            PREV_001ABE58,
            ee::s03::OVERRIDE_001ABE58,
            ee::s03::f_001abe58__generated,
            0x001a_be58
        ),
        (
            fk_001abed8,
            PREV_001ABED8,
            ee::s03::OVERRIDE_001ABED8,
            ee::s03::f_001abed8__generated,
            0x001a_bed8
        ),
        (
            fk_001b2378,
            PREV_001B2378,
            ee::s03::OVERRIDE_001B2378,
            ee::s03::f_001b2378__generated,
            0x001b_2378
        ),
        (
            fk_001b2cd0,
            PREV_001B2CD0,
            ee::s03::OVERRIDE_001B2CD0,
            ee::s03::f_001b2cd0__generated,
            0x001b_2cd0
        ),
        (
            fk_001b3978,
            PREV_001B3978,
            ee::s03::OVERRIDE_001B3978,
            ee::s03::f_001b3978__generated,
            0x001b_3978
        ),
        (
            fk_001b43a8,
            PREV_001B43A8,
            ee::s03::OVERRIDE_001B43A8,
            ee::s03::f_001b43a8__generated,
            0x001b_43a8
        ),
        (
            fk_001b4470,
            PREV_001B4470,
            ee::s03::OVERRIDE_001B4470,
            ee::s03::f_001b4470__generated,
            0x001b_4470
        ),
        (
            fk_001b45e8,
            PREV_001B45E8,
            ee::s03::OVERRIDE_001B45E8,
            ee::s03::f_001b45e8__generated,
            0x001b_45e8
        ),
        (
            fk_001b4888,
            PREV_001B4888,
            ee::s03::OVERRIDE_001B4888,
            ee::s03::f_001b4888__generated,
            0x001b_4888
        ),
        (
            fk_001b5068,
            PREV_001B5068,
            ee::s03::OVERRIDE_001B5068,
            ee::s03::f_001b5068__generated,
            0x001b_5068
        ),
        (
            fk_001bcb58,
            PREV_001BCB58,
            ee::s03::OVERRIDE_001BCB58,
            ee::s03::f_001bcb58__generated,
            0x001b_cb58
        ),
        (
            fk_001bee30,
            PREV_001BEE30,
            ee::s03::OVERRIDE_001BEE30,
            ee::s03::f_001bee30__generated,
            0x001b_ee30
        ),
        (
            fk_001c57c0,
            PREV_001C57C0,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c_57c0
        ),
        (
            fk_0025b800,
            PREV_0025B800,
            ee::s06::OVERRIDE_0025B800,
            ee::s06::f_0025b800__generated,
            0x0025_b800
        ),
        (
            fk_0025da90,
            PREV_0025DA90,
            ee::s06::OVERRIDE_0025DA90,
            ee::s06::f_0025da90__generated,
            0x0025_da90
        ),
        (
            fk_0025df00,
            PREV_0025DF00,
            ee::s06::OVERRIDE_0025DF00,
            ee::s06::f_0025df00__generated,
            0x0025_df00
        ),
        (
            fk_00260f80,
            PREV_00260F80,
            ee::s06::OVERRIDE_00260F80,
            ee::s06::f_00260f80__generated,
            0x0026_0f80
        ),
        (
            fk_002613c8,
            PREV_002613C8,
            ee::s06::OVERRIDE_002613C8,
            ee::s06::f_002613c8__generated,
            0x0026_13c8
        ),
        (
            fk_00261408,
            PREV_00261408,
            ee::s06::OVERRIDE_00261408,
            ee::s06::f_00261408__generated,
            0x0026_1408
        ),
        (
            fk_00261460,
            PREV_00261460,
            ee::s06::OVERRIDE_00261460,
            ee::s06::f_00261460__generated,
            0x0026_1460
        ),
        (
            fk_002636f0,
            PREV_002636F0,
            ee::s07::OVERRIDE_002636F0,
            ee::s07::f_002636f0__generated,
            0x0026_36f0
        ),
        (
            fk_002c67e8,
            PREV_002C67E8,
            ee::s08::OVERRIDE_002C67E8,
            ee::s08::f_002c67e8__generated,
            0x002c_67e8
        ),
        (
            fk_00317670,
            PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            fk_00317d70,
            PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            fk_0039f698,
            PREV_0039F698,
            ee::s13::OVERRIDE_0039F698,
            ee::s13::f_0039f698__generated,
            0x0039_f698
        ),
        (
            fk_004166f4,
            PREV_004166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4
        ),
    );

    fn bit(var: u32, k: u32) -> u32 {
        (fakes::mix(var as u64 * 64 + k as u64) & 1) as u32
    }

    /// Machine for one seed: gated fakes, forced-zero results and toggle
    /// values from `var`, a jump table, `pc` at the entry.
    fn setup(
        c: &mut Cpu,
        seed: u32,
        entry: u32,
        calls: u32,
        var: u32,
        table: u32,
        labels: &[u32],
    ) -> Rng {
        oc911_install();
        let r = fakes::machine(c, seed, entry as u64, calls);
        c.bus.write32(CTRL, MAGIC);
        for n in 0..16 {
            c.bus
                .write32(CTRL + 0x100 + 4 * n, bit(var, n) & bit(var, n + 20));
        }
        c.bus.write32(CTRL + 0x20, bit(var, 40));
        c.bus.write32(CTRL + 0x24, bit(var, 41));
        for (i, l) in labels.iter().enumerate() {
            c.bus.write32(table + 4 * i as u32, *l);
        }
        c.pc = entry;
        r
    }

    #[test]
    fn c_fe_state_chat_room_on_buddy_action_event_matches() {
        const EVENTS: [u32; 8] = [0xf, 0x10, 0x11, 0x12, 0x13, 0xe, 0x14, 0xffff_fff0];
        check(
            ee::s03::f_001b35b0__generated,
            c_fe_state_chat_room_on_buddy_action_event,
            8 * 24,
            &|c, seed| {
                let (case, var) = (seed % 8, seed / 8);
                let mut r = setup(
                    c,
                    seed,
                    0x001b_35b0,
                    9,
                    var,
                    0x0046_3da0,
                    &[0x1b35fc, 0x1b3624, 0x1b3658, 0x1b3688, 0x1b36d4],
                );
                c.g[A0][0] = (CTRL + 0x1000) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(EVENTS[case as usize]);
            },
        );
    }

    #[test]
    fn c_fe_state_chat_room_v31_matches() {
        // Table entries 0..0x30 of `0x463de0`.
        let mut labels = [0x1b3ba8u32; 0x30];
        labels[0] = 0x1b3aa4;
        labels[2] = 0x1b3b54;
        labels[4] = 0x1b3a78;
        labels[5] = 0x1b3b74;
        labels[0xc] = 0x1b3aec;
        for i in [
            0xd, 0xe, 0xf, 0x10, 0x11, 0x12, 0x13, 0x2c, 0x2f, 0x2e, 0x2b,
        ] {
            labels[i] = 0x1b3b88;
        }
        const IDX: [u32; 14] = [
            0,
            1,
            2,
            3,
            4,
            5,
            0xc,
            0xd,
            0x13,
            0x2c,
            0x2f,
            0x30,
            0xffff_ffff,
            0xc,
        ];
        check(
            ee::s03::f_001b3a30__generated,
            c_fe_state_chat_room_v31,
            14 * 20,
            &|c, seed| {
                let (case, var) = (seed % 14, seed / 14);
                let mut r = setup(c, seed, 0x001b_3a30, 9, var, 0x0046_3de0, &labels);
                let this = CTRL + 0x1000;
                c.bus.write32(this + 0x6f8, bit(var, 50));
                c.g[A0][0] = this as u64;
                c.g[A1][0] = sx(0xdc_u32.wrapping_add(IDX[case as usize]));
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_net_config_update_matches() {
        const LABELS: [u32; 9] = [
            0x1b42e8, 0x1b41fc, 0x1b4208, 0x1b423c, 0x1b426c, 0x1b427c, 0x1b4328, 0x1b4334,
            0x1b4368,
        ];
        const STATES: [u32; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xffff_ffff];
        check(
            ee::s03::f_001b41a8__generated,
            c_fe_state_net_config_update,
            11 * 24,
            &|c, seed| {
                let (case, var) = (seed % 11, seed / 11);
                let _ = setup(c, seed, 0x001b_41a8, 9, var, 0x0046_3ee0, &LABELS);
                let this = CTRL + 0x1000;
                let obj = CTRL + 0x2000;
                let vt = CTRL + 0x3000;
                c.bus.write32(this + 0x708, STATES[case as usize]);
                c.bus
                    .write32(this + 0x6e0, [0xffff_ffff, 0, 5][(var % 3) as usize]);
                c.bus.write32(this + 0x10, CTRL + 0x4000);
                // The gp-relative object (an engine with a vtable), or none
                // (then the object pointer is read from address 0).
                let o = if bit(var, 60) != 0 && bit(var, 61) != 0 {
                    0
                } else {
                    obj
                };
                c.bus.write32(GAME_GP.wrapping_sub(0x1578), o);
                c.bus.write32(GAME_GP.wrapping_sub(0x1570), CTRL + 0x5000);
                c.bus.write32(if o == 0 { 0 } else { obj }, vt);
                for off in [0x18u32, 0x28, 0x58, 0x1a8] {
                    c.bus.write16(vt + off, 0);
                    c.bus.write32(vt + off + 4, METHOD);
                }
                c.g[A0][0] = this as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_net_config_on_login_popup_event_matches() {
        const EVENTS: [u32; 10] = [0xf, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0xe, 0x17];
        check(
            ee::s03::f_001b5a58__generated,
            c_fe_state_net_config_on_login_popup_event,
            10 * 24,
            &|c, seed| {
                let (case, var) = (seed % 10, seed / 10);
                let mut r = setup(
                    c,
                    seed,
                    0x001b_5a58,
                    12,
                    var,
                    0x0046_40b0,
                    &[
                        0x1b5aa8, 0x1b5ac4, 0x1b5bac, 0x1b5bac, 0x1b5bac, 0x1b5ac4, 0x1b5aa0,
                        0x1b5b5c,
                    ],
                );
                let this = CTRL + 0x1000;
                let obj = CTRL + 0x2000;
                let vt = CTRL + 0x3000;
                c.bus.write32(this + 0x10, CTRL + 0x4000);
                c.bus.write32(this + 0x6dc, bit(var, 70));
                c.bus.write32(this + 0x6ec, obj);
                c.bus.write32(obj + 8, vt);
                c.bus.write16(vt + 0x40, 0);
                c.bus.write32(vt + 0x44, METHOD);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x238),
                    if bit(var, 71) != 0 { CTRL + 0x6000 } else { 0 },
                );
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = sx(EVENTS[case as usize]);
            },
        );
    }
}
