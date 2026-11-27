a=open('scratchpad/oc1081/a.rs').read().rstrip()
b=open('scratchpad/oc1081/b.rs').read().rstrip()
def fin(t):
    assert t.endswith('c.pc = t;\nreturn Ok(());')
    return t[:-len('return Ok(());')]+'Ok(())'
a=fin(a); b=fin(b)
def ind(t): return '\n'.join(('        '+l if l else l) for l in t.split('\n'))
head='''//! Staging module oc1081 (wave wave267): hand-written replacements, each with its
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
'''
mid='''    })
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
'''
tail='''    })
}
'''
open('scratchpad/oc1081/head.rs','w').write(head+ind(a)+'\n'+mid+ind(b)+'\n'+tail)
