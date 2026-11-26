//! Staging module oc910 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc910`.

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
        0x001a_fe68,
        c_fe_state_challenge_received_popup_init,
        "cFEStateChallengeReceivedPopup_Init",
    ),
    (
        0x001b_0708,
        c_fe_state_challenge_received_popup_v31,
        "cFEStateChallengeReceivedPopup_v31",
    ),
    (
        0x001b_15d8,
        c_fe_state_online_character_select_v31,
        "cFEStateOnlineCharacterSelect_v31",
    ),
    (
        0x001b_31a0,
        c_fe_state_chat_room_on_widget_action,
        "cFEStateChatRoom_OnWidgetAction",
    ),
];

/// `cFEStateChallengeReceivedPopup_Init` (0x001afe68): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_challenge_received_popup_init(c: &mut Cpu) {
    if c.pc != 0x001afe68 {
        return ee::s03::f_001afe68__generated(c);
    }
    guest(c, |c| {
        'b_1b00a0: {
            'b_1b009c: {
                c.cycles += 22;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S2][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x100));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x104));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001afe94);
                c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001afe94)?;
                c.g[A0][0] = 0x460000;
                c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = sx(0x001afea4);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x35d8));
                call_ptr(c, 0x00317670, 0x001afea4)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001afeb8);
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00397cc0, 0x001afeb8)?;
                let t = c.g[V0][0] == 0u64;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                if t {
                    break 'b_1b009c;
                } else {
                    'b_1afee4: {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x001afed0);
                        c.g[A2][0] = 0u64;
                        call_ptr(c, 0x0039c870, 0x001afed0)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        let t = c.g[V1][0] != 0u64;
                        if t {
                            break 'b_1afee4;
                        } else {
                            c.cycles += 2;
                            c.g[RA][0] = sx(0x001afee4);
                            call_ptr(c, 0x00260f80, 0x001afee4)?;
                            break 'b_1afee4;
                        }
                    }
                    'b_1aff00: {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001afeec);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        call_ptr(c, 0x00261460, 0x001afeec)?;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        if t {
                            break 'b_1aff00;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x001afefc);
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            call_ptr(c, 0x002613c8, 0x001afefc)?;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            break 'b_1aff00;
                        }
                    }
                    'b_1aff14: {
                        c.cycles += 2;
                        if c.g[V0][0] != 0u64 {
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            break 'b_1aff14;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x001aff10);
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                            call_ptr(c, 0x0025b800, 0x001aff10)?;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            break 'b_1aff14;
                        }
                    }
                    c.cycles += 21;
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6d0));
                    c.g[RA][0] = sx(0x001aff20);
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                    call_ptr(c, 0x00263270, 0x001aff20)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x001aff34);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x98));
                    call_ptr(c, 0x00318448, 0x001aff34)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0x2f));
                    c.ldr(2, (c.g[S1][0] as u32).wrapping_add(0x28));
                    c.ldl(4, (c.g[S1][0] as u32).wrapping_add(0x37));
                    c.ldr(4, (c.g[S1][0] as u32).wrapping_add(0x30));
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0xd3));
                    c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0xcc));
                    c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xdb));
                    c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0xd4));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xdc), c.g[A1][0]);
                    let t = c.g[S1][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1b00a0;
                    } else {
                        c.cycles += 77;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        c.g[RA][0] = sx(0x001aff78);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8c));
                        call_ptr(c, 0x003184e8, 0x001aff78)?;
                        c.g[S0][0] = 0x530000;
                        c.g[A2][0] = 0x20;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8b8));
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x001aff90);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x003e6448, 0x001aff90)?;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                        c.g[A0][0] = 0xffffffffffffffbf;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                        c.g[A1][0] = 0xffffffffffffff7f;
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        c.g[A2][0] = 0xfffffffffffffeff;
                        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 7);
                        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                        c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
                        c.g[V0][0] = c.g[V0][0] & 0x1;
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
                        c.g[RA][0] = sx(0x001affec);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                        call_ptr(c, 0x0014dc80, 0x001affec)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001afff8);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0014dd58, 0x001afff8)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[V1][0] = 0x530000;
                        c.g[S0][0] = c.g[V0][0];
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x5c08), c.g[A0][0]);
                        c.g[V0][0] = 0x1;
                        c.g[A0][0] = c.g[S0][0];
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1f38), c.g[V0][0]);
                        c.g[RA][0] = sx(0x001b001c);
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                        call_ptr(c, 0x00144df0, 0x001b001c)?;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x001b0028);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x001451e8, 0x001b0028)?;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x001b0034);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x00144d98, 0x001b0034)?;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001b0048);
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001b0048)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A1][0] = 0x7;
                        c.g[RA][0] = sx(0x001b0058);
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                        call_ptr(c, 0x0014dd58, 0x001b0058)?;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        c.g[A0][0] = 0x530000;
                        c.sw((c.g[A0][0] as u32).wrapping_add(0x4b34), c.g[A1][0]);
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                        let t = c.g[A1][0] as u32;
                        c.g[RA][0] = sx(0x001b0078);
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001b0078)?;
                        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001b0088);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001b0850,
                            f: ee::s03::cFEStateChallengeReceivedPopup_FillChallengeText,
                        }
                        .call(c, 0x001b0088)?;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        c.g[RA][0] = sx(0x001b0094);
                        c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x98));
                        call_ptr(c, 0x0025e6d0, 0x001b0094)?;
                        c.g[RA][0] = sx(0x001b009c);
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        Callee {
                            addr: 0x001af930,
                            f: ee::s03::NetFE_ClearPlayerStats,
                        }
                        .call(c, 0x001b009c)?;
                        break 'b_1b009c;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1b00a0;
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

/// `cFEStateChallengeReceivedPopup_v31` (0x001b0708): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_challenge_received_popup_v31(c: &mut Cpu) {
    if c.pc != 0x001b0708 {
        return ee::s03::f_001b0708__generated(c);
    }
    guest(c, |c| 'l_1b0708: loop {
        'b_1b0840: {
            'b_1b083c: {
                'b_1b0828: {
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
                        break 'b_1b0828;
                    } else {
                        'b_1b0814: {
                            'b_1b0800: {
                                'b_1b07c8: {
                                    'b_1b07ac: {
                                        'b_1b079c: {
                                            c.cycles += 7;
                                            c.g[V0][0] = 0x460000;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                            c.g[V0][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x36d0));
                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                            let t = c.g[A0][0] as u32;
                                            match t {
                                                0x001b0708 => {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x001b0708);
                                                    }
                                                    continue 'l_1b0708;
                                                }
                                                0x001b0748 => {
                                                    'b_1b075c: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x238));
                                                        let t = c.g[V0][0] != 0u64;
                                                        if t {
                                                            break 'b_1b075c;
                                                        } else {
                                                            c.cycles += 2;
                                                            c.g[RA][0] = sx(0x001b075c);
                                                            call_ptr(c, 0x00260f80, 0x001b075c)?;
                                                            break 'b_1b075c;
                                                        }
                                                    }
                                                    'b_1b0778: {
                                                        c.cycles += 4;
                                                        c.g[RA][0] = sx(0x001b0764);
                                                        c.g[A0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x238));
                                                        call_ptr(c, 0x00261460, 0x001b0764)?;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x238));
                                                        if t {
                                                            break 'b_1b0778;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[RA][0] = sx(0x001b0774);
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x238));
                                                            call_ptr(c, 0x002613c8, 0x001b0774)?;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x238));
                                                            break 'b_1b0778;
                                                        }
                                                    }
                                                    'b_1b078c: {
                                                        c.cycles += 2;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x6d0));
                                                            break 'b_1b078c;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[RA][0] = sx(0x001b0788);
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0xc8));
                                                            call_ptr(c, 0x0025b800, 0x001b0788)?;
                                                            c.g[A1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x6d0));
                                                            break 'b_1b078c;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[RA][0] = sx(0x001b0794);
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x238)
                                                        );
                                                    call_ptr(c, 0x00263018, 0x001b0794)?;
                                                    let t = true;
                                                    c.g[V0][0] = 0u64;
                                                    if t {
                                                        break 'b_1b083c;
                                                    } else {
                                                        break 'b_1b079c;
                                                    }
                                                }
                                                0x001b079c => {
                                                    break 'b_1b079c;
                                                }
                                                0x001b07ac => {
                                                    break 'b_1b07ac;
                                                }
                                                0x001b07c8 => {
                                                    break 'b_1b07c8;
                                                }
                                                0x001b0800 => {
                                                    break 'b_1b0800;
                                                }
                                                0x001b0814 => {
                                                    break 'b_1b0814;
                                                }
                                                0x001b0828 => {
                                                    break 'b_1b0828;
                                                }
                                                _ => {
                                                    c.jump_out(t, 0x001b0708, 0x001b0850);
                                                    return Ok(());
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001b07a4);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x001ab108,
                                            f: ee::s03::NetFEState_ShowWaitPlayerPopup,
                                        }
                                        .call(c, 0x001b07a4)?;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_1b083c;
                                        } else {
                                            break 'b_1b07ac;
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x001b07b4);
                                    c.g[A0][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001b0c08,
                                        f: ee::s03::cFEStateChallengeReceivedPopup_ShowWaiting,
                                    }
                                    .call(c, 0x001b07b4)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x001b07c0);
                                    c.g[A1][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x001ab178,
                                        f: ee::s03::NetFEState_ShowChallengeErrorPopup,
                                    }
                                    .call(c, 0x001b07c0)?;
                                    let t = true;
                                    c.g[V0][0] = 0u64;
                                    if t {
                                        break 'b_1b083c;
                                    } else {
                                        break 'b_1b07c8;
                                    }
                                }
                                c.cycles += 14;
                                c.g[A1][0] = 0x460000;
                                c.g[A0][0] = 0x6e4;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1c60));
                                c.g[A2][0] = 0x100;
                                c.g[RA][0] = sx(0x001b07e0);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x001b07e0)?;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x001b07ec);
                                c.g[A0][0] = c.g[V0][0];
                                Callee {
                                    addr: 0x001b0f60,
                                    f: ee::s03::cFEStateOnlineCharacterSelect_ctor,
                                }
                                .call(c, 0x001b07ec)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001b07f8);
                                c.g[A1][0] = c.g[V0][0];
                                Callee {
                                    addr: 0x001a8a40,
                                    f: ee::s03::NetFEState_GotoNextState,
                                }
                                .call(c, 0x001b07f8)?;
                                let t = true;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_1b083c;
                                } else {
                                    break 'b_1b0800;
                                }
                            }
                            c.cycles += 5;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001b080c);
                            c.g[A1][0] = c.g[A2][0];
                            Callee {
                                addr: 0x001afa60,
                                f: ee::s03::NetFE_ShowPlayerStats,
                            }
                            .call(c, 0x001b080c)?;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_1b083c;
                            } else {
                                break 'b_1b0814;
                            }
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001b0820);
                        c.g[A1][0] = c.g[A2][0];
                        Callee {
                            addr: 0x001afd88,
                            f: ee::s03::NetFE_ShowPlayerConnQuality,
                        }
                        .call(c, 0x001b0820)?;
                        let t = true;
                        c.g[V0][0] = 0u64;
                        if t {
                            break 'b_1b083c;
                        } else {
                            break 'b_1b0828;
                        }
                    }
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x001b0834);
                c.g[A1][0] = c.g[S1][0];
                Callee {
                    addr: 0x001a8e40,
                    f: ee::s03::NetFEState_HandleNetEvent,
                }
                .call(c, 0x001b0834)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                if t {
                    break 'b_1b0840;
                } else {
                    break 'b_1b083c;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1b0840;
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

/// `cFEStateOnlineCharacterSelect_v31` (0x001b15d8): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_online_character_select_v31(c: &mut Cpu) {
    if c.pc != 0x001b15d8 {
        return ee::s03::f_001b15d8__generated(c);
    }
    guest(c, |c| 'l_1b15d8: loop {
        'b_1b174c: {
            'b_1b1748: {
                'b_1b1744: {
                    'b_1b1734: {
                        c.cycles += 8;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                        c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xe0));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[V0][0] = (c.g[V1][0] < 0x34u64) as u64;
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S0][0] = c.g[A0][0];
                        let t = c.g[V0][0] == 0u64;
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        if t {
                            break 'b_1b1734;
                        } else {
                            'b_1b1724: {
                                'b_1b16b8: {
                                    'b_1b168c: {
                                        'b_1b167c: {
                                            'b_1b166c: {
                                                'b_1b1664: {
                                                    'b_1b1658: {
                                                        c.cycles += 7;
                                                        c.g[V0][0] = 0x460000;
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x38d0));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                        let t = c.g[A0][0] as u32;
                                                        match t {
                                                            0x001b15d8 => {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x001b15d8,
                                                                    );
                                                                }
                                                                continue 'l_1b15d8;
                                                            }
                                                            0x001b1614 => {
                                                                c.cycles += 9;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                c.g[A1][0] = 0x7;
                                                                c.g[RA][0] = sx(0x001b1624);
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x78));
                                                                call_ptr(
                                                                    c, 0x0014dd58, 0x001b1624,
                                                                )?;
                                                                c.g[V1][0] = 0x530000;
                                                                c.g[S1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x4b30));
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S1][0] as u32)
                                                                        .wrapping_add(0x32c));
                                                                let t = (c.g[A1][0] as i64) < 0;
                                                                c.g[V0][0] = 0u64;
                                                                if t {
                                                                    break 'b_1b1748;
                                                                } else {
                                                                    c.cycles += 8;
                                                                    c.g[RA][0] = sx(0x001b1640);
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    Callee { addr: 0x001b1760, f: ee::s03::cFEStateOnlineCharacterSelect_ShowOpponentChar }.call(c, 0x001b1640)?;
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x32c));
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    c.g[RA][0] = sx(0x001b1650);
                                                                    c.g[A2][0] = 0x1;
                                                                    Callee { addr: 0x001b1b10, f: ee::s03::cFEStateOnlineCharacterSelect_ShowCharacter }.call(c, 0x001b1650)?;
                                                                    let t = true;
                                                                    c.g[V0][0] = 0u64;
                                                                    if t {
                                                                        break 'b_1b1748;
                                                                    } else {
                                                                        break 'b_1b1658;
                                                                    }
                                                                }
                                                            }
                                                            0x001b1658 => {
                                                                break 'b_1b1658;
                                                            }
                                                            0x001b1664 => {
                                                                break 'b_1b1664;
                                                            }
                                                            0x001b167c => {
                                                                break 'b_1b167c;
                                                            }
                                                            0x001b168c => {
                                                                break 'b_1b168c;
                                                            }
                                                            0x001b16b8 => {
                                                                break 'b_1b16b8;
                                                            }
                                                            0x001b1734 => {
                                                                break 'b_1b1734;
                                                            }
                                                            0x001b1744 => {
                                                                break 'b_1b1744;
                                                            }
                                                            _ => {
                                                                c.jump_out(
                                                                    t, 0x001b15d8, 0x001b175c,
                                                                );
                                                                return Ok(());
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 3;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    let t = true;
                                                    c.g[A1][0] = 0x1;
                                                    if t {
                                                        break 'b_1b166c;
                                                    } else {
                                                        break 'b_1b1664;
                                                    }
                                                }
                                                c.cycles += 2;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] = 0u64;
                                                break 'b_1b166c;
                                            }
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001b1674);
                                            c.g[A2][0] = 0x1;
                                            Callee { addr: 0x001b1a08, f: ee::s03::cFEStateOnlineCharacterSelect_ShiftFrameLayer }.call(c, 0x001b1674)?;
                                            let t = true;
                                            c.g[V0][0] = 0u64;
                                            if t {
                                                break 'b_1b1748;
                                            } else {
                                                break 'b_1b167c;
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001b1684);
                                        c.g[A0][0] = c.g[S0][0];
                                        Callee {
                                            addr: 0x001ab288,
                                            f: ee::s03::NetFEState_ShowSessionLostPopup,
                                        }
                                        .call(c, 0x001b1684)?;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_1b1748;
                                        } else {
                                            break 'b_1b168c;
                                        }
                                    }
                                    c.cycles += 11;
                                    c.g[A1][0] = 0x460000;
                                    c.g[A0][0] = 0x73c;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xcc8));
                                    c.g[A2][0] = 0u64;
                                    c.g[RA][0] = sx(0x001b16a4);
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, 0x00317d70, 0x001b16a4)?;
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x001b16b0);
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x00192d00, 0x001b16b0)?;
                                    let t = true;
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        break 'b_1b1724;
                                    } else {
                                        break 'b_1b16b8;
                                    }
                                }
                                'b_1b1700: {
                                    'b_1b16fc: {
                                        c.cycles += 4;
                                        c.g[RA][0] = sx(0x001b16c0);
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                        call_ptr(c, 0x0025da90, 0x001b16c0)?;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1d98));
                                        if t {
                                            break 'b_1b16fc;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A1][0] = 0x460000;
                                            if t {
                                                break 'b_1b1700;
                                            } else {
                                                c.cycles += 11;
                                                c.g[A1][0] = 0x460000;
                                                c.g[A0][0] = 0x73c;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_sub(0xf0));
                                                c.g[A2][0] = 0u64;
                                                c.g[RA][0] = sx(0x001b16e8);
                                                c.g[A3][0] = 0u64;
                                                call_ptr(c, 0x00317d70, 0x001b16e8)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                                c.g[RA][0] = sx(0x001b16f4);
                                                c.g[A0][0] = c.g[V0][0];
                                                Callee {
                                                    addr: 0x001b1f88,
                                                    f: ee::s03::cFEStateChatRoom_ctor,
                                                }
                                                .call(c, 0x001b16f4)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_1b1724;
                                                } else {
                                                    break 'b_1b16fc;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 1;
                                    c.g[A1][0] = 0x460000;
                                    break 'b_1b1700;
                                }
                                c.cycles += 9;
                                c.g[A0][0] = 0x6d0;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xd8));
                                c.g[A2][0] = 0u64;
                                c.g[RA][0] = sx(0x001b1714);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x001b1714)?;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x001b1720);
                                c.g[A0][0] = c.g[V0][0];
                                Callee {
                                    addr: 0x001bfbb8,
                                    f: ee::s03::cFEStateOnlineMainMenu_ctor,
                                }
                                .call(c, 0x001b1720)?;
                                c.g[A0][0] = c.g[S0][0];
                                break 'b_1b1724;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001b172c);
                            c.g[A1][0] = c.g[V0][0];
                            Callee {
                                addr: 0x001a8a40,
                                f: ee::s03::NetFEState_GotoNextState,
                            }
                            .call(c, 0x001b172c)?;
                            let t = true;
                            c.g[V0][0] = 0u64;
                            if t {
                                break 'b_1b1748;
                            } else {
                                break 'b_1b1734;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001b173c);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001a8e40,
                        f: ee::s03::NetFEState_HandleNetEvent,
                    }
                    .call(c, 0x001b173c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1b174c;
                    } else {
                        break 'b_1b1744;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_1b1748;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1b174c;
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

/// `cFEStateChatRoom_OnWidgetAction` (0x001b31a0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn c_fe_state_chat_room_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001b31a0 {
        return ee::s03::f_001b31a0__generated(c);
    }
    guest(c, |c| 'l_1b31a0: loop {
        'b_1b33b4: {
            'b_1b33a0: {
                c.cycles += 14;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S4][0] = c.g[A2][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S3][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S2][0] = c.g[A1][0];
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = (c.g[V1][0] < 0xdu64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                if t {
                    break 'b_1b33a0;
                } else {
                    'b_1b338c: {
                        'b_1b3378: {
                            'b_1b3368: {
                                'b_1b3348: {
                                    'b_1b32e8: {
                                        'b_1b32d4: {
                                            'b_1b3290: {
                                                'b_1b3248: {
                                                    c.cycles += 7;
                                                    c.g[V0][0] = 0x460000;
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x3d40));
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add((c.g[V0][0] as u32)));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                    let t = c.g[A0][0] as u32;
                                                    match t {
                                                        0x001b31a0 => {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x001b31a0);
                                                            }
                                                            continue 'l_1b31a0;
                                                        }
                                                        0x001b31f4 => {
                                                            c.cycles += 6;
                                                            c.g[S0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x18));
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0xc8));
                                                            c.g[RA][0] = sx(0x001b3204);
                                                            c.g[A1][0] = c.g[S0][0];
                                                            call_ptr(c, 0x0025f248, 0x001b3204)?;
                                                            if c.g[V0][0] != 0u64 {
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x80),
                                                                );
                                                                break 'b_1b33b4;
                                                            } else {
                                                                c.cycles += 15;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0xc8));
                                                                c.g[A2][0] = 0u64;
                                                                c.g[RA][0] = sx(0x001b321c);
                                                                c.g[A1][0] = c.g[S0][0];
                                                                call_ptr(
                                                                    c, 0x0025f1e0, 0x001b321c,
                                                                )?;
                                                                c.g[A1][0] = c.g[V0][0];
                                                                c.g[RA][0] = sx(0x001b3228);
                                                                c.g[A0][0] = c.g[S3][0];
                                                                Callee { addr: 0x001a9890, f: ee::s03::NetFEState_SetSelectedUser }.call(c, 0x001b3228)?;
                                                                c.g[RA][0] = sx(0x001b3230);
                                                                c.g[A0][0] = c.g[S3][0];
                                                                Callee { addr: 0x001a98b0, f: ee::s03::NetFEState_GetSelectedUser }.call(c, 0x001b3230)?;
                                                                c.g[A0][0] = c.g[S3][0];
                                                                c.g[A1][0] = c.g[V0][0];
                                                                c.g[RA][0] = sx(0x001b3240);
                                                                c.g[A2][0] = 0x1;
                                                                Callee { addr: 0x001ab370, f: ee::s03::NetFEState_ShowBuddyActionPopup }.call(c, 0x001b3240)?;
                                                                let t = true;
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x80),
                                                                );
                                                                if t {
                                                                    break 'b_1b33b4;
                                                                } else {
                                                                    break 'b_1b3248;
                                                                }
                                                            }
                                                        }
                                                        0x001b3248 => {
                                                            break 'b_1b3248;
                                                        }
                                                        0x001b3290 => {
                                                            break 'b_1b3290;
                                                        }
                                                        0x001b32e8 => {
                                                            break 'b_1b32e8;
                                                        }
                                                        0x001b3348 => {
                                                            break 'b_1b3348;
                                                        }
                                                        0x001b3368 => {
                                                            break 'b_1b3368;
                                                        }
                                                        0x001b33a0 => {
                                                            break 'b_1b33a0;
                                                        }
                                                        _ => {
                                                            c.jump_out(t, 0x001b31a0, 0x001b33d4);
                                                            return Ok(());
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[S1][0] = 0u64;
                                                c.g[S5][0] = 0x460000;
                                                c.g[A0][0] = c.g[SP][0];
                                                'l_1b3258: loop {
                                                    c.cycles += 8;
                                                    c.g[A1][0] =
                                                        sx((c.g[S5][0] as u32)
                                                            .wrapping_add(0x3a60));
                                                    c.g[RA][0] = sx(0x001b3264);
                                                    c.g[A2][0] = c.g[S1][0];
                                                    call_ptr(c, 0x00417828, 0x001b3264)?;
                                                    c.g[S0][0] = c
                                                        .lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                                    c.g[RA][0] = sx(0x001b3270);
                                                    c.g[A0][0] = c.g[SP][0];
                                                    call_ptr(c, 0x00317670, 0x001b3270)?;
                                                    if c.g[S0][0] == c.g[V0][0] {
                                                        c.g[A0][0] = c.g[S3][0];
                                                        break 'b_1b3378;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[S1][0] =
                                                            sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V0][0] =
                                                            ((c.g[S1][0] as i64) < 6i64) as u64;
                                                        let t = c.g[V0][0] != 0u64;
                                                        c.g[A0][0] = c.g[SP][0];
                                                        if t {
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x001b3258);
                                                            }
                                                            continue 'l_1b3258;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A0][0] = c.g[S3][0];
                                                            if t {
                                                                break 'b_1b32d4;
                                                            } else {
                                                                break 'b_1b3290;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            c.g[S1][0] = 0u64;
                                            c.g[S5][0] = 0x460000;
                                            c.g[A0][0] = c.g[SP][0];
                                            'l_1b32a0: loop {
                                                c.cycles += 8;
                                                c.g[A1][0] =
                                                    sx((c.g[S5][0] as u32).wrapping_add(0x3a60));
                                                c.g[RA][0] = sx(0x001b32ac);
                                                c.g[A2][0] = c.g[S1][0];
                                                call_ptr(c, 0x00417828, 0x001b32ac)?;
                                                c.g[S0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x38));
                                                c.g[RA][0] = sx(0x001b32b8);
                                                c.g[A0][0] = c.g[SP][0];
                                                call_ptr(c, 0x00317670, 0x001b32b8)?;
                                                if c.g[S0][0] == c.g[V0][0] {
                                                    c.g[A0][0] = c.g[S3][0];
                                                    break 'b_1b338c;
                                                } else {
                                                    c.cycles += 4;
                                                    c.g[S1][0] =
                                                        sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                                    c.g[V0][0] =
                                                        ((c.g[S1][0] as i64) < 6i64) as u64;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[A0][0] = c.g[SP][0];
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001b32a0);
                                                        }
                                                        continue 'l_1b32a0;
                                                    } else {
                                                        c.cycles += 1;
                                                        c.g[A0][0] = c.g[S3][0];
                                                        break 'b_1b32d4;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[A1][0] = c.g[S2][0];
                                        c.g[RA][0] = sx(0x001b32e0);
                                        c.g[A2][0] = c.g[S4][0];
                                        Callee {
                                            addr: 0x001a8918,
                                            f: ee::s03::NetFEState_OnWidgetAction,
                                        }
                                        .call(c, 0x001b32e0)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                        if t {
                                            break 'b_1b33b4;
                                        } else {
                                            break 'b_1b32e8;
                                        }
                                    }
                                    c.cycles += 24;
                                    c.g[V0][0] = 0x1;
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[RA][0] = sx(0x001b32f8);
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x6f8), c.g[V0][0]);
                                    Callee {
                                        addr: 0x001a9c50,
                                        f: ee::s03::NetFEState_GetPopup,
                                    }
                                    .call(c, 0x001b32f8)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A0][0] = 0x460000;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xbc), 0u64);
                                    c.g[RA][0] = sx(0x001b330c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3d20));
                                    call_ptr(c, 0x00317670, 0x001b330c)?;
                                    c.g[A2][0] = 0x802;
                                    c.g[V1][0] = 0x3;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x164), 0u64);
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[A1][0] = c.g[S0][0];
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x150), c.g[A2][0]);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x001b3330);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                    Callee {
                                        addr: 0x001a9bc0,
                                        f: ee::s03::NetFEState_ShowPopup,
                                    }
                                    .call(c, 0x001b3330)?;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                    c.g[A1][0] = 0xffffffffffffffff;
                                    c.g[RA][0] = sx(0x001b3340);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x0025d800, 0x001b3340)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                    if t {
                                        break 'b_1b33b4;
                                    } else {
                                        break 'b_1b3348;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                                c.g[V0][0] = c.g[V0][0] & 0x6;
                                let t = c.g[V0][0] != 0u64;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                if t {
                                    break 'b_1b33b4;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001b3360);
                                    c.g[A0][0] = c.g[S3][0];
                                    Callee {
                                        addr: 0x001a9b58,
                                        f: ee::s03::NetFEState_OpenMessenger,
                                    }
                                    .call(c, 0x001b3360)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                    if t {
                                        break 'b_1b33b4;
                                    } else {
                                        break 'b_1b3368;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001b3370);
                            c.g[A0][0] = c.g[S3][0];
                            Callee {
                                addr: 0x001b33d8,
                                f: ee::s03::cFEStateChatRoom_OnKeyboardDone,
                            }
                            .call(c, 0x001b3370)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                            if t {
                                break 'b_1b33b4;
                            } else {
                                break 'b_1b3378;
                            }
                        }
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x001b3384);
                        c.g[A2][0] = 0x1;
                        Callee {
                            addr: 0x001b3d20,
                            f: ee::s03::cFEStateChatRoom_HighlightPlayer,
                        }
                        .call(c, 0x001b3384)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                        if t {
                            break 'b_1b33b4;
                        } else {
                            break 'b_1b338c;
                        }
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001b3398);
                    c.g[A2][0] = 0u64;
                    Callee {
                        addr: 0x001b3d20,
                        f: ee::s03::cFEStateChatRoom_HighlightPlayer,
                    }
                    .call(c, 0x001b3398)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    if t {
                        break 'b_1b33b4;
                    } else {
                        break 'b_1b33a0;
                    }
                }
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001b33b0);
            c.g[A2][0] = c.g[S4][0];
            Callee {
                addr: 0x001a8918,
                f: ee::s03::NetFEState_OnWidgetAction,
            }
            .call(c, 0x001b33b0)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_1b33b4;
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, elf_image, fill, Rng, DATA, GAME_GP};
    use std::collections::BTreeSet;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Fixture areas, clear of the fake log (DATA+0x100000) and of the
    /// pointers the fakes return (remapped into `OC910_WIN`).
    pub(crate) const OC910_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC910_MAGIC: u32 = 0x0c91_0c0d;
    /// Forced result of fake call `n`: word `n & 0x3f` (0: the fake's own
    /// pointer, else `v0 = word - 1`).
    pub(crate) const OC910_RES: u32 = OC910_CTRL + 0x1000;
    /// Pointers returned by the fakes point into this window (every 0x100
    /// bytes, +0xc holds a vtable pointer).
    pub(crate) const OC910_WIN: u32 = DATA + 0x21_0000;
    /// Vtable of the window objects: half delta at +8, method at +0xc.
    pub(crate) const OC910_VT: u32 = DATA + 0x22_0000;
    pub(crate) const OC910_THIS: u32 = DATA + 0x23_0000;
    pub(crate) const OC910_WORLD: u32 = DATA + 0x24_0000;
    pub(crate) const OC910_LOBBY: u32 = DATA + 0x25_0000;
    pub(crate) const OC910_WIDGET: u32 = DATA + 0x26_0000;
    /// Object whose +0x100 half and +0x104 method the Init's first call uses.
    pub(crate) const OC910_OBJ: u32 = DATA + 0x27_0000;

    /// The fake callee: log the call like `fakes::fake_call`, then apply the
    /// forced result of this call index; an unforced pointer result is moved
    /// into the window.
    pub(crate) fn oc910_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(fakes::LOG_COUNT);
        fakes::fake_call(c, addr);
        if c.pc == ra {
            let m = c.bus.read32(OC910_RES + 4 * (n & 0x3f));
            if m != 0 {
                c.g[V0][0] = m.wrapping_sub(1) as u64;
            } else {
                c.g[V0][0] = (OC910_WIN + (c.g[V0][0] as u32 & 0xf00)) as u64;
            }
        }
    }

    /// Gated fakes of the callees in their override slots.
    macro_rules! oc910_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC910_CTRL) != OC910_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc910_call(c, $addr);
                }
            )*
            fn oc910_install() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc910_fakes!(
        (
            oc910_fk_00144d98,
            OC910_PREV_00144D98,
            ee::s01::OVERRIDE_00144D98,
            ee::s01::f_00144d98__generated,
            0x0014_4d98
        ),
        (
            oc910_fk_00144df0,
            OC910_PREV_00144DF0,
            ee::s01::OVERRIDE_00144DF0,
            ee::s01::f_00144df0__generated,
            0x0014_4df0
        ),
        (
            oc910_fk_001451e8,
            OC910_PREV_001451E8,
            ee::s01::OVERRIDE_001451E8,
            ee::s01::f_001451e8__generated,
            0x0014_51e8
        ),
        (
            oc910_fk_0014dc80,
            OC910_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc910_fk_0014dd58,
            OC910_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc910_fk_00192d00,
            OC910_PREV_00192D00,
            ee::s02::OVERRIDE_00192D00,
            ee::s02::f_00192d00__generated,
            0x0019_2d00
        ),
        (
            oc910_fk_001a8918,
            OC910_PREV_001A8918,
            ee::s03::OVERRIDE_001A8918,
            ee::s03::f_001a8918__generated,
            0x001a_8918
        ),
        (
            oc910_fk_001a8a40,
            OC910_PREV_001A8A40,
            ee::s03::OVERRIDE_001A8A40,
            ee::s03::f_001a8a40__generated,
            0x001a_8a40
        ),
        (
            oc910_fk_001a8e40,
            OC910_PREV_001A8E40,
            ee::s03::OVERRIDE_001A8E40,
            ee::s03::f_001a8e40__generated,
            0x001a_8e40
        ),
        (
            oc910_fk_001a9890,
            OC910_PREV_001A9890,
            ee::s03::OVERRIDE_001A9890,
            ee::s03::f_001a9890__generated,
            0x001a_9890
        ),
        (
            oc910_fk_001a98b0,
            OC910_PREV_001A98B0,
            ee::s03::OVERRIDE_001A98B0,
            ee::s03::f_001a98b0__generated,
            0x001a_98b0
        ),
        (
            oc910_fk_001a9b58,
            OC910_PREV_001A9B58,
            ee::s03::OVERRIDE_001A9B58,
            ee::s03::f_001a9b58__generated,
            0x001a_9b58
        ),
        (
            oc910_fk_001a9bc0,
            OC910_PREV_001A9BC0,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a_9bc0
        ),
        (
            oc910_fk_001a9c50,
            OC910_PREV_001A9C50,
            ee::s03::OVERRIDE_001A9C50,
            ee::s03::f_001a9c50__generated,
            0x001a_9c50
        ),
        (
            oc910_fk_001ab108,
            OC910_PREV_001AB108,
            ee::s03::OVERRIDE_001AB108,
            ee::s03::f_001ab108__generated,
            0x001a_b108
        ),
        (
            oc910_fk_001ab178,
            OC910_PREV_001AB178,
            ee::s03::OVERRIDE_001AB178,
            ee::s03::f_001ab178__generated,
            0x001a_b178
        ),
        (
            oc910_fk_001ab288,
            OC910_PREV_001AB288,
            ee::s03::OVERRIDE_001AB288,
            ee::s03::f_001ab288__generated,
            0x001a_b288
        ),
        (
            oc910_fk_001ab370,
            OC910_PREV_001AB370,
            ee::s03::OVERRIDE_001AB370,
            ee::s03::f_001ab370__generated,
            0x001a_b370
        ),
        (
            oc910_fk_001af930,
            OC910_PREV_001AF930,
            ee::s03::OVERRIDE_001AF930,
            ee::s03::f_001af930__generated,
            0x001a_f930
        ),
        (
            oc910_fk_001afa60,
            OC910_PREV_001AFA60,
            ee::s03::OVERRIDE_001AFA60,
            ee::s03::f_001afa60__generated,
            0x001a_fa60
        ),
        (
            oc910_fk_001afd88,
            OC910_PREV_001AFD88,
            ee::s03::OVERRIDE_001AFD88,
            ee::s03::f_001afd88__generated,
            0x001a_fd88
        ),
        (
            oc910_fk_001b0850,
            OC910_PREV_001B0850,
            ee::s03::OVERRIDE_001B0850,
            ee::s03::f_001b0850__generated,
            0x001b_0850
        ),
        (
            oc910_fk_001b0c08,
            OC910_PREV_001B0C08,
            ee::s03::OVERRIDE_001B0C08,
            ee::s03::f_001b0c08__generated,
            0x001b_0c08
        ),
        (
            oc910_fk_001b0f60,
            OC910_PREV_001B0F60,
            ee::s03::OVERRIDE_001B0F60,
            ee::s03::f_001b0f60__generated,
            0x001b_0f60
        ),
        (
            oc910_fk_001b1760,
            OC910_PREV_001B1760,
            ee::s03::OVERRIDE_001B1760,
            ee::s03::f_001b1760__generated,
            0x001b_1760
        ),
        (
            oc910_fk_001b1a08,
            OC910_PREV_001B1A08,
            ee::s03::OVERRIDE_001B1A08,
            ee::s03::f_001b1a08__generated,
            0x001b_1a08
        ),
        (
            oc910_fk_001b1b10,
            OC910_PREV_001B1B10,
            ee::s03::OVERRIDE_001B1B10,
            ee::s03::f_001b1b10__generated,
            0x001b_1b10
        ),
        (
            oc910_fk_001b1f88,
            OC910_PREV_001B1F88,
            ee::s03::OVERRIDE_001B1F88,
            ee::s03::f_001b1f88__generated,
            0x001b_1f88
        ),
        (
            oc910_fk_001b33d8,
            OC910_PREV_001B33D8,
            ee::s03::OVERRIDE_001B33D8,
            ee::s03::f_001b33d8__generated,
            0x001b_33d8
        ),
        (
            oc910_fk_001b3d20,
            OC910_PREV_001B3D20,
            ee::s03::OVERRIDE_001B3D20,
            ee::s03::f_001b3d20__generated,
            0x001b_3d20
        ),
        (
            oc910_fk_001bfbb8,
            OC910_PREV_001BFBB8,
            ee::s03::OVERRIDE_001BFBB8,
            ee::s03::f_001bfbb8__generated,
            0x001b_fbb8
        ),
        (
            oc910_fk_0025b800,
            OC910_PREV_0025B800,
            ee::s06::OVERRIDE_0025B800,
            ee::s06::f_0025b800__generated,
            0x0025_b800
        ),
        (
            oc910_fk_0025d800,
            OC910_PREV_0025D800,
            ee::s06::OVERRIDE_0025D800,
            ee::s06::f_0025d800__generated,
            0x0025_d800
        ),
        (
            oc910_fk_0025da90,
            OC910_PREV_0025DA90,
            ee::s06::OVERRIDE_0025DA90,
            ee::s06::f_0025da90__generated,
            0x0025_da90
        ),
        (
            oc910_fk_0025e6d0,
            OC910_PREV_0025E6D0,
            ee::s06::OVERRIDE_0025E6D0,
            ee::s06::f_0025e6d0__generated,
            0x0025_e6d0
        ),
        (
            oc910_fk_0025f1e0,
            OC910_PREV_0025F1E0,
            ee::s06::OVERRIDE_0025F1E0,
            ee::s06::f_0025f1e0__generated,
            0x0025_f1e0
        ),
        (
            oc910_fk_0025f248,
            OC910_PREV_0025F248,
            ee::s06::OVERRIDE_0025F248,
            ee::s06::f_0025f248__generated,
            0x0025_f248
        ),
        (
            oc910_fk_00260f80,
            OC910_PREV_00260F80,
            ee::s06::OVERRIDE_00260F80,
            ee::s06::f_00260f80__generated,
            0x0026_0f80
        ),
        (
            oc910_fk_002613c8,
            OC910_PREV_002613C8,
            ee::s06::OVERRIDE_002613C8,
            ee::s06::f_002613c8__generated,
            0x0026_13c8
        ),
        (
            oc910_fk_00261460,
            OC910_PREV_00261460,
            ee::s06::OVERRIDE_00261460,
            ee::s06::f_00261460__generated,
            0x0026_1460
        ),
        (
            oc910_fk_00263018,
            OC910_PREV_00263018,
            ee::s07::OVERRIDE_00263018,
            ee::s07::f_00263018__generated,
            0x0026_3018
        ),
        (
            oc910_fk_00263270,
            OC910_PREV_00263270,
            ee::s07::OVERRIDE_00263270,
            ee::s07::f_00263270__generated,
            0x0026_3270
        ),
        (
            oc910_fk_00317670,
            OC910_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc910_fk_00317d70,
            OC910_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc910_fk_00318448,
            OC910_PREV_00318448,
            ee::s10::OVERRIDE_00318448,
            ee::s10::f_00318448__generated,
            0x0031_8448
        ),
        (
            oc910_fk_003184e8,
            OC910_PREV_003184E8,
            ee::s10::OVERRIDE_003184E8,
            ee::s10::f_003184e8__generated,
            0x0031_84e8
        ),
        (
            oc910_fk_00397cc0,
            OC910_PREV_00397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc910_fk_0039c870,
            OC910_PREV_0039C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc910_fk_003e6448,
            OC910_PREV_003E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc910_fk_00417828,
            OC910_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
    );

    /// The machine, the ELF image (jump tables), the armed fakes with random
    /// results, the window, the objects and the globals the four functions read.
    /// `RES` forcing: 50% own pointer, 25% result 0, 25% result 1.
    fn oc910_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        elf_image(c);
        oc910_install();
        c.bus.write32(OC910_CTRL, OC910_MAGIC);
        for i in 0..0x40u32 {
            let m = match r.next_u32() % 4 {
                0 | 1 => 0,
                2 => 1,
                _ => 2,
            };
            c.bus.write32(OC910_RES + 4 * i, m);
        }
        fill(c, &mut r, OC910_WIN, 0x500);
        for k in 0..16 {
            c.bus.write32(OC910_WIN + k * 0x100 + 0xc, OC910_VT);
        }
        c.bus.write16(OC910_VT + 8, (r.next_u32() & 0xfff0) as u16);
        c.bus.write32(OC910_VT + 0xc, FAKE);
        fill(c, &mut r, OC910_THIS, 0x200);
        fill(c, &mut r, OC910_WORLD, 0x100);
        fill(c, &mut r, OC910_LOBBY, 0x100);
        fill(c, &mut r, OC910_WIDGET, 0x100);
        c.bus
            .write16(OC910_OBJ + 0x100, (r.next_u32() & 0xfff0) as u16);
        c.bus.write32(OC910_OBJ + 0x104, FAKE + 0x10);
        let gp = GAME_GP;
        c.bus.write32(gp.wrapping_sub(0xc8), OC910_LOBBY);
        c.bus.write32(gp.wrapping_sub(0x848), OC910_WORLD);
        let buddy = if r.next_u32() % 3 == 0 { 0 } else { OC910_WIN };
        c.bus.write32(gp.wrapping_add(0x238), buddy);
        let flag = if r.next_u32() % 2 == 0 {
            0
        } else {
            r.next_u32() | 1
        };
        c.bus.write32(gp.wrapping_add(0x1d98), flag);
        r
    }

    /// Distinct total cycle counts of the generated body over the seeds.
    fn oc910_paths(generated: GuestFn, seeds: u32, setup: &dyn Fn(&mut Cpu, u32)) -> BTreeSet<u64> {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = crate::difftest::fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        seen
    }

    // ---- cFEStateChallengeReceivedPopup_Init ----

    /// Race-setup popup `Init`: opens the screen by hash, starts the buddy
    /// manager and copies the challenger's settings into the race interface.
    fn oc910_init_setup(c: &mut Cpu, seed: u32) {
        oc910_base(c, seed, 0x0c91_0001, 24);
        // The screen creation (call 2) fails every fourth seed.
        if seed % 4 == 0 {
            c.bus.write32(OC910_RES + 8, 1);
        } else if seed % 4 == 1 {
            c.bus.write32(OC910_RES + 8, 0);
        }
        c.bus.write32(OC910_THIS + 8, OC910_OBJ);
        c.g[A0][0] = OC910_THIS as u64;
        c.pc = 0x001a_fe68;
    }

    #[test]
    fn c_fe_state_challenge_received_popup_init_matches() {
        check(
            ee::s03::f_001afe68__generated,
            c_fe_state_challenge_received_popup_init,
            240,
            &oc910_init_setup,
        );
    }

    /// The screen failing, the buddy-manager arms, the challenger missing or
    /// present and unwinds give several distinct paths.
    #[test]
    fn c_fe_state_challenge_received_popup_init_paths() {
        let seen = oc910_paths(ee::s03::f_001afe68__generated, 240, &oc910_init_setup);
        assert!(seen.len() >= 4, "only {} paths: {seen:?}", seen.len());
    }

    // ---- cFEStateChallengeReceivedPopup_v31 ----

    /// Net events the popup's switch (jump table at 0x4636d0, events 0xe3..=0x113)
    /// handles, one per arm, and three out of range.
    pub(crate) const OC910_V31_EVENTS: [u32; 14] = [
        0xe3,
        0xe4,
        0xe5,
        0xed,
        0xef,
        0xf3,
        0x110,
        0x100,
        0xe2,
        0x114,
        0,
        0xef,
        0xef,
        0xffff_ffff,
    ];

    fn oc910_v31_setup(c: &mut Cpu, seed: u32) {
        oc910_base(c, seed, 0x0c91_0002, 12);
        c.g[A0][0] = OC910_THIS as u64;
        c.g[A1][0] = OC910_V31_EVENTS[(seed % 14) as usize] as u64;
        c.pc = 0x001b_0708;
    }

    /// `cFEStateChallengeReceivedPopup_v31` (vtable slot 31, the net-event
    /// handler): the event switch, event 0xef reconnecting the buddy manager.
    #[test]
    fn c_fe_state_challenge_received_popup_v31_matches() {
        check(
            ee::s03::f_001b0708__generated,
            c_fe_state_challenge_received_popup_v31,
            14 * 16,
            &oc910_v31_setup,
        );
    }

    #[test]
    fn c_fe_state_challenge_received_popup_v31_paths() {
        let seen = oc910_paths(ee::s03::f_001b0708__generated, 14 * 16, &oc910_v31_setup);
        assert!(seen.len() >= 8, "only {} paths: {seen:?}", seen.len());
    }

    // ---- cFEStateOnlineCharacterSelect_v31 ----

    /// Events of the jump table at 0x4638d0 (events 0xe0..=0x113), one per arm,
    /// and out-of-range ones.
    pub(crate) const OC910_CHARSEL_EVENTS: [u32; 10] =
        [0xe0, 0xe1, 0xee, 0xf3, 0xf5, 0xf6, 0xf7, 0x10c, 0xdf, 0x114];

    fn oc910_charsel_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc910_base(c, seed, 0x0c91_0003, 8);
        // Event 0xf3 tests the sign of the word at 0x534b30 + 0x32c.
        let word = if (seed / 10) % 2 == 0 {
            0xffff_fff0
        } else {
            r.next_u32() & 0x7fff_ffff
        };
        c.bus.write32(0x0053_4b30 + 0x32c, word);
        c.g[A0][0] = OC910_THIS as u64;
        c.g[A1][0] = OC910_CHARSEL_EVENTS[(seed % 10) as usize] as u64;
        c.pc = 0x001b_15d8;
    }

    /// `cFEStateOnlineCharacterSelect_v31` (vtable slot 31, the net-event handler): shows the opponent,
    /// shifts the frame layer, goes on to the chat room or the main menu.
    #[test]
    fn c_fe_state_online_character_select_v31_matches() {
        check(
            ee::s03::f_001b15d8__generated,
            c_fe_state_online_character_select_v31,
            200,
            &oc910_charsel_setup,
        );
    }

    #[test]
    fn c_fe_state_online_character_select_v31_paths() {
        let seen = oc910_paths(ee::s03::f_001b15d8__generated, 200, &oc910_charsel_setup);
        assert!(seen.len() >= 8, "only {} paths: {seen:?}", seen.len());
    }

    // ---- cFEStateChatRoom_OnWidgetAction ----

    /// Widget actions 1..=0xd of the jump table at 0x463d40, and two others.
    pub(crate) const OC910_CHAT_EVENTS: [u32; 15] =
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 0, 0x20];

    fn oc910_chat_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc910_base(c, seed, 0x0c91_0004, 16);
        let event = OC910_CHAT_EVENTS[(seed % 15) as usize];
        // Events 1 and 2 scan six hash slots (sprintf = call 2i, StrHash =
        // call 2i+1) for the widget's hash (+0x38): the match comes at
        // iteration k (0..6), or never (k 6, 7).
        let k = (seed / 15) % 8;
        for i in 0..6u32 {
            c.bus
                .write32(OC910_RES + 4 * (2 * i + 1), if i == k { 1 } else { 2 });
        }
        c.bus.write32(
            OC910_WIDGET + 0x38,
            if k == 7 { r.next_u32() | 1 } else { 0 },
        );
        if event == 6 {
            // GetPopup (call 0) must return a pointer: its fields are written.
            c.bus.write32(OC910_RES, 0);
        }
        c.g[A0][0] = OC910_THIS as u64;
        c.g[A1][0] = OC910_WIDGET as u64;
        c.g[A2][0] = event as u64;
        c.pc = 0x001b_31a0;
    }

    /// `cFEStateChatRoom_OnWidgetAction`: the chat-room button switch (select
    /// a player, highlight, keyboard, messenger, leave).
    #[test]
    fn c_fe_state_chat_room_on_widget_action_matches() {
        check(
            ee::s03::f_001b31a0__generated,
            c_fe_state_chat_room_on_widget_action,
            240,
            &oc910_chat_setup,
        );
    }

    #[test]
    fn c_fe_state_chat_room_on_widget_action_paths() {
        let seen = oc910_paths(ee::s03::f_001b31a0__generated, 240, &oc910_chat_setup);
        assert!(seen.len() >= 12, "only {} paths: {seen:?}", seen.len());
    }
}
