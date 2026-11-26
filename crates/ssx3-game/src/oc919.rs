//! Staging module oc919 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc919`.

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
        0x001b_2378,
        cfe_state_chat_room_refresh_player_list,
        "cFEStateChatRoom_RefreshPlayerList",
    ),
    (
        0x001b_2eb8,
        cfe_state_chat_room_on_widget_created,
        "cFEStateChatRoom_OnWidgetCreated",
    ),
    (
        0x001b_4888,
        cfe_state_net_config_show_card_error_popup,
        "cFEStateNetConfig_ShowCardErrorPopup",
    ),
    (
        0x001b_5830,
        cfe_state_net_config_on_auth_popup_event,
        "cFEStateNetConfig_OnAuthPopupEvent",
    ),
    (
        0x001b_5e28,
        cfe_state_net_config_v31,
        "cFEStateNetConfig_v31",
    ),
];

/// `void cFEStateChatRoom_RefreshPlayerList(self, int reselect)` (0x1b2378):
/// rebuild the six visible rows of the chat room's player list. Asks the lobby
/// manager for the player count, scrolls/resizes the `PlayerListMenu`, fills
/// each row's text (a player whose entry carries the "room changed" flag also
/// renames the room and opens the lobby menu state), then re-selects the
/// player by name when `reselect` is set and refreshes the quality icons.
/// Fields of `self`: `+0x10` engine, `+0x40` screen, `+0x6e8` room-name text,
/// `+0x704` scroll offset, `+0x70c` selected player name.
pub fn cfe_state_chat_room_refresh_player_list(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b25b0: {
            c.cycles += 17;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x200));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x1e0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x1d0));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x1c0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1b0));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1a0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x190));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x180));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x170));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x160));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x150), c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1f0));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x144), c.g[A1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            if c.g[V0][0] == 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                break 'b_1b25b0;
            } else {
                'b_1b240c: {
                    c.cycles += 17;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    call_ptr(c, 0x0025_f1b0, 0x001b_23c4)?;
                    c.g[A1][0] = u64::MAX;
                    c.g[A2][0] = 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S6][0] = c.g[V0][0];
                    call_ptr(c, 0x001b_3d20, 0x001b_23d8)?;
                    c.g[A0][0] = 0x0046_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3a38));
                    call_ptr(c, 0x0031_7670, 0x001b_23e4)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001b_23f0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x96));
                    let t = c.g[V0][0] == c.g[S6][0];
                    c.g[V0][0] = ((c.g[S6][0] as i64) < 7i64) as u64;
                    if t {
                        break 'b_1b240c;
                    } else {
                        c.cycles += 3;
                        c.g[A1][0] = c.g[S6][0] & 0xff;
                        call_ptr(c, 0x0039_b760, 0x001b_2408)?;
                        c.g[V0][0] = ((c.g[S6][0] as i64) < 7i64) as u64;
                        break 'b_1b240c;
                    }
                }
                'b_1b2414: {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x704), 0u64);
                        break 'b_1b2414;
                    } else {
                        break 'b_1b2414;
                    }
                }
                c.cycles += 21;
                c.g[V0][0] = 0x0046_0000;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
                c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3a48));
                c.ldl(4, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.ldr(4, (c.g[A3][0] as u32));
                c.ldl(5, (c.g[A3][0] as u32).wrapping_add(0xf));
                c.ldr(5, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[A2][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x10));
                c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                c.sdr(4, (c.g[SP][0] as u32));
                c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0xf));
                c.sdr(5, (c.g[SP][0] as u32).wrapping_add(0x8));
                c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                c.g[S4][0] = 0x5;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x140), 0u64);
                c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 20);
                c.g[FP][0] = 0x004a_0000;
                'l_1b2468: loop {
                    'b_1b2570: {
                        'b_1b2560: {
                            c.cycles += 26;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x704));
                            c.g[V1][0] = 0x0046_0000;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3a60));
                            c.g[A2][0] = c.g[S4][0];
                            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                            call_ptr(c, 0x0041_7828, 0x001b_2484)?;
                            c.g[A0][0] = c.g[SP][0];
                            call_ptr(c, 0x0031_7670, 0x001b_248c)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039_d860, 0x001b_2498)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[S7][0];
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x58));
                            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x38), 0u64);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x3c), 0u64);
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001b_24c4)?;
                            c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[S6][0] as i64)) as u64;
                            let t = c.g[V1][0] == 0u64;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                            if t {
                                break 'b_1b2560;
                            } else {
                                c.cycles += 9;
                                c.g[A1][0] = c.g[S3][0];
                                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                                call_ptr(c, 0x0025_f1e0, 0x001b_24dc)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A0][0] = c.g[S5][0];
                                call_ptr(c, 0x0031_84e8, 0x001b_24e8)?;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x140));
                                let t = c.g[V1][0] == 0u64;
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                if t {
                                    break 'b_1b2570;
                                } else {
                                    'b_1b2550: {
                                        c.cycles += 10;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                        call_ptr(c, 0x0025_da90, 0x001b_24fc)?;
                                        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.g[S1][0];
                                        call_ptr(c, 0x001b_25e0, 0x001b_250c)?;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                        call_ptr(c, 0x0025_da90, 0x001b_2514)?;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x6e8));
                                            break 'b_1b2550;
                                        } else {
                                            c.cycles += 13;
                                            c.g[A2][0] = 0x0046_0000;
                                            c.g[A0][0] = 0xb84;
                                            c.g[A1][0] =
                                                sx((c.g[A2][0] as u32).wrapping_add(0x1b30));
                                            c.g[A3][0] = 0u64;
                                            c.g[A2][0] = 0u64;
                                            call_ptr(c, 0x0031_7d70, 0x001b_2534)?;
                                            c.g[A1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x001b_cb58, 0x001b_2540)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x001a_8a40, 0x001b_254c)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x6e8));
                                            break 'b_1b2550;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A1][0] = c.g[S1][0];
                                    call_ptr(c, 0x003a_0e90, 0x001b_2558)?;
                                    let t = true;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                                    if t {
                                        break 'b_1b2570;
                                    } else {
                                        break 'b_1b2560;
                                    }
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S5][0];
                        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1390));
                        call_ptr(c, 0x0031_84e8, 0x001b_256c)?;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_1b2570;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                    call_ptr(c, 0x003a_0c50, 0x001b_257c)?;
                    let t = (c.g[S4][0] as i64) >= 0;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x18), c.g[S3][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001b_2468);
                        }
                        continue 'l_1b2468;
                    } else {
                        'b_1b2598: {
                            c.cycles += 3;
                            c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x144));
                            let t = c.g[A3][0] == 0u64;
                            c.g[A0][0] = c.g[S0][0];
                            if t {
                                break 'b_1b2598;
                            } else {
                                c.cycles += 2;
                                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x70c));
                                call_ptr(c, 0x001b_3978, 0x001b_2598)?;
                                break 'b_1b2598;
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x001b_3bf0, 0x001b_25a0)?;
                        c.g[A0][0] = c.g[S5][0];
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x0031_8260, 0x001b_25ac)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                        break 'b_1b25b0;
                    }
                }
            }
        }
        c.cycles += 12;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x190));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x180));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x150));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateChatRoom_OnWidgetCreated(self, widget)` (0x1b2eb8): the chat
/// room screen's widget-created hook. Compares the widget's name hash (`+0x38`)
/// with the known widget names and keeps the widget in its slot of `self`
/// (`+0x6d0` quality icons, `+0x6e8` room name, `+0x6ec`/`+0x6f0` menus,
/// `+0x6f4`..`+0x700` sliders and text scroll), setting visibility, focus, flags
/// and button captions as the screen needs; a widget named `quality<k>` goes to
/// icon `k-1`.
pub fn cfe_state_chat_room_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b313c: {
            'b_1b3138: {
                'b_1b2f50: {
                    'b_1b2f38: {
                        'b_1b2f08: {
                            c.cycles += 16;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                            c.g[S0][0] = c.g[A0][0];
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                            c.g[S1][0] = c.g[A1][0];
                            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                            c.g[A0][0] = 0x0046_0000;
                            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3c08));
                            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                            c.g[S3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                            call_ptr(c, 0x0031_7670, 0x001b_2ef0)?;
                            let t = c.g[S3][0] != c.g[V0][0];
                            c.g[A0][0] = 0x0046_0000;
                            if t {
                                break 'b_1b2f08;
                            } else {
                                c.cycles += 4;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x6e8), c.g[S1][0]);
                                c.g[A1][0] = 0x1;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                if t {
                                    break 'b_1b2f38;
                                } else {
                                    break 'b_1b2f08;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3c18));
                        call_ptr(c, 0x0031_7670, 0x001b_2f10)?;
                        let t = c.g[S3][0] != c.g[V0][0];
                        c.g[A0][0] = 0x0046_0000;
                        if t {
                            break 'b_1b2f50;
                        } else {
                            c.cycles += 8;
                            c.g[A0][0] = 0x0046_0000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3c30));
                            call_ptr(c, 0x0031_7670, 0x001b_2f24)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x003a_1000, 0x001b_2f30)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            break 'b_1b2f38;
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001b_2f48)?;
                    let t = true;
                    c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6d0));
                    if t {
                        break 'b_1b3138;
                    } else {
                        break 'b_1b2f50;
                    }
                }
                'b_1b2f90: {
                    c.cycles += 4;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3108));
                    call_ptr(c, 0x0031_7670, 0x001b_2f58)?;
                    let t = c.g[S3][0] != c.g[V0][0];
                    c.g[A0][0] = 0x0046_0000;
                    if t {
                        break 'b_1b2f90;
                    } else {
                        c.cycles += 12;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001b_2f78)?;
                        c.g[A1][0] = 0x004a_0000;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1408));
                        call_ptr(c, 0x003a_0c50, 0x001b_2f88)?;
                        let t = true;
                        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6d0));
                        if t {
                            break 'b_1b3138;
                        } else {
                            break 'b_1b2f90;
                        }
                    }
                }
                'b_1b3134: {
                    'b_1b30e4: {
                        'b_1b30c8: {
                            'b_1b2fc4: {
                                c.cycles += 4;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3c48));
                                call_ptr(c, 0x0031_7670, 0x001b_2f98)?;
                                if c.g[S3][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x0046_0000;
                                    break 'b_1b2fc4;
                                } else {
                                    c.cycles += 9;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0x1;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001b_2fb8)?;
                                    c.g[A0][0] = 0x0046_0000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3c68));
                                    if t {
                                        break 'b_1b30c8;
                                    } else {
                                        break 'b_1b2fc4;
                                    }
                                }
                            }
                            'b_1b302c: {
                                c.cycles += 4;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3a38));
                                call_ptr(c, 0x0031_7670, 0x001b_2fcc)?;
                                if c.g[S3][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x0046_0000;
                                    break 'b_1b302c;
                                } else {
                                    'b_1b301c: {
                                        c.cycles += 13;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                                        c.g[A1][0] = 0x1;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                        c.g[V0][0] = c.g[V0][0] | 0x80;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        call_ptr(c, t, 0x001b_2ff8)?;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                        call_ptr(c, 0x0025_f1b0, 0x001b_3000)?;
                                        let t = (c.g[V0][0] as i64) > 0;
                                        c.g[A1][0] = c.g[V0][0] & 0xff;
                                        if t {
                                            break 'b_1b301c;
                                        } else {
                                            c.cycles += 5;
                                            c.g[A0][0] = c.g[S1][0];
                                            c.g[A1][0] = 0x6;
                                            call_ptr(c, 0x0039_b760, 0x001b_3014)?;
                                            let t = true;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x6fc),
                                                c.g[S1][0],
                                            );
                                            if t {
                                                break 'b_1b3134;
                                            } else {
                                                break 'b_1b301c;
                                            }
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S1][0];
                                    call_ptr(c, 0x0039_b760, 0x001b_3024)?;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x6fc), c.g[S1][0]);
                                    if t {
                                        break 'b_1b3134;
                                    } else {
                                        break 'b_1b302c;
                                    }
                                }
                            }
                            'b_1b305c: {
                                c.cycles += 4;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3c80));
                                call_ptr(c, 0x0031_7670, 0x001b_3034)?;
                                let t = c.g[S3][0] != c.g[V0][0];
                                c.g[A0][0] = 0x0046_0000;
                                if t {
                                    break 'b_1b305c;
                                } else {
                                    c.cycles += 8;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0x1;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x38));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x3c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001b_3054)?;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x700), c.g[S1][0]);
                                    if t {
                                        break 'b_1b3134;
                                    } else {
                                        break 'b_1b305c;
                                    }
                                }
                            }
                            'b_1b3078: {
                                c.cycles += 4;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ad8));
                                call_ptr(c, 0x0031_7670, 0x001b_3064)?;
                                if c.g[S3][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x0046_0000;
                                    break 'b_1b3078;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x0046_0000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3c90));
                                    if t {
                                        break 'b_1b30c8;
                                    } else {
                                        break 'b_1b3078;
                                    }
                                }
                            }
                            'b_1b3094: {
                                c.cycles += 4;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ca0));
                                call_ptr(c, 0x0031_7670, 0x001b_3080)?;
                                if c.g[S3][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x0046_0000;
                                    break 'b_1b3094;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x0046_0000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3cb0));
                                    if t {
                                        break 'b_1b30c8;
                                    } else {
                                        break 'b_1b3094;
                                    }
                                }
                            }
                            'b_1b30b0: {
                                c.cycles += 4;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3cc0));
                                call_ptr(c, 0x0031_7670, 0x001b_309c)?;
                                if c.g[S3][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x0046_0000;
                                    break 'b_1b30b0;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x0046_0000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3cd0));
                                    if t {
                                        break 'b_1b30c8;
                                    } else {
                                        break 'b_1b30b0;
                                    }
                                }
                            }
                            c.cycles += 4;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3ce0));
                            call_ptr(c, 0x0031_7670, 0x001b_30b8)?;
                            let t = c.g[S3][0] != c.g[V0][0];
                            c.g[A0][0] = 0x004a_0000;
                            if t {
                                break 'b_1b30e4;
                            } else {
                                c.cycles += 2;
                                c.g[A0][0] = 0x0046_0000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3cf0));
                                break 'b_1b30c8;
                            }
                        }
                        c.cycles += 7;
                        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6d0));
                        call_ptr(c, 0x0031_7670, 0x001b_30d0)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x003a_1000, 0x001b_30dc)?;
                        let t = true;
                        c.g[S2][0] = 0u64;
                        if t {
                            break 'b_1b313c;
                        } else {
                            break 'b_1b30e4;
                        }
                    }
                    'b_1b3128: {
                        'b_1b30fc: {
                            c.cycles += 4;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b68));
                            call_ptr(c, 0x0031_7670, 0x001b_30ec)?;
                            let t = c.g[S3][0] != c.g[V0][0];
                            c.g[A0][0] = 0x0046_0000;
                            if t {
                                break 'b_1b30fc;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x6ec), c.g[S1][0]);
                                if t {
                                    break 'b_1b3128;
                                } else {
                                    break 'b_1b30fc;
                                }
                            }
                        }
                        'b_1b3114: {
                            c.cycles += 4;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3d00));
                            call_ptr(c, 0x0031_7670, 0x001b_3104)?;
                            let t = c.g[S3][0] != c.g[V0][0];
                            c.g[A0][0] = 0x004a_0000;
                            if t {
                                break 'b_1b3114;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x6f0), c.g[S1][0]);
                                if t {
                                    break 'b_1b3134;
                                } else {
                                    break 'b_1b3114;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1b70));
                        call_ptr(c, 0x0031_7670, 0x001b_311c)?;
                        let t = c.g[S3][0] != c.g[V0][0];
                        c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6d0));
                        if t {
                            break 'b_1b3138;
                        } else {
                            c.cycles += 1;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x6f4), c.g[S1][0]);
                            break 'b_1b3128;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = c.g[V0][0] | 0x1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                    break 'b_1b3134;
                }
                c.cycles += 1;
                c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6d0));
                break 'b_1b3138;
            }
            c.cycles += 1;
            c.g[S2][0] = 0u64;
            break 'b_1b313c;
        }
        c.cycles += 1;
        c.g[S5][0] = 0x0046_0000;
        'l_1b3140: loop {
            'b_1b3178: {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 6i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                if t {
                    break 'b_1b3178;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x3d10));
                    call_ptr(c, 0x0041_7828, 0x001b_315c)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x0031_7670, 0x001b_3164)?;
                    if c.g[S3][0] != c.g[V0][0] {
                        c.g[S2][0] = c.g[S0][0];
                        if event_due(c) {
                            deliver_event_here(c, 0x001b_3140);
                        }
                        continue 'l_1b3140;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                        c.sw((c.g[V0][0] as u32), c.g[S1][0]);
                        break 'b_1b3178;
                    }
                }
            }
            c.cycles += 9;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
            c.pc = t;
            break 'l_1b3140;
        }
        Ok(())
    })
}

/// `void cFEStateNetConfig_ShowCardErrorPopup(self, popupId, cardError, cardSlot)`
/// (0x1b4888): fill the network popup (`+0xbc` info block of the popup from
/// `cFEStateNet_GetPopup`) with the message for a memory card error (2 not
/// detected, 0x14 not formatted, 0x18 corrupt, 6 different card), give it
/// Retry/Continue (or Cancel) buttons and show it.
pub fn cfe_state_net_config_show_card_error_popup(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b4b58: {
            'b_1b4b54: {
                'b_1b4b4c: {
                    'b_1b4b34: {
                        c.cycles += 21;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
                        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
                        c.g[S7][0] = c.g[A0][0];
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.g[S0][0] = c.g[A2][0];
                        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[S5][0] = c.g[A3][0];
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.g[FP][0] = c.g[A1][0];
                        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), 0u64);
                        call_ptr(c, 0x001a_9c50, 0x001b_48cc)?;
                        c.g[S4][0] = c.g[V0][0];
                        c.g[V0][0] = 0x6;
                        let t = c.g[S0][0] == c.g[V0][0];
                        c.g[S6][0] = sx((c.g[S4][0] as u32).wrapping_add(0xbc));
                        if t {
                            break 'b_1b4b34;
                        } else {
                            'b_1b4b1c: {
                                'b_1b4a90: {
                                    'b_1b4a28: {
                                        'b_1b4914: {
                                            'b_1b48f8: {
                                                c.cycles += 3;
                                                c.g[V0][0] = ((c.g[S0][0] as i64) < 7i64) as u64;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_1b48f8;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[S0][0] == c.g[V0][0];
                                                    c.g[A0][0] = 0x0046_0000;
                                                    if t {
                                                        break 'b_1b4914;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x2328));
                                                        if t {
                                                            break 'b_1b4b58;
                                                        } else {
                                                            break 'b_1b48f8;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x14;
                                            let t = c.g[S0][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x18;
                                            if t {
                                                break 'b_1b4a28;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[S0][0] == c.g[V0][0];
                                                c.g[V0][0] = 0x2;
                                                if t {
                                                    break 'b_1b4a90;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.g[A0][0] = 0x0046_0000;
                                                    if t {
                                                        break 'b_1b4b54;
                                                    } else {
                                                        break 'b_1b4914;
                                                    }
                                                }
                                            }
                                        }
                                        'b_1b49a8: {
                                            c.cycles += 3;
                                            c.g[V0][0] = u64::MAX;
                                            let t = c.g[S5][0] != c.g[V0][0];
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            if t {
                                                break 'b_1b49a8;
                                            } else {
                                                c.cycles += 34;
                                                c.g[A0][0] = 0x0046_0000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x3f08));
                                                c.g[S3][0] =
                                                    sx((c.g[S7][0] as u32).wrapping_add(0x71c));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                c.g[S5][0] = 0x1;
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                call_ptr(c, 0x0031_7670, 0x001b_4944)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A1][0] = c.g[V0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[A0][0] = c.g[S0][0];
                                                call_ptr(c, t, 0x001b_4954)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                c.g[S2][0] = c.g[V0][0];
                                                c.g[A0][0] = 0x0046_0000;
                                                c.g[S0][0] =
                                                    c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x3f20));
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add((c.g[V0][0] as u32)));
                                                call_ptr(c, 0x0031_7670, 0x001b_4978)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, t, 0x001b_4988)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[A3][0] = c.g[S2][0];
                                                c.g[A0][0] = c.g[S3][0];
                                                c.g[A2][0] = 0x1;
                                                call_ptr(c, 0x002c_26d0, 0x001b_499c)?;
                                                c.sw(
                                                    (c.g[S4][0] as u32).wrapping_add(0xc4),
                                                    c.g[S3][0],
                                                );
                                                let t = true;
                                                c.sw(
                                                    (c.g[S4][0] as u32).wrapping_add(0xc0),
                                                    c.g[S5][0],
                                                );
                                                if t {
                                                    break 'b_1b4b4c;
                                                } else {
                                                    break 'b_1b49a8;
                                                }
                                            }
                                        }
                                        c.cycles += 32;
                                        c.g[A0][0] = 0x0046_0000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f08));
                                        c.g[S3][0] = sx((c.g[S7][0] as u32).wrapping_add(0x71c));
                                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                        c.g[V0][0] = 0x1;
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        call_ptr(c, 0x0031_7670, 0x001b_49d0)?;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                        c.g[A1][0] = c.g[V0][0];
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, t, 0x001b_49e0)?;
                                        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[S2][0] = c.g[V0][0];
                                        c.g[A0][0] = 0x0046_0000;
                                        c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f20));
                                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                        call_ptr(c, 0x0031_7670, 0x001b_4a04)?;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                        c.g[A0][0] = c.g[S0][0];
                                        let t = c.g[V1][0] as u32;
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, t, 0x001b_4a14)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                        c.g[A3][0] = c.g[S2][0];
                                        let t = true;
                                        c.g[A0][0] = c.g[S3][0];
                                        if t {
                                            break 'b_1b4b1c;
                                        } else {
                                            break 'b_1b4a28;
                                        }
                                    }
                                    c.cycles += 26;
                                    c.g[A1][0] = 0x004a_0000;
                                    c.g[A0][0] = c.g[SP][0];
                                    c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
                                    call_ptr(c, 0x002c_2718, 0x001b_4a3c)?;
                                    c.g[S2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x71c));
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[A0][0] = 0x0046_0000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2370));
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                    call_ptr(c, 0x0031_7670, 0x001b_4a60)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x001b_4a70)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A2][0] = c.g[SP][0];
                                    call_ptr(c, 0x002c_26d0, 0x001b_4a80)?;
                                    c.g[V0][0] = 0x1;
                                    c.sw((c.g[S4][0] as u32).wrapping_add(0xc4), c.g[S2][0]);
                                    let t = true;
                                    c.sw((c.g[S4][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
                                    if t {
                                        break 'b_1b4b4c;
                                    } else {
                                        break 'b_1b4a90;
                                    }
                                }
                                c.cycles += 35;
                                c.g[A1][0] = 0x004a_0000;
                                c.g[A0][0] = c.g[SP][0];
                                c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x12c0));
                                call_ptr(c, 0x002c_2718, 0x001b_4aa4)?;
                                c.g[S3][0] = sx((c.g[S7][0] as u32).wrapping_add(0x71c));
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = 0x0046_0000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f08));
                                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                call_ptr(c, 0x0031_7670, 0x001b_4ac8)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                c.g[A1][0] = c.g[V0][0];
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, t, 0x001b_4ad8)?;
                                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[S2][0] = c.g[V0][0];
                                c.g[A0][0] = 0x0046_0000;
                                c.g[S0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8c));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f70));
                                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                c.g[S0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                call_ptr(c, 0x0031_7670, 0x001b_4afc)?;
                                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                c.g[A0][0] = c.g[S0][0];
                                let t = c.g[V1][0] as u32;
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, t, 0x001b_4b0c)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A2][0] = c.g[S2][0];
                                c.g[A0][0] = c.g[S3][0];
                                c.g[A3][0] = c.g[SP][0];
                                break 'b_1b4b1c;
                            }
                            c.cycles += 6;
                            call_ptr(c, 0x002c_26d0, 0x001b_4b24)?;
                            c.g[V0][0] = 0x1;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0xc4), c.g[S3][0]);
                            let t = true;
                            c.sw((c.g[S4][0] as u32).wrapping_add(0xc0), c.g[V0][0]);
                            if t {
                                break 'b_1b4b4c;
                            } else {
                                break 'b_1b4b34;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = 0x0046_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2688));
                    call_ptr(c, 0x0031_7670, 0x001b_4b40)?;
                    c.g[V1][0] = 0x3;
                    c.sw((c.g[S4][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                    break 'b_1b4b4c;
                }
                c.cycles += 2;
                c.g[V0][0] = 0x2;
                c.g[A0][0] = 0x0046_0000;
                break 'b_1b4b54;
            }
            c.cycles += 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2328));
            break 'b_1b4b58;
        }
        'b_1b4bbc: {
            'b_1b4ba0: {
                c.cycles += 9;
                c.sw((c.g[S6][0] as u32), c.g[V0][0]);
                call_ptr(c, 0x0031_7670, 0x001b_4b60)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S6][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x001c_57c0, 0x001b_4b70)?;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = 0x0046_0000;
                if t {
                    break 'b_1b4ba0;
                } else {
                    c.cycles += 9;
                    c.g[A0][0] = 0x0046_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x820));
                    call_ptr(c, 0x0031_7670, 0x001b_4b88)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x001c_57c0, 0x001b_4b98)?;
                    let t = true;
                    c.sw((c.g[S6][0] as u32).wrapping_add(0x94), c.g[FP][0]);
                    if t {
                        break 'b_1b4bbc;
                    } else {
                        break 'b_1b4ba0;
                    }
                }
            }
            c.cycles += 7;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c88));
            call_ptr(c, 0x0031_7670, 0x001b_4ba8)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S6][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x001c_57c0, 0x001b_4bb8)?;
            c.sw((c.g[S6][0] as u32).wrapping_add(0x94), c.g[FP][0]);
            break 'b_1b4bbc;
        }
        c.cycles += 16;
        c.g[A0][0] = c.g[S7][0];
        c.sw((c.g[S6][0] as u32).wrapping_add(0xa8), 0u64);
        c.g[A1][0] = c.g[S4][0];
        call_ptr(c, 0x001a_9bc0, 0x001b_4bcc)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        Ok(())
    })
}

/// `void cFEStateNetConfig_OnAuthPopupEvent(self, child, event)` (0x1b5830):
/// reacts to the popup events 0x0f/0x10 (flag `+0x700`), 0x14 (stop the network
/// session, mark `+0x6d8`) and 0x16 (push the DNAS-error or account-memory
/// popup state); every kind forwards to `cFEStateNet_OnChildEvent`.
pub fn cfe_state_net_config_on_auth_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b5a44: {
            'b_1b5a40: {
                'b_1b5a38: {
                    'b_1b595c: {
                        'b_1b58bc: {
                            'b_1b589c: {
                                c.cycles += 8;
                                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                                c.g[V0][0] = 0x10;
                                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                                c.g[S0][0] = c.g[A0][0];
                                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                                let t = c.g[A2][0] == c.g[V0][0];
                                c.g[S1][0] = c.g[A1][0];
                                if t {
                                    break 'b_1b589c;
                                } else {
                                    'b_1b5888: {
                                        'b_1b586c: {
                                            c.cycles += 3;
                                            c.g[V0][0] = (c.g[A2][0] < 0x11u64) as u64;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0xf;
                                            if t {
                                                break 'b_1b586c;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.g[A0][0] = c.g[S0][0];
                                                if t {
                                                    break 'b_1b5888;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1b5a38;
                                                    } else {
                                                        break 'b_1b586c;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x14;
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.g[V0][0] = 0x16;
                                        if t {
                                            break 'b_1b58bc;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[A2][0] == c.g[V0][0];
                                            c.g[A0][0] = c.g[S0][0];
                                            if t {
                                                break 'b_1b595c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_1b5a38;
                                                } else {
                                                    break 'b_1b5888;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 5;
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A2][0] = 0xf;
                                    call_ptr(c, 0x001a_97b8, 0x001b_5894)?;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x700), 0u64);
                                    if t {
                                        break 'b_1b5a40;
                                    } else {
                                        break 'b_1b589c;
                                    }
                                }
                            }
                            c.cycles += 8;
                            c.g[V0][0] = 0x1;
                            c.g[A0][0] = c.g[S0][0];
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x700), c.g[V0][0]);
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = 0x10;
                            call_ptr(c, 0x001a_97b8, 0x001b_58b4)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_1b5a44;
                            } else {
                                break 'b_1b58bc;
                            }
                        }
                        'b_1b58d0: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            let t = c.g[V0][0] != 0u64;

                            if t {
                                break 'b_1b58d0;
                            } else {
                                c.cycles += 2;
                                call_ptr(c, 0x0026_0f80, 0x001b_58d0)?;
                                break 'b_1b58d0;
                            }
                        }
                        'b_1b58ec: {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            call_ptr(c, 0x0026_1460, 0x001b_58d8)?;
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                            if t {
                                break 'b_1b58ec;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                call_ptr(c, 0x0026_13c8, 0x001b_58e8)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                break 'b_1b58ec;
                            }
                        }
                        'b_1b58fc: {
                            c.cycles += 2;
                            let t = c.g[V0][0] != 0u64;

                            if t {
                                break 'b_1b58fc;
                            } else {
                                c.cycles += 2;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                call_ptr(c, 0x0025_b800, 0x001b_58fc)?;
                                break 'b_1b58fc;
                            }
                        }
                        c.cycles += 24;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                        call_ptr(c, 0x0026_1408, 0x001b_5904)?;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                        call_ptr(c, 0x0025_b800, 0x001b_590c)?;
                        call_ptr(c, 0x0025_91b8, 0x001b_5914)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0026_36f0, 0x001b_5920)?;
                        c.g[V0][0] = 0x1;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6ec));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x6d8), c.g[V0][0]);
                        c.g[A1][0] = 0u64;
                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001b_5944)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = 0x14;
                        call_ptr(c, 0x001a_97b8, 0x001b_5954)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_1b5a44;
                        } else {
                            break 'b_1b595c;
                        }
                    }
                    'b_1b5a30: {
                        'b_1b59cc: {
                            c.cycles += 6;
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = 0x16;
                            call_ptr(c, 0x001a_97b8, 0x001b_5968)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x700));
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0x0046_0000;
                            if t {
                                break 'b_1b59cc;
                            } else {
                                c.cycles += 22;
                                c.g[A0][0] = 0x6e8;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4098));
                                c.g[A2][0] = 0u64;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x0031_7d70, 0x001b_5988)?;
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x714));
                                c.g[A2][0] = c.g[S0][0];
                                c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x718));
                                c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x710));
                                call_ptr(c, 0x001b_6ab8, 0x001b_59a4)?;
                                c.g[V1][0] = 0x81b;
                                c.g[A1][0] = c.g[V0][0];
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039_f290, 0x001b_59c4)?;
                                let t = true;
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x6d8), 0u64);
                                if t {
                                    break 'b_1b5a30;
                                } else {
                                    break 'b_1b59cc;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6d8));
                        if c.g[V0][0] != 0u64 {
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x6d8), 0u64);
                            break 'b_1b5a30;
                        } else {
                            c.cycles += 22;
                            c.g[A1][0] = 0x0046_0000;
                            c.g[A0][0] = 0xb10;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d10));
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0u64;
                            call_ptr(c, 0x0031_7d70, 0x001b_59f0)?;
                            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A2][0] = c.g[S0][0];
                            c.g[A3][0] = 0u64;
                            c.g[T0][0] = 0u64;
                            c.g[T1][0] = 0u64;
                            call_ptr(c, 0x001b_6ed0, 0x001b_5a0c)?;
                            c.g[V1][0] = 0x82c;
                            c.g[A1][0] = c.g[V0][0];
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039_f290, 0x001b_5a2c)?;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x6d8), 0u64);
                            break 'b_1b5a30;
                        }
                    }
                    c.cycles += 2;
                    let t = true;
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x700), 0u64);
                    if t {
                        break 'b_1b5a40;
                    } else {
                        break 'b_1b5a38;
                    }
                }
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x001a_97b8, 0x001b_5a40)?;
                break 'b_1b5a40;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1b5a44;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

/// `int cFEStateNetConfig_v31(self, id, message)` (0x1b5e28): the state's
/// message handler. 0x105 pops the state when the attribute bits allow it;
/// 277..278 (disconnect errors) store the error text in `+0x704` and show a
/// one-button popup; everything else goes to the base handler
/// (`cFEStateNet_OnMessage`).
pub fn cfe_state_net_config_v31(c: &mut Cpu) {
    guest(c, |c| {
        'b_1b6060: {
            'b_1b605c: {
                'b_1b6058: {
                    'b_1b5e74: {
                        c.cycles += 10;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                        c.g[V0][0] = 0x105;
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[S2][0] = c.g[A2][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.g[S3][0] = c.g[A0][0];
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                        if t {
                            break 'b_1b5e74;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                            c.g[V0][0] = c.g[V0][0] & 0x6;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_1b6058;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039_f190, 0x001b_5e6c)?;
                                let t = true;
                                c.g[V0][0] = 0u64;
                                if t {
                                    break 'b_1b605c;
                                } else {
                                    break 'b_1b5e74;
                                }
                            }
                        }
                    }
                    'b_1b6048: {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 261i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = c.g[S3][0];
                        if t {
                            break 'b_1b6048;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < 279i64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < 277i64) as u64;
                            if t {
                                break 'b_1b6048;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;

                                if t {
                                    break 'b_1b6048;
                                } else {
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[V0][0] = 0x2d64_0000;
                                    if t {
                                        break 'b_1b6058;
                                    } else {
                                        'b_1b5f28: {
                                            'b_1b5f24: {
                                                'b_1b5f20: {
                                                    'b_1b5f10: {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c.g[V0][0] | 0x6576;
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[V0][0] = ((c.g[V0][0] as i64)
                                                            < (c.g[V1][0] as i64))
                                                            as u64;
                                                        if t {
                                                            break 'b_1b5f10;
                                                        } else {
                                                            'b_1b5ed8: {
                                                                c.cycles += 2;
                                                                let t = c.g[V0][0] != 0u64;
                                                                c.g[V0][0] = 0x2d69_0000;
                                                                if t {
                                                                    break 'b_1b5ed8;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[V0][0] = 0x2d61_0000;
                                                                    c.g[V0][0] =
                                                                        c.g[V0][0] | 0x7468;
                                                                    let t =
                                                                        c.g[V1][0] == c.g[V0][0];
                                                                    c.g[V0][0] = 0x2d63_0000;
                                                                    if t {
                                                                        break 'b_1b5f10;
                                                                    } else {
                                                                        c.cycles += 3;
                                                                        c.g[V0][0] =
                                                                            c.g[V0][0] | 0x6667;
                                                                        let t = c.g[V1][0]
                                                                            == c.g[V0][0];
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        if t {
                                                                            break 'b_1b5f10;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = true;
                                                                            c.g[A0][0] =
                                                                                0x0046_0000;
                                                                            if t {
                                                                                break 'b_1b5f24;
                                                                            } else {
                                                                                break 'b_1b5ed8;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.g[V0][0] | 0x6e69;
                                                            let t = c.g[V1][0] == c.g[V0][0];
                                                            c.g[V0][0] = ((c.g[V0][0] as i64)
                                                                < (c.g[V1][0] as i64))
                                                                as u64;
                                                            if t {
                                                                break 'b_1b5f10;
                                                            } else {
                                                                'b_1b5f04: {
                                                                    c.cycles += 2;
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.g[V0][0] = 0x2d74_0000;
                                                                    if t {
                                                                        break 'b_1b5f04;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[V0][0] = 0x2d64_0000;
                                                                        c.g[V0][0] =
                                                                            c.g[V0][0] | 0x6966;
                                                                        let t = c.g[V1][0]
                                                                            == c.g[V0][0];
                                                                        c.g[V0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x848));
                                                                        if t {
                                                                            break 'b_1b5f10;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = true;
                                                                            c.g[A0][0] =
                                                                                0x0046_0000;
                                                                            if t {
                                                                                break 'b_1b5f24;
                                                                            } else {
                                                                                break 'b_1b5f04;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 3;
                                                                c.g[V0][0] = c.g[V0][0] | 0x6f6e;
                                                                let t = c.g[V1][0] != c.g[V0][0];
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                if t {
                                                                    break 'b_1b5f20;
                                                                } else {
                                                                    break 'b_1b5f10;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[A0][0] = 0x0046_0000;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x41b0));
                                                    if t {
                                                        break 'b_1b5f28;
                                                    } else {
                                                        break 'b_1b5f20;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[A0][0] = 0x0046_0000;
                                                break 'b_1b5f24;
                                            }
                                            c.cycles += 1;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x41d0));
                                            break 'b_1b5f28;
                                        }
                                        'b_1b5f64: {
                                            c.cycles += 12;
                                            c.g[S0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            call_ptr(c, 0x0031_7670, 0x001b_5f3c)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A0][0] = c.g[S0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, t, 0x001b_5f4c)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x704));
                                            let t = c.g[A0][0] == 0u64;
                                            c.g[S1][0] = c.g[V0][0];
                                            if t {
                                                break 'b_1b5f64;
                                            } else {
                                                c.cycles += 3;
                                                call_ptr(c, 0x0031_7e98, 0x001b_5f60)?;
                                                c.sw((c.g[S3][0] as u32).wrapping_add(0x704), 0u64);
                                                break 'b_1b5f64;
                                            }
                                        }
                                        'b_1b5ff0: {
                                            'b_1b5fb8: {
                                                c.cycles += 6;
                                                c.g[A1][0] = c.g[S2][0];
                                                c.g[A0][0] = c.g[S3][0];
                                                call_ptr(c, 0x001a_af90, 0x001b_5f70)?;
                                                c.g[S0][0] = c.g[V0][0];
                                                let t = c.g[S0][0] == 0u64;

                                                if t {
                                                    break 'b_1b5fb8;
                                                } else {
                                                    c.cycles += 15;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x003d_bca0, 0x001b_5f84)?;
                                                    c.g[V0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                    c.g[A1][0] = 0x0046_0000;
                                                    c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32).wrapping_add(0x688));
                                                    c.g[A2][0] = 0x100;
                                                    c.g[A3][0] = 0u64;
                                                    call_ptr(c, 0x0031_7e30, 0x001b_5fa0)?;
                                                    c.g[A1][0] = c.g[S0][0];
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.sw(
                                                        (c.g[S3][0] as u32).wrapping_add(0x704),
                                                        c.g[V0][0],
                                                    );
                                                    call_ptr(c, 0x003d_be20, 0x001b_5fb0)?;
                                                    let t = true;
                                                    c.g[A0][0] = c.g[S3][0];
                                                    if t {
                                                        break 'b_1b5ff0;
                                                    } else {
                                                        break 'b_1b5fb8;
                                                    }
                                                }
                                            }
                                            c.cycles += 14;
                                            c.g[A0][0] = c.g[S1][0];
                                            call_ptr(c, 0x003d_bca0, 0x001b_5fc0)?;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                            c.g[A1][0] = 0x0046_0000;
                                            c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x688));
                                            c.g[A2][0] = 0x100;
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x0031_7e30, 0x001b_5fdc)?;
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[A0][0] = c.g[V0][0];
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x704),
                                                c.g[V0][0],
                                            );
                                            call_ptr(c, 0x002c_2508, 0x001b_5fec)?;
                                            c.g[A0][0] = c.g[S3][0];
                                            break 'b_1b5ff0;
                                        }
                                        c.cycles += 22;
                                        c.g[S2][0] = 0x1;
                                        call_ptr(c, 0x001a_9c50, 0x001b_5ff8)?;
                                        c.g[S1][0] = c.g[V0][0];
                                        c.g[A0][0] = 0x0046_0000;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0xbc), c.g[S2][0]);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1618));
                                        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xbc));
                                        call_ptr(c, 0x0031_7670, 0x001b_6010)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A2][0] = c.g[V0][0];
                                        c.g[A1][0] = 0u64;
                                        call_ptr(c, 0x001c_57c0, 0x001b_6020)?;
                                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x704));
                                        c.g[V1][0] = 0x81c;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0xc0), c.g[S2][0]);
                                        c.g[A0][0] = c.g[S3][0];
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                        c.g[A1][0] = c.g[S1][0];
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0x150), c.g[V1][0]);
                                        call_ptr(c, 0x001a_9bc0, 0x001b_6040)?;
                                        let t = true;
                                        c.g[V0][0] = 0u64;
                                        if t {
                                            break 'b_1b605c;
                                        } else {
                                            break 'b_1b6048;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x001a_8e40, 0x001b_6050)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_1b6060;
                    } else {
                        break 'b_1b6058;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0u64;
                break 'b_1b605c;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1b6060;
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use crate::game::fe::feasyncfile::tests::{
        fa_fake_call, fa_force, oc865_hash, oc865_str, FA_CTRL, FA_MAGIC,
    };
    use std::ptr::addr_of_mut;

    /// Gated fakes (like `feasyncfile`'s `fa_*`): while the machine's RAM
    /// holds `FA_MAGIC` at `FA_CTRL` they log the call and return as
    /// `fakes::fake_call` does; otherwise they run the body they replaced.
    macro_rules! oc919_fakes {
        ($(($fake:ident, $slot:expr, $gen:path, $addr:expr)),* $(,)?) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(FA_CTRL) != FA_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                    }
                    fa_fake_call(c, $addr);
                }
            )*
            fn oc919_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
                fakes::hook(
                    addr_of_mut!(ee::s06::OVERRIDE_0025F1E0),
                    oc919_fake_player_entry,
                    fakes::prev_cell(oc919_fake_player_entry),
                );
            }
        };
    }

    oc919_fakes!(
        (
            oc919_fake_001a8a40,
            ee::s03::OVERRIDE_001A8A40,
            ee::s03::f_001a8a40__generated,
            0x001a_8a40
        ),
        (
            oc919_fake_001b25e0,
            ee::s03::OVERRIDE_001B25E0,
            ee::s03::f_001b25e0__generated,
            0x001b_25e0
        ),
        (
            oc919_fake_001b3978,
            ee::s03::OVERRIDE_001B3978,
            ee::s03::f_001b3978__generated,
            0x001b_3978
        ),
        (
            oc919_fake_001b3bf0,
            ee::s03::OVERRIDE_001B3BF0,
            ee::s03::f_001b3bf0__generated,
            0x001b_3bf0
        ),
        (
            oc919_fake_001b3d20,
            ee::s03::OVERRIDE_001B3D20,
            ee::s03::f_001b3d20__generated,
            0x001b_3d20
        ),
        (
            oc919_fake_001bcb58,
            ee::s03::OVERRIDE_001BCB58,
            ee::s03::f_001bcb58__generated,
            0x001b_cb58
        ),
        (
            oc919_fake_0025da90,
            ee::s06::OVERRIDE_0025DA90,
            ee::s06::f_0025da90__generated,
            0x0025_da90
        ),
        (
            oc919_fake_0025f1b0,
            ee::s06::OVERRIDE_0025F1B0,
            ee::s06::f_0025f1b0__generated,
            0x0025_f1b0
        ),
        (
            oc919_fake_00317d70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc919_fake_00318260,
            ee::s10::OVERRIDE_00318260,
            ee::s10::f_00318260__generated,
            0x0031_8260
        ),
        (
            oc919_fake_003184e8,
            ee::s10::OVERRIDE_003184E8,
            ee::s10::f_003184e8__generated,
            0x0031_84e8
        ),
        (
            oc919_fake_0039b760,
            ee::s13::OVERRIDE_0039B760,
            ee::s13::f_0039b760__generated,
            0x0039_b760
        ),
        (
            oc919_fake_0039d860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc919_fake_003a0c50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc919_fake_003a0e90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90
        ),
        (
            oc919_fake_003a1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc919_fake_001a9bc0,
            ee::s03::OVERRIDE_001A9BC0,
            ee::s03::f_001a9bc0__generated,
            0x001a_9bc0
        ),
        (
            oc919_fake_001a9c50,
            ee::s03::OVERRIDE_001A9C50,
            ee::s03::f_001a9c50__generated,
            0x001a_9c50
        ),
        (
            oc919_fake_001c57c0,
            ee::s03::OVERRIDE_001C57C0,
            ee::s03::f_001c57c0__generated,
            0x001c_57c0
        ),
        (
            oc919_fake_002c26d0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0
        ),
        (
            oc919_fake_002c2718,
            ee::s08::OVERRIDE_002C2718,
            ee::s08::f_002c2718__generated,
            0x002c_2718
        ),
        (
            oc919_fake_001a97b8,
            ee::s03::OVERRIDE_001A97B8,
            ee::s03::f_001a97b8__generated,
            0x001a_97b8
        ),
        (
            oc919_fake_001b6ab8,
            ee::s03::OVERRIDE_001B6AB8,
            ee::s03::f_001b6ab8__generated,
            0x001b_6ab8
        ),
        (
            oc919_fake_001b6ed0,
            ee::s03::OVERRIDE_001B6ED0,
            ee::s03::f_001b6ed0__generated,
            0x001b_6ed0
        ),
        (
            oc919_fake_0025b800,
            ee::s06::OVERRIDE_0025B800,
            ee::s06::f_0025b800__generated,
            0x0025_b800
        ),
        (
            oc919_fake_00260f80,
            ee::s06::OVERRIDE_00260F80,
            ee::s06::f_00260f80__generated,
            0x0026_0f80
        ),
        (
            oc919_fake_002613c8,
            ee::s06::OVERRIDE_002613C8,
            ee::s06::f_002613c8__generated,
            0x0026_13c8
        ),
        (
            oc919_fake_00261408,
            ee::s06::OVERRIDE_00261408,
            ee::s06::f_00261408__generated,
            0x0026_1408
        ),
        (
            oc919_fake_00261460,
            ee::s06::OVERRIDE_00261460,
            ee::s06::f_00261460__generated,
            0x0026_1460
        ),
        (
            oc919_fake_002636f0,
            ee::s07::OVERRIDE_002636F0,
            ee::s07::f_002636f0__generated,
            0x0026_36f0
        ),
        (
            oc919_fake_001a8e40,
            ee::s03::OVERRIDE_001A8E40,
            ee::s03::f_001a8e40__generated,
            0x001a_8e40
        ),
        (
            oc919_fake_001aaf90,
            ee::s03::OVERRIDE_001AAF90,
            ee::s03::f_001aaf90__generated,
            0x001a_af90
        ),
        (
            oc919_fake_002c2508,
            ee::s08::OVERRIDE_002C2508,
            ee::s08::f_002c2508__generated,
            0x002c_2508
        ),
        (
            oc919_fake_00317e30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc919_fake_00317e98,
            ee::s10::OVERRIDE_00317E98,
            ee::s10::f_00317e98__generated,
            0x0031_7e98
        ),
        (
            oc919_fake_0039f190,
            ee::s13::OVERRIDE_0039F190,
            ee::s13::f_0039f190__generated,
            0x0039_f190
        ),
        (
            oc919_fake_003dbca0,
            ee::s14::OVERRIDE_003DBCA0,
            ee::s14::f_003dbca0__generated,
            0x003d_bca0
        ),
        (
            oc919_fake_003dbe20,
            ee::s14::OVERRIDE_003DBE20,
            ee::s14::f_003dbe20__generated,
            0x003d_be20
        ),
    );

    /// The flag word `LobbyMan_GetPlayerEntry` reports through its third
    /// argument (the fake writes it).
    const OC919_PLAYER_FLAG: u32 = DATA + 0x1f00;

    /// Gated fake of `LobbyMan_GetPlayerEntry(lobby, idx, &flag)`: like the
    /// other fakes, and it stores the fixture's flag word through `a2`.
    fn oc919_fake_player_entry(c: &mut Cpu) {
        if c.bus.read32(FA_CTRL) != FA_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc919_fake_player_entry),
                ee::s06::f_0025f1e0__generated,
            );
        }
        let flag_ptr = c.g[A2][0] as u32;
        let ra = c.g[RA][0] as u32;
        fa_fake_call(c, 0x0025_f1e0);
        if c.pc == ra {
            let flag = c.bus.read32(OC919_PLAYER_FLAG);
            c.bus.write32(flag_ptr, flag);
        }
    }

    /// `fakes::machine_rt` with the fakes installed and switched on.
    fn oc919_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc919_install_fakes();
        let r = fakes::machine_rt(c, seed, salt, calls);
        c.bus.write32(FA_CTRL, FA_MAGIC);
        r
    }

    const OC919_WORLD: u32 = DATA + 0x4000;
    const OC919_LOC: u32 = DATA + 0x4100;
    const OC919_LOC_VT: u32 = DATA + 0x4200;

    /// A vtable at `vt` whose slots `offs` are `{delta, fake method}`.
    fn oc919_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, offs: &[u32]) {
        for &off in offs {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, fakes::FAKE);
        }
    }

    /// The world at `gp - 0x848`, whose `+0x8c` object is the localisation
    /// table (its vtable slot `0x20` is a fake method).
    fn oc919_world(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC919_WORLD);
        c.bus.write32(OC919_WORLD + 0x8c, OC919_LOC);
        c.bus.write32(OC919_LOC + 4, OC919_LOC_VT);
        oc919_vtable(c, r, OC919_LOC_VT, &[0x20]);
    }

    /// `cFEStateChatRoom_RefreshPlayerList(self, reselect)`: rebuild the six
    /// visible rows of the chat room's player list from the lobby manager.
    #[test]
    fn cfe_state_chat_room_refresh_player_list_matches() {
        check(
            ee::s03::f_001b2378__generated,
            cfe_state_chat_room_refresh_player_list,
            200,
            &|c, seed| {
                let mut r = oc919_machine(c, seed, 0x001b_2378, 60);
                let this = DATA + 0x1000;
                let obj = DATA + 0x2000;
                let vt = DATA + 0x2100;
                let counts = [0u32, 1, 3, 5, 6, 7, 9];
                let n = counts[seed as usize % counts.len()];
                // The screen is absent for one seed in eight.
                let screen = if seed % 8 == 5 { 0 } else { DATA + 0x2800 };
                c.bus.write32(this + 0x40, screen);
                c.bus
                    .write32(this + 0x704, [0u32, 2, 5][(seed as usize / 7) % 3]);
                c.bus.write32(this + 0x10, DATA + 0x2900);
                c.bus.write32(this + 0x6e8, DATA + 0x2a00);
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x3000);
                c.bus.write32(GAME_GP + 0xda0, r.next_u32());
                oc865_str(c, 0x0046_3a38, "PlayerListMenu");
                oc865_str(c, 0x0046_3a48, "ListItemNameXXXX");
                oc865_str(c, 0x0046_3a60, "PlayerListItem%02d");
                fa_force(c, 0x0025_f1b0, n);
                fa_force(c, 0x0039_d860, obj);
                fa_force(c, 0x0025_da90, [0u32, 1, 5][(seed as usize / 3) % 3]);
                c.bus.write32(OC919_PLAYER_FLAG, (seed >> 2) & 1);
                // Every menu/row object is the same widget; the menu's row
                // count byte equals `n` for half of the seeds.
                c.bus.write32(obj + 8, vt);
                let count_byte = if (seed >> 1) & 1 == 0 {
                    n as u8
                } else {
                    n as u8 ^ 0x55
                };
                c.bus.write8(obj + 0x96, count_byte);
                oc919_vtable(c, &mut r, vt, &[0x58]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.pc = 0x001b_2378;
            },
        );
    }

    /// Offsets (from the string bases `0x460000` and `0x4a0000`) of the
    /// widget names `cFEStateChatRoom_OnWidgetCreated` compares the id with.
    const OC919_WIDGET_NAME_OFFS: [u32; 19] = [
        0x3c08, 0x3c18, 0x3c30, 0x3108, 0x3c48, 0x3c68, 0x3a38, 0x3c80, 0x3ad8, 0x3c90, 0x3ca0,
        0x3cb0, 0x3cc0, 0x3cd0, 0x3ce0, 0x3cf0, 0x1b68, 0x3d00, 0x1b70,
    ];

    /// `cFEStateChatRoom_OnWidgetCreated(self, widget)`: remember the widget
    /// (by name hash) in its slot of the chat room and set it up; widgets
    /// named `quality1..6` go to the quality icon array.
    #[test]
    fn cfe_state_chat_room_on_widget_created_matches() {
        check(
            ee::s03::f_001b2eb8__generated,
            cfe_state_chat_room_on_widget_created,
            200,
            &|c, seed| {
                let mut r = oc919_machine(c, seed, 0x001b_2eb8, 10);
                let this = DATA + 0x1000;
                let widget = DATA + 0x2000;
                let vt = DATA + 0x2100;
                // A distinct name at every address the function may hash.
                let mut names = Vec::new();
                for base in [0x0046_0000u32, 0x004a_0000] {
                    for off in OC919_WIDGET_NAME_OFFS {
                        let addr = base + off;
                        let name = format!("p{addr:x}");
                        oc865_str(c, addr, &name);
                        names.push(name);
                    }
                }
                oc865_str(c, 0x0046_3d10, "q%d");
                let pick = seed as usize % 48;
                let id = if pick < names.len() {
                    oc865_hash(&names[pick])
                } else if pick < names.len() + 6 {
                    oc865_hash(&format!("q{}", pick - names.len() + 1))
                } else {
                    r.next_u32()
                };
                c.bus.write32(widget + 0x38, id);
                c.bus.write32(widget + 0x14, r.next_u32());
                c.bus.write32(widget + 8, vt);
                oc919_vtable(c, &mut r, vt, &[0x38, 0x48]);
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x3000);
                fa_force(c, 0x0025_f1b0, [0u32, 3, 8][(seed as usize / 48) % 3]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(widget);
                c.pc = 0x001b_2eb8;
            },
        );
    }

    /// `cFEStateNetConfig_ShowCardErrorPopup(self, popupId, cardError,
    /// cardSlot)`: fill in the network popup's message for a memory card
    /// error (not detected, not formatted, corrupt, different card) and
    /// show it.
    #[test]
    fn cfe_state_net_config_show_card_error_popup_matches() {
        check(
            ee::s03::f_001b4888__generated,
            cfe_state_net_config_show_card_error_popup,
            220,
            &|c, seed| {
                let mut r = oc919_machine(c, seed, 0x001b_4888, 22);
                oc919_world(c, &mut r);
                let this = DATA + 0x1000;
                let popup = DATA + 0x3000;
                fa_force(c, 0x001a_9c50, popup);
                let errors = [
                    2u64,
                    6,
                    0x14,
                    0x18,
                    0,
                    1,
                    3,
                    7,
                    0x15,
                    0x100,
                    0xffff_ffff_ffff_fff0,
                ];
                let slots = [u64::MAX, 0, 1, 2, 5, 0xffff_ffff];
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = errors[seed as usize % errors.len()];
                c.g[A3][0] = slots[(seed as usize / errors.len()) % slots.len()];
                c.pc = 0x001b_4888;
            },
        );
    }

    /// `cFEStateNetConfig_OnAuthPopupEvent(self, child, event)`: the
    /// authentication popup's outcome (retry, cancel, close, error) updates
    /// the state flags and pushes the next popup.
    #[test]
    fn cfe_state_net_config_on_auth_popup_event_matches() {
        check(
            ee::s03::f_001b5830__generated,
            cfe_state_net_config_on_auth_popup_event,
            256,
            &|c, seed| {
                let mut r = oc919_machine(c, seed, 0x001b_5830, 20);
                let this = DATA + 0x1000;
                let child = DATA + 0x1800;
                let pair = DATA + 0x3a00;
                let pair_vt = DATA + 0x3b00;
                let screen = DATA + 0x3c00;
                let ring = DATA + 0x3d00;
                let owner = DATA + 0x3e00;
                let item = DATA + 0x3f00;
                let list = owner + 0x18;
                let kinds = [0x10u64, 0xf, 0x14, 0x16, 0x11, 0, 0x20, 0x1_0000_0010];
                let s = seed as usize;
                c.bus.write32(this + 0x10, DATA + 0x1900);
                c.bus.write32(this + 0x40, screen);
                c.bus.write32(this + 0x6d8, ((s >> 6) & 1) as u32);
                c.bus.write32(this + 0x6ec, pair);
                c.bus.write32(this + 0x700, ((s >> 5) & 1) as u32);
                c.bus.write32(this + 0x710, r.next_u32());
                c.bus.write32(this + 0x714, r.next_u32());
                c.bus.write32(this + 0x718, r.next_u32());
                c.bus.write32(pair + 8, pair_vt);
                oc919_vtable(c, &mut r, pair_vt, &[0x40]);
                // screen -> state ring -> owner holding the (empty) popup list
                c.bus.write32(screen + 0xd0, ring);
                c.bus.write32(ring + 0x10, owner);
                c.bus.write32(list, list);
                c.bus.write32(list + 4, list + 0xc);
                c.bus.write32(list + 0x10, list + 0xc);
                c.bus.write32(list + 0xc, list);
                // The network session (absent for half of the seeds).
                let session = if (s >> 3) & 1 == 0 { 0 } else { DATA + 0x3800 };
                c.bus.write32(GAME_GP + 0x238, session);
                fa_force(c, 0x0026_1460, ((s >> 4) & 1) as u32);
                fa_force(c, 0x001b_6ab8, item);
                fa_force(c, 0x001b_6ed0, item);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(child);
                c.g[A2][0] = kinds[s % kinds.len()];
                c.pc = 0x001b_5830;
            },
        );
    }

    /// `cFEStateNetConfig_v31(self, id, message)` (the state's net message
    /// handler): close on 0x105, and for the disconnect messages 277..278 show
    /// a popup with the error text of the message; everything else goes to
    /// the base handler.
    #[test]
    fn cfe_state_net_config_v31_matches() {
        check(
            ee::s03::f_001b5e28__generated,
            cfe_state_net_config_v31,
            264,
            &|c, seed| {
                let mut r = oc919_machine(c, seed, 0x001b_5e28, 14);
                oc919_world(c, &mut r);
                let this = DATA + 0x1000;
                let msg = DATA + 0x1800;
                let popup = DATA + 0x3000;
                let ids = [
                    0x105u64,
                    0x104,
                    0x106,
                    277,
                    278,
                    279,
                    276,
                    0x1ff,
                    0x200,
                    0,
                    0xffff_ffff_ffff_ff00,
                ];
                let types = [
                    0u32,
                    0x2d64_6576,
                    0x2d61_7468,
                    0x2d63_6667,
                    0x2d69_6e69,
                    0x2d64_6966,
                    0x2d74_6f6e,
                    0x1234_5678,
                ];
                let s = seed as usize;
                c.bus.write32(this + 0x10, DATA + 0x1900);
                c.bus.write32(this + 0x1c, [0u32, 2, 4, 8][(s / 3) & 3]);
                c.bus.write32(
                    this + 0x704,
                    if (s >> 4) & 1 == 0 { 0 } else { DATA + 0x3900 },
                );
                c.bus.write32(msg + 8, types[(s / ids.len()) % types.len()]);
                fa_force(c, 0x001a_9c50, popup);
                fa_force(
                    c,
                    0x001a_af90,
                    if (s >> 5) & 1 == 0 { 0 } else { DATA + 0x3a00 },
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = ids[s % ids.len()];
                c.g[A2][0] = sx(msg);
                c.pc = 0x001b_5e28;
            },
        );
    }
}
