//! Staging module oc397 (wave wave217): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc397`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

// ---------------------------------------------------------------------------
// Font_FindLineBreak

/// Font: horizontal shadow offset (float; its magnitude shortens the line).
pub(crate) const FONT_FIND_LINE_BREAK_SHADOW_X: u32 = 0x28;
/// Font: lowest character of the direct glyph range.
pub(crate) const FONT_FIND_LINE_BREAK_LO: u32 = 0xc;
/// Font: highest character of the direct glyph range.
pub(crate) const FONT_FIND_LINE_BREAK_HI: u32 = 0x10;
/// Font: glyph count (the end of the sorted extra glyphs).
pub(crate) const FONT_FIND_LINE_BREAK_COUNT: u32 = 0x0;
/// Font: glyph table (12-byte entries: `u16 code` at +0, `u8 w` at +2,
/// `s8 advance` at +8, `s8 x offset` at +9).
pub(crate) const FONT_FIND_LINE_BREAK_TABLE: u32 = 0x4;
/// Font: the glyph used for a character missing from the table.
pub(crate) const FONT_FIND_LINE_BREAK_DEFAULT: u32 = 0x8;

/// The "line is full" exit of `Font_FindLineBreak`: end the copied line
/// after the last word (`dst[lastSpace + 1] = 0`, v0 = that address - 1),
/// then skip the spaces after it in the source; returns (v0) the start of
/// the next line. `extra` is the cost of the branch that led here, `head`
/// the skip loop's pc.
fn font_find_line_break_skip_spaces(c: &mut Cpu, extra: u64, head: u32) {
    c.cycles += extra;
    c.g[A0][0] = sx(r32(c, T8).wrapping_add(r32(c, T6)));
    let at = r32(c, V0).wrapping_add(1);
    c.sb(at, 0);
    c.g[V1][0] = 0x20;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
    loop {
        c.cycles += 7;
        c.g[V0][0] = c.lb(r32(c, A0));
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            return;
        }
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `char* Font_FindLineBreak(cFont* font, const char* str, char* dst, int max,
/// float scale, float width)` (0x392430): copy `str` into `dst` until the
/// scaled glyph widths reach `width` (less the shadow offset) or `max - 1`
/// characters. When the line is full it breaks after the last whole word
/// and returns where the next line starts (spaces skipped); if a single word
/// does not fit it breaks inside it and returns the rest; if everything fits
/// it returns 0. Glyphs come from the direct range `lo..=hi`, else a binary
/// search of the sorted extra glyphs (the default glyph when missing), and
/// none below `lo`.
///
/// Registers as in the original: t0 str, t8 start, t5 n, t7 k, t6 last
/// word end (-1: none yet), f3 width so far, f4 pending advance, f13 the
/// width limit.
pub fn font_find_line_break(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x40));
    let frame = r32(c, SP);
    c.g[T3][0] = c.g[A0][0];
    c.sq(16, frame + 0x30);
    c.g[T0][0] = c.g[A1][0];
    c.sq(17, frame + 0x20);
    c.sq(18, frame + 0x10);
    c.sq(19, frame);
    let font = r32(c, T3);
    // width -= |shadow x|, clamped at 0.
    c.mtc1(0, 1);
    c.lwc1(0, font.wrapping_add(FONT_FIND_LINE_BREAK_SHADOW_X));
    c.abs_s(0, 0);
    c.sub_s(13, 13, 0);
    c.c_lt_s(13, 1);
    if c.fcc() {
        c.mov_s(13, 1);
    }
    'ret: {
        'tail: {
            c.cycles += 8;
            c.mov_s(3, 1);
            c.g[V0][0] = c.lb(r32(c, T0));
            c.g[T8][0] = c.g[T0][0];
            c.mov_s(4, 3);
            c.g[T6][0] = u64::MAX;
            c.g[T5][0] = 0;
            let empty = c.g[V0][0] == 0;
            c.g[T7][0] = 0;
            if empty {
                break 'tail;
            }
            c.cycles += 5;
            c.g[S1][0] = sx(r32(c, A3).wrapping_sub(1));
            c.g[S0][0] = 0xc;
            c.g[S2][0] = u64::MAX;
            c.g[V0][0] = c.lb(r32(c, T0));
            c.g[V0][0] ^= 0x20;
            loop {
                // A space: break here if the line is full, else remember the
                // end of the word before it.
                c.cycles += 2;
                let is_space = c.g[V0][0] == 0;
                c.g[T9][0] = c.lbu(r32(c, T0));
                if is_space {
                    c.cycles += 4;
                    c.c_le_s(13, 3);
                    let full = c.fcc();
                    c.g[V0][0] = sx(r32(c, T6).wrapping_add(r32(c, A2)));
                    if full {
                        font_find_line_break_skip_spaces(c, 4, 0x0039_24c8);
                        break 'ret;
                    }
                    c.cycles += 2;
                    let first = (c.g[T7][0] as i64) <= 0;
                    c.g[V0][0] = sx(r32(c, T7).wrapping_add(r32(c, T8)));
                    if !first {
                        c.cycles += 4;
                        c.g[A0][0] = sx(r32(c, T7).wrapping_sub(1));
                        c.g[V1][0] = c.lb(r32(c, V0).wrapping_sub(1));
                        c.g[V1][0] ^= 0x20;
                        if c.g[V1][0] != 0 {
                            c.g[T6][0] = c.g[A0][0];
                        }
                    }
                }
                // The glyph (t2; 0 = none); t1 = &dst[n].
                'glyph: {
                    c.cycles += 5;
                    c.g[V1][0] = c.lbu(r32(c, T0));
                    c.g[T2][0] = c.lw(font.wrapping_add(FONT_FIND_LINE_BREAK_LO));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
                    let below = c.g[V0][0] != 0;
                    c.g[T1][0] = sx(r32(c, A2).wrapping_add(r32(c, T5)));
                    if below {
                        c.cycles += 2;
                        c.g[T2][0] = 0;
                        break 'glyph;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(font.wrapping_add(FONT_FIND_LINE_BREAK_HI));
                    c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        // Direct range: table + (ch - lo) * 12.
                        c.cycles += 7;
                        c.mult(0, V1, V1, S0);
                        c.mult(0, T2, T2, S0);
                        c.g[V0][0] = c.lw(font.wrapping_add(FONT_FIND_LINE_BREAK_TABLE));
                        c.g[T1][0] = sx(r32(c, A2).wrapping_add(r32(c, T5)));
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                        c.g[T2][0] = sx(r32(c, V0).wrapping_sub(r32(c, T2)));
                        break 'glyph;
                    }
                    // Above the range: binary search glyphs hi - lo .. count.
                    c.g[A3][0] = c.lw(font.wrapping_add(FONT_FIND_LINE_BREAK_COUNT));
                    c.cycles += 5;
                    c.g[T2][0] = sx(r32(c, A0).wrapping_sub(r32(c, T2)));
                    c.g[S3][0] = c.lw(font.wrapping_add(FONT_FIND_LINE_BREAK_TABLE));
                    c.g[T1][0] = sx(r32(c, A2).wrapping_add(r32(c, T5)));
                    c.g[T4][0] = 0xc;
                    c.g[V0][0] = sx(r32(c, T2).wrapping_add(r32(c, A3)));
                    loop {
                        c.cycles += 6;
                        c.g[A1][0] = sx((r32(c, V0) as i32 >> 1) as u32);
                        c.mult(0, A0, A1, T4);
                        c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
                        c.g[A0][0] = c.lhu(r32(c, V0));
                        if c.g[V1][0] == c.g[A0][0] {
                            c.g[T2][0] = c.g[V0][0];
                            break 'glyph;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                        if c.g[V0][0] == 0 {
                            c.g[T2][0] = sx(r32(c, A1).wrapping_add(1));
                        } else {
                            c.cycles += 1;
                            c.g[A3][0] = c.g[A1][0];
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[A3][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V0][0] = sx(r32(c, T2).wrapping_add(r32(c, A3)));
                        if !more {
                            c.cycles += 1;
                            c.g[T2][0] = c.lw(font.wrapping_add(FONT_FIND_LINE_BREAK_DEFAULT));
                            break 'glyph;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0039_2560);
                        }
                    }
                }
                // right = (xoff + w) * scale; width += pend + right;
                // pend = advance * scale - right.
                c.cycles += 2;
                if c.g[T2][0] != 0 {
                    c.cycles += 13;
                    let glyph = r32(c, T2);
                    c.g[V0][0] = c.lb(glyph.wrapping_add(9));
                    c.g[V1][0] = c.lbu(glyph.wrapping_add(2));
                    c.g[A0][0] = c.lb(glyph.wrapping_add(8));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.mtc1(V0, 0);
                    c.cvt_s_w(0, 0);
                    c.mtc1(A0, 1);
                    c.cvt_s_w(1, 1);
                    c.mul_s(0, 0, 12);
                    c.mul_s(1, 1, 12);
                    c.add_s(2, 4, 0);
                    c.sub_s(4, 1, 0);
                    c.add_s(3, 3, 2);
                }
                // Copy the character, unless the line is full with no word
                // break yet: then break inside the word.
                c.cycles += 4;
                c.c_le_s(13, 3);
                let dst = r32(c, T1);
                if !c.fcc() {
                    let ch = c.g[T9][0];
                    c.sb(dst, ch);
                } else {
                    c.cycles += 2;
                    if c.g[T6][0] == c.g[S2][0] {
                        c.sb(dst, 0);
                        c.cycles += 2;
                        c.g[V0][0] = c.g[T0][0];
                        break 'ret;
                    }
                    c.cycles += 1;
                    let ch = c.g[T9][0];
                    c.sb(dst, ch);
                }
                c.cycles += 4;
                c.g[T5][0] = sx(r32(c, T5).wrapping_add(1));
                c.g[T7][0] = sx(r32(c, T7).wrapping_add(1));
                let at_max = c.g[T5][0] == c.g[S1][0];
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
                if at_max {
                    break 'tail;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(r32(c, T0));
                let more = c.g[V0][0] != 0;
                c.g[V0][0] ^= 0x20;
                if !more {
                    break 'tail;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0039_24a0);
                }
            }
        }
        // End of the string (or max): break after the last word if full.
        c.cycles += 4;
        c.c_le_s(13, 3);
        let full = c.fcc();
        c.g[V0][0] = sx(r32(c, T6).wrapping_add(r32(c, A2)));
        if full {
            font_find_line_break_skip_spaces(c, 5, 0x0039_2630);
            break 'ret;
        }
        c.cycles += 3;
        c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, T5)));
        c.g[V0][0] = 0;
        let end = r32(c, V1);
        c.sb(end, 0);
    }
    c.cycles += 6;
    let frame = r32(c, SP);
    c.lq(16, frame + 0x30);
    c.lq(17, frame + 0x20);
    c.lq(18, frame + 0x10);
    c.lq(19, frame);
    c.g[SP][0] = sx(frame.wrapping_add(0x40));
    ret(c);
}

// ---------------------------------------------------------------------------
// MemCardStateBase_Update

/// MemCardStateBase: vtable (gcc `{delta, fn}` slots).
pub(crate) const MCSB_UPDATE_VT: u32 = 0x8;
/// MemCardStateBase: the UI screen (`UIScreen_FindObject`'s first argument).
pub(crate) const MCSB_UPDATE_SCREEN: u32 = 0x40;
/// MemCardStateBase: non-zero while busy (no screen update).
pub(crate) const MCSB_UPDATE_BUSY: u32 = 0x1a4;
/// MemCardStateBase: the card-check mode (1: wait for the manager's idle).
pub(crate) const MCSB_UPDATE_CHECK_MODE: u32 = 0x1b0;
/// MemCardStateBase: the state (0-6; 6 = checking the card).
pub(crate) const MCSB_UPDATE_STATE: u32 = 0x1c0;
/// MemCardStateBase: the selected menu entry (fallback).
pub(crate) const MCSB_UPDATE_SELECTION: u32 = 0x1c4;
/// MemCardStateBase: set once the slot selection was started.
pub(crate) const MCSB_UPDATE_STARTED: u32 = 0x1d8;
/// MemCardStateBase: set together with `STARTED`.
pub(crate) const MCSB_UPDATE_STARTED2: u32 = 0x214;
/// MemCardStateBase: the open popup (0: none).
pub(crate) const MCSB_UPDATE_POPUP: u32 = 0x218;
/// MCManager: its card object.
pub(crate) const MCSB_UPDATE_MGR_CARD: u32 = 0x434;
/// MCManager: its own vtable (slot 8: `SetMode(0x31)`).
pub(crate) const MCSB_UPDATE_MGR_VT: u32 = 0x748;
/// MCManager: the current operation's mode.
pub(crate) const MCSB_UPDATE_MGR_MODE: u32 = 0x130;
/// The per-state handler jump table (`.rodata`, 7 entries).
pub(crate) const MCSB_UPDATE_JUMP_TABLE: u32 = 0x0046_7960;
/// Manager modes for which the state's "mode changed" slot (0x178) runs.
pub(crate) const MCSB_UPDATE_MODES: [u64; 13] = [
    0x28, 0xb, 0xf, 0x10, 0x11, 0x12, 0x1f, 0x21, 0x20, 0x22, 0x2c, 0x19, 0x1a,
];

/// A gcc virtual call `obj->slot(...)`: a0 = `obj + delta` (the delta is
/// loaded into a0 first), `fn_reg` = the method, returning to `ret`.
fn mcsb_update_vcall(
    c: &mut Cpu,
    obj: u64,
    vt: u64,
    off: u32,
    fn_reg: usize,
    ret: u32,
) -> crate::call::Flow {
    let slot = (vt as u32).wrapping_add(off);
    c.g[A0][0] = c.lh(slot);
    c.g[fn_reg][0] = c.lw(slot.wrapping_add(4));
    let t = c.g[fn_reg][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, ret)
}

/// `void MemCardStateBase_Update(MemCardStateBase* self)` (0x1d8de0): the
/// memory-card screens' per-frame update. Without a popup and when not busy
/// it updates the screen (the overlay form in game); the first time it
/// starts the manager's slot selection; it updates the manager unless
/// checking the card (state 6) with nothing pending; it runs the state's
/// handler slot; on certain manager modes it calls slot 0x178; with a status
/// message pending it fetches the status text into a temporary buffer and
/// shows it in the popup; then slot 0xd0; and with no popup and a card
/// present it tells the menu (slots 0x130/0x138) whether the selected slot
/// and the card are usable.
pub fn mem_card_state_base_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        // The screen update.
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_POPUP));
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_BUSY));
            let busy = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
            if !busy {
                c.cycles += 3;
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
                if c.g[V1][0] != 0 {
                    c.cycles += 4;
                    call_ptr(c, 0x0020_e900, 0x001d_8e30)?;
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0039_e510, 0x001d_8e40)?;
                }
            }
        }
        // The manager; the first time, start the slot selection.
        c.cycles += 6;
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
        call_ptr(c, 0x0022_7f80, 0x001d_8e48)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_STARTED));
        if c.g[V0][0] == 0 {
            c.cycles += 18;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0024_1380, 0x001d_8e60)?;
            c.g[V0][0] = 0x1;
            c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_VT));
            c.sw(r32(c, S1).wrapping_add(MCSB_UPDATE_STARTED2), 1);
            c.g[A1][0] = 0x1;
            c.sw(r32(c, S1).wrapping_add(MCSB_UPDATE_STARTED), 1);
            let (obj, vt) = (c.g[S1][0], c.g[V1][0]);
            mcsb_update_vcall(c, obj, vt, 0x118, V0, 0x001d_8e84)?;
            c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(MCSB_UPDATE_MGR_VT));
            c.g[A1][0] = 0x31;
            let (obj, vt) = (c.g[S2][0], c.g[V1][0]);
            mcsb_update_vcall(c, obj, vt, 0x8, V0, 0x001d_8e9c)?;
        }
        c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_STATE));
        // MCManager_Update unless checking the card with nothing pending.
        'after_update: {
            'update: {
                c.cycles += 3;
                c.g[V0][0] = 0x6;
                let not_checking = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0x1;
                if not_checking {
                    break 'update;
                }
                c.cycles += 3;
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_CHECK_MODE));
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x0023_c898, 0x001d_8ec0)?;
                    if c.g[V0][0] == 0 {
                        break 'update;
                    }
                    c.cycles += 1;
                }
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_CHECK_MODE));
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_STATE));
                    break 'after_update;
                }
                c.cycles += 8;
                c.g[A1][0] = c.lw(r32(c, S2).wrapping_add(MCSB_UPDATE_MGR_CARD));
                c.g[V1][0] = c.lw(r32(c, A1));
                let (obj, vt) = (c.g[A1][0], c.g[V1][0]);
                mcsb_update_vcall(c, obj, vt, 0x58, V0, 0x001d_8eec)?;
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_STATE));
                    break 'after_update;
                }
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0023_d618, 0x001d_8efc)?;
            c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_STATE));
        }
        // The state's handler (jump table); v1 = the status globals after.
        'handled: {
            'default: {
                c.cycles += 3;
                c.g[V0][0] = (c.g[V1][0] < 0x7) as u64;
                let out_of_range = c.g[V0][0] == 0;
                c.g[V0][0] = 0x46_0000;
                if out_of_range {
                    break 'default;
                }
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx(MCSB_UPDATE_JUMP_TABLE);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = c.lw(r32(c, V1));
                let target = c.g[A0][0] as u32;
                let (slot, cost) = match target {
                    0x001d_8f24 => (0xf8, 4),
                    0x001d_8f34 => (0xd8, 4),
                    0x001d_8f44 => (0x100, 4),
                    0x001d_8f54 => (0xe0, 4),
                    0x001d_8f64 => (0xf0, 4),
                    0x001d_8f74 => (0xe8, 3),
                    0x001d_8f90 => (0, 0),
                    _ => {
                        c.jump_out(target, 0x001d_8de0, 0x001d_9254);
                        return Err(crate::call::Unwound);
                    }
                };
                if slot != 0 {
                    // States 0-5: one handler slot each.
                    c.cycles += cost + 4;
                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_VT));
                    let (obj, vt) = (c.g[S1][0], c.g[V0][0]);
                    mcsb_update_vcall(c, obj, vt, slot, V1, 0x001d_8f88)?;
                    c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x10c8));
                    break 'handled;
                }
                // State 6: slot 0x108 once the card check may go on.
                c.cycles += 7;
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x848));
                call_ptr(c, 0x0022_7f80, 0x001d_8f98)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_CHECK_MODE));
                c.g[V0][0] = 0x1;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0023_c898, 0x001d_8fb4)?;
                    let idle = c.g[V0][0] != 0;
                    c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x10c8));
                    if !idle {
                        break 'handled;
                    }
                    c.cycles += 1;
                }
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_CHECK_MODE));
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_VT));
                } else {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(MCSB_UPDATE_MGR_CARD));
                    c.g[V1][0] = c.lw(r32(c, A1));
                    let (obj, vt) = (c.g[A1][0], c.g[V1][0]);
                    mcsb_update_vcall(c, obj, vt, 0x58, V0, 0x001d_8fe0)?;
                    let pending = c.g[V0][0] != 0;
                    c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x10c8));
                    if pending {
                        break 'handled;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_VT));
                }
                c.cycles += 4;
                let (obj, vt) = (c.g[S1][0], c.g[V0][0]);
                mcsb_update_vcall(c, obj, vt, 0x108, V1, 0x001d_8ffc)?;
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw(r32(c, GP).wrapping_sub(0x10c8));
        }
        // The manager's mode, the status message (v1 = status globals).
        'after_status: {
            'status_done: {
                'show_status: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x210));
                    let has_message = c.g[V0][0] != 0;
                    c.g[A1][0] = 0x46_0000;
                    if has_message {
                        break 'show_status;
                    }
                    'mode_done: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x208));
                        let t = c.g[V0][0] != 0;
                        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x10c8));
                        if t {
                            break 'mode_done;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x204));
                        let t = c.g[V0][0] != 0;
                        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x10c8));
                        if t {
                            break 'mode_done;
                        }
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(MCSB_UPDATE_MGR_MODE));
                        let mut hit = false;
                        for (i, &mode) in MCSB_UPDATE_MODES.iter().enumerate() {
                            if i > 0 {
                                c.cycles += 2;
                            }
                            let t = c.g[V1][0] == mode;
                            c.g[V0][0] = match MCSB_UPDATE_MODES.get(i + 1) {
                                Some(&next) => next,
                                None => c.lw(r32(c, GP).wrapping_sub(0x10c8)),
                            };
                            if t {
                                hit = true;
                                break;
                            }
                        }
                        if !hit {
                            break 'mode_done;
                        }
                        c.cycles += 6;
                        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_VT));
                        let (obj, vt) = (c.g[S1][0], c.g[V0][0]);
                        mcsb_update_vcall(c, obj, vt, 0x178, V1, 0x001d_90a8)?;
                        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x10c8));
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x210));
                    let none = c.g[V1][0] == 0;
                    c.g[A1][0] = 0x46_0000;
                    if none {
                        break 'status_done;
                    }
                }
                // The status text into a "Temp String" buffer, to the popup.
                c.cycles += 19;
                c.g[A0][0] = 0x7d0;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x1d60));
                c.g[A2][0] = 0x100;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7e30, 0x001d_90cc)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 0x320;
                call_ptr(c, 0x0024_1e18, 0x001d_90e0)?;
                c.g[A2][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_POPUP));
                c.g[A1][0] = c.g[S0][0];
                c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x8));
                let (obj, vt) = (c.g[A2][0], c.g[V0][0]);
                mcsb_update_vcall(c, obj, vt, 0xd8, V1, 0x001d_90fc)?;
                if c.g[S0][0] == 0 {
                    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_VT));
                    break 'after_status;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0031_7e98, 0x001d_910c)?;
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_VT));
        }
        // Slot 0xd0; then, with no popup and a card, the menu's state.
        'done: {
            c.cycles += 7;
            let (obj, vt) = (c.g[S1][0], c.g[V0][0]);
            mcsb_update_vcall(c, obj, vt, 0xd0, V1, 0x001d_9120)?;
            c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_POPUP));
            if c.g[V0][0] != 0 {
                frame.restore(c, S0, 0x60);
                break 'done;
            }
            c.cycles += 8;
            c.g[A1][0] = c.lw(r32(c, S2).wrapping_add(MCSB_UPDATE_MGR_CARD));
            c.g[V1][0] = c.lw(r32(c, A1));
            let (obj, vt) = (c.g[A1][0], c.g[V1][0]);
            mcsb_update_vcall(c, obj, vt, 0x258, V0, 0x001d_9144)?;
            let no_card = c.g[V0][0] == 0;
            frame.restore(c, S0, 0x60);
            if no_card {
                break 'done;
            }
            // The selection: the menu object's byte +0x95, else +0x1c4.
            c.cycles += 8;
            c.g[A0][0] = 0x4a_1600;
            call_ptr(c, 0x0031_7670, 0x001d_9158)?;
            c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_SCREEN));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001d_9164)?;
            let no_menu = c.g[V0][0] == 0;
            c.g[S0][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_SELECTION));
            if !no_menu {
                c.cycles += 1;
                c.g[S0][0] = c.lbu(r32(c, V0).wrapping_add(0x95));
            }
            // Slot 0x130(slot empty or card unusable).
            c.cycles += 14;
            c.g[A3][0] = c.lw(r32(c, S2).wrapping_add(MCSB_UPDATE_MGR_CARD));
            c.g[S5][0] = 0;
            c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_VT));
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.lw(r32(c, A3));
            c.g[V0][0] = c.lh(r32(c, V1).wrapping_add(0x130));
            c.g[S4][0] = sx(r32(c, V1).wrapping_add(0x134));
            c.g[S3][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
            let (obj, vt) = (c.g[A3][0], c.g[A2][0]);
            mcsb_update_vcall(c, obj, vt, 0x218, V0, 0x001d_91a0)?;
            if c.g[V0][0] == 0 {
                c.g[S5][0] = 0x1;
                c.cycles += 1;
            } else {
                c.cycles += 8;
                c.g[A1][0] = c.lw(r32(c, S2).wrapping_add(MCSB_UPDATE_MGR_CARD));
                c.g[V1][0] = c.lw(r32(c, A1));
                let (obj, vt) = (c.g[A1][0], c.g[V1][0]);
                mcsb_update_vcall(c, obj, vt, 0x1a8, V0, 0x001d_91c0)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[S5][0] = 0x1;
                }
            }
            c.g[V0][0] = c.lw(r32(c, S4));
            c.cycles += 25;
            c.g[A0][0] = c.g[S3][0];
            let t = c.g[V0][0] as u32;
            c.g[A1][0] = c.g[S5][0];
            call_ptr(c, t, 0x001d_91dc)?;
            // Slot 0x138(!slot usable, !card usable).
            c.g[A2][0] = c.lw(r32(c, S2).wrapping_add(MCSB_UPDATE_MGR_CARD));
            c.g[A1][0] = c.g[S0][0];
            c.g[V1][0] = c.lw(r32(c, A2));
            let (obj, vt) = (c.g[A2][0], c.g[V1][0]);
            mcsb_update_vcall(c, obj, vt, 0x218, V0, 0x001d_91f8)?;
            c.g[A1][0] = c.lw(r32(c, S2).wrapping_add(MCSB_UPDATE_MGR_CARD));
            c.g[S0][0] = c.g[V0][0] ^ 0x1;
            c.g[V1][0] = c.lw(r32(c, A1));
            let (obj, vt) = (c.g[A1][0], c.g[V1][0]);
            mcsb_update_vcall(c, obj, vt, 0x1a8, V0, 0x001d_9214)?;
            c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(MCSB_UPDATE_VT));
            c.g[A2][0] = c.g[V0][0] ^ 0x1;
            c.g[A1][0] = c.g[S0][0];
            let (obj, vt) = (c.g[S1][0], c.g[V1][0]);
            mcsb_update_vcall(c, obj, vt, 0x138, V0, 0x001d_9230)?;
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0039_2430, font_find_line_break, "Font_FindLineBreak"),
    (
        0x001d_8de0,
        mem_card_state_base_update,
        "MemCardStateBase_Update",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check_lohi, scramble, Rng, DATA};

    /// A font at DATA: direct glyphs for 0x20..=0x60, sorted extra glyphs
    /// (odd codes from 0x61) after them, a default glyph (or none), and a
    /// text of words, space runs and characters below/above the range.
    fn font_find_line_break_fixture(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x0039_2430 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
        scramble(c, &mut r);
        c.pc = 0x0039_2430;
        let font = DATA;
        let table = DATA + 0x400;
        let text = DATA + 0x2000;
        let out = DATA + 0x3000;
        let (lo, hi) = (0x20u32, 0x60u32);
        let direct = hi - lo + 1;
        let count = direct + 12;
        c.bus.write32(font + FONT_FIND_LINE_BREAK_COUNT, count);
        c.bus.write32(font + FONT_FIND_LINE_BREAK_TABLE, table);
        let default = if seed % 4 == 0 { 0 } else { table + 0x900 };
        c.bus.write32(font + FONT_FIND_LINE_BREAK_DEFAULT, default);
        c.bus.write32(font + FONT_FIND_LINE_BREAK_LO, lo);
        c.bus.write32(font + FONT_FIND_LINE_BREAK_HI, hi);
        c.bus
            .write32(font + FONT_FIND_LINE_BREAK_SHADOW_X, r.f32(3.0));
        for i in 0..=count {
            let g = table + i * 12;
            let code = if i < direct {
                lo + i
            } else {
                0x61 + 2 * (i - direct)
            };
            c.bus.write16(g, code as u16);
            c.bus.write8(g + 2, (r.next_u32() % 12) as u8);
            c.bus.write8(g + 8, (r.next_u32() % 16) as u8);
            c.bus.write8(g + 9, ((r.next_u32() % 7) as i8 - 2) as u8);
        }
        c.bus.write8(table + 0x900 + 2, 5);
        c.bus.write8(table + 0x900 + 8, 6);
        c.bus.write8(table + 0x900 + 9, 0xff);
        let len = if seed % 13 == 0 { 0 } else { r.next_u32() % 48 };
        const CHARS: &[u8] = b"  abcdeAB  ,.xyz\x10\x7a\x63\x64\x90\xff qrs   ";
        for i in 0..len {
            let ch = CHARS[(r.next_u32() as usize) % CHARS.len()];
            c.bus.write8(text + i, ch);
        }
        c.bus.write8(text + len, 0);
        for i in 0..64 {
            c.bus.write8(out + i, 0xcc);
        }
        c.g[A0][0] = sx(font);
        c.g[A1][0] = sx(text);
        c.g[A2][0] = sx(out);
        c.g[A3][0] = sx(match seed % 5 {
            0 => 0xffff_ffff,
            1 => 6,
            _ => 1 + r.next_u32() % 60,
        });
        c.f[12] = 0.5f32.to_bits().wrapping_add(r.next_u32() % 0x0080_0000);
        c.f[13] = match seed % 6 {
            0 => 1000.0f32.to_bits(),
            1 => 1.0f32.to_bits(),
            _ => ((r.next_u32() % 200) as f32).to_bits(),
        };
    }

    /// `Font_FindLineBreak` (0x392430): every exit (fits, word break at a
    /// space, at the end, inside an over-long word, max reached).
    #[test]
    fn font_find_line_break_matches() {
        check_lohi(
            ee::s12::f_00392430__generated,
            font_find_line_break,
            300,
            &font_find_line_break_fixture,
        );
    }

    // --- MemCardStateBase_Update ---

    use crate::difftest::{fakes, GAME_GP};

    /// The fixture's objects (above the fakes' random pointers).
    const OC397_BASE: u32 = DATA + 0x20_0000;
    /// Gate word for the callee fakes, then the forced results: a count at
    /// +4 and `(target, result)` pairs from +8 (results may be 0).
    const OC397_CTRL: u32 = OC397_BASE;
    const OC397_MAGIC: u32 = 0x0c39_7ad5;
    const OC397_SELF: u32 = OC397_BASE + 0x1000;
    const OC397_SELF_VT: u32 = OC397_BASE + 0x1800;
    const OC397_MGR: u32 = OC397_BASE + 0x2000;
    const OC397_MGR_VT: u32 = OC397_BASE + 0x2c00;
    const OC397_CARD: u32 = OC397_BASE + 0x3000;
    const OC397_CARD_VT: u32 = OC397_BASE + 0x3400;
    const OC397_STATUS: u32 = OC397_BASE + 0x4000;
    const OC397_WORLD: u32 = OC397_BASE + 0x5000;
    const OC397_POPUP: u32 = OC397_BASE + 0x6000;
    const OC397_POPUP_VT: u32 = OC397_BASE + 0x6400;
    const OC397_UI: u32 = OC397_BASE + 0x7000;
    const OC397_UI_OBJ: u32 = OC397_BASE + 0x7100;
    const OC397_UI_VT: u32 = OC397_BASE + 0x7200;
    const OC397_MENU: u32 = OC397_BASE + 0x8000;

    /// `fakes::fake` (log, random pointer result, unwinds), then the result
    /// the fixture forced for `target`, if any (0 included).
    fn oc397_mcsb_fake(c: &mut Cpu, target: u32, ret: u32) {
        fakes::fake(c, target, ret);
        if c.pc != ret {
            return;
        }
        let n = c.bus.read32(OC397_CTRL + 4);
        for i in 0..n {
            if c.bus.read32(OC397_CTRL + 8 + 8 * i) == target {
                c.g[V0][0] = c.bus.read32(OC397_CTRL + 12 + 8 * i) as u64;
            }
        }
    }

    fn oc397_mcsb_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        oc397_mcsb_fake(c, target, ra);
    }

    /// Gated fakes in the direct callees' override slots: with the gate set a
    /// call goes to [`oc397_mcsb_fake`]; otherwise the slot's previous body.
    macro_rules! oc397_gated_fakes {
        ($($name:ident: $slot:expr => $gen:expr, $addr:expr;)*) => {
            $(
                mod $name {
                    use super::{oc397_mcsb_fake, OC397_CTRL, OC397_MAGIC};
                    use crate::difftest::fakes;
                    use ssx3_ee as ee;
                    use ssx3_ee_rt::cpu::reg::RA;
                    use ssx3_ee_rt::Cpu;
                    use std::sync::atomic::AtomicUsize;
                    pub static PREV: AtomicUsize = AtomicUsize::new(0);
                    pub fn fake(c: &mut Cpu) {
                        if c.bus.read32(OC397_CTRL) != OC397_MAGIC {
                            return fakes::fall_back(c, &PREV, $gen);
                        }
                        let ra = c.g[RA][0] as u32;
                        oc397_mcsb_fake(c, $addr, ra);
                    }
                }
            )*
            fn oc397_install_fakes() {
                $(
                    fakes::hook(std::ptr::addr_of_mut!($slot), $name::fake, &$name::PREV);
                )*
            }
        };
    }

    oc397_gated_fakes! {
        oc397_fk_pause: ee::s06::OVERRIDE_00231840 => ee::s06::f_00231840__generated, 0x0023_1840;
        oc397_fk_mc_get: ee::s05::OVERRIDE_00227F80 => ee::s05::f_00227f80__generated, 0x0022_7f80;
        oc397_fk_mc_select: ee::s06::OVERRIDE_00241380 => ee::s06::f_00241380__generated, 0x0024_1380;
        oc397_fk_mc_idle: ee::s06::OVERRIDE_0023C898 => ee::s06::f_0023c898__generated, 0x0023_c898;
        oc397_fk_mc_update: ee::s06::OVERRIDE_0023D618 => ee::s06::f_0023d618__generated, 0x0023_d618;
        oc397_fk_mc_slot_name: ee::s06::OVERRIDE_00241DC8 => ee::s06::f_00241dc8__generated, 0x0024_1dc8;
        oc397_fk_new: ee::s10::OVERRIDE_00317E30 => ee::s10::f_00317e30__generated, 0x0031_7e30;
        oc397_fk_free: ee::s10::OVERRIDE_00317E98 => ee::s10::f_00317e98__generated, 0x0031_7e98;
        oc397_fk_hash: ee::s10::OVERRIDE_00317670 => ee::s10::f_00317670__generated, 0x0031_7670;
        oc397_fk_ustr: ee::s08::OVERRIDE_002C2540 => ee::s08::f_002c2540__generated, 0x002c_2540;
        oc397_fk_find: ee::s13::OVERRIDE_0039D860 => ee::s13::f_0039d860__generated, 0x0039_d860;
    }

    /// A `{delta, fn}` vtable slot calling the fake method `FAKE + 0x10*k`.
    fn oc397_slot(c: &mut Cpu, vt: u32, off: u32, k: u32, delta: i16) -> u32 {
        c.bus.write16(vt + off, delta as u16);
        let target = fakes::FAKE + 0x10 * k;
        c.bus.write32(vt + off + 4, target);
        target
    }

    /// Every state (0-6 and out of range), first frame or not, popup or
    /// not, busy or not, in game (overlay) or front end, the card-check
    /// waits, the manager modes, the status message (buffer or none), and
    /// the menu selection, against logging fakes. The screen updates
    /// (`cOVStateOverlay_Update`, `UIState_Update`, no override slots) run
    /// for real until their first fake call, which unwinds.
    #[test]
    fn mem_card_state_base_update_matches() {
        oc397_install_fakes();
        fakes::covered(
            ee::s04::f_001d8de0__generated,
            mem_card_state_base_update,
            240,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1d_8de0, 14);
                c.hooks.miss = oc397_mcsb_miss;
                c.pc = 0x001d_8de0;
                c.bus.write32(OC397_CTRL, OC397_MAGIC);
                let mut forced: Vec<(u32, u32)> = Vec::new();
                let bit = |r: &mut crate::difftest::Rng, n: u32| r.next_u32() % n == 0;
                // Globals: the world (gp-0x848) and the status (gp-0x10c8).
                c.bus.write32(GAME_GP - 0x848, OC397_WORLD);
                c.bus.write32(GAME_GP - 0x10c8, OC397_STATUS);
                // The per-state jump table.
                for (i, t) in [
                    0x001d_8f24u32,
                    0x001d_8f34,
                    0x001d_8f44,
                    0x001d_8f54,
                    0x001d_8f64,
                    0x001d_8f74,
                    0x001d_8f90,
                ]
                .iter()
                .enumerate()
                {
                    c.bus.write32(MCSB_UPDATE_JUMP_TABLE + 4 * i as u32, *t);
                }
                // Self and its vtable.
                c.g[A0][0] = sx(OC397_SELF);
                c.bus.write32(OC397_SELF + MCSB_UPDATE_VT, OC397_SELF_VT);
                for (k, off) in [
                    0xd0u32, 0xd8, 0xe0, 0xe8, 0xf0, 0xf8, 0x100, 0x108, 0x118, 0x130, 0x138, 0x178,
                ]
                .iter()
                .enumerate()
                {
                    let delta = ((r.next_u32() % 5) as i16 - 2) * 4;
                    oc397_slot(c, OC397_SELF_VT, *off, k as u32, delta);
                }
                let message = bit(&mut r, 3);
                let popup = message || bit(&mut r, 3);
                c.bus.write32(
                    OC397_SELF + MCSB_UPDATE_POPUP,
                    if popup { OC397_POPUP } else { 0 },
                );
                let busy = bit(&mut r, 2);
                c.bus.write32(OC397_SELF + MCSB_UPDATE_BUSY, busy as u32);
                let in_game = bit(&mut r, 2);
                c.bus.write32(OC397_WORLD + 0x84, in_game as u32);
                c.bus
                    .write32(OC397_SELF + MCSB_UPDATE_STARTED, (!bit(&mut r, 3)) as u32);
                c.bus.write32(OC397_SELF + MCSB_UPDATE_STATE, seed % 9);
                c.bus
                    .write32(OC397_SELF + MCSB_UPDATE_CHECK_MODE, r.next_u32() % 3);
                c.bus
                    .write32(OC397_SELF + MCSB_UPDATE_SELECTION, r.next_u32() % 8);
                c.bus
                    .write32(OC397_SELF + MCSB_UPDATE_SCREEN, OC397_BASE + 0x9000);
                // The screen update's first call: UIState_Update's slot 0x160.
                c.bus.write32(OC397_SELF + 0x10, OC397_UI);
                c.bus.write32(OC397_UI + 0xc, OC397_UI_OBJ);
                c.bus.write32(OC397_UI_OBJ + 8, OC397_UI_VT);
                oc397_slot(c, OC397_UI_VT, 0x160, 20, 0);
                if !popup && !busy {
                    // Unwind in the screen update (its first fake call).
                    c.bus
                        .write32(fakes::LOG_UNWIND_AT, if in_game { 1 } else { 0 });
                } else if seed % 4 != 1 {
                    c.bus.write32(fakes::LOG_UNWIND_AT, u32::MAX);
                }
                // The manager, its vtable and card.
                forced.push((0x0022_7f80, OC397_MGR));
                c.bus.write32(OC397_MGR + MCSB_UPDATE_MGR_VT, OC397_MGR_VT);
                oc397_slot(c, OC397_MGR_VT, 0x8, 21, 4);
                c.bus.write32(OC397_MGR + MCSB_UPDATE_MGR_CARD, OC397_CARD);
                c.bus.write32(OC397_CARD, OC397_CARD_VT);
                for (k, off) in [0x58u32, 0x1a8, 0x218, 0x258].iter().enumerate() {
                    let t = oc397_slot(c, OC397_CARD_VT, *off, 22 + k as u32, -8);
                    forced.push((t, (r.next_u32() % 2) * (1 + r.next_u32() % 3)));
                }
                forced.push((0x0023_c898, r.next_u32() % 2));
                // Status flags; the manager's mode (>= 0x38 for the short
                // status text path).
                let flag = |r: &mut crate::difftest::Rng| (r.next_u32() % 4 == 0) as u32;
                c.bus.write32(OC397_STATUS + 0x210, message as u32);
                let f208 = flag(&mut r);
                let f204 = flag(&mut r);
                c.bus.write32(OC397_STATUS + 0x208, f208);
                c.bus.write32(OC397_STATUS + 0x204, f204);
                let mode = if message {
                    0x38 + r.next_u32() % 8
                } else if bit(&mut r, 2) {
                    MCSB_UPDATE_MODES[(r.next_u32() % 13) as usize] as u32
                } else {
                    r.next_u32() % 0x30
                };
                c.bus.write32(OC397_MGR + MCSB_UPDATE_MGR_MODE, mode);
                // The status text buffer (or none), the popup.
                forced.push((
                    0x0031_7e30,
                    if bit(&mut r, 3) {
                        0
                    } else {
                        OC397_BASE + 0xa000
                    },
                ));
                c.bus.write32(OC397_POPUP + 8, OC397_POPUP_VT);
                oc397_slot(c, OC397_POPUP_VT, 0xd8, 26, 0);
                // The menu object (or none) and its selection byte.
                forced.push((0x0039_d860, if bit(&mut r, 2) { 0 } else { OC397_MENU }));
                c.bus.write8(OC397_MENU + 0x95, r.next_u32() as u8);
                c.bus.write32(OC397_CTRL + 4, forced.len() as u32);
                for (i, (t, v)) in forced.iter().enumerate() {
                    c.bus.write32(OC397_CTRL + 8 + 8 * i as u32, *t);
                    c.bus.write32(OC397_CTRL + 12 + 8 * i as u32, *v);
                }
            },
        );
    }
}
