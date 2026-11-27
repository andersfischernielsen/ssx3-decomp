//! Staging module oc1081 (wave wave267): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1081`.

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
    (0x0039_da20, cuiscroll_set_range, "cUIScroll_SetRange"),
    (0x003a_1588, cuitextscroll_render, "cUITextScroll_Render"),
];

// cUIScroll field offsets (scroll bar widget).
/// Flag word; bit 0: the thumb is resized to the visible fraction.
pub(crate) const CUISCROLL_FLAGS: u32 = 0x14;
/// Owning screen (`UIScreen*`), 0 for none.
pub(crate) const CUISCROLL_SCREEN: u32 = 0x5c;
/// Track, thumb, up arrow, down arrow and extra widgets.
pub(crate) const CUISCROLL_TRACK: u32 = 0x74;
pub(crate) const CUISCROLL_THUMB: u32 = 0x78;
pub(crate) const CUISCROLL_UP: u32 = 0x7c;
pub(crate) const CUISCROLL_DOWN: u32 = 0x80;
pub(crate) const CUISCROLL_EXTRA: u32 = 0x84;
/// Hide mode when the thumb fills the whole track (0 hide all, 1 hide thumb
/// and arrows, 2 hide thumb only).
pub(crate) const CUISCROLL_MODE: u32 = 0x88;
/// Inline label strings played as screen frames when the position moves up
/// or down.
pub(crate) const CUISCROLL_UP_LABEL: u32 = 0x8c;
pub(crate) const CUISCROLL_DOWN_LABEL: u32 = 0xa5;
/// Position, total and visible counts (ints).
pub(crate) const CUISCROLL_POS: u32 = 0xc0;
pub(crate) const CUISCROLL_TOTAL: u32 = 0xc4;
pub(crate) const CUISCROLL_VISIBLE: u32 = 0xc8;

/// `void cUIScroll_SetRange(cUIScroll* self, int pos, int visible, int total)`
/// (0x39da20, `cUIScrollBar_setValues`): plays the up/down label frames of
/// the owning screen when the position moves, stores the range, then sizes
/// and places the thumb on the track (position ratio times the track height,
/// height by the visible fraction, clamped to the track) and shows or hides
/// the arrows and parts by the hide mode.
pub fn cuiscroll_set_range(c: &mut Cpu) {
    guest(c, |c| {
        'b_39dab4: {
        c.cycles += 17;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S3][0] = c.g[A1][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[S4][0] = c.g[A3][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.g[S5][0] = c.g[A2][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[RA][0]);
        c.g[RA][0] = sx(0x0039da5c);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x00416810, 0x0039da5c)?;
        if c.g[V0][0] == 0u64 {
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa5));
        break 'b_39dab4;
        } else {
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0u64 {
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa5));
        break 'b_39dab4;
        } else {
        c.cycles += 3;
        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
        if c.g[S1][0] == 0u64 {
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa5));
        break 'b_39dab4;
        } else {
        'b_39dab0: {
        c.cycles += 9;
        c.g[RA][0] = sx(0x0039da88);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x00317670, 0x0039da88)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0039da94);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0039c778, 0x0039da94)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = 0xffff;
        let t = c.g[A1][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        if t {
        break 'b_39dab0;
        } else {
        c.cycles += 3;
        c.g[A1][0] = c.g[A1][0] & 0xffff;
        c.g[RA][0] = sx(0x0039dab0);
        c.g[A2][0] = 0x1;
        call_ptr(c, 0x0039c870, 0x0039dab0)?;
        break 'b_39dab0;
        }
        }
        c.cycles += 1;
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0xa5));
        break 'b_39dab4;
        }
        }
        }
        }
        'b_39db14: {
        c.cycles += 4;
        c.g[RA][0] = sx(0x0039dabc);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x00416810, 0x0039dabc)?;
        if c.g[V0][0] == 0u64 {
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
        break 'b_39db14;
        } else {
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S3][0] as i64)) as u64;
        if c.g[V0][0] == 0u64 {
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
        break 'b_39db14;
        } else {
        c.cycles += 3;
        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
        if c.g[S1][0] == 0u64 {
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
        break 'b_39db14;
        } else {
        'b_39db10: {
        c.cycles += 9;
        c.g[RA][0] = sx(0x0039dae8);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x00317670, 0x0039dae8)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0039daf4);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0039c778, 0x0039daf4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = 0xffff;
        let t = c.g[A1][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        if t {
        break 'b_39db10;
        } else {
        c.cycles += 3;
        c.g[A1][0] = c.g[A1][0] & 0xffff;
        c.g[RA][0] = sx(0x0039db10);
        c.g[A2][0] = 0x1;
        call_ptr(c, 0x0039c870, 0x0039db10)?;
        break 'b_39db10;
        }
        }
        c.cycles += 1;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
        break 'b_39db14;
        }
        }
        }
        }
        'b_39de48: {
        'b_39de44: {
        c.cycles += 4;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[S4][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[S3][0]);
        let t = c.g[A2][0] == 0u64;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc8), c.g[S5][0]);
        if t {
        break 'b_39de44;
        } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        if c.g[V0][0] == 0u64 {
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        break 'b_39de48;
        } else {
        'b_39dcc8: {
        'b_39dca8: {
        c.cycles += 32;
        c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x4b));
        c.ldr(3, (c.g[A2][0] as u32).wrapping_add(0x44));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4c));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.sdr(3, (c.g[SP][0] as u32));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        c.ldl(8, (c.g[V0][0] as u32).wrapping_add(0x4b));
        c.ldr(8, (c.g[V0][0] as u32).wrapping_add(0x44));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x17));
        c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V1][0]);
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039db78);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039db78)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa0));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039db98);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039db98)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0039dba4);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0039de68, 0x0039dba4)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc4));
        let t = c.g[V1][0] == 0u64;
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
        if t {
        break 'b_39dca8;
        } else {
        'b_39dc14: {
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = c.g[V0][0] & 0x1;
        let t = c.g[V0][0] == 0u64;
        c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x24));
        if t {
        break 'b_39dc14;
        } else {
        c.cycles += 20;
        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc0));
        c.cvt_s_w(1, 1);
        c.mtc1(3, 5);
        c.cvt_s_w(5, 5);
        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xc8));
        c.cvt_s_w(0, 0);
        c.lwc1(2, (c.g[SP][0] as u32));
        c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.div_s(1, 1, 5);
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.div_s(0, 0, 5);
        c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.mul_s(1, 6, 1);
        c.mul_s(0, 6, 0);
        c.add_s(3, 3, 1);
        c.c_lt_s(6, 0);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
        let t = !c.fcc();
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
        if t {
        break 'b_39dc14;
        } else {
        c.cycles += 1;
        c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x34));
        break 'b_39dc14;
        }
        }
        }
        'b_39dc60: {
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
        if c.g[A2][0] == 0u64 {
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        break 'b_39dc60;
        } else {
        'b_39dc48: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        if t {
        break 'b_39dc48;
        } else {
        c.cycles += 7;
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039dc40);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039dc40)?;
        let t = true;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        if t {
        break 'b_39dc60;
        } else {
        break 'b_39dc48;
        }
        }
        }
        c.cycles += 6;
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039dc5c);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039dc5c)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        break 'b_39dc60;
        }
        }
        c.cycles += 2;
        let t = c.g[A2][0] == 0u64;
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
        if t {
        break 'b_39dcc8;
        } else {
        'b_39dc90: {
        'b_39dc8c: {
        c.cycles += 7;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc0));
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        if t {
        break 'b_39dc8c;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[A1][0] = 0u64;
        if t {
        break 'b_39dc90;
        } else {
        break 'b_39dc8c;
        }
        }
        }
        c.cycles += 1;
        c.g[A1][0] = 0x1;
        break 'b_39dc90;
        }
        c.cycles += 6;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039dca0);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039dca0)?;
        let t = true;
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
        if t {
        break 'b_39dcc8;
        } else {
        break 'b_39dca8;
        }
        }
        }
        }
        c.cycles += 8;
        c.lwc1(1, (c.g[SP][0] as u32));
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x24));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
        break 'b_39dcc8;
        }
        'b_39de10: {
        'b_39ddb4: {
        c.cycles += 5;
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
        c.c_eq_s(1, 0);
        let t = !c.fcc();
        c.g[V0][0] = 0x1;
        if t {
        break 'b_39ddb4;
        } else {
        'b_39dd90: {
        'b_39dd80: {
        'b_39dd54: {
        'b_39dd20: {
        c.cycles += 3;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x88));
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
        if t {
        break 'b_39dd20;
        } else {
        'b_39dd10: {
        'b_39dd00: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x2;
        if t {
        break 'b_39dd00;
        } else {
        c.cycles += 2;
        if c.g[V1][0] == 0u64 {
        c.g[A0][0] = c.g[S0][0];
        break 'b_39dd10;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        if t {
        break 'b_39de10;
        } else {
        break 'b_39dd00;
        }
        }
        }
        }
        c.cycles += 2;
        if c.g[V1][0] == c.g[V0][0] {
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
        break 'b_39dd54;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        if t {
        break 'b_39de10;
        } else {
        break 'b_39dd10;
        }
        }
        }
        c.cycles += 4;
        c.g[RA][0] = sx(0x0039dd18);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0039de68, 0x0039dd18)?;
        let t = true;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        if t {
        break 'b_39de10;
        } else {
        break 'b_39dd20;
        }
        }
        }
        'b_39dd40: {
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x7c));
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = 0u64;
        if t {
        break 'b_39dd40;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039dd40);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039dd40)?;
        break 'b_39dd40;
        }
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = 0u64;
        if t {
        break 'b_39dd90;
        } else {
        c.cycles += 2;
        let t = true;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        if t {
        break 'b_39dd80;
        } else {
        break 'b_39dd54;
        }
        }
        }
        'b_39dd70: {
        c.cycles += 2;
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = 0x1;
        if t {
        break 'b_39dd70;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039dd70);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039dd70)?;
        break 'b_39dd70;
        }
        }
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x80));
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = 0x1;
        if t {
        break 'b_39dd90;
        } else {
        c.cycles += 1;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        break 'b_39dd80;
        }
        }
        c.cycles += 4;
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039dd90);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039dd90)?;
        break 'b_39dd90;
        }
        c.cycles += 9;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        c.g[A1][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039ddac);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039ddac)?;
        let t = true;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        if t {
        break 'b_39de10;
        } else {
        break 'b_39ddb4;
        }
        }
        }
        'b_39ddd4: {
        c.cycles += 3;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x84));
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = 0x1;
        if t {
        break 'b_39ddd4;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039ddd4);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039ddd4)?;
        break 'b_39ddd4;
        }
        }
        c.cycles += 15;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        c.g[A1][0] = 0x1;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039ddf0);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039ddf0)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x74));
        c.g[A1][0] = 0x1;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039de0c);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039de0c)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        break 'b_39de10;
        }
        c.cycles += 13;
        c.g[A1][0] = c.g[S1][0];
        c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x17));
        c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        c.sdl(8, (c.g[V0][0] as u32).wrapping_add(0x4b));
        c.sdr(8, (c.g[V0][0] as u32).wrapping_add(0x44));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x78));
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xa8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xac));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0039de44);
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x0039de44)?;
        break 'b_39de44;
        }
        }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        break 'b_39de48;
        }
        c.cycles += 8;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        Ok(())
    })
}

// cUITextScroll field offsets.
pub(crate) const CUITEXTSCROLL_SCROLL: u32 = 0xb8;
pub(crate) const CUITEXTSCROLL_TOP: u32 = 0xc6;
pub(crate) const CUITEXTSCROLL_COUNT: u32 = 0xc8;

/// `void cUITextScroll_Render(cUITextScroll* self, float x, float y)`
/// (0x3a1588): sets up the font (scale, shadow colour, draw layer), then
/// draws the visible lines from `top`: each in the normal or the selection
/// colour, aligned, advancing by the line height; finally updates the
/// attached scroll bar with `SetRange(top, rows, count)`.
pub fn cuitextscroll_render(c: &mut Cpu) {
    guest(c, |c| {
        'b_3a15dc: {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xd0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.g[S1][0] = c.g[A0][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.mov_s(21, 13);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        let t = c.g[V0][0] != 0u64;
        c.mov_s(20, 12);
        if t {
        break 'b_3a15dc;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x003a15dc);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x003a15dc)?;
        break 'b_3a15dc;
        }
        }
        'b_3a189c: {
        c.cycles += 5;
        c.g[RA][0] = sx(0x003a15e4);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x003a12d0, 0x003a15e4)?;
        c.g[S2][0] = c.g[V0][0];
        let t = c.g[S2][0] == 0u64;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        if t {
        break 'b_3a189c;
        } else {
        'b_3a16b8: {
        'b_3a16a0: {
        c.cycles += 16;
        c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x50));
        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x54));
        c.swc1(3, (c.g[SP][0] as u32));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x30));
        c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x34));
        c.mul_s(0, 0, 3);
        c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x38));
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.mul_s(2, 2, 1);
        c.swc1(2, (c.g[S2][0] as u32).wrapping_add(0x3c));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        if c.g[V0][0] == 0u64 {
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
        break 'b_3a16a0;
        } else {
        c.cycles += 28;
        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x1c));
        c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0x7f));
        c.ldr(2, (c.g[S1][0] as u32).wrapping_add(0x78));
        c.ldl(3, (c.g[S1][0] as u32).wrapping_add(0x87));
        c.ldr(3, (c.g[S1][0] as u32).wrapping_add(0x80));
        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
        c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x1f));
        c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[AT][0] = 0x40000000;
        c.mtc1(1, 1);
        c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x17));
        c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.ldl(8, (c.g[SP][0] as u32).wrapping_add(0x1f));
        c.ldr(8, (c.g[SP][0] as u32).wrapping_add(0x18));
        c.sdl(7, (c.g[S2][0] as u32).wrapping_add(0x57));
        c.sdr(7, (c.g[S2][0] as u32).wrapping_add(0x50));
        c.sdl(8, (c.g[S2][0] as u32).wrapping_add(0x5f));
        c.sdr(8, (c.g[S2][0] as u32).wrapping_add(0x58));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x27));
        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sdl(4, (c.g[S2][0] as u32).wrapping_add(0x2f));
        c.sdr(4, (c.g[S2][0] as u32).wrapping_add(0x28));
        let t = true;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        if t {
        break 'b_3a16b8;
        } else {
        break 'b_3a16a0;
        }
        }
        }
        c.cycles += 6;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), 0u64);
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x17));
        c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0x2f));
        c.sdr(2, (c.g[S2][0] as u32).wrapping_add(0x28));
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
        break 'b_3a16b8;
        }
        'b_3a1884: {
        'b_3a1880: {
        c.cycles += 35;
        c.g[A1][0] = 0xfffffffffffffc1f;
        c.g[V0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x15));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xe84));
        c.g[V0][0] = c.g[V0][0] & 0x1f;
        c.g[AT][0] = 0x40400000;
        c.mtc1(1, 4);
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x9));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
        c.g[V0][0] = c.g[V0][0] & 0x3e0;
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x18), 0u64);
        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x44));
        c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x48));
        c.add_s(0, 0, 20);
        c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x54));
        c.add_s(1, 1, 21);
        c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x64));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x14));
        c.cvt_s_w(0, 0);
        c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x34));
        c.mul_s(0, 0, 1);
        c.mul_s(2, 2, 0);
        c.add_s(20, 2, 4);
        c.div_s(3, 3, 20);
        c.cvt_w_s(0, 3);
        c.mfc1(2, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
        c.g[S4][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
        let t = (c.g[S4][0] as i64) <= 0;
        c.g[S3][0] = 0u64;
        if t {
        break 'b_3a1880;
        } else {
        c.cycles += 1;
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc6));
        'l_3a1748: loop {
        c.cycles += 6;
        c.g[V1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
        c.g[A0][0] = c.g[V0][0] & 0xffff;
        c.g[V1][0] = (c.g[A0][0] < c.g[V1][0]) as u64;
        if c.g[V1][0] == 0u64 {
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb8));
        break 'b_3a1884;
        } else {
        'b_3a17c8: {
        'b_3a17a4: {
        c.cycles += 4;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
        c.g[V0][0] = c.g[V0][0] & 0x1;
        let t = c.g[V0][0] == 0u64;

        if t {
        break 'b_3a17a4;
        } else {
        c.cycles += 3;
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc4));
        let t = c.g[A0][0] != c.g[V0][0];

        if t {
        break 'b_3a17a4;
        } else {
        c.cycles += 10;
        c.ldl(2, (c.g[S1][0] as u32).wrapping_add(0xaf));
        c.ldr(2, (c.g[S1][0] as u32).wrapping_add(0xa8));
        c.ldl(3, (c.g[S1][0] as u32).wrapping_add(0xb7));
        c.ldr(3, (c.g[S1][0] as u32).wrapping_add(0xb0));
        c.sdl(2, (c.g[S2][0] as u32).wrapping_add(0x47));
        c.sdr(2, (c.g[S2][0] as u32).wrapping_add(0x40));
        c.sdl(3, (c.g[S2][0] as u32).wrapping_add(0x4f));
        c.sdr(3, (c.g[S2][0] as u32).wrapping_add(0x48));
        let t = true;
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc6));
        if t {
        break 'b_3a17c8;
        } else {
        break 'b_3a17a4;
        }
        }
        }
        }
        c.cycles += 9;
        c.ldl(7, (c.g[S1][0] as u32).wrapping_add(0x23));
        c.ldr(7, (c.g[S1][0] as u32).wrapping_add(0x1c));
        c.ldl(8, (c.g[S1][0] as u32).wrapping_add(0x2b));
        c.ldr(8, (c.g[S1][0] as u32).wrapping_add(0x24));
        c.sdl(7, (c.g[S2][0] as u32).wrapping_add(0x47));
        c.sdr(7, (c.g[S2][0] as u32).wrapping_add(0x40));
        c.sdl(8, (c.g[S2][0] as u32).wrapping_add(0x4f));
        c.sdr(8, (c.g[S2][0] as u32).wrapping_add(0x48));
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc6));
        break 'b_3a17c8;
        }
        c.cycles += 46;
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xbc));
        c.g[A3][0] = 0u64;
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0x54));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = c.lw((c.g[V0][0] as u32));
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
        c.g[RA][0] = sx(0x003a17fc);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x003921f0, 0x003a17fc)?;
        c.lwc1(1, (c.g[S2][0] as u32).wrapping_add(0x14));
        c.cvt_s_w(1, 1);
        c.lwc1(3, (c.g[S2][0] as u32).wrapping_add(0x34));
        c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x54));
        c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x38));
        c.g[A0][0] = c.g[S1][0];
        c.mul_s(1, 1, 3);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x48), 0u64);
        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.mul_s(0, 0, 1);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
        c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x17));
        c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
        c.sdl(7, (c.g[SP][0] as u32).wrapping_add(0x57));
        c.sdr(7, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x58), c.g[T0][0]);
        c.g[RA][0] = sx(0x003a184c);
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x0039feb8, 0x003a184c)?;
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[A1][0] = c.g[S0][0];
        c.lwc1(13, (c.g[S1][0] as u32).wrapping_add(0xcc));
        c.g[A0][0] = c.g[S2][0];
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x54));
        c.add_s(0, 0, 20);
        c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.add_s(13, 1, 13);
        c.g[RA][0] = sx(0x003a1874);
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
        call_ptr(c, 0x00391e30, 0x003a1874)?;
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S4][0] as i64)) as u64;
        if c.g[V0][0] != 0u64 {
        c.g[V0][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc6));
        if event_due(c) { deliver_event_here(c, 0x003a1748); } continue 'l_3a1748;
        } else {
        break 'b_3a1880;
        }
        }
        }
        }
        }
        c.cycles += 1;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xb8));
        break 'b_3a1884;
        }
        'b_3a1898: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[A2][0] = c.g[S4][0];
        if t {
        break 'b_3a1898;
        } else {
        c.cycles += 3;
        c.g[A3][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc8));
        c.g[RA][0] = sx(0x003a1898);
        c.g[A1][0] = c.lhu((c.g[S1][0] as u32).wrapping_add(0xc6));
        call_ptr(c, 0x0039da20, 0x003a1898)?;
        break 'b_3a1898;
        }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
        break 'b_3a189c;
        }
        }
        c.cycles += 9;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x60));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xc8));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
        c.pc = t;
        Ok(())
    })
}
