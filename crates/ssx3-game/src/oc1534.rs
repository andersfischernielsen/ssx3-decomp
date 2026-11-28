//! Staging module oc1534 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1534`.

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
    (0x002c_dd78, menu_sys_float_item_handle_input, "MenuSysFloatItem_HandleInput"),
    (0x002c_e488, menu_sys_text_edit_item_handle_input, "MenuSysTextEditItem_HandleInput"),
    (0x002c_e910, menu_sys_text_edit_item_draw, "MenuSysTextEditItem_Draw"),
    (0x002c_eee8, menu_sys_color_channel_item_measure, "MenuSysColorChannelItem_Measure"),
];

/// `int MenuSysFloatItem_HandleInput(FloatItem* self)` (0x2cdd78): the
/// float-editing menu row. Runs the base item input (a non-zero result or a
/// disabled item returns at once), then reads the stick (`right - left`,
/// dead zone +-0.1) into a direction -1/0/+1. Holding the same direction
/// counts up `+0x2c` (a new direction resets it, direction kept at `+0x30`);
/// the step grows with the square of the hold count and is divided by 1/5/20/50
/// while the fine/slow modifiers are held. The value `*(+0x18)` (scaled by
/// `+0x24`, offset `+0x28`) is moved by `d * step * (max - min)`, clamped to
/// `[+0x1c, +0x20]` and stored back; then the menu's change callback (vtable
/// `+0x12c`, slot at `+0x20`/`+0x24`) runs with the item.
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code, unused_mut)]
pub fn menu_sys_float_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
'b_2cdf5c: {
'b_2cdf58: {
c.cycles += 13;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = c.g[A0][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cdd98);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
Callee { addr: 0x002ca2b0, f: ee::s09::MenuSysItem_HandleInput }.call(c, 0x002cdd98)?;
c.g[S1][0] = c.g[V0][0];
c.g[RA][0] = sx(0x002cdda4);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x002ca3b0, f: ee::s09::MenuSysItem_IsDisabled }.call(c, 0x002cdda4)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.g[S1][0];
if t {
break 'b_2cdf58;
} else {
c.cycles += 2;
if c.g[S1][0] != 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
break 'b_2cdf5c;
} else {
'b_2cde08: {
'b_2cddf0: {
c.cycles += 10;
c.g[RA][0] = sx(0x002cddbc);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x002ca428, f: ee::s09::MenuSysItem_GetRightAxis }.call(c, 0x002cddbc)?;
c.mov_s(20, 0);
c.g[RA][0] = sx(0x002cddc8);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x002ca408, f: ee::s09::MenuSysItem_GetLeftAxis }.call(c, 0x002cddc8)?;
c.sub_s(20, 20, 0);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fd8));
c.c_le_s(20, 0);
let t = c.fcc();

if t {
break 'b_2cddf0;
} else {
c.cycles += 5;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fd4));
c.c_le_s(0, 20);
let t = !c.fcc();
c.g[V1][0] = 0u64;
if t {
break 'b_2cde08;
} else {
break 'b_2cddf0;
}
}
}
c.cycles += 5;
c.mtc1(0, 0);
c.c_lt_s(20, 0);
let t = !c.fcc();
c.g[V1][0] = 0x1;
if t {
break 'b_2cde08;
} else {
c.cycles += 1;
c.g[V1][0] = 0xffffffffffffffff;
break 'b_2cde08;
}
}
'b_2cde28: {
'b_2cde24: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
if c.g[V1][0] != c.g[V0][0] {
c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0u64);
break 'b_2cde24;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
let t = true;
c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
if t {
break 'b_2cde28;
} else {
break 'b_2cde24;
}
}
}
c.cycles += 1;
c.sw((c.g[S0][0] as u32).wrapping_add(0x30), c.g[V1][0]);
break 'b_2cde28;
}
c.cycles += 2;
let t = c.g[V1][0] == 0u64;
c.g[V0][0] = c.g[S1][0];
if t {
break 'b_2cdf58;
} else {
'b_2cdea0: {
'b_2cde68: {
c.cycles += 6;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 21);
c.g[RA][0] = sx(0x002cde40);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x002ca488, f: ee::s09::MenuSysItem_IsSlowModHeld }.call(c, 0x002cde40)?;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_2cde68;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x002cde50);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x002ca4a8, f: ee::s09::MenuSysItem_IsFineModHeld }.call(c, 0x002cde50)?;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_2cde68;
} else {
c.cycles += 4;
c.g[AT][0] = 0x42480000;
c.mtc1(1, 21);
let t = true;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
if t {
break 'b_2cdea0;
} else {
break 'b_2cde68;
}
}
}
}
'b_2cde88: {
c.cycles += 4;
c.g[RA][0] = sx(0x002cde70);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x002ca488, f: ee::s09::MenuSysItem_IsSlowModHeld }.call(c, 0x002cde70)?;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_2cde88;
} else {
c.cycles += 4;
c.g[AT][0] = 0x41a00000;
c.mtc1(1, 21);
let t = true;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
if t {
break 'b_2cdea0;
} else {
break 'b_2cde88;
}
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x002cde90);
c.g[A0][0] = c.g[S0][0];
Callee { addr: 0x002ca4a8, f: ee::s09::MenuSysItem_IsFineModHeld }.call(c, 0x002cde90)?;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
if t {
break 'b_2cdea0;
} else {
c.cycles += 2;
c.g[AT][0] = 0x40a00000;
c.mtc1(1, 21);
break 'b_2cdea0;
}
}
'b_2cdf1c: {
c.cycles += 31;
c.g[AT][0] = 0x42480000;
c.mtc1(1, 3);
c.mult(0, 2, 2, 2);
c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x3fd0));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 4);
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x3fcc));
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.lwc1(7, (c.g[S0][0] as u32).wrapping_add(0x1c));
c.lwc1(9, (c.g[S0][0] as u32).wrapping_add(0x24));
c.lwc1(2, (c.g[V1][0] as u32));
c.mul_s(0, 0, 3);
c.lwc1(6, (c.g[S0][0] as u32).wrapping_add(0x20));
c.mul_s(2, 9, 2);
c.lwc1(8, (c.g[S0][0] as u32).wrapping_add(0x28));
c.sub_s(3, 6, 7);
c.div_s(0, 0, 21);
c.add_s(2, 2, 8);
c.mul_s(0, 0, 5);
c.max_s(0, 0, 4);
c.mul_s(1, 0, 1);
c.div_s(1, 1, 21);
c.mul_s(1, 20, 1);
c.mul_s(1, 1, 3);
c.add_s(2, 2, 1);
c.c_lt_s(2, 7);
if c.fcc() {
c.mov_s(2, 7);
break 'b_2cdf1c;
} else {
break 'b_2cdf1c;
}
}
'b_2cdf2c: {
c.cycles += 4;
c.c_lt_s(6, 2);
if c.fcc() {
c.mov_s(2, 6);
break 'b_2cdf2c;
} else {
break 'b_2cdf2c;
}
}
c.cycles += 11;
c.sub_s(0, 2, 8);
c.g[A1][0] = c.g[S0][0];
c.div_s(0, 0, 9);
c.swc1(0, (c.g[V1][0] as u32));
c.g[A2][0] = c.lw((c.g[S0][0] as u32));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x002cdf54);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x002cdf54)?;
c.g[V0][0] = c.g[S1][0];
break 'b_2cdf58;
}
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
break 'b_2cdf5c;
}
c.cycles += 6;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.pc = t; return Ok(());
    });
}

/// `int MenuSysTextEditItem_HandleInput(TextEditItem* self)` (0x2ce488): the
/// name-entry row. After the base input (non-zero result returns), edits the
/// 15 character buffer at `+0x18` with the cursor at `+0x28` from the menu
/// input code: 2/3 move the cursor (wrapping at 16), 4/5 step the character
/// under the cursor down/up until it is a letter, `_` or `!`, 9 inserts a `_`
/// at the cursor (shifting the tail right), 10 deletes at the cursor (shifting
/// left), 6 returns 3 (done), and any other code takes the typed key: code 8
/// is backspace, an accepted character is inserted at the cursor.
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code, unused_mut)]
pub fn menu_sys_text_edit_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
'b_2ce7fc: {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x002ce4ac);
c.g[S1][0] = c.g[A0][0];
Callee { addr: 0x002ca2b0, f: ee::s09::MenuSysItem_HandleInput }.call(c, 0x002ce4ac)?;
c.g[S4][0] = c.g[V0][0];
let t = c.g[S4][0] != 0u64;
c.g[V0][0] = c.g[S4][0];
if t {
break 'b_2ce7fc;
} else {
'b_2ce7f8: {
'b_2ce530: {
'b_2ce520: {
'b_2ce508: {
c.cycles += 14;
c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
c.g[RA][0] = sx(0x002ce4c4);
c.g[S3][0] = c.g[A0][0];
call_ptr(c, 0x00416810, 0x002ce4c4)?;
c.g[S2][0] = c.g[V0][0];
c.g[V1][0] = 0xf;
c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
c.g[A0][0] = c.g[S1][0];
c.g[V0][0] = ((c.g[S0][0] as i64) < 16i64) as u64;
c.g[RA][0] = sx(0x002ce4e0);
if c.g[V0][0] == 0 { c.g[S0][0] = c.g[V1][0]; }
Callee { addr: 0x002ca3e8, f: ee::s09::MenuSysItem_GetInputCode }.call(c, 0x002ce4e0)?;
c.g[V1][0] = c.g[V0][0];
c.g[V0][0] = 0x2;
if c.g[V1][0] != c.g[V0][0] {
c.g[V0][0] = 0x3;
break 'b_2ce508;
} else {
'b_2ce4fc: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
if c.g[S0][0] == 0u64 {
break 'b_2ce4fc;
} else {
break 'b_2ce4fc;
}
}
c.cycles += 3;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
let t = true;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if t {
break 'b_2ce520;
} else {
break 'b_2ce508;
}
}
}
c.cycles += 2;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = 0x4;
if t {
break 'b_2ce530;
} else {
'b_2ce51c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
if c.g[S0][0] == 0u64 {
break 'b_2ce51c;
} else {
break 'b_2ce51c;
}
}
c.cycles += 1;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
break 'b_2ce520;
}
}
c.cycles += 4;
c.div(0, 2, 16);
c.g[V1][0] = c.hi[0];
let t = true;
c.sw((c.g[S1][0] as u32).wrapping_add(0x28), c.g[V1][0]);
if t {
break 'b_2ce7f8;
} else {
break 'b_2ce530;
}
}
'b_2ce5a4: {
c.cycles += 2;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = 0x5;
if t {
break 'b_2ce5a4;
} else {
c.cycles += 6;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
c.g[A3][0] = 0x5f;
c.g[A2][0] = 0x21;
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
c.g[V0][0] = c.lbu((c.g[A0][0] as u32));
'l_2ce550: loop {
'b_2ce594: {
'b_2ce590: {
c.cycles += 8;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = sx((c.g[V0][0] as u32) << 24);
c.sb((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x61));
c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = 0u64;
if t {
break 'b_2ce590;
} else {
c.cycles += 4;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x41));
c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
if c.g[V0][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2ce594;
} else {
c.cycles += 2;
if c.g[V1][0] == c.g[A3][0] {
c.g[A1][0] = 0x1;
break 'b_2ce594;
} else {
c.cycles += 2;
let t = c.g[V1][0] != c.g[A2][0];

if t {
break 'b_2ce594;
} else {
break 'b_2ce590;
}
}
}
}
}
c.cycles += 1;
c.g[A1][0] = 0x1;
break 'b_2ce594;
}
c.cycles += 2;
if c.g[A1][0] == 0u64 {
c.g[V0][0] = c.lbu((c.g[A0][0] as u32));
if event_due(c) { deliver_event_here(c, 0x002ce550); } continue 'l_2ce550;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = c.g[S4][0];
if t {
break 'b_2ce7fc;
} else {
break 'b_2ce5a4;
}
}
}
}
}
'b_2ce61c: {
c.cycles += 2;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = 0x9;
if t {
break 'b_2ce61c;
} else {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
c.g[A3][0] = 0x5f;
c.g[A2][0] = 0x21;
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
c.g[V0][0] = c.lbu((c.g[A0][0] as u32));
'l_2ce5c8: loop {
'b_2ce60c: {
'b_2ce608: {
c.cycles += 8;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
c.g[V1][0] = sx((c.g[V0][0] as u32) << 24);
c.sb((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x61));
c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A1][0] = 0u64;
if t {
break 'b_2ce608;
} else {
c.cycles += 4;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x41));
c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
if c.g[V0][0] != 0u64 {
c.g[A1][0] = 0x1;
break 'b_2ce60c;
} else {
c.cycles += 2;
if c.g[V1][0] == c.g[A3][0] {
c.g[A1][0] = 0x1;
break 'b_2ce60c;
} else {
c.cycles += 2;
let t = c.g[V1][0] != c.g[A2][0];

if t {
break 'b_2ce60c;
} else {
break 'b_2ce608;
}
}
}
}
}
c.cycles += 1;
c.g[A1][0] = 0x1;
break 'b_2ce60c;
}
c.cycles += 2;
if c.g[A1][0] == 0u64 {
c.g[V0][0] = c.lbu((c.g[A0][0] as u32));
if event_due(c) { deliver_event_here(c, 0x002ce5c8); } continue 'l_2ce5c8;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = c.g[S4][0];
if t {
break 'b_2ce7fc;
} else {
break 'b_2ce61c;
}
}
}
}
}
'b_2ce678: {
c.cycles += 2;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = 0xa;
if t {
break 'b_2ce678;
} else {
c.cycles += 3;
c.g[V0][0] = ((c.g[S2][0] as i64) < 14i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.g[S4][0];
if t {
break 'b_2ce7fc;
} else {
'b_2ce668: {
c.cycles += 4;
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S2][0] as i64)) as u64;
if c.g[V0][0] == 0u64 {
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
break 'b_2ce668;
} else {
c.cycles += 2;
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S3][0] as u32)));
'l_2ce648: loop {
c.cycles += 7;
c.g[V0][0] = c.lbu((c.g[A0][0] as u32));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[S2][0] as i64)) as u64;
c.sb((c.g[A0][0] as u32).wrapping_add(0x1), c.g[V0][0]);
let t = c.g[V1][0] != 0u64;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
if t {
if event_due(c) { deliver_event_here(c, 0x002ce648); } continue 'l_2ce648;
} else {
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
break 'b_2ce668;
}
}
}
}
c.cycles += 4;
c.g[V1][0] = 0x5f;
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
let t = true;
c.sb((c.g[V0][0] as u32), c.g[V1][0]);
if t {
break 'b_2ce7f8;
} else {
break 'b_2ce678;
}
}
}
}
'b_2ce6d8: {
c.cycles += 2;
let t = c.g[V1][0] != c.g[V0][0];
c.g[V0][0] = 0x6;
if t {
break 'b_2ce6d8;
} else {
c.cycles += 2;
let t = c.g[S2][0] == 0u64;
c.g[V0][0] = c.g[S4][0];
if t {
break 'b_2ce7fc;
} else {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S2][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
if t {
break 'b_2ce7f8;
} else {
'b_2ce6cc: {
c.cycles += 4;
c.g[A0][0] = c.g[V1][0];
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.g[A2][0];
if t {
break 'b_2ce6cc;
} else {
c.cycles += 2;
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
'l_2ce6b0: loop {
c.cycles += 7;
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
c.sb((c.g[A1][0] as u32), c.g[V0][0]);
let t = c.g[V1][0] != 0u64;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
if t {
if event_due(c) { deliver_event_here(c, 0x002ce6b0); } continue 'l_2ce6b0;
} else {
break 'b_2ce6cc;
}
}
}
}
c.cycles += 3;
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[T0][0] as u32)));
let t = true;
c.sb((c.g[V0][0] as u32), 0u64);
if t {
break 'b_2ce7f8;
} else {
break 'b_2ce6d8;
}
}
}
}
}
'b_2ce6e8: {
c.cycles += 2;
if c.g[V1][0] != c.g[V0][0] {
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
break 'b_2ce6e8;
} else {
c.cycles += 2;
let t = true;
c.g[S4][0] = 0x3;
if t {
break 'b_2ce7f8;
} else {
break 'b_2ce6e8;
}
}
}
'b_2ce7f4: {
'b_2ce768: {
c.cycles += 10;
c.g[RA][0] = sx(0x002ce6f0);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
Callee { addr: 0x002cc250, f: ee::s09::MenuSys_GetKeyCode }.call(c, 0x002ce6f0)?;
c.g[V1][0] = c.lw((c.g[S1][0] as u32));
c.g[S0][0] = c.g[V0][0];
c.g[RA][0] = sx(0x002ce700);
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
Callee { addr: 0x002cc258, f: ee::s09::MenuSys_GetKeyChar }.call(c, 0x002ce700)?;
c.g[A2][0] = c.g[V0][0];
c.g[V0][0] = 0x8;
let t = c.g[S0][0] != c.g[V0][0];
c.g[V1][0] = sx((c.g[A2][0] as u32) << 24);
if t {
break 'b_2ce768;
} else {
c.cycles += 2;
let t = c.g[S2][0] == 0u64;
c.g[V0][0] = c.g[S4][0];
if t {
break 'b_2ce7fc;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
let t = (c.g[V0][0] as i64) <= 0;
c.g[A3][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
if t {
break 'b_2ce7f8;
} else {
'b_2ce754: {
c.cycles += 4;
c.g[A2][0] = c.g[V0][0];
c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.g[A3][0];
if t {
break 'b_2ce754;
} else {
c.cycles += 1;
c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S3][0] as u32)));
'l_2ce738: loop {
c.cycles += 7;
c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
c.sb((c.g[A1][0] as u32), c.g[V0][0]);
let t = c.g[V1][0] != 0u64;
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
if t {
if event_due(c) { deliver_event_here(c, 0x002ce738); } continue 'l_2ce738;
} else {
break 'b_2ce754;
}
}
}
}
c.cycles += 5;
c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.sb((c.g[V1][0] as u32), 0u64);
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
let t = true;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
if t {
break 'b_2ce7f4;
} else {
break 'b_2ce768;
}
}
}
}
}
'b_2ce7a4: {
'b_2ce7a0: {
c.cycles += 5;
c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x61));
c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A0][0] = 0u64;
if t {
break 'b_2ce7a0;
} else {
c.cycles += 4;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x41));
c.g[V0][0] = (c.g[V0][0] < 0x1au64) as u64;
if c.g[V0][0] != 0u64 {
c.g[A0][0] = 0x1;
break 'b_2ce7a4;
} else {
c.cycles += 3;
c.g[V0][0] = 0x5f;
let t = c.g[V1][0] == c.g[V0][0];
c.g[V0][0] = 0x21;
if t {
break 'b_2ce7a0;
} else {
c.cycles += 2;
let t = c.g[V1][0] != c.g[V0][0];

if t {
break 'b_2ce7a4;
} else {
break 'b_2ce7a0;
}
}
}
}
}
c.cycles += 1;
c.g[A0][0] = 0x1;
break 'b_2ce7a4;
}
c.cycles += 2;
let t = c.g[A0][0] == 0u64;
c.g[V0][0] = ((c.g[S2][0] as i64) < 14i64) as u64;
if t {
break 'b_2ce7f8;
} else {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;
c.g[V0][0] = c.g[S4][0];
if t {
break 'b_2ce7fc;
} else {
'b_2ce7e8: {
c.cycles += 4;
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[S2][0] as i64)) as u64;
if c.g[V0][0] == 0u64 {
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
break 'b_2ce7e8;
} else {
c.cycles += 1;
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S3][0] as u32)));
'l_2ce7c8: loop {
c.cycles += 7;
c.g[V0][0] = c.lbu((c.g[A0][0] as u32));
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[S2][0] as i64)) as u64;
c.sb((c.g[A0][0] as u32).wrapping_add(0x1), c.g[V0][0]);
let t = c.g[V1][0] != 0u64;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
if t {
if event_due(c) { deliver_event_here(c, 0x002ce7c8); } continue 'l_2ce7c8;
} else {
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
break 'b_2ce7e8;
}
}
}
}
c.cycles += 3;
c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sb((c.g[V1][0] as u32), c.g[A2][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
break 'b_2ce7f4;
}
}
}
c.cycles += 1;
c.sw((c.g[S1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
break 'b_2ce7f8;
}
c.cycles += 1;
c.g[V0][0] = c.g[S4][0];
break 'b_2ce7fc;
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
c.pc = t; return Ok(());
    });
}

/// `void MenuSysTextEditItem_Draw(TextEditItem* self, Row* row)` (0x2ce910):
/// draws the name-entry row: builds the text (with a trailing `_` caret when
/// the item is the page's selected one and the cursor is at the end), draws
/// the row background through the menu's slot `+0x40/+0x44`, the label
/// (`+0x14`) and the text with a drop shadow (four offset copies and the
/// colour quad through the draw context at `gp-0x854`), then restores the
/// font colour.
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code, unused_mut)]
pub fn menu_sys_text_edit_item_draw(c: &mut Cpu) {
    guest(c, |c| {
'b_2ce998: {
'b_2ce994: {
c.cycles += 24;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x180));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[S1][0] = c.g[A0][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
c.g[S2][0] = c.g[A1][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x110), c.g[RA][0]);
c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18));
c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x178));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x170));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x168));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x160));
c.g[RA][0] = sx(0x002ce94c);
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
Callee { addr: 0x002cbf30, f: ee::s09::MenuSysPage_GetSelectedItem }.call(c, 0x002ce94c)?;
c.g[V0][0] = c.g[S1][0] ^ c.g[V0][0];
c.g[A0][0] = c.g[SP][0];
c.g[A1][0] = c.g[S0][0];
c.g[RA][0] = sx(0x002ce960);
c.g[S3][0] = (c.g[V0][0] < 0x1u64) as u64;
call_ptr(c, 0x004166f4, 0x002ce960)?;
c.g[RA][0] = sx(0x002ce968);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x00416810, 0x002ce968)?;
let t = c.g[S3][0] == 0u64;
c.g[V1][0] = c.g[V0][0];
if t {
break 'b_2ce994;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
if c.g[V0][0] != c.g[V1][0] {
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
break 'b_2ce998;
} else {
c.cycles += 3;
c.g[V0][0] = ((c.g[V0][0] as i64) < 15i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = 0x4a0000;
if t {
break 'b_2ce994;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[SP][0];
c.g[RA][0] = sx(0x002ce994);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x39d0));
call_ptr(c, 0x004162d0, 0x002ce994)?;
break 'b_2ce994;
}
}
}
}
c.cycles += 1;
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
break 'b_2ce998;
}
'b_2ce9c0: {
'b_2ce9b8: {
c.cycles += 5;
c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x12c));
c.g[V0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x44));
let t = c.g[S3][0] == 0u64;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
if t {
break 'b_2ce9b8;
} else {
c.cycles += 3;
c.g[V0][0] = 0x4d0000;
let t = true;
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5380));
if t {
break 'b_2ce9c0;
} else {
break 'b_2ce9b8;
}
}
}
c.cycles += 2;
c.g[V0][0] = 0x4d0000;
c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5390));
break 'b_2ce9c0;
}
'b_2ce9ec: {
'b_2ce9e8: {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x002ce9cc);
c.g[A1][0] = c.g[S2][0];
call_ptr(c, t, 0x002ce9cc)?;
c.g[RA][0] = sx(0x002ce9d4);
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
Callee { addr: 0x002cbf30, f: ee::s09::MenuSysPage_GetSelectedItem }.call(c, 0x002ce9d4)?;
if c.g[S1][0] != c.g[V0][0] {
c.g[V0][0] = 0x4d0000;
break 'b_2ce9e8;
} else {
c.cycles += 3;
c.g[V0][0] = 0x4d0000;
let t = true;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
if t {
break 'b_2ce9ec;
} else {
break 'b_2ce9e8;
}
}
}
c.cycles += 1;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53b0));
break 'b_2ce9ec;
}
'b_2ceaa8: {
c.cycles += 34;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 22);
c.g[A0][0] = c.lw((c.g[S1][0] as u32));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[AT][0] = 0x40000000;
c.mtc1(1, 2);
c.lwc1(0, (c.g[V0][0] as u32));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x28));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x27));
c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x2f));
c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x28));
c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x47));
c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x40));
c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0x4f));
c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x48));
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x37));
c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x2f));
c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x28));
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
let t = c.g[A1][0] == 0u64;
c.g[V1][0] = c.lw((c.g[S1][0] as u32));
if t {
break 'b_2ceaa8;
} else {
c.cycles += 13;
c.lwc1(13, (c.g[S2][0] as u32).wrapping_add(0x4));
c.g[AT][0] = 0x40c00000;
c.mtc1(1, 0);
c.add_s(13, 13, 0);
c.lwc1(1, (c.g[S2][0] as u32));
c.g[AT][0] = 0x41800000;
c.mtc1(1, 12);
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
c.add_s(12, 1, 12);
c.sub_s(13, 13, 22);
c.g[RA][0] = sx(0x002ceaa4);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
call_ptr(c, 0x00391cb0, 0x002ceaa4)?;
c.g[V1][0] = c.lw((c.g[S1][0] as u32));
break 'b_2ceaa8;
}
}
'b_2cecd0: {
c.cycles += 27;
c.g[A1][0] = c.g[SP][0];
c.g[A2][0] = 0u64;
c.g[A3][0] = 0u64;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
c.g[AT][0] = 0x40c00000;
c.mtc1(1, 23);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
c.g[AT][0] = 0x41800000;
c.mtc1(1, 21);
c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cead8);
c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00391fb0, 0x002cead8)?;
c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.g[SP][0];
c.lwc1(12, (c.g[S2][0] as u32));
c.sub_s(1, 1, 0);
c.lwc1(13, (c.g[S2][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[S1][0] as u32));
c.add_s(13, 13, 23);
c.add_s(12, 12, 1);
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
c.sub_s(13, 13, 22);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
c.g[RA][0] = sx(0x002ceb0c);
c.sub_s(12, 12, 21);
call_ptr(c, 0x00391cb0, 0x002ceb0c)?;
let t = c.g[S3][0] == 0u64;
c.g[A2][0] = 0u64;
if t {
break 'b_2cecd0;
} else {
c.cycles += 41;
c.g[V1][0] = c.lw((c.g[S1][0] as u32));
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
c.g[A3][0] = 0u64;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002ceb38);
c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00391fb0, 0x002ceb38)?;
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1));
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x28));
c.mov_s(20, 0);
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
c.g[A2][0] = 0u64;
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A3][0] = 0u64;
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002ceb68);
c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00391fb0, 0x002ceb68)?;
c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x8));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.lwc1(4, (c.g[S2][0] as u32).wrapping_add(0xc));
c.g[V1][0] = 0xffffffffffffffff;
c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x4));
c.sub_s(0, 2, 0);
c.lwc1(3, (c.g[S2][0] as u32));
c.sub_s(2, 2, 20);
c.add_s(1, 1, 4);
c.g[AT][0] = 0x40800000;
c.mtc1(1, 5);
c.add_s(0, 3, 0);
c.g[V0][0] = 0x3;
c.add_s(3, 3, 2);
c.sub_s(1, 1, 23);
c.sub_s(7, 0, 21);
c.sub_s(12, 3, 21);
c.sub_s(8, 1, 22);
c.sub_s(6, 1, 5);
'l_2cebb8: loop {
c.cycles += 7;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
let t = c.g[V0][0] != c.g[V1][0];
c.g[T2][0] = 0xffffffffffffff83;
if t {
if event_due(c) { deliver_event_here(c, 0x002cebb8); } continue 'l_2cebb8;
} else {
c.cycles += 63;
c.mtc1(0, 4);
c.g[V0][0] = 0x7f;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 5);
c.g[AT][0] = 0x42fe0000;
c.mtc1(1, 1);
c.g[T3][0] = 0xffffffffffffffff;
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x24));
c.g[A1][0] = 0x4;
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x28));
c.g[A3][0] = 0u64;
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x2c));
c.mul_s(2, 2, 1);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x104));
c.mul_s(3, 3, 1);
c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x100));
c.mul_s(0, 0, 1);
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x108));
c.cvt_w_s(1, 2);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x10c));
c.cvt_w_s(1, 3);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
c.cvt_w_s(1, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
c.sw((c.g[SP][0] as u32).wrapping_add(0x5c), c.g[V0][0]);
c.lq(9, (c.g[SP][0] as u32).wrapping_add(0x100));
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x104));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x44));
c.lq(8, (c.g[SP][0] as u32).wrapping_add(0x100));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x40));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x48));
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x104));
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x100));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
c.sq(8, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sq(9, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x104));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x100));
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
c.g[V0][0] = c.g[V0][0] | 0x4;
c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.sh((c.g[V1][0] as u32).wrapping_add(0x10), c.g[T3][0]);
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x002cecd0);
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x002cecd0)?;
break 'b_2cecd0;
}
}
}
}
c.cycles += 20;
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
c.sw((c.g[SP][0] as u32).wrapping_add(0x30), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x34), 0u64);
c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x37));
c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x2f));
c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x28));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x110));
c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x178));
c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x170));
c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x168));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x160));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x180));
c.pc = t; return Ok(());
    });
}

/// `Vec4* MenuSysColorChannelItem_Measure(Vec4* out, Item* item)` (0x2ceee8):
/// formats the item's float value as text (`sprintf` of the value widened to
/// double), measures the label and the channel names with `Font_MeasureText`
/// (keeping the wider of each pair) and writes the row's size vector (the
/// widths plus a fixed padding) to `out`; returns `out` in `v0`.
#[allow(unused_labels, unused_assignments, unused_variables, unreachable_code, unused_mut)]
pub fn menu_sys_color_channel_item_measure(c: &mut Cpu) {
    guest(c, |c| {
'b_2cf138: {
'b_2cf110: {
'b_2cf080: {
'b_2cf04c: {
c.cycles += 75;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[S0][0] = 0x480000;
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6700));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[S3][0] = c.g[A1][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S2][0] = 0x480000;
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x7470));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
c.g[S6][0] = c.g[A0][0];
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[S5][0] = c.g[S2][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[RA][0] = sx(0x002cef4c);
c.lwc1(12, (c.g[V1][0] as u32));
call_ptr(c, 0x00413af8, 0x002cef4c)?;
c.g[A0][0] = c.g[S1][0];
c.g[A1][0] = c.g[S0][0];
c.g[RA][0] = sx(0x002cef5c);
c.g[A2][0] = c.g[V0][0];
call_ptr(c, 0x00417828, 0x002cef5c)?;
c.mtc1(0, 0);
c.g[A2][0] = 0u64;
c.g[A0][0] = c.lw((c.g[S3][0] as u32));
c.g[A3][0] = 0u64;
c.swc1(0, (c.g[SP][0] as u32));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[AT][0] = 0x41400000;
c.mtc1(1, 2);
c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x14));
c.cvt_s_w(0, 0);
c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x34));
c.mul_s(0, 0, 1);
c.add_s(0, 0, 2);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x124));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cefb4);
c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00391fb0, 0x002cefb4)?;
c.g[V1][0] = c.lw((c.g[S3][0] as u32));
c.g[A2][0] = 0u64;
c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x7470));
c.g[A3][0] = 0u64;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
c.mov_s(21, 0);
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cefdc);
c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00391fb0, 0x002cefdc)?;
c.g[V1][0] = c.lw((c.g[S3][0] as u32));
c.mov_s(20, 0);
c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
c.g[A2][0] = 0u64;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
c.g[A3][0] = 0u64;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cf004);
c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00391fb0, 0x002cf004)?;
c.c_lt_s(0, 20);
let t = !c.fcc();
c.g[A2][0] = 0u64;
if t {
break 'b_2cf04c;
} else {
c.cycles += 12;
c.g[V0][0] = c.lw((c.g[S3][0] as u32));
c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x7470));
c.g[A3][0] = 0u64;
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cf034);
c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00391fb0, 0x002cf034)?;
c.c_lt_s(21, 0);
if c.fcc() {
c.g[V1][0] = c.lw((c.g[S3][0] as u32));
break 'b_2cf080;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x7470));
if t {
break 'b_2cf110;
} else {
break 'b_2cf04c;
}
}
}
}
c.cycles += 12;
c.g[V0][0] = c.lw((c.g[S3][0] as u32));
c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
c.g[A3][0] = 0u64;
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cf06c);
c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00391fb0, 0x002cf06c)?;
c.c_lt_s(21, 0);
let t = !c.fcc();
c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x7470));
if t {
break 'b_2cf110;
} else {
c.cycles += 1;
c.g[V1][0] = c.lw((c.g[S3][0] as u32));
break 'b_2cf080;
}
}
'b_2cf0f0: {
'b_2cf0e8: {
c.cycles += 23;
c.g[A2][0] = 0u64;
c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x7470));
c.g[A3][0] = 0u64;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x7470));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cf0a4);
c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00391fb0, 0x002cf0a4)?;
c.g[V1][0] = c.lw((c.g[S3][0] as u32));
c.mov_s(20, 0);
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
c.g[A2][0] = 0u64;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
c.g[A3][0] = 0u64;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cf0cc);
c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00391fb0, 0x002cf0cc)?;
c.c_lt_s(0, 20);
let t = !c.fcc();
c.g[A2][0] = 0u64;
if t {
break 'b_2cf0e8;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S3][0] as u32));
let t = true;
c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x7470));
if t {
break 'b_2cf0f0;
} else {
break 'b_2cf0e8;
}
}
}
c.cycles += 2;
c.g[V0][0] = c.lw((c.g[S3][0] as u32));
c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
break 'b_2cf0f0;
}
c.cycles += 8;
c.g[A3][0] = 0u64;
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x124));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x58));
c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cf108);
c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00391fb0, 0x002cf108)?;
let t = true;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
if t {
break 'b_2cf138;
} else {
break 'b_2cf110;
}
}
c.cycles += 10;
c.g[V1][0] = c.lw((c.g[S3][0] as u32));
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
c.g[A2][0] = 0u64;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
c.g[A3][0] = 0u64;
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cf134);
c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00391fb0, 0x002cf134)?;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
break 'b_2cf138;
}
c.cycles += 36;
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[A2][0] = 0u64;
c.g[A3][0] = 0u64;
c.g[V1][0] = c.lw((c.g[S3][0] as u32));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x124));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x58));
c.lwc1(12, (c.g[A0][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cf15c);
c.lwc1(13, (c.g[A0][0] as u32).wrapping_add(0x3c));
call_ptr(c, 0x00391fb0, 0x002cf15c)?;
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[S6][0];
c.g[AT][0] = 0x43100000;
c.mtc1(1, 2);
c.add_s(1, 1, 0);
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.add_s(1, 1, 2);
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x70));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[SP][0] as u32));
c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
c.sdl(3, (c.g[S6][0] as u32).wrapping_add(0x7));
c.sdr(3, (c.g[S6][0] as u32));
c.sdl(4, (c.g[S6][0] as u32).wrapping_add(0xf));
c.sdr(4, (c.g[S6][0] as u32).wrapping_add(0x8));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x50));
c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xd8));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xe0));
c.pc = t; return Ok(());
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fake, fall_back, hook, machine};
    use crate::difftest::{check, check_lohi, fresh, run_generated, Rng, DATA, GAME_GP};
    use std::collections::BTreeSet;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's gated fakes (in the machine's own RAM).
    const OC1534_CTRL: u32 = DATA + 0x8_0000;
    const OC1534_MAGIC: u32 = 0x1534_a115;
    /// Forced result of the `MenuSysPage_GetSelectedItem` fake.
    const OC1534_SELECTED: u32 = OC1534_CTRL + 4;
    /// Float the `InputBinding_EvalFloat` fake returns for binding `a1`:
    /// `OC1534_AXES + 4 * a1`.
    const OC1534_AXES: u32 = OC1534_CTRL + 0x10;

    static OC1534_PREV_AXIS: AtomicUsize = AtomicUsize::new(0);
    static OC1534_PREV_SELECTED: AtomicUsize = AtomicUsize::new(0);
    static OC1534_PREV_DRAW_TEXT: AtomicUsize = AtomicUsize::new(0);
    static OC1534_PREV_MEASURE: AtomicUsize = AtomicUsize::new(0);

    /// `InputBinding_EvalFloat(binding, id)`: the float at `OC1534_AXES + 4 * id`
    /// in `f0` (what the stick and modifier reads of the menu input return).
    fn oc1534_fake_axis(c: &mut Cpu) {
        if c.bus.read32(OC1534_CTRL) != OC1534_MAGIC {
            return fall_back(c, &OC1534_PREV_AXIS, ee::s10::f_00320bf0__generated);
        }
        let id = c.g[A1][0] as u32;
        c.f[0] = c.bus.read32(OC1534_AXES + 4 * (id & 0xff));
        c.cycles += 9;
        ret(c);
    }

    /// `MenuSysPage_GetSelectedItem(menu)`: the item stored at `OC1534_SELECTED`.
    fn oc1534_fake_selected(c: &mut Cpu) {
        if c.bus.read32(OC1534_CTRL) != OC1534_MAGIC {
            return fall_back(c, &OC1534_PREV_SELECTED, ee::s09::f_002cbf30__generated);
        }
        let v = c.bus.read32(OC1534_SELECTED);
        c.g[V0][0] = sx(v);
        c.cycles += 7;
        ret(c);
    }

    /// `Font_DrawText`: a logged fake call (arguments compared).
    fn oc1534_fake_draw_text(c: &mut Cpu) {
        if c.bus.read32(OC1534_CTRL) != OC1534_MAGIC {
            return fall_back(c, &OC1534_PREV_DRAW_TEXT, ee::s12::f_00391cb0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fake(c, 0x0039_1cb0, ra);
    }

    /// `Font_MeasureText`: a logged fake call returning a pseudo-random float.
    fn oc1534_fake_measure(c: &mut Cpu) {
        if c.bus.read32(OC1534_CTRL) != OC1534_MAGIC {
            return fall_back(c, &OC1534_PREV_MEASURE, ee::s12::f_00391fb0__generated);
        }
        let ra = c.g[RA][0] as u32;
        fake(c, 0x0039_1fb0, ra);
    }

    fn oc1534_install() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00320BF0),
            oc1534_fake_axis,
            &OC1534_PREV_AXIS,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CBF30),
            oc1534_fake_selected,
            &OC1534_PREV_SELECTED,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391CB0),
            oc1534_fake_draw_text,
            &OC1534_PREV_DRAW_TEXT,
        );
        hook(
            addr_of_mut!(ee::s12::OVERRIDE_00391FB0),
            oc1534_fake_measure,
            &OC1534_PREV_MEASURE,
        );
    }

    /// Install the fakes and open their gate on this machine.
    fn oc1534_gate(c: &mut Cpu) {
        oc1534_install();
        c.bus.write32(OC1534_CTRL, OC1534_MAGIC);
    }

    fn w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    fn gp_word(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    /// Distinct cycle totals of the generated body over the seeds: the
    /// fixture must reach at least `min` paths.
    fn oc1534_paths(gen: GuestFn, seeds: u32, min: usize, setup: &dyn Fn(&mut Cpu, u32)) {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, gen);
            seen.insert(c.cycles - before);
        }
        assert!(
            seen.len() >= min,
            "only {} distinct paths (wanted {min}): {seen:?}",
            seen.len()
        );
    }

    // ------------------------------------------- MenuSysFloatItem_HandleInput

    const FLOAT_ITEM: u32 = DATA + 0x1000;
    const FLOAT_MENU: u32 = DATA + 0x2000;
    const FLOAT_INPUT: u32 = DATA + 0x3000;
    const FLOAT_VALUE: u32 = DATA + 0x4000;
    const FLOAT_VT: u32 = DATA + 0x5000;

    /// A float item whose input code, stick, modifiers, hold counter and
    /// value range come from the seed.
    fn float_input_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x2c_dd78, 1);
        oc1534_gate(c);
        w(c, gp_word(-0x3fd8), (-0.1f32).to_bits());
        w(c, gp_word(-0x3fd4), 0.1f32.to_bits());
        w(c, gp_word(-0x3fd0), 6.944_444_6e-5f32.to_bits());
        w(c, gp_word(-0x3fcc), 5.0e-5f32.to_bits());
        c.g[A0][0] = FLOAT_ITEM as u64;
        w(c, FLOAT_ITEM, FLOAT_MENU);
        w(c, FLOAT_MENU + 0x124, FLOAT_INPUT);
        w(c, FLOAT_MENU + 0x12c, FLOAT_VT);
        // Callback slot: a fake method with a (negative or positive) delta.
        c.bus.write16(FLOAT_VT + 0x20, (r.next_u32() & 0xfffc) as u16);
        w(c, FLOAT_VT + 0x24, crate::difftest::fakes::FAKE);
        let code = [0u32, 1, 7, 8, 2, 3, 4, 5, 6, 9][(seed % 10) as usize];
        w(c, FLOAT_INPUT + 0x48, code);
        w(c, FLOAT_INPUT + 0x5c, DATA + 0x3400);
        w(c, FLOAT_ITEM + 4, if seed % 7 == 0 { 8 } else { 0 });
        w(c, FLOAT_ITEM + 0x18, FLOAT_VALUE);
        w(c, FLOAT_VALUE, r.f32(100.0));
        w(c, FLOAT_ITEM + 0x1c, (-50.0f32).to_bits());
        w(c, FLOAT_ITEM + 0x20, 50.0f32.to_bits());
        w(
            c,
            FLOAT_ITEM + 0x24,
            (if seed % 5 == 0 { 1.0f32 } else { 0.5 + (seed % 4) as f32 }).to_bits(),
        );
        w(c, FLOAT_ITEM + 0x28, r.f32(5.0));
        w(c, FLOAT_ITEM + 0x2c, (seed * 37) % 300);
        w(
            c,
            FLOAT_ITEM + 0x30,
            if seed % 2 == 0 { 1 } else { 0xffff_ffff },
        );
        let (left, right) = match (seed / 5) % 6 {
            0 => (0.0f32, 0.0f32),
            1 => (0.0, 1.0),
            2 => (1.0, 0.0),
            3 => (0.03, 0.0),
            4 => (f32::from_bits(r.f32(1.0)), f32::from_bits(r.f32(1.0))),
            _ => (0.0, -0.8),
        };
        w(c, OC1534_AXES + 4 * 0x3e, left.to_bits());
        w(c, OC1534_AXES + 4 * 0x3f, right.to_bits());
        let slow = if (seed >> 2) & 1 == 1 { 1.0f32 } else { 0.0 };
        let fine = if (seed >> 3) & 1 == 1 { 1.0f32 } else { 0.0 };
        w(c, OC1534_AXES + 4 * 0x42, slow.to_bits());
        w(c, OC1534_AXES + 4 * 0x43, fine.to_bits());
    }

    /// The float row: base input result, disabled flag, dead zone, direction
    /// change versus repeat, the four slow/fine divisors, both clamps and the
    /// change callback (which also unwinds on some seeds).
    #[test]
    fn menu_sys_float_item_handle_input_matches() {
        check(
            ee::s09::f_002cdd78__generated,
            menu_sys_float_item_handle_input,
            160,
            &float_input_setup,
        );
        oc1534_paths(ee::s09::f_002cdd78__generated, 160, 8, &float_input_setup);
    }

    // ---------------------------------------- MenuSysTextEditItem_HandleInput

    const TEXT_ITEM: u32 = DATA + 0x1000;
    const TEXT_MENU: u32 = DATA + 0x2000;
    const TEXT_INPUT: u32 = DATA + 0x3000;

    fn text_char(r: &mut Rng) -> u8 {
        b"abcxyzABCXYZ_!09 ~"[(r.next_u32() % 18) as usize]
    }

    /// A 15 character name buffer with the cursor, and the input code / typed
    /// key from the seed.
    fn text_input_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x2c_e488, 0);
        c.g[A0][0] = TEXT_ITEM as u64;
        w(c, TEXT_ITEM, TEXT_MENU);
        w(c, TEXT_MENU + 0x124, TEXT_INPUT);
        let code = [2u32, 3, 4, 5, 6, 9, 10, 11, 12, 13, 0, 1, 7, 8][(seed % 14) as usize];
        w(c, TEXT_INPUT + 0x48, code);
        let key = if (seed / 14) % 4 == 0 {
            8
        } else {
            (r.next_u32() % 3) + 0x20
        };
        w(c, TEXT_INPUT + 0x4c, key);
        w(c, TEXT_INPUT + 0x50, text_char(&mut r) as u32);
        // Buffer: junk, then `len` characters and the terminator.
        for i in 0..16 {
            c.bus.write8(TEXT_ITEM + 0x18 + i, (r.next_u32() & 0x7f) as u8 | 0x20);
        }
        let len = (seed / 3 * 7 + seed) % 16;
        for i in 0..len {
            let ch = text_char(&mut r);
            c.bus.write8(TEXT_ITEM + 0x18 + i, ch);
        }
        c.bus.write8(TEXT_ITEM + 0x18 + len, 0);
        w(c, TEXT_ITEM + 4, 0);
        w(c, TEXT_ITEM + 0x28, (seed * 5 + seed / 7) % 16);
    }

    /// The name-entry row: base input result, cursor moves (wrap at 16), the
    /// letter step loops, insert, delete, done, backspace and typed keys.
    #[test]
    fn menu_sys_text_edit_item_handle_input_matches() {
        check_lohi(
            ee::s09::f_002ce488__generated,
            menu_sys_text_edit_item_handle_input,
            420,
            &text_input_setup,
        );
        oc1534_paths(ee::s09::f_002ce488__generated, 420, 12, &text_input_setup);
    }

    // ---------------------------------------------- MenuSysTextEditItem_Draw

    const DRAW_ITEM: u32 = DATA + 0x1000;
    const DRAW_ROW: u32 = DATA + 0x1800;
    const DRAW_MENU: u32 = DATA + 0x2000;
    const DRAW_INPUT: u32 = DATA + 0x3000;
    const DRAW_FONT: u32 = DATA + 0x3800;
    const DRAW_MENU_VT: u32 = DATA + 0x5000;
    const DRAW_GFX: u32 = DATA + 0x2_0000;

    /// The row's text buffer, selection, cursor, label, font, colour tables
    /// and the draw context (`gp-0x854`) from the seed.
    fn text_draw_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x2c_e910, 14);
        oc1534_gate(c);
        c.g[A0][0] = DRAW_ITEM as u64;
        c.g[A1][0] = DRAW_ROW as u64;
        w(c, DRAW_ITEM, DRAW_MENU);
        w(c, DRAW_MENU + 0x124, DRAW_INPUT);
        w(c, DRAW_MENU + 0x12c, DRAW_MENU_VT);
        c.bus.write16(DRAW_MENU_VT + 0x40, (r.next_u32() & 0xfffc) as u16);
        w(c, DRAW_MENU_VT + 0x44, crate::difftest::fakes::FAKE);
        w(c, DRAW_INPUT + 0x58, DRAW_FONT);
        w(c, DRAW_FONT + 0x14, seed % 9);
        w(c, DRAW_FONT + 0x34, r.f32(2.0));
        w(c, DRAW_FONT + 0x38, r.f32(2.0));
        w(c, DRAW_FONT + 0x3c, r.f32(2.0));
        for i in 0..4 {
            w(c, DRAW_ROW + 4 * i, r.f32(50.0));
        }
        // Text: `len` characters, cursor at the end unless the seed says so.
        let len = (seed / 2) % 16;
        for i in 0..len {
            let ch = text_char(&mut r);
            c.bus.write8(DRAW_ITEM + 0x18 + i, ch);
        }
        c.bus.write8(DRAW_ITEM + 0x18 + len, 0);
        let cursor = if seed % 4 == 1 { (len + 1) % 16 } else { len };
        w(c, DRAW_ITEM + 0x28, cursor);
        w(c, DRAW_ITEM + 0x14, if seed % 3 == 2 { 0 } else { DATA + 0x4000 });
        let selected = if seed % 3 == 0 { DRAW_ITEM } else { DRAW_ITEM + 0x40 };
        w(c, OC1534_SELECTED, selected);
        // Colour tables the item picks from (`0x4d5380..0x4d53bf`).
        for i in 0..16 {
            w(c, 0x004d_5380 + 4 * i, r.f32(1.0));
        }
        // Draw context: `gp-0x854` -> object with `+0xe84` (command word
        // block) and `+0x10d8` (an object whose slot `+0x238/+0x23c` runs).
        w(c, gp_word(-0x854), DRAW_GFX);
        w(c, DRAW_GFX + 0xe84, DRAW_GFX + 0x1000);
        w(c, DRAW_GFX + 0x10d8, DRAW_GFX + 0x2000);
        c.bus.write16(DRAW_GFX + 0x2000 + 0x238, (r.next_u32() & 0xfffc) as u16);
        w(c, DRAW_GFX + 0x2000 + 0x23c, crate::difftest::fakes::FAKE);
    }

    /// The name-entry row's drawing: selected or not, caret appended or not,
    /// label or none, with the shadow passes and the draw-context call.
    #[test]
    fn menu_sys_text_edit_item_draw_matches() {
        check(
            ee::s09::f_002ce910__generated,
            menu_sys_text_edit_item_draw,
            96,
            &text_draw_setup,
        );
        oc1534_paths(ee::s09::f_002ce910__generated, 96, 5, &text_draw_setup);
    }

    // ------------------------------------- MenuSysColorChannelItem_Measure

    /// A colour-channel item with a float value, a font and the output
    /// vector; `Font_MeasureText` is a logged fake returning random floats.
    fn color_measure_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x2c_eee8, 8);
        oc1534_gate(c);
        let item = DATA + 0x1000;
        let menu = DATA + 0x2000;
        let input = DATA + 0x3000;
        let font = DATA + 0x3800;
        let values = DATA + 0x4000;
        c.g[A0][0] = (DATA + 0x800) as u64;
        c.g[A1][0] = item as u64;
        w(c, item, menu);
        w(c, menu + 0x124, input);
        w(c, input + 0x58, font);
        w(c, font + 0x14, seed % 11);
        w(c, font + 0x34, r.f32(2.0));
        w(c, font + 0x38, r.f32(2.0));
        w(c, font + 0x3c, r.f32(2.0));
        w(c, item + 0x14, values);
        w(c, item + 0x18, seed % 4);
        for i in 0..4 {
            w(c, values + 4 * i, r.f32(255.0));
        }
    }

    /// Both orders of the two channel-name widths decide which label width is
    /// kept; the fake's random floats take every branch over the seeds.
    #[test]
    fn menu_sys_color_channel_item_measure_matches() {
        check(
            ee::s09::f_002ceee8__generated,
            menu_sys_color_channel_item_measure,
            120,
            &color_measure_setup,
        );
        oc1534_paths(
            ee::s09::f_002ceee8__generated,
            120,
            3,
            &color_measure_setup,
        );
    }
}
