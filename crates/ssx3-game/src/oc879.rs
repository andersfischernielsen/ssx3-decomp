//! Staging module oc879 (wave wave251): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc879`.

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
        0x001b_10f8,
        online_select_on_widget_created,
        "cFEStateOnlineCharacterSelect_OnWidgetCreated",
    ),
    (
        0x001b_1280,
        online_select_on_widget_action,
        "cFEStateOnlineCharacterSelect_OnWidgetAction",
    ),
    (
        0x001b_1808,
        online_select_fill_player_info,
        "cFEStateOnlineCharacterSelect_FillPlayerInfo",
    ),
    (
        0x001b_1a08,
        online_select_shift_frame_layer,
        "cFEStateOnlineCharacterSelect_ShiftFrameLayer",
    ),
];

/// `cFEStateOnlineCharacterSelect_CharToListIndex` (0x1b0f28), a direct call.
const OC879_CHAR_TO_LIST_INDEX: Callee = Callee {
    addr: 0x001b_0f28,
    f: ee::s03::f_001b0f28,
};
/// `cFEStateOnlineCharacterSelect_ShiftFrameLayer` (0x1b1a08), a direct call.
const OC879_SHIFT_FRAME_LAYER: Callee = Callee {
    addr: 0x001b_1a08,
    f: ee::s03::f_001b1a08,
};
/// `cFEStateOnlineCharacterSelect_ShowCharacter` (0x1b1b10), a direct call.
const OC879_SHOW_CHARACTER: Callee = Callee {
    addr: 0x001b_1b10,
    f: ee::s03::f_001b1b10,
};
/// `cFEStateNet_GetPopup` (0x1a9c50), a direct call.
const OC879_NET_GET_POPUP: Callee = Callee {
    addr: 0x001a_9c50,
    f: ee::s03::f_001a9c50,
};
/// `cFEStateNet_ShowPopup` (0x1a9bc0), a direct call.
const OC879_NET_SHOW_POPUP: Callee = Callee {
    addr: 0x001a_9bc0,
    f: ee::s03::f_001a9bc0,
};
/// `cFEPopupConfirm_CfgSetOptionTextId` (0x1c57c0), a direct call.
const OC879_POPUP_CFG_OPTION_TEXT_ID: Callee = Callee {
    addr: 0x001c_57c0,
    f: ee::s03::f_001c57c0,
};
/// `cFEStateNet_onWidgetEvent` (0x1a8918), a direct call.
const OC879_NET_ON_WIDGET_ACTION: Callee = Callee {
    addr: 0x001a_8918,
    f: ee::s03::f_001a8918,
};

/// `void cFEStateOnlineCharacterSelect_OnWidgetCreated(self, widget)`
/// (0x1b10f8): fills the character list box of the online character select
/// (`cUIListBox_addEntryByAsciiString` for each character in the table), selects
/// the current entry and stores it on `self`.
pub fn online_select_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S3, 0x20);
        frame.save(c, S1, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        frame.save(c, S0, 0x50);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3818));
        c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001b_1128)?;
        let differs = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x0046_0000;
        'b_1268: {
            'b_1210: {
                if differs {
                    break 'b_1210;
                }
                c.cycles += 8;
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x14)).lw(c);
                c.g[V1][0] = 0xffff_ffff_ffff_fffe;
                c.g[A0][0] = 0x0044_0000;
                c.g[V0][0] &= c.g[V1][0];
                c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf68));
                Addr(r32(c, S1).wrapping_add(0x14)).sw(c, c.g[V0][0]);
                c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa));
                'l_1150: loop {
                    c.cycles += 17;
                    call_ptr(c, 0x0014_dc80, 0x001b_1158)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0014_dd58, 0x001b_1164)?;
                    c.g[A1][0] = Addr(r32(c, S0)).lb(c);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x0014_eec8, 0x001b_1174)?;
                    c.g[A2][0] = Addr(r32(c, S0)).lb(c);
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x0039_a3f8, 0x001b_1188)?;
                    let more = (c.g[S0][0] as i64) < (c.g[S2][0] as i64);
                    c.g[V1][0] = 0xffff_ffff_ffff_fffe;
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x001b_1150);
                        }
                        continue 'l_1150;
                    }
                    c.cycles += 31;
                    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x14)).lw(c);
                    c.g[A2][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] &= c.g[V1][0];
                    Addr(r32(c, S1).wrapping_add(0x14)).sw(c, c.g[V0][0]);
                    c.g[A0][0] = Addr(r32(c, A2).wrapping_add(0x48)).lh(c);
                    c.g[V0][0] = Addr(r32(c, A2).wrapping_add(0x4c)).lw(c);
                    let slot = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                    call_ptr(c, slot, 0x001b_11b8)?;
                    c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x78)).lw(c);
                    call_ptr(c, 0x0014_dd58, 0x001b_11c8)?;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014_7488, 0x001b_11d4)?;
                    c.g[A0][0] = c.g[V0][0];
                    Addr(r32(c, S3).wrapping_add(0x6e0)).sw(c, c.g[V0][0]);
                    OC879_CHAR_TO_LIST_INDEX.call(c, 0x001b_11e0)?;
                    c.g[A1][0] = c.g[V0][0] & 0xff;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0039_a7a8, 0x001b_11ec)?;
                    Addr(r32(c, S3).wrapping_add(0x6d0)).sw(c, c.g[S1][0]);
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0xc0)).lh(c);
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc4)).lw(c);
                    let slot = c.g[V1][0] as u32;
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                    call_ptr(c, slot, 0x001b_1208)?;
                    frame.restore(c, S0, 0x50);
                    break 'b_1268;
                }
            }
            c.cycles += 5;
            c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x38)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3830));
            call_ptr(c, 0x0031_7670, 0x001b_121c)?;
            let same = c.g[S0][0] == c.g[V0][0];
            frame.restore(c, S0, 0x50);
            if !same {
                break 'b_1268;
            }
            c.cycles += 17;
            c.g[A1][0] = 0x004a_0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1390));
            call_ptr(c, 0x003a_0c50, 0x001b_1234)?;
            c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0x8)).lw(c);
            c.g[A1][0] = c.g[SP][0];
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x58)).lh(c);
            let slot_sp = sp(c);
            c.swc1(0, slot_sp);
            Addr(sp(c).wrapping_add(0x4)).sw(c, 0);
            Addr(sp(c).wrapping_add(0x8)).sw(c, 0);
            Addr(sp(c).wrapping_add(0xc)).sw(c, 0);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x5c)).lw(c);
            let slot = c.g[V0][0] as u32;
            c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
            call_ptr(c, slot, 0x001b_1264)?;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateOnlineCharacterSelect_OnWidgetAction(self, widget, action)`
/// (0x1b1280). With no widget it does nothing. Action 5 shows the picked
/// character and action 6 the session popup (or clears the frame layer while
/// one is up); both end there. Action 9 and the actions from 7 up first refresh
/// the character row, and every other action goes on to the network state's
/// widget handler (`cFEStateNet_onWidgetEvent`).
pub fn online_select_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        let no_widget = c.g[S3][0] == 0;
        frame.save_ra(c, 0x0);
        'x: {
            'y: {
                'z: {
                    if no_widget {
                        break 'z;
                    }
                    'd: {
                        'f: {
                            'e: {
                                'a: {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x6;
                                    let is6 = c.g[S1][0] == c.g[V0][0];
                                    c.g[V0][0] = (c.g[S1][0] < 0x7) as u64;
                                    if is6 {
                                        break 'a;
                                    }
                                    'b: {
                                        'c: {
                                            c.cycles += 2;
                                            let not_small = c.g[V0][0] == 0;
                                            c.g[V0][0] = 0x5;
                                            if not_small {
                                                break 'c;
                                            }
                                            c.cycles += 2;
                                            let is5 = c.g[S1][0] == c.g[V0][0];
                                            c.g[A0][0] = c.g[S2][0];
                                            if is5 {
                                                break 'b;
                                            }
                                            c.cycles += 2;
                                            c.g[A1][0] = c.g[S3][0];
                                            break 'd;
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x9;
                                        let is9 = c.g[S1][0] == c.g[V0][0];
                                        c.g[A0][0] = 0x0046_0000;
                                        if is9 {
                                            break 'e;
                                        }
                                        c.cycles += 2;
                                        c.g[A0][0] = c.g[S2][0];
                                        break 'f;
                                    }
                                    // Tail of 'b (action 5): the picked character is shown.
                                    c.cycles += 3;
                                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0x6dc)).lw(c);
                                    let shown = c.g[V0][0] != 0;
                                    frame.restore(c, S0, 0x40);
                                    if shown {
                                        break 'y;
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(0x2dc)).lw(c);
                                    let no_list = c.g[V0][0] == 0;
                                    frame.restore(c, S1, 0x30);
                                    if no_list {
                                        break 'x;
                                    }
                                    'h: {
                                        c.cycles += 5;
                                        c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x6d0)).lw(c);
                                        call_ptr(c, 0x0039_a738, 0x001b_1300)?;
                                        c.g[V1][0] = Addr(r32(c, S2).wrapping_add(0x6e0)).lw(c);
                                        let same = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
                                        if same {
                                            break 'h;
                                        }
                                        c.cycles += 13;
                                        c.g[A1][0] = 0x1;
                                        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x78)).lw(c);
                                        call_ptr(c, 0x0014_dd58, 0x001b_1318)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A1][0] = 0;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A2][0] = 0;
                                        call_ptr(c, 0x0014_74a8, 0x001b_132c)?;
                                        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                                        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x8)).lh(c);
                                        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
                                        let slot = c.g[V1][0] as u32;
                                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                                        call_ptr(c, slot, 0x001b_1340)?;
                                    }
                                    c.cycles += 21;
                                    c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x6d0)).lw(c);
                                    c.g[S0][0] = Addr(r32(c, GP).wrapping_add(0x2dc)).lw(c);
                                    call_ptr(c, 0x0039_a738, 0x001b_134c)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x0026_5018, 0x001b_1358)?;
                                    c.g[V1][0] = Addr(r32(c, S2).wrapping_add(0x14)).lb(c);
                                    c.g[A1][0] = 0x1;
                                    Addr(r32(c, S2).wrapping_add(0x6d8)).sw(c, c.g[V1][0]);
                                    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x8)).lw(c);
                                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x40)).lh(c);
                                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x44)).lw(c);
                                    let slot = c.g[V1][0] as u32;
                                    c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
                                    call_ptr(c, slot, 0x001b_1378)?;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[A1][0] = 0x1;
                                    c.g[A2][0] = 0;
                                    OC879_SHIFT_FRAME_LAYER.call(c, 0x001b_1388)?;
                                    c.g[V0][0] = 0x1;
                                    Addr(r32(c, S2).wrapping_add(0x6dc)).sw(c, c.g[V0][0]);
                                    break 'z;
                                }
                                // 'a falls through to here when the popup path
                                // (action 9 with no widget state) is taken.
                                'g: {
                                    c.cycles += 3;
                                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0x6dc)).lw(c);
                                    let has_popup = c.g[V0][0] != 0;
                                    if has_popup {
                                        break 'g;
                                    }
                                    c.cycles += 35;
                                    c.g[A0][0] = c.g[S2][0];
                                    OC879_NET_GET_POPUP.call(c, 0x001b_13a8)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A0][0] = 0x0046_0000;
                                    c.g[V0][0] = 0x2;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x268));
                                    Addr(r32(c, S0).wrapping_add(0xbc)).sw(c, c.g[V0][0]);
                                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                    call_ptr(c, 0x0031_7670, 0x001b_13c4)?;
                                    c.g[V1][0] = 0x3;
                                    c.g[A0][0] = 0x0046_0000;
                                    Addr(r32(c, S0).wrapping_add(0xc0)).sw(c, c.g[V1][0]);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                    Addr(r32(c, S0).wrapping_add(0xc4)).sw(c, c.g[V0][0]);
                                    call_ptr(c, 0x0031_7670, 0x001b_13dc)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A1][0] = 0;
                                    c.g[A0][0] = c.g[S1][0];
                                    OC879_POPUP_CFG_OPTION_TEXT_ID.call(c, 0x001b_13ec)?;
                                    c.g[A0][0] = 0x0046_0000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                    call_ptr(c, 0x0031_7670, 0x001b_13f8)?;
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A1][0] = 0x1;
                                    OC879_POPUP_CFG_OPTION_TEXT_ID.call(c, 0x001b_1408)?;
                                    c.g[V0][0] = 0x838;
                                    c.g[V1][0] = 0x1;
                                    Addr(r32(c, S0).wrapping_add(0x150)).sw(c, c.g[V0][0]);
                                    c.g[A0][0] = c.g[S2][0];
                                    Addr(r32(c, S0).wrapping_add(0x168)).sw(c, c.g[V1][0]);
                                    c.g[A1][0] = c.g[S0][0];
                                    OC879_NET_SHOW_POPUP.call(c, 0x001b_1424)?;
                                    frame.restore(c, S0, 0x40);
                                    break 'y;
                                }
                                c.cycles += 14;
                                c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0x2dc)).lw(c);
                                call_ptr(c, 0x0026_5140, 0x001b_1434)?;
                                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x8)).lw(c);
                                c.g[A1][0] = 0;
                                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x40)).lh(c);
                                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x44)).lw(c);
                                let slot = c.g[V1][0] as u32;
                                c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
                                call_ptr(c, slot, 0x001b_144c)?;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = 0;
                                c.g[A2][0] = 0;
                                OC879_SHIFT_FRAME_LAYER.call(c, 0x001b_145c)?;
                                Addr(r32(c, S2).wrapping_add(0x6dc)).sw(c, 0);
                                break 'z;
                            }
                            // Tail of 'f (action 7 and up): the pick is shown.
                            c.cycles += 5;
                            c.g[S0][0] = Addr(r32(c, S3).wrapping_add(0x38)).lw(c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3818));
                            call_ptr(c, 0x0031_7670, 0x001b_1470)?;
                            if c.g[S0][0] != c.g[V0][0] {
                                c.g[A0][0] = c.g[S2][0];
                                break 'f;
                            }
                            c.cycles += 13;
                            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x6d0)).lw(c);
                            c.g[S0][0] = Addr(r32(c, GP).wrapping_add(0x2dc)).lw(c);
                            call_ptr(c, 0x0039_a738, 0x001b_1484)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0026_4f88, 0x001b_1490)?;
                            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0x6d0)).lw(c);
                            call_ptr(c, 0x0039_a738, 0x001b_1498)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = 0;
                            OC879_SHOW_CHARACTER.call(c, 0x001b_14a8)?;
                            c.g[A0][0] = c.g[S2][0];
                        }
                        c.cycles += 1;
                        c.g[A1][0] = c.g[S3][0];
                    }
                    c.cycles += 2;
                    c.g[A2][0] = c.g[S1][0];
                    OC879_NET_ON_WIDGET_ACTION.call(c, 0x001b_14b8)?;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x40);
            }
            c.cycles += 1;
            frame.restore(c, S1, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0x0);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateOnlineCharacterSelect_FillPlayerInfo(self)` (0x1b1808): looks
/// up the named widgets of the screen at `self+0x40` and sets their text and
/// values for the player rows (the widgets come from `cUIScreen_getObjectByHashName`,
/// the text from `cUIText_setUnicodeString`).
pub fn online_select_fill_player_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S4, 0x110);
        frame.save(c, S0, 0x150);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x140);
        frame.save(c, S2, 0x130);
        frame.save(c, S3, 0x120);
        frame.save_ra(c, 0x100);
        c.g[V0][0] = Addr(r32(c, S4).wrapping_add(0x40)).lw(c);
        let no_screen = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x150);
        'x: {
            if no_screen {
                break 'x;
            }
            'q: {
                c.cycles += 9;
                c.g[A0][0] = 0x0046_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3108));
                call_ptr(c, 0x0031_7670, 0x001b_1840)?;
                c.g[A0][0] = Addr(r32(c, S4).wrapping_add(0x40)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001b_184c)?;
                c.g[S3][0] = c.g[V0][0];
                let none = c.g[S3][0] == 0;
                c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0xc8)).lw(c);
                if none {
                    break 'q;
                }
                c.cycles += 9;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = Addr(r32(c, V0).wrapping_add(0x80)).lw(c);
                call_ptr(c, 0x003a_0c50, 0x001b_1864)?;
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x8)).lw(c);
                c.g[A1][0] = 0x1;
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
                let slot = c.g[V1][0] as u32;
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
                call_ptr(c, slot, 0x001b_187c)?;
            }
            'r: {
                c.cycles += 9;
                c.g[A0][0] = 0x0046_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x39a0));
                call_ptr(c, 0x0031_7670, 0x001b_1888)?;
                c.g[A0][0] = Addr(r32(c, S4).wrapping_add(0x40)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001b_1894)?;
                c.g[S3][0] = c.g[V0][0];
                let none = c.g[S3][0] == 0;
                c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0xc8)).lw(c);
                if none {
                    break 'r;
                }
                c.cycles += 9;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = Addr(r32(c, V0).wrapping_add(0x98)).lw(c);
                call_ptr(c, 0x003a_0c50, 0x001b_18ac)?;
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x8)).lw(c);
                c.g[A1][0] = 0x1;
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
                let slot = c.g[V1][0] as u32;
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
                call_ptr(c, slot, 0x001b_18c4)?;
            }
            's: {
                c.cycles += 9;
                c.g[A0][0] = 0x0046_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x34e8));
                call_ptr(c, 0x0031_7670, 0x001b_18d0)?;
                c.g[A0][0] = Addr(r32(c, S4).wrapping_add(0x40)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001b_18dc)?;
                c.g[S3][0] = c.g[V0][0];
                let none = c.g[S3][0] == 0;
                c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
                if none {
                    break 's;
                }
                c.cycles += 27;
                c.g[A0][0] = 0x0046_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3550));
                c.g[S2][0] = Addr(r32(c, GP).wrapping_sub(0xc8)).lw(c);
                c.g[S0][0] = Addr(r32(c, V0).wrapping_add(0x8c)).lw(c);
                c.g[S1][0] = Addr(r32(c, S0).wrapping_add(0x4)).lw(c);
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x20)).lh(c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x0031_7670, 0x001b_1908)?;
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0x24)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                let slot = c.g[V1][0] as u32;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, slot, 0x001b_1918)?;
                c.g[A3][0] = Addr(r32(c, S2).wrapping_add(0xb8)).lw(c);
                c.g[A1][0] = 0x004a_0000;
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a88));
                call_ptr(c, 0x002c_2718, 0x001b_1930)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x003a_0e90, 0x001b_193c)?;
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x8)).lw(c);
                c.g[A1][0] = 0x1;
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
                let slot = c.g[V1][0] as u32;
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
                call_ptr(c, slot, 0x001b_1954)?;
            }
            't: {
                c.cycles += 9;
                c.g[A0][0] = 0x0046_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x39b8));
                call_ptr(c, 0x0031_7670, 0x001b_1960)?;
                c.g[A0][0] = Addr(r32(c, S4).wrapping_add(0x40)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001b_196c)?;
                c.g[S3][0] = c.g[V0][0];
                let none = c.g[S3][0] == 0;
                c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
                if none {
                    break 't;
                }
                c.cycles += 27;
                c.g[A0][0] = 0x0046_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3550));
                c.g[S2][0] = Addr(r32(c, GP).wrapping_sub(0xc8)).lw(c);
                c.g[S0][0] = Addr(r32(c, V0).wrapping_add(0x8c)).lw(c);
                c.g[S1][0] = Addr(r32(c, S0).wrapping_add(0x4)).lw(c);
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x20)).lh(c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x0031_7670, 0x001b_1998)?;
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0x24)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                let slot = c.g[V1][0] as u32;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, slot, 0x001b_19a8)?;
                c.g[A3][0] = Addr(r32(c, S2).wrapping_add(0xcc)).lw(c);
                c.g[A1][0] = 0x004a_0000;
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1a88));
                call_ptr(c, 0x002c_2718, 0x001b_19c0)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x003a_0e90, 0x001b_19cc)?;
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(0x8)).lw(c);
                c.g[A1][0] = 0x1;
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
                let slot = c.g[V1][0] as u32;
                c.g[A0][0] = sx(r32(c, S3).wrapping_add(r32(c, A0)));
                call_ptr(c, slot, 0x001b_19e4)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x150);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore(c, S3, 0x120);
        frame.restore(c, S4, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    });
}

/// `void cFEStateOnlineCharacterSelect_ShiftFrameLayer(self, on, flag)`
/// (0x1b1a08): moves the 5-bit layer counter at bits 8..12 of the frame
/// widget's word at +0x14 by +2 when `on` is set and by -2 otherwise, after
/// the screen's frame widget is looked up and updated (only when `flag` is set).
pub fn online_select_shift_frame_layer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0x40)).lw(c);
        let no_screen = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        'x: {
            'y: {
                if no_screen {
                    break 'y;
                }
                'p: {
                    'q: {
                        c.cycles += 2;
                        let no_flag = c.g[A2][0] == 0;
                        c.g[A0][0] = 0x0046_0000;
                        if no_flag {
                            break 'q;
                        }
                        c.cycles += 27;
                        c.g[A0][0] = 0x0046_0000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x39d0));
                        call_ptr(c, 0x0031_7670, 0x001b_1a40)?;
                        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x40)).lw(c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_d860, 0x001b_1a4c)?;
                        c.g[A0][0] = 0x0046_0000;
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3830));
                        call_ptr(c, 0x0031_7670, 0x001b_1a5c)?;
                        c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x40)).lw(c);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_d860, 0x001b_1a68)?;
                        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x8)).lw(c);
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 0);
                        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x58)).lh(c);
                        let slot_sp = sp(c);
                        c.swc1(0, slot_sp);
                        Addr(sp(c).wrapping_add(0x4)).sw(c, 0);
                        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                        Addr(sp(c).wrapping_add(0x8)).sw(c, 0);
                        Addr(sp(c).wrapping_add(0xc)).sw(c, 0);
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x5c)).lw(c);
                        let slot = c.g[V0][0] as u32;
                        c.g[A1][0] = c.g[SP][0];
                        call_ptr(c, slot, 0x001b_1a98)?;
                        break 'p;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x39e0));
                    call_ptr(c, 0x0031_7670, 0x001b_1aa8)?;
                    c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001b_1ab4)?;
                    c.g[S1][0] = c.g[V0][0];
                }
                c.cycles += 2;
                let no_widget = c.g[S1][0] == 0;
                frame.restore(c, S0, 0x40);
                if no_widget {
                    break 'x;
                }
                'r: {
                    'u: {
                        c.cycles += 4;
                        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x14)).lw(c);
                        c.g[V0][0] = sx((c.g[A0][0] as u32) >> 8);
                        let off = c.g[S2][0] == 0;
                        c.g[V0][0] &= 0x1f;
                        if off {
                            break 'u;
                        }
                        c.cycles += 2;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                        break 'r;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
                }
                c.cycles += 6;
                c.g[V1][0] = 0xffff_ffff_ffff_e0ff;
                c.g[V0][0] &= 0x1f;
                c.g[V1][0] = c.g[A0][0] & c.g[V1][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
                c.g[V1][0] |= c.g[V0][0];
                Addr(r32(c, S1).wrapping_add(0x14)).sw(c, c.g[V1][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes::machine, DATA, GAME_GP};

    /// A leaf (`0x1406c0`, returns 0) for every indirect slot the paths call.
    const OC879_LEAF: u32 = 0x0014_06c0;

    /// The globals these paths read: a zeroed BE-library singleton, the world
    /// (with a list object at +0x78), the state object (+0x8c chain) and the
    /// overlay list.
    pub(crate) fn oc879_globals(c: &mut Cpu) {
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
        c.bus.write32(DATA + 0x1000 + 0x78, DATA + 0x1400);
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x1800);
        c.bus.write32(DATA + 0x1800 + 0x8c, DATA + 0x1900);
        c.bus.write32(DATA + 0x1900 + 0x4, DATA + 0x1a00);
        c.bus.write32(GAME_GP.wrapping_add(0x2dc), DATA + 0x1c00);
    }

    /// A sub-object whose table fields point at the leaf, so every indirect
    /// call through it returns at once.
    pub(crate) fn oc879_sub(c: &mut Cpu, a: u32) {
        c.bus.write16(a + 0x40, 0);
        c.bus.write16(a + 0x48, 0);
        c.bus.write16(a + 0x58, 0);
        c.bus.write16(a + 0xc0, 0);
        c.bus.write32(a + 0x44, OC879_LEAF);
        c.bus.write32(a + 0x4c, OC879_LEAF);
        c.bus.write32(a + 0x5c, OC879_LEAF);
        c.bus.write32(a + 0xc4, OC879_LEAF);
        c.bus.write32(a + 0x80, DATA + 0x3000);
        c.bus.write32(a + 0x98, DATA + 0x3000);
        c.bus.write32(a + 0xc, DATA + 0x3100);
        c.bus.write32(a + 0x14, 0x1234);
        c.bus.write32(a + 0x20, 0);
    }

    #[test]
    fn online_select_on_widget_created_matches() {
        check(
            ee::s03::f_001b10f8__generated,
            online_select_on_widget_created,
            24,
            &|c, seed| {
                let _ = machine(c, seed, 0x1b10f8, 0);
                oc879_globals(c);
                let obj = DATA + 0x100;
                let widget = DATA + 0x200;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = widget as u64;
                c.bus
                    .write32(widget + 0x38, if seed % 2 == 0 { 0 } else { 0x55 });
                c.bus.write32(widget + 0x14, (seed << 8) | 0x3);
                c.bus.write32(widget + 0x8, DATA + 0x400);
                oc879_sub(c, DATA + 0x400);
                oc879_sub(c, DATA + 0x2000);
            },
        );
    }

    #[test]
    fn online_select_on_widget_action_matches() {
        check(
            ee::s03::f_001b1280__generated,
            online_select_on_widget_action,
            48,
            &|c, seed| {
                let _ = machine(c, seed, 0x1b1280, 0);
                oc879_globals(c);
                let obj = DATA + 0x100;
                let widget = DATA + 0x200;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { widget as u64 };
                c.g[A2][0] = (seed % 10) as u64;
                c.bus.write32(obj + 0x40, DATA + 0x500);
                c.bus.write32(obj + 0x6d0, DATA + 0x2000);
                c.bus.write32(obj + 0x6dc, seed % 2);
                c.bus.write32(obj + 0x6e0, seed % 3);
                c.bus.write32(obj + 0x14, 0x1234);
                c.bus.write32(widget + 0x8, DATA + 0x400);
                c.bus.write32(widget + 0x38, seed % 4);
                oc879_sub(c, DATA + 0x400);
                oc879_sub(c, DATA + 0x2000);
                oc879_sub(c, DATA + 0x500);
            },
        );
    }

    #[test]
    fn online_select_fill_player_info_matches() {
        check(
            ee::s03::f_001b1808__generated,
            online_select_fill_player_info,
            24,
            &|c, seed| {
                let _ = machine(c, seed, 0x1b1808, 0);
                oc879_globals(c);
                let obj = DATA + 0x100;
                c.g[A0][0] = obj as u64;
                c.bus
                    .write32(obj + 0x40, if seed % 3 == 0 { 0 } else { DATA + 0x500 });
                oc879_sub(c, DATA + 0x500);
                oc879_sub(c, DATA + 0x1400);
                oc879_sub(c, DATA + 0x2000);
            },
        );
    }

    #[test]
    fn online_select_shift_frame_layer_matches() {
        check(
            ee::s03::f_001b1a08__generated,
            online_select_shift_frame_layer,
            24,
            &|c, seed| {
                let _ = machine(c, seed, 0x1b1a08, 0);
                oc879_globals(c);
                let obj = DATA + 0x100;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = ((seed / 2) % 2) as u64;
                c.bus
                    .write32(obj + 0x40, if seed % 4 == 0 { 0 } else { DATA + 0x500 });
                oc879_sub(c, DATA + 0x500);
                oc879_sub(c, DATA + 0x2000);
            },
        );
    }
}
