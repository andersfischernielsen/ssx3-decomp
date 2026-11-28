//! Staging module oc1533 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1533`.

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
    (0x002c_b8c8, menu_sys_page_draw_scroll_arrow_down, "MenuSysPage_DrawScrollArrowDown"),
    (0x002c_bac8, menu_sys_page_draw_scroll_arrow_up, "MenuSysPage_DrawScrollArrowUp"),
    (0x002c_c8f0, menu_sys_enum_item_handle_input, "MenuSysEnumItem_HandleInput"),
    (0x002c_d5a0, menu_sys_int8_item_handle_input, "MenuSysInt8Item_HandleInput"),
    (0x002c_d9d8, menu_sys_int_item_handle_input, "MenuSysIntItem_HandleInput"),
];

/// `void MenuSysPage_DrawScrollArrowDown(MenuSysPage* self, float* rect)`
/// (0x2cb8c8, reference name `cMenu_renderUpArrow`): asks the page's vtable
/// (slot 8) to draw the background, marks the current draw layer, fills six
/// vertices with the arrow colour (global at 0x4d53a0, scaled to 0..128) and
/// positions them as a small triangle at the bottom of `rect` (x, y, w, h),
/// then submits them through the renderer's vtable (+0x238). The generated
/// body is the authority for registers and cycles.
pub fn menu_sys_page_draw_scroll_arrow_down(c: &mut Cpu) {
    guest(c, |c| {
c.cycles += 24;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
c.g[A2][0] = 0x4d0000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5380));
c.sd((c.g[SP][0] as u32).wrapping_add(0x130), c.g[RA][0]);
c.g[S0][0] = c.g[A1][0];
c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x12c));
c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x40));
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x44));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x002cb8f4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
call_ptr(c, t, 0x002cb8f4)?;
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A0][0] = 0xffffffffffffffff;
c.g[A1][0] = 0xffffffffffffff83;
c.g[A3][0] = 0xffffffffffffffff;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
c.g[A2][0] = 0x5;
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
c.g[V0][0] = c.g[V0][0] | 0x4;
c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
'l_2cb928: loop {
c.cycles += 7;
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
let t = c.g[A2][0] != c.g[A3][0];
c.g[V0][0] = 0x4d0000;
if t {
if event_due(c) { deliver_event_here(c, 0x002cb928); } continue 'l_2cb928;
} else {
c.cycles += 21;
c.g[AT][0] = 0x43000000;
c.mtc1(1, 0);
c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x53a0));
c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
c.g[T0][0] = 0x80;
c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
c.mul_s(1, 1, 0);
c.mul_s(3, 3, 0);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 4);
c.mul_s(2, 2, 0);
c.g[V1][0] = c.g[SP][0];
c.cvt_w_s(0, 1);
c.mfc1(7, 0);
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
c.cvt_w_s(0, 3);
c.mfc1(5, 0);
c.cvt_w_s(0, 2);
c.mfc1(4, 0);
c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]);
'l_2cb998: loop {
c.cycles += 10;
c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A1][0]);
c.sw((c.g[V1][0] as u32), 0u64);
c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]);
if event_due(c) { deliver_event_here(c, 0x002cb998); } continue 'l_2cb998;
} else {
c.cycles += 65;
c.lwc1(6, (c.g[S0][0] as u32).wrapping_add(0x4));
c.g[A1][0] = 0x6;
c.lwc1(3, (c.g[S0][0] as u32).wrapping_add(0xc));
c.g[A2][0] = c.g[SP][0];
c.g[AT][0] = 0x40800000;
c.mtc1(1, 1);
c.g[A3][0] = 0u64;
c.add_s(3, 6, 3);
c.lwc1(4, (c.g[S0][0] as u32));
c.g[AT][0] = 0x40c00000;
c.mtc1(1, 9);
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
c.sub_s(3, 3, 1);
c.g[AT][0] = 0x40400000;
c.mtc1(1, 2);
c.add_s(8, 4, 9);
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 1);
c.add_s(7, 4, 0);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 5);
c.mul_s(0, 0, 1);
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x124));
c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
c.sub_s(1, 3, 2);
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x120));
c.add_s(6, 6, 2);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x12c));
c.add_s(4, 4, 0);
c.sub_s(7, 7, 9);
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.add_s(2, 6, 2);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x124));
c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x124));
c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x124));
c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0x120));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x124));
c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x124));
c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x238));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x002cbab4);
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x002cbab4)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x130));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
c.pc = t; return Ok(());
}
}
}
}
    });
}

/// `void MenuSysPage_DrawScrollArrowUp(MenuSysPage* self, float* rect)`
/// (0x2cbac8, reference name `cMenu_renderDownArrow`): the mirror of
/// `MenuSysPage_DrawScrollArrowDown`, with the triangle at the top of `rect`.
pub fn menu_sys_page_draw_scroll_arrow_up(c: &mut Cpu) {
    guest(c, |c| {
c.cycles += 24;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x150));
c.g[A2][0] = 0x4d0000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x5380));
c.sd((c.g[SP][0] as u32).wrapping_add(0x130), c.g[RA][0]);
c.g[S0][0] = c.g[A1][0];
c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x12c));
c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x40));
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x44));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x002cbaf4);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
call_ptr(c, t, 0x002cbaf4)?;
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A0][0] = 0xffffffffffffffff;
c.g[A1][0] = 0xffffffffffffff83;
c.g[A3][0] = 0xffffffffffffffff;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
c.g[A2][0] = 0x5;
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
c.g[V0][0] = c.g[V0][0] | 0x4;
c.sw((c.g[A0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
'l_2cbb28: loop {
c.cycles += 7;
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
let t = c.g[A2][0] != c.g[A3][0];
c.g[V0][0] = 0x4d0000;
if t {
if event_due(c) { deliver_event_here(c, 0x002cbb28); } continue 'l_2cbb28;
} else {
c.cycles += 21;
c.g[AT][0] = 0x43000000;
c.mtc1(1, 0);
c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x53a0));
c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x53a0));
c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
c.g[T0][0] = 0x80;
c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
c.mul_s(1, 1, 0);
c.mul_s(3, 3, 0);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 4);
c.mul_s(2, 2, 0);
c.g[V1][0] = c.g[SP][0];
c.cvt_w_s(0, 1);
c.mfc1(7, 0);
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
c.cvt_w_s(0, 3);
c.mfc1(5, 0);
c.cvt_w_s(0, 2);
c.mfc1(4, 0);
c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]);
'l_2cbb98: loop {
c.cycles += 10;
c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[A1][0]);
c.sw((c.g[V1][0] as u32), 0u64);
c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
c.swc1(4, (c.g[V1][0] as u32).wrapping_add(0x8));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[T0][0]);
if event_due(c) { deliver_event_here(c, 0x002cbb98); } continue 'l_2cbb98;
} else {
c.cycles += 65;
c.lwc1(6, (c.g[S0][0] as u32).wrapping_add(0x4));
c.g[A1][0] = 0x6;
c.lwc1(5, (c.g[S0][0] as u32).wrapping_add(0xc));
c.g[A2][0] = c.g[SP][0];
c.lwc1(4, (c.g[S0][0] as u32));
c.g[A3][0] = 0u64;
c.g[AT][0] = 0x40400000;
c.mtc1(1, 3);
c.add_s(5, 6, 5);
c.g[AT][0] = 0x40c00000;
c.mtc1(1, 9);
c.add_s(6, 6, 3);
c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x8));
c.add_s(8, 4, 9);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 7);
c.g[AT][0] = 0x40800000;
c.mtc1(1, 2);
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 0);
c.add_s(10, 6, 3);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x124));
c.sub_s(5, 5, 2);
c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
c.mul_s(0, 1, 0);
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x120));
c.add_s(1, 4, 1);
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x12c));
c.sub_s(3, 5, 3);
c.add_s(4, 4, 0);
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sub_s(1, 1, 9);
c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x124));
c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x124));
c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x120));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x124));
c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0x120));
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x124));
c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x124));
c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x120));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x110));
c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x238));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x002cbcb4);
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x002cbcb4)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x140));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x130));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x150));
c.pc = t; return Ok(());
}
}
}
}
    });
}

/// `int MenuSysEnumItem_HandleInput(MenuSysEnumItem* self)` (0x2cc8f0,
/// reference name `cEnumMenuItem_input`): the base item input first (returns
/// its result when non-zero or the item is disabled); input code 6 selects
/// through the menu's vtable; otherwise it finds the current value among the
/// entries (count at +0x18, `{label, value}` at +0x1c, value pointer at
/// +0x20) and moves to the previous (codes 2/0) or next (codes 3/1) entry
/// with wrap-around, notifying the menu (vtable slot 4) after a change.
pub fn menu_sys_enum_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
'b_2ccaf4: {
'b_2ccaf0: {
c.cycles += 15;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x002cc918);
c.g[S0][0] = c.g[A0][0];
call_ptr(c, 0x002ca2b0, 0x002cc918)?;
c.g[S5][0] = c.g[V0][0];
c.g[RA][0] = sx(0x002cc924);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca3b0, 0x002cc924)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.g[S5][0];
if t {
break 'b_2ccaf0;
} else {
c.cycles += 2;
if c.g[S5][0] != 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
break 'b_2ccaf4;
} else {
'b_2ccaec: {
'b_2cc970: {
c.cycles += 5;
c.g[RA][0] = sx(0x002cc93c);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca3e8, 0x002cc93c)?;
c.g[V1][0] = 0x6;
if c.g[V0][0] != c.g[V1][0] {
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
break 'b_2cc970;
} else {
c.cycles += 10;
c.g[A3][0] = c.lw((c.g[S0][0] as u32));
c.g[A1][0] = c.g[S0][0];
c.g[A2][0] = 0u64;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x12c));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x18));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x002cc968);
c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x002cc968)?;
let t = true;
c.g[S5][0] = c.g[V0][0];
if t {
break 'b_2ccaec;
} else {
break 'b_2cc970;
}
}
}
c.cycles += 4;
c.g[S1][0] = 0u64;
c.g[S2][0] = 0x2;
c.g[S3][0] = 0x3;
c.g[S4][0] = 0x1;
'l_2cc980: loop {
'b_2ccacc: {
'b_2ccac8: {
'b_2cca00: {
c.cycles += 2;
if c.g[S1][0] != c.g[A2][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
break 'b_2cca00;
} else {
'b_2cc9c0: {
'b_2cc9ac: {
c.cycles += 4;
c.g[RA][0] = sx(0x002cc990);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca3e8, 0x002cc990)?;
if c.g[V0][0] == c.g[S2][0] {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
break 'b_2cc9ac;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x002cc9a0);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca3e8, 0x002cc9a0)?;
let t = c.g[V0][0] != 0u64;

if t {
break 'b_2cc9c0;
} else {
c.cycles += 1;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
break 'b_2cc9ac;
}
}
}
c.cycles += 5;
c.g[A1][0] = c.g[S0][0];
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
let t = true;
c.sw((c.g[V1][0] as u32), c.g[V0][0]);
if t {
break 'b_2ccacc;
} else {
break 'b_2cc9c0;
}
}
'b_2cc9e4: {
c.cycles += 4;
c.g[RA][0] = sx(0x002cc9c8);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca3e8, 0x002cc9c8)?;
if c.g[V0][0] == c.g[S3][0] {
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
break 'b_2cc9e4;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x002cc9d8);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca3e8, 0x002cc9d8)?;
let t = c.g[V0][0] != c.g[S4][0];
c.g[V0][0] = c.g[S5][0];
if t {
break 'b_2ccaf0;
} else {
c.cycles += 1;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
break 'b_2cc9e4;
}
}
}
c.cycles += 7;
c.g[A1][0] = c.g[S0][0];
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
let t = true;
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0x4));
if t {
break 'b_2ccac8;
} else {
break 'b_2cca00;
}
}
}
c.cycles += 7;
c.g[V1][0] = sx((c.g[S1][0] as u32) << 3);
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A1][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
if c.g[A1][0] != c.g[V0][0] {
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
if event_due(c) { deliver_event_here(c, 0x002cc980); } continue 'l_2cc980;
} else {
'b_2cca78: {
'b_2cca40: {
c.cycles += 4;
c.g[RA][0] = sx(0x002cca24);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca3e8, 0x002cca24)?;
if c.g[V0][0] == c.g[S2][0] {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
break 'b_2cca40;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x002cca34);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca3e8, 0x002cca34)?;
let t = c.g[V0][0] != 0u64;

if t {
break 'b_2cca78;
} else {
c.cycles += 1;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
break 'b_2cca40;
}
}
}
'b_2cca54: {
c.cycles += 5;
c.g[A1][0] = c.g[S0][0];
c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
if c.g[A0][0] == 0u64 {
break 'b_2cca54;
} else {
break 'b_2cca54;
}
}
c.cycles += 9;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
c.div(0, 3, 4);
c.g[V0][0] = c.hi[0];
c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
let t = true;
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
if t {
break 'b_2ccacc;
} else {
break 'b_2cca78;
}
}
'b_2cca9c: {
c.cycles += 4;
c.g[RA][0] = sx(0x002cca80);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca3e8, 0x002cca80)?;
if c.g[V0][0] == c.g[S3][0] {
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
break 'b_2cca9c;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x002cca90);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca3e8, 0x002cca90)?;
let t = c.g[V0][0] != c.g[S4][0];
c.g[V0][0] = c.g[S5][0];
if t {
break 'b_2ccaf0;
} else {
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
break 'b_2cca9c;
}
}
}
'b_2ccab4: {
c.cycles += 6;
c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[A1][0] = c.g[S0][0];
c.div(0, 4, 2);
if c.g[V0][0] == 0u64 {
break 'b_2ccab4;
} else {
break 'b_2ccab4;
}
}
c.cycles += 5;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
c.g[V1][0] = c.hi[0];
c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
break 'b_2ccac8;
}
}
c.cycles += 1;
c.sw((c.g[A3][0] as u32), c.g[V0][0]);
break 'b_2ccacc;
}
c.cycles += 8;
c.g[A2][0] = c.lw((c.g[S0][0] as u32));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x002ccae4);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x002ccae4)?;
let t = true;
c.g[V0][0] = c.g[S5][0];
if t {
break 'b_2ccaf0;
} else {
break 'b_2ccaec;
}
}
}
c.cycles += 1;
c.g[V0][0] = c.g[S5][0];
break 'b_2ccaf0;
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
break 'b_2ccaf4;
}
c.cycles += 8;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
c.pc = t; return Ok(());
    });
}

/// `int MenuSysInt8Item_HandleInput(MenuSysInt8Item* self)` (0x2cd5a0): the
/// signed-byte slider. Direction = sign of (right axis - left axis) outside a
/// +-0.1 dead zone; a held direction counts up (+0x20) and accelerates with
/// `n * n / 0x120`, the slow/fine modifiers divide the step by 20/5 (50 both);
/// the float remainder at +0x28 carries the fraction, the byte (pointer at
/// +0x18) is clamped to the limits at +0x1c/+0x1d and the menu is notified.
pub fn menu_sys_int8_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
'b_2cd7a0: {
'b_2cd79c: {
c.cycles += 13;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = c.g[A0][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cd5c0);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
call_ptr(c, 0x002ca2b0, 0x002cd5c0)?;
c.g[S1][0] = c.g[V0][0];
c.g[RA][0] = sx(0x002cd5cc);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca3b0, 0x002cd5cc)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.g[S1][0];
if t {
break 'b_2cd79c;
} else {
c.cycles += 2;
if c.g[S1][0] != 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
break 'b_2cd7a0;
} else {
'b_2cd630: {
'b_2cd618: {
c.cycles += 10;
c.g[RA][0] = sx(0x002cd5e4);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca428, 0x002cd5e4)?;
c.mov_s(20, 0);
c.g[RA][0] = sx(0x002cd5f0);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca408, 0x002cd5f0)?;
c.sub_s(20, 20, 0);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3ff0));
c.c_le_s(20, 0);
let t = c.fcc();

if t {
break 'b_2cd618;
} else {
c.cycles += 5;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fec));
c.c_le_s(0, 20);
let t = !c.fcc();
c.g[V1][0] = 0u64;
if t {
break 'b_2cd630;
} else {
break 'b_2cd618;
}
}
}
c.cycles += 5;
c.mtc1(0, 0);
c.c_lt_s(20, 0);
let t = !c.fcc();
c.g[V1][0] = 0x1;
if t {
break 'b_2cd630;
} else {
c.cycles += 1;
c.g[V1][0] = 0xffffffffffffffff;
break 'b_2cd630;
}
}
'b_2cd65c: {
'b_2cd64c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
if c.g[V1][0] != c.g[V0][0] {
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), 0u64);
break 'b_2cd64c;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
let t = true;
c.sw((c.g[S0][0] as u32).wrapping_add(0x20), c.g[V0][0]);
if t {
break 'b_2cd65c;
} else {
break 'b_2cd64c;
}
}
}
c.cycles += 4;
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V1][0]);
c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x28));
break 'b_2cd65c;
}
'b_2cd798: {
c.cycles += 2;
let t = c.g[V1][0] == 0u64;
c.g[V1][0] = 0x120;
if t {
break 'b_2cd798;
} else {
'b_2cd674: {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fe8));
if c.g[V1][0] == 0u64 {
break 'b_2cd674;
} else {
break 'b_2cd674;
}
}
'b_2cd700: {
'b_2cd6c8: {
c.cycles += 13;
c.mult(0, 2, 2, 2);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 21);
c.mul_s(20, 20, 0);
c.g[A0][0] = c.g[S0][0];
c.div(0, 2, 3);
c.g[V0][0] = c.lo[0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.g[RA][0] = sx(0x002cd6a0);
c.mul_s(20, 20, 0);
call_ptr(c, 0x002ca488, 0x002cd6a0)?;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_2cd6c8;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x002cd6b0);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca4a8, 0x002cd6b0)?;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_2cd6c8;
} else {
c.cycles += 4;
c.g[AT][0] = 0x42480000;
c.mtc1(1, 21);
let t = true;

if t {
break 'b_2cd700;
} else {
break 'b_2cd6c8;
}
}
}
}
'b_2cd6e8: {
c.cycles += 4;
c.g[RA][0] = sx(0x002cd6d0);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca488, 0x002cd6d0)?;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_2cd6e8;
} else {
c.cycles += 4;
c.g[AT][0] = 0x41a00000;
c.mtc1(1, 21);
let t = true;

if t {
break 'b_2cd700;
} else {
break 'b_2cd6e8;
}
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x002cd6f0);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca4a8, 0x002cd6f0)?;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_2cd700;
} else {
c.cycles += 2;
c.g[AT][0] = 0x40a00000;
c.mtc1(1, 21);
break 'b_2cd700;
}
}
'b_2cd744: {
c.cycles += 14;
c.div_s(20, 20, 21);
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x28));
c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
c.add_s(20, 0, 20);
c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x28));
c.g[T0][0] = c.lb((c.g[T1][0] as u32));
c.mtc1(8, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 20);
c.cvt_w_s(1, 0);
c.cvt_s_w(1, 1);
c.c_lt_s(0, 1);
let t = !c.fcc();

if t {
break 'b_2cd744;
} else {
c.cycles += 3;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.sub_s(1, 1, 0);
break 'b_2cd744;
}
}
c.cycles += 21;
c.cvt_w_s(0, 1);
c.mfc1(2, 0);
c.g[A2][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x1c));
c.g[A3][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x1d));
c.g[A1][0] = c.g[S0][0];
c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[A2][0]; }
c.g[A0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[A0][0] != 0 { c.g[V0][0] = c.g[A3][0]; }
c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 20, 0);
c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x28));
c.sb((c.g[T1][0] as u32), c.g[V0][0]);
c.g[A2][0] = c.lw((c.g[S0][0] as u32));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x002cd798);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x002cd798)?;
break 'b_2cd798;
}
}
c.cycles += 1;
c.g[V0][0] = c.g[S1][0];
break 'b_2cd79c;
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
break 'b_2cd7a0;
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

/// `int MenuSysIntItem_HandleInput(MenuSysIntItem* self)` (0x2cd9d8): the
/// same slider for a 32-bit value (limits at +0x1c/+0x20 as words, repeat
/// counter +0x24, direction +0x28, float remainder +0x2c).
pub fn menu_sys_int_item_handle_input(c: &mut Cpu) {
    guest(c, |c| {
'b_2cdbd8: {
'b_2cdbd4: {
c.cycles += 13;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S0][0] = c.g[A0][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
c.g[RA][0] = sx(0x002cd9f8);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
call_ptr(c, 0x002ca2b0, 0x002cd9f8)?;
c.g[S1][0] = c.g[V0][0];
c.g[RA][0] = sx(0x002cda04);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca3b0, 0x002cda04)?;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = c.g[S1][0];
if t {
break 'b_2cdbd4;
} else {
c.cycles += 2;
if c.g[S1][0] != 0u64 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
break 'b_2cdbd8;
} else {
'b_2cda68: {
'b_2cda50: {
c.cycles += 10;
c.g[RA][0] = sx(0x002cda1c);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca428, 0x002cda1c)?;
c.mov_s(20, 0);
c.g[RA][0] = sx(0x002cda28);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca408, 0x002cda28)?;
c.sub_s(20, 20, 0);
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fe4));
c.c_le_s(20, 0);
let t = c.fcc();

if t {
break 'b_2cda50;
} else {
c.cycles += 5;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fe0));
c.c_le_s(0, 20);
let t = !c.fcc();
c.g[V1][0] = 0u64;
if t {
break 'b_2cda68;
} else {
break 'b_2cda50;
}
}
}
c.cycles += 5;
c.mtc1(0, 0);
c.c_lt_s(20, 0);
let t = !c.fcc();
c.g[V1][0] = 0x1;
if t {
break 'b_2cda68;
} else {
c.cycles += 1;
c.g[V1][0] = 0xffffffffffffffff;
break 'b_2cda68;
}
}
'b_2cda94: {
'b_2cda84: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
if c.g[V1][0] != c.g[V0][0] {
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
break 'b_2cda84;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
let t = true;
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
if t {
break 'b_2cda94;
} else {
break 'b_2cda84;
}
}
}
c.cycles += 4;
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[V1][0]);
c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2c));
break 'b_2cda94;
}
'b_2cdbd0: {
c.cycles += 2;
let t = c.g[V1][0] == 0u64;
c.g[V1][0] = 0x120;
if t {
break 'b_2cdbd0;
} else {
'b_2cdaac: {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x3fdc));
if c.g[V1][0] == 0u64 {
break 'b_2cdaac;
} else {
break 'b_2cdaac;
}
}
'b_2cdb38: {
'b_2cdb00: {
c.cycles += 13;
c.mult(0, 2, 2, 2);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 21);
c.mul_s(20, 20, 0);
c.g[A0][0] = c.g[S0][0];
c.div(0, 2, 3);
c.g[V0][0] = c.lo[0];
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.g[RA][0] = sx(0x002cdad8);
c.mul_s(20, 20, 0);
call_ptr(c, 0x002ca488, 0x002cdad8)?;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_2cdb00;
} else {
c.cycles += 4;
c.g[RA][0] = sx(0x002cdae8);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca4a8, 0x002cdae8)?;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_2cdb00;
} else {
c.cycles += 4;
c.g[AT][0] = 0x42480000;
c.mtc1(1, 21);
let t = true;

if t {
break 'b_2cdb38;
} else {
break 'b_2cdb00;
}
}
}
}
'b_2cdb20: {
c.cycles += 4;
c.g[RA][0] = sx(0x002cdb08);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca488, 0x002cdb08)?;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_2cdb20;
} else {
c.cycles += 4;
c.g[AT][0] = 0x41a00000;
c.mtc1(1, 21);
let t = true;

if t {
break 'b_2cdb38;
} else {
break 'b_2cdb20;
}
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x002cdb28);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x002ca4a8, 0x002cdb28)?;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_2cdb38;
} else {
c.cycles += 2;
c.g[AT][0] = 0x40a00000;
c.mtc1(1, 21);
break 'b_2cdb38;
}
}
'b_2cdb7c: {
c.cycles += 14;
c.div_s(20, 20, 21);
c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0x2c));
c.g[T1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
c.add_s(20, 0, 20);
c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x2c));
c.g[T0][0] = c.lw((c.g[T1][0] as u32));
c.mtc1(8, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 0, 20);
c.cvt_w_s(1, 0);
c.cvt_s_w(1, 1);
c.c_lt_s(0, 1);
let t = !c.fcc();

if t {
break 'b_2cdb7c;
} else {
c.cycles += 3;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.sub_s(1, 1, 0);
break 'b_2cdb7c;
}
}
c.cycles += 21;
c.cvt_w_s(0, 1);
c.mfc1(2, 0);
c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x20));
c.g[A1][0] = c.g[S0][0];
c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
if c.g[V1][0] != 0 { c.g[V0][0] = c.g[A2][0]; }
c.g[A0][0] = ((c.g[A3][0] as i64) < (c.g[V0][0] as i64)) as u64;
if c.g[A0][0] != 0 { c.g[V0][0] = c.g[A3][0]; }
c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.add_s(0, 20, 0);
c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2c));
c.sw((c.g[T1][0] as u32), c.g[V0][0]);
c.g[A2][0] = c.lw((c.g[S0][0] as u32));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x12c));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x002cdbd0);
c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x002cdbd0)?;
break 'b_2cdbd0;
}
}
c.cycles += 1;
c.g[V0][0] = c.g[S1][0];
break 'b_2cdbd4;
}
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
break 'b_2cdbd8;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, check_reaching, machine};
    use crate::difftest::{check_lohi, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The item-callee fakes' gate word, the per-seed salt behind it, and the
    /// fixture addresses (all inside the test data area).
    const OC1533_CTRL: u32 = DATA + 0xe_0000;
    const OC1533_MAGIC: u32 = 0x1533_a5c3;
    const OC1533_SELF: u32 = DATA + 0x1000;
    const OC1533_RECT: u32 = DATA + 0x200;
    const OC1533_MENU: u32 = DATA + 0x2000;
    const OC1533_VTABLE: u32 = DATA + 0x2800;
    const OC1533_WORLD: u32 = DATA + 0x3000;
    const OC1533_ENTRIES: u32 = DATA + 0x6000;
    const OC1533_VALUE: u32 = DATA + 0x7000;
    const OC1533_TARGET: u32 = DATA + 0x8000;
    const OC1533_HEAD: u32 = DATA + 0x9000;

    /// What a fake item callee answers (see `oc1533_fake_body`).
    const KIND_HANDLE_INPUT: u32 = 0;
    const KIND_IS_DISABLED: u32 = 1;
    const KIND_INPUT_CODE: u32 = 2;
    const KIND_AXIS: u32 = 3;
    const KIND_MOD_HELD: u32 = 4;

    static OC1533_PREV_HANDLE_INPUT: AtomicUsize = AtomicUsize::new(0);
    static OC1533_PREV_IS_DISABLED: AtomicUsize = AtomicUsize::new(0);
    static OC1533_PREV_INPUT_CODE: AtomicUsize = AtomicUsize::new(0);
    static OC1533_PREV_LEFT_AXIS: AtomicUsize = AtomicUsize::new(0);
    static OC1533_PREV_RIGHT_AXIS: AtomicUsize = AtomicUsize::new(0);
    static OC1533_PREV_SLOW_MOD: AtomicUsize = AtomicUsize::new(0);
    static OC1533_PREV_FINE_MOD: AtomicUsize = AtomicUsize::new(0);

    /// A gated fake of one `MenuSysItem_*` callee: logs the call like every
    /// fake, then answers from a hash of the per-seed salt and the call
    /// number (`v0` for the codes and flags, `f0` for the axes). With the
    /// gate off it runs the body the slot held before.
    fn oc1533_fake_body(
        c: &mut Cpu,
        prev: &AtomicUsize,
        addr: u32,
        generated: fn(&mut Cpu),
        kind: u32,
    ) {
        if c.bus.read32(OC1533_CTRL) != OC1533_MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        let ra = r32(c, RA);
        let n = c.bus.read32(fakes::LOG_COUNT);
        let salt = c.bus.read32(OC1533_CTRL + 4);
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        let h = fakes::mix(salt as u64 ^ (n as u64) << 32 ^ addr as u64);
        let pick = (h >> 8) as usize;
        match kind {
            KIND_HANDLE_INPUT => {
                let v = if h % 4 == 0 { 1 + (pick % 7) as u32 } else { 0 };
                c.g[V0][0] = sx(v);
            }
            KIND_IS_DISABLED => {
                c.g[V0][0] = if h % 6 == 0 { 8 } else { 0 };
            }
            KIND_INPUT_CODE => {
                const CODES: [u32; 10] = [0, 1, 2, 3, 6, 2, 3, 1, 0, 2];
                c.g[V0][0] = sx(CODES[pick % 10]);
            }
            KIND_AXIS => {
                const AXES: [f32; 10] = [0.0, 0.05, -0.05, 0.1, -0.1, 0.3, -0.3, 1.0, -1.0, 0.12];
                c.f[0] = AXES[pick % 10].to_bits();
            }
            _ => {
                c.g[V0][0] = sx((pick % 2) as u32);
            }
        }
    }

    fn oc1533_fake_handle_input(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_HANDLE_INPUT,
            0x002c_a2b0,
            ee::s09::f_002ca2b0__generated,
            KIND_HANDLE_INPUT,
        )
    }
    fn oc1533_fake_is_disabled(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_IS_DISABLED,
            0x002c_a3b0,
            ee::s09::f_002ca3b0__generated,
            KIND_IS_DISABLED,
        )
    }
    fn oc1533_fake_input_code(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_INPUT_CODE,
            0x002c_a3e8,
            ee::s09::f_002ca3e8__generated,
            KIND_INPUT_CODE,
        )
    }
    fn oc1533_fake_left_axis(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_LEFT_AXIS,
            0x002c_a408,
            ee::s09::f_002ca408__generated,
            KIND_AXIS,
        )
    }
    fn oc1533_fake_right_axis(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_RIGHT_AXIS,
            0x002c_a428,
            ee::s09::f_002ca428__generated,
            KIND_AXIS,
        )
    }
    fn oc1533_fake_slow_mod(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_SLOW_MOD,
            0x002c_a488,
            ee::s09::f_002ca488__generated,
            KIND_MOD_HELD,
        )
    }
    fn oc1533_fake_fine_mod(c: &mut Cpu) {
        oc1533_fake_body(
            c,
            &OC1533_PREV_FINE_MOD,
            0x002c_a4a8,
            ee::s09::f_002ca4a8__generated,
            KIND_MOD_HELD,
        )
    }

    fn oc1533_hook() {
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA2B0),
            oc1533_fake_handle_input,
            &OC1533_PREV_HANDLE_INPUT,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA3B0),
            oc1533_fake_is_disabled,
            &OC1533_PREV_IS_DISABLED,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA3E8),
            oc1533_fake_input_code,
            &OC1533_PREV_INPUT_CODE,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA408),
            oc1533_fake_left_axis,
            &OC1533_PREV_LEFT_AXIS,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA428),
            oc1533_fake_right_axis,
            &OC1533_PREV_RIGHT_AXIS,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA488),
            oc1533_fake_slow_mod,
            &OC1533_PREV_SLOW_MOD,
        );
        fakes::hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA4A8),
            oc1533_fake_fine_mod,
            &OC1533_PREV_FINE_MOD,
        );
    }

    /// The menu object behind `self + 0`: its vtable (at `menu + 0x12c`)
    /// holds `{delta, fn}` pairs at 0x18, 0x20, 0x40 whose functions are
    /// fake addresses (the miss hook logs those calls).
    fn oc1533_menu(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(OC1533_SELF, OC1533_MENU);
        c.bus.write32(OC1533_MENU + 0x12c, OC1533_VTABLE);
        for (i, off) in [0x18u32, 0x20, 0x40].into_iter().enumerate() {
            let delta = (r.next_u32() % 0x100) & !3;
            c.bus.write16(OC1533_VTABLE + off, delta as u16);
            c.bus
                .write32(OC1533_VTABLE + off + 4, fakes::FAKE + 0x10 * i as u32);
        }
    }

    /// Fixture of the scroll arrows: `self` with the menu vtable, the world
    /// global at `gp - 0x854` (target object at `+0xe84`, head object at
    /// `+0x10d8`), the arrow colour at `0x4d53a0` and a rectangle.
    fn oc1533_arrow_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 3);
        oc1533_menu(c, &mut r);
        c.g[A0][0] = sx(OC1533_SELF);
        c.g[A1][0] = sx(OC1533_RECT);
        c.bus.write32(GAME_GP - 0x854, OC1533_WORLD);
        c.bus.write32(OC1533_WORLD + 0xe84, OC1533_TARGET);
        c.bus.write32(OC1533_WORLD + 0x10d8, OC1533_HEAD);
        fill(c, &mut r, OC1533_TARGET, 8);
        c.bus.write16(OC1533_HEAD + 0x238, (r.next_u32() % 0x100 & !3) as u16);
        c.bus.write32(OC1533_HEAD + 0x23c, fakes::FAKE + 0x30);
        for i in 0..3 {
            let v = if seed % 5 == 0 { r.edge_f32() } else { r.f32(1.5) };
            c.bus.write32(0x004d_53a0 + 4 * i, v);
        }
        for i in 0..4 {
            let v = if seed % 7 == 0 { r.edge_f32() } else { r.f32(300.0) };
            c.bus.write32(OC1533_RECT + 4 * i, v);
        }
    }

    /// `MenuSysPage_DrawScrollArrowDown`: both vtable calls and the 6-vertex
    /// loop, with random rectangles and colours.
    #[test]
    fn menu_sys_page_draw_scroll_arrow_down_matches() {
        check_reaching(
            ee::s09::f_002cb8c8__generated,
            menu_sys_page_draw_scroll_arrow_down,
            300,
            &[0x002c_b8f4, 0x002c_bab4],
            &|c, seed| oc1533_arrow_setup(c, seed, 0x2c_b8c8),
        );
    }

    /// `MenuSysPage_DrawScrollArrowUp`: the same shape with the triangle
    /// pointing the other way.
    #[test]
    fn menu_sys_page_draw_scroll_arrow_up_matches() {
        check_reaching(
            ee::s09::f_002cbac8__generated,
            menu_sys_page_draw_scroll_arrow_up,
            300,
            &[0x002c_baf4, 0x002c_bcb4],
            &|c, seed| oc1533_arrow_setup(c, seed, 0x2c_bac8),
        );
    }

    /// Common part of the item fixtures: the gated fakes on, the menu object,
    /// the gp-relative float constants (0.1 steps), `self` in `a0`.
    fn oc1533_item_setup(c: &mut Cpu, seed: u32, salt: u64, consts: u32) -> Rng {
        oc1533_hook();
        let mut r = machine(c, seed, salt, 9);
        c.bus.write32(OC1533_CTRL, OC1533_MAGIC);
        c.bus.write32(OC1533_CTRL + 4, r.next_u32());
        oc1533_menu(c, &mut r);
        c.g[A0][0] = sx(OC1533_SELF);
        if consts != 0 {
            c.bus.write32(GAME_GP - consts, (-0.1f32).to_bits());
            c.bus.write32(GAME_GP - consts + 4, 0.1f32.to_bits());
            c.bus.write32(GAME_GP - consts + 8, 0.1f32.to_bits());
        }
        r
    }

    /// `MenuSysEnumItem_HandleInput` (an enum chooser): count at `+0x18`,
    /// `{label, value}` entries at `+0x1c`, the edited value behind the
    /// pointer at `+0x20`. Up/down codes step to the previous/next entry (or
    /// wrap when the value matches none), code 6 selects through the menu.
    #[test]
    fn menu_sys_enum_item_handle_input_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = oc1533_item_setup(c, seed, 0x2c_c8f0, 0);
            let count = r.next_u32() % 5;
            c.bus.write32(OC1533_SELF + 0x18, count);
            c.bus.write32(OC1533_SELF + 0x1c, OC1533_ENTRIES);
            c.bus.write32(OC1533_SELF + 0x20, OC1533_VALUE);
            let mut values = [0u32; 5];
            for (i, v) in values.iter_mut().enumerate() {
                *v = r.next_u32() % 4;
                c.bus.write32(OC1533_ENTRIES + 8 * i as u32, r.next_u32());
                c.bus.write32(OC1533_ENTRIES + 8 * i as u32 + 4, *v);
            }
            let current = if r.next_u32() % 4 == 0 || count == 0 {
                99
            } else {
                values[(r.next_u32() % count) as usize]
            };
            c.bus.write32(OC1533_VALUE, current);
        };
        check_lohi(
            ee::s09::f_002cc8f0__generated,
            menu_sys_enum_item_handle_input,
            400,
            &setup,
        );
        check_reaching(
            ee::s09::f_002cc8f0__generated,
            menu_sys_enum_item_handle_input,
            400,
            &[
                0x002c_c918, 0x002c_c924, 0x002c_c93c, 0x002c_c968, 0x002c_c990, 0x002c_c9a0,
                0x002c_c9c8, 0x002c_c9d8, 0x002c_ca24, 0x002c_ca34, 0x002c_ca80, 0x002c_ca90,
                0x002c_cae4,
            ],
            &setup,
        );
    }

    /// Fixture of the two number sliders. `base` is the offset of the first
    /// field after the pointer/limits (`0x20` for the 8-bit slider, `0x24` for
    /// the word one): repeat counter, last direction, float remainder.
    fn oc1533_slider_setup(c: &mut Cpu, seed: u32, salt: u64, wide: bool) {
        let consts = if wide { 0x3fe4 } else { 0x3ff0 };
        let mut r = oc1533_item_setup(c, seed, salt, consts);
        c.bus.write32(OC1533_SELF + 0x18, OC1533_VALUE);
        let lo = -((r.next_u32() % 11) as i32);
        let hi = (r.next_u32() % 11) as i32;
        let cur = (r.next_u32() % 25) as i32 - 12;
        let count = [0u32, 3, 10, 30, 90, 200, 1000][(r.next_u32() % 7) as usize];
        let dir = [-1i32, 0, 1][(r.next_u32() % 3) as usize] as u32;
        let accum = r.f32(2.0);
        if wide {
            c.bus.write32(OC1533_SELF + 0x1c, lo as u32);
            c.bus.write32(OC1533_SELF + 0x20, hi as u32);
            c.bus.write32(OC1533_VALUE, cur as u32);
            c.bus.write32(OC1533_SELF + 0x24, count);
            c.bus.write32(OC1533_SELF + 0x28, dir);
            c.bus.write32(OC1533_SELF + 0x2c, accum);
        } else {
            c.bus.write8(OC1533_SELF + 0x1c, lo as u8);
            c.bus.write8(OC1533_SELF + 0x1d, hi as u8);
            c.bus.write8(OC1533_VALUE, cur as u8);
            c.bus.write32(OC1533_SELF + 0x20, count);
            c.bus.write32(OC1533_SELF + 0x24, dir);
            c.bus.write32(OC1533_SELF + 0x28, accum);
        }
    }

    const OC1533_SLIDER_RETS_8: [u32; 9] = [
        0x002c_d5c0, 0x002c_d5cc, 0x002c_d5e4, 0x002c_d5f0, 0x002c_d6a0, 0x002c_d6b0, 0x002c_d6d0,
        0x002c_d6f0, 0x002c_d798,
    ];
    const OC1533_SLIDER_RETS_32: [u32; 9] = [
        0x002c_d9f8, 0x002c_da04, 0x002c_da1c, 0x002c_da28, 0x002c_dad8, 0x002c_dae8, 0x002c_db08,
        0x002c_db28, 0x002c_dbd0,
    ];

    /// `MenuSysInt8Item_HandleInput` (a signed-byte slider): the stick axis
    /// difference gives a direction (dead zone 0.1), a held direction speeds
    /// up with the square of the repeat count, slow/fine modifiers divide the
    /// step; the byte is clamped to its limits and the menu is notified.
    #[test]
    fn menu_sys_int8_item_handle_input_matches() {
        let setup = |c: &mut Cpu, seed: u32| oc1533_slider_setup(c, seed, 0x2c_d5a0, false);
        check_lohi(
            ee::s09::f_002cd5a0__generated,
            menu_sys_int8_item_handle_input,
            400,
            &setup,
        );
        check_reaching(
            ee::s09::f_002cd5a0__generated,
            menu_sys_int8_item_handle_input,
            400,
            &OC1533_SLIDER_RETS_8,
            &setup,
        );
    }

    /// `MenuSysIntItem_HandleInput`: the same slider on a 32-bit value (the
    /// fields sit 4 bytes further on).
    #[test]
    fn menu_sys_int_item_handle_input_matches() {
        let setup = |c: &mut Cpu, seed: u32| oc1533_slider_setup(c, seed, 0x2c_d9d8, true);
        check_lohi(
            ee::s09::f_002cd9d8__generated,
            menu_sys_int_item_handle_input,
            400,
            &setup,
        );
        check_reaching(
            ee::s09::f_002cd9d8__generated,
            menu_sys_int_item_handle_input,
            400,
            &OC1533_SLIDER_RETS_32,
            &setup,
        );
    }
}
