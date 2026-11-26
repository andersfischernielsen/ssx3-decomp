//! Staging module oc922 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc922`.
//!
//! Not ported here: `cFEStateStats_OnWidgetCreated` (0x001c2f00) is a resumable
//! `match pc` dispatcher body (a hash chain with a ten-iteration loop); the
//! orchestrator converts it with `tools/port_dispatcher.py`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001c_1560,
        oc922_net_quick_play_popup_on_widget_action,
        "cFEStateNetQuickPlayPopup_OnWidgetAction",
    ),
    (
        0x001c_3310,
        oc922_stats_refresh_results,
        "cFEStateStats_RefreshResults",
    ),
    (
        0x001c_4538,
        oc922_stats_set_event_course_visible,
        "cFEStateStats_SetEventCourseVisible",
    ),
    (
        0x001c_47a8,
        oc922_stats_fill_event_list,
        "cFEStateStats_FillEventList",
    ),
];

// Callees, all called through the function table as the generated bodies do.
const OC922_STR_HASH: u32 = 0x0031_7670;
const OC922_FIND_OBJECT: u32 = 0x0039_d860;
const OC922_SPRINTF: u32 = 0x0041_7828;
const OC922_SET_STRING: u32 = 0x003a_0c50;
const OC922_SET_TEXT_ID: u32 = 0x003a_1000;
const OC922_RESET_SCROLL: u32 = 0x0039_b760;
const OC922_LIST_SELECTED: u32 = 0x0039_a738;
const OC922_LIST_CLEAR: u32 = 0x0039_a670;
const OC922_LIST_ADD_STR_ID: u32 = 0x0039_a568;
const OC922_LIST_ADD_ASCII: u32 = 0x0039_a3f8;
const OC922_GET_INTERFACE: u32 = 0x0014_dd58;
const OC922_GET_BE: u32 = 0x0014_dc80;
const OC922_EVENT_NAME: u32 = 0x0014_53b8;
const OC922_SET_NUM_AI: u32 = 0x0014_4d98;
const OC922_RANK_COUNT: u32 = 0x0025_f338;
const OC922_RANK_ENTRY: u32 = 0x0025_f388;
const OC922_START_QUICK_PLAY: u32 = 0x0025_ee20;
const OC922_CANCEL_QUICK_PLAY: u32 = 0x0025_eed8;

/// `lw` of `reg + off`.
#[inline]
fn oc922_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    c.lw((c.g[reg][0] as u32).wrapping_add(off))
}

/// `addiu`: `reg + k`, sign-extended.
#[inline]
fn oc922_add(c: &Cpu, reg: usize, k: u32) -> u64 {
    sx((c.g[reg][0] as u32).wrapping_add(k))
}

/// `addu a, b` of two registers, sign-extended.
#[inline]
fn oc922_add_regs(c: &Cpu, a: usize, b: usize) -> u64 {
    sx((c.g[a][0] as u32).wrapping_add(c.g[b][0] as u32))
}

/// The compiled virtual call `obj->vfn(a1)` of gcc 2.9x: `v0 = obj->vtable`
/// (`lw obj+8`), `a0 = lh vt+slot` (the `this` delta), `v1 = lw vt+slot+4`,
/// `a0 = obj + delta`, `jalr v1` returning to `ret`. `obj_reg` holds `obj`,
/// `a1` is already set.
fn oc922_vcall(c: &mut Cpu, obj_reg: usize, slot: u32, ret: u32) -> Flow {
    c.g[V0][0] = oc922_lw(c, obj_reg, 0x8);
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(slot));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(slot + 4));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = oc922_add_regs(c, obj_reg, A0);
    call_ptr(c, target, ret)
}

// ---------------------------------------------------------------------------
// cFEStateNetQuickPlayPopup_OnWidgetAction (0x001c1560)
// ---------------------------------------------------------------------------

/// Epilogue of the popup handler: `lq s0`, `s1`/`s2`, `ra`, `sp += 0x40`.
fn oc922_qp_epilogue(c: &mut Cpu, frame: Frame) {
    frame.restore(c, 16, 0x30);
    c.cycles += 5;
    frame.restore(c, 17, 0x20);
    frame.restore(c, 18, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// "Aborting quickmatch" on the searching text (string-id lookup, `SetTextId`)
/// and `setStatic(1)` on the OK button. `rets` are the three return addresses:
/// the two message variants of the generated body differ only in them.
fn oc922_qp_abort_texts(c: &mut Cpu, rets: [u32; 3]) -> Flow {
    c.g[A0][0] = 0x0046_0000;
    c.g[A0][0] = oc922_add(c, A0, 0x5938);
    call_ptr(c, OC922_STR_HASH, rets[0])?;
    c.g[A0][0] = oc922_lw(c, S2, 0x6e4);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, OC922_SET_TEXT_ID, rets[1])?;
    c.g[A2][0] = oc922_lw(c, S2, 0x6f4);
    c.g[A1][0] = 1;
    oc922_vcall(c, A2, 0x40, rets[2])
}

/// Tail of every "abort the search" path: cancel the quick play in the lobby
/// manager.
fn oc922_qp_cancel(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    call_ptr(c, OC922_CANCEL_QUICK_PLAY, 0x001c_178c)
}

/// The OK button while idle: start the search. Marks the popup searching,
/// copies the five option list boxes' selections (+0x70c..+0x71c) into the
/// search options (+0x6d0..+0x6e0), submits them to the BE network interface
/// and the lobby manager, shows the searching text, relabels the OK button
/// "Cancel" and sets the blink counter (+0x6e8) to 30.
fn oc922_qp_start_search(c: &mut Cpu) -> Flow {
    c.cycles += 67;
    c.sw((c.g[S2][0] as u32).wrapping_add(0x6f0), 0);
    c.sw((c.g[S2][0] as u32).wrapping_add(0x6ec), c.g[V0][0]);
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 0;
    c.g[S1][0] = oc922_add(c, S2, 0x6d0);
    Callee {
        addr: 0x001c_13e0,
        f: ee::s03::cFEStateNetQuickPlayPopup_SetOptionsHintVisible,
    }
    .call(c, 0x001c_1640)?;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[A1][0] = 0x7;
    c.g[A0][0] = oc922_lw(c, V0, 0x78);
    call_ptr(c, OC922_GET_INTERFACE, 0x001c_1650)?;
    c.g[A0][0] = oc922_lw(c, S2, 0x70c);
    c.g[S0][0] = c.g[V0][0];
    call_ptr(c, OC922_LIST_SELECTED, 0x001c_165c)?;
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    c.g[V1][0] = 0x0053_0000;
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4b34), c.g[V0][0]);
    // interface->submit()
    c.g[V0][0] = oc922_lw(c, S0, 0xc);
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = oc922_add_regs(c, S0, A0);
    call_ptr(c, target, 0x001c_167c)?;
    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[A1][0] = 0;
    c.g[A0][0] = oc922_lw(c, V0, 0x78);
    call_ptr(c, OC922_GET_INTERFACE, 0x001c_168c)?;
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, OC922_SET_NUM_AI, 0x001c_1698)?;
    c.g[A0][0] = oc922_lw(c, S2, 0x70c);
    call_ptr(c, OC922_LIST_SELECTED, 0x001c_16a0)?;
    c.g[A0][0] = oc922_lw(c, S2, 0x710);
    c.sw((c.g[S2][0] as u32).wrapping_add(0x6d0), c.g[V0][0]);
    call_ptr(c, OC922_LIST_SELECTED, 0x001c_16ac)?;
    c.g[A0][0] = oc922_lw(c, S2, 0x714);
    c.sw((c.g[S2][0] as u32).wrapping_add(0x6d4), c.g[V0][0]);
    call_ptr(c, OC922_LIST_SELECTED, 0x001c_16b8)?;
    c.g[A0][0] = oc922_lw(c, S2, 0x718);
    c.sw((c.g[S2][0] as u32).wrapping_add(0x6d8), c.g[V0][0]);
    call_ptr(c, OC922_LIST_SELECTED, 0x001c_16c4)?;
    c.g[A0][0] = oc922_lw(c, S2, 0x71c);
    c.sw((c.g[S2][0] as u32).wrapping_add(0x6dc), c.g[V0][0]);
    call_ptr(c, OC922_LIST_SELECTED, 0x001c_16d0)?;
    c.g[A2][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x14));
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    c.sw((c.g[S2][0] as u32).wrapping_add(0x6e0), c.g[V0][0]);
    call_ptr(c, OC922_START_QUICK_PLAY, 0x001c_16e4)?;
    // searching text visible
    c.g[A2][0] = oc922_lw(c, S2, 0x6e4);
    c.g[A1][0] = 1;
    oc922_vcall(c, A2, 0x48, 0x001c_1700)?;
    // OK button label: "Cancel"
    c.g[A0][0] = 0x0046_0000;
    c.g[A0][0] = oc922_add(c, A0, 0x2c88);
    call_ptr(c, OC922_STR_HASH, 0x001c_170c)?;
    c.g[A0][0] = oc922_lw(c, S2, 0x6f4);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, OC922_SET_TEXT_ID, 0x001c_1718)?;
    c.g[V0][0] = 0x1e;
    c.g[A0][0] = c.g[S2][0];
    c.sw((c.g[S2][0] as u32).wrapping_add(0x6e8), c.g[V0][0]);
    c.g[A1][0] = 0x1;
    Callee {
        addr: 0x001c_17f8,
        f: ee::s03::cFEStateNetQuickPlayPopup_SetOptionsEnabled,
    }
    .call(c, 0x001c_172c)
}

/// `cFEStateNetQuickPlayPopup::onWidgetEvent(self, widget, msg)` (0x001c1560):
/// the quick-match popup of the network front end. Message 7 forwards to the
/// base state while not searching; 5 (OK pressed on the OK button) starts the
/// search or, while searching, aborts it; 6 (back) aborts a running search or
/// tells the parent state the popup is done (event 0x14); everything else goes
/// to the base state `NetFEState_OnWidgetAction`.
pub fn oc922_net_quick_play_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x6;
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        let msg = c.g[A2][0];
        // The generated branch tree: 6, then 5, then 7, else the default.
        let mut case = 0u32;
        if msg == c.g[V0][0] {
            case = 6;
        } else {
            c.cycles += 3;
            let below7 = msg < 0x7;
            c.g[V0][0] = 0x5;
            if below7 {
                c.cycles += 2;
                if msg == 5 {
                    c.g[V0][0] = oc922_lw(c, S2, 0x6f4);
                    case = 5;
                } else {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x7;
                if msg == 7 {
                    case = 7;
                }
            }
        }
        match case {
            7 => {
                c.cycles += 3;
                c.g[V0][0] = oc922_lw(c, S2, 0x6ec);
                // `lq s0` sits in the branch delay slot: before the call.
                frame.restore(c, 16, 0x30);
                if c.g[V0][0] == 0 {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = 0x7;
                    Callee {
                        addr: 0x001a_8918,
                        f: ee::s03::NetFEState_OnWidgetAction,
                    }
                    .call(c, 0x001c_15c0)?;
                }
            }
            5 => {
                c.cycles += 2;
                // `lq s0` in the branch delay slot (before the search calls).
                frame.restore(c, 16, 0x30);
                // Only the popup's OK button (+0x6f4) reacts.
                if c.g[V0][0] == c.g[A1][0] {
                    c.cycles += 3;
                    c.g[V0][0] = oc922_lw(c, S2, 0x6ec);
                    let searching = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x1;
                    if searching {
                        c.cycles += 19;
                        oc922_qp_abort_texts(c, [0x001c_15e8, 0x001c_15f4, 0x001c_1610])?;
                        c.g[A1][0] = oc922_lw(c, S2, 0x6ec);
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] ^= 0x1;
                        Callee {
                            addr: 0x001c_13e0,
                            f: ee::s03::cFEStateNetQuickPlayPopup_SetOptionsHintVisible,
                        }
                        .call(c, 0x001c_1620)?;
                        oc922_qp_cancel(c)?;
                    } else {
                        oc922_qp_start_search(c)?;
                    }
                }
            }
            6 => {
                c.cycles += 7;
                c.g[A1][0] = oc922_lw(c, S2, 0x6ec);
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] ^= 0x1;
                Callee {
                    addr: 0x001c_13e0,
                    f: ee::s03::cFEStateNetQuickPlayPopup_SetOptionsHintVisible,
                }
                .call(c, 0x001c_1744)?;
                c.g[V0][0] = oc922_lw(c, S2, 0x6ec);
                if c.g[V0][0] == 0 {
                    // Not searching: tell the parent state the popup is done.
                    c.g[A3][0] = oc922_lw(c, S2, 0x20);
                    c.cycles += 9;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = 0x14;
                    oc922_vcall(c, A3, 0x88, 0x001c_17b0)?;
                } else {
                    c.cycles += 13;
                    oc922_qp_abort_texts(c, [0x001c_175c, 0x001c_1768, 0x001c_1784])?;
                    oc922_qp_cancel(c)?;
                }
            }
            _ => {
                c.cycles += 3;
                c.g[A0][0] = c.g[S2][0];
                Callee {
                    addr: 0x001a_8918,
                    f: ee::s03::NetFEState_OnWidgetAction,
                }
                .call(c, 0x001c_17c0)?;
            }
        }
        oc922_qp_epilogue(c, frame);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFEStateStats_RefreshResults (0x001c3310)
// ---------------------------------------------------------------------------

/// `sprintf(sp, fmt, row)`, `hash = StrHash(sp)`, `FindObject(screen, hash)`:
/// one named widget of result row `s7` (the format string is `0x460000 +
/// fmt_lo`). The widget is left in `v0`.
fn oc922_rr_find_row_widget(c: &mut Cpu, fmt_lo: u32, rets: [u32; 3]) -> Flow {
    c.g[V0][0] = 0x0046_0000;
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = c.g[S7][0];
    c.g[A1][0] = oc922_add(c, V0, fmt_lo);
    call_ptr(c, OC922_SPRINTF, rets[0])?;
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, OC922_STR_HASH, rets[1])?;
    c.g[A0][0] = oc922_lw(c, S4, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, OC922_FIND_OBJECT, rets[2])
}

/// Body of `RefreshResults` after the screen check: the ranking list (up to
/// ten visible rows starting at `s5`, with `s6` entries in all).
fn oc922_rr_body(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 12;
    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    call_ptr(c, OC922_RANK_COUNT, 0x001c_3354)?;
    c.g[A0][0] = 0x0046_0000;
    c.g[S6][0] = c.g[V0][0];
    c.g[A0][0] = oc922_add(c, A0, 0x5d50);
    call_ptr(c, OC922_STR_HASH, 0x001c_3364)?;
    c.g[A0][0] = oc922_lw(c, S4, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, OC922_FIND_OBJECT, 0x001c_3370)?;
    c.g[S0][0] = c.g[V0][0];
    let no_menu = c.g[S0][0] == 0;
    c.g[V0][0] = ((c.g[S6][0] as i64) < 11) as u64;
    if !no_menu {
        // ResultsMenu: resize the scroll list to the entry count.
        c.cycles += 3;
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x96));
        let same = c.g[V0][0] == c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        if !same {
            c.cycles += 4;
            c.g[A1][0] = c.g[S6][0] & 0xff;
            call_ptr(c, OC922_RESET_SCROLL, 0x001c_3390)?;
            let nonzero = c.g[S6][0] != 0;
            c.g[V0][0] = ((c.g[S6][0] as i64) < 11) as u64;
            if !nonzero {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, OC922_RESET_SCROLL, 0x001c_33a4)?;
                c.cycles += 1;
                c.g[V0][0] = ((c.g[S6][0] as i64) < 11) as u64;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = ((c.g[S6][0] as i64) < 11) as u64;
        }
    }
    // First visible row: the stored one, unless the list is longer than ten
    // and the stored one would show less than a full page.
    c.cycles += 2;
    let long_list = c.g[V0][0] == 0;
    c.g[S5][0] = oc922_lw(c, S4, 0x6e8);
    if long_list {
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(c.g[S5][0] as u32));
        c.g[V1][0] = 0x9;
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0xa));
        if c.g[V1][0] == 0 {
            c.g[S5][0] = c.g[A0][0];
        }
    }
    c.cycles += 7;
    c.g[V0][0] = oc922_add(c, SP, 0x60);
    c.g[S7][0] = 0;
    c.sw((c.g[SP][0] as u32).wrapping_add(0x6c), c.g[V0][0]);
    c.g[FP][0] = oc922_add(c, SP, 0x20);
    c.g[V0][0] = oc922_add(c, SP, 0x64);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x68), c.g[V0][0]);
    loop {
        c.cycles += 41;
        oc922_rr_find_row_widget(c, 0x5d90, [0x001c_33f4, 0x001c_33fc, 0x001c_3408])?;
        c.g[S3][0] = c.g[V0][0];
        oc922_rr_find_row_widget(c, 0x5da0, [0x001c_3420, 0x001c_3428, 0x001c_3434])?;
        c.g[S2][0] = c.g[V0][0];
        oc922_rr_find_row_widget(c, 0x5db0, [0x001c_344c, 0x001c_3454, 0x001c_3460])?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = oc922_lw(c, SP, 0x6c);
        c.g[A1][0] = c.g[S5][0];
        c.g[A3][0] = oc922_lw(c, SP, 0x68);
        call_ptr(c, OC922_RANK_ENTRY, 0x001c_3478)?;
        c.g[S1][0] = c.g[V0][0];
        // Row filled only with an entry and a non-empty list; otherwise the
        // three widgets are cleared to the empty string.
        let mut filled = false;
        if c.g[S1][0] != 0 {
            c.cycles += 2;
            if (c.g[S6][0] as i64) > 0 {
                filled = true;
            }
        }
        if filled {
            c.cycles += 2;
            if c.g[S0][0] != 0 {
                // points column: the time or the plain number
                c.cycles += 3;
                c.g[V0][0] = oc922_lw(c, S4, 0x6ec);
                let plain = c.g[V0][0] == 0;
                c.g[A2][0] = oc922_lw(c, SP, 0x60);
                if !plain {
                    c.cycles += 5;
                    c.g[A1][0] = oc922_add(c, SP, 0x20);
                    c.g[A0][0] = c.g[S4][0];
                    Callee {
                        addr: 0x001c_4f58,
                        f: ee::s03::cFEStateStats_FormatTime,
                    }
                    .call(c, 0x001c_34ac)?;
                    c.g[A1][0] = oc922_add(c, SP, 0x20);
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = 0x004a_0000;
                    c.g[A1][0] = oc922_add(c, V0, 0x12c0);
                    c.g[A0][0] = oc922_add(c, SP, 0x20);
                    call_ptr(c, OC922_SPRINTF, 0x001c_34c4)?;
                    c.g[A1][0] = oc922_add(c, SP, 0x20);
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, OC922_SET_STRING, 0x001c_34d0)?;
            }
            // persona (name) column
            c.cycles += 2;
            c.g[A0][0] = c.g[S3][0];
            if c.g[S3][0] != 0 {
                c.cycles += 2;
                c.g[A1][0] = oc922_add(c, S1, 0x8);
                call_ptr(c, OC922_SET_STRING, 0x001c_34e0)?;
            }
            // results column: the rank, "%3d"
            c.cycles += 2;
            let no_results = c.g[S2][0] == 0;
            c.g[V0][0] = 0x004a_0000;
            if !no_results {
                c.cycles += 9;
                c.g[A2][0] = oc922_lw(c, S1, 0x4);
                c.g[A0][0] = c.g[FP][0];
                c.g[A1][0] = oc922_add(c, V0, 0x1c20);
                call_ptr(c, OC922_SPRINTF, 0x001c_34f8)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[FP][0];
                call_ptr(c, OC922_SET_STRING, 0x001c_3504)?;
                c.g[S7][0] = oc922_add(c, S7, 1);
            } else {
                c.cycles += 1;
                c.g[S7][0] = oc922_add(c, S7, 1);
            }
        } else {
            c.cycles += 9;
            c.g[A1][0] = oc922_add(c, GP, 0u32.wrapping_sub(0x1d60));
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, OC922_SET_STRING, 0x001c_3518)?;
            c.g[A1][0] = oc922_add(c, GP, 0u32.wrapping_sub(0x1d60));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, OC922_SET_STRING, 0x001c_3524)?;
            c.g[A1][0] = oc922_add(c, GP, 0u32.wrapping_sub(0x1d60));
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, OC922_SET_STRING, 0x001c_3530)?;
            c.cycles += 1;
            c.g[S7][0] = oc922_add(c, S7, 1);
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S7][0] as i64) < 10) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S5][0] = oc922_add(c, S5, 1);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x001c_33e0);
            }
            continue;
        }
        c.cycles += 1;
        frame.restore(c, 16, 0x100);
        break;
    }
    Ok(())
}

/// `cFEStateStats::refreshResults()` (0x001c3310): refill the multiplayer
/// results table of the stats screen. With a screen, it sizes the
/// `ResultsMenu` scroll list to the lobby's ranking count, picks the first
/// visible row (the stored one, or the last full page), then fills ten rows
/// (`personaItem%d`, `resultsItem%d`, `pointsItem%d`) from the ranking entries
/// (name, rank "%3d", score as a time or a number) or clears them to "".
pub fn oc922_stats_refresh_results(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x110);
        frame.save(c, 20, 0xc0);
        frame.save(c, 16, 0x100);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, 17, 0xf0);
        frame.save(c, 18, 0xe0);
        frame.save(c, 19, 0xd0);
        frame.save(c, 21, 0xb0);
        frame.save(c, 22, 0xa0);
        frame.save(c, 23, 0x90);
        frame.save(c, 30, 0x80);
        frame.save_ra(c, 0x70);
        c.g[V0][0] = oc922_lw(c, S4, 0x40);
        frame.restore(c, 16, 0x100);
        if c.g[V0][0] != 0 {
            oc922_rr_body(c, frame)?;
        }
        c.cycles += 11;
        frame.restore(c, 17, 0xf0);
        frame.restore(c, 18, 0xe0);
        frame.restore(c, 19, 0xd0);
        frame.restore(c, 20, 0xc0);
        frame.restore(c, 21, 0xb0);
        frame.restore(c, 22, 0xa0);
        frame.restore(c, 23, 0x90);
        frame.restore(c, 30, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFEStateStats_SetEventCourseVisible (0x001c4538)
// ---------------------------------------------------------------------------

/// One of the screen's widgets (`0x460000 + hash_lo` string, looked up with
/// `FindObject(screen, StrHash(..))`), and if it exists `widget->vfn_0x48(a1)`
/// (set visible). `a1` is the visibility (`s1`, or 0 for the last two).
fn oc922_secv_widget(c: &mut Cpu, hash_lo: u32, rets: [u32; 3], visible_from_s1: bool) -> Flow {
    c.cycles += 9;
    c.g[A0][0] = 0x0046_0000;
    c.g[A0][0] = oc922_add(c, A0, hash_lo);
    call_ptr(c, OC922_STR_HASH, rets[0])?;
    c.g[A0][0] = oc922_lw(c, S0, 0x40);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, OC922_FIND_OBJECT, rets[1])?;
    c.g[A2][0] = c.g[V0][0];
    let missing = c.g[A2][0] == 0;
    c.g[A1][0] = if visible_from_s1 { c.g[S1][0] } else { 0 };
    if !missing {
        c.cycles += 5;
        oc922_vcall(c, A2, 0x48, rets[2])?;
    }
    Ok(())
}

/// `cFEStateStats::setEventCourseVisible(self, visible)` (0x001c4538): with a
/// screen, set the eight event/course widgets (found by hash) visible or
/// hidden with `visible`; when hiding, two more widgets are set to 0 as well.
pub fn oc922_stats_set_event_course_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = oc922_lw(c, S0, 0x40);
        c.g[S1][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            oc922_secv_widget(c, 0x5ea8, [0x001c_4564, 0x001c_4570, 0x001c_4590], true)?;
            oc922_secv_widget(c, 0x5d30, [0x001c_459c, 0x001c_45a8, 0x001c_45c8], true)?;
            oc922_secv_widget(c, 0x5c78, [0x001c_45d4, 0x001c_45e0, 0x001c_4600], true)?;
            oc922_secv_widget(c, 0x5eb8, [0x001c_460c, 0x001c_4618, 0x001c_4638], true)?;
            oc922_secv_widget(c, 0x5d40, [0x001c_4644, 0x001c_4650, 0x001c_4670], true)?;
            oc922_secv_widget(c, 0x5c88, [0x001c_467c, 0x001c_4688, 0x001c_46a8], true)?;
            oc922_secv_widget(c, 0x5ec8, [0x001c_46b4, 0x001c_46c0, 0x001c_46e0], true)?;
            oc922_secv_widget(c, 0x5ed8, [0x001c_46ec, 0x001c_46f8, 0x001c_4718], true)?;
            c.cycles += 2;
            if c.g[S1][0] != 0 {
                frame.restore(c, 16, 0x20);
            } else {
                oc922_secv_widget(c, 0x5e10, [0x001c_472c, 0x001c_4738, 0x001c_4758], false)?;
                oc922_secv_widget(c, 0x5e20, [0x001c_4764, 0x001c_4770, 0x001c_4790], false)?;
                c.cycles += 1;
                frame.restore(c, 16, 0x20);
            }
        } else {
            c.cycles += 1;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cFEStateStats_FillEventList (0x001c47a8)
// ---------------------------------------------------------------------------

/// The two list-fill loops of `FillEventList`: clear the list box (`s1`), set
/// its first command, then for each table entry (`n` = `s3 + 1` of them, from
/// `0x470000 - table_sub`) add either the "all events" string id (entry equal
/// to 4) or the event's name. `rets` are the generated return addresses:
/// clear, first command, [string hash, add id, event name, add ascii].
fn oc922_fel_fill(
    c: &mut Cpu,
    head: u32,
    clear_ret: u32,
    cmd_ret: u32,
    s3: u32,
    table_sub: u32,
    rets: [u32; 4],
) -> Flow {
    c.cycles += 14;
    c.g[A0][0] = c.g[S1][0];
    c.g[S7][0] = 0x4;
    call_ptr(c, OC922_LIST_CLEAR, clear_ret)?;
    c.g[S6][0] = 0x0046_0000;
    c.g[V0][0] = oc922_lw(c, S1, 0x8);
    c.g[A1][0] = 0;
    c.g[S3][0] = s3 as u64;
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = oc922_add_regs(c, S1, A0);
    call_ptr(c, target, cmd_ret)?;
    c.g[V0][0] = 0x0047_0000;
    c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(table_sub));
    c.g[S0][0] = c.lw(c.g[S2][0] as u32);
    loop {
        c.cycles += 2;
        let other = c.g[S0][0] != c.g[S7][0];
        c.g[A0][0] = c.g[S4][0];
        if !other {
            c.cycles += 8;
            c.g[A0][0] = oc922_add(c, S6, 0x5ee8);
            call_ptr(c, OC922_STR_HASH, rets[0])?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0x4;
            call_ptr(c, OC922_LIST_ADD_STR_ID, rets[1])?;
            c.g[S2][0] = oc922_add(c, S2, 0x4);
        } else {
            c.cycles += 7;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, OC922_EVENT_NAME, rets[2])?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, OC922_LIST_ADD_ASCII, rets[3])?;
            c.g[S2][0] = oc922_add(c, S2, 0x4);
        }
        c.cycles += 3;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1));
        if (c.g[S3][0] as i64) >= 0 {
            c.g[S0][0] = c.lw(c.g[S2][0] as u32);
            if event_due(c) {
                deliver_event_here(c, head);
            }
            continue;
        }
        return Ok(());
    }
}

/// `cFEStateStats::fillEventList()` (0x001c47a8): fill the event list box of
/// the stats screen (found by hash). The display mode (+0x6e0) picks the
/// list: mode 0 clears it and sends it its first command with 1; mode 1 fills
/// it from the two-entry table and then runs `FillCourseList`; mode 2 does the
/// same with the four-entry table; other modes leave the list alone.
pub fn oc922_stats_fill_event_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x90);
        frame.save(c, 21, 0x30);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, 17, 0x70);
        c.g[A0][0] = 0x0046_0000;
        frame.save(c, 16, 0x80);
        frame.save(c, 18, 0x60);
        frame.save(c, 19, 0x50);
        frame.save(c, 20, 0x40);
        frame.save(c, 22, 0x20);
        frame.save(c, 23, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = oc922_add(c, A0, 0x5c78);
        call_ptr(c, OC922_STR_HASH, 0x001c_47e0)?;
        c.g[A0][0] = oc922_lw(c, S5, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, OC922_FIND_OBJECT, 0x001c_47ec)?;
        c.g[S1][0] = c.g[V0][0];
        frame.restore(c, 16, 0x80);
        // How the generated body leaves: `Done` pops s1 (+1 cycle) first,
        // `Quick` goes straight to the final restores.
        let mut pop_s1 = true;
        if c.g[S1][0] != 0 {
            c.cycles += 10;
            call_ptr(c, OC922_GET_BE, 0x001c_4800)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, OC922_GET_INTERFACE, 0x001c_480c)?;
            c.g[S4][0] = c.g[V0][0];
            c.g[V1][0] = oc922_lw(c, S5, 0x6e0);
            c.g[V0][0] = 0x1;
            let mode1 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            // 0 = clear only, 1 and 2 = fill, 3 = nothing
            let mut mode = 3u32;
            if mode1 {
                mode = 1;
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0x2;
                    c.cycles += 2;
                    let is2 = c.g[V1][0] == c.g[V0][0];
                    frame.restore(c, 16, 0x80);
                    if is2 {
                        mode = 2;
                    } else {
                        c.cycles += 2;
                        frame.restore(c, 17, 0x70);
                        pop_s1 = false;
                    }
                } else {
                    c.cycles += 2;
                    let is0 = c.g[V1][0] == 0;
                    frame.restore(c, 16, 0x80);
                    if is0 {
                        mode = 0;
                    } else {
                        c.cycles += 2;
                        frame.restore(c, 17, 0x70);
                        pop_s1 = false;
                    }
                }
            }
            match mode {
                0 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, OC922_LIST_CLEAR, 0x001c_4850)?;
                    c.g[V0][0] = oc922_lw(c, S1, 0x8);
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = oc922_add_regs(c, S1, A0);
                    call_ptr(c, target, 0x001c_4868)?;
                    frame.restore(c, 16, 0x80);
                }
                1 => {
                    oc922_fel_fill(
                        c,
                        0x001c_48a8,
                        0x001c_487c,
                        0x001c_489c,
                        1,
                        0x2468,
                        [0x001c_48b8, 0x001c_48c8, 0x001c_48d8, 0x001c_48e8],
                    )?;
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S5][0];
                    Callee {
                        addr: 0x001c_49c8,
                        f: ee::s03::cFEStateStats_FillCourseList,
                    }
                    .call(c, 0x001c_4900)?;
                    frame.restore(c, 16, 0x80);
                }
                2 => {
                    oc922_fel_fill(
                        c,
                        0x001c_4940,
                        0x001c_4914,
                        0x001c_4934,
                        3,
                        0x2478,
                        [0x001c_4950, 0x001c_4960, 0x001c_4970, 0x001c_4980],
                    )?;
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S5][0];
                    Callee {
                        addr: 0x001c_49c8,
                        f: ee::s03::cFEStateStats_FillCourseList,
                    }
                    .call(c, 0x001c_4998)?;
                    frame.restore(c, 16, 0x80);
                }
                _ => {}
            }
        }
        if pop_s1 {
            c.cycles += 1;
            frame.restore(c, 17, 0x70);
        }
        c.cycles += 9;
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
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Callee fakes act only when this magic word is in the machine's RAM
    /// (every fixture writes it); with it absent they run the body they replaced.
    pub(crate) const OC922_CFG: u32 = DATA + 0xf_d000;
    pub(crate) const OC922_MAGIC: u32 = 0x0922_7a5e;
    /// Test objects: every widget, list box, interface and screen pointer of
    /// the fixtures points at this object, whose vtable slots are fake methods.
    pub(crate) const OC922_OBJ: u32 = DATA + 0x4000;
    pub(crate) const OC922_VT: u32 = DATA + 0x5000;
    pub(crate) const OC922_ENTRY: u32 = DATA + 0x6000;
    pub(crate) const OC922_THIS: u32 = DATA + 0x8000;
    pub(crate) const OC922_WORLD: u32 = DATA + 0x7000;

    /// A callee fake that logs the call (and unwinds when the machine says so);
    /// with the gate off it runs the callee body it replaced.
    macro_rules! oc922_gated {
        ($fake:ident, $gen:path, $addr:expr) => {
            pub(crate) fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC922_CFG) != OC922_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    /// Like `oc922_gated!`, but the result is set by `$result(c, call_index)`
    /// after the call has been logged.
    macro_rules! oc922_gated_result {
        ($fake:ident, $gen:path, $addr:expr, $result:expr) => {
            pub(crate) fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC922_CFG) != OC922_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                }
                let n = c.bus.read32(fakes::LOG_COUNT);
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
                c.g[V0][0] = $result(c, n);
            }
        };
    }

    /// `OC922_OBJ` or 0, by a bit of the mask word at `CFG + off` picked by
    /// the call index.
    fn oc922_masked_object(c: &mut Cpu, n: u32, off: u32, object: u32) -> u64 {
        let mask = c.bus.read32(OC922_CFG + off);
        if (mask >> (n % 32)) & 1 != 0 {
            object as u64
        } else {
            0
        }
    }

    oc922_gated!(oc922_fake_hash, ee::s10::f_00317670__generated, 0x0031_7670);
    oc922_gated!(
        oc922_fake_sprintf,
        ee::s15::f_00417828__generated,
        0x0041_7828
    );
    oc922_gated!(
        oc922_fake_set_string,
        ee::s13::f_003a0c50__generated,
        0x003a_0c50
    );
    oc922_gated!(
        oc922_fake_set_text_id,
        ee::s13::f_003a1000__generated,
        0x003a_1000
    );
    oc922_gated!(
        oc922_fake_reset_scroll,
        ee::s13::f_0039b760__generated,
        0x0039_b760
    );
    oc922_gated!(
        oc922_fake_list_selected,
        ee::s13::f_0039a738__generated,
        0x0039_a738
    );
    oc922_gated!(
        oc922_fake_list_clear,
        ee::s13::f_0039a670__generated,
        0x0039_a670
    );
    oc922_gated!(
        oc922_fake_list_add_id,
        ee::s13::f_0039a568__generated,
        0x0039_a568
    );
    oc922_gated!(
        oc922_fake_list_add_ascii,
        ee::s13::f_0039a3f8__generated,
        0x0039_a3f8
    );
    oc922_gated!(
        oc922_fake_event_name,
        ee::s01::f_001453b8__generated,
        0x0014_53b8
    );
    oc922_gated!(
        oc922_fake_set_num_ai,
        ee::s01::f_00144d98__generated,
        0x0014_4d98
    );
    oc922_gated!(
        oc922_fake_start_quick,
        ee::s06::f_0025ee20__generated,
        0x0025_ee20
    );
    oc922_gated!(
        oc922_fake_cancel_quick,
        ee::s06::f_0025eed8__generated,
        0x0025_eed8
    );
    oc922_gated!(
        oc922_fake_net_action,
        ee::s03::f_001a8918__generated,
        0x001a_8918
    );
    oc922_gated!(
        oc922_fake_hint_visible,
        ee::s03::f_001c13e0__generated,
        0x001c_13e0
    );
    oc922_gated!(
        oc922_fake_options_enabled,
        ee::s03::f_001c17f8__generated,
        0x001c_17f8
    );
    oc922_gated!(
        oc922_fake_format_time,
        ee::s03::f_001c4f58__generated,
        0x001c_4f58
    );
    oc922_gated!(
        oc922_fake_fill_course,
        ee::s03::f_001c49c8__generated,
        0x001c_49c8
    );

    oc922_gated_result!(
        oc922_fake_find,
        ee::s13::f_0039d860__generated,
        0x0039_d860,
        |c: &mut Cpu, n| oc922_masked_object(c, n, 8, OC922_OBJ)
    );
    oc922_gated_result!(
        oc922_fake_rank_entry,
        ee::s06::f_0025f388__generated,
        0x0025_f388,
        |c: &mut Cpu, n| oc922_masked_object(c, n, 0xc, OC922_ENTRY)
    );
    oc922_gated_result!(
        oc922_fake_rank_count,
        ee::s06::f_0025f338__generated,
        0x0025_f338,
        |c: &mut Cpu, _n| sx(c.bus.read32(OC922_CFG + 4))
    );
    oc922_gated_result!(
        oc922_fake_get_interface,
        ee::s01::f_0014dd58__generated,
        0x0014_dd58,
        |_c: &mut Cpu, _n| OC922_OBJ as u64
    );
    oc922_gated_result!(
        oc922_fake_get_be,
        ee::s01::f_0014dc80__generated,
        0x0014_dc80,
        |_c: &mut Cpu, _n| OC922_OBJ as u64
    );

    /// Installs every fake in its callee's override slot, once per process.
    pub(crate) fn oc922_install_fakes() {
        macro_rules! install {
            ($slot:path, $fake:ident) => {
                fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
            };
        }
        install!(ee::s10::OVERRIDE_00317670, oc922_fake_hash);
        install!(ee::s15::OVERRIDE_00417828, oc922_fake_sprintf);
        install!(ee::s13::OVERRIDE_003A0C50, oc922_fake_set_string);
        install!(ee::s13::OVERRIDE_003A1000, oc922_fake_set_text_id);
        install!(ee::s13::OVERRIDE_0039B760, oc922_fake_reset_scroll);
        install!(ee::s13::OVERRIDE_0039A738, oc922_fake_list_selected);
        install!(ee::s13::OVERRIDE_0039A670, oc922_fake_list_clear);
        install!(ee::s13::OVERRIDE_0039A568, oc922_fake_list_add_id);
        install!(ee::s13::OVERRIDE_0039A3F8, oc922_fake_list_add_ascii);
        install!(ee::s01::OVERRIDE_001453B8, oc922_fake_event_name);
        install!(ee::s01::OVERRIDE_00144D98, oc922_fake_set_num_ai);
        install!(ee::s06::OVERRIDE_0025EE20, oc922_fake_start_quick);
        install!(ee::s06::OVERRIDE_0025EED8, oc922_fake_cancel_quick);
        install!(ee::s03::OVERRIDE_001A8918, oc922_fake_net_action);
        install!(ee::s03::OVERRIDE_001C13E0, oc922_fake_hint_visible);
        install!(ee::s03::OVERRIDE_001C17F8, oc922_fake_options_enabled);
        install!(ee::s03::OVERRIDE_001C4F58, oc922_fake_format_time);
        install!(ee::s03::OVERRIDE_001C49C8, oc922_fake_fill_course);
        install!(ee::s13::OVERRIDE_0039D860, oc922_fake_find);
        install!(ee::s06::OVERRIDE_0025F388, oc922_fake_rank_entry);
        install!(ee::s06::OVERRIDE_0025F338, oc922_fake_rank_count);
        install!(ee::s01::OVERRIDE_0014DD58, oc922_fake_get_interface);
        install!(ee::s01::OVERRIDE_0014DC80, oc922_fake_get_be);
    }

    /// A scrambled machine with the callee gate open, the shared test object
    /// (all its vtable slots are fake methods with a seed-chosen `this` delta),
    /// the ranking entry, the world and an unwind picked among `calls` fake calls.
    pub(crate) fn oc922_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC922_CFG, OC922_MAGIC);
        let mut mask = r.next_u32();
        if seed % 3 == 0 {
            mask |= r.next_u32();
        }
        if seed % 5 == 0 {
            mask = u32::MAX;
        }
        c.bus.write32(OC922_CFG + 8, mask);
        let entry_mask = if seed % 4 == 0 {
            u32::MAX
        } else {
            r.next_u32()
        };
        c.bus.write32(OC922_CFG + 0xc, entry_mask);
        c.bus.write32(OC922_OBJ + 0x8, OC922_VT);
        c.bus.write32(OC922_OBJ + 0xc, OC922_VT);
        for (i, off) in [0x8u32, 0x38, 0x40, 0x48, 0x58, 0x88]
            .into_iter()
            .enumerate()
        {
            c.bus.write16(OC922_VT + off, (0x10 * (seed % 3)) as u16);
            c.bus
                .write32(OC922_VT + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
        c.bus.write32(OC922_ENTRY + 4, r.next_u32() & 0xff);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), OC922_WORLD);
        c.bus.write32(OC922_WORLD + 0x78, DATA + 0x7800);
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x7100);
        r
    }

    /// `onWidgetEvent` of the quick-play popup: messages 5 (OK), 6 (back), 7
    /// and others, searching or idle, the widget being the OK button or not.
    #[test]
    fn net_quick_play_popup_on_widget_action_matches() {
        oc922_install_fakes();
        covered(
            ee::s03::f_001c1560__generated,
            oc922_net_quick_play_popup_on_widget_action,
            200,
            &|c, seed| {
                let mut r = oc922_machine(c, seed, 0x1c_1560, 30);
                c.bus.write32(OC922_THIS + 0x14, r.next_u32());
                for off in [0x20u32, 0x6e4, 0x6f4, 0x70c, 0x710, 0x714, 0x718, 0x71c] {
                    c.bus.write32(OC922_THIS + off, OC922_OBJ);
                }
                c.bus.write32(OC922_THIS + 0x6ec, (seed / 8) % 2);
                const MSGS: [u64; 10] = [5, 6, 7, 5, 6, 7, 0, 9, 0x1_0000_0005, 4];
                c.g[A0][0] = OC922_THIS as u64;
                c.g[A1][0] = if (seed / 2) % 4 == 0 {
                    (OC922_OBJ + 4) as u64
                } else {
                    OC922_OBJ as u64
                };
                c.g[A2][0] = MSGS[(seed % 10) as usize];
            },
        );
    }

    /// `refreshResults`: no screen, the scroll list resized or not, short and
    /// long ranking lists, the stored first row, entries present or missing,
    /// the score shown as a time or as a number.
    #[test]
    fn stats_refresh_results_matches() {
        oc922_install_fakes();
        covered(
            ee::s03::f_001c3310__generated,
            oc922_stats_refresh_results,
            240,
            &|c, seed| {
                let mut r = oc922_machine(c, seed, 0x1c_3310, 160);
                const COUNTS: [u32; 10] = [0, 1, 3, 10, 11, 12, 15, 25, 0xffff_ffff, 5];
                let count = COUNTS[(seed % 10) as usize];
                c.bus.write32(OC922_CFG + 4, count);
                c.bus.write8(
                    OC922_OBJ + 0x96,
                    if seed % 4 < 2 {
                        count as u8
                    } else {
                        count as u8 ^ 1
                    },
                );
                c.bus
                    .write32(OC922_THIS + 0x40, if seed % 8 == 7 { 0 } else { OC922_OBJ });
                c.bus.write32(OC922_THIS + 0x6e8, r.next_u32() % 20);
                c.bus.write32(OC922_THIS + 0x6ec, (seed / 3) % 2);
                c.g[A0][0] = OC922_THIS as u64;
            },
        );
    }

    /// `setEventCourseVisible`: no screen, widgets found or missing, shown
    /// (`a1 != 0`, early exit) and hidden (two more widgets).
    #[test]
    fn stats_set_event_course_visible_matches() {
        oc922_install_fakes();
        covered(
            ee::s03::f_001c4538__generated,
            oc922_stats_set_event_course_visible,
            200,
            &|c, seed| {
                let _ = oc922_machine(c, seed, 0x1c_4538, 24);
                c.bus
                    .write32(OC922_THIS + 0x40, if seed % 9 == 8 { 0 } else { OC922_OBJ });
                c.g[A0][0] = OC922_THIS as u64;
                c.g[A1][0] = [0u64, 1, 5, 0, 0xffff_ffff_0000_0000][(seed % 5) as usize];
            },
        );
    }

    /// `fillEventList`: no list, the display modes 0, 1, 2 and the others
    /// (negative included), table entries equal to 4 or not.
    #[test]
    fn stats_fill_event_list_matches() {
        oc922_install_fakes();
        covered(
            ee::s03::f_001c47a8__generated,
            oc922_stats_fill_event_list,
            240,
            &|c, seed| {
                let mut r = oc922_machine(c, seed, 0x1c_47a8, 20);
                c.bus.write32(
                    OC922_THIS + 0x40,
                    if seed % 10 == 9 { 0 } else { OC922_OBJ },
                );
                const MODES: [u32; 6] = [0, 1, 2, 3, 0xffff_ffff, 1];
                c.bus
                    .write32(OC922_THIS + 0x6e0, MODES[(seed % 6) as usize]);
                // The two event-id tables (0x46db88 four entries, 0x46db98 two).
                for i in 0..6u32 {
                    let v = if r.next_u32() % 3 == 0 {
                        4
                    } else {
                        r.next_u32() % 8
                    };
                    c.bus.write32(0x0046_db88 + 4 * i, v);
                }
                c.g[A0][0] = OC922_THIS as u64;
            },
        );
    }
}
