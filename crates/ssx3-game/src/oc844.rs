//! Staging module oc844 (wave wave249): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc844`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// `void cFEStateProfileLoad::OnWidgetAction(cFEStateProfileLoad* this, void* item, int event)`
/// (0x18fd90): the profile load/delete screen's widget handler. A jump table
/// (0x45e2f0, event 5..=14) picks the case:
///
/// - 5 (item selected): with a valid item and no operation running, remembers
///   the slot and either opens the 8-character name keyboard popup (load
///   screens 1 and 3) or confirms the load (screen 2);
/// - 6 (back): cancels the card operation and sets the state to 6;
/// - 7 (delete): starts the delete action on the selected slot;
/// - 13 (keyboard done): stores the typed name (or "Player N") on the player
///   interface and the memory card manager, then begins the card operation;
/// - the others do nothing.
///
/// The generated body is a labelled-block jump table with an `__entries` twin
/// for resumes; the guard keeps the entry form.
pub fn cfe_state_profile_load_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x0018_fd90 {
        return ee::s02::f_0018fd90__entries(c);
    }
    guest(c, |c| 'l_18fd90: loop {
        'b_1902a0: {
            'b_19029c: {
                'b_190298: {
                    c.cycles += 13;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x5));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[V0][0] = (c.g[V1][0] < 0xau64) as u64;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S1][0] = c.g[A1][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
                    let t = c.g[V0][0] == 0u64;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
                    if t {
                        break 'b_190298;
                    } else {
                        'b_1901cc: {
                            'b_190164: {
                                'b_18fecc: {
                                    c.cycles += 7;
                                    c.g[V0][0] = 0x460000;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1d10));
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                    let t = c.g[A0][0] as u32;
                                    match t {
                                        0x0018fd90 => {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x0018fd90);
                                            }
                                            continue 'l_18fd90;
                                        }
                                        0x0018fde0 => {
                                            'b_18feac: {
                                                'b_18fe3c: {
                                                    c.cycles += 14;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x1a4)
                                                        );
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[S0][0] =
                                                        sx((c.g[V0][0] as u32).wrapping_add(0x74));
                                                    call_ptr(c, 0x00227f80, 0x0018fdf0)?;
                                                    c.g[S5][0] = c.g[V0][0];
                                                    call_ptr(c, 0x0014dc80, 0x0018fdf8)?;
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[A1][0] = 0x1;
                                                    call_ptr(c, 0x0014dd58, 0x0018fe04)?;
                                                    c.g[S4][0] = c.g[V0][0];
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x00416810, 0x0018fe10)?;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    if t {
                                                        break 'b_18fe3c;
                                                    } else {
                                                        c.cycles += 9;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1cc));
                                                        c.g[A0][0] = c.g[S4][0];
                                                        c.g[A2][0] = c.g[S0][0];
                                                        call_ptr(c, 0x00147138, 0x0018fe28)?;
                                                        c.g[A0][0] =
                                                            sx((c.g[S5][0] as u32)
                                                                .wrapping_add(0x276));
                                                        c.g[A1][0] = c.g[S0][0];
                                                        call_ptr(c, 0x002c2540, 0x0018fe34)?;
                                                        let t = true;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S4][0] as u32)
                                                                .wrapping_add(0xc));
                                                        if t {
                                                            break 'b_18feac;
                                                        } else {
                                                            break 'b_18fe3c;
                                                        }
                                                    }
                                                }
                                                c.cycles += 28;
                                                c.g[A0][0] = 0x460000;
                                                c.g[S2][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x70));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x1d50));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                call_ptr(c, 0x00317670, 0x0018fe5c)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, t, 0x0018fe6c)?;
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[A0][0] = c.g[S2][0];
                                                call_ptr(c, 0x002c2580, 0x0018fe78)?;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                                                c.g[A0][0] = c.g[SP][0];
                                                c.g[A1][0] = c.g[S2][0];
                                                c.g[A2][0] =
                                                    sx((c.g[A2][0] as u32).wrapping_add(0x1));
                                                call_ptr(c, 0x00417828, 0x0018fe8c)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1cc));
                                                c.g[A0][0] = c.g[S4][0];
                                                c.g[A2][0] = c.g[SP][0];
                                                call_ptr(c, 0x00147138, 0x0018fe9c)?;
                                                c.g[A0][0] =
                                                    sx((c.g[S5][0] as u32).wrapping_add(0x276));
                                                c.g[A1][0] = c.g[SP][0];
                                                call_ptr(c, 0x002c2540, 0x0018fea8)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                                                break 'b_18feac;
                                            }
                                            c.cycles += 8;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                            let t = c.g[V1][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[S4][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x0018febc)?;
                                            c.g[A0][0] = c.g[S3][0];
                                            Callee {
                                                addr: 0x0018_fc98,
                                                f: ee::s02::cFEStateProfileLoad_BeginCardOp,
                                            }
                                            .call(c, 0x0018fec4)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                            if t {
                                                break 'b_19029c;
                                            } else {
                                                break 'b_18fecc;
                                            }
                                        }
                                        0x0018fecc => {
                                            break 'b_18fecc;
                                        }
                                        0x00190164 => {
                                            break 'b_190164;
                                        }
                                        0x001901cc => {
                                            break 'b_1901cc;
                                        }
                                        0x00190298 => {
                                            break 'b_190298;
                                        }
                                        _ => {
                                            c.jump_out(t, 0x0018fd90, 0x001902bc);
                                            return Ok(());
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S1][0] == 0u64;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_19029c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                                    if c.g[V0][0] != 0u64 {
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        break 'b_1902a0;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        call_ptr(c, 0x00227f80, 0x0018fee8)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
                                        c.g[V0][0] = 0x3;
                                        if c.g[A1][0] != c.g[V0][0] {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                            break 'b_19029c;
                                        } else {
                                            'b_190134: {
                                                c.cycles += 6;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                                c.g[V1][0] = 0x2;
                                                c.g[A2][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b08));
                                                c.sw(
                                                    (c.g[S3][0] as u32).wrapping_add(0x1c4),
                                                    c.g[V0][0],
                                                );
                                                let t = c.g[A2][0] == c.g[V1][0];
                                                c.sw(
                                                    (c.g[A0][0] as u32).wrapping_add(0xf8),
                                                    c.g[V0][0],
                                                );
                                                if t {
                                                    break 'b_190134;
                                                } else {
                                                    'b_18ff38: {
                                                        'b_18ff30: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] =
                                                                ((c.g[A2][0] as i64) < 3i64) as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[V0][0] = 0x1;
                                                            if t {
                                                                break 'b_18ff30;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = c.g[A2][0] == c.g[V0][0];
                                                                c.lq(
                                                                    16,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x140),
                                                                );
                                                                if t {
                                                                    break 'b_18ff38;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t = true;
                                                                    c.lq(
                                                                        17,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x130),
                                                                    );
                                                                    if t {
                                                                        break 'b_1902a0;
                                                                    } else {
                                                                        break 'b_18ff30;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = c.g[A2][0] != c.g[A1][0];
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x140),
                                                        );
                                                        if t {
                                                            break 'b_19029c;
                                                        } else {
                                                            break 'b_18ff38;
                                                        }
                                                    }
                                                    c.cycles += 4;
                                                    c.g[S2][0] =
                                                        c.lw(
                                                            (c.g[S3][0] as u32).wrapping_add(0x1c0)
                                                        );
                                                    c.g[V0][0] = 0x1;
                                                    let t = c.g[S2][0] != c.g[V0][0];
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x140),
                                                    );
                                                    if t {
                                                        break 'b_19029c;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x224));
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.lq(
                                                            17,
                                                            (c.g[SP][0] as u32).wrapping_add(0x130),
                                                        );
                                                        if t {
                                                            break 'b_1902a0;
                                                        } else {
                                                            'b_1900a8: {
                                                                c.cycles += 3;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S3][0] as u32)
                                                                        .wrapping_add(0x1a4));
                                                                let t = c.g[V0][0] != 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x848));
                                                                if t {
                                                                    break 'b_1900a8;
                                                                } else {
                                                                    c.cycles += 82;
                                                                    c.g[A1][0] = 0x460000;
                                                                    c.g[A0][0] = 0x444;
                                                                    c.g[A1][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x1d38));
                                                                    c.g[A2][0] = 0x100;
                                                                    c.g[A3][0] = 0u64;
                                                                    call_ptr(
                                                                        c, 0x00317d70, 0x0018ff78,
                                                                    )?;
                                                                    c.g[T0][0] = c.lbu(
                                                                        (c.g[S3][0] as u32)
                                                                            .wrapping_add(0x15),
                                                                    );
                                                                    c.g[A2][0] = c.g[S3][0];
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x10));
                                                                    c.g[A3][0] = 0u64;
                                                                    c.g[A0][0] = c.g[V0][0];
                                                                    call_ptr(
                                                                        c, 0x001cb030, 0x0018ff90,
                                                                    )?;
                                                                    c.g[V1][0] = 0x37;
                                                                    c.sw(
                                                                        (c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4),
                                                                        c.g[V0][0],
                                                                    );
                                                                    c.sw(
                                                                        (c.g[V0][0] as u32)
                                                                            .wrapping_add(0x140),
                                                                        c.g[V1][0],
                                                                    );
                                                                    c.g[A1][0] = 0x8;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    call_ptr(
                                                                        c, 0x001cd088, 0x0018ffa8,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A2][0] = 0x1;
                                                                    c.g[A3][0] = 0x2;
                                                                    c.g[A1][0] = 0x4b;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x0018ffbc,
                                                                    )?;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x1;
                                                                    c.sw(
                                                                        (c.g[V0][0] as u32)
                                                                            .wrapping_add(0x58),
                                                                        c.g[S2][0],
                                                                    );
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    call_ptr(
                                                                        c, 0x001ce408, 0x0018ffd0,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0xe;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x0018ffe4,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x27;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x0018fff8,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x28;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x0019000c,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x34;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x00190020,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x1a;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x00190034,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x19;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x00190048,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x35;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x0019005c,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x40;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x00190070,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x41;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x00190084,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x42;
                                                                    c.g[A2][0] = 0u64;
                                                                    c.g[A3][0] = 0x2;
                                                                    call_ptr(
                                                                        c, 0x001ce3c8, 0x00190098,
                                                                    )?;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[S3][0] as u32)
                                                                            .wrapping_add(0x1a4));
                                                                    c.g[A1][0] = 0x1;
                                                                    call_ptr(
                                                                        c, 0x001cd0b0, 0x001900a4,
                                                                    )?;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_sub(0x848));
                                                                    break 'b_1900a8;
                                                                }
                                                            }
                                                            c.cycles += 35;
                                                            c.g[A0][0] = 0x460000;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x1d28));
                                                            c.g[S0][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x8c));
                                                            c.g[S1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.lh((c.g[S1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            call_ptr(c, 0x00317670, 0x001900c4)?;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x24));
                                                            c.g[A0][0] = c.g[S0][0];
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, t, 0x001900d4)?;
                                                            c.g[A0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x001ccf70, 0x001900e0)?;
                                                            c.g[V0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                            c.sw(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x54),
                                                                c.g[S2][0],
                                                            );
                                                            call_ptr(c, 0x0014dc80, 0x001900ec)?;
                                                            c.g[A0][0] = c.g[V0][0];
                                                            c.g[A1][0] = 0x1;
                                                            call_ptr(c, 0x0014dd58, 0x001900f8)?;
                                                            c.g[A1][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1cc));
                                                            c.g[A0][0] = c.g[V0][0];
                                                            call_ptr(c, 0x00147170, 0x00190104)?;
                                                            c.g[A0][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x001cb418, 0x00190110)?;
                                                            c.g[V1][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[A1][0] = c.lw((c.g[S3][0] as u32)
                                                                .wrapping_add(0x1a4));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xd0));
                                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            call_ptr(c, 0x0039f290, 0x00190128)?;
                                                            c.sw(
                                                                (c.g[S3][0] as u32)
                                                                    .wrapping_add(0x1a0),
                                                                c.g[S2][0],
                                                            );
                                                            let t = true;
                                                            c.sw(
                                                                (c.g[S3][0] as u32)
                                                                    .wrapping_add(0x19c),
                                                                0u64,
                                                            );
                                                            if t {
                                                                break 'b_190298;
                                                            } else {
                                                                break 'b_190134;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x224));
                                            let t = c.g[V0][0] == 0u64;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                            if t {
                                                break 'b_19029c;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A1][0] = 0x1;
                                                call_ptr(c, 0x0023c8f0, 0x00190148)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x160));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x164));
                                                let t = c.g[V1][0] as u32;
                                                c.g[A0][0] = sx((c.g[S3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x0019015c)?;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                if t {
                                                    break 'b_19029c;
                                                } else {
                                                    break 'b_190164;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 7;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            call_ptr(c, 0x00227f80, 0x0019016c)?;
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A0][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                            let t = c.g[V0][0] == c.g[A0][0];
                            c.g[V0][0] = 0x6;
                            if t {
                                break 'b_190298;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_19029c;
                                } else {
                                    'b_1901c4: {
                                        'b_1901ac: {
                                            'b_1901a8: {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == 0u64;
                                                c.sw(
                                                    (c.g[S3][0] as u32).wrapping_add(0x1e4),
                                                    c.g[A0][0],
                                                );
                                                if t {
                                                    break 'b_1901a8;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x2;
                                                    if c.g[V1][0] == c.g[V0][0] {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x748));
                                                        break 'b_1901ac;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] != c.g[A0][0];
                                                        c.g[V0][0] = 0x6;
                                                        if t {
                                                            break 'b_1901c4;
                                                        } else {
                                                            break 'b_1901a8;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x748));
                                            break 'b_1901ac;
                                        }
                                        c.cycles += 6;
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001901c0)?;
                                        c.g[V0][0] = 0x6;
                                        break 'b_1901c4;
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                                    if t {
                                        break 'b_190298;
                                    } else {
                                        break 'b_1901cc;
                                    }
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1a8));
                        c.g[S2][0] = 0x1;
                        let t = c.g[V0][0] == c.g[S2][0];
                        c.g[V0][0] = 0x6;
                        if t {
                            break 'b_190298;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1b08));
                            if t {
                                break 'b_190298;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0x2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                if t {
                                    break 'b_19029c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x220));
                                    if c.g[V0][0] == 0u64 {
                                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
                                        break 'b_1902a0;
                                    } else {
                                        c.cycles += 12;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        call_ptr(c, 0x00227f80, 0x00190208)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                                        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x428));
                                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x00190228)?;
                                        if c.g[V0][0] == 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                            break 'b_19029c;
                                        } else {
                                            c.cycles += 6;
                                            c.g[V0][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                                            c.sw(
                                                (c.g[S3][0] as u32).wrapping_add(0x1c4),
                                                c.g[V0][0],
                                            );
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0xf8),
                                                c.g[V0][0],
                                            );
                                            c.g[S1][0] =
                                                c.lw((c.g[S3][0] as u32).wrapping_add(0x1c0));
                                            if c.g[S1][0] != c.g[S2][0] {
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                                                break 'b_19029c;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                                                c.g[A1][0] =
                                                    c.lw((c.g[S3][0] as u32).wrapping_add(0x1c4));
                                                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                                let t = c.g[V0][0] as u32;
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00190264)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x4;
                                                if t {
                                                    break 'b_190298;
                                                } else {
                                                    c.cycles += 11;
                                                    c.sw(
                                                        (c.g[S3][0] as u32).wrapping_add(0x1a8),
                                                        c.g[S1][0],
                                                    );
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.sw(
                                                        (c.g[S3][0] as u32).wrapping_add(0x1c0),
                                                        c.g[V0][0],
                                                    );
                                                    c.g[A1][0] = 0u64;
                                                    call_ptr(c, 0x0023d570, 0x00190280)?;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x748)
                                                        );
                                                    c.g[A1][0] = 0x18;
                                                    c.g[A0][0] =
                                                        c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                                    c.g[V1][0] =
                                                        c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x00190298)?;
                                                    break 'b_190298;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
                break 'b_19029c;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x130));
            break 'b_1902a0;
        }
        c.cycles += 7;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
        c.pc = t;
        return Ok(());
    });
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0018_fd90,
    cfe_state_profile_load_on_widget_action,
    "cFEStateProfileLoad_OnWidgetAction",
)];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, put_words, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    // Layout of the fixture (all below DATA + 0x100000, the fakes' log).
    const OC844_THIS: u32 = DATA + 0x1_0000;
    const OC844_THIS_VT: u32 = DATA + 0x2_0000;
    const OC844_ITEM: u32 = DATA + 0x3_0000;
    const OC844_MP: u32 = DATA + 0x4_0000;
    const OC844_MP_VT: u32 = DATA + 0x5_0000;
    const OC844_MEM: u32 = DATA + 0x6_0000;
    const OC844_MEM_VT: u32 = DATA + 0x7_0000;
    const OC844_WORLD: u32 = DATA + 0x8_0000;
    const OC844_LIST: u32 = DATA + 0x8_1000;
    const OC844_ENTRY: u32 = DATA + 0x8_2000;
    const OC844_PLAYER: u32 = DATA + 0x9_0000;
    const OC844_PLAYER_VT: u32 = DATA + 0x9_1000;
    const OC844_KB: u32 = DATA + 0xa_0000;
    const OC844_KB_LIST: u32 = DATA + 0xa_1000;
    const OC844_SCREEN: u32 = DATA + 0xb_0000;
    const OC844_STATE: u32 = DATA + 0xb_1000;
    const OC844_ENGINE: u32 = DATA + 0xb_2000;
    /// Jump table of the event switch (event 5 at index 0).
    const OC844_TABLE: u32 = 0x0045_e2f0;
    /// A real leaf (`jr ra` returning 0) for vtable slots that must return 0.
    const OC844_ZERO_LEAF: u32 = 0x0014_06c0;
    /// `cFEStateProfileLoad_mScreenAction`.
    const OC844_SCREEN_ACTION: u32 = GAME_GP - 0x1b08;

    const OC844_CTRL: u32 = DATA + 0x20_0000;
    const OC844_MAGIC: u32 = 0x0844_c0de;

    fn oc844_mail(idx: u32) -> u32 {
        OC844_CTRL + 4 + 4 * idx
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox (0 = the fake's pointer) unless it unwound.
    fn oc844_fake_call(c: &mut Cpu, addr: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc844_mail(idx));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc844_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC844_CTRL) != OC844_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc844_fake_call(c, $addr, $idx);
                }
            )*
            fn oc844_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc844_fakes!(
        (
            oc844_fake_mc_manager,
            OC844_PREV_MC_MANAGER,
            ee::s05::OVERRIDE_00227F80,
            ee::s05::f_00227f80__generated,
            0x0022_7f80,
            0
        ),
        (
            oc844_fake_get_be,
            OC844_PREV_GET_BE,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            1
        ),
        (
            oc844_fake_get_interface,
            OC844_PREV_GET_INTERFACE,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            2
        ),
        (
            oc844_fake_strlen,
            OC844_PREV_STRLEN,
            ee::s15::OVERRIDE_00416810,
            ee::s15::f_00416810__generated,
            0x0041_6810,
            3
        ),
        (
            oc844_fake_set_player_name,
            OC844_PREV_SET_NAME,
            ee::s01::OVERRIDE_00147138,
            ee::s01::f_00147138__generated,
            0x0014_7138,
            4
        ),
        (
            oc844_fake_strcpy_a,
            OC844_PREV_STRCPY_A,
            ee::s08::OVERRIDE_002C2540,
            ee::s08::f_002c2540__generated,
            0x002c_2540,
            5
        ),
        (
            oc844_fake_strcpy_b,
            OC844_PREV_STRCPY_B,
            ee::s08::OVERRIDE_002C2580,
            ee::s08::f_002c2580__generated,
            0x002c_2580,
            6
        ),
        (
            oc844_fake_str_hash,
            OC844_PREV_STR_HASH,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            7
        ),
        (
            oc844_fake_sprintf,
            OC844_PREV_SPRINTF,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828,
            8
        ),
        (
            oc844_fake_set_max_chars,
            OC844_PREV_MAX_CHARS,
            ee::s04::OVERRIDE_001CD088,
            ee::s04::f_001cd088__generated,
            0x001c_d088,
            9
        ),
        (
            oc844_fake_set_include_key,
            OC844_PREV_INCLUDE_KEY,
            ee::s04::OVERRIDE_001CE3C8,
            ee::s04::f_001ce3c8__generated,
            0x001c_e3c8,
            10
        ),
        (
            oc844_fake_set_disable_shift,
            OC844_PREV_DISABLE_SHIFT,
            ee::s04::OVERRIDE_001CE408,
            ee::s04::f_001ce408__generated,
            0x001c_e408,
            11
        ),
        (
            oc844_fake_set_static,
            OC844_PREV_SET_STATIC,
            ee::s04::OVERRIDE_001CCF70,
            ee::s04::f_001ccf70__generated,
            0x001c_cf70,
            12
        ),
        (
            oc844_fake_get_player_name,
            OC844_PREV_GET_NAME,
            ee::s01::OVERRIDE_00147170,
            ee::s01::f_00147170__generated,
            0x0014_7170,
            13
        ),
        (
            oc844_fake_set_input_text,
            OC844_PREV_INPUT_TEXT,
            ee::s04::OVERRIDE_001CB418,
            ee::s04::f_001cb418__generated,
            0x001c_b418,
            14
        ),
        (
            oc844_fake_set_min_chars,
            OC844_PREV_MIN_CHARS,
            ee::s04::OVERRIDE_001CD0B0,
            ee::s04::f_001cd0b0__generated,
            0x001c_d0b0,
            15
        ),
        (
            oc844_fake_read_set_file_type,
            OC844_PREV_READ_TYPE,
            ee::s06::OVERRIDE_0023C8F0,
            ee::s06::f_0023c8f0__generated,
            0x0023_c8f0,
            16
        ),
        (
            oc844_fake_set_action,
            OC844_PREV_SET_ACTION,
            ee::s06::OVERRIDE_0023D570,
            ee::s06::f_0023d570__generated,
            0x0023_d570,
            17
        ),
    );

    /// A vtable method slot (`lh off+0 -> this adjust`, `lw off+4 -> function`)
    /// at `vt + off`: the fake, or a real leaf that returns 0.
    fn oc844_slot(c: &mut Cpu, vt: u32, off: u32, adjust: u32, zero: bool) {
        c.bus.write16(vt + off, adjust as u16);
        c.bus
            .write32(vt + off + 4, if zero { OC844_ZERO_LEAF } else { FAKE });
    }

    /// `cFEStateProfileLoad_OnWidgetAction`: every event (5..=14, and ones
    /// outside the table), item present/absent, operation running or not,
    /// screen status, screen actions 0..=4, states 0..=6, the keyboard
    /// popup / confirm-load / back / delete paths, empty and non-empty typed
    /// names, callee results of 0, the unwinds of its calls and a jump-table
    /// target outside the function (the jump-out arm).
    ///
    /// Not reached: the first-use keyboard construction (`this->mpKeyboard ==
    /// 0` calls the real `cKeyboardPopup` constructor, which has no override
    /// slot to fake and runs away on fixture data), and the jump-table arm
    /// that restarts the function (it re-enters the prologue forever).
    #[test]
    fn cfe_state_profile_load_on_widget_action_matches() {
        oc844_install_fakes();
        const EVENTS: [u32; 14] = [5, 5, 5, 6, 7, 7, 7, 8, 9, 13, 13, 14, 15, 3];
        check(
            ee::s02::f_0018fd90__generated,
            cfe_state_profile_load_on_widget_action,
            2400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0844, 36);
                c.pc = 0x0018_fd90;
                let pick = |r: &mut Rng, list: &[u32]| list[(r.next_u32() as usize) % list.len()];
                c.bus.write32(OC844_CTRL, OC844_MAGIC);
                let event = EVENTS[(seed as usize) % EVENTS.len()];
                // Jump table: event 5 -> 0x18fecc, 6 -> 0x190164, 7 -> 0x1901cc,
                // 13 -> 0x18fde0, the rest -> the epilogue.
                let mut table = [0x0019_0298u32; 10];
                table[0] = 0x0018_fecc;
                table[1] = 0x0019_0164;
                table[2] = 0x0019_01cc;
                table[8] = 0x0018_fde0;
                if seed % 41 == 0 {
                    table[(event.wrapping_sub(5) % 10) as usize] = 0x0019_0000;
                }
                put_words(c, OC844_TABLE, &table);
                // Callee results.
                c.bus.write32(oc844_mail(0), OC844_MP + 3);
                c.bus.write32(oc844_mail(2), OC844_PLAYER + 3);
                c.bus
                    .write32(oc844_mail(3), if r.next_u32() % 3 == 0 { 1 } else { 8 });
                // The screen object.
                let this = OC844_THIS;
                c.bus.write32(this + 0x8, OC844_THIS_VT);
                c.bus.write32(this + 0x10, DATA + 0xc_0000);
                c.bus.write8(this + 0x15, r.next_u32() as u8);
                c.bus.write32(this + 0x40, OC844_SCREEN);
                c.bus.write32(OC844_SCREEN + 0xd0, OC844_STATE);
                c.bus.write32(OC844_STATE + 0x10, OC844_ENGINE);
                c.bus.write32(this + 0x1a4, OC844_KB);
                c.bus.write32(OC844_KB + 0x20, OC844_KB_LIST);
                c.bus.write32(OC844_KB + 0x1c, r.next_u32());
                c.bus.write32(OC844_KB_LIST + 0x1c, r.next_u32());
                let processing = pick(&mut r, &[0, 0, 0, 1, 2]);
                c.bus.write32(this + 0x1a8, processing);
                c.bus
                    .write32(this + 0x1c0, pick(&mut r, &[1, 1, 1, 0, 2, 3, 4, 5, 6]));
                c.bus.write32(this + 0x1cc, pick(&mut r, &[0, 1, 2, 3]));
                c.bus.write32(this + 0x214, pick(&mut r, &[3, 3, 3, 3, 2]));
                c.bus.write32(this + 0x220, pick(&mut r, &[1, 1, 0]));
                // 0x224 (main button) must be 0 for event 13: the real
                // `BeginCardOp` it calls has deeper paths otherwise.
                let main_button = if event == 13 {
                    0
                } else {
                    pick(&mut r, &[1, 1, 1, 0])
                };
                c.bus.write32(this + 0x224, main_button);
                c.bus.write32(this + 0x1c4, r.next_u32());
                c.bus.write32(this + 0x1e4, r.next_u32());
                c.bus.write32(this + 0x19c, r.next_u32());
                c.bus.write32(this + 0x1a0, r.next_u32());
                c.bus
                    .write32(OC844_SCREEN_ACTION, pick(&mut r, &[1, 1, 3, 3, 2, 2, 0, 4]));
                // The item (null for some seeds).
                c.bus.write32(OC844_ITEM + 0x18, r.next_u32());
                // Memory card manager and its objects.
                c.bus.write32(OC844_MP + 0x434, OC844_MEM);
                c.bus.write32(OC844_MP + 0x428, r.next_u32());
                c.bus.write32(OC844_MP + 0x748, OC844_MP_VT);
                c.bus.write32(OC844_MEM, OC844_MEM_VT);
                c.bus.write32(OC844_MP + 0xf8, r.next_u32());
                let z = |r: &mut Rng| r.next_u32() % 5 == 0;
                let adj = |r: &mut Rng| (r.next_u32() & 0x3c) + 4;
                let a = adj(&mut r);
                oc844_slot(c, OC844_THIS_VT, 0x160, a, false);
                let a = adj(&mut r);
                oc844_slot(c, OC844_MP_VT, 0x8, a, false);
                let (a, zero) = (adj(&mut r), z(&mut r));
                oc844_slot(c, OC844_MEM_VT, 0x1b0, a, zero);
                let (a, zero) = (adj(&mut r), z(&mut r));
                oc844_slot(c, OC844_MEM_VT, 0x218, a, zero);
                let a = adj(&mut r);
                oc844_slot(c, OC844_PLAYER_VT, 0x8, a, false);
                c.bus.write32(OC844_PLAYER + 0xc, OC844_PLAYER_VT);
                // World -> list -> entry (a vtable call for the "Player N" text).
                c.bus.write32(GAME_GP - 0x848, OC844_WORLD);
                c.bus.write32(OC844_WORLD + 0x8c, OC844_LIST);
                c.bus.write32(OC844_LIST + 4, OC844_ENTRY);
                c.bus.write16(OC844_ENTRY + 0x20, 0x10);
                c.bus.write32(OC844_ENTRY + 0x24, FAKE);
                // Arguments.
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 8 == 7 { 0 } else { sx(OC844_ITEM) };
                c.g[A2][0] = sx(event);
            },
        );
    }
}
