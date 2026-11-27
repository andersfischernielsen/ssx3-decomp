//! Staging module oc973 (wave wave258): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc973`.

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
    (0x001c_b7f0, keyboard_popup_move_up, "cKeyboardPopup_MoveUp"),
    (0x001d_9f38, fe_state_net_help_popup_on_widget_created, "cFEStateNetHelpPopup_OnWidgetCreated"),
    (0x001e_3d58, ov_screen_message_center_init, "OVScreenMessageCenter_Init"),
    (0x001e_4458, ov_screen_message_center_update_button_text, "OVScreenMessageCenter_UpdateButtonText"),
    (0x001e_4658, ov_screen_message_center_update_scroll_arrows, "OVScreenMessageCenter_UpdateScrollArrows"),
    (0x001e_64a8, ov_screen_freestyle_standings_update, "OVScreenFreestyleStandings_Update"),
];

/// `cKeyboardPopup_MoveUp` (0x001cb7f0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn keyboard_popup_move_up(c: &mut Cpu) {
    if c.pc != 0x001cb7f0 {
        return ee::s04::f_001cb7f0__generated(c);
    }
    guest(c, |c| {
'b_1cb824: {
'b_1cb820: {
c.cycles += 10;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
let t = c.g[V1][0] == 0u64;
c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
if t {
break 'b_1cb820;
} else {
c.cycles += 2;
let t = true;
c.g[S1][0] = c.g[V0][0] ^ 0x1;
if t {
break 'b_1cb824;
} else {
break 'b_1cb820;
}
}
}
c.cycles += 1;
c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
break 'b_1cb824;
}
'b_1cb840: {
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xf));
c.g[V0][0] = (c.g[V0][0] < 0xcu64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
if t {
break 'b_1cb840;
} else {
c.cycles += 2;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6c));
c.g[S1][0] = c.g[V0][0] ^ 0x1;
break 'b_1cb840;
}
}
'b_1cb99c: {
'b_1cb998: {
'b_1cb8ac: {
'b_1cb8a8: {
c.cycles += 2;
let t = c.g[A1][0] == 0u64;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x430));
if t {
break 'b_1cb8a8;
} else {
c.cycles += 4;
c.sw((c.g[S0][0] as u32).wrapping_add(0x434), c.g[A0][0]);
c.g[V0][0] = 0x1;
let t = c.g[V1][0] != c.g[V0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x430), 0u64);
if t {
break 'b_1cb8a8;
} else {
'b_1cb88c: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x144));
c.g[V0][0] = 0x2;
if c.g[V1][0] == c.g[V0][0] {
c.g[A0][0] = c.g[S0][0];
break 'b_1cb88c;
} else {
'b_1cb880: {
c.cycles += 2;
let t = c.g[S1][0] == 0u64;

if t {
break 'b_1cb880;
} else {
c.cycles += 2;
let t = c.g[V1][0] == 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_1cb88c;
} else {
c.cycles += 2;
let t = true;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
if t {
break 'b_1cb8ac;
} else {
break 'b_1cb880;
}
}
}
}
c.cycles += 2;
if c.g[V1][0] != 0u64 {
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb8ac;
} else {
c.cycles += 1;
c.g[A0][0] = c.g[S0][0];
break 'b_1cb88c;
}
}
}
c.cycles += 5;
c.g[A1][0] = c.g[S2][0];
c.g[RA][0] = sx(0x001cb898);
c.g[A2][0] = 0u64;
Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001cb898)?;
if c.g[V0][0] == 0u64 {
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb8ac;
} else {
c.cycles += 2;
let t = true;
c.sw((c.g[S0][0] as u32).wrapping_add(0x140), c.g[S2][0]);
if t {
break 'b_1cb998;
} else {
break 'b_1cb8a8;
}
}
}
}
}
c.cycles += 1;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
break 'b_1cb8ac;
}
'b_1cb91c: {
'b_1cb914: {
'b_1cb8fc: {
'b_1cb8cc: {
c.cycles += 4;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x46));
c.g[V0][0] = (c.g[V0][0] < 0xeu64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_1cb8cc;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x001cb8c4);
c.g[A1][0] = c.g[S1][0];
Callee { addr: 0x001cbf20, f: ee::s04::cKeyboardPopup_MoveUpFromBottomRow }.call(c, 0x001cb8c4)?;
let t = true;

if t {
break 'b_1cb8fc;
} else {
break 'b_1cb8cc;
}
}
}
'b_1cb8ec: {
c.cycles += 4;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xe));
c.g[V0][0] = (c.g[V0][0] < 0xeu64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = (c.g[V1][0] < 0xeu64) as u64;
if t {
break 'b_1cb8ec;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x001cb8e4);
c.g[A1][0] = c.g[S1][0];
Callee { addr: 0x001cbdb0, f: ee::s04::cKeyboardPopup_MoveUpFromDigitRow }.call(c, 0x001cb8e4)?;
let t = true;

if t {
break 'b_1cb8fc;
} else {
break 'b_1cb8ec;
}
}
}
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[A0][0] = c.g[S0][0];
if t {
break 'b_1cb914;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x001cb8fc);
c.g[A1][0] = c.g[S1][0];
Callee { addr: 0x001cbb70, f: ee::s04::cKeyboardPopup_MoveUpFromTopRow }.call(c, 0x001cb8fc)?;
break 'b_1cb8fc;
}
}
c.cycles += 2;
if c.g[V0][0] != 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
break 'b_1cb99c;
} else {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
c.g[V0][0] = (c.g[V1][0] < 0xeu64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x46));
if t {
break 'b_1cb91c;
} else {
break 'b_1cb914;
}
}
}
c.cycles += 2;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
break 'b_1cb91c;
}
'b_1cb96c: {
c.cycles += 9;
c.sw((c.g[S0][0] as u32).wrapping_add(0x140), c.g[V0][0]);
c.g[V0][0] = 0x440000;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1390));
c.g[A0][0] = 0x2a;
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V1][0] = c.lb((c.g[V0][0] as u32));
if c.g[V1][0] != c.g[A0][0] {
c.g[A0][0] = c.g[S0][0];
break 'b_1cb96c;
} else {
c.cycles += 2;
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
'l_1cb948: loop {
c.cycles += 7;
c.g[A1][0] = c.g[V0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x140), c.g[V0][0]);
c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V0][0] = c.lb((c.g[V1][0] as u32));
let t = c.g[V0][0] == c.g[A0][0];
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
if t {
if event_due(c) { deliver_event_here(c, 0x001cb948); } continue 'l_1cb948;
} else {
c.cycles += 2;
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x140));
c.g[A0][0] = c.g[S0][0];
break 'b_1cb96c;
}
}
}
}
'b_1cb994: {
c.cycles += 4;
c.g[RA][0] = sx(0x001cb974);
c.g[A2][0] = c.g[SP][0];
Callee { addr: 0x001ce560, f: ee::s04::cKeyboardPopup_IsKeyEnabled }.call(c, 0x001cb974)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0x2;
if t {
break 'b_1cb994;
} else {
c.cycles += 6;
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x001cb988);
c.g[A1][0] = 0u64;
Callee { addr: 0x001cb7f0, f: ee::s04::cKeyboardPopup_MoveUp }.call(c, 0x001cb988)?;
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = (c.g[S1][0] < 0x1u64) as u64;
if t {
break 'b_1cb998;
} else {
break 'b_1cb994;
}
}
}
c.cycles += 1;
c.sw((c.g[S0][0] as u32).wrapping_add(0x144), c.g[V0][0]);
break 'b_1cb998;
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
break 'b_1cb99c;
}
c.cycles += 5;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.pc = t; return Ok(());
    });
}

/// `cFEStateNetHelpPopup_OnWidgetCreated` (0x001d9f38): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn fe_state_net_help_popup_on_widget_created(c: &mut Cpu) {
    if c.pc != 0x001d9f38 {
        return ee::s04::f_001d9f38__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x001d9f38;
    loop {
        match pc {
            0x001d9f38 => {
                c.cycles += 15;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S2][0] = c.g[A0][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[S1][0] = c.g[A1][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = 0x460000;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x79b0));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[RA][0] = sx(0x001d9f6c);
                c.g[S3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                call_ptr(c, 0x00317670, 0x001d9f6c)?;
                let t = c.g[S3][0] != c.g[V0][0]; 
                c.g[A0][0] = 0x4a0000;
                if t { pc = 0x001d9f7c; continue; }
                pc = 0x001d9f74; continue;
            }
            0x001d9f74 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0xa8), c.g[S1][0]);
                if t { pc = 0x001d9fa8; continue; }
                pc = 0x001d9f7c; continue;
            }
            0x001d9f7c => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001d9f84);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2040));
                call_ptr(c, 0x00317670, 0x001d9f84)?;
                
                if c.g[S3][0] != c.g[V0][0] { c.g[A0][0] = 0x460000; pc = 0x001d9f94; continue; }
                pc = 0x001d9f8c; continue;
            }
            0x001d9f8c => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0xac), c.g[S1][0]);
                if t { pc = 0x001d9fa8; continue; }
                pc = 0x001d9f94; continue;
            }
            0x001d9f94 => {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001d9f9c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x79c0));
                call_ptr(c, 0x00317670, 0x001d9f9c)?;
                
                if c.g[S3][0] != c.g[V0][0] { c.g[A0][0] = 0x4a0000; pc = 0x001d9fb8; continue; }
                pc = 0x001d9fa4; continue;
            }
            0x001d9fa4 => {
                c.cycles += 1;
                c.sw((c.g[S2][0] as u32).wrapping_add(0xb0), c.g[S1][0]);
                pc = 0x001d9fa8; continue;
            }
            0x001d9fa8 => {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.g[V0][0] | 0x80;
                let t = true; 
                c.sw((c.g[S1][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                if t { pc = 0x001da0f0; continue; }
                pc = 0x001d9fb8; continue;
            }
            0x001d9fb8 => {
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001d9fc4);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2048));
                call_ptr(c, 0x00317670, 0x001d9fc4)?;
                
                if c.g[S0][0] != c.g[V0][0] { c.g[A0][0] = 0x460000; pc = 0x001d9fd4; continue; }
                pc = 0x001d9fcc; continue;
            }
            0x001d9fcc => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0xd0), c.g[S1][0]);
                if t { pc = 0x001da0f0; continue; }
                pc = 0x001d9fd4; continue;
            }
            0x001d9fd4 => {
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001d9fe0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x79d0));
                call_ptr(c, 0x00317670, 0x001d9fe0)?;
                
                if c.g[S0][0] != c.g[V0][0] { c.g[A0][0] = 0x460000; pc = 0x001d9ff0; continue; }
                pc = 0x001d9fe8; continue;
            }
            0x001d9fe8 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0xd4), c.g[S1][0]);
                if t { pc = 0x001da0f0; continue; }
                pc = 0x001d9ff0; continue;
            }
            0x001d9ff0 => {
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001d9ffc);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x79e0));
                call_ptr(c, 0x00317670, 0x001d9ffc)?;
                
                if c.g[S0][0] != c.g[V0][0] { c.g[A0][0] = 0x460000; pc = 0x001da00c; continue; }
                pc = 0x001da004; continue;
            }
            0x001da004 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0xd8), c.g[S1][0]);
                if t { pc = 0x001da0f0; continue; }
                pc = 0x001da00c; continue;
            }
            0x001da00c => {
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001da018);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x79f0));
                call_ptr(c, 0x00317670, 0x001da018)?;
                
                if c.g[S0][0] != c.g[V0][0] { c.g[A0][0] = 0x460000; pc = 0x001da028; continue; }
                pc = 0x001da020; continue;
            }
            0x001da020 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0xdc), c.g[S1][0]);
                if t { pc = 0x001da0f0; continue; }
                pc = 0x001da028; continue;
            }
            0x001da028 => {
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001da034);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7a00));
                call_ptr(c, 0x00317670, 0x001da034)?;
                
                if c.g[S0][0] != c.g[V0][0] { c.g[A0][0] = 0x4a0000; pc = 0x001da044; continue; }
                pc = 0x001da03c; continue;
            }
            0x001da03c => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0xe0), c.g[S1][0]);
                if t { pc = 0x001da0f0; continue; }
                pc = 0x001da044; continue;
            }
            0x001da044 => {
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001da050);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2050));
                call_ptr(c, 0x00317670, 0x001da050)?;
                
                if c.g[S0][0] != c.g[V0][0] { c.g[A0][0] = 0x4a0000; pc = 0x001da060; continue; }
                pc = 0x001da058; continue;
            }
            0x001da058 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0xe8), c.g[S1][0]);
                if t { pc = 0x001da0f0; continue; }
                pc = 0x001da060; continue;
            }
            0x001da060 => {
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001da06c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2058));
                call_ptr(c, 0x00317670, 0x001da06c)?;
                
                if c.g[S0][0] != c.g[V0][0] { c.g[A0][0] = 0x4a0000; pc = 0x001da07c; continue; }
                pc = 0x001da074; continue;
            }
            0x001da074 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0xe4), c.g[S1][0]);
                if t { pc = 0x001da0f0; continue; }
                pc = 0x001da07c; continue;
            }
            0x001da07c => {
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001da088);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2060));
                call_ptr(c, 0x00317670, 0x001da088)?;
                
                if c.g[S0][0] != c.g[V0][0] { c.g[A0][0] = 0x4a0000; pc = 0x001da098; continue; }
                pc = 0x001da090; continue;
            }
            0x001da090 => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0xf0), c.g[S1][0]);
                if t { pc = 0x001da0f0; continue; }
                pc = 0x001da098; continue;
            }
            0x001da098 => {
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001da0a4);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2068));
                call_ptr(c, 0x00317670, 0x001da0a4)?;
                
                if c.g[S0][0] != c.g[V0][0] { c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xb4)); pc = 0x001da0b4; continue; }
                pc = 0x001da0ac; continue;
            }
            0x001da0ac => {
                c.cycles += 2;
                let t = true; 
                c.sw((c.g[S2][0] as u32).wrapping_add(0xec), c.g[S1][0]);
                if t { pc = 0x001da0f0; continue; }
                pc = 0x001da0b4; continue;
            }
            0x001da0b4 => {
                c.cycles += 3;
                c.g[S0][0] = 0u64;
                let t = true; 
                c.g[S4][0] = 0x460000;
                if t { pc = 0x001da0c4; continue; }
                pc = 0x001da0c0; continue;
            }
            0x001da0c0 => {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                pc = 0x001da0c4; continue;
            }
            0x001da0c4 => {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 7i64) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x7a10));
                if t { pc = 0x001da0f0; continue; }
                pc = 0x001da0d0; continue;
            }
            0x001da0d0 => {
                c.cycles += 7;
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x001da0dc);
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x00417828, 0x001da0dc)?;
                c.g[RA][0] = sx(0x001da0e4);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x001da0e4)?;
                
                if c.g[S3][0] != c.g[V0][0] { c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x4)); if back_edge_unwinds(c, 0x001da0c0) { return Ok(()); } pc = 0x001da0c0; continue; }
                pc = 0x001da0ec; continue;
            }
            0x001da0ec => {
                c.cycles += 1;
                c.sw((c.g[S2][0] as u32), c.g[S1][0]);
                pc = 0x001da0f0; continue;
            }
            0x001da0f0 => {
                c.cycles += 8;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x001d9f38, 0x001da110); return Ok(()); }
        }
    }
    });
}

/// `OVScreenMessageCenter_Init` (0x001e3d58): builds the message-center screen: adds it to the UI engine by its hash name, plays it, and refreshes the list, count, button and arrow widgets.
/// The body keeps the generated block structure: every register, cycle and call is as generated.
pub fn ov_screen_message_center_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e3da8: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2078));
            c.g[RA][0] = sx(0x001e3d7c);
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001e3d7c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001e3d90);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001e3d90)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x17c), c.g[V0][0]);
            if t {
                break 'b_1e3da8;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001e3da8);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001e3da8)?;
                break 'b_1e3da8;
            }
        }
        'b_1e3dcc: {
            'b_1e3dc4: {
                c.cycles += 5;
                c.g[RA][0] = sx(0x001e3db0);
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0020a380, 0x001e3db0)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x1;
                if t {
                    break 'b_1e3dc4;
                } else {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];

                    if t {
                        break 'b_1e3dcc;
                    } else {
                        break 'b_1e3dc4;
                    }
                }
            }
            c.cycles += 2;
            call_ptr(c, 0x0020d110, 0x001e3dcc)?;
            break 'b_1e3dcc;
        }
        c.cycles += 25;
        c.g[RA][0] = sx(0x001e3dd4);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x174), 0u64);
        Callee {
            addr: 0x001e33e8,
            f: ee::s04::MessageCenter_GetMessageCount,
        }
        .call(c, 0x001e3dd4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x4;
        c.g[RA][0] = sx(0x001e3de4);
        c.g[A1][0] = 0x1;
        Callee {
            addr: 0x001e2bb8,
            f: ee::s04::MessageCenter_CountGroupMessages,
        }
        .call(c, 0x001e3de4)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x144));
        c.g[RA][0] = sx(0x001e3df4);
        c.g[A1][0] = c.g[S0][0] & 0xff;
        call_ptr(c, 0x0039b760, 0x001e3df4)?;
        c.g[RA][0] = sx(0x001e3dfc);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x001e48b0,
            f: ee::s04::OVScreenMessageCenter_RefreshList,
        }
        .call(c, 0x001e3dfc)?;
        c.g[RA][0] = sx(0x001e3e04);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x001e4758,
            f: ee::s04::OVScreenMessageCenter_UpdateCountText,
        }
        .call(c, 0x001e3e04)?;
        c.g[RA][0] = sx(0x001e3e0c);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x001e4458,
            f: ee::s04::OVScreenMessageCenter_UpdateButtonText,
        }
        .call(c, 0x001e3e0c)?;
        c.g[RA][0] = sx(0x001e3e14);
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x001e4658,
            f: ee::s04::OVScreenMessageCenter_UpdateScrollArrows,
        }
        .call(c, 0x001e3e14)?;
        c.g[V0][0] = 0x1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1030), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenMessageCenter_UpdateButtonText` (0x001e4458): shows or hides the message-center buttons from the message count and the first row.
/// The body keeps the generated block structure: every register, cycle and call is as generated.
pub fn ov_screen_message_center_update_button_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e4568: {
            'b_1e4564: {
                'b_1e4550: {
                    'b_1e4534: {
                        'b_1e44b0: {
                            c.cycles += 12;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[S1][0] = c.g[A0][0];
                            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x144));
                            c.g[RA][0] = sx(0x001e4478);
                            c.g[S0][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x95));
                            Callee {
                                addr: 0x001e33e8,
                                f: ee::s04::MessageCenter_GetMessageCount,
                            }
                            .call(c, 0x001e4478)?;
                            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S0][0] as u32));
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                            let t = (c.g[V0][0] as i64) < 0;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14c));
                            if t {
                                break 'b_1e44b0;
                            } else {
                                c.cycles += 8;
                                c.g[RA][0] = sx(0x001e4490);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001e4878,
                                    f: ee::s04::OVScreenMessageCenter_ShowWidget,
                                }
                                .call(c, 0x001e4490)?;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                                c.g[RA][0] = sx(0x001e449c);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x001e4878,
                                    f: ee::s04::OVScreenMessageCenter_ShowWidget,
                                }
                                .call(c, 0x001e449c)?;
                                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x158));
                                let t = c.g[V0][0] == 0u64;
                                c.g[A0][0] = 0x470000;
                                if t {
                                    break 'b_1e4564;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fa0));
                                    if t {
                                        break 'b_1e4534;
                                    } else {
                                        break 'b_1e44b0;
                                    }
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[RA][0] = sx(0x001e44b8);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x001e4840,
                            f: ee::s04::OVScreenMessageCenter_HideWidget,
                        }
                        .call(c, 0x001e44b8)?;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x150));
                        c.g[RA][0] = sx(0x001e44c4);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x001e4840,
                            f: ee::s04::OVScreenMessageCenter_HideWidget,
                        }
                        .call(c, 0x001e44c4)?;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x158));
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0x4;
                        if t {
                            break 'b_1e4564;
                        } else {
                            c.cycles += 17;
                            c.g[A1][0] = sx(0u32.wrapping_sub(c.g[S0][0] as u32));
                            c.g[RA][0] = sx(0x001e44dc);
                            c.g[A2][0] = 0x1;
                            Callee {
                                addr: 0x001e2c80,
                                f: ee::s04::MessageCenter_GetGroupMessageId,
                            }
                            .call(c, 0x001e44dc)?;
                            c.g[A2][0] = 0x18;
                            c.g[V1][0] = 0x4c0000;
                            c.mult(0, 6, 2, 6);
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6c08));
                            c.g[A0][0] = 0x14;
                            c.g[A1][0] = 0x440000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1630));
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                            c.g[A2][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                            c.mult(0, 4, 6, 4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[A0][0] = 0x470000;
                            if t {
                                break 'b_1e4550;
                            } else {
                                'b_1e452c: {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001e451c);
                                    c.g[A0][0] = c.g[A2][0];
                                    Callee {
                                        addr: 0x001e3a30,
                                        f: ee::s04::MessageCenter_IsFlagSet,
                                    }
                                    .call(c, 0x001e451c)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A0][0] = 0x470000;
                                    if t {
                                        break 'b_1e452c;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f90));
                                        if t {
                                            break 'b_1e4534;
                                        } else {
                                            break 'b_1e452c;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 0x470000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f80));
                                break 'b_1e4534;
                            }
                        }
                    }
                    c.cycles += 7;
                    call_ptr(c, 0x00317670, 0x001e453c)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x158));
                    c.g[RA][0] = sx(0x001e4548);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001e4548)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1e4568;
                    } else {
                        break 'b_1e4550;
                    }
                }
                c.cycles += 5;
                c.g[RA][0] = sx(0x001e4558);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fa0));
                call_ptr(c, 0x00317670, 0x001e4558)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x158));
                c.g[RA][0] = sx(0x001e4564);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001e4564)?;
                break 'b_1e4564;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1e4568;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenMessageCenter_UpdateScrollArrows` (0x001e4658): shows the scroll arrows from the list position, through the arrow widget's vtable.
/// The body keeps the generated block structure: every register, cycle and call is as generated.
pub fn ov_screen_message_center_update_scroll_arrows(c: &mut Cpu) {
    guest(c, |c| {
        'b_1e46b4: {
            'b_1e4698: {
                c.cycles += 8;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S1][0] = c.g[A0][0];
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x174));
                let t = c.g[V0][0] != 0u64;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x15c));
                if t {
                    break 'b_1e4698;
                } else {
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e4690);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001e4690)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x102c));
                    if t {
                        break 'b_1e46b4;
                    } else {
                        break 'b_1e4698;
                    }
                }
            }
            c.cycles += 7;
            c.g[A1][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001e46b0);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001e46b0)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x102c));
            break 'b_1e46b4;
        }
        'b_1e46e8: {
            'b_1e46d4: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;

                if t {
                    break 'b_1e46d4;
                } else {
                    c.cycles += 6;
                    Callee {
                        addr: 0x001e33e8,
                        f: ee::s04::MessageCenter_GetMessageCount,
                    }
                    .call(c, 0x001e46c4)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 0x4;
                    let t = true;
                    c.g[A1][0] = 0x1;
                    if t {
                        break 'b_1e46e8;
                    } else {
                        break 'b_1e46d4;
                    }
                }
            }
            c.cycles += 5;
            Callee {
                addr: 0x001e33e8,
                f: ee::s04::MessageCenter_GetMessageCount,
            }
            .call(c, 0x001e46dc)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x4;
            c.g[A1][0] = 0u64;
            break 'b_1e46e8;
        }
        'b_1e4744: {
            'b_1e4728: {
                c.cycles += 8;
                Callee {
                    addr: 0x001e2bb8,
                    f: ee::s04::MessageCenter_CountGroupMessages,
                }
                .call(c, 0x001e46f0)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x174));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x160));
                if t {
                    break 'b_1e4728;
                } else {
                    c.cycles += 8;
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001e4720);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001e4720)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1e4744;
                    } else {
                        break 'b_1e4728;
                    }
                }
            }
            c.cycles += 7;
            c.g[A1][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001e4740);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001e4740)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1e4744;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenFreestyleStandings_Update` (0x001e64a8): one call to the standings update.
/// The body keeps the generated block structure: every register, cycle and call is as generated.
pub fn ov_screen_freestyle_standings_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        call_ptr(c, 0x0020e900, 0x001e64b8)?;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::fakes::FAKE;
    use crate::difftest::{check, elf_image, Rng, DATA, GAME_GP};
    use crate::game::fe::fepopup::tests::{
        oc930_machine, oc930_mail, oc930_popup, FK_IS_KEY_ENABLED, FK_MOVE_UP, OC930_SELF,
    };
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's callee fakes (in each machine's own RAM);
    /// per-fake call counters and result mailboxes sit behind it.
    pub(crate) const OC973_CTRL: u32 = DATA + 0x2d_0000;
    pub(crate) const OC973_MAGIC: u32 = 0x0973_0973;
    pub(crate) const OC973_SELF: u32 = DATA + 0x1000;
    pub(crate) const OC973_SUB: u32 = DATA + 0x3000;
    pub(crate) const OC973_WIDGET: u32 = DATA + 0x5000;
    pub(crate) const OC973_VT: u32 = DATA + 0x6000;

    // Fake indexes (counter and mailbox of each faked callee).
    pub(crate) const OK_MSG_COUNT: u32 = 0;
    pub(crate) const OK_COUNT_GROUP: u32 = 1;
    pub(crate) const OK_GROUP_MSG_ID: u32 = 6;
    pub(crate) const OK_IS_FLAG: u32 = 7;
    pub(crate) const OK_CREATE_SCREEN: u32 = 10;

    /// A fake callee: logs the call like `fakes::fake`, then (unless it
    /// unwound) returns `mail - 1` when the fixture put `mail` (non-zero) in
    /// the mailbox of this fake for this call number (16 calls, then wraps).
    pub(crate) fn oc973_fake_call(c: &mut Cpu, addr: u32, idx: u32) {
        let ra = c.g[RA][0] as u32;
        let n = c.bus.read32(OC973_CTRL + 0x20 + 4 * idx);
        c.bus.write32(OC973_CTRL + 0x20 + 4 * idx, n + 1);
        let mail = c.bus.read32(OC973_CTRL + 0x100 + 0x40 * idx + 4 * (n & 15));
        fakes::fake(c, addr, ra);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 1) as u64;
        }
    }

    macro_rules! oc973_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $idx:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC973_CTRL) != OC973_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc973_fake_call(c, $addr, $idx);
                }
            )*
            pub(crate) fn oc973_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc973_fakes!(
        (oc973_fake_msg_count, OC973_PREV_MSG_COUNT, ee::s04::OVERRIDE_001E33E8, ee::s04::f_001e33e8__generated, 0x001e33e8, 0),
        (oc973_fake_count_group, OC973_PREV_COUNT_GROUP, ee::s04::OVERRIDE_001E2BB8, ee::s04::f_001e2bb8__generated, 0x001e2bb8, 1),
        (oc973_fake_refresh_list, OC973_PREV_REFRESH_LIST, ee::s04::OVERRIDE_001E48B0, ee::s04::f_001e48b0__generated, 0x001e48b0, 2),
        (oc973_fake_update_count, OC973_PREV_UPDATE_COUNT, ee::s04::OVERRIDE_001E4758, ee::s04::f_001e4758__generated, 0x001e4758, 3),
        (oc973_fake_show_widget, OC973_PREV_SHOW_WIDGET, ee::s04::OVERRIDE_001E4878, ee::s04::f_001e4878__generated, 0x001e4878, 4),
        (oc973_fake_hide_widget, OC973_PREV_HIDE_WIDGET, ee::s04::OVERRIDE_001E4840, ee::s04::f_001e4840__generated, 0x001e4840, 5),
        (oc973_fake_group_msg_id, OC973_PREV_GROUP_MSG_ID, ee::s04::OVERRIDE_001E2C80, ee::s04::f_001e2c80__generated, 0x001e2c80, 6),
        (oc973_fake_is_flag, OC973_PREV_IS_FLAG, ee::s04::OVERRIDE_001E3A30, ee::s04::f_001e3a30__generated, 0x001e3a30, 7),
        (oc973_fake_hash, OC973_PREV_HASH, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated, 0x00317670, 8),
        (oc973_fake_set_text, OC973_PREV_SET_TEXT, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated, 0x003a1000, 9),
        (oc973_fake_create_screen, OC973_PREV_CREATE_SCREEN, ee::s13::OVERRIDE_00397CC0, ee::s13::f_00397cc0__generated, 0x00397cc0, 10),
        (oc973_fake_start_thread, OC973_PREV_START_THREAD, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039c870, 11),
        (oc973_fake_overlay_init, OC973_PREV_OVERLAY_INIT, ee::s05::OVERRIDE_0020A380, ee::s05::f_0020a380__generated, 0x0020a380, 12),
        (oc973_fake_pda_open, OC973_PREV_PDA_OPEN, ee::s05::OVERRIDE_0020D110, ee::s05::f_0020d110__generated, 0x0020d110, 13),
        (oc973_fake_reset_scroll, OC973_PREV_RESET_SCROLL, ee::s13::OVERRIDE_0039B760, ee::s13::f_0039b760__generated, 0x0039b760, 14),
        (oc973_fake_overlay_update, OC973_PREV_OVERLAY_UPDATE, ee::s05::OVERRIDE_0020E900, ee::s05::f_0020e900__generated, 0x0020e900, 15),
        (oc973_fake_button_text, OC973_PREV_BUTTON_TEXT, ee::s04::OVERRIDE_001E4458, ee::s04::f_001e4458__generated, 0x001e4458, 16),
        (oc973_fake_scroll_arrows, OC973_PREV_SCROLL_ARROWS, ee::s04::OVERRIDE_001E4658, ee::s04::f_001e4658__generated, 0x001e4658, 17),
    );

    /// Mailbox word of fake `idx` for call number `n`: the fake returns `value`.
    pub(crate) fn oc973_mail(c: &mut Cpu, idx: u32, n: u32, value: u32) {
        c.bus
            .write32(OC973_CTRL + 0x100 + 0x40 * idx + 4 * n, value + 1);
    }

    /// A machine with the game image and this module's fakes switched on.
    pub(crate) fn oc973_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc973_install_fakes();
        let r = machine(c, seed, salt, calls);
        elf_image(c);
        c.bus.write32(OC973_CTRL, OC973_MAGIC);
        r
    }

    /// `cKeyboardPopup::MoveUp(self, record)`: the popup's keyboard cursor
    /// (`+0x140`) moves up one row. Keys from every row group (top 0..13, digit
    /// row 14..27, bottom 70..83, and the rest), caps (`+0x68`/`+0x6c`), a
    /// recorded move (`record`) with the previous state (`+0x430`), a pending
    /// key state (`+0x144`) and enabled / disabled keys.
    #[test]
    fn c_keyboard_popup_move_up_matches() {
        const KEYS: [u32; 16] = [0, 5, 13, 14, 15, 20, 26, 27, 40, 56, 69, 70, 75, 83, 84, 90];
        fakes::covered(
            ee::s04::f_001cb7f0__generated,
            keyboard_popup_move_up,
            192,
            &|c, seed| {
                let mut r = oc930_machine(c, seed, 0x1cb7f0, 6);
                oc930_popup(c, &mut r, seed, KEYS[(seed % 16) as usize]);
                let s = OC930_SELF;
                c.bus.write32(s + 0x68, (seed / 16) % 2);
                c.bus.write32(s + 0x6c, (seed / 32) % 2);
                c.bus.write32(s + 0x430, (seed / 64) % 2);
                c.bus.write32(s + 0x144, r.next_u32() % 3);
                c.bus.write32(s + 0x434, r.next_u32() % 90);
                oc930_mail(c, FK_IS_KEY_ENABLED, 0, r.next_u32() & 1);
                oc930_mail(c, FK_IS_KEY_ENABLED, 1, r.next_u32() & 1);
                oc930_mail(c, FK_MOVE_UP, 0, r.next_u32() & 1);
                c.pc = 0x001c_b7f0;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (r.next_u32() & 1) as u64;
            },
        );
    }

    /// Reads the NUL-terminated string at `at` from the machine.
    fn oc973_cstr(c: &mut Cpu, at: u32) -> Vec<u8> {
        let mut v = Vec::new();
        loop {
            let b = c.bus.read8(at + v.len() as u32);
            if b == 0 {
                return v;
            }
            v.push(b);
        }
    }

    /// `cFEStateNetHelpPopup::OnWidgetCreated(self, widget)`: the widget's
    /// name hash picks which of the popup's widget slots (`+0xa8..+0xf0`, the
    /// first three flagged `0x80`) stores it, or one of seven `TextOption%d`
    /// rows; a hash that matches nothing stores nothing.
    #[test]
    fn c_fe_state_net_help_popup_on_widget_created_matches() {
        const NAMES: [u32; 12] = [
            0x4679b0, 0x4a2040, 0x4679c0, 0x4a2048, 0x4679d0, 0x4679e0, 0x4679f0, 0x467a00,
            0x4a2050, 0x4a2058, 0x4a2060, 0x4a2068,
        ];
        check(
            ee::s04::f_001d9f38__generated,
            fe_state_net_help_popup_on_widget_created,
            63,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d9f38, 0);
                elf_image(c);
                let (s, w) = (DATA + 0x1000, DATA + 0x2000);
                let k = seed % 21;
                let hash = if k < 12 {
                    let name = oc973_cstr(c, NAMES[k as usize]);
                    crate::game::hashvalue::str_hash(&name)
                } else if k < 19 {
                    let name = format!("TextOption{}", k - 12);
                    crate::game::hashvalue::str_hash(name.as_bytes())
                } else {
                    r.next_u32()
                };
                c.bus.write32(w + 0x38, hash);
                c.bus.write32(w + 0x14, r.next_u32());
                c.pc = 0x001d_9f38;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(w);
            },
        );
    }

    /// `OVScreenMessageCenter::Init(self)`: adds the screen by its hash name
    /// (playing it when created), the overlay init, the PDA template for mode 0
    /// or 1, then refreshes list, count, button text and arrows and sets the
    /// "initialised" global.
    #[test]
    fn ov_screen_message_center_init_matches() {
        fakes::covered(
            ee::s04::f_001e3d58__generated,
            ov_screen_message_center_init,
            96,
            &|c, seed| {
                let _ = oc973_machine(c, seed, 0x1e3d58, 12);
                let s = OC973_SELF;
                c.bus.write32(s + 0x10, DATA + 0x8000);
                c.bus.write32(s + 0x18, seed % 3);
                c.bus.write32(s + 0x144, OC973_SUB);
                oc973_mail(c, OK_CREATE_SCREEN, 0, if seed % 3 == 0 { 0 } else { DATA + 0x3800 });
                oc973_mail(c, OK_MSG_COUNT, 0, seed % 7);
                oc973_mail(c, OK_COUNT_GROUP, 0, seed % 3);
                c.pc = 0x001e_3d58;
                c.g[A0][0] = sx(s);
            },
        );
    }

    /// `OVScreenMessageCenter::UpdateButtonText(self)`: from the message count
    /// minus the first-row offset (`sub+0x95`), shows the buttons with the
    /// "view" text, or hides them and picks "collapse" / "expand" / "view" for the
    /// row's group (flag set or not, the group's first message or not).
    #[test]
    fn ov_screen_message_center_update_button_text_matches() {
        fakes::covered(
            ee::s04::f_001e4458__generated,
            ov_screen_message_center_update_button_text,
            192,
            &|c, seed| {
                let mut r = oc973_machine(c, seed, 0x1e4458, 7);
                let s = OC973_SELF;
                c.bus.write32(s + 0x144, OC973_SUB);
                c.bus.write8(OC973_SUB + 0x95, (seed % 4) as u8);
                c.bus.write32(s + 0x14c, OC973_WIDGET);
                c.bus.write32(s + 0x150, OC973_WIDGET + 0x100);
                c.bus
                    .write32(s + 0x158, if seed % 5 == 0 { 0 } else { OC973_WIDGET + 0x200 });
                let id = (seed / 3) % 6;
                let group = id % 3;
                // The message table entry's group and the group entry's first message.
                c.bus.write32(0x004c_6c08 + 0x18 * id + 4, group);
                let first = if (seed / 7) % 2 == 0 { id } else { id + 1 };
                c.bus.write32(0x0044_1630 + 0x14 * group + 4, first);
                oc973_mail(c, OK_MSG_COUNT, 0, r.next_u32() % 7);
                oc973_mail(c, OK_GROUP_MSG_ID, 0, id);
                oc973_mail(c, OK_IS_FLAG, 0, (seed / 2) % 2);
                c.pc = 0x001e_4458;
                c.g[A0][0] = sx(s);
            },
        );
    }

    /// `OVScreenMessageCenter::UpdateScrollArrows(self)`: the up arrow (vtable
    /// slot `+0x4c`, this adjusted by the halfword at `+0x48`) is shown when the
    /// list is scrolled, the down arrow when more than eight rows remain
    /// (message count plus the group messages, if the global flag allows).
    #[test]
    fn ov_screen_message_center_update_scroll_arrows_matches() {
        fakes::covered(
            ee::s04::f_001e4658__generated,
            ov_screen_message_center_update_scroll_arrows,
            192,
            &|c, seed| {
                let _ = oc973_machine(c, seed, 0x1e4658, 4);
                let s = OC973_SELF;
                c.bus.write32(s + 0x174, if seed % 3 == 0 { 0 } else { seed % 5 });
                c.bus.write32(s + 0x15c, OC973_WIDGET);
                c.bus.write32(s + 0x160, OC973_WIDGET + 0x100);
                c.bus.write32(OC973_WIDGET + 8, OC973_VT);
                c.bus.write32(OC973_WIDGET + 0x108, OC973_VT);
                c.bus.write16(OC973_VT + 0x48, (4 * (seed % 3)) as u16);
                c.bus.write32(OC973_VT + 0x4c, FAKE);
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x102c), (seed / 3) % 2);
                oc973_mail(c, OK_MSG_COUNT, 0, (seed / 2) % 14);
                oc973_mail(c, OK_COUNT_GROUP, 0, (seed / 5) % 3);
                c.pc = 0x001e_4658;
                c.g[A0][0] = sx(s);
            },
        );
    }

    /// `OVScreenFreestyleStandings::Update(self)`: one call to the overlay update.
    #[test]
    fn ov_screen_freestyle_standings_update_matches() {
        fakes::covered(
            ee::s04::f_001e64a8__generated,
            ov_screen_freestyle_standings_update,
            24,
            &|c, seed| {
                let _ = oc973_machine(c, seed, 0x1e64a8, 1);
                c.pc = 0x001e_64a8;
                c.g[A0][0] = sx(OC973_SELF);
            },
        );
    }
}
