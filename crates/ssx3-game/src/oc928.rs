//! Staging module oc928 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc928`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001c_f270,
    c_fe_state_reward_gallery_handle_input,
    "cFEStateRewardGallery_HandleInput",
)];

/// `int cFEStateRewardGallery_HandleInput(cFEStateRewardGallery* self,
/// cUIEvent* ev)` (0x1cf270): the reward gallery's pad handling. Back (event
/// vtable +0x34) hides the page's cells, starts the screen's exit thread and
/// switches state; confirm (+0x2c) buys or opens the item under the cursor
/// (self +0x64 page start, +0x5c columns, +0x68 item count, +0x8f0 reward kind);
/// left/right/up/down (+0x9c, +0xa4, +0x8c, +0x94) move the cursor inside the
/// page grid (+0x58 rows, wrapping) and the page up/down events (+0xdc, +0xcc)
/// move the page start by one row block (+0x60 rows in all). A changed cursor
/// stores the new cell (+0x8fc), notifies the state's vtable (+0x144, +0x10c)
/// and sends a small code (1 move, 2 horizontal move, 4 refused, 3 back, 6 buy;
/// probably a UI sound or animation id) to the default object (`self->+0x10->+0x14`, else gp+0x2968).
/// Returns 1 when the event was handled, 0 otherwise. The body keeps the
/// generated block structure (labels are the generated block addresses).
pub fn c_fe_state_reward_gallery_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 16, 0x80);
        frame.save(c, 18, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 22, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x70);
        c.g[S6][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2968));
        frame.save(c, 19, 0x50);
        frame.save(c, 20, 0x40);
        frame.save(c, 21, 0x30);
        frame.save(c, 23, 0x10);
        frame.save_ra(c, 0);
        'b_1cf7f4: {
            'b_1cf3a8: {
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x34));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                let t = c.g[V0][0] as u32;
                if c.g[V1][0] != 0 {
                    c.g[S6][0] = c.g[V1][0];
                }
                call_ptr(c, t, 0x001cf2c4)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x3;
                if t {
                    break 'b_1cf3a8;
                } else {
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x924));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001cf2e4)?;
                    c.g[V1][0] = 0u64;
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 5);
                    'l_1cf2f0: loop {
                        c.cycles += 4;
                        c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                        c.g[S1][0] = 0x7;
                        'l_1cf300: loop {
                            'b_1cf320: {
                                c.cycles += 3;
                                c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_1cf320;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001cf320)?;
                                    break 'b_1cf320;
                                }
                            }
                            c.cycles += 3;
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[S1][0] as i64) >= 0;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001cf300);
                                }
                                continue 'l_1cf300;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = c.g[S3][0];
                                c.g[V0][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 5);
                                    if event_due(c) {
                                        deliver_event_here(c, 0x001cf2f0);
                                    }
                                    continue 'l_1cf2f0;
                                } else {
                                    c.cycles += 27;
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x23a0));
                                    call_ptr(c, 0x00317670, 0x001cf348)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039c778, 0x001cf354)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                    c.g[A2][0] = 0x1;
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039c870, 0x001cf364)?;
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8f0));
                                    c.g[A3][0] = c.lw((c.g[V1][0] as u32));
                                    c.g[A2][0] = !(0u64 | c.g[A2][0]);
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[A2][0] = sx((c.g[A2][0] as u32) >> 31);
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001cf390)?;
                                    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f400, 0x001cf3a0)?;
                                    c.g[V0][0] = 0x1;
                                    break 'b_1cf7f4;
                                }
                            }
                        }
                    }
                }
            }
            'b_1cf3b8: {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8fc));
                let t = c.g[V0][0] == 0u64;
                c.g[S7][0] = 0u64;
                if t {
                    break 'b_1cf3b8;
                } else {
                    c.cycles += 1;
                    c.g[S7][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    break 'b_1cf3b8;
                }
            }
            'b_1cf3cc: {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                c.g[S5][0] = 0u64;
                c.div(0, 23, 2);
                break 'b_1cf3cc;
            }
            'b_1cf7dc: {
                'b_1cf7d8: {
                    'b_1cf54c: {
                        c.cycles += 12;
                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[S3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[A1][0] = c.lo[0];
                        c.g[A2][0] = c.hi[0];
                        c.g[S1][0] = c.g[A1][0];
                        let t = c.g[V0][0] as u32;
                        c.g[S4][0] = c.g[A2][0];
                        call_ptr(c, t, 0x001cf3f4)?;
                        if c.g[V0][0] == 0u64 {
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1cf54c;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                            c.mult(0, 5, 2, 3);
                            let t = (c.g[A0][0] as i64) <= 0;
                            c.g[S3][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                            if t {
                                break 'b_1cf7d8;
                            } else {
                                'b_1cf52c: {
                                    'b_1cf510: {
                                        'b_1cf50c: {
                                            'b_1cf4fc: {
                                                'b_1cf468: {
                                                    c.cycles += 8;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = c.g[S3][0];
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V1][0] as u32).wrapping_add(0x130)
                                                        );
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x134)
                                                        );
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x001cf42c)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x8;
                                                    if t {
                                                        break 'b_1cf468;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x4c));
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[A1][0] = 0x6;
                                                        if t {
                                                            break 'b_1cf50c;
                                                        } else {
                                                            c.cycles += 10;
                                                            c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[A0][0] = sx((c.g[S6][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x001cf454)?;
                                                            c.g[V0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = c.g[S3][0];
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0xf8));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xfc));
                                                            break 'b_1cf4fc;
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8f0));
                                                let t = (c.g[V0][0] as i64) < 0;
                                                c.g[A1][0] = c.g[S3][0];
                                                if t {
                                                    break 'b_1cf52c;
                                                } else {
                                                    c.cycles += 15;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V1][0] as u32).wrapping_add(0x110)
                                                        );
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x114)
                                                        );
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x001cf488)?;
                                                    c.g[V1][0] =
                                                        c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                    c.g[S1][0] = c.g[V0][0];
                                                    c.g[S0][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x8f0)
                                                        );
                                                    c.g[A1][0] = c.g[S1][0];
                                                    c.g[A0][0] =
                                                        c.lh(
                                                            (c.g[V1][0] as u32).wrapping_add(0x120)
                                                        );
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x124)
                                                        );
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                        .wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x001cf4a8)?;
                                                    if c.g[S0][0] != c.g[V0][0] {
                                                        c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                                                        break 'b_1cf510;
                                                    } else {
                                                        c.cycles += 10;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[A1][0] = c.g[S1][0];
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x118));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x11c));
                                                        let t = c.g[V0][0] as u32;
                                                        c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        call_ptr(c, t, 0x001cf4c8)?;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x54));
                                                        c.g[V1][0] = ((c.g[V1][0] as i64)
                                                            < (c.g[V0][0] as i64))
                                                            as u64;
                                                        let t = c.g[V1][0] != 0u64;
                                                        c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                                                        if t {
                                                            break 'b_1cf510;
                                                        } else {
                                                            c.cycles += 9;
                                                            c.g[A1][0] = 0x6;
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[A0][0] = sx((c.g[S6][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x001cf4ec)?;
                                                            c.g[V0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = c.g[S3][0];
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x100));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x104));
                                                            break 'b_1cf4fc;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 4;
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] = sx((c.g[S2][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001cf504)?;
                                            c.g[V0][0] = 0x1;
                                            break 'b_1cf7f4;
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                                        break 'b_1cf510;
                                    }
                                    c.cycles += 7;
                                    c.g[A1][0] = 0x4;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001cf524)?;
                                    c.g[V0][0] = 0x1;
                                    break 'b_1cf7f4;
                                }
                                c.cycles += 8;
                                c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                                c.g[A1][0] = 0x4;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                                let t = c.g[V1][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001cf544)?;
                                c.g[V0][0] = 0x1;
                                break 'b_1cf7f4;
                            }
                        }
                    }
                    'b_1cf6c8: {
                        'b_1cf57c: {
                            c.cycles += 6;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001cf55c)?;
                            if c.g[V0][0] == 0u64 {
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                break 'b_1cf57c;
                            } else {
                                c.cycles += 3;
                                c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1));
                                let t = (c.g[S4][0] as i64) >= 0;
                                c.g[S5][0] = 0x1;
                                if t {
                                    break 'b_1cf6c8;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                                    c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    break 'b_1cf6c8;
                                }
                            }
                        }
                        'b_1cf698: {
                            'b_1cf690: {
                                'b_1cf5d0: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001cf58c)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        break 'b_1cf5d0;
                                    } else {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[S4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[S4][0] = 0u64;
                                            break 'b_1cf690;
                                        } else {
                                            c.cycles += 8;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[S1][0] as u32)));
                                            c.mult(0, 5, 2, 4);
                                            c.g[V0][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[S4][0] as u32)));
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[S5][0] = 0x1;
                                            if t {
                                                break 'b_1cf6c8;
                                            } else {
                                                c.cycles += 2;
                                                c.g[S4][0] = 0u64;
                                                break 'b_1cf6c8;
                                            }
                                        }
                                    }
                                }
                                'b_1cf600: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001cf5e0)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        break 'b_1cf600;
                                    } else {
                                        c.cycles += 3;
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
                                        let t = (c.g[S1][0] as i64) >= 0;
                                        c.g[S5][0] = 0x1;
                                        if t {
                                            break 'b_1cf6c8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                                            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                            break 'b_1cf6c8;
                                        }
                                    }
                                }
                                'b_1cf64c: {
                                    c.cycles += 6;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001cf610)?;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        break 'b_1cf64c;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                                        c.g[V0][0] =
                                            ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[S1][0] = 0u64;
                                            break 'b_1cf690;
                                        } else {
                                            c.cycles += 6;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                                            c.g[V1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x60));
                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                .wrapping_add((c.g[S1][0] as u32)));
                                            c.g[V1][0] =
                                                ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[S5][0] = 0x1;
                                            if t {
                                                break 'b_1cf6c8;
                                            } else {
                                                c.cycles += 2;
                                                c.g[S1][0] = 0u64;
                                                break 'b_1cf6c8;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 6;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xd8));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xdc));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001cf65c)?;
                                if c.g[V0][0] == 0u64 {
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    break 'b_1cf698;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                                    c.g[S3][0] =
                                        sx((c.g[S3][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                                    let t = (c.g[S3][0] as i64) >= 0;
                                    c.g[S5][0] = 0x1;
                                    if t {
                                        break 'b_1cf6c8;
                                    } else {
                                        'b_1cf680: {
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x60));
                                            break 'b_1cf680;
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                        c.div(0, 2, 3);
                                        c.g[V0][0] = c.lo[0];
                                        c.mult(0, 19, 2, 3);
                                        break 'b_1cf690;
                                    }
                                }
                            }
                            c.cycles += 2;
                            c.g[S5][0] = 0x1;
                            break 'b_1cf6c8;
                        }
                        c.cycles += 6;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc8));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xcc));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001cf6a8)?;
                        let t = c.g[V0][0] == 0u64;
                        if t {
                            break 'b_1cf6c8;
                        } else {
                            c.cycles += 6;
                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x58));
                            c.g[S5][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x60));
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                c.g[S3][0] = 0u64;
                            }
                            break 'b_1cf6c8;
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[S5][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_1cf7f4;
                    } else {
                        'b_1cf710: {
                            c.cycles += 12;
                            c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5c));
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x68));
                            c.mult(0, 3, 17, 6);
                            c.mult(0, 4, 19, 6);
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
                            c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                c.g[S1][0] = c.g[V1][0];
                            }
                            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
                            if c.g[S7][0] != c.g[A1][0] {
                                c.div(0, 5, 6);
                                break 'b_1cf710;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x64));
                                if c.g[S3][0] == c.g[V0][0] {
                                    c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                                    break 'b_1cf7dc;
                                } else {
                                    c.cycles += 1;
                                    c.div(0, 5, 6);
                                    break 'b_1cf710;
                                }
                            }
                        }
                        'b_1cf71c: {
                            c.cycles += 3;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x64), c.g[S3][0]);
                            break 'b_1cf71c;
                        }
                        'b_1cf7bc: {
                            'b_1cf7a0: {
                                c.cycles += 26;
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lo[0];
                                c.g[V0][0] = c.hi[0];
                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[V0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x924));
                                c.sw((c.g[S2][0] as u32).wrapping_add(0x8fc), c.g[V1][0]);
                                c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x140));
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x144));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001cf750)?;
                                c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x108));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10c));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001cf768)?;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x88));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001cf77c)?;
                                if c.g[V0][0] != 0u64 {
                                    c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                                    break 'b_1cf7a0;
                                } else {
                                    c.cycles += 7;
                                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x90));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x94));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                    call_ptr(c, t, 0x001cf798)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                                    if t {
                                        break 'b_1cf7bc;
                                    } else {
                                        break 'b_1cf7a0;
                                    }
                                }
                            }
                            c.cycles += 7;
                            c.g[A1][0] = 0x2;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x001cf7b4)?;
                            c.g[V0][0] = 0x1;
                            break 'b_1cf7f4;
                        }
                        c.cycles += 7;
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001cf7d0)?;
                        c.g[V0][0] = 0x1;
                        break 'b_1cf7f4;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S6][0] as u32));
                break 'b_1cf7dc;
            }
            c.cycles += 6;
            c.g[A1][0] = 0x4;
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x001cf7f0)?;
            c.g[V0][0] = 0x1;
            break 'b_1cf7f4;
        }
        c.cycles += 11;
        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore(c, 20, 0x40);
        frame.restore(c, 21, 0x30);
        frame.restore(c, 22, 0x20);
        frame.restore(c, 23, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{fresh, run_generated, Rng, DATA, GAME_GP};
    use crate::game::fe::festaterewards::tests::{base_9, force, strict, KEEP_F0};

    /// Gate word of this module's callee fakes (the machine's RAM, never
    /// `DATA+0x100000`).
    const OC928_CTRL: u32 = DATA + 0x0020_0000;
    const OC928_MAGIC: u32 = 0x0928_c0de;

    // Fixture layout (the state `self` is DATA, 0xa00 bytes).
    const OC928_EV: u32 = DATA + 0x1000;
    const OC928_EV_VT: u32 = DATA + 0x1100;
    const OC928_HOLDER: u32 = DATA + 0x1200;
    const OC928_DEF_OBJ: u32 = DATA + 0x1300;
    const OC928_DEF_VT: u32 = DATA + 0x1400;
    const OC928_KID_VT: u32 = DATA + 0x1500;
    const OC928_CELL: u32 = DATA + 0x1600;
    const OC928_SCREEN: u32 = DATA + 0x1700;
    const OC928_SELF_VT: u32 = DATA + 0x2000;
    const OC928_KIDS: u32 = DATA + 0x3000;

    fn oc928_open(c: &mut Cpu) -> bool {
        c.bus.read32(OC928_CTRL) == OC928_MAGIC
    }

    /// `StrHash`, `UIScreen_LookupValue`, `UIScreen_StartThread` and
    /// `UIStateMgr_SwitchState` are faked while the gate word is set (they run
    /// real UI code on fixture data otherwise).
    fn oc928_fake_317670(c: &mut Cpu) {
        if !oc928_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc928_fake_317670),
                ee::s10::f_00317670__generated,
            );
        }
        fakes::fake_call(c, 0x0031_7670);
    }

    fn oc928_fake_39c778(c: &mut Cpu) {
        if !oc928_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc928_fake_39c778),
                ee::s13::f_0039c778__generated,
            );
        }
        fakes::fake_call(c, 0x0039_c778);
    }

    fn oc928_fake_39c870(c: &mut Cpu) {
        if !oc928_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc928_fake_39c870),
                ee::s13::f_0039c870__generated,
            );
        }
        fakes::fake_call(c, 0x0039_c870);
    }

    fn oc928_fake_39f400(c: &mut Cpu) {
        if !oc928_open(c) {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc928_fake_39f400),
                ee::s13::f_0039f400__generated,
            );
        }
        fakes::fake_call(c, 0x0039_f400);
    }

    fn oc928_install() {
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc928_fake_317670,
            fakes::prev_cell(oc928_fake_317670),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039C778),
            oc928_fake_39c778,
            fakes::prev_cell(oc928_fake_39c778),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc928_fake_39c870,
            fakes::prev_cell(oc928_fake_39c870),
        );
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s13::OVERRIDE_0039F400),
            oc928_fake_39f400,
            fakes::prev_cell(oc928_fake_39f400),
        );
    }

    fn oc928_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// A reward gallery with its event: `seed % 9` picks the pad event that is
    /// "pressed" (back, confirm, left, right, up, down, page up, page down,
    /// none); the grid, cursor and reward kind are random. `kids` fills the
    /// 4 x 8 cell table (the back path calls each cell), `calls` bounds the
    /// call index at which a fake unwinds.
    fn oc928_fix(c: &mut Cpu, seed: u32, calls: u32, kids: bool) {
        oc928_install();
        let mut r: Rng = base_9(c, seed, 0x1cf270, calls);
        oc928_w(c, OC928_CTRL, OC928_MAGIC);
        let me = DATA;
        c.g[A0][0] = me as u64;
        c.g[A1][0] = OC928_EV as u64;

        // The state, its vtable and the default object.
        oc928_w(c, me + 8, OC928_SELF_VT);
        oc928_w(c, me + 0x10, OC928_HOLDER);
        oc928_w(
            c,
            OC928_HOLDER + 0x14,
            if seed % 4 == 0 { 0 } else { OC928_DEF_OBJ },
        );
        oc928_w(c, OC928_DEF_OBJ, OC928_DEF_VT);
        oc928_w(c, GAME_GP + 0x2968, OC928_DEF_VT);
        c.bus.write16(OC928_DEF_VT + 0x10, 0);
        oc928_w(c, OC928_DEF_VT + 0x14, FAKE + 0x100);
        oc928_w(c, me + 0x40, OC928_SCREEN);

        let cols = 1 + r.next_u32() % 4;
        let rows = 1 + r.next_u32() % 3;
        oc928_w(c, me + 0x5c, cols);
        oc928_w(c, me + 0x58, rows);
        oc928_w(c, me + 0x60, 1 + r.next_u32() % 8);
        oc928_w(c, me + 0x64, r.next_u32() % 4);
        oc928_w(c, me + 0x68, r.next_u32() % 31);
        oc928_w(c, me + 0x54, r.next_u32() % 10);
        let k4c = r.next_u32();
        oc928_w(c, me + 0x4c, if k4c % 2 == 0 { 8 } else { k4c % 8 });
        let kind = if seed % 3 == 0 {
            (-5i32) as u32
        } else {
            r.next_u32() % 7
        };
        oc928_w(c, me + 0x8f0, kind);
        oc928_w(c, me + 0x8fc, if seed % 5 == 0 { 0 } else { OC928_CELL });
        oc928_w(c, OC928_CELL + 0x18, r.next_u32() % (cols * rows + 3));

        // The 4 x 8 cell table at +0x924 (row stride 0x20), each cell with a
        // vtable slot +0x4c.
        oc928_w(c, OC928_KID_VT + 0x48, 0);
        oc928_w(c, OC928_KID_VT + 0x4c, FAKE + 0x110);
        if kids {
            for row in 0..4u32 {
                for col in 0..8u32 {
                    let k = OC928_KIDS + 0x40 * (row * 8 + col);
                    let here = r.next_u32() % 3 != 0;
                    oc928_w(
                        c,
                        me + 0x924 + 0x20 * row + 4 * col,
                        if here { k } else { 0 },
                    );
                    oc928_w(c, k + 8, OC928_KID_VT);
                }
            }
        }

        // The event's vtable: pad queries (delta, function) at off, off+4.
        oc928_w(c, OC928_EV + 8, OC928_EV_VT);
        const EV_SLOTS: [u32; 8] = [0x30, 0x28, 0x98, 0xa0, 0x88, 0x90, 0xd8, 0xc8];
        let active = seed % 9;
        for (i, &off) in EV_SLOTS.iter().enumerate() {
            let target = FAKE + 0x10 * i as u32;
            oc928_w(c, OC928_EV_VT + off, 0);
            oc928_w(c, OC928_EV_VT + off + 4, target);
            let i = i as u32;
            let v = if i < active {
                0
            } else if i == active {
                1
            } else {
                r.next_u32() & 1
            };
            force(c, i, target, v, KEEP_F0);
        }

        // The state's vtable: the item queries and notifications.
        const SELF_SLOTS: [u32; 8] = [0x130, 0x110, 0x120, 0x118, 0xf8, 0x100, 0x140, 0x108];
        for (i, &off) in SELF_SLOTS.iter().enumerate() {
            let target = FAKE + 0x80 + 0x10 * i as u32;
            oc928_w(c, OC928_SELF_VT + off, 0);
            oc928_w(c, OC928_SELF_VT + off + 4, target);
        }
        let same_kind = r.next_u32() & 1 != 0;
        force(c, 8, FAKE + 0x80, r.next_u32() & 1, KEEP_F0);
        force(c, 9, FAKE + 0x90, r.next_u32() % 40, KEEP_F0);
        force(
            c,
            10,
            FAKE + 0xa0,
            if same_kind && (kind as i32) >= 0 {
                kind
            } else {
                r.next_u32() % 7
            },
            KEEP_F0,
        );
        force(c, 11, FAKE + 0xb0, r.next_u32() % 10, KEEP_F0);
    }

    /// The pad handling matches the generated body: every event, cursor
    /// wrap, confirm outcome, and unwinds inside the call chains.
    #[test]
    fn c_fe_state_reward_gallery_handle_input_matches() {
        strict(
            ee::s04::f_001cf270__generated,
            c_fe_state_reward_gallery_handle_input,
            180,
            &|c, seed| oc928_fix(c, seed, 12, true),
        );
    }

    /// The fixture reaches many distinct paths (total cycle costs of the
    /// generated body over the seeds: no unwinds, no cell table).
    #[test]
    fn c_fe_state_reward_gallery_handle_input_paths() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..400u32 {
            let mut c = fresh();
            oc928_fix(&mut c, seed, 0, false);
            let before = c.cycles;
            run_generated(&mut c, ee::s04::f_001cf270__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 16, "only {} paths: {seen:?}", seen.len());
    }
}
