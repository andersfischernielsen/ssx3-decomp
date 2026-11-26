//! Staging module oc800 (wave wave247): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc800`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0017_a978,
        debug_menu_check_world_files,
        "DebugMenu_CheckWorldFiles",
    ),
    (
        0x0018_17e8,
        char_select_on_widget_action,
        "cFEStateCharSelect_OnWidgetAction",
    ),
    (
        0x0019_13b8,
        rule_select_fill_rule_list,
        "cFEStateRuleSelect_FillRuleList",
    ),
    (
        0x0019_4ea8,
        main_menu_on_widget_action,
        "cFEStateMainMenu_OnWidgetAction",
    ),
];

/// `int DebugMenu_CheckWorldFiles(DebugMenu* self, int n, WorldSection* names, char* tag)`
/// (0x0017a978, `cTrackFileMan_checkLocation1`): whether every one of the `n`
/// section names is already registered in the menu's location table (the
/// file/big-file name pair, compared with `strcasecmp`); if one is missing it
/// builds the world's big-file path from `tag`, opens it, asks `isValidMap`
/// (0x3a6948) which sections it holds, and clears the `bFound` flag of the
/// entries the table already knows. Returns the `isValidMap` result (0 if all
/// were known or none found). Converted from the resumable generated body.
pub fn debug_menu_check_world_files(c: &mut Cpu) {
    if c.pc != 0x0017a978 {
        return ee::s02::f_0017a978__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0017a978;
        loop {
            match pc {
                0x0017a978 => {
                    c.cycles += 18;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.g[S6][0] = c.g[A3][0];
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[S7][0] = c.g[A2][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[FP][0] = c.g[A1][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
                    let t = (c.g[FP][0] as i64) <= 0;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
                    if t {
                        pc = 0x0017aa40;
                        continue;
                    }
                    pc = 0x0017a9c0;
                    continue;
                }
                0x0017a9c0 => {
                    c.cycles += 2;
                    c.g[S2][0] = c.g[S7][0];
                    pc = 0x0017a9c8;
                    continue;
                }
                0x0017a9c8 => {
                    c.cycles += 5;
                    c.g[S5][0] = 0u64;
                    c.g[S3][0] = 0u64;
                    c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x44));
                    let t = true;
                    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x4));
                    if t {
                        pc = 0x0017a9e8;
                        continue;
                    }
                    pc = 0x0017a9dc;
                    continue;
                }
                0x0017a9dc => {
                    c.cycles += 1;
                    pc = 0x0017a9e0;
                    continue;
                }
                0x0017a9e0 => {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                    pc = 0x0017a9e8;
                    continue;
                }
                0x0017a9e8 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[S4][0] as u32);
                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        pc = 0x0017aa24;
                        continue;
                    }
                    pc = 0x0017a9f8;
                    continue;
                }
                0x0017a9f8 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    c.g[RA][0] = sx(0x0017aa04);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x0017aa04)?;

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8c));
                        if back_edge_unwinds(c, 0x0017a9e0) {
                            return Ok(());
                        }
                        pc = 0x0017a9e0;
                        continue;
                    }
                    pc = 0x0017aa0c;
                    continue;
                }
                0x0017aa0c => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0017aa18);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x0017aa18)?;

                    if c.g[V0][0] != 0u64 {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8c));
                        if back_edge_unwinds(c, 0x0017a9e0) {
                            return Ok(());
                        }
                        pc = 0x0017a9e0;
                        continue;
                    }
                    pc = 0x0017aa20;
                    continue;
                }
                0x0017aa20 => {
                    c.cycles += 1;
                    c.g[S5][0] = 0x1;
                    pc = 0x0017aa24;
                    continue;
                }
                0x0017aa24 => {
                    c.cycles += 2;
                    let t = c.g[S5][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
                    if t {
                        pc = 0x0017aa54;
                        continue;
                    }
                    pc = 0x0017aa2c;
                    continue;
                }
                0x0017aa2c => {
                    c.cycles += 5;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[FP][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                    if t {
                        if back_edge_unwinds(c, 0x0017a9c8) {
                            return Ok(());
                        }
                        pc = 0x0017a9c8;
                        continue;
                    }
                    pc = 0x0017aa40;
                    continue;
                }
                0x0017aa40 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                    let t = c.g[V1][0] != 0u64;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x0017aa60;
                        continue;
                    }
                    pc = 0x0017aa4c;
                    continue;
                }
                0x0017aa4c => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0017ab98;
                        continue;
                    }
                    pc = 0x0017aa54;
                    continue;
                }
                0x0017aa54 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1;
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    if t {
                        if back_edge_unwinds(c, 0x0017aa40) {
                            return Ok(());
                        }
                        pc = 0x0017aa40;
                        continue;
                    }
                    pc = 0x0017aa60;
                    continue;
                }
                0x0017aa60 => {
                    c.cycles += 31;
                    c.g[S0][0] = 0x460000;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x31e0));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0017aa74);
                    c.g[A1][0] = c.g[S6][0];
                    call_ptr(c, 0x003182d8, 0x0017aa74)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0017aa84);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003186d0, 0x0017aa84)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0017aa90);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0017aa90)?;
                    c.g[A2][0] = 0x4a0000;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1298));
                    c.g[RA][0] = sx(0x0017aaa8);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00318630, 0x0017aaa8)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A2][0] = 0x100;
                    c.g[RA][0] = sx(0x0017aab8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003168b0, 0x0017aab8)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0017aac4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0017aac4)?;
                    c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A1][0] = c.g[FP][0];
                    c.g[RA][0] = sx(0x0017aad4);
                    c.g[A2][0] = c.g[S7][0];
                    call_ptr(c, 0x003a6948, 0x0017aad4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x38), c.g[V0][0]);
                    if t {
                        pc = 0x0017aafc;
                        continue;
                    }
                    pc = 0x0017aadc;
                    continue;
                }
                0x0017aadc => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x0017aae8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x003168e8, 0x0017aae8)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0017aaf4);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0017aaf4)?;
                    let t = true;
                    c.g[V0][0] = 0u64;
                    if t {
                        pc = 0x0017ab98;
                        continue;
                    }
                    pc = 0x0017aafc;
                    continue;
                }
                0x0017aafc => {
                    c.cycles += 2;
                    let t = (c.g[FP][0] as i64) <= 0;
                    c.g[A0][0] = 0u64;
                    if t {
                        pc = 0x0017ab7c;
                        continue;
                    }
                    pc = 0x0017ab04;
                    continue;
                }
                0x0017ab04 => {
                    c.cycles += 1;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 3);
                    pc = 0x0017ab08;
                    continue;
                }
                0x0017ab08 => {
                    c.cycles += 5;
                    c.g[S3][0] = c.g[V1][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S7][0] as u32));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S5][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0017ab6c;
                        continue;
                    }
                    pc = 0x0017ab1c;
                    continue;
                }
                0x0017ab1c => {
                    c.cycles += 1;
                    c.g[S2][0] = 0u64;
                    pc = 0x0017ab20;
                    continue;
                }
                0x0017ab20 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(c.g[S4][0] as u32);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = 0x8c;
                    if t {
                        pc = 0x0017ab6c;
                        continue;
                    }
                    pc = 0x0017ab30;
                    continue;
                }
                0x0017ab30 => {
                    c.cycles += 9;
                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S7][0] as u32));
                    c.mult(0, 2, 18, 3);
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x0017ab4c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x0017ab4c)?;

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        if back_edge_unwinds(c, 0x0017ab20) {
                            return Ok(());
                        }
                        pc = 0x0017ab20;
                        continue;
                    }
                    pc = 0x0017ab54;
                    continue;
                }
                0x0017ab54 => {
                    c.cycles += 5;
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x0017ab60);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x0041aa88, 0x0017ab60)?;

                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        if back_edge_unwinds(c, 0x0017ab20) {
                            return Ok(());
                        }
                        pc = 0x0017ab20;
                        continue;
                    }
                    pc = 0x0017ab68;
                    continue;
                }
                0x0017ab68 => {
                    c.cycles += 1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                    pc = 0x0017ab6c;
                    continue;
                }
                0x0017ab6c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[FP][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 3);
                    if t {
                        if back_edge_unwinds(c, 0x0017ab08) {
                            return Ok(());
                        }
                        pc = 0x0017ab08;
                        continue;
                    }
                    pc = 0x0017ab7c;
                    continue;
                }
                0x0017ab7c => {
                    c.cycles += 7;
                    c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x0017ab88);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x003168e8, 0x0017ab88)?;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0017ab94);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x00318260, 0x0017ab94)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x38));
                    pc = 0x0017ab98;
                    continue;
                }
                0x0017ab98 => {
                    c.cycles += 12;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x90));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x80));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0017a978, 0x0017abc8);
                    return Ok(());
                }
            }
        }
    });
}

/// `void cFEStateCharSelect_OnWidgetAction(self, Widget* w, int event)`
/// (0x001817e8): the character-select widget events (`event - 1` through a
/// jump table at 0x45d6c0): 1 show widget and update the attribute bars and
/// the rider's character, 2 hide it, 5 select (pick the character, set the
/// pad mask, push the next state, hide the preview), 6 back, 7 push the
/// options screen, 8 push the profile-load screen, 3/4 nothing.
pub fn char_select_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x001817e8 {
        return ee::s02::f_001817e8__generated(c);
    }
    guest(c, |c| 'l_1817e8: loop {
        'b_181ba8: {
            'b_181ba4: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S6][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                c.g[S7][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
                let t = c.g[S7][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_181ba4;
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        break 'b_181ba4;
                    } else {
                        'b_181b84: {
                            'b_181ae8: {
                                'b_181a50: {
                                    'b_181a30: {
                                        'b_181980: {
                                            'b_18193c: {
                                                c.cycles += 6;
                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x2940));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x001817e8 => {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001817e8);
                                                        }
                                                        continue 'l_1817e8;
                                                    }
                                                    0x00181844 => {
                                                        c.cycles += 56;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                        c.g[S3][0] = 0x1;
                                                        c.g[A1][0] = c
                                                            .lb((c.g[S6][0] as u32)
                                                                .wrapping_add(0x44));
                                                        c.g[A0][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x7c));
                                                        c.g[RA][0] = sx(0x0018185c);
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0xb0));
                                                        call_ptr(c, 0x001a0548, 0x0018185c)?;
                                                        c.sw(
                                                            (c.g[V0][0] as u32).wrapping_add(0xc20),
                                                            c.g[S3][0],
                                                        );
                                                        c.g[V1][0] = 0x440000;
                                                        c.sw(
                                                            (c.g[V0][0] as u32).wrapping_add(0xc1c),
                                                            0u64,
                                                        );
                                                        c.g[V1][0] =
                                                            sx((c.g[V1][0] as u32)
                                                                .wrapping_add(0xf68));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S7][0] as u32)
                                                                .wrapping_add(0x18));
                                                        c.g[S1][0] = c
                                                            .lb((c.g[S6][0] as u32)
                                                                .wrapping_add(0x44));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(c.g[V1][0] as u32));
                                                        c.g[S2][0] = c
                                                            .lb((c.g[S6][0] as u32)
                                                                .wrapping_add(0x14));
                                                        c.g[S4][0] = c.lb(c.g[V0][0] as u32);
                                                        c.g[RA][0] = sx(0x00181888);
                                                        c.g[S5][0] = c
                                                            .lw((c.g[S6][0] as u32)
                                                                .wrapping_add(0x4c));
                                                        call_ptr(c, 0x0014dc80, 0x00181888)?;
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x00181894);
                                                        c.g[A1][0] = 0x1;
                                                        call_ptr(c, 0x0014dd58, 0x00181894)?;
                                                        c.g[S0][0] = c.g[V0][0];
                                                        c.g[A2][0] = c.g[S2][0];
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x001818a8);
                                                        c.g[A1][0] = c.g[S1][0];
                                                        call_ptr(c, 0x001472c8, 0x001818a8)?;
                                                        c.g[A2][0] = c.g[S4][0];
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x001818b8);
                                                        c.g[A1][0] = c.g[S1][0];
                                                        call_ptr(c, 0x00147448, 0x001818b8)?;
                                                        c.g[A1][0] = c.g[S1][0];
                                                        c.g[A2][0] = c.g[S5][0];
                                                        c.g[RA][0] = sx(0x001818c8);
                                                        c.g[A0][0] = c.g[S0][0];
                                                        call_ptr(c, 0x001474a8, 0x001818c8)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc));
                                                        let t = c.g[V1][0] as u32;
                                                        c.g[RA][0] = sx(0x001818dc);
                                                        c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        call_ptr(c, t, 0x001818dc)?;
                                                        c.g[V0][0] = c
                                                            .lb((c.g[S6][0] as u32)
                                                                .wrapping_add(0x14));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[S6][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[S3][0] = sx((c.g[S3][0] as u32)
                                                            << ((c.g[V0][0] as u32) & 31));
                                                        c.g[A1][0] = c
                                                            .lb((c.g[S6][0] as u32)
                                                                .wrapping_add(0x44));
                                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                        c.g[RA][0] = sx(0x001818f8);
                                                        c.g[A2][0] = c.g[S3][0] & 0xff;
                                                        call_ptr(c, 0x001a1cb8, 0x001818f8)?;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S6][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[A1][0] = c.g[S6][0];
                                                        c.g[A2][0] = c
                                                            .lw((c.g[S7][0] as u32)
                                                                .wrapping_add(0x18));
                                                        c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                                        c.g[V1][0] = c
                                                            .lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x20));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x24));
                                                        let t = c.g[V0][0] as u32;
                                                        c.g[RA][0] = sx(0x0018191c);
                                                        c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        call_ptr(c, t, 0x0018191c)?;
                                                        if c.g[V0][0] == 0u64 {
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                            break 'b_181b84;
                                                        } else {
                                                            c.cycles += 6;
                                                            c.g[A0][0] = c.lw((c.g[S6][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A1][0] = c.g[V0][0];
                                                            c.g[RA][0] = sx(0x00181934);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            call_ptr(c, 0x0039f400, 0x00181934)?;
                                                            let t = true;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                            if t {
                                                                break 'b_181b84;
                                                            } else {
                                                                break 'b_18193c;
                                                            }
                                                        }
                                                    }
                                                    0x0018193c => {
                                                        break 'b_18193c;
                                                    }
                                                    0x00181980 => {
                                                        break 'b_181980;
                                                    }
                                                    0x00181a30 => {
                                                        break 'b_181a30;
                                                    }
                                                    0x00181a50 => {
                                                        break 'b_181a50;
                                                    }
                                                    0x00181ae8 => {
                                                        break 'b_181ae8;
                                                    }
                                                    0x00181ba4 => {
                                                        break 'b_181ba4;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x001817e8, 0x00181bd0);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 11;
                                            c.g[V0][0] =
                                                c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                                            c.g[A1][0] = c.g[S6][0];
                                            c.g[A2][0] =
                                                c.lw((c.g[S7][0] as u32).wrapping_add(0x18));
                                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[V1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x00181960);
                                            c.g[A0][0] =
                                                sx((c.g[A3][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x00181960)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                            if t {
                                                break 'b_181ba8;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x00181978);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                                call_ptr(c, 0x0039f400, 0x00181978)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                if t {
                                                    break 'b_181b84;
                                                } else {
                                                    break 'b_181980;
                                                }
                                            }
                                        }
                                        c.cycles += 44;
                                        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0x1;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00181998);
                                        c.g[A0][0] =
                                            sx((c.g[S7][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x00181998)?;
                                        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x18));
                                        c.g[S0][0] = 0x440000;
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xf68));
                                        c.g[A1][0] = c.lb((c.g[S6][0] as u32).wrapping_add(0x44));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                        c.g[A0][0] = c.g[S6][0];
                                        c.g[RA][0] = sx(0x001819b8);
                                        c.g[A2][0] = c.lb(c.g[V0][0] as u32);
                                        Callee {
                                            addr: 0x00181bd0,
                                            f: ee::s02::cFEStateCharSelect_UpdateAttributeBars,
                                        }
                                        .call(c, 0x001819b8)?;
                                        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x18));
                                        c.g[S1][0] = c.lb((c.g[S6][0] as u32).wrapping_add(0x44));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                                        c.g[S4][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4c));
                                        c.g[S3][0] = c.lb(c.g[V0][0] as u32);
                                        c.g[RA][0] = sx(0x001819d4);
                                        c.g[S2][0] = c.lb((c.g[S6][0] as u32).wrapping_add(0x14));
                                        call_ptr(c, 0x0014dc80, 0x001819d4)?;
                                        c.g[A0][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001819e0);
                                        c.g[A1][0] = 0x1;
                                        call_ptr(c, 0x0014dd58, 0x001819e0)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.g[S2][0];
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001819f4);
                                        c.g[A1][0] = c.g[S1][0];
                                        call_ptr(c, 0x001472c8, 0x001819f4)?;
                                        c.g[A2][0] = c.g[S3][0];
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x00181a04);
                                        c.g[A1][0] = c.g[S1][0];
                                        call_ptr(c, 0x00147448, 0x00181a04)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[S1][0];
                                        c.g[RA][0] = sx(0x00181a14);
                                        c.g[A2][0] = c.g[S4][0];
                                        call_ptr(c, 0x001474a8, 0x00181a14)?;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00181a28);
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x00181a28)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                        if t {
                                            break 'b_181ba8;
                                        } else {
                                            break 'b_181a30;
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00181a48);
                                    c.g[A0][0] =
                                        sx((c.g[S7][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00181a48)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                                    if t {
                                        break 'b_181ba8;
                                    } else {
                                        break 'b_181a50;
                                    }
                                }
                                c.cycles += 38;
                                c.g[RA][0] = sx(0x00181a58);
                                call_ptr(c, 0x0014dc80, 0x00181a58)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x00181a64);
                                c.g[A1][0] = 0x1;
                                call_ptr(c, 0x0014dd58, 0x00181a64)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lb((c.g[S6][0] as u32).wrapping_add(0x44));
                                c.g[A2][0] = c.lb((c.g[S6][0] as u32).wrapping_add(0x14));
                                c.g[RA][0] = sx(0x00181a78);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x001472c8, 0x00181a78)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00181a8c);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00181a8c)?;
                                c.g[A1][0] = 0x460000;
                                c.g[A3][0] = 0u64;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2968));
                                c.g[A0][0] = 0x54;
                                c.g[RA][0] = sx(0x00181aa4);
                                c.g[A2][0] = 0x100;
                                call_ptr(c, 0x00317d70, 0x00181aa4)?;
                                c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x00181ab4);
                                c.g[A2][0] = 0x1;
                                Callee {
                                    addr: 0x001887a0,
                                    f: ee::s02::cFEStateOptions_ctor,
                                }
                                .call(c, 0x00181ab4)?;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                                c.g[A0][0] = 0xffffffffffffc0ff;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[A2][0] = 0u64;
                                c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                                c.g[A3][0] = 0u64;
                                c.g[V1][0] = c.g[V1][0] | 0x100;
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x00181ae0);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f320, 0x00181ae0)?;
                                let t = true;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_181b84;
                                } else {
                                    break 'b_181ae8;
                                }
                            }
                            'b_181b80: {
                                c.cycles += 37;
                                c.g[RA][0] = sx(0x00181af0);
                                call_ptr(c, 0x0014dc80, 0x00181af0)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x00181afc);
                                c.g[A1][0] = 0x1;
                                call_ptr(c, 0x0014dd58, 0x00181afc)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = c.lb((c.g[S6][0] as u32).wrapping_add(0x44));
                                c.g[A2][0] = c.lb((c.g[S6][0] as u32).wrapping_add(0x14));
                                c.g[RA][0] = sx(0x00181b10);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x001472c8, 0x00181b10)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00181b24);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00181b24)?;
                                c.g[A1][0] = 0x460000;
                                c.g[A3][0] = 0u64;
                                c.g[A0][0] = 0x280;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2958));
                                c.g[RA][0] = sx(0x00181b3c);
                                c.g[A2][0] = 0x100;
                                call_ptr(c, 0x00317d70, 0x00181b3c)?;
                                c.g[V1][0] = c.lb((c.g[S6][0] as u32).wrapping_add(0x14));
                                c.g[A3][0] = 0x1;
                                c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A3][0] = sx((c.g[A3][0] as u32) << ((c.g[V1][0] as u32) & 31));
                                c.g[A2][0] = 0x2;
                                c.g[RA][0] = sx(0x00181b5c);
                                c.g[A3][0] = c.g[A3][0] & 0xff;
                                Callee {
                                    addr: 0x0018eca0,
                                    f: ee::s02::cFEStateProfileLoad_ctor,
                                }
                                .call(c, 0x00181b5c)?;
                                c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x00181b6c);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                call_ptr(c, 0x0039f400, 0x00181b6c)?;
                                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                let t = c.g[V1][0] == 0u64;
                                c.g[V0][0] = 0x9;
                                if t {
                                    break 'b_181b80;
                                } else {
                                    c.cycles += 1;
                                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                                    break 'b_181b80;
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            break 'b_181b84;
                        }
                        c.cycles += 8;
                        c.g[A1][0] = c.lb((c.g[S6][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7c));
                        c.g[RA][0] = sx(0x00181b94);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
                        call_ptr(c, 0x001a0548, 0x00181b94)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00181ba0);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0019e538, 0x00181ba0)?;
                        c.sw((c.g[S6][0] as u32).wrapping_add(0x80), 0u64);
                        break 'b_181ba4;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            break 'b_181ba8;
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = t;
        return Ok(());
    });
}

/// `void cFEStateRuleSelect_FillRuleList(self, signed char first)`
/// (0x001913b8): fills the ten rule rows (listbox `0..9` and label `0t..9t`,
/// then `a`) from the game-rule mapping: label text, list entries by rule kind
/// (jump table at 0x45faf0: off/on, no/yes, percent, seconds, ... ), the
/// current value; then the arrow widgets and the menu frame (normal or
/// greyed out) and the row visibility.
pub fn rule_select_fill_rule_list(c: &mut Cpu) {
    if c.pc != 0x001913b8 {
        return ee::s02::f_001913b8__generated(c);
    }
    guest(c, |c| {
        c.cycles += 34;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
        c.g[V0][0] = 0x4a0000;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[V1][0] = 0x4a0000;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S3][0] = c.g[A0][0];
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S5][0] = 0u64;
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S7][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
        c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x16a8));
        c.g[A2][0] = c.lb(c.g[T1][0] as u32);
        c.g[A3][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x1));
        c.g[T0][0] = c.lb((c.g[T1][0] as u32).wrapping_add(0x2));
        c.sb(c.g[SP][0] as u32, c.g[A2][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A3][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x2), c.g[T0][0]);
        c.g[A3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x16b0));
        c.g[V0][0] = c.lb(c.g[A3][0] as u32);
        c.g[A1][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x1));
        c.g[A2][0] = c.lb((c.g[A3][0] as u32).wrapping_add(0x2));
        c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x11), c.g[A1][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x12), c.g[A2][0]);
        'l_191440: loop {
            'b_1917f0: {
                'b_1917ac: {
                    'b_1917a8: {
                        c.cycles += 13;
                        c.g[RA][0] = sx(0x00191448);
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x00317670, 0x00191448)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x00191454);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x00191454)?;
                        c.g[A0][0] = c.g[S6][0];
                        c.g[RA][0] = sx(0x00191460);
                        c.g[S0][0] = c.g[V0][0];
                        call_ptr(c, 0x00317670, 0x00191460)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x0019146c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x0019146c)?;
                        let t = c.g[S0][0] == 0u64;
                        c.g[S2][0] = c.g[V0][0];
                        if t {
                            break 'b_1917a8;
                        } else {
                            c.cycles += 2;
                            let t = c.g[S2][0] == 0u64;
                            c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S7][0] as u32));
                            if t {
                                break 'b_1917a8;
                            } else {
                                c.cycles += 6;
                                c.g[V1][0] = c.lb((c.g[S3][0] as u32).wrapping_add(0x50));
                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                                c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                let t = c.g[V1][0] == 0u64;
                                c.g[A1][0] = 0x440000;
                                if t {
                                    break 'b_1917f0;
                                } else {
                                    'b_19175c: {
                                        c.cycles += 20;
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x48));
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1018));
                                        c.g[V1][0] = 0x440000;
                                        c.g[A0][0] =
                                            sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1008));
                                        c.g[S4][0] = c.lb(c.g[A0][0] as u32);
                                        c.g[V0][0] = sx((c.g[S4][0] as u32) << 2);
                                        c.g[V1][0] =
                                            sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                        c.g[V0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                        c.g[S1][0] = c.lb(c.g[V1][0] as u32);
                                        c.g[RA][0] = sx(0x001914c4);
                                        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                                        call_ptr(c, 0x00317670, 0x001914c4)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001914d0);
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x003a1000, 0x001914d0)?;
                                        c.g[RA][0] = sx(0x001914d8);
                                        c.g[A0][0] = c.g[S0][0];
                                        call_ptr(c, 0x0039a670, 0x001914d8)?;
                                        c.g[V0][0] = (c.g[S1][0] < 0x7u64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[S4][0]);
                                        if t {
                                            break 'b_19175c;
                                        } else {
                                            'b_191708: {
                                                'b_191688: {
                                                    'b_1915e0: {
                                                        'b_19156c: {
                                                            'b_19154c: {
                                                                'b_191528: {
                                                                    c.cycles += 7;
                                                                    c.g[V0][0] = 0x460000;
                                                                    c.g[V1][0] = sx((c.g[S1][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_sub(0x510));
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V0][0] as u32,
                                                                        ));
                                                                    c.g[A0][0] =
                                                                        c.lw(c.g[V1][0] as u32);
                                                                    let t = c.g[A0][0] as u32;
                                                                    match t {
                                                                        0x00191500 => {
                                                                            c.cycles += 10;
                                                                            c.g[A0][0] = 0x460000;
                                                                            c.g[RA][0] =
                                                                                sx(0x0019150c);
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x20c0,
                                                                                    ));
                                                                            call_ptr(
                                                                                c, 0x00317670,
                                                                                0x0019150c,
                                                                            )?;
                                                                            c.g[A1][0] = c.g[V0][0];
                                                                            c.g[A2][0] = 0u64;
                                                                            c.g[RA][0] =
                                                                                sx(0x0019151c);
                                                                            c.g[A0][0] = c.g[S0][0];
                                                                            call_ptr(
                                                                                c, 0x0039a568,
                                                                                0x0019151c,
                                                                            )?;
                                                                            c.g[A0][0] = 0x460000;
                                                                            let t = true;
                                                                            c.g[A0][0] =
                                                                                sx((c.g[A0][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x1fd8,
                                                                                    ));
                                                                            if t {
                                                                                break 'b_19154c;
                                                                            } else {
                                                                                break 'b_191528;
                                                                            }
                                                                        }
                                                                        0x00191528 => {
                                                                            break 'b_191528;
                                                                        }
                                                                        0x0019156c => {
                                                                            break 'b_19156c;
                                                                        }
                                                                        0x001915e0 => {
                                                                            break 'b_1915e0;
                                                                        }
                                                                        0x00191688 => {
                                                                            break 'b_191688;
                                                                        }
                                                                        0x00191708 => {
                                                                            break 'b_191708;
                                                                        }
                                                                        0x0019175c => {
                                                                            break 'b_19175c;
                                                                        }
                                                                        _ => {
                                                                            c.jump_out(
                                                                                t, 0x001913b8,
                                                                                0x00191938,
                                                                            );
                                                                            return Ok(());
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 9;
                                                                c.g[A0][0] = 0x460000;
                                                                c.g[RA][0] = sx(0x00191534);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x580));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x00191534,
                                                                )?;
                                                                c.g[A1][0] = c.g[V0][0];
                                                                c.g[A2][0] = 0u64;
                                                                c.g[RA][0] = sx(0x00191544);
                                                                c.g[A0][0] = c.g[S0][0];
                                                                call_ptr(
                                                                    c, 0x0039a568, 0x00191544,
                                                                )?;
                                                                c.g[A0][0] = 0x460000;
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x2338));
                                                                break 'b_19154c;
                                                            }
                                                            c.cycles += 8;
                                                            c.g[RA][0] = sx(0x00191554);
                                                            call_ptr(c, 0x00317670, 0x00191554)?;
                                                            c.g[A1][0] = c.g[V0][0];
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x00191564);
                                                            c.g[A2][0] = 0x1;
                                                            call_ptr(c, 0x0039a568, 0x00191564)?;
                                                            let t = true;

                                                            if t {
                                                                break 'b_19175c;
                                                            } else {
                                                                break 'b_19156c;
                                                            }
                                                        }
                                                        c.cycles += 29;
                                                        c.g[A0][0] = 0x460000;
                                                        c.g[RA][0] = sx(0x00191578);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x20c0));
                                                        call_ptr(c, 0x00317670, 0x00191578)?;
                                                        c.g[A1][0] = c.g[V0][0];
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[RA][0] = sx(0x00191588);
                                                        c.g[A2][0] = 0u64;
                                                        call_ptr(c, 0x0039a568, 0x00191588)?;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16b8));
                                                        c.g[RA][0] = sx(0x0019159c);
                                                        c.g[A2][0] = 0x5a;
                                                        call_ptr(c, 0x0039a3f8, 0x0019159c)?;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16c0));
                                                        c.g[RA][0] = sx(0x001915b0);
                                                        c.g[A2][0] = 0x50;
                                                        call_ptr(c, 0x0039a3f8, 0x001915b0)?;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16c8));
                                                        c.g[RA][0] = sx(0x001915c4);
                                                        c.g[A2][0] = 0x46;
                                                        call_ptr(c, 0x0039a3f8, 0x001915c4)?;
                                                        c.g[A1][0] = 0x4a0000;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16d0));
                                                        c.g[RA][0] = sx(0x001915d8);
                                                        c.g[A2][0] = 0x3c;
                                                        call_ptr(c, 0x0039a3f8, 0x001915d8)?;
                                                        let t = true;

                                                        if t {
                                                            break 'b_19175c;
                                                        } else {
                                                            break 'b_1915e0;
                                                        }
                                                    }
                                                    c.cycles += 42;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16d8));
                                                    c.g[RA][0] = sx(0x001915f4);
                                                    c.g[A2][0] = 0xa;
                                                    call_ptr(c, 0x0039a3f8, 0x001915f4)?;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16e0));
                                                    c.g[RA][0] = sx(0x00191608);
                                                    c.g[A2][0] = 0x5;
                                                    call_ptr(c, 0x0039a3f8, 0x00191608)?;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A2][0] = 0x3;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16e8));
                                                    c.g[RA][0] = sx(0x0019161c);
                                                    c.g[A0][0] = c.g[S0][0];
                                                    call_ptr(c, 0x0039a3f8, 0x0019161c)?;
                                                    c.g[A0][0] = 0x460000;
                                                    c.g[RA][0] = sx(0x00191628);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32)
                                                            .wrapping_sub(0x20c0));
                                                    call_ptr(c, 0x00317670, 0x00191628)?;
                                                    c.g[A1][0] = c.g[V0][0];
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x00191638);
                                                    c.g[A2][0] = 0u64;
                                                    call_ptr(c, 0x0039a568, 0x00191638)?;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A2][0] = 0xffffffffffff0000;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16f0));
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x00191650);
                                                    c.g[A2][0] = c.g[A2][0] | 0xfffd;
                                                    call_ptr(c, 0x0039a3f8, 0x00191650)?;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A2][0] = 0xffffffffffff0000;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x16f8));
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x00191668);
                                                    c.g[A2][0] = c.g[A2][0] | 0xfffb;
                                                    call_ptr(c, 0x0039a3f8, 0x00191668)?;
                                                    c.g[A1][0] = 0x4a0000;
                                                    c.g[A2][0] = 0xffffffffffff0000;
                                                    c.g[A1][0] =
                                                        sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x1700));
                                                    c.g[A0][0] = c.g[S0][0];
                                                    c.g[RA][0] = sx(0x00191680);
                                                    c.g[A2][0] = c.g[A2][0] | 0xfff6;
                                                    call_ptr(c, 0x0039a3f8, 0x00191680)?;
                                                    let t = true;

                                                    if t {
                                                        break 'b_19175c;
                                                    } else {
                                                        break 'b_191688;
                                                    }
                                                }
                                                c.cycles += 32;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x13a0));
                                                c.g[RA][0] = sx(0x0019169c);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x0039a3f8, 0x0019169c)?;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x14f8));
                                                c.g[RA][0] = sx(0x001916b0);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x0039a3f8, 0x001916b0)?;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1500));
                                                c.g[RA][0] = sx(0x001916c4);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x0039a3f8, 0x001916c4)?;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1508));
                                                c.g[RA][0] = sx(0x001916d8);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x0039a3f8, 0x001916d8)?;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1510));
                                                c.g[RA][0] = sx(0x001916ec);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x0039a3f8, 0x001916ec)?;
                                                c.g[A1][0] = 0x4a0000;
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x1518));
                                                c.g[RA][0] = sx(0x00191700);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x0039a3f8, 0x00191700)?;
                                                let t = true;

                                                if t {
                                                    break 'b_19175c;
                                                } else {
                                                    break 'b_191708;
                                                }
                                            }
                                            c.cycles += 21;
                                            c.g[A0][0] = 0x460000;
                                            c.g[RA][0] = sx(0x00191714);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x570));
                                            call_ptr(c, 0x00317670, 0x00191714)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x00191724);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x0039a568, 0x00191724)?;
                                            c.g[A0][0] = 0x460000;
                                            c.g[RA][0] = sx(0x00191730);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x558));
                                            call_ptr(c, 0x00317670, 0x00191730)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x00191740);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x0039a568, 0x00191740)?;
                                            c.g[A0][0] = 0x460000;
                                            c.g[RA][0] = sx(0x0019174c);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x540));
                                            call_ptr(c, 0x00317670, 0x0019174c)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0019175c);
                                            c.g[A2][0] = 0u64;
                                            call_ptr(c, 0x0039a568, 0x0019175c)?;
                                            break 'b_19175c;
                                        }
                                    }
                                    'b_191790: {
                                        c.cycles += 2;
                                        let t = c.g[S4][0] == 0u64;
                                        c.g[A1][0] = c.g[S4][0];
                                        if t {
                                            break 'b_191790;
                                        } else {
                                            c.cycles += 11;
                                            c.g[RA][0] = sx(0x0019176c);
                                            c.g[A0][0] = c.g[S3][0];
                                            Callee {
                                                addr: 0x00192240,
                                                f: ee::s02::cFEStateRuleSelect_GetRuleValue,
                                            }
                                            .call(c, 0x0019176c)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x0019177c);
                                            c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                            call_ptr(c, 0x0039a768, 0x0019177c)?;
                                            c.g[A1][0] =
                                                c.lbu((c.g[SP][0] as u32).wrapping_add(0x20));
                                            c.g[RA][0] = sx(0x00191788);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x0039a7a8, 0x00191788)?;
                                            let t = true;
                                            c.g[V0][0] = ((c.g[S5][0] as i64) < 9i64) as u64;
                                            if t {
                                                break 'b_1917ac;
                                            } else {
                                                break 'b_191790;
                                            }
                                        }
                                    }
                                    c.cycles += 6;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001917a8);
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001917a8)?;
                                    break 'b_1917a8;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[S5][0] as i64) < 9i64) as u64;
                    break 'b_1917ac;
                }
                'b_1917d8: {
                    'b_1917cc: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.lbu(c.g[SP][0] as u32);
                        if t {
                            break 'b_1917cc;
                        } else {
                            c.cycles += 6;
                            c.g[V1][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                            let t = true;
                            c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
                            if t {
                                break 'b_1917d8;
                            } else {
                                break 'b_1917cc;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x58));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                    break 'b_1917d8;
                }
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                c.g[S5][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
                c.g[V1][0] = ((c.g[S5][0] as i64) < 11i64) as u64;
                let t = c.g[V1][0] != 0u64;

                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00191440);
                    }
                    continue 'l_191440;
                } else {
                    break 'b_1917f0;
                }
            }
            'b_191814: {
                c.cycles += 9;
                c.g[A0][0] = 0x4a0000;
                c.g[S0][0] = 0xff;
                c.g[RA][0] = sx(0x00191800);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1398));
                call_ptr(c, 0x00317670, 0x00191800)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x0019180c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x0019180c)?;
                if c.g[V0][0] != 0u64 {
                    c.g[S0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                    break 'b_191814;
                } else {
                    break 'b_191814;
                }
            }
            'b_1918d0: {
                'b_1918bc: {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x48));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                    let t = c.g[V1][0] != 0u64;
                    c.g[V0][0] = 0x460000;
                    if t {
                        break 'b_1918bc;
                    } else {
                        'b_19187c: {
                            c.cycles += 15;
                            c.g[A0][0] = 0x460000;
                            c.g[RA][0] = sx(0x00191834);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d90));
                            call_ptr(c, 0x00317670, 0x00191834)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x00191840);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x00191840)?;
                            c.g[A0][0] = 0x460000;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x00191850);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1d80));
                            call_ptr(c, 0x00317670, 0x00191850)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x0019185c);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x0019185c)?;
                            let t = c.g[S0][0] == 0u64;
                            c.g[S1][0] = c.g[V0][0];
                            if t {
                                break 'b_19187c;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x0019187c);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x0019187c)?;
                                break 'b_19187c;
                            }
                        }
                        'b_191898: {
                            c.cycles += 2;
                            let t = c.g[S1][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_191898;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00191898);
                                c.g[A0][0] =
                                    sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00191898)?;
                                break 'b_191898;
                            }
                        }
                        c.cycles += 9;
                        c.g[V0][0] = 0x4a0000;
                        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1708));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1708));
                        c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x6));
                        c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                        c.sh((c.g[SP][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                        let t = true;
                        c.sb((c.g[SP][0] as u32).wrapping_add(0x36), c.g[A1][0]);
                        if t {
                            break 'b_1918d0;
                        } else {
                            break 'b_1918bc;
                        }
                    }
                }
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x528));
                c.g[A0][0] = c.ld((c.g[V0][0] as u32).wrapping_sub(0x528));
                c.g[A1][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x8));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A1][0]);
                break 'b_1918d0;
            }
            'b_191900: {
                c.cycles += 9;
                c.g[RA][0] = sx(0x001918d8);
                c.g[A0][0] = c.g[FP][0];
                call_ptr(c, 0x00317670, 0x001918d8)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001918e4);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039c778, 0x001918e4)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if t {
                    break 'b_191900;
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00191900);
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039c870, 0x00191900)?;
                    break 'b_191900;
                }
            }
            c.cycles += 14;
            c.g[RA][0] = sx(0x00191908);
            c.g[A0][0] = c.g[S3][0];
            Callee {
                addr: 0x00192488,
                f: ee::s02::cFEStateRuleSelect_UpdateRowVisibility,
            }
            .call(c, 0x00191908)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
            c.pc = t;
            return Ok(());
        }
    });
}

/// `void cFEStateMainMenu_OnWidgetAction(self, Widget* item, int event)`
/// (0x00194ea8): main menu events. 5 select: the item's `mMiscData` (jump
/// table at 0x4600b0, 5 to 49) sets the game path and the go-to-next flag
/// (49 also creates the online managers), then the next state is pushed; 6
/// back; 7 options; 8 the hidden cheat counter that reveals the three
/// extra menu items at 7.
pub fn main_menu_on_widget_action(c: &mut Cpu) {
    if c.pc != 0x00194ea8 {
        return ee::s02::f_00194ea8__generated(c);
    }
    guest(c, |c| 'l_194ea8: loop {
        'b_1952d4: {
            'b_1952d0: {
                'b_1952cc: {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.g[S2][0] = c.g[A1][0];
                    let t = c.g[S2][0] == 0u64;
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    if t {
                        break 'b_1952cc;
                    } else {
                        'b_195208: {
                            'b_1951ac: {
                                'b_195188: {
                                    'b_19516c: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x6;
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                        if t {
                                            break 'b_19516c;
                                        } else {
                                            'b_194f08: {
                                                'b_194eec: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x5;
                                                    if t {
                                                        break 'b_194eec;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[A2][0] == c.g[V0][0];
                                                        c.lq(
                                                            16,
                                                            (c.g[SP][0] as u32).wrapping_add(0x30),
                                                        );
                                                        if t {
                                                            break 'b_194f08;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.lq(
                                                                17,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x20),
                                                            );
                                                            if t {
                                                                break 'b_1952d4;
                                                            } else {
                                                                break 'b_194eec;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x7;
                                                let t = c.g[A2][0] == c.g[V0][0];
                                                c.g[V0][0] = 0x8;
                                                if t {
                                                    break 'b_1951ac;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[A2][0] == c.g[V0][0];
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_195208;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.lq(
                                                            17,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        if t {
                                                            break 'b_1952d4;
                                                        } else {
                                                            break 'b_194f08;
                                                        }
                                                    }
                                                }
                                            }
                                            'b_194fd0: {
                                                c.cycles += 13;
                                                c.g[RA][0] = sx(0x00194f10);
                                                c.g[A0][0] = 0u64;
                                                call_ptr(c, 0x001a86e8, 0x00194f10)?;
                                                c.g[RA][0] = sx(0x00194f18);
                                                call_ptr(c, 0x0014dc80, 0x00194f18)?;
                                                c.g[A0][0] = c.g[V0][0];
                                                c.g[RA][0] = sx(0x00194f24);
                                                c.g[A1][0] = 0u64;
                                                call_ptr(c, 0x0014dd58, 0x00194f24)?;
                                                c.g[S0][0] = c.g[V0][0];
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                                c.g[A0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_sub(0x5));
                                                c.g[V1][0] = (c.g[A0][0] < 0x2du64) as u64;
                                                let t = c.g[V1][0] == 0u64;
                                                c.g[V0][0] = 0x460000;
                                                if t {
                                                    break 'b_194fd0;
                                                } else {
                                                    'b_194fcc: {
                                                        'b_194fc8: {
                                                            'b_194f9c: {
                                                                'b_194f88: {
                                                                    'b_194f68: {
                                                                        c.cycles += 6;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0xb0,
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[V1][0] as u32);
                                                                        let t = c.g[A0][0] as u32;
                                                                        match t {
                                                                            0x00194ea8 => {
                                                                                if event_due(c) {
                                                                                    deliver_event_here(c, 0x00194ea8);
                                                                                }
                                                                                continue 'l_194ea8;
                                                                            }
                                                                            0x00194f54 => {
                                                                                c.cycles += 5;
                                                                                c.g[A0][0] =
                                                                                    c.g[S0][0];
                                                                                c.g[RA][0] =
                                                                                    sx(0x00194f60);
                                                                                c.g[A1][0] = 0u64;
                                                                                call_ptr(
                                                                                    c, 0x00145108,
                                                                                    0x00194f60,
                                                                                )?;
                                                                                let t = true;
                                                                                c.g[V0][0] = 0x1;
                                                                                if t {
                                                                                    break 'b_194fcc;
                                                                                } else {
                                                                                    break 'b_194f68;
                                                                                }
                                                                            }
                                                                            0x00194f68 => {
                                                                                break 'b_194f68;
                                                                            }
                                                                            0x00194f88 => {
                                                                                break 'b_194f88;
                                                                            }
                                                                            0x00194f9c => {
                                                                                break 'b_194f9c;
                                                                            }
                                                                            0x00194fc8 => {
                                                                                break 'b_194fc8;
                                                                            }
                                                                            0x00194fd0 => {
                                                                                break 'b_194fd0;
                                                                            }
                                                                            _ => {
                                                                                c.jump_out(
                                                                                    t, 0x00194ea8,
                                                                                    0x001952e4,
                                                                                );
                                                                                return Ok(());
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 8;
                                                                    c.g[A0][0] = c.g[S0][0];
                                                                    c.g[RA][0] = sx(0x00194f74);
                                                                    c.g[A1][0] = 0x1;
                                                                    call_ptr(
                                                                        c, 0x00145108, 0x00194f74,
                                                                    )?;
                                                                    c.g[V0][0] = c
                                                                        .lb((c.g[S1][0] as u32)
                                                                            .wrapping_add(0x54));
                                                                    c.g[V1][0] = 0x1;
                                                                    c.sw(
                                                                        (c.g[S1][0] as u32)
                                                                            .wrapping_add(0x48),
                                                                        c.g[V1][0],
                                                                    );
                                                                    let t = true;
                                                                    c.g[V0][0] = ((c.g[V0][0]
                                                                        as i64)
                                                                        < 7i64)
                                                                        as u64;
                                                                    if t {
                                                                        break 'b_194fd0;
                                                                    } else {
                                                                        break 'b_194f88;
                                                                    }
                                                                }
                                                                c.cycles += 5;
                                                                c.g[A0][0] = c.g[S0][0];
                                                                c.g[RA][0] = sx(0x00194f94);
                                                                c.g[A1][0] = 0x2;
                                                                call_ptr(
                                                                    c, 0x00145108, 0x00194f94,
                                                                )?;
                                                                let t = true;
                                                                c.g[V0][0] = 0x1;
                                                                if t {
                                                                    break 'b_194fcc;
                                                                } else {
                                                                    break 'b_194f9c;
                                                                }
                                                            }
                                                            c.cycles += 11;
                                                            c.g[A0][0] = c.g[S0][0];
                                                            c.g[RA][0] = sx(0x00194fa8);
                                                            c.g[A1][0] = 0x2;
                                                            call_ptr(c, 0x00145108, 0x00194fa8)?;
                                                            c.g[RA][0] = sx(0x00194fb0);
                                                            call_ptr(c, 0x00255860, 0x00194fb0)?;
                                                            c.g[RA][0] = sx(0x00194fb8);
                                                            call_ptr(c, 0x0025b650, 0x00194fb8)?;
                                                            c.g[RA][0] = sx(0x00194fc0);
                                                            call_ptr(c, 0x00260f80, 0x00194fc0)?;
                                                            c.g[RA][0] = sx(0x00194fc8);
                                                            call_ptr(c, 0x00266ca0, 0x00194fc8)?;
                                                            break 'b_194fc8;
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V0][0] = 0x1;
                                                        break 'b_194fcc;
                                                    }
                                                    c.cycles += 1;
                                                    c.sw(
                                                        (c.g[S1][0] as u32).wrapping_add(0x48),
                                                        c.g[V0][0],
                                                    );
                                                    break 'b_194fd0;
                                                }
                                            }
                                            'b_195150: {
                                                c.cycles += 9;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x00194fe4);
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x00194fe4)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                                c.g[V0][0] = 0x16;
                                                if c.g[A0][0] != c.g[V0][0] {
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                                    break 'b_195150;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = c
                                                        .lw((c.g[S1][0] as u32).wrapping_add(0x48));
                                                    if c.g[V0][0] != 0u64 {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                        break 'b_195150;
                                                    } else {
                                                        'b_1950a0: {
                                                            c.cycles += 35;
                                                            c.g[V0][0] = c.lb((c.g[S1][0] as u32)
                                                                .wrapping_add(0x14));
                                                            c.g[A1][0] = 0x460000;
                                                            c.g[A3][0] = 0u64;
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x80));
                                                            c.g[A0][0] = 0x4c;
                                                            c.g[A2][0] = 0u64;
                                                            c.g[RA][0] = sx(0x00195020);
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x50),
                                                                c.g[V0][0],
                                                            );
                                                            call_ptr(c, 0x00317d70, 0x00195020)?;
                                                            c.g[S0][0] = c.g[V0][0];
                                                            c.g[A1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[A2][0] = c.g[S1][0];
                                                            c.g[RA][0] = sx(0x00195034);
                                                            c.g[A0][0] = c.g[S0][0];
                                                            call_ptr(c, 0x0039e318, 0x00195034)?;
                                                            c.g[A1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x50));
                                                            c.g[V1][0] = 0x470000;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[V0][0] = 0x1;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_sub(0x6ae8));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                << ((c.g[A1][0] as u32) & 31));
                                                            c.g[A1][0] = c.g[S0][0];
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x18));
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                                c.g[V1][0],
                                                            );
                                                            c.sb(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x15),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[RA][0] = sx(0x00195064);
                                                            c.sb(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x48),
                                                                0u64,
                                                            );
                                                            call_ptr(c, 0x0039f290, 0x00195064)?;
                                                            c.sw(
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x4c),
                                                                c.g[S2][0],
                                                            );
                                                            c.g[A0][0] = 0x4a0000;
                                                            c.g[RA][0] = sx(0x00195074);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x17e0));
                                                            call_ptr(c, 0x00317670, 0x00195074)?;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[RA][0] = sx(0x00195080);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x0039d860, 0x00195080)?;
                                                            c.g[A2][0] = c.g[V0][0];
                                                            let t = c.g[A2][0] == 0u64;
                                                            c.g[A1][0] = 0u64;
                                                            if t {
                                                                break 'b_1950a0;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x48));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4c));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[RA][0] = sx(0x001950a0);
                                                                c.g[A0][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                call_ptr(c, t, 0x001950a0)?;
                                                                break 'b_1950a0;
                                                            }
                                                        }
                                                        'b_1950d8: {
                                                            c.cycles += 9;
                                                            c.g[A0][0] = 0x460000;
                                                            c.g[RA][0] = sx(0x001950ac);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0xa0));
                                                            call_ptr(c, 0x00317670, 0x001950ac)?;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[RA][0] = sx(0x001950b8);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x0039d860, 0x001950b8)?;
                                                            c.g[A2][0] = c.g[V0][0];
                                                            let t = c.g[A2][0] == 0u64;
                                                            c.g[A1][0] = 0u64;
                                                            if t {
                                                                break 'b_1950d8;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x48));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4c));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[RA][0] = sx(0x001950d8);
                                                                c.g[A0][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                call_ptr(c, t, 0x001950d8)?;
                                                                break 'b_1950d8;
                                                            }
                                                        }
                                                        'b_195110: {
                                                            c.cycles += 9;
                                                            c.g[A0][0] = 0x4a0000;
                                                            c.g[RA][0] = sx(0x001950e4);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1458));
                                                            call_ptr(c, 0x00317670, 0x001950e4)?;
                                                            c.g[A0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[RA][0] = sx(0x001950f0);
                                                            c.g[A1][0] = c.g[V0][0];
                                                            call_ptr(c, 0x0039d860, 0x001950f0)?;
                                                            c.g[A2][0] = c.g[V0][0];
                                                            let t = c.g[A2][0] == 0u64;
                                                            c.g[A1][0] = 0u64;
                                                            if t {
                                                                break 'b_195110;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x48));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4c));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[RA][0] = sx(0x00195110);
                                                                c.g[A0][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                call_ptr(c, t, 0x00195110)?;
                                                                break 'b_195110;
                                                            }
                                                        }
                                                        c.cycles += 9;
                                                        c.g[A0][0] = 0x4a0000;
                                                        c.g[RA][0] = sx(0x0019511c);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x1460));
                                                        call_ptr(c, 0x00317670, 0x0019511c)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x40));
                                                        c.g[RA][0] = sx(0x00195128);
                                                        c.g[A1][0] = c.g[V0][0];
                                                        call_ptr(c, 0x0039d860, 0x00195128)?;
                                                        c.g[A2][0] = c.g[V0][0];
                                                        let t = c.g[A2][0] == 0u64;
                                                        c.g[A1][0] = 0u64;
                                                        if t {
                                                            break 'b_1952cc;
                                                        } else {
                                                            c.cycles += 7;
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x48));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x4c));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[RA][0] = sx(0x00195148);
                                                            c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x00195148)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            if t {
                                                                break 'b_1952d0;
                                                            } else {
                                                                break 'b_195150;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[A1][0] = c.g[S1][0];
                                            c.g[A2][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                            c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[V1][0] =
                                                c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
                                            let t = true;
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                                            if t {
                                                break 'b_195188;
                                            } else {
                                                break 'b_19516c;
                                            }
                                        }
                                    }
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                                    break 'b_195188;
                                }
                                c.cycles += 4;
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x00195190);
                                c.g[A0][0] =
                                    sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x00195190)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = c.g[V0][0];
                                if t {
                                    break 'b_1952cc;
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                                    c.g[RA][0] = sx(0x001951a4);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x0039f400, 0x001951a4)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_1952d0;
                                    } else {
                                        break 'b_1951ac;
                                    }
                                }
                            }
                            c.cycles += 23;
                            c.g[A1][0] = 0x460000;
                            c.g[A3][0] = 0u64;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2968));
                            c.g[A0][0] = 0x54;
                            c.g[RA][0] = sx(0x001951c4);
                            c.g[A2][0] = 0x100;
                            call_ptr(c, 0x00317d70, 0x001951c4)?;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[A0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001951d4);
                            c.g[A2][0] = 0x1;
                            Callee {
                                addr: 0x001887a0,
                                f: ee::s02::cFEStateOptions_ctor,
                            }
                            .call(c, 0x001951d4)?;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                            c.g[A0][0] = 0xffffffffffffc0ff;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A2][0] = 0u64;
                            c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
                            c.g[A3][0] = 0u64;
                            c.g[V1][0] = c.g[V1][0] | 0x100;
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            c.g[RA][0] = sx(0x00195200);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                            call_ptr(c, 0x0039f320, 0x00195200)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_1952d0;
                            } else {
                                break 'b_195208;
                            }
                        }
                        c.cycles += 7;
                        c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x54));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.g[V1][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
                        c.g[V1][0] = ((c.g[V1][0] as i64) < 7i64) as u64;
                        let t = c.g[V1][0] != 0u64;
                        c.sb((c.g[S1][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                        if t {
                            break 'b_1952cc;
                        } else {
                            'b_19525c: {
                                c.cycles += 9;
                                c.g[A0][0] = 0x460000;
                                c.g[RA][0] = sx(0x00195230);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
                                call_ptr(c, 0x00317670, 0x00195230)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x0019523c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x0019523c)?;
                                c.g[A2][0] = c.g[V0][0];
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_19525c;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0019525c);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x0019525c)?;
                                    break 'b_19525c;
                                }
                            }
                            'b_195294: {
                                c.cycles += 9;
                                c.g[A0][0] = 0x460000;
                                c.g[RA][0] = sx(0x00195268);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                                call_ptr(c, 0x00317670, 0x00195268)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x00195274);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x00195274)?;
                                c.g[A2][0] = c.g[V0][0];
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_195294;
                                } else {
                                    c.cycles += 5;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00195294);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x00195294)?;
                                    break 'b_195294;
                                }
                            }
                            c.cycles += 9;
                            c.g[A0][0] = 0x4a0000;
                            c.g[RA][0] = sx(0x001952a0);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x17e8));
                            call_ptr(c, 0x00317670, 0x001952a0)?;
                            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001952ac);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001952ac)?;
                            c.g[A2][0] = c.g[V0][0];
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = 0u64;
                            if t {
                                break 'b_1952cc;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001952cc);
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001952cc)?;
                                break 'b_1952cc;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_1952d0;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1952d4;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, Rng, DATA};
    use crate::game::main::loadscreens::tests::{
        oc736_fake_call, oc736_fixture, oc736_force, oc736_force_zero, oc736_install_fakes,
    };
    use std::sync::atomic::AtomicUsize;

    // Fixture objects (clear of the fakes' log at DATA+0x100000, of the oc736
    // fixture objects at DATA+0x200000.. and of the pointers the fakes return).
    pub(crate) const OC800_CTRL: u32 = DATA + 0x20_0100;
    pub(crate) const OC800_MAGIC: u32 = 0x4f43_3830;
    /// Per-callee call sequence: `{mask, counter}` at `OC800_SEQ + 8 * slot`.
    pub(crate) const OC800_SEQ: u32 = DATA + 0x20_1000;
    pub(crate) const OC800_SELF: u32 = DATA + 0x2a_0000;
    pub(crate) const OC800_ENG: u32 = DATA + 0x2b_0000;
    pub(crate) const OC800_FACT: u32 = DATA + 0x2c_0000;
    pub(crate) const OC800_FVT: u32 = DATA + 0x2c_1000;
    pub(crate) const OC800_WIDGET: u32 = DATA + 0x2d_0000;
    pub(crate) const OC800_WVT: u32 = DATA + 0x2d_1000;
    pub(crate) const OC800_NAMES: u32 = DATA + 0x2e_0000;
    pub(crate) const OC800_MAP: u32 = DATA + 0x2e_1000;

    /// The mask word of callee `addr`: call n returns its fake result when bit
    /// `n % 32` is set and 0 when it is clear (0 = no mask).
    pub(crate) fn oc800_mask(c: &mut Cpu, addr: u32, mask: u32) {
        c.bus.write32(OC800_SEQ + ((addr >> 2) & 0x3ff) * 8, mask);
    }

    /// The fake callee: the shared oc736 fake (log, result mailbox), then the
    /// per-callee mask, if any, forces a 0 result on its clear bits.
    pub(crate) fn oc800_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let slot = OC800_SEQ + ((addr >> 2) & 0x3ff) * 8;
        let mask = c.bus.read32(slot);
        let n = c.bus.read32(slot + 4);
        if mask != 0 {
            c.bus.write32(slot + 4, n + 1);
        }
        oc736_fake_call(c, addr);
        if mask != 0 && c.pc == ra && (mask >> (n % 32)) & 1 == 0 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc800_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC800_CTRL) != OC800_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc800_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc800_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc800_fakes!(
        (
            oc800_fake_147318,
            OC800_PREV_147318,
            ee::s01::OVERRIDE_00147318,
            ee::s01::f_00147318__generated,
            0x00147318
        ),
        (
            oc800_fake_1474a8,
            OC800_PREV_1474A8,
            ee::s01::OVERRIDE_001474A8,
            ee::s01::f_001474a8__generated,
            0x001474a8
        ),
        (
            oc800_fake_181bd0,
            OC800_PREV_181BD0,
            ee::s02::OVERRIDE_00181BD0,
            ee::s02::f_00181bd0__generated,
            0x00181bd0
        ),
        (
            oc800_fake_1887a0,
            OC800_PREV_1887A0,
            ee::s02::OVERRIDE_001887A0,
            ee::s02::f_001887a0__generated,
            0x001887a0
        ),
        (
            oc800_fake_192240,
            OC800_PREV_192240,
            ee::s02::OVERRIDE_00192240,
            ee::s02::f_00192240__generated,
            0x00192240
        ),
        (
            oc800_fake_192488,
            OC800_PREV_192488,
            ee::s02::OVERRIDE_00192488,
            ee::s02::f_00192488__generated,
            0x00192488
        ),
        (
            oc800_fake_1a1cb8,
            OC800_PREV_1A1CB8,
            ee::s03::OVERRIDE_001A1CB8,
            ee::s03::f_001a1cb8__generated,
            0x001a1cb8
        ),
        (
            oc800_fake_1a86e8,
            OC800_PREV_1A86E8,
            ee::s03::OVERRIDE_001A86E8,
            ee::s03::f_001a86e8__generated,
            0x001a86e8
        ),
        (
            oc800_fake_1d53b0,
            OC800_PREV_1D53B0,
            ee::s04::OVERRIDE_001D53B0,
            ee::s04::f_001d53b0__generated,
            0x001d53b0
        ),
        (
            oc800_fake_25b650,
            OC800_PREV_25B650,
            ee::s06::OVERRIDE_0025B650,
            ee::s06::f_0025b650__generated,
            0x0025b650
        ),
        (
            oc800_fake_260f80,
            OC800_PREV_260F80,
            ee::s06::OVERRIDE_00260F80,
            ee::s06::f_00260f80__generated,
            0x00260f80
        ),
        (
            oc800_fake_266ca0,
            OC800_PREV_266CA0,
            ee::s07::OVERRIDE_00266CA0,
            ee::s07::f_00266ca0__generated,
            0x00266ca0
        ),
        (
            oc800_fake_317670,
            OC800_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc800_fake_317d70,
            OC800_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x00317d70
        ),
        (
            oc800_fake_39a3f8,
            OC800_PREV_39A3F8,
            ee::s13::OVERRIDE_0039A3F8,
            ee::s13::f_0039a3f8__generated,
            0x0039a3f8
        ),
        (
            oc800_fake_39a568,
            OC800_PREV_39A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039a568
        ),
        (
            oc800_fake_39a670,
            OC800_PREV_39A670,
            ee::s13::OVERRIDE_0039A670,
            ee::s13::f_0039a670__generated,
            0x0039a670
        ),
        (
            oc800_fake_39a768,
            OC800_PREV_39A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039a768
        ),
        (
            oc800_fake_39a7a8,
            OC800_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039a7a8
        ),
        (
            oc800_fake_39c778,
            OC800_PREV_39C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039c778
        ),
        (
            oc800_fake_39c870,
            OC800_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039c870
        ),
        (
            oc800_fake_39d860,
            OC800_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc800_fake_39e318,
            OC800_PREV_39E318,
            ee::s13::OVERRIDE_0039E318,
            ee::s13::f_0039e318__generated,
            0x0039e318
        ),
        (
            oc800_fake_39f320,
            OC800_PREV_39F320,
            ee::s13::OVERRIDE_0039F320,
            ee::s13::f_0039f320__generated,
            0x0039f320
        ),
        (
            oc800_fake_39f400,
            OC800_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039f400
        ),
        (
            oc800_fake_3a1000,
            OC800_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a1000
        ),
        (
            oc800_fake_3e5698,
            OC800_PREV_3E5698,
            ee::s14::OVERRIDE_003E5698,
            ee::s14::f_003e5698__generated,
            0x003e5698
        ),
        (
            oc800_fake_3e56e0,
            OC800_PREV_3E56E0,
            ee::s14::OVERRIDE_003E56E0,
            ee::s14::f_003e56e0__generated,
            0x003e56e0
        ),
        (
            oc800_fake_3f3b18,
            OC800_PREV_3F3B18,
            ee::s14::OVERRIDE_003F3B18,
            ee::s14::f_003f3b18__generated,
            0x003f3b18
        ),
        (
            oc800_fake_41aa88,
            OC800_PREV_41AA88,
            ee::s15::OVERRIDE_0041AA88,
            ee::s15::f_0041aa88__generated,
            0x0041aa88
        ),
    );

    /// Scrambled machine with the oc736 fakes (gate `FX_CTRL`) and the oc800
    /// fakes (gate `OC800_CTRL`) on: the callees of the ported menu handlers
    /// are fake calls that log their registers.
    pub(crate) fn oc800_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc736_install_fakes();
        oc800_install_fakes();
        let r = oc736_fixture(c, seed, salt, calls);
        c.bus.write32(OC800_CTRL, OC800_MAGIC);
        r
    }

    /// The UI engine (`ENG`, whose first word is the state factory) and the
    /// factory's `onSelect` (+0x20/+0x24) and `onBack` (+0x28/+0x2c) methods:
    /// a fake call (returns a pointer) or the leaf `0x1406c0` (returns 0).
    pub(crate) fn oc800_engine(c: &mut Cpu, seed: u32) {
        c.bus.write32(OC800_ENG, OC800_FACT);
        c.bus.write32(OC800_FACT + 4, OC800_FVT);
        for (off, k) in [(0x20u32, 0u32), (0x28, 2)] {
            c.bus
                .write16(OC800_FVT + off, 0x10 * ((seed >> k) % 3) as u16);
            let f = if (seed >> (k + 1)) % 2 == 0 {
                fakes::FAKE + 0x30 + off
            } else {
                0x0014_06c0
            };
            c.bus.write32(OC800_FVT + off + 4, f);
        }
    }

    /// The widget: id (or `mMiscData`) at `+0x18`, a vtable with the
    /// `setVisible` slot (+0x48) and the 0x40 slot, both fake calls.
    pub(crate) fn oc800_widget(c: &mut Cpu, id: u32) {
        c.bus.write32(OC800_WIDGET + 0x18, id);
        c.bus.write32(OC800_WIDGET + 8, OC800_WVT);
        for off in [0x40u32, 0x48] {
            c.bus.write16(OC800_WVT + off, 0x10);
            c.bus.write32(OC800_WVT + off + 4, fakes::FAKE + 0x40 + off);
        }
    }

    /// `DebugMenu_CheckWorldFiles`: names already in the location table or
    /// not (the `strcasecmp` fake answers by a per-seed bit mask: all equal,
    /// all different, mixed), `n` of 0 to 3 names with random `bFound`
    /// flags, and `isValidMap` answering 0 or a pointer.
    #[test]
    fn debug_menu_check_world_files_matches() {
        check(
            ee::s02::f_0017a978__generated,
            debug_menu_check_world_files,
            300,
            &|c, seed| {
                let mut r = oc800_machine(c, seed, 0x0017_a978, 24);
                let mask = match seed % 4 {
                    0 => 0x8000_0000,
                    1 => 0xffff_ffff,
                    _ => r.next_u32() | 0x8000_0000,
                };
                oc800_mask(c, 0x0041_aa88, mask);
                if seed % 3 == 0 {
                    oc736_force_zero(c, 0x003a_6948);
                }
                c.bus.write32(OC800_SELF, (seed / 4) % 4);
                let n = (seed / 16) % 4;
                for i in 0..4u32 {
                    c.bus.write32(OC800_NAMES + 8 * i, r.next_u32());
                    c.bus.write32(OC800_NAMES + 8 * i + 4, r.next_u32() & 1);
                }
                c.g[A0][0] = sx(OC800_SELF);
                c.g[A1][0] = sx(n);
                c.g[A2][0] = sx(OC800_NAMES);
                c.pc = 0x0017_a978;
            },
        );
    }

    /// The jump table of `cFEStateCharSelect_OnWidgetAction` (events 1 to 8).
    pub(crate) fn oc800_char_select_table(c: &mut Cpu) {
        for (i, t) in [
            0x0018_1980u32,
            0x0018_1a30,
            0x0018_1ba4,
            0x0018_1ba4,
            0x0018_1844,
            0x0018_193c,
            0x0018_1a50,
            0x0018_1ae8,
        ]
        .into_iter()
        .enumerate()
        {
            c.bus.write32(0x0045_d6c0 + 4 * i as u32, t);
        }
    }

    pub(crate) fn oc800_char_select_setup(c: &mut Cpu, seed: u32, ev: u32, null_widget: bool) {
        let mut r = oc800_machine(c, seed, 0x0018_17e8, 20);
        oc800_char_select_table(c);
        oc800_engine(c, seed);
        c.bus.write32(OC800_SELF + 0x10, OC800_ENG);
        c.bus.write8(OC800_SELF + 0x14, (seed % 4) as u8);
        c.bus.write8(OC800_SELF + 0x44, ((seed / 4) % 4) as u8);
        c.bus.write32(OC800_SELF + 0x4c, r.next_u32());
        c.bus.write32(OC800_SELF + 0x80, 1);
        // D_00440F68: the character id of each widget id.
        for i in 0..0x10u32 {
            c.bus.write8(0x0044_0f68 + i, r.next_u32() as u8);
        }
        oc800_widget(c, (seed / 7) % 0x10);
        // The top of the engine (case 8 writes 9 into `*(engine)->+0xc`).
        c.g[A0][0] = sx(OC800_SELF);
        c.g[A1][0] = if null_widget { 0 } else { sx(OC800_WIDGET) };
        c.g[A2][0] = sx(ev);
        c.pc = 0x0018_17e8;
    }

    /// `cFEStateCharSelect_OnWidgetAction`: events 1 to 7, an event with no
    /// table entry (0, 9) and a null widget; the state factory answers a
    /// state or 0 (nothing to push).
    #[test]
    fn char_select_on_widget_action_matches() {
        check(
            ee::s02::f_001817e8__generated,
            char_select_on_widget_action,
            300,
            &|c, seed| {
                let ev = [1u32, 2, 3, 4, 5, 6, 7, 0, 9, 5, 6, 1][(seed % 12) as usize];
                let null_widget = (seed / 12) % 11 == 0;
                oc800_char_select_setup(c, seed, ev, null_widget);
            },
        );
    }

    /// Event 8 of `cFEStateCharSelect_OnWidgetAction`: the profile-load screen
    /// (the real `cFEStateProfileLoad_ctor`, which has no override slot, on
    /// the fakes of its callees), the engine's top state present or not.
    #[test]
    fn char_select_on_widget_action_profile_matches() {
        check(
            ee::s02::f_001817e8__generated,
            char_select_on_widget_action,
            120,
            &|c, seed| {
                oc800_char_select_setup(c, seed, 8, false);
                if seed % 2 == 0 {
                    c.bus.write32(OC800_ENG, 0);
                }
            },
        );
    }

    /// `cFEStateRuleSelect_FillRuleList`: `first` of 0 to 3 rows into a list
    /// of 0 to 13 rules of every kind (0 to 8), widgets present or missing
    /// (the `FindObject` fake answers by a per-seed bit mask), the frame
    /// lookup failing (0xffff) or not.
    #[test]
    fn rule_select_fill_rule_list_matches() {
        check(
            ee::s02::f_001913b8__generated,
            rule_select_fill_rule_list,
            300,
            &|c, seed| {
                let mut r = oc800_machine(c, seed, 0x0019_13b8, 200);
                // The kind jump table (kinds 0 to 6).
                for (i, t) in [
                    0x0019_175cu32,
                    0x0019_1500,
                    0x0019_1528,
                    0x0019_156c,
                    0x0019_15e0,
                    0x0019_1688,
                    0x0019_1708,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus.write32(0x0045_faf0 + 4 * i as u32, t);
                }
                // D_00441008 (kind per rule) and D_00441018 (label id per rule).
                for i in 0..0x10u32 {
                    c.bus.write8(0x0044_1008 + i, (r.next_u32() % 9) as u8);
                    c.bus.write32(0x0044_1018 + 4 * i, r.next_u32());
                }
                c.bus.write32(OC800_SELF + 0x40, OC800_ENG);
                c.bus.write32(OC800_SELF + 0x48, OC800_MAP);
                c.bus.write8(OC800_SELF + 0x50, ((seed / 3) % 14) as u8);
                for i in 0..0x20u32 {
                    let rule = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32() % 12
                    };
                    c.bus.write8(OC800_MAP + i, rule as u8);
                }
                let mask = match seed % 3 {
                    0 => 0xffff_ffff,
                    _ => r.next_u32() | r.next_u32() | 0x8000_0000,
                };
                oc800_mask(c, 0x0039_d860, mask);
                if seed % 2 == 0 {
                    oc736_force(c, 0x0039_c778, 0xffff);
                }
                c.g[A0][0] = sx(OC800_SELF);
                c.g[A1][0] = sx((seed / 42) % 4);
                c.pc = 0x0019_13b8;
            },
        );
    }

    /// The jump table of the item switch of `cFEStateMainMenu_OnWidgetAction`
    /// (`mMiscData` 5 to 49): 5 -> 0x194fc8, 9 -> 0x194f68, 11 -> 0x194f54,
    /// 22 -> 0x194f88, 49 -> 0x194f9c, else 0x194fd0.
    pub(crate) fn oc800_main_menu_table(c: &mut Cpu) {
        for i in 0..45u32 {
            c.bus.write32(0x0046_00b0 + 4 * i, 0x0019_4fd0);
        }
        for (misc, t) in [
            (5u32, 0x0019_4fc8u32),
            (9, 0x0019_4f68),
            (11, 0x0019_4f54),
            (22, 0x0019_4f88),
            (49, 0x0019_4f9c),
        ] {
            c.bus.write32(0x0046_00b0 + 4 * (misc - 5), t);
        }
    }

    pub(crate) fn oc800_main_menu_setup(
        c: &mut Cpu,
        seed: u32,
        ev: u32,
        misc: u32,
        hack: u32,
        null_widget: bool,
    ) {
        let mut r = oc800_machine(c, seed, 0x0019_4ea8, 40);
        oc800_main_menu_table(c);
        oc800_engine(c, seed);
        c.bus.write32(OC800_SELF + 0x10, OC800_ENG);
        c.bus.write8(OC800_SELF + 0x14, (seed % 4) as u8);
        c.bus.write32(OC800_SELF + 0x40, OC800_ENG + 0x100);
        c.bus.write32(OC800_SELF + 0x48, 0);
        c.bus.write8(OC800_SELF + 0x54, hack as u8);
        oc800_widget(c, misc);
        let mask = match seed % 3 {
            0 => 0xffff_ffff,
            _ => r.next_u32() | 0x8000_0000,
        };
        oc800_mask(c, 0x0039_d860, mask);
        c.g[A0][0] = sx(OC800_SELF);
        c.g[A1][0] = if null_widget { 0 } else { sx(OC800_WIDGET) };
        c.g[A2][0] = sx(ev);
        c.pc = 0x0019_4ea8;
    }

    /// `cFEStateMainMenu_OnWidgetAction`: select with each item kind that has
    /// no online setup (game path 0 to 2, the plain go-on item, an item the
    /// switch ignores), back, options, the cheat counter below and at 7, an
    /// unknown event, and a null item.
    ///
    /// Not reached: the `mMiscData == 22 && !goToNextState` popup (the switch
    /// sets the flag on every path that leaves 22, so it is dead code).
    #[test]
    fn main_menu_on_widget_action_matches() {
        check(
            ee::s02::f_00194ea8__generated,
            main_menu_on_widget_action,
            300,
            &|c, seed| {
                let (ev, misc) = [
                    (5u32, 11u32),
                    (5, 9),
                    (5, 22),
                    (5, 5),
                    (5, 7),
                    (6, 0),
                    (7, 0),
                    (8, 0),
                    (0, 0),
                    (9, 0),
                ][(seed % 10) as usize];
                let hack = (seed / 10) % 9;
                let null_widget = (seed / 10) % 13 == 12;
                oc800_main_menu_setup(c, seed, ev, misc, hack, null_widget);
            },
        );
    }

    /// Item 49 of `cFEStateMainMenu_OnWidgetAction`: the online managers
    /// (`LobbyMan_Create`, `BuddyMan_Create`, `cVoipMan_Create` on fakes; the
    /// real `cCommSystem_Create` and its constructor, which have no override
    /// slot, on the fakes of their callees).
    #[test]
    fn main_menu_on_widget_action_online_matches() {
        check(
            ee::s02::f_00194ea8__generated,
            main_menu_on_widget_action,
            80,
            &|c, seed| {
                oc800_main_menu_setup(c, seed, 5, 49, 0, false);
            },
        );
    }
}
