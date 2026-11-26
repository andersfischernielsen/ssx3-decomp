//! Staging module oc918 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc918`.

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
        0x001a_fa60,
        net_fe_show_player_stats,
        "NetFE_ShowPlayerStats",
    ),
    (
        0x001b_0150,
        c_fe_state_challenge_received_popup_on_widget_created,
        "cFEStateChallengeReceivedPopup_OnWidgetCreated",
    ),
    (
        0x001b_0850,
        c_fe_state_challenge_received_popup_fill_challenge_text,
        "cFEStateChallengeReceivedPopup_FillChallengeText",
    ),
    (
        0x001b_1b98,
        c_fe_state_online_character_select_show_char_stats,
        "cFEStateOnlineCharacterSelect_ShowCharStats",
    ),
    (
        0x001b_2070,
        c_fe_state_chat_room_init,
        "cFEStateChatRoom_Init",
    ),
];

/// `void NetFE_ShowPlayerStats(cUIScreen* screen, FindPlayerEvent* msg)` (0x001afa60):
/// the net lobby's player-stats panel. Copies the opponent name and stats into
/// LobbyMan, then fills the name, rank, games-played, win-ratio (percent) and
/// disconnect text widgets of the screen.
#[allow(
    unused_assignments,
    unused_labels,
    unused_variables,
    unused_mut,
    clippy::all
)]
pub fn net_fe_show_player_stats(c: &mut Cpu) {
    guest(c, |c| {
        'b_1afb08: {
            c.cycles += 40;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x190));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
            c.g[S2][0] = c.g[A1][0];
            c.g[S4][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
            c.g[A0][0] = c.g[SP][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x98));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
            c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(0x18));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x180));
            call_ptr(c, 0x00317fe8, 0x001a_faa0)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            c.ldl(2, (c.g[S2][0] as u32).wrapping_add(0x1f));
            c.ldr(2, (c.g[S2][0] as u32).wrapping_add(0x18));
            c.ldl(4, (c.g[S2][0] as u32).wrapping_add(0x27));
            c.ldr(4, (c.g[S2][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28));
            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0xd3));
            c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0xcc));
            c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xdb));
            c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0xd4));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xdc), c.g[A1][0]);
            c.g[A1][0] = c.lw((c.g[SP][0] as u32));
            c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            call_ptr(c, 0x0025dd08, 0x001a_fadc)?;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3108));
            call_ptr(c, 0x00317670, 0x001a_fae8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0039d860, 0x001a_faf4)?;
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] == 0u64;
            c.g[A1][0] = c.lw((c.g[SP][0] as u32));
            if t {
                break 'b_1afb08;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, 0x003a0c50, 0x001a_fb08)?;
                break 'b_1afb08;
            }
        }
        'b_1afb80: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x34e8));
            call_ptr(c, 0x00317670, 0x001a_fb14)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0039d860, 0x001a_fb20)?;
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_1afb80;
            } else {
                c.cycles += 21;
                c.g[A0][0] = 0x460000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3550));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                call_ptr(c, 0x00317670, 0x001a_fb48)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x001a_fb58)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = 0x4a0000;
                c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a88));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                call_ptr(c, 0x002c2718, 0x001a_fb74)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0e90, 0x001a_fb80)?;
                break 'b_1afb80;
            }
        }
        'b_1afbf8: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x34f8));
            call_ptr(c, 0x00317670, 0x001a_fb8c)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0039d860, 0x001a_fb98)?;
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_1afbf8;
            } else {
                c.cycles += 21;
                c.g[A0][0] = 0x460000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3568));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                call_ptr(c, 0x00317670, 0x001a_fbc0)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x001a_fbd0)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = 0x4a0000;
                c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a88));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                call_ptr(c, 0x002c2718, 0x001a_fbec)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0e90, 0x001a_fbf8)?;
                break 'b_1afbf8;
            }
        }
        'b_1afcdc: {
            c.cycles += 9;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3508));
            call_ptr(c, 0x00317670, 0x001a_fc04)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x0039d860, 0x001a_fc10)?;
            c.g[S3][0] = c.g[V0][0];
            if c.g[S3][0] == 0u64 {
                c.g[A0][0] = 0x460000;
                break 'b_1afcdc;
            } else {
                'b_1afc6c: {
                    'b_1afc68: {
                        'b_1afc58: {
                            c.cycles += 6;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x1c));
                            c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x20));
                            c.mtc1(0, 20);
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                            let t = c.g[V0][0] == 0u64;

                            if t {
                                break 'b_1afc58;
                            } else {
                                c.cycles += 9;
                                c.mtc1(3, 0);
                                c.cvt_s_w(0, 0);
                                c.g[AT][0] = 0x42c80000;
                                c.mtc1(1, 2);
                                c.mtc1(5, 1);
                                c.cvt_s_w(1, 1);
                                c.div_s(0, 0, 1);
                                let t = true;
                                c.mul_s(20, 0, 2);
                                if t {
                                    break 'b_1afc68;
                                } else {
                                    break 'b_1afc58;
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V1][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        if t {
                            break 'b_1afc6c;
                        } else {
                            c.cycles += 2;
                            c.g[AT][0] = 0x42c80000;
                            c.mtc1(1, 20);
                            break 'b_1afc68;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    break 'b_1afc6c;
                }
                c.cycles += 28;
                c.g[A0][0] = 0x460000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3580));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                call_ptr(c, 0x00317670, 0x001a_fc88)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x001a_fc98)?;
                c.g[S0][0] = 0x460000;
                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.mov_s(12, 20);
                c.g[S2][0] = c.g[V0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3590));
                call_ptr(c, 0x00413af8, 0x001a_fcb0)?;
                c.g[T0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = c.g[S0][0];
                c.g[A3][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.g[S1][0];
                c.g[T1][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x002c2718, 0x001a_fccc)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003a0e90, 0x001a_fcd8)?;
                c.g[A0][0] = 0x460000;
                break 'b_1afcdc;
            }
        }
        'b_1afd50: {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3520));
            call_ptr(c, 0x00317670, 0x001a_fce4)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001a_fcf0)?;
            c.g[S3][0] = c.g[V0][0];
            let t = c.g[S3][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if t {
                break 'b_1afd50;
            } else {
                c.cycles += 21;
                c.g[A0][0] = 0x460000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x35a8));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                call_ptr(c, 0x00317670, 0x001a_fd18)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x001a_fd28)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A3][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a88));
                c.g[A2][0] = c.g[V0][0];
                call_ptr(c, 0x002c2718, 0x001a_fd44)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0e90, 0x001a_fd50)?;
                break 'b_1afd50;
            }
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x001a_fd5c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x180));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateChallengeReceivedPopup_OnWidgetCreated(self, cUIThing* widget)`
/// (0x001b0150): per-widget setup of the "challenge received" popup, chosen by
/// the widget's name hash (title, accept/decline buttons, opponent name, the
/// peak/course/event labels and the ranked, CPU-riders, power-ups, multipliers
/// and point-icons result texts).
#[allow(
    unused_assignments,
    unused_labels,
    unused_variables,
    unused_mut,
    clippy::all
)]
pub fn c_fe_state_challenge_received_popup_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b0520: {
            'b_1b0500: {
                'b_1b04e4: {
                    c.cycles += 13;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                    c.g[A0][0] = 0x460000;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1520));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    call_ptr(c, 0x00317670, 0x001b_017c)?;
                    if c.g[S0][0] == c.g[V0][0] {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        break 'b_1b04e4;
                    } else {
                        'b_1b0240: {
                            'b_1b021c: {
                                'b_1b01bc: {
                                    c.cycles += 6;
                                    c.g[A0][0] = 0x460000;
                                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x35f8));
                                    call_ptr(c, 0x00317670, 0x001b_0194)?;
                                    if c.g[S0][0] != c.g[V0][0] {
                                        c.g[A0][0] = 0x460000;
                                        break 'b_1b01bc;
                                    } else {
                                        c.cycles += 8;
                                        c.g[A0][0] = 0x460000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3608));
                                        call_ptr(c, 0x00317670, 0x001b_01a8)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x003a1000, 0x001b_01b4)?;
                                        let t = true;
                                        c.g[V0][0] = 0xb;
                                        if t {
                                            break 'b_1b021c;
                                        } else {
                                            break 'b_1b01bc;
                                        }
                                    }
                                }
                                'b_1b01d8: {
                                    c.cycles += 5;
                                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3618));
                                    call_ptr(c, 0x00317670, 0x001b_01c8)?;
                                    if c.g[S0][0] != c.g[V0][0] {
                                        c.g[A0][0] = 0x460000;
                                        break 'b_1b01d8;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[V0][0] = 0xc;
                                        if t {
                                            break 'b_1b021c;
                                        } else {
                                            break 'b_1b01d8;
                                        }
                                    }
                                }
                                'b_1b0204: {
                                    c.cycles += 5;
                                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3108));
                                    call_ptr(c, 0x00317670, 0x001b_01e4)?;
                                    if c.g[S0][0] != c.g[V0][0] {
                                        c.g[A0][0] = 0x460000;
                                        break 'b_1b0204;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x98));
                                        call_ptr(c, 0x003a0c50, 0x001b_01fc)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_1b04e4;
                                        } else {
                                            break 'b_1b0204;
                                        }
                                    }
                                }
                                c.cycles += 5;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3118));
                                call_ptr(c, 0x00317670, 0x001b_0210)?;
                                if c.g[S0][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1b0240;
                                } else {
                                    c.cycles += 1;
                                    c.g[V0][0] = 0x7;
                                    break 'b_1b021c;
                                }
                            }
                            c.cycles += 9;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001b_0238)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                            if t {
                                break 'b_1b0520;
                            } else {
                                break 'b_1b0240;
                            }
                        }
                        'b_1b0274: {
                            c.cycles += 5;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3140));
                            call_ptr(c, 0x00317670, 0x001b_024c)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1b0274;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0x460000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3150));
                                call_ptr(c, 0x00317670, 0x001b_0260)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x003a1000, 0x001b_026c)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_1b04e4;
                                } else {
                                    break 'b_1b0274;
                                }
                            }
                        }
                        'b_1b02a8: {
                            c.cycles += 5;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3168));
                            call_ptr(c, 0x00317670, 0x001b_0280)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1b02a8;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0x460000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3178));
                                call_ptr(c, 0x00317670, 0x001b_0294)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x003a1000, 0x001b_02a0)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_1b04e4;
                                } else {
                                    break 'b_1b02a8;
                                }
                            }
                        }
                        'b_1b02dc: {
                            c.cycles += 5;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3190));
                            call_ptr(c, 0x00317670, 0x001b_02b4)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x460000;
                                break 'b_1b02dc;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0x460000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x31a0));
                                call_ptr(c, 0x00317670, 0x001b_02c8)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x003a1000, 0x001b_02d4)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_1b04e4;
                                } else {
                                    break 'b_1b02dc;
                                }
                            }
                        }
                        'b_1b04e0: {
                            c.cycles += 5;
                            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x31b8));
                            call_ptr(c, 0x00317670, 0x001b_02e8)?;
                            let t = c.g[S0][0] == c.g[V0][0];
                            c.g[V0][0] = 0x460000;
                            if t {
                                break 'b_1b04e0;
                            } else {
                                'b_1b0320: {
                                    c.cycles += 6;
                                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                    c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3628));
                                    c.g[A0][0] = c.g[S3][0];
                                    call_ptr(c, 0x00317670, 0x001b_0300)?;
                                    let t = c.g[S0][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x460000;
                                    if t {
                                        break 'b_1b0320;
                                    } else {
                                        c.cycles += 6;
                                        c.g[A1][0] = 0x4a0000;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a90));
                                        call_ptr(c, 0x003a0c50, 0x001b_0318)?;
                                        let t = true;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        if t {
                                            break 'b_1b04e4;
                                        } else {
                                            break 'b_1b0320;
                                        }
                                    }
                                }
                                'b_1b048c: {
                                    'b_1b0470: {
                                        'b_1b0388: {
                                            c.cycles += 6;
                                            c.g[S0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                            c.g[S2][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x3640));
                                            c.g[A0][0] = c.g[S2][0];
                                            call_ptr(c, 0x00317670, 0x001b_0330)?;
                                            let t = c.g[S0][0] != c.g[V0][0];
                                            c.g[A0][0] = 0x460000;
                                            if t {
                                                break 'b_1b0388;
                                            } else {
                                                'b_1b0368: {
                                                    c.cycles += 9;
                                                    call_ptr(c, 0x0014dc80, 0x001b_0340)?;
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[A1][0] = 0u64;
                                                    call_ptr(c, 0x0014dd58, 0x001b_034c)?;
                                                    c.g[V1][0] = 0x530000;
                                                    c.g[A2][0] = c
                                                        .lw((c.g[V1][0] as u32)
                                                            .wrapping_add(0x5c04));
                                                    let t = c.g[A2][0] != 0u64;
                                                    c.g[A1][0] = 0x4a0000;
                                                    if t {
                                                        break 'b_1b0368;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[A0][0] = 0x460000;
                                                        let t = true;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x3228));
                                                        if t {
                                                            break 'b_1b0470;
                                                        } else {
                                                            break 'b_1b0368;
                                                        }
                                                    }
                                                }
                                                c.cycles += 8;
                                                c.g[A0][0] = c.g[SP][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
                                                call_ptr(c, 0x00417828, 0x001b_0374)?;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[A1][0] = c.g[SP][0];
                                                call_ptr(c, 0x003a0c50, 0x001b_0380)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                if t {
                                                    break 'b_1b0520;
                                                } else {
                                                    break 'b_1b0388;
                                                }
                                            }
                                        }
                                        'b_1b0400: {
                                            c.cycles += 5;
                                            c.g[S0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x3658));
                                            call_ptr(c, 0x00317670, 0x001b_0394)?;
                                            if c.g[S0][0] != c.g[V0][0] {
                                                c.g[A0][0] = 0x460000;
                                                break 'b_1b0400;
                                            } else {
                                                'b_1b03ec: {
                                                    'b_1b03d4: {
                                                        c.cycles += 6;
                                                        c.g[V1][0] = 0x530000;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8d0));
                                                        c.g[V0][0] =
                                                            sx(((c.g[V0][0] as u32) as i32 >> 8)
                                                                as u32);
                                                        c.g[V0][0] = c.g[V0][0] & 0x1;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A0][0] = 0x460000;
                                                        if t {
                                                            break 'b_1b03d4;
                                                        } else {
                                                            c.cycles += 8;
                                                            c.g[A0][0] = 0x460000;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x328));
                                                            call_ptr(c, 0x00317670, 0x001b_03c0)?;
                                                            c.g[A1][0] = c.g[V0][0];
                                                            c.g[A0][0] = c.g[S1][0];
                                                            call_ptr(c, 0x003a1000, 0x001b_03cc)?;
                                                            let t = true;
                                                            c.g[A1][0] = 0x4a0000;
                                                            if t {
                                                                break 'b_1b03ec;
                                                            } else {
                                                                break 'b_1b03d4;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 6;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x2338));
                                                    call_ptr(c, 0x00317670, 0x001b_03dc)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[A0][0] = c.g[S1][0];
                                                    call_ptr(c, 0x003a1000, 0x001b_03e8)?;
                                                    c.g[A1][0] = 0x4a0000;
                                                    break 'b_1b03ec;
                                                }
                                                c.cycles += 5;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1a90));
                                                call_ptr(c, 0x003a0c50, 0x001b_03f8)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                                if t {
                                                    break 'b_1b0520;
                                                } else {
                                                    break 'b_1b0400;
                                                }
                                            }
                                        }
                                        'b_1b0468: {
                                            'b_1b0434: {
                                                c.cycles += 5;
                                                c.g[S0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x3670));
                                                call_ptr(c, 0x00317670, 0x001b_040c)?;
                                                if c.g[S0][0] != c.g[V0][0] {
                                                    c.g[A0][0] = 0x460000;
                                                    break 'b_1b0434;
                                                } else {
                                                    c.cycles += 6;
                                                    c.g[V1][0] = 0x530000;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x8d0)
                                                        );
                                                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32
                                                        >> 6)
                                                        as u32);
                                                    c.g[V0][0] = c.g[V0][0] & 0x1;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[A0][0] = 0x460000;
                                                    if t {
                                                        break 'b_1b0468;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x328));
                                                        if t {
                                                            break 'b_1b0470;
                                                        } else {
                                                            break 'b_1b0434;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 5;
                                            c.g[S0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x3688));
                                            call_ptr(c, 0x00317670, 0x001b_0440)?;
                                            if c.g[S0][0] != c.g[V0][0] {
                                                c.g[A0][0] = 0x460000;
                                                break 'b_1b048c;
                                            } else {
                                                c.cycles += 6;
                                                c.g[V1][0] = 0x530000;
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                                                c.g[V0][0] =
                                                    sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A0][0] = 0x460000;
                                                if t {
                                                    break 'b_1b0468;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                                    if t {
                                                        break 'b_1b0470;
                                                    } else {
                                                        break 'b_1b0468;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        c.g[A0][0] = 0x460000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                        break 'b_1b0470;
                                    }
                                    c.cycles += 7;
                                    call_ptr(c, 0x00317670, 0x001b_0478)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x001b_0484)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    if t {
                                        break 'b_1b0520;
                                    } else {
                                        break 'b_1b048c;
                                    }
                                }
                                c.cycles += 5;
                                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x36a0));
                                call_ptr(c, 0x00317670, 0x001b_0498)?;
                                if c.g[S0][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    break 'b_1b04e4;
                                } else {
                                    c.cycles += 6;
                                    c.g[A0][0] = 0x460000;
                                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x36b0));
                                    call_ptr(c, 0x00317670, 0x001b_04b0)?;
                                    if c.g[S0][0] == c.g[V0][0] {
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        break 'b_1b04e4;
                                    } else {
                                        c.cycles += 5;
                                        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x00317670, 0x001b_04c4)?;
                                        if c.g[S0][0] == c.g[V0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                            break 'b_1b04e4;
                                        } else {
                                            c.cycles += 5;
                                            c.g[S0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                            c.g[A0][0] = c.g[S2][0];
                                            call_ptr(c, 0x00317670, 0x001b_04d8)?;
                                            if c.g[S0][0] != c.g[V0][0] {
                                                c.g[A0][0] = 0x460000;
                                                break 'b_1b0500;
                                            } else {
                                                break 'b_1b04e0;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        break 'b_1b04e4;
                    }
                }
                c.cycles += 7;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x001b_04f8)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                if t {
                    break 'b_1b0520;
                } else {
                    break 'b_1b0500;
                }
            }
            c.cycles += 5;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30e0));
            call_ptr(c, 0x00317670, 0x001b_050c)?;
            let t = c.g[S0][0] != c.g[V0][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            if t {
                break 'b_1b0520;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.g[V0][0] | 0x80;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                break 'b_1b0520;
            }
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
    })
}

/// `void cFEStateChallengeReceivedPopup_FillChallengeText(cUIScreen* screen, iface)`
/// (0x001b0850): fill the event, course, ranked, power-ups, multipliers,
/// point-icons and CPU-riders result texts of the challenge popup (a blank
/// string when `iface` is null).
#[allow(
    unused_assignments,
    unused_labels,
    unused_variables,
    unused_mut,
    clippy::all
)]
pub fn c_fe_state_challenge_received_popup_fill_challenge_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b0bf0: {
            'b_1b0bec: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A2][0];
                let t = c.g[S2][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                if t {
                    break 'b_1b0bec;
                } else {
                    'b_1b08c8: {
                        c.cycles += 9;
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x36a0));
                        call_ptr(c, 0x00317670, 0x001b_087c)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039d860, 0x001b_0888)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A0][0] = 0x460000;
                            break 'b_1b08c8;
                        } else {
                            'b_1b08b0: {
                                c.cycles += 2;
                                let t = c.g[S1][0] != 0u64;
                                c.g[A1][0] = 0x4a0000;
                                if t {
                                    break 'b_1b08b0;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1390));
                                    call_ptr(c, 0x003a0c50, 0x001b_08a8)?;
                                    let t = true;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1b08c8;
                                    } else {
                                        break 'b_1b08b0;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x00145340, 0x001b_08b8)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a0c50, 0x001b_08c4)?;
                            c.g[A0][0] = 0x460000;
                            break 'b_1b08c8;
                        }
                    }
                    'b_1b091c: {
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x36b0));
                        call_ptr(c, 0x00317670, 0x001b_08d0)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039d860, 0x001b_08dc)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A0][0] = 0x460000;
                            break 'b_1b091c;
                        } else {
                            'b_1b0904: {
                                c.cycles += 2;
                                let t = c.g[S1][0] != 0u64;
                                c.g[A1][0] = 0x4a0000;
                                if t {
                                    break 'b_1b0904;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1390));
                                    call_ptr(c, 0x003a0c50, 0x001b_08fc)?;
                                    let t = true;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1b091c;
                                    } else {
                                        break 'b_1b0904;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x00144bf0, 0x001b_090c)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a0c50, 0x001b_0918)?;
                            c.g[A0][0] = 0x460000;
                            break 'b_1b091c;
                        }
                    }
                    'b_1b09b0: {
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3628));
                        call_ptr(c, 0x00317670, 0x001b_0924)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039d860, 0x001b_0930)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A0][0] = 0x460000;
                            break 'b_1b09b0;
                        } else {
                            'b_1b095c: {
                                c.cycles += 2;
                                let t = c.g[S1][0] != 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_1b095c;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1390));
                                    call_ptr(c, 0x003a0c50, 0x001b_0954)?;
                                    let t = true;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1b09b0;
                                    } else {
                                        break 'b_1b095c;
                                    }
                                }
                            }
                            'b_1b0998: {
                                c.cycles += 7;
                                c.g[A1][0] = 0x7;
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                call_ptr(c, 0x0014dd58, 0x001b_0968)?;
                                c.g[V1][0] = 0x530000;
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b34));
                                if c.g[V0][0] == 0u64 {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1b0998;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                    call_ptr(c, 0x00317670, 0x001b_0984)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x001b_0990)?;
                                    let t = true;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1b09b0;
                                    } else {
                                        break 'b_1b0998;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                            call_ptr(c, 0x00317670, 0x001b_09a0)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001b_09ac)?;
                            c.g[A0][0] = 0x460000;
                            break 'b_1b09b0;
                        }
                    }
                    'b_1b0a3c: {
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3798));
                        call_ptr(c, 0x00317670, 0x001b_09b8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039d860, 0x001b_09c4)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A0][0] = 0x460000;
                            break 'b_1b0a3c;
                        } else {
                            'b_1b09f0: {
                                c.cycles += 2;
                                let t = c.g[S1][0] != 0u64;
                                c.g[V1][0] = 0x530000;
                                if t {
                                    break 'b_1b09f0;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1390));
                                    call_ptr(c, 0x003a0c50, 0x001b_09e8)?;
                                    let t = true;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1b0a3c;
                                    } else {
                                        break 'b_1b09f0;
                                    }
                                }
                            }
                            'b_1b0a24: {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 8) as u32);
                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                if c.g[V0][0] == 0u64 {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1b0a24;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                    call_ptr(c, 0x00317670, 0x001b_0a10)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x001b_0a1c)?;
                                    let t = true;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1b0a3c;
                                    } else {
                                        break 'b_1b0a24;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                            call_ptr(c, 0x00317670, 0x001b_0a2c)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001b_0a38)?;
                            c.g[A0][0] = 0x460000;
                            break 'b_1b0a3c;
                        }
                    }
                    'b_1b0ac8: {
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3670));
                        call_ptr(c, 0x00317670, 0x001b_0a44)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039d860, 0x001b_0a50)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A0][0] = 0x460000;
                            break 'b_1b0ac8;
                        } else {
                            'b_1b0a7c: {
                                c.cycles += 2;
                                let t = c.g[S1][0] != 0u64;
                                c.g[V1][0] = 0x530000;
                                if t {
                                    break 'b_1b0a7c;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1390));
                                    call_ptr(c, 0x003a0c50, 0x001b_0a74)?;
                                    let t = true;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1b0ac8;
                                    } else {
                                        break 'b_1b0a7c;
                                    }
                                }
                            }
                            'b_1b0ab0: {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                if c.g[V0][0] == 0u64 {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1b0ab0;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                    call_ptr(c, 0x00317670, 0x001b_0a9c)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x001b_0aa8)?;
                                    let t = true;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1b0ac8;
                                    } else {
                                        break 'b_1b0ab0;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                            call_ptr(c, 0x00317670, 0x001b_0ab8)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001b_0ac4)?;
                            c.g[A0][0] = 0x460000;
                            break 'b_1b0ac8;
                        }
                    }
                    'b_1b0b54: {
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3688));
                        call_ptr(c, 0x00317670, 0x001b_0ad0)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x0039d860, 0x001b_0adc)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A0][0] = 0x460000;
                            break 'b_1b0b54;
                        } else {
                            'b_1b0b08: {
                                c.cycles += 2;
                                let t = c.g[S1][0] != 0u64;
                                c.g[V1][0] = 0x530000;
                                if t {
                                    break 'b_1b0b08;
                                } else {
                                    c.cycles += 6;
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1390));
                                    call_ptr(c, 0x003a0c50, 0x001b_0b00)?;
                                    let t = true;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1b0b54;
                                    } else {
                                        break 'b_1b0b08;
                                    }
                                }
                            }
                            'b_1b0b3c: {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                                c.g[V0][0] = c.g[V0][0] & 0x1;
                                if c.g[V0][0] == 0u64 {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1b0b3c;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                    call_ptr(c, 0x00317670, 0x001b_0b28)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x001b_0b34)?;
                                    let t = true;
                                    c.g[A0][0] = 0x460000;
                                    if t {
                                        break 'b_1b0b54;
                                    } else {
                                        break 'b_1b0b3c;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                            call_ptr(c, 0x00317670, 0x001b_0b44)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001b_0b50)?;
                            c.g[A0][0] = 0x460000;
                            break 'b_1b0b54;
                        }
                    }
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3640));
                    call_ptr(c, 0x00317670, 0x001b_0b5c)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001b_0b68)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        break 'b_1b0bf0;
                    } else {
                        'b_1b0b90: {
                            c.cycles += 2;
                            let t = c.g[S1][0] != 0u64;
                            c.g[A1][0] = 0x4a0000;
                            if t {
                                break 'b_1b0b90;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1390));
                                call_ptr(c, 0x003a0c50, 0x001b_0b88)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_1b0bf0;
                                } else {
                                    break 'b_1b0b90;
                                }
                            }
                        }
                        'b_1b0bd4: {
                            c.cycles += 9;
                            call_ptr(c, 0x0014dc80, 0x001b_0b98)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x0014dd58, 0x001b_0ba4)?;
                            c.g[V1][0] = 0x530000;
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c04));
                            let t = (c.g[A2][0] as i64) > 0;
                            c.g[A1][0] = 0x4a0000;
                            if t {
                                break 'b_1b0bd4;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0x460000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3228));
                                call_ptr(c, 0x00317670, 0x001b_0bc0)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x003a1000, 0x001b_0bcc)?;
                                let t = true;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                                if t {
                                    break 'b_1b0bf0;
                                } else {
                                    break 'b_1b0bd4;
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
                        call_ptr(c, 0x00417828, 0x001b_0be0)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, 0x003a0c50, 0x001b_0bec)?;
                        break 'b_1b0bec;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1b0bf0;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateOnlineCharacterSelect_ShowCharStats(self, index, show, opponent)`
/// (0x001b1b98): show or hide a character's stat panel (index below 31) and
/// fill the seven stat value texts and progress bars, from the opponent's
/// attributes, a fixed 40 for ranked games, or the human stat interface.
#[allow(
    unused_assignments,
    unused_labels,
    unused_variables,
    unused_mut,
    clippy::all
)]
pub fn c_fe_state_online_character_select_show_char_stats(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b1e90: {
            'b_1b1e8c: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.g[S1][0] = c.g[A1][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
                c.g[S0][0] = c.g[A3][0];
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[S2][0] = c.g[A2][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
                c.g[S7][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
                c.g[V0][0] = (c.g[S1][0] < 0x1fu64) as u64;
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
                let t = c.g[V0][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[RA][0]);
                if t {
                    break 'b_1b1e8c;
                } else {
                    'b_1b1c3c: {
                        'b_1b1c14: {
                            c.cycles += 2;
                            let t = c.g[S2][0] == 0u64;
                            c.g[V0][0] = 0x440000;
                            if t {
                                break 'b_1b1c14;
                            } else {
                                c.cycles += 12;
                                c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12c8));
                                c.g[A1][0] = 0x4a0000;
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[A3][0] = 0x4a0000;
                                c.g[A2][0] = c.lw((c.g[V1][0] as u32));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x19f8));
                                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1b18));
                                c.g[A0][0] = c.g[SP][0];
                                call_ptr(c, 0x00417828, 0x001b_1c0c)?;
                                let t = true;

                                if t {
                                    break 'b_1b1c3c;
                                } else {
                                    break 'b_1b1c14;
                                }
                            }
                        }
                        c.cycles += 10;
                        c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12c8));
                        c.g[A1][0] = 0x4a0000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.g[A3][0] = 0x4a0000;
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x19f8));
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1b20));
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x00417828, 0x001b_1c3c)?;
                        break 'b_1b1c3c;
                    }
                    'b_1b1c70: {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x00317670, 0x001b_1c44)?;
                        c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001b_1c50)?;
                        c.g[A2][0] = c.g[V0][0];
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = c.g[S0][0];
                        if t {
                            break 'b_1b1c70;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001b_1c70)?;
                            break 'b_1b1c70;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                    if t {
                        break 'b_1b1e90;
                    } else {
                        'b_1b1dcc: {
                            'b_1b1cec: {
                                c.cycles += 12;
                                call_ptr(c, 0x0014dc80, 0x001b_1c80)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x3;
                                call_ptr(c, 0x0014dd58, 0x001b_1c8c)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = 0x7;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                call_ptr(c, 0x0014dd58, 0x001b_1ca0)?;
                                let t = c.g[S2][0] == 0u64;
                                c.g[V0][0] = 0x440000;
                                if t {
                                    break 'b_1b1cec;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = 0x440000;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1350));
                                    c.g[V1][0] = 0x530000;
                                    c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1378));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x60), c.g[V0][0]);
                                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4fd8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                                    'l_1b1cc8: loop {
                                        c.cycles += 7;
                                        c.g[V0][0] =
                                            sx((c.g[A2][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                        c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] = ((c.g[A1][0] as i64) < 7i64) as u64;
                                        c.sw((c.g[A0][0] as u32), c.g[V1][0]);
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001b1cc8);
                                            }
                                            continue 'l_1b1cc8;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[S4][0] = 0u64;
                                            if t {
                                                break 'b_1b1dcc;
                                            } else {
                                                break 'b_1b1cec;
                                            }
                                        }
                                    }
                                }
                            }
                            'b_1b1d3c: {
                                c.cycles += 8;
                                c.g[V0][0] = 0x530000;
                                c.g[A0][0] = 0x440000;
                                c.g[V1][0] = 0x440000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1340));
                                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4b34));
                                c.g[FP][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1360));
                                let t = c.g[A1][0] == 0u64;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x60), c.g[A0][0]);
                                if t {
                                    break 'b_1b1d3c;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x28;
                                    c.g[V1][0] = 0x6;
                                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x48));
                                    'l_1b1d18: loop {
                                        c.cycles += 7;
                                        c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                        let t = (c.g[V1][0] as i64) >= 0;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x001b1d18);
                                            }
                                            continue 'l_1b1d18;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[S4][0] = 0u64;
                                            if t {
                                                break 'b_1b1dcc;
                                            } else {
                                                break 'b_1b1d3c;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 36;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = c.g[S1][0];
                            call_ptr(c, 0x00148bc8, 0x001b_1d4c)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = c.g[S1][0];
                            call_ptr(c, 0x00148c68, 0x001b_1d60)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = c.g[S1][0];
                            call_ptr(c, 0x00148b78, 0x001b_1d74)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = c.g[S1][0];
                            call_ptr(c, 0x00148cb8, 0x001b_1d88)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = c.g[S1][0];
                            call_ptr(c, 0x00148d58, 0x001b_1d9c)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x40), c.g[V0][0]);
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0u64;
                            c.g[A2][0] = c.g[S1][0];
                            call_ptr(c, 0x00148d08, 0x001b_1db0)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x44), c.g[V0][0]);
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A2][0] = c.g[S1][0];
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x00148c18, 0x001b_1dc4)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                            c.g[S4][0] = 0u64;
                            break 'b_1b1dcc;
                        }
                        c.cycles += 3;
                        c.g[S6][0] = 0x5;
                        c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                        'l_1b1dd8: loop {
                            'b_1b1e7c: {
                                c.cycles += 16;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x60));
                                c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x30));
                                c.sb((c.g[A0][0] as u32), c.g[S0][0]);
                                call_ptr(c, 0x00317670, 0x001b_1de8)?;
                                c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001b_1df4)?;
                                c.sb((c.g[FP][0] as u32), c.g[S0][0]);
                                c.g[A0][0] = c.g[FP][0];
                                c.g[S1][0] = c.g[V0][0];
                                call_ptr(c, 0x00317670, 0x001b_1e04)?;
                                c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001b_1e10)?;
                                let t = c.g[S1][0] == 0u64;
                                c.g[S2][0] = c.g[V0][0];
                                if t {
                                    break 'b_1b1e7c;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S2][0] == 0u64;
                                    c.g[V0][0] = 0x4a0000;
                                    if t {
                                        break 'b_1b1e7c;
                                    } else {
                                        'b_1b1e38: {
                                            c.cycles += 6;
                                            c.g[S0][0] = c.lw((c.g[S3][0] as u32));
                                            c.g[A0][0] = c.g[S5][0];
                                            c.g[A1][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x12c0));
                                            c.div(0, 16, 22);
                                            if c.g[S6][0] == 0u64 {
                                                break 'b_1b1e38;
                                            } else {
                                                break 'b_1b1e38;
                                            }
                                        }
                                        c.cycles += 17;
                                        c.g[A2][0] = c.lo[0];
                                        call_ptr(c, 0x00417828, 0x001b_1e44)?;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x1;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001b_1e5c)?;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = c.g[S5][0];
                                        call_ptr(c, 0x003a0c50, 0x001b_1e68)?;
                                        c.g[V0][0] = 0x37;
                                        c.g[A0][0] = c.g[S2][0];
                                        c.sw((c.g[S2][0] as u32).wrapping_add(0x80), c.g[V0][0]);
                                        c.g[A1][0] = c.g[S0][0];
                                        call_ptr(c, 0x0039c428, 0x001b_1e7c)?;
                                        break 'b_1b1e7c;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = ((c.g[S4][0] as i64) < 7i64) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001b1dd8);
                                }
                                continue 'l_1b1dd8;
                            } else {
                                break 'b_1b1e8c;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
            break 'b_1b1e90;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x70));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        return Ok(());
    })
}

/// `void cFEStateChatRoom_Init(self)` (0x001b2070): create the chat room
/// screen, select the player's persona, set up the chat text scroll and the
/// two scroll bars (arrows, animation names, hide-on-empty state).
#[allow(
    unused_assignments,
    unused_labels,
    unused_variables,
    unused_mut,
    clippy::all
)]
pub fn c_fe_state_chat_room_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b227c: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = 0x460000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3a08));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001b_20a0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001b_20b4)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1b227c;
            } else {
                'b_1b21c8: {
                    'b_1b21c0: {
                        c.cycles += 63;
                        c.g[A2][0] = 0u64;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0039c870, 0x001b_20cc)?;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = 0x1;
                        Callee {
                            addr: 0x001b2378,
                            f: ee::s03::f_001b2378,
                        }
                        .call(c, 0x001b_20d8)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x80));
                        Callee {
                            addr: 0x001b3978,
                            f: ee::s03::f_001b3978,
                        }
                        .call(c, 0x001b_20e8)?;
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6f0));
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x74));
                        c.g[V0][0] = c.g[V0][0] | 0x1;
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6f0));
                        call_ptr(c, 0x003a1310, 0x001b_2104)?;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6f0));
                        c.g[A0][0] = 0x460000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19c8));
                        c.ldl(3, (c.g[S3][0] as u32).wrapping_add(0x733));
                        c.ldr(3, (c.g[S3][0] as u32).wrapping_add(0x72c));
                        c.ldl(5, (c.g[S3][0] as u32).wrapping_add(0x73b));
                        c.ldr(5, (c.g[S3][0] as u32).wrapping_add(0x734));
                        c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0xaf));
                        c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0xa8));
                        c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xb7));
                        c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0xb0));
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6f0));
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6f4));
                        c.sw((c.g[V1][0] as u32).wrapping_add(0xb8), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x001b_2140)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001b_214c)?;
                        c.g[A0][0] = 0x460000;
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19d8));
                        call_ptr(c, 0x00317670, 0x001b_215c)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001b_2168)?;
                        c.g[A0][0] = 0x460000;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x19e8));
                        call_ptr(c, 0x00317670, 0x001b_2178)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001b_2184)?;
                        c.g[S4][0] = c.g[V0][0];
                        c.g[V1][0] = 0x4a0000;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6f4));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), c.g[S1][0]);
                        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1b48));
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6f4));
                        c.g[A0][0] = c.g[S1][0];
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x80), c.g[S0][0]);
                        c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6f4));
                        call_ptr(c, 0x00416810, 0x001b_21ac)?;
                        c.g[V0][0] = (c.g[V0][0] < 0x19u64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
                        if t {
                            break 'b_1b21c0;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.sb((c.g[S0][0] as u32).wrapping_add(0x8c), 0u64);
                            if t {
                                break 'b_1b21c8;
                            } else {
                                break 'b_1b21c0;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x004166f4, 0x001b_21c8)?;
                    break 'b_1b21c8;
                }
                'b_1b21f8: {
                    'b_1b21f0: {
                        c.cycles += 8;
                        c.g[V0][0] = 0x4a0000;
                        c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6f4));
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1b50));
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00416810, 0x001b_21dc)?;
                        c.g[V0][0] = (c.g[V0][0] < 0x19u64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa5));
                            break 'b_1b21f0;
                        } else {
                            c.cycles += 2;
                            let t = true;
                            c.sb((c.g[S0][0] as u32).wrapping_add(0xa5), 0u64);
                            if t {
                                break 'b_1b21f8;
                            } else {
                                break 'b_1b21f0;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x004166f4, 0x001b_21f8)?;
                    break 'b_1b21f8;
                }
                c.cycles += 33;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6f4));
                c.g[S2][0] = 0x2;
                c.g[A0][0] = 0x4a0000;
                c.sw((c.g[V0][0] as u32).wrapping_add(0x84), c.g[S4][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b58));
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6f4));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x88), c.g[S2][0]);
                call_ptr(c, 0x00317670, 0x001b_2218)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001b_2224)?;
                c.g[A0][0] = 0x460000;
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3a18));
                call_ptr(c, 0x00317670, 0x001b_2234)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001b_2240)?;
                c.g[A0][0] = 0x460000;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3a28));
                call_ptr(c, 0x00317670, 0x001b_2250)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001b_225c)?;
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6ec));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x7c), c.g[S1][0]);
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6ec));
                c.sw((c.g[A0][0] as u32).wrapping_add(0x80), c.g[S0][0]);
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6ec));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x84), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6ec));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x88), c.g[S2][0]);
                break 'b_1b227c;
            }
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate of the oc918 fakes: only machines that write `OC918_MAGIC` here run them.
    pub(crate) const OC918_CTRL: u32 = DATA + 0x20_0000;
    pub(crate) const OC918_MAGIC: u32 = 0x0c91_8ab5;
    /// Fixture layout (offsets from `DATA`).
    pub(crate) const OC918_LOBBY: u32 = DATA + 0x1000;
    pub(crate) const OC918_SCREEN: u32 = DATA + 0x2000;
    pub(crate) const OC918_ARG: u32 = DATA + 0x3000;
    pub(crate) const OC918_NODES: u32 = DATA + 0x4000;
    pub(crate) const OC918_SELF: u32 = DATA + 0x5000;
    pub(crate) const OC918_WORLD: u32 = DATA + 0x7000;
    pub(crate) const OC918_STAT_OBJ: u32 = DATA + 0x7100;
    pub(crate) const OC918_STAT_VT: u32 = DATA + 0x7200;
    pub(crate) const OC918_IFACE: u32 = DATA + 0x7300;
    pub(crate) const OC918_IFACE_VT: u32 = DATA + 0x7400;
    pub(crate) const OC918_UI_VT: u32 = DATA + 0x7500;

    fn oc918_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// A gated fake of a guest function: logs the call (every register) and
    /// returns; `$ret` makes it return the word at `OC918_CTRL + 4`.
    macro_rules! oc918_gated_fake {
        ($fake:ident, $generated:path, $target:expr, $ret:expr) => {
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC918_CTRL) != OC918_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $target, ra);
                if $ret && c.pc == ra {
                    let r = c.bus.read32(OC918_CTRL + 4);
                    c.g[V0][0] = sx(r);
                }
            }
        };
    }

    oc918_gated_fake!(
        oc918_fake_add_screen,
        ee::s13::f_00397cc0__generated,
        0x0039_7cc0,
        true
    );
    oc918_gated_fake!(
        oc918_fake_play_frame,
        ee::s13::f_0039c870__generated,
        0x0039_c870,
        false
    );
    oc918_gated_fake!(
        oc918_fake_set_input_map,
        ee::s13::f_003a1310__generated,
        0x003a_1310,
        false
    );
    oc918_gated_fake!(
        oc918_fake_refresh_players,
        ee::s03::f_001b2378__generated,
        0x001b_2378,
        false
    );
    oc918_gated_fake!(
        oc918_fake_select_player,
        ee::s03::f_001b3978__generated,
        0x001b_3978,
        false
    );
    oc918_gated_fake!(
        oc918_fake_set_ascii,
        ee::s13::f_003a0c70__generated,
        0x003a_0c70,
        false
    );
    oc918_gated_fake!(
        oc918_fake_set_wide,
        ee::s13::f_003a0eb0__generated,
        0x003a_0eb0,
        false
    );
    oc918_gated_fake!(
        oc918_fake_progress_set,
        ee::s13::f_0039c428__generated,
        0x0039_c428,
        false
    );
    oc918_gated_fake!(
        oc918_fake_event_name,
        ee::s01::f_00145340__generated,
        0x0014_5340,
        false
    );
    oc918_gated_fake!(
        oc918_fake_location_name,
        ee::s01::f_00144bf0__generated,
        0x0014_4bf0,
        false
    );

    /// Hook every oc918 fake once (idempotent; they only act on gated machines).
    pub(crate) fn oc918_install_fakes() {
        let all: [(*mut fn(&mut Cpu), fn(&mut Cpu)); 10] = [
            (
                addr_of_mut!(ee::s13::OVERRIDE_00397CC0),
                oc918_fake_add_screen,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_0039C870),
                oc918_fake_play_frame,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003A1310),
                oc918_fake_set_input_map,
            ),
            (
                addr_of_mut!(ee::s03::OVERRIDE_001B2378),
                oc918_fake_refresh_players,
            ),
            (
                addr_of_mut!(ee::s03::OVERRIDE_001B3978),
                oc918_fake_select_player,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003A0C70),
                oc918_fake_set_ascii,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_003A0EB0),
                oc918_fake_set_wide,
            ),
            (
                addr_of_mut!(ee::s13::OVERRIDE_0039C428),
                oc918_fake_progress_set,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_00145340),
                oc918_fake_event_name,
            ),
            (
                addr_of_mut!(ee::s01::OVERRIDE_00144BF0),
                oc918_fake_location_name,
            ),
        ];
        for (slot, fake) in all {
            fakes::hook(slot, fake, fakes::prev_cell(fake));
        }
    }

    /// `GetHashValue32` of the string at `addr`, run on `c` before the fixture
    /// scrambles it (pc and cycles are put back).
    pub(crate) fn oc918_hash(c: &mut Cpu, addr: u32) -> u32 {
        let (pc, cycles) = (c.pc, c.cycles);
        c.g[SP][0] = sx(crate::difftest::SP);
        c.g[A0][0] = sx(addr);
        c.g[RA][0] = sx(0x1234);
        ee::s10::f_00317670(c);
        let h = c.g[V0][0] as u32;
        c.pc = pc;
        c.cycles = cycles;
        h
    }

    /// Hash of the one-character string `ch` (scratch bytes at `DATA + 0x180`).
    fn oc918_hash_char(c: &mut Cpu, ch: u8) -> u32 {
        c.bus.write8(DATA + 0x180, ch);
        c.bus.write8(DATA + 0x181, 0);
        oc918_hash(c, DATA + 0x180)
    }

    /// Machine with allocator, the BE interface table, a world and the gate.
    pub(crate) fn oc918_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = fakes::machine_rt(c, seed, salt, calls);
        oc918_w(c, OC918_CTRL, OC918_MAGIC);
        oc918_w(c, GAME_GP.wrapping_sub(0x1efc), 0);
        oc918_w(c, GAME_GP.wrapping_sub(0x848), OC918_WORLD);
        // BE interface table (0x5308a8): entry 0, vtable slot +0xc is a fake.
        oc918_w(c, 0x0053_08a8, OC918_IFACE);
        oc918_w(c, OC918_IFACE + 4, OC918_IFACE_VT);
        c.bus.write16(OC918_IFACE_VT + 8, 0);
        oc918_w(c, OC918_IFACE_VT + 0xc, FAKE);
        // World: +0x8c holds the interface object whose vtable slot +0x24 is a fake.
        oc918_w(c, OC918_WORLD + 0x8c, OC918_STAT_OBJ);
        oc918_w(c, OC918_WORLD + 0x78, OC918_STAT_OBJ);
        oc918_w(c, OC918_STAT_OBJ + 4, OC918_STAT_VT);
        c.bus.write16(OC918_STAT_VT + 0x20, 0);
        oc918_w(c, OC918_STAT_VT + 0x24, FAKE);
        // Widget vtable: deltas 0, methods fakes (+0x2c, +0xc4, +0x4c, +0xa4).
        for off in [0x28, 0x48, 0xa0, 0xc0] {
            c.bus.write16(OC918_UI_VT + off, 0);
            oc918_w(c, OC918_UI_VT + off + 4, FAKE);
        }
        r
    }

    /// A UI object: vtable and name hash.
    pub(crate) fn oc918_node(c: &mut Cpu, node: u32, hash: u32) {
        oc918_w(c, node + 8, OC918_UI_VT);
        oc918_w(c, node + 0x38, hash);
    }

    /// A screen whose object list (`+0x3c` array, `+0x38` count holder) is `nodes`.
    pub(crate) fn oc918_screen(c: &mut Cpu, screen: u32, nodes: &[u32]) {
        let arr = screen + 0x200;
        let cnt = screen + 0x300;
        for (i, n) in nodes.iter().enumerate() {
            oc918_w(c, arr + 4 * i as u32, *n);
        }
        oc918_w(c, cnt, nodes.len() as u32);
        oc918_w(c, screen + 0x3c, arr);
        oc918_w(c, screen + 0x38, cnt);
    }

    /// Random words in the BE option globals the functions read.
    fn oc918_globals(c: &mut Cpu, r: &mut Rng) {
        for a in [0x0053_08d0, 0x0053_4b34, 0x0053_5c04] {
            let v = r.next_u32();
            oc918_w(c, a, v);
        }
        // Opponent character attributes (7 bytes) and ranked flag.
        for i in 0..8 {
            let v = r.next_u32() as u8;
            c.bus.write8(0x0053_4fd8 + i, v);
        }
        if r.next_u32() % 3 == 0 {
            oc918_w(c, 0x0053_4b34, 0);
        }
    }

    // -- 0x001afa60 NetFE_ShowPlayerStats ------------------------------

    #[test]
    fn net_fe_show_player_stats_matches() {
        oc918_install_fakes();
        check(
            ee::s03::f_001afa60__generated,
            net_fe_show_player_stats,
            24,
            &|c, seed| {
                // All name strings are empty in a bare machine: one hash fits all.
                let h0 = oc918_hash(c, DATA + 0x100);
                let mut r = oc918_base(c, seed, 0x1afa_60, 6);
                // Every other seed has no widget with that hash.
                oc918_node(c, OC918_NODES, if seed % 2 == 0 { h0 } else { h0 ^ 1 });
                oc918_screen(c, OC918_SCREEN, &[OC918_NODES]);
                // LobbyMan with no peer (the first word is 0), a short opponent name.
                oc918_w(c, GAME_GP.wrapping_sub(0xc8), OC918_LOBBY);
                oc918_w(c, OC918_LOBBY, 0);
                let len = seed % 5;
                for i in 0..len {
                    c.bus.write8(OC918_LOBBY + 0x98 + i, b'A' + i as u8);
                }
                c.bus.write8(OC918_LOBBY + 0x98 + len, 0);
                // The stats message: win ratio below, at and above 1, and no games.
                let games = r.next_u32() % 50 + 1;
                let (wins, games) = match (seed / 2) % 4 {
                    0 => (games / 2, games),
                    1 => (games, games),
                    2 => (0, games),
                    _ => (games + 3, 0),
                };
                fill(c, &mut r, OC918_ARG + 0x14, 8);
                oc918_w(c, OC918_ARG + 0x1c, wins);
                oc918_w(c, OC918_ARG + 0x20, games);
                c.g[A0][0] = OC918_SCREEN as u64;
                c.g[A1][0] = OC918_ARG as u64;
            },
        );
    }

    // -- 0x001b0150 cFEStateChallengeReceivedPopup_OnWidgetCreated ------

    /// Offsets (from 0x460000) of the strings the function compares the widget's
    /// name hash with, or reads as text ids.
    pub(crate) const OC918_POPUP_NAMES: [u32; 20] = [
        0x1520, 0x35f8, 0x3608, 0x3618, 0x3108, 0x3118, 0x3140, 0x3150, 0x3168, 0x3178, 0x3190,
        0x31a0, 0x31b8, 0x3228, 0x3658, 0x3670, 0x3688, 0x36a0, 0x36b0, 0x30e0,
    ];

    #[test]
    fn c_fe_state_challenge_received_popup_on_widget_created_matches() {
        oc918_install_fakes();
        check(
            ee::s03::f_001b0150__generated,
            c_fe_state_challenge_received_popup_on_widget_created,
            44,
            &|c, seed| {
                elf_image(c);
                // The widget's name: one of the compared strings, or none of them.
                let k = (seed % 22) as usize;
                let hash = if k < OC918_POPUP_NAMES.len() {
                    oc918_hash(c, 0x0046_0000 + OC918_POPUP_NAMES[k])
                } else {
                    0x1234_5678 ^ seed
                };
                let mut r = oc918_base(c, seed, 0x1b01_50, 6);
                oc918_globals(c, &mut r);
                oc918_w(c, GAME_GP.wrapping_sub(0xc8), OC918_LOBBY);
                fill(c, &mut r, OC918_LOBBY, 0x40);
                let widget = OC918_NODES;
                fill(c, &mut r, widget, 0x40);
                oc918_node(c, widget, hash);
                c.g[A0][0] = OC918_SELF as u64;
                c.g[A1][0] = widget as u64;
            },
        );
    }

    // -- 0x001b0850 cFEStateChallengeReceivedPopup_FillChallengeText ---

    #[test]
    fn c_fe_state_challenge_received_popup_fill_challenge_text_matches() {
        oc918_install_fakes();
        check(
            ee::s03::f_001b0850__generated,
            c_fe_state_challenge_received_popup_fill_challenge_text,
            40,
            &|c, seed| {
                let h0 = oc918_hash(c, DATA + 0x100);
                let mut r = oc918_base(c, seed, 0x1b08_50, 8);
                oc918_globals(c, &mut r);
                // Screen: null (early exit), no matching widgets, or all of them.
                let screen = if seed % 8 == 0 { 0 } else { OC918_SCREEN };
                oc918_node(c, OC918_NODES, if seed % 4 == 1 { h0 ^ 1 } else { h0 });
                oc918_screen(c, OC918_SCREEN, &[OC918_NODES]);
                // iface null or not.
                let iface = if (seed / 2) % 2 == 0 { 0 } else { OC918_ARG };
                fill(c, &mut r, OC918_ARG, 0x40);
                c.g[A0][0] = screen as u64;
                c.g[A1][0] = iface as u64;
            },
        );
    }

    // -- 0x001b1b98 cFEStateOnlineCharacterSelect_ShowCharStats -------

    #[test]
    fn c_fe_state_online_character_select_show_char_stats_matches() {
        oc918_install_fakes();
        check(
            ee::s03::f_001b1b98__generated,
            c_fe_state_online_character_select_show_char_stats,
            68,
            &|c, seed| {
                // The panel name is empty here; the stat texts and bars are one
                // character ('0'..'6') long.
                let h0 = oc918_hash(c, DATA + 0x100);
                let digit: Vec<u32> = (0..7).map(|i| oc918_hash_char(c, b'0' + i)).collect();
                let mut r = oc918_base(c, seed, 0x1b1b_98, 8);
                oc918_globals(c, &mut r);
                let mask = r.next_u32();
                let mut nodes = Vec::new();
                if seed % 5 != 0 {
                    let n = OC918_NODES;
                    oc918_node(c, n, h0);
                    nodes.push(n);
                }
                for i in 0..7u32 {
                    if mask >> i & 1 == 0 || seed % 7 == 0 {
                        let n = OC918_NODES + 0x100 * (i + 1);
                        oc918_node(c, n, digit[i as usize]);
                        nodes.push(n);
                    }
                }
                oc918_screen(c, OC918_SCREEN, &nodes);
                oc918_w(c, OC918_SELF + 0x40, OC918_SCREEN);
                c.g[A0][0] = OC918_SELF as u64;
                c.g[A1][0] = (seed % 34) as u64; // character index (31 and up: early exit)
                c.g[A2][0] = ((seed / 3) % 2) as u64; // show
                c.g[A3][0] = ((seed / 2) % 2) as u64; // opponent side
            },
        );
    }

    // -- 0x001b2070 cFEStateChatRoom_Init -------------------------------

    #[test]
    fn c_fe_state_chat_room_init_matches() {
        oc918_install_fakes();
        check(
            ee::s03::f_001b2070__generated,
            c_fe_state_chat_room_init,
            24,
            &|c, seed| {
                let h0 = oc918_hash(c, DATA + 0x100);
                let mut r = oc918_base(c, seed, 0x1b20_70, 8);
                // CreateScreen returns null (early exit) or the screen.
                oc918_w(
                    c,
                    OC918_CTRL + 4,
                    if seed % 4 == 0 { 0 } else { OC918_SCREEN },
                );
                oc918_node(c, OC918_NODES, if seed % 4 == 2 { h0 ^ 1 } else { h0 });
                oc918_screen(c, OC918_SCREEN, &[OC918_NODES]);
                oc918_w(c, GAME_GP.wrapping_sub(0xc8), OC918_LOBBY);
                fill(c, &mut r, OC918_LOBBY, 0x40);
                let this = OC918_SELF;
                fill(c, &mut r, this, 0x1c0);
                oc918_w(c, this + 0x10, OC918_ARG);
                oc918_w(c, this + 0x6ec, DATA + 0x6000);
                oc918_w(c, this + 0x6f0, DATA + 0x6200);
                oc918_w(c, this + 0x6f4, DATA + 0x6400);
                c.g[A0][0] = this as u64;
            },
        );
    }
}
