//! Staging module oc1073 (wave wave265): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1073`.

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
        0x0039_a4b0,
        ui_list_box_add_item_wide,
        "UIListBox_AddItemWide",
    ),
    (
        0x0039_ab50,
        ui_menu_find_next_selectable,
        "UIMenu_FindNextSelectable",
    ),
    (0x0039_ae98, ui_menu_select_index, "UIMenu_SelectIndex"),
    (
        0x0039_b878,
        ui_menu_find_last_selectable,
        "UIMenu_FindLastSelectable",
    ),
];

/// `UIListBox::addEntryByUnicodeString(this, str, data)` (0x39a4b0). Copies the
/// wide string `str` into a new heap buffer of `(length + 1) * 2` bytes and
/// appends a record at `this + 0x9c + 20 * count`: buffer pointer (+0), zero
/// (+4), `data` (+8). Bumps the count at `+0x318`; `v0` gets the old count
/// (the new entry's index, a byte).
pub fn ui_list_box_add_item_wide(c: &mut Cpu) {
    if c.pc != 0x0039a4b0 {
        return ee::s13::f_0039a4b0__generated(c);
    }
    guest(c, |c| {
        c.cycles += 46;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S3][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0039a4dc);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x003dbca0, 0x0039a4dc)?;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[A1][0] = 0x490000;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[RA][0] = sx(0x0039a4f8);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3dc0));
        call_ptr(c, 0x00317e30, 0x0039a4f8)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0039a508);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003dbe20, 0x0039a508)?;
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.g[A0][0] = 0x14;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.mult(0, 2, 3, 4);
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xa0), 0u64);
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.mult(0, 3, 2, 4);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), c.g[S1][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.mult(0, 2, 3, 4);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xa4), c.g[S3][0]);
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x318));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sb((c.g[S0][0] as u32).wrapping_add(0x318), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] & 0xff;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `UIMenu::getNext` (0x39ab50): the next selectable entry after the current
/// index (`+0x95`) over `count` (`+0x96`) entries. An entry is selectable when
/// its flags (`+0x14`) have bit 5 clear; with the menu's `+0x90` bit 2 set, the
/// owner's virtual hook decides instead (nonzero accepts). Entries come from
/// the list at `+0x74`. `v0` gets the index found (0 when none qualifies).
pub fn ui_menu_find_next_selectable(c: &mut Cpu) {
    if c.pc != 0x0039ab50 {
        return ee::s13::f_0039ab50__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0039ab50;
        loop {
            match pc {
                0x0039ab50 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S3][0] = 0xff;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x95));
                    c.g[S2][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[S0][0] = c.g[V0][0] & 0xff;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
                    pc = 0x0039ab88;
                    continue;
                }
                0x0039ab88 => {
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;

                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                        pc = 0x0039ac00;
                        continue;
                    }
                    pc = 0x0039ab94;
                    continue;
                }
                0x0039ab94 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = c.g[V0][0] & 0x4;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x0039abd8;
                        continue;
                    }
                    pc = 0x0039aba4;
                    continue;
                }
                0x0039aba4 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0039abc8);
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0039abc8)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x0039abf8;
                        continue;
                    }
                    pc = 0x0039abd0;
                    continue;
                }
                0x0039abd0 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.g[S0][0];
                    if t {
                        pc = 0x0039ac2c;
                        continue;
                    }
                    pc = 0x0039abd8;
                    continue;
                }
                0x0039abd8 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[RA][0] = sx(0x0039abe4);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00397870,
                        f: ee::s13::UIList_GetAt,
                    }
                    .call(c, 0x0039abe4)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                    c.g[V1][0] = c.g[V1][0] & 0x1;
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        if back_edge_unwinds(c, 0x0039abd0) {
                            return Ok(());
                        }
                        pc = 0x0039abd0;
                        continue;
                    }
                    pc = 0x0039abf8;
                    continue;
                }
                0x0039abf8 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[S0][0] = c.g[V0][0] & 0xff;
                    if t {
                        pc = 0x0039ac18;
                        continue;
                    }
                    pc = 0x0039ac00;
                    continue;
                }
                0x0039ac00 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 7) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x0039ac18;
                        continue;
                    }
                    pc = 0x0039ac10;
                    continue;
                }
                0x0039ac10 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x95));
                    if t {
                        pc = 0x0039ac2c;
                        continue;
                    }
                    pc = 0x0039ac18;
                    continue;
                }
                0x0039ac18 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[S2][0] = c.g[V0][0] & 0xff;

                    if c.g[S2][0] != c.g[S3][0] {
                        c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
                        if back_edge_unwinds(c, 0x0039ab88) {
                            return Ok(());
                        }
                        pc = 0x0039ab88;
                        continue;
                    }
                    pc = 0x0039ac28;
                    continue;
                }
                0x0039ac28 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = 0x0039ac2c;
                    continue;
                }
                0x0039ac2c => {
                    c.cycles += 7;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039ab50, 0x0039ac48);
                    return Ok(());
                }
            }
        }
    });
}

/// `bool UIMenu::onInputChangeItem(UIMenu* this, u8 old)` (0x39ae98): when the
/// current index (`+0x95`) differs from `old`, scrolls the visible window
/// (`+0x97` visible rows, `+0x98` first row) so the current entry shows, calls
/// the owner's virtual hooks, makes the entry the current item
/// (`UIMenu_SetCurrentItem`) and returns true. Returns false when `old` is
/// already current.
pub fn ui_menu_select_index(c: &mut Cpu) {
    if c.pc != 0x0039ae98 {
        return ee::s13::f_0039ae98__generated(c);
    }
    guest(c, |c| {
        'b_39afec: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.g[A1][0] = c.g[A1][0] & 0xff;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x95));
            let t = c.g[A1][0] == c.g[V0][0];
            c.g[V0][0] = 0u64;
            if t {
                break 'b_39afec;
            } else {
                'b_39afa4: {
                    'b_39af30: {
                        c.cycles += 5;
                        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x95));
                        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x98));
                        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
                        if (c.g[S1][0] as i64) >= 0 {
                            c.g[A0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x97));
                            break 'b_39af30;
                        } else {
                            c.cycles += 24;
                            c.sb((c.g[S0][0] as u32).wrapping_add(0x98), c.g[V1][0]);
                            c.g[A1][0] = c.g[S0][0];
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                            c.g[A2][0] = 0x2;
                            c.g[A3][0] = c.g[V1][0] & 0xff;
                            c.g[S1][0] = 0u64;
                            c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0039af00);
                            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x0039af00)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A3][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x98));
                            c.g[A2][0] = 0x4;
                            c.g[T0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xd0));
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x0039af28);
                            c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                            call_ptr(c, t, 0x0039af28)?;
                            let t = true;
                            c.g[A1][0] = c.g[S1][0];
                            if t {
                                break 'b_39afa4;
                            } else {
                                break 'b_39af30;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        break 'b_39afa4;
                    } else {
                        c.cycles += 26;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        c.sb((c.g[S0][0] as u32).wrapping_add(0x98), c.g[V0][0]);
                        c.g[A2][0] = 0x3;
                        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x97));
                        c.g[T0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xd0));
                        c.g[A3][0] = c.g[V0][0] & 0xff;
                        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0039af78);
                        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0039af78)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A3][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x98));
                        c.g[A2][0] = 0x4;
                        c.g[T0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xd0));
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0039afa0);
                        c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x0039afa0)?;
                        c.g[A1][0] = c.g[S1][0];
                        break 'b_39afa4;
                    }
                }
                c.cycles += 18;
                c.g[RA][0] = sx(0x0039afac);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
                Callee {
                    addr: 0x00397870,
                    f: ee::s13::UIList_GetAt,
                }
                .call(c, 0x0039afac)?;
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x90));
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0039afc0);
                c.g[A2][0] = c.g[A2][0] & 0x1;
                Callee {
                    addr: 0x0039ad38,
                    f: ee::s13::UIMenu_SetCurrentItem,
                }
                .call(c, 0x0039afc0)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
                c.g[A1][0] = c.g[S0][0];
                c.g[A3][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x98));
                c.g[A2][0] = 0x5;
                c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0039afe8);
                c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x0039afe8)?;
                c.g[V0][0] = 0x1;
                break 'b_39afec;
            }
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `UIMenu::getPrev`-style search (0x39b878), the backward twin of
/// `ui_menu_find_next_selectable`: from the given index (or `count - 1` when it
/// is past the end) back towards 0, the first selectable entry (flags bit 5
/// clear, or the owner's virtual hook when `+0x90` bit 2 is set). `v0` gets
/// its index, or 0xff when none qualifies.
pub fn ui_menu_find_last_selectable(c: &mut Cpu) {
    if c.pc != 0x0039b878 {
        return ee::s13::f_0039b878__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x0039b878;
        loop {
            match pc {
                0x0039b878 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[A1][0] = c.g[A1][0] & 0xff;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V1][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x96));
                    c.g[V0][0] = (c.g[A1][0] < c.g[V1][0]) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = c.g[A1][0];
                    if t {
                        pc = 0x0039b8b4;
                        continue;
                    }
                    pc = 0x0039b8a8;
                    continue;
                }
                0x0039b8a8 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                    c.g[A1][0] = c.g[V0][0] & 0xff;
                    c.g[S0][0] = c.g[A1][0];
                    pc = 0x0039b8b4;
                    continue;
                }
                0x0039b8b4 => {
                    c.cycles += 2;
                    let t = (c.g[S0][0] as i64) < 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 16);
                    if t {
                        pc = 0x0039b93c;
                        continue;
                    }
                    pc = 0x0039b8bc;
                    continue;
                }
                0x0039b8bc => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[S3][0] = 0xffffffffffff0000;
                    pc = 0x0039b8c8;
                    continue;
                }
                0x0039b8c8 => {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x74));
                    c.g[RA][0] = sx(0x0039b8d4);
                    c.g[A1][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00397870,
                        f: ee::s13::UIList_GetAt,
                    }
                    .call(c, 0x0039b8d4)?;
                    c.g[V1][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = c.g[V0][0] & 0x4;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[S1][0];
                    if t {
                        pc = 0x0039b91c;
                        continue;
                    }
                    pc = 0x0039b8e8;
                    continue;
                }
                0x0039b8e8 => {
                    c.cycles += 11;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5c));
                    c.g[A2][0] = 0x1;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xa0));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0039b90c);
                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x0039b90c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[S2][0];
                    if t {
                        pc = 0x0039b930;
                        continue;
                    }
                    pc = 0x0039b914;
                    continue;
                }
                0x0039b914 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.g[S0][0] & 0xff;
                    if t {
                        pc = 0x0039b940;
                        continue;
                    }
                    pc = 0x0039b91c;
                    continue;
                }
                0x0039b91c => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                    c.g[V0][0] = c.g[V0][0] & 0x1;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[S2][0];
                    if t {
                        if back_edge_unwinds(c, 0x0039b914) {
                            return Ok(());
                        }
                        pc = 0x0039b914;
                        continue;
                    }
                    pc = 0x0039b930;
                    continue;
                }
                0x0039b930 => {
                    c.cycles += 3;
                    c.g[S0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
                    let t = (c.g[S0][0] as i64) >= 0;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                    if t {
                        if back_edge_unwinds(c, 0x0039b8c8) {
                            return Ok(());
                        }
                        pc = 0x0039b8c8;
                        continue;
                    }
                    pc = 0x0039b93c;
                    continue;
                }
                0x0039b93c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xff;
                    pc = 0x0039b940;
                    continue;
                }
                0x0039b940 => {
                    c.cycles += 7;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0039b878, 0x0039b95c);
                    return Ok(());
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::{check, fakes, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The menu under test: list header `+0x74`, current index `+0x95`,
    /// count `+0x96`, scroll state `+0x97`/`+0x98`.
    pub(crate) const NAV_MENU: u32 = DATA + 0x400;
    pub(crate) const NAV_OWNER: u32 = DATA + 0x800;
    pub(crate) const NAV_HELPER: u32 = DATA + 0xa00;
    pub(crate) const NAV_VT: u32 = DATA + 0xc00;
    pub(crate) const NAV_NODES: u32 = DATA + 0x1000;
    /// Gate word of the faked `UIMenu_SetCurrentItem` (in this machine's RAM).
    pub(crate) const OC1073_CTRL: u32 = DATA + 0x34_0000;
    pub(crate) const OC1073_MAGIC: u32 = 0x1073_c0de;
    static OC1073_PREV_SET_CURRENT: AtomicUsize = AtomicUsize::new(0);

    /// Gated fake for `UIMenu_SetCurrentItem` (0x39ad38): logs the call when
    /// the gate is armed, otherwise runs the body it replaced.
    pub(crate) fn oc1073_fake_set_current(c: &mut Cpu) {
        if c.bus.read32(OC1073_CTRL) != OC1073_MAGIC {
            return fakes::fall_back(c, &OC1073_PREV_SET_CURRENT, ee::s13::f_0039ad38__generated);
        }
        fakes::fake_call(c, 0x0039_ad38);
    }

    /// The menu's owner chain (`+0x5c` owner, `owner+0xd0` helper,
    /// `helper+8` vtable), a virtual hook at vtable slot `+0xa0` that calls
    /// the fake at `FAKE`, and an intrusive list of `count` nodes behind the
    /// menu's list header (first node at `+0x78`, `node+4` links, `node+0x14`
    /// flags).
    pub(crate) fn nav_wire(c: &mut Cpu, r: &mut Rng, count: u32) {
        c.bus.write32(NAV_MENU + 0x5c, NAV_OWNER);
        c.bus.write32(NAV_OWNER + 0xd0, NAV_HELPER);
        c.bus.write32(NAV_HELPER + 8, NAV_VT);
        c.bus
            .write16(NAV_VT + 0xa0, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(NAV_VT + 0xa4, fakes::FAKE);
        c.bus
            .write32(NAV_MENU + 0x78, if count == 0 { 0 } else { NAV_NODES });
        for i in 0..count {
            let node = NAV_NODES + 0x100 * i;
            let next = if i + 1 < count { node + 0x100 } else { 0 };
            c.bus.write32(node + 4, next);
            c.bus.write32(node + 0x14, r.next_u32());
        }
    }

    /// A menu with `seed % 6` entries and a random current index, the
    /// virtual-hook flag `+0x90` bit 2 on every third seed pair, and random
    /// menu flags (`+0x14`, bit 7 wraps the search).
    pub(crate) fn nav_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine_rt(c, seed, salt, 0);
        let count = seed % 6;
        let cur = if count == 0 { 0 } else { r.next_u32() % count };
        c.bus.write8(NAV_MENU + 0x95, cur as u8);
        c.bus.write8(NAV_MENU + 0x96, count as u8);
        c.bus.write32(NAV_MENU + 0x14, r.next_u32());
        c.bus
            .write32(NAV_MENU + 0x90, if (seed / 3) % 2 == 0 { 4 } else { 0 });
        nav_wire(c, &mut r, count);
        c.g[A0][0] = sx(NAV_MENU);
        r
    }

    #[test]
    fn ui_list_box_add_item_wide_matches() {
        check(
            ee::s13::f_0039a4b0__generated,
            ui_list_box_add_item_wide,
            200,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x39a4_b0, 0);
                let this = DATA + 0x400;
                let text = DATA + 0x2000;
                let len = (seed / 6) % 6;
                c.bus.write8(this + 0x318, (seed % 6) as u8);
                for i in 0..len {
                    c.bus
                        .write16(text + 2 * i, (0x20 + r.next_u32() % 0x5f) as u16);
                }
                c.bus.write16(text + 2 * len, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(text);
                c.g[A2][0] = r.next_u32() as u64;
                c.pc = 0x0039_a4b0;
            },
        );
    }

    #[test]
    fn ui_menu_find_next_selectable_matches() {
        check(
            ee::s13::f_0039ab50__generated,
            ui_menu_find_next_selectable,
            300,
            &|c, seed| {
                nav_setup(c, seed, 0x39ab_50);
                c.pc = 0x0039_ab50;
            },
        );
    }

    #[test]
    fn ui_menu_find_last_selectable_matches() {
        check(
            ee::s13::f_0039b878__generated,
            ui_menu_find_last_selectable,
            300,
            &|c, seed| {
                let mut r = nav_setup(c, seed, 0x39b8_78);
                // The start index: in range on odd seeds, past the end otherwise.
                let start = if seed % 2 == 1 {
                    r.next_u32() % 6
                } else {
                    r.next_u32() % 8
                };
                c.g[A1][0] = (start | (r.next_u32() & !0xff)) as u64;
                c.pc = 0x0039_b878;
            },
        );
    }

    #[test]
    fn ui_menu_select_index_matches() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039AD38),
            oc1073_fake_set_current,
            &OC1073_PREV_SET_CURRENT,
        );
        check(
            ee::s13::f_0039ae98__generated,
            ui_menu_select_index,
            300,
            &|c, seed| {
                let mut r = machine_rt(c, seed, 0x39ae_98, 0);
                c.bus.write32(OC1073_CTRL, OC1073_MAGIC);
                nav_wire(c, &mut r, 8);
                let cur = r.next_u32() % 8;
                let limit = r.next_u32() % 8;
                let old = r.next_u32() % 8;
                c.bus.write8(NAV_MENU + 0x95, cur as u8);
                c.bus.write8(NAV_MENU + 0x97, limit as u8);
                c.bus.write8(NAV_MENU + 0x98, old as u8);
                // The argument is the old index: equal to the current one on
                // every fourth seed (no change), with junk in the high bits.
                let index = if seed % 4 == 0 { cur } else { r.next_u32() % 8 };
                c.g[A0][0] = sx(NAV_MENU);
                c.g[A1][0] = (index | (r.next_u32() & !0xff)) as u64;
                c.pc = 0x0039_ae98;
            },
        );
    }
}
