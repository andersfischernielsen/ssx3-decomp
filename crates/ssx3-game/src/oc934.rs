//! Staging module oc934 (wave wave255): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc934`.

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
        0x001c_ac98,
        c_ui_state_buy_popup_on_widget_created,
        "cUIStateBuyPopup_OnWidgetCreated",
    ),
    (
        0x001c_c8d8,
        c_keyboard_popup_update_scroll_window,
        "cKeyboardPopup_UpdateScrollWindow",
    ),
    (
        0x001c_cc30,
        c_keyboard_popup_update_cursor,
        "cKeyboardPopup_UpdateCursor",
    ),
    (
        0x001c_d348,
        c_keyboard_popup_transition_in,
        "cKeyboardPopup_TransitionIn",
    ),
    (
        0x001c_d708,
        c_keyboard_popup_refresh_keys,
        "cKeyboardPopup_RefreshKeys",
    ),
];

// ---------------------------------------------------------------------------
// Small helpers shared by the functions below (register-level, as the
// generated code does them).

/// The low 32 bits of a register (an address).
fn kbd_r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// `off(reg)` as an address.
fn kbd_ea(c: &Cpu, reg: usize, off: u32) -> u32 {
    kbd_r(c, reg).wrapping_add(off)
}

/// `addiu dst, src, off`.
fn kbd_addi(c: &mut Cpu, dst: usize, src: usize, off: u32) {
    c.g[dst][0] = sx(kbd_ea(c, src, off));
}

/// `lw dst, off(base)`.
fn kbd_lw(c: &mut Cpu, dst: usize, base: usize, off: u32) {
    let a = kbd_ea(c, base, off);
    c.g[dst][0] = c.lw(a);
}

/// `lwc1 f, off(base)`.
fn kbd_lwc1(c: &mut Cpu, f: usize, base: usize, off: u32) {
    let a = kbd_ea(c, base, off);
    c.lwc1(f, a);
}

/// The widget virtual call the compiler emits for `obj->vtbl[slot]`: the
/// vtable is `*(obj + 8)`, each entry is `{i16 delta, fn at +4}` at `slot`.
/// Leaves `v0` (vtable), `a0` (this + delta) and `v1` (function) as the
/// generated code does, then calls it returning to `ret`.
fn kbd_vcall(c: &mut Cpu, obj: u32, slot: u32, ret: u32) -> Flow {
    let ea = obj.wrapping_add(8);
    c.g[V0][0] = c.lw(ea);
    let ea = kbd_ea(c, V0, slot);
    c.g[A0][0] = c.lh(ea);
    let ea = kbd_ea(c, V0, slot + 4);
    c.g[V1][0] = c.lw(ea);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// `GetHashValue32` (StrHash).
const KBD_STR_HASH: u32 = 0x0031_7670;
/// `cUIScreen_getObjectByHashName(screen, hash)`.
const KBD_FIND_OBJECT: u32 = 0x0039_d860;
/// `strlen`.
const KBD_STRLEN: u32 = 0x0041_6810;
/// `strncpy`.
const KBD_STRNCPY: u32 = 0x0041_6ce0;
/// `cUIText_getFont`.
const KBD_GET_FONT: u32 = 0x003a_04f0;
/// `Font::Measure(font, str, rect *, flag)`.
const KBD_MEASURE: u32 = 0x0039_1fb0;
/// `cUIText_setAsciiString`.
const KBD_SET_ASCII: u32 = 0x003a_0c50;

fn kbd_apply_font_scale() -> Callee {
    Callee {
        addr: 0x001c_e4c8,
        f: ee::s04::f_001ce4c8,
    }
}

// ---------------------------------------------------------------------------
// cUIStateBuyPopup_OnWidgetCreated

/// Offsets in `cUIStateBuyPopup` (the popup info block).
pub(crate) const BUY_POPUP_ITEM_NAME_TYPE: u32 = 0x54;
pub(crate) const BUY_POPUP_ITEM_TYPE_TYPE: u32 = 0x5c;
pub(crate) const BUY_POPUP_QUESTION_TYPE: u32 = 0x64;
pub(crate) const BUY_POPUP_CREDITS_LEFT: u32 = 0x50;

/// The text-source switch of one string widget: the string type word at
/// `type_off` selects `setAsciiString` (2), `setUnicodeString` (1) or
/// `setUnicodeStringByID` (3) with the value at `ptr_off`; any other type does
/// nothing. `question` is the third string, whose branches are one cycle
/// shorter (the delay slots sit in the same block).
fn buy_popup_string_case(
    c: &mut Cpu,
    frame: Frame,
    type_off: u32,
    ptr_off: u32,
    question: bool,
) -> Flow {
    let slot = if question { 1 } else { 2 };
    c.cycles += 4;
    kbd_lw(c, V1, S0, type_off);
    c.g[V0][0] = 1;
    let ty = c.g[V1][0];
    let (target, ret) = if ty == 2 {
        c.cycles += slot;
        kbd_lw(c, A1, S0, ptr_off);
        (0x003a_0c50, 0x001c_ad78)
    } else {
        c.cycles += 2;
        c.g[V0][0] = 3;
        if ty == 1 {
            c.cycles += slot;
            kbd_lw(c, A1, S0, ptr_off);
            (0x003a_0e90, 0x001c_ad94)
        } else {
            c.cycles += 2;
            if ty != 3 {
                frame.restore(c, S0, 0x40);
                return Ok(());
            }
            c.cycles += slot;
            kbd_lw(c, A1, S0, ptr_off);
            (0x003a_1000, 0x001c_adb0)
        }
    };
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, target, ret)?;
    frame.restore(c, S0, 0x40);
    Ok(())
}

/// Money answer: `setAsciiString(text, getDollarString(value))`, `value`
/// already in `a0`.
fn buy_popup_dollar_text(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    call_ptr(c, 0x0019_8af0, 0x001c_ae04)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, KBD_SET_ASCII, 0x001c_ae10)?;
    frame.restore(c, S0, 0x40);
    Ok(())
}

/// `void cUIStateBuyPopup_OnWidgetCreated(cUIStateBuyPopup *this, cUIText *widget)`
/// (0x1cac98): fills a freshly created widget of the buy popup by the hash of
/// its name: the item type, item name and question strings (ascii, wide or
/// by text id), the cost and the money in dollars (cash purchases), the
/// remaining credits (`"%d"`, credit purchases) and hides the group that does
/// not apply.
pub fn c_ui_state_buy_popup_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[A0][0] = 0x46_0000;
        kbd_lw(c, S1, S2, 0x38);
        kbd_addi(c, A0, A0, 0x62d8);
        call_ptr(c, KBD_STR_HASH, 0x001c_acc4)?;
        'end: {
            // item type
            if c.g[S1][0] == c.g[V0][0] {
                buy_popup_string_case(c, frame, BUY_POPUP_ITEM_TYPE_TYPE, 0x60, false)?;
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // item name
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x62e8);
            call_ptr(c, KBD_STR_HASH, 0x001c_ad0c)?;
            if c.g[S1][0] == c.g[V0][0] {
                buy_popup_string_case(c, frame, BUY_POPUP_ITEM_NAME_TYPE, 0x58, false)?;
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // question
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x62f8);
            call_ptr(c, KBD_STR_HASH, 0x001c_ad54)?;
            if c.g[S1][0] == c.g[V0][0] {
                buy_popup_string_case(c, frame, BUY_POPUP_QUESTION_TYPE, 0x68, true)?;
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // cost answer (cash only)
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x5f0);
            call_ptr(c, KBD_STR_HASH, 0x001c_adc0)?;
            if c.g[S1][0] == c.g[V0][0] {
                c.cycles += 3;
                kbd_lw(c, V0, S0, BUY_POPUP_CREDITS_LEFT);
                if (c.g[V0][0] as i64) > 0 {
                    frame.restore(c, S0, 0x40);
                    break 'end;
                }
                c.cycles += 2;
                kbd_lw(c, A0, S0, 0x48);
                buy_popup_dollar_text(c, frame)?;
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // "you have" answer (cash only)
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x610);
            call_ptr(c, KBD_STR_HASH, 0x001c_ade4)?;
            if c.g[S1][0] == c.g[V0][0] {
                c.cycles += 3;
                kbd_lw(c, V0, S0, BUY_POPUP_CREDITS_LEFT);
                if (c.g[V0][0] as i64) > 0 {
                    frame.restore(c, S0, 0x40);
                    break 'end;
                }
                c.cycles += 1;
                kbd_lw(c, A0, S0, 0x4c);
                buy_popup_dollar_text(c, frame)?;
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // credits remaining ("%d")
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x6308);
            call_ptr(c, KBD_STR_HASH, 0x001c_ae20)?;
            if c.g[S1][0] == c.g[V0][0] {
                c.cycles += 3;
                kbd_lw(c, A2, S0, BUY_POPUP_CREDITS_LEFT);
                let none = (c.g[A2][0] as i64) <= 0;
                c.g[A1][0] = 0x4a_0000;
                if none {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x40);
                    break 'end;
                }
                c.cycles += 8;
                c.g[A0][0] = c.g[SP][0];
                kbd_addi(c, A1, A1, 0x12c0);
                call_ptr(c, 0x0041_7828, 0x001c_ae40)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, KBD_SET_ASCII, 0x001c_ae4c)?;
                frame.restore(c, S0, 0x40);
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // group "credit": hidden when paying cash
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x6318);
            call_ptr(c, KBD_STR_HASH, 0x001c_ae5c)?;
            if c.g[S1][0] == c.g[V0][0] {
                c.cycles += 3;
                kbd_lw(c, V0, S0, BUY_POPUP_CREDITS_LEFT);
                let visible = (c.g[V0][0] as i64) > 0;
                frame.restore(c, S0, 0x40);
                if visible {
                    break 'end;
                }
                c.cycles += 8;
                let widget = kbd_r(c, S2);
                c.g[A1][0] = 0;
                kbd_vcall(c, widget, 0x48, 0x001c_ae88)?;
                frame.restore(c, S0, 0x40);
                break 'end;
            }
            c.g[A0][0] = 0x46_0000;
            // group "cash": hidden when paying with credits
            c.cycles += 4;
            kbd_addi(c, A0, A0, 0x6328);
            call_ptr(c, KBD_STR_HASH, 0x001c_ae98)?;
            if c.g[S1][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x40);
                break 'end;
            }
            c.cycles += 3;
            kbd_lw(c, V0, S0, BUY_POPUP_CREDITS_LEFT);
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[A1][0] = 0;
            if none {
                c.cycles += 1;
                frame.restore(c, S0, 0x40);
                break 'end;
            }
            c.cycles += 5;
            let widget = kbd_r(c, S2);
            kbd_vcall(c, widget, 0x48, 0x001c_aec0)?;
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

// ---------------------------------------------------------------------------
// cKeyboardPopup

/// Offsets in `cKeyboardPopup`.
pub(crate) const KBD_POPUP_SCREEN: u32 = 0x40;
pub(crate) const KBD_POPUP_MAX_LEN: u32 = 0x60;
pub(crate) const KBD_POPUP_MASKED: u32 = 0x70;
pub(crate) const KBD_POPUP_TEXT: u32 = 0x74;
pub(crate) const KBD_POPUP_BUF: u32 = 0xb4;
pub(crate) const KBD_POPUP_MASK_BUF: u32 = 0xf4;
pub(crate) const KBD_POPUP_CURSOR: u32 = 0x134;
pub(crate) const KBD_POPUP_START: u32 = 0x138;
pub(crate) const KBD_POPUP_END: u32 = 0x13c;
pub(crate) const KBD_POPUP_WIDTH: u32 = 0x3f4;
pub(crate) const KBD_POPUP_MARGIN: u32 = 0x400;

/// The tail every re-measure of the scroll window shares: terminate the copied
/// window text at `buf + n` (`s0 + s1`), apply the font scale, fetch the font
/// of the text object (`s3`) and measure the window into the rect on the
/// stack (`sp`).
fn kbd_scroll_measure(c: &mut Cpu, ret_scale: u32, ret_font: u32, ret_measure: u32) -> Flow {
    c.g[V1][0] = sx(kbd_r(c, S0).wrapping_add(kbd_r(c, S1)));
    c.g[A1][0] = c.g[S3][0];
    let ea = kbd_r(c, V1);
    c.sb(ea, 0);
    c.g[A0][0] = c.g[S2][0];
    kbd_apply_font_scale().call(c, ret_scale)?;
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, KBD_GET_FONT, ret_font)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    kbd_lwc1(c, 12, A0, 0x38);
    c.g[A2][0] = c.g[SP][0];
    kbd_lwc1(c, 13, A0, 0x3c);
    c.g[A3][0] = 0;
    call_ptr(c, KBD_MEASURE, ret_measure)
}

/// `void cKeyboardPopup_UpdateScrollWindow(cKeyboardPopup *this, int dir)`
/// (0x1cc8d8): re-fits the visible window `[start, end)` of the edit text so
/// that the measured text fits the field width (shrinking from the end when
/// `dir` is set, from the start otherwise; then growing while it still
/// fits), copies it to the display buffer, and shows or hides the left and
/// right scroll arrows.
pub fn c_keyboard_popup_update_scroll_window(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S2, 0x80);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        kbd_addi(c, S4, S2, KBD_POPUP_TEXT);
        frame.save(c, S0, 0xa0);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S1, 0x90);
        c.g[A0][0] = c.g[S4][0];
        frame.save(c, S3, 0x70);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 20, 0xb0);
        call_ptr(c, KBD_STRLEN, 0x001c_c91c)?;
        let empty = c.g[V0][0] == 0;
        kbd_addi(c, S0, S2, KBD_POPUP_BUF);
        'fitted: {
            if empty {
                c.cycles += 3;
                let ea = kbd_ea(c, S2, KBD_POPUP_BUF);
                c.sb(ea, 0);
                c.g[S7][0] = 0x46_0000;
                c.g[FP][0] = 0x46_0000;
                break 'fitted;
            }
            c.cycles += 36;
            c.g[A0][0] = 0x46_0000;
            kbd_addi(c, A0, A0, 0x6530);
            c.g[S6][0] = c.g[S0][0];
            call_ptr(c, KBD_STR_HASH, 0x001c_c934)?;
            kbd_lw(c, A0, S2, KBD_POPUP_SCREEN);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, KBD_FIND_OBJECT, 0x001c_c940)?;
            c.g[S3][0] = c.g[V0][0];
            // Copy the current window and measure it.
            kbd_lw(c, V1, S2, KBD_POPUP_END);
            kbd_lw(c, V0, S2, KBD_POPUP_START);
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx(kbd_r(c, V1).wrapping_sub(kbd_r(c, V0)));
            c.g[A1][0] = sx(kbd_r(c, V0).wrapping_add(KBD_POPUP_TEXT));
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = sx(kbd_r(c, S2).wrapping_add(kbd_r(c, A1)));
            call_ptr(c, KBD_STRNCPY, 0x001c_c964)?;
            kbd_scroll_measure(c, 0x001c_c978, 0x001c_c980, 0x001c_c99c)?;
            c.g[A0][0] = c.g[S4][0];
            kbd_lwc1(c, 20, S2, KBD_POPUP_WIDTH);
            call_ptr(c, KBD_STRLEN, 0x001c_c9a8)?;
            kbd_lw(c, V1, S2, KBD_POPUP_CURSOR);
            let cursor_not_at_end = c.g[V1][0] != c.g[V0][0];
            kbd_lwc1(c, 0, SP, 8);
            if cursor_not_at_end {
                c.cycles += 1;
                kbd_addi(c, S4, S2, KBD_POPUP_TEXT);
            } else {
                // The cursor is at the end: leave room for the margin.
                c.cycles += 6;
                kbd_lwc1(c, 1, S2, KBD_POPUP_MARGIN);
                c.sub_s(20, 20, 1);
                c.c_lt_s(20, 0);
                let fits = !c.fcc();
                kbd_addi(c, S4, S2, KBD_POPUP_TEXT);
                if !fits {
                    c.cycles += 2;
                    if c.g[S5][0] != 0 {
                        c.add_s(20, 20, 1);
                    } else {
                        c.cycles += 1;
                        kbd_lwc1(c, 0, SP, 8);
                        c.cycles += 1;
                        kbd_addi(c, S4, S2, KBD_POPUP_TEXT);
                    }
                }
            }
            // Shrink while too wide.
            c.cycles += 5;
            c.g[S7][0] = 0x46_0000;
            c.c_lt_s(20, 0);
            let too_wide = c.fcc();
            c.g[FP][0] = 0x46_0000;
            if too_wide {
                c.cycles += 2;
                c.g[S0][0] = c.g[S6][0];
                loop {
                    c.cycles += 2;
                    if c.g[S5][0] == 0 {
                        kbd_lw(c, V0, S2, KBD_POPUP_START);
                        c.cycles += 2;
                        c.g[V0][0] = sx(kbd_r(c, V0).wrapping_add(1));
                        let ea = kbd_ea(c, S2, KBD_POPUP_START);
                        c.sw(ea, c.g[V0][0]);
                    } else {
                        c.cycles += 4;
                        kbd_lw(c, V0, S2, KBD_POPUP_END);
                        c.g[V0][0] = sx(kbd_r(c, V0).wrapping_sub(1));
                        let ea = kbd_ea(c, S2, KBD_POPUP_END);
                        c.sw(ea, c.g[V0][0]);
                    }
                    c.cycles += 26;
                    kbd_lw(c, V1, S2, KBD_POPUP_START);
                    c.g[A0][0] = c.g[S0][0];
                    kbd_lw(c, V0, S2, KBD_POPUP_END);
                    c.g[A1][0] = sx(kbd_r(c, V1).wrapping_add(KBD_POPUP_TEXT));
                    c.g[S1][0] = sx(kbd_r(c, V0).wrapping_sub(kbd_r(c, V1)));
                    c.g[A1][0] = sx(kbd_r(c, S2).wrapping_add(kbd_r(c, A1)));
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, KBD_STRNCPY, 0x001c_ca38)?;
                    kbd_scroll_measure(c, 0x001c_ca4c, 0x001c_ca54, 0x001c_ca70)?;
                    kbd_lwc1(c, 1, SP, 8);
                    c.c_lt_s(20, 1);
                    if c.fcc() {
                        if event_due(c) {
                            deliver_event_here(c, 0x001c_c9f8);
                        }
                        continue;
                    }
                    break;
                }
            }
            // Grow while the window still fits.
            c.cycles += 7;
            kbd_lwc1(c, 0, S2, KBD_POPUP_WIDTH);
            kbd_lwc1(c, 1, S2, KBD_POPUP_MARGIN);
            kbd_lwc1(c, 2, SP, 8);
            c.sub_s(0, 0, 1);
            c.c_lt_s(2, 0);
            if !c.fcc() {
                break 'fitted;
            }
            c.cycles += 6;
            kbd_lw(c, V0, S2, KBD_POPUP_END);
            kbd_lw(c, V1, S2, KBD_POPUP_MAX_LEN);
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            let full = c.g[V0][0] == 0;
            c.g[S0][0] = c.g[S6][0];
            if full {
                break 'fitted;
            }
            c.cycles += 1;
            kbd_lw(c, A1, S2, KBD_POPUP_START);
            loop {
                c.cycles += 29;
                c.g[V0][0] = sx(kbd_r(c, A0).wrapping_add(1));
                c.g[A0][0] = c.g[S0][0];
                let ea = kbd_ea(c, S2, KBD_POPUP_END);
                c.sw(ea, c.g[V0][0]);
                c.g[S1][0] = sx(kbd_r(c, V0).wrapping_sub(kbd_r(c, A1)));
                c.g[A1][0] = sx(kbd_r(c, A1).wrapping_add(KBD_POPUP_TEXT));
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = sx(kbd_r(c, S2).wrapping_add(kbd_r(c, A1)));
                call_ptr(c, KBD_STRNCPY, 0x001c_cad8)?;
                kbd_scroll_measure(c, 0x001c_caec, 0x001c_caf4, 0x001c_cb10)?;
                kbd_lwc1(c, 1, S2, KBD_POPUP_WIDTH);
                kbd_lwc1(c, 0, S2, KBD_POPUP_MARGIN);
                kbd_lwc1(c, 2, SP, 8);
                c.sub_s(1, 1, 0);
                c.c_lt_s(2, 1);
                if !c.fcc() {
                    break 'fitted;
                }
                c.cycles += 6;
                kbd_lw(c, V0, S2, KBD_POPUP_END);
                kbd_lw(c, V1, S2, KBD_POPUP_MAX_LEN);
                c.g[A0][0] = c.g[V0][0];
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    kbd_lw(c, A1, S2, KBD_POPUP_START);
                    if event_due(c) {
                        deliver_event_here(c, 0x001c_cab8);
                    }
                    continue;
                }
                c.cycles += 2;
                break 'fitted;
            }
        }
        // Left arrow: shown when the window does not start at 0.
        c.cycles += 23;
        c.g[A0][0] = sx(kbd_r(c, S7).wrapping_add(0x6540));
        call_ptr(c, KBD_STR_HASH, 0x001c_cb60)?;
        kbd_lw(c, A0, S2, KBD_POPUP_SCREEN);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, KBD_FIND_OBJECT, 0x001c_cb6c)?;
        c.g[S3][0] = c.g[V0][0];
        kbd_lw(c, A1, S2, KBD_POPUP_START);
        c.g[A1][0] = (c.g[A1][0] != 0) as u64;
        let left = kbd_r(c, S3);
        kbd_vcall(c, left, 0x48, 0x001c_cb8c)?;
        // Right arrow: shown when the window ends before the text does.
        c.g[A0][0] = sx(kbd_r(c, FP).wrapping_add(0x6558));
        call_ptr(c, KBD_STR_HASH, 0x001c_cb94)?;
        kbd_lw(c, A0, S2, KBD_POPUP_SCREEN);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, KBD_FIND_OBJECT, 0x001c_cba0)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, KBD_STRLEN, 0x001c_cbac)?;
        if c.g[V0][0] == 0 {
            kbd_lw(c, V0, S3, 8);
            c.cycles += 6;
            c.g[A1][0] = 0;
            let right = kbd_r(c, S3);
            kbd_vcall(c, right, 0x48, 0x001c_cbf8)?;
            frame.restore(c, S0, 0xa0);
        } else {
            c.cycles += 12;
            kbd_lw(c, S1, S3, 8);
            c.g[A0][0] = c.g[S4][0];
            let ea = kbd_ea(c, S1, 0x48);
            c.g[S0][0] = c.lh(ea);
            c.g[S0][0] = sx(kbd_r(c, S3).wrapping_add(kbd_r(c, S0)));
            call_ptr(c, KBD_STRLEN, 0x001c_cbc8)?;
            kbd_lw(c, A1, S2, KBD_POPUP_END);
            c.g[A0][0] = c.g[S0][0];
            kbd_lw(c, V1, S1, 0x4c);
            let target = kbd_r(c, V1);
            c.g[A1][0] = (c.g[A1][0] < c.g[V0][0]) as u64;
            call_ptr(c, target, 0x001c_cbdc)?;
            frame.restore(c, S0, 0xa0);
        }
        c.cycles += 12;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    });
}

/// `void cKeyboardPopup_UpdateCursor(cKeyboardPopup *this)` (0x1ccc30): puts the
/// cursor widget after the visible text (`+0x3e8` is the origin of the
/// text object, the cursor x is the measured width of the characters up to
/// the cursor, `+0x40c` the origin of the cursor widget) and the blink
/// marker after the character under the cursor (`*` when the field is masked).
pub fn c_keyboard_popup_update_cursor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        c.g[S0][0] = 0x46_0000;
        kbd_addi(c, S0, S0, 0x6530);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x90);
        frame.save(c, S2, 0xa0);
        frame.save_ra(c, 0x80);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, KBD_STR_HASH, 0x001c_cc5c)?;
        kbd_lw(c, A0, S1, KBD_POPUP_SCREEN);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, KBD_FIND_OBJECT, 0x001c_cc68)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, KBD_STR_HASH, 0x001c_cc74)?;
        kbd_lw(c, A0, S1, KBD_POPUP_SCREEN);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, KBD_FIND_OBJECT, 0x001c_cc80)?;
        c.g[S0][0] = c.g[V0][0];
        let no_text = c.g[S0][0] == 0;
        kbd_addi(c, A1, S1, KBD_POPUP_WIDTH);
        if !no_text {
            // text->getPos(&this->origin) (vtable slot 20), origin = text->pos.
            c.cycles += 11;
            kbd_lw(c, V0, S0, 8);
            let ea = kbd_ea(c, V0, 0xa0);
            c.g[A0][0] = c.lh(ea);
            kbd_lw(c, V1, V0, 0xa4);
            let target = kbd_r(c, V1);
            c.g[A0][0] = sx(kbd_r(c, S0).wrapping_add(kbd_r(c, A0)));
            call_ptr(c, target, 0x001c_cca0)?;
            let ea = kbd_ea(c, S0, 0x4b);
            c.ldl(V0, ea);
            let ea = kbd_ea(c, S0, 0x44);
            c.ldr(V0, ea);
            kbd_lw(c, V1, S0, 0x4c);
            let ea = kbd_ea(c, S1, 0x3ef);
            c.sdl(V0, ea);
            let ea = kbd_ea(c, S1, 0x3e8);
            c.sdr(V0, ea);
            let ea = kbd_ea(c, S1, 0x3f0);
            c.sw(ea, c.g[V1][0]);
        }
        // cursor y and the character under the cursor.
        c.cycles += 21;
        kbd_lwc1(c, 0, S1, 0x3f8);
        kbd_lwc1(c, 1, S1, 0x3ec);
        kbd_lwc1(c, 2, S1, 0x404);
        c.add_s(0, 0, 1);
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 3);
        kbd_lw(c, V0, S1, KBD_POPUP_CURSOR);
        c.sub_s(0, 0, 2);
        c.g[V0][0] = sx(kbd_r(c, S1).wrapping_add(kbd_r(c, V0)));
        c.sub_s(0, 0, 3);
        let ea = kbd_ea(c, S1, 0x410);
        c.swc1(0, ea);
        let ea = kbd_ea(c, V0, KBD_POPUP_TEXT);
        c.g[A0][0] = c.lbu(ea);
        let ea = kbd_ea(c, V0, KBD_POPUP_TEXT);
        c.g[V1][0] = c.lb(ea);
        let ea = kbd_ea(c, S1, 0x407);
        c.ldl(A2, ea);
        let ea = kbd_ea(c, S1, 0x400);
        c.ldr(A2, ea);
        kbd_lw(c, A3, S1, 0x408);
        let ea = kbd_ea(c, SP, 0x7);
        c.sdl(A2, ea);
        let ea = kbd_ea(c, SP, 0);
        c.sdr(A2, ea);
        let ea = kbd_ea(c, SP, 8);
        c.sw(ea, c.g[A3][0]);
        if c.g[V1][0] != 0 {
            // Measure the character under the cursor (`*` when masked).
            c.cycles += 3;
            kbd_lw(c, V0, S1, KBD_POPUP_MASKED);
            let masked = c.g[V0][0] != 0;
            let ea = kbd_ea(c, SP, 0x21);
            c.sb(ea, 0);
            if masked {
                c.cycles += 3;
                c.g[V0][0] = 0x2a;
                let ea = kbd_ea(c, SP, 0x20);
                c.sb(ea, c.g[V0][0]);
            } else {
                c.cycles += 1;
                let ea = kbd_ea(c, SP, 0x20);
                c.sb(ea, c.g[A0][0]);
            }
            c.cycles += 17;
            c.g[A1][0] = c.g[S3][0];
            c.g[A0][0] = c.g[S1][0];
            kbd_apply_font_scale().call(c, 0x001c_cd34)?;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, KBD_GET_FONT, 0x001c_cd3c)?;
            let ea = kbd_ea(c, SP, 0x20);
            c.g[A3][0] = c.lb(ea);
            c.g[A0][0] = c.g[V0][0];
            kbd_lwc1(c, 12, A0, 0x38);
            kbd_addi(c, A1, SP, 0x20);
            c.g[A3][0] ^= 0x20;
            kbd_lwc1(c, 13, A0, 0x3c);
            c.g[A3][0] = (c.g[A3][0] < 1) as u64;
            kbd_addi(c, A2, SP, 0x10);
            call_ptr(c, KBD_MEASURE, 0x001c_cd60)?;
            let ea = kbd_ea(c, SP, 0x18);
            c.lwc1(1, ea);
            let ea = kbd_ea(c, SP, 0);
            c.swc1(1, ea);
        }
        c.g[A0][0] = 0x46_0000;
        // cursor widget position (vtable slot 21) from the window text.
        c.cycles += 15;
        kbd_addi(c, A0, A0, 0x6570);
        call_ptr(c, KBD_STR_HASH, 0x001c_cd74)?;
        kbd_lw(c, A0, S1, KBD_POPUP_SCREEN);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, KBD_FIND_OBJECT, 0x001c_cd80)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[SP][0];
        let obj = kbd_r(c, S0);
        c.g[V0][0] = c.lw(obj.wrapping_add(8));
        let ea = kbd_ea(c, V0, 0xa8);
        c.g[A0][0] = c.lh(ea);
        let ea = kbd_ea(c, V0, 0xac);
        c.g[V1][0] = c.lw(ea);
        let target = kbd_r(c, V1);
        c.g[A0][0] = sx(obj.wrapping_add(kbd_r(c, A0)));
        call_ptr(c, target, 0x001c_cd9c)?;
        kbd_lw(c, V0, S1, KBD_POPUP_MASKED);
        if c.g[V0][0] == 0 {
            kbd_lw(c, A2, S1, KBD_POPUP_CURSOR);
            c.cycles += 8;
            c.g[V0][0] = sx(kbd_r(c, SP).wrapping_add(0x30));
            kbd_lw(c, V1, S1, KBD_POPUP_START);
            c.g[A0][0] = c.g[V0][0];
            kbd_addi(c, A1, S1, KBD_POPUP_BUF);
            c.g[A2][0] = sx(kbd_r(c, A2).wrapping_sub(kbd_r(c, V1)));
            c.g[S2][0] = c.g[V0][0];
            call_ptr(c, KBD_STRNCPY, 0x001c_cdf8)?;
            kbd_lw(c, V1, S1, KBD_POPUP_START);
        } else {
            c.cycles += 13;
            c.g[A0][0] = c.g[S1][0];
            kbd_addi(c, A1, S1, KBD_POPUP_BUF);
            Callee {
                addr: 0x001c_ce98,
                f: ee::s04::f_001cce98,
            }
            .call(c, 0x001c_cdb4)?;
            kbd_lw(c, A2, S1, KBD_POPUP_CURSOR);
            c.g[V0][0] = sx(kbd_r(c, SP).wrapping_add(0x30));
            kbd_lw(c, V1, S1, KBD_POPUP_START);
            c.g[A0][0] = c.g[V0][0];
            kbd_addi(c, A1, S1, KBD_POPUP_MASK_BUF);
            c.g[S2][0] = c.g[V0][0];
            c.g[A2][0] = sx(kbd_r(c, A2).wrapping_sub(kbd_r(c, V1)));
            call_ptr(c, KBD_STRNCPY, 0x001c_cdd4)?;
            kbd_lw(c, V1, S1, KBD_POPUP_START);
        }
        // Measure the text up to the cursor; the cursor x is its width.
        c.cycles += 39;
        c.g[A1][0] = c.g[S3][0];
        kbd_lw(c, V0, S1, KBD_POPUP_CURSOR);
        c.g[A0][0] = c.g[S1][0];
        c.g[V0][0] = sx(kbd_r(c, V0).wrapping_sub(kbd_r(c, V1)));
        c.g[V0][0] = sx(kbd_r(c, S2).wrapping_add(kbd_r(c, V0)));
        let ea = kbd_r(c, V0);
        c.sb(ea, 0);
        kbd_apply_font_scale().call(c, 0x001c_ce18)?;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, KBD_GET_FONT, 0x001c_ce20)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        kbd_lwc1(c, 12, A0, 0x38);
        kbd_addi(c, A2, SP, 0x10);
        kbd_lwc1(c, 13, A0, 0x3c);
        c.g[A3][0] = 1;
        call_ptr(c, KBD_MEASURE, 0x001c_ce3c)?;
        let ea = kbd_ea(c, S1, 0x413);
        c.ldl(A2, ea);
        let ea = kbd_ea(c, S1, 0x40c);
        c.ldr(A2, ea);
        kbd_lw(c, A3, S1, 0x414);
        let ea = kbd_ea(c, SP, 0x77);
        c.sdl(A2, ea);
        let ea = kbd_ea(c, SP, 0x70);
        c.sdr(A2, ea);
        let ea = kbd_ea(c, SP, 0x78);
        c.sw(ea, c.g[A3][0]);
        frame.restore(c, S2, 0xa0);
        kbd_lwc1(c, 1, S1, 0x3e8);
        kbd_lwc1(c, 0, SP, 0x18);
        frame.restore(c, S1, 0xb0);
        c.add_s(0, 0, 1);
        frame.restore(c, S3, 0x90);
        frame.restore_ra(c, 0x80);
        let ea = kbd_ea(c, SP, 0x70);
        c.swc1(0, ea);
        let ea = kbd_ea(c, SP, 0x77);
        c.ldl(A0, ea);
        let ea = kbd_ea(c, SP, 0x70);
        c.ldr(A0, ea);
        kbd_lw(c, A1, SP, 0x78);
        let ea = kbd_ea(c, S0, 0x4b);
        c.sdl(A0, ea);
        let ea = kbd_ea(c, S0, 0x44);
        c.sdr(A0, ea);
        let ea = kbd_ea(c, S0, 0x4c);
        c.sw(ea, c.g[A1][0]);
        frame.restore(c, S0, 0xc0);
        frame.close(c);
        Ok(())
    });
}

/// A call of `obj->vtbl[9]` (`setVisible(flag)`) the keyboard popup does when
/// `obj` (in `a2`) is not null: `a1` already holds the flag.
fn kbd_set_visible(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 5;
    let obj = kbd_r(c, A2);
    kbd_vcall(c, obj, 0x48, ret)
}

/// `int cKeyboardPopup_TransitionIn(cKeyboardPopup *this, int first)` (0x1cd348):
/// when shown for the first time resets the edit state (shift flag from the
/// caps state, empty window), lays out the keys, shows the key set of the
/// keyboard type (`+0x43c`: ascii, accents or symbols) and starts the
/// matching screen frame, caches the text and cursor origins of the screen
/// objects, loads the initial text (the text, text id or wide string given
/// before) and places the cursor. Returns 1.
pub fn c_keyboard_popup_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        let not_first = c.g[A1][0] == 0; // `first` is 0: nothing to do
        frame.save_ra(c, 0x20);
        'epilogue: {
            if not_first {
                break 'epilogue;
            }
            c.cycles += 5;
            c.g[S1][0] = 1;
            kbd_lw(c, V0, GP, 0x300);
            let ea = kbd_ea(c, S0, 0x41c);
            c.sw(ea, c.g[S1][0]);
            let no_ctl = c.g[V0][0] == 0;
            let ea = kbd_ea(c, S0, 0x68);
            c.sw(ea, c.g[S1][0]);
            if !no_ctl {
                c.cycles += 4;
                kbd_lw(c, V0, V0, 0x88);
                c.g[V0][0] = sx(kbd_r(c, V0) >> 1);
                c.g[V0][0] &= 1;
                let ea = kbd_ea(c, S0, 0x68);
                c.sw(ea, c.g[V0][0]);
            }
            c.cycles += 3;
            kbd_lw(c, V0, S0, 0x418);
            let has_text = c.g[V0][0] != 0;
            let ea = kbd_ea(c, S0, 0x6c);
            c.sw(ea, 0);
            if !has_text {
                c.cycles += 1;
                let ea = kbd_ea(c, S0, 0x74);
                c.sb(ea, 0);
            }
            c.cycles += 11;
            for off in [0x138, 0x13c] {
                let ea = kbd_ea(c, S0, off);
                c.sw(ea, 0);
            }
            c.g[A0][0] = c.g[S0][0];
            let ea = kbd_ea(c, S0, 0x134);
            c.sw(ea, 0);
            c.g[S2][0] = 0;
            c.g[S4][0] = 0;
            c.g[S3][0] = 0;
            Callee {
                addr: 0x001c_d708,
                f: ee::s04::f_001cd708,
            }
            .call(c, 0x001c_d3c0)?;
            // Key set by keyboard type: 0 symbols (s4), 2 accents (s3), else
            // ascii (s2); the matching name picks the screen frame.
            kbd_lw(c, V1, S0, 0x43c);
            let ty = c.g[V1][0];
            c.g[V0][0] = ((ty as i64) < 2) as u64;
            let ascii_default;
            if ty == c.g[S1][0] {
                c.cycles += 1;
                c.g[A1][0] = 0x4a_0000;
                ascii_default = true;
                c.cycles += 1;
                c.g[S2][0] = 1;
            } else {
                c.cycles += 2;
                let ge2 = c.g[V0][0] == 0;
                c.g[V0][0] = 2;
                if ge2 {
                    c.cycles += 2;
                    let accents = ty == 2;
                    c.g[A1][0] = 0x4a_0000;
                    if accents {
                        c.cycles += 3;
                        c.g[A1][0] = 0x46_0000;
                        c.g[S3][0] = 1;
                        kbd_addi(c, A1, A1, 0x6700);
                        ascii_default = false;
                    } else {
                        c.cycles += 2;
                        c.g[S2][0] = 1;
                        ascii_default = true;
                    }
                } else {
                    c.cycles += 2;
                    let symbols = ty == 0;
                    c.g[A1][0] = 0x4a_0000;
                    if symbols {
                        c.cycles += 3;
                        c.g[S4][0] = 1;
                        kbd_addi(c, A1, A1, 0x1cd0);
                        ascii_default = false;
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = 0x4a_0000;
                        c.cycles += 1;
                        c.g[S2][0] = 1;
                        ascii_default = true;
                    }
                }
            }
            if ascii_default {
                c.cycles += 4;
                kbd_addi(c, A1, A1, 0x1cd8);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0041_7828, 0x001c_d430)?;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0041_7828, 0x001c_d414)?;
            }
            // Show the key sets: objects at +0x48 (ascii), +0x50 (accents),
            // +0x4c (symbols).
            kbd_lw(c, A2, S0, 0x48);
            c.cycles += 2;
            c.g[A1][0] = c.g[S2][0];
            if c.g[A2][0] != 0 {
                kbd_set_visible(c, 0x001c_d450)?;
            }
            c.cycles += 3;
            kbd_lw(c, A2, S0, 0x50);
            c.g[A1][0] = c.g[S3][0];
            if c.g[A2][0] != 0 {
                kbd_set_visible(c, 0x001c_d470)?;
            }
            c.cycles += 3;
            kbd_lw(c, A2, S0, 0x4c);
            c.g[A1][0] = c.g[S4][0];
            if c.g[A2][0] != 0 {
                kbd_set_visible(c, 0x001c_d490)?;
            }
            // Play the screen frame for the keyboard type.
            c.cycles += 9;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, KBD_STR_HASH, 0x001c_d498)?;
            kbd_lw(c, A0, S0, KBD_POPUP_SCREEN);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_c778, 0x001c_d4a4)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = 0xffff;
            let no_frame = c.g[A1][0] == c.g[V0][0];
            c.g[A1][0] &= 0xffff;
            if !no_frame {
                c.cycles += 3;
                kbd_lw(c, A0, S0, KBD_POPUP_SCREEN);
                c.g[A2][0] = 1;
                call_ptr(c, 0x0039_c870, 0x001c_d4c0)?;
            }
            // Cursor widget origin -> this+0x40c.
            c.cycles += 9;
            c.g[A0][0] = 0x46_0000;
            kbd_addi(c, A0, A0, 0x6570);
            call_ptr(c, KBD_STR_HASH, 0x001c_d4cc)?;
            kbd_lw(c, A0, S0, KBD_POPUP_SCREEN);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, KBD_FIND_OBJECT, 0x001c_d4d8)?;
            c.g[S1][0] = c.g[V0][0];
            let none = c.g[S1][0] == 0;
            kbd_addi(c, A1, S0, KBD_POPUP_MARGIN);
            if !none {
                c.cycles += 11;
                let obj = kbd_r(c, S1);
                c.g[V0][0] = c.lw(obj.wrapping_add(8));
                let ea = kbd_ea(c, V0, 0xa0);
                c.g[A0][0] = c.lh(ea);
                kbd_lw(c, V1, V0, 0xa4);
                let target = kbd_r(c, V1);
                c.g[A0][0] = sx(obj.wrapping_add(kbd_r(c, A0)));
                call_ptr(c, target, 0x001c_d4f8)?;
                let ea = kbd_ea(c, S1, 0x4b);
                c.ldl(V0, ea);
                let ea = kbd_ea(c, S1, 0x44);
                c.ldr(V0, ea);
                kbd_lw(c, V1, S1, 0x4c);
                let ea = kbd_ea(c, S0, 0x413);
                c.sdl(V0, ea);
                let ea = kbd_ea(c, S0, 0x40c);
                c.sdr(V0, ea);
                let ea = kbd_ea(c, S0, 0x414);
                c.sw(ea, c.g[V1][0]);
            }
            // Text object origin -> this+0x3e8, and its extent.
            c.cycles += 9;
            c.g[A0][0] = 0x46_0000;
            kbd_addi(c, A0, A0, 0x6530);
            call_ptr(c, KBD_STR_HASH, 0x001c_d51c)?;
            kbd_lw(c, A0, S0, KBD_POPUP_SCREEN);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, KBD_FIND_OBJECT, 0x001c_d528)?;
            c.g[S1][0] = c.g[V0][0];
            let none = c.g[S1][0] == 0;
            kbd_addi(c, A1, S0, KBD_POPUP_WIDTH);
            if !none {
                c.cycles += 15;
                let obj = kbd_r(c, S1);
                c.g[V0][0] = c.lw(obj.wrapping_add(8));
                let ea = kbd_ea(c, V0, 0xa0);
                c.g[A0][0] = c.lh(ea);
                kbd_lw(c, V1, V0, 0xa4);
                let target = kbd_r(c, V1);
                c.g[A0][0] = sx(obj.wrapping_add(kbd_r(c, A0)));
                call_ptr(c, target, 0x001c_d548)?;
                let ea = kbd_ea(c, S1, 0x4b);
                c.ldl(A2, ea);
                let ea = kbd_ea(c, S1, 0x44);
                c.ldr(A2, ea);
                kbd_lw(c, V0, S1, 0x4c);
                let ea = kbd_ea(c, S0, 0x3ef);
                c.sdl(A2, ea);
                let ea = kbd_ea(c, S0, 0x3e8);
                c.sdr(A2, ea);
                let ea = kbd_ea(c, S0, 0x3f0);
                c.sw(ea, c.g[V0][0]);
                kbd_lwc1(c, 0, S0, 0x3f8);
                kbd_lwc1(c, 1, S0, 0x3ec);
                c.add_s(0, 0, 1);
                let ea = kbd_ea(c, S0, 0x410);
                c.swc1(0, ea);
            }
            // Offset object: add its position to the text origin.
            c.cycles += 8;
            c.g[A0][0] = 0x46_0000;
            kbd_addi(c, A0, A0, 0x6710);
            call_ptr(c, KBD_STR_HASH, 0x001c_d57c)?;
            kbd_lw(c, A0, S0, KBD_POPUP_SCREEN);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, KBD_FIND_OBJECT, 0x001c_d588)?;
            if c.g[V0][0] != 0 {
                c.cycles += 19;
                let ea = kbd_ea(c, V0, 0x4b);
                c.ldl(A1, ea);
                let ea = kbd_ea(c, V0, 0x44);
                c.ldr(A1, ea);
                kbd_lw(c, A2, V0, 0x4c);
                let ea = kbd_ea(c, SP, 0x17);
                c.sdl(A1, ea);
                let ea = kbd_ea(c, SP, 0x10);
                c.sdr(A1, ea);
                let ea = kbd_ea(c, SP, 0x18);
                c.sw(ea, c.g[A2][0]);
                kbd_lwc1(c, 3, SP, 0x10);
                kbd_lwc1(c, 4, SP, 0x14);
                kbd_lwc1(c, 5, SP, 0x18);
                kbd_lwc1(c, 0, S0, 0x3e8);
                kbd_lwc1(c, 1, S0, 0x3ec);
                kbd_lwc1(c, 2, S0, 0x3f0);
                c.add_s(0, 0, 3);
                c.add_s(1, 1, 4);
                c.add_s(2, 2, 5);
                for (f, off) in [(0, 0x3e8), (1, 0x3ec), (2, 0x3f0)] {
                    let ea = kbd_ea(c, S0, off);
                    c.swc1(f, ea);
                }
            }
            c.g[A0][0] = 0x46_0000;
            // Hide the two scroll arrows.
            for (hash_off, ret_hash, ret_find, ret_call) in [
                (0x6540, 0x001c_d5e4, 0x001c_d5f0, 0x001c_d610),
                (0x6558, 0x001c_d61c, 0x001c_d628, 0x001c_d648),
            ] {
                c.cycles += if hash_off == 0x6540 { 8 } else { 9 };
                if hash_off != 0x6540 {
                    c.g[A0][0] = 0x46_0000;
                }
                kbd_addi(c, A0, A0, hash_off);
                call_ptr(c, KBD_STR_HASH, ret_hash)?;
                kbd_lw(c, A0, S0, KBD_POPUP_SCREEN);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, KBD_FIND_OBJECT, ret_find)?;
                c.g[A2][0] = c.g[V0][0];
                let none = c.g[A2][0] == 0;
                c.g[A1][0] = 0;
                if !none {
                    kbd_set_visible(c, ret_call)?;
                }
            }
            // Initial text.
            c.cycles += 3;
            kbd_lw(c, V0, S0, 0x418);
            let has_text = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if has_text {
                c.cycles += 2;
                kbd_addi(c, A1, S0, KBD_POPUP_TEXT);
                Callee {
                    addr: 0x001c_b418,
                    f: ee::s04::f_001cb418,
                }
                .call(c, 0x001c_d65c)?;
            }
            c.cycles += 3;
            kbd_lw(c, A1, S0, 0x42c);
            if c.g[A1][0] != 0 {
                // A text id: set it and clear it.
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                Callee {
                    addr: 0x001c_d020,
                    f: ee::s04::f_001cd020,
                }
                .call(c, 0x001c_d670)?;
                let ea = kbd_ea(c, S0, 0x42c);
                c.sw(ea, 0);
            } else {
                kbd_lw(c, A1, S0, 0x428);
                c.cycles += 2;
                if c.g[A1][0] != 0 {
                    // A wide string: set it, then free it.
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x001c_cf70,
                        f: ee::s04::f_001ccf70,
                    }
                    .call(c, 0x001c_d688)?;
                    kbd_lw(c, A0, S0, 0x428);
                    if c.g[A0][0] != 0 {
                        c.cycles += 4;
                        call_ptr(c, 0x0031_7e98, 0x001c_d69c)?;
                    }
                    let ea = kbd_ea(c, S0, 0x428);
                    c.sw(ea, 0);
                } else {
                    kbd_lw(c, A1, S0, 0x424);
                    // A string object: set its text and release it.
                    c.cycles += 3;
                    let ea = kbd_r(c, A1).wrapping_sub(8);
                    c.g[V0][0] = c.lw(ea);
                    let empty = c.g[V0][0] == 0;
                    kbd_addi(c, S1, S0, 0x424);
                    if !empty {
                        c.cycles += 6;
                        c.g[A0][0] = c.g[S0][0];
                        Callee {
                            addr: 0x001c_cf00,
                            f: ee::s04::f_001ccf00,
                        }
                        .call(c, 0x001c_d6b8)?;
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x0031_80b8, 0x001c_d6c0)?;
                    } else {
                        c.cycles += 4;
                        c.g[A1][0] = 0x4a_0000;
                        c.g[A0][0] = c.g[S0][0];
                        kbd_addi(c, A1, A1, 0x1408);
                        Callee {
                            addr: 0x001c_cf00,
                            f: ee::s04::f_001ccf00,
                        }
                        .call(c, 0x001c_d6d8)?;
                    }
                }
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x001c_cc30,
                f: ee::s04::f_001ccc30,
            }
            .call(c, 0x001c_d6e0)?;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x70);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `void cKeyboardPopup_RefreshKeys(cKeyboardPopup *this)` (0x1cd708): redraws the
/// labels of the 0x54 keys: each key text gets its character (the upper or the
/// lower table by shift and caps, symbols keys always upper; `@` and space
/// entries take their special label), is greyed when the key is disabled,
/// and the key's frame is shown or hidden for the keyboard type; finally the
/// frame of the current cursor key is highlighted.
pub fn c_keyboard_popup_refresh_keys(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S0, 0xc0);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0xb0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save(c, FP, 0x40);
        frame.save_ra(c, 0x30);
        kbd_lw(c, V0, S2, KBD_POPUP_SCREEN);
        let no_screen = c.g[V0][0] == 0;
        c.g[V1][0] = 0x44_0000;
        'epilogue: {
            if no_screen {
                break 'epilogue;
            }
            c.cycles += 9;
            c.g[V0][0] = 0x44_0000;
            let ea = kbd_ea(c, SP, 0x21);
            c.sb(ea, 0);
            c.g[FP][0] = sx(kbd_r(c, V0).wrapping_add(0x13e8));
            c.g[S4][0] = sx(kbd_r(c, V1).wrapping_add(0x1390));
            c.g[S3][0] = 0;
            c.g[S7][0] = 0x4c_0000;
            c.g[S6][0] = 0x4c_0000;
            c.g[S5][0] = 0x4c_0000;
            loop {
                'key: {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0;
                    key_widget_name().call(c, 0x001c_d77c)?;
                    if c.g[V0][0] == 0 {
                        // No key with this index.
                        c.g[S3][0] = sx(kbd_r(c, S3).wrapping_add(1));
                        break 'key;
                    }
                    // The key's text object.
                    c.cycles += 10;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, KBD_STR_HASH, 0x001c_d78c)?;
                    kbd_lw(c, A0, S2, KBD_POPUP_SCREEN);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, KBD_FIND_OBJECT, 0x001c_d798)?;
                    c.g[S1][0] = c.g[V0][0];
                    let ea = kbd_r(c, S4);
                    c.g[V1][0] = c.lb(ea);
                    c.g[V0][0] = 0x40;
                    let mut special = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x20;
                    if !special {
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            kbd_lw(c, V0, S2, 0x68);
                        } else {
                            special = true;
                        }
                    }
                    if special {
                        // `@` and space: the special key label.
                        c.cycles += 6;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = c.g[S3][0];
                        Callee {
                            addr: 0x001c_d9a0,
                            f: ee::s04::f_001cd9a0,
                        }
                        .call(c, 0x001c_d7c4)?;
                        c.g[A1][0] = c.g[S3][0];
                    } else {
                        // Upper or lower case by caps and shift.
                        c.cycles += 3;
                        kbd_lw(c, V1, S2, 0x6c);
                        let no_shift = c.g[V1][0] == 0;
                        c.g[V0][0] = (0 < c.g[V0][0]) as u64;
                        if no_shift {
                            c.cycles += 1;
                            c.g[S0][0] = (0 < c.g[V0][0]) as u64;
                        } else {
                            c.cycles += 2;
                            c.g[S0][0] = c.g[V0][0] ^ 1;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx(kbd_r(c, S3).wrapping_sub(0xf));
                        c.g[V0][0] = (c.g[V0][0] < 0xc) as u64;
                        if c.g[V0][0] != 0 {
                            c.g[S0][0] = c.g[V1][0] ^ 1;
                        }
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001c_e520,
                            f: ee::s04::f_001ce520,
                        }
                        .call(c, 0x001c_d7fc)?;
                        let lower = c.g[S0][0] == 0;
                        c.g[V0][0] = sx(kbd_r(c, S3).wrapping_add(kbd_r(c, FP)));
                        let sea = kbd_ea(c, SP, 0x20);
                        if lower {
                            c.cycles += 2;
                            let ea = kbd_r(c, V0);
                            c.g[V1][0] = c.lbu(ea);
                            c.sb(sea, c.g[V1][0]);
                        } else {
                            c.cycles += 3;
                            let ea = kbd_r(c, S4);
                            c.g[V0][0] = c.lbu(ea);
                            c.sb(sea, c.g[V0][0]);
                        }
                        // Set the character, grey it when the key is disabled.
                        c.cycles += 2;
                        let text_none = c.g[S1][0] == 0;
                        c.g[A0][0] = c.g[S1][0];
                        if text_none {
                            c.cycles += 1;
                            c.g[A1][0] = c.g[S3][0];
                        } else {
                            c.cycles += 8;
                            kbd_addi(c, A1, SP, 0x20);
                            call_ptr(c, KBD_SET_ASCII, 0x001c_d828)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A1][0] = c.g[S3][0];
                            c.g[A2][0] = 0;
                            Callee {
                                addr: 0x001c_e560,
                                f: ee::s04::f_001ce560,
                            }
                            .call(c, 0x001c_d838)?;
                            let enabled = c.g[V0][0] != 0;
                            kbd_lw(c, V0, S1, 8);
                            if enabled {
                                c.cycles += 5;
                                kbd_addi(c, A1, S6, 0x67c8);
                                let obj = kbd_r(c, S1);
                                kbd_vcall(c, obj, 0x58, 0x001c_d870)?;
                                c.cycles += 1;
                                c.g[A1][0] = c.g[S3][0];
                            } else {
                                c.cycles += 7;
                                kbd_addi(c, A1, S7, 0x67b8);
                                let obj = kbd_r(c, S1);
                                kbd_vcall(c, obj, 0x58, 0x001c_d854)?;
                                c.g[A1][0] = c.g[S3][0];
                            }
                        }
                    }
                    // The key's frame object.
                    c.cycles += 12;
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 1;
                    c.g[A0][0] = c.g[S2][0];
                    key_widget_name().call(c, 0x001c_d884)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, KBD_STR_HASH, 0x001c_d88c)?;
                    kbd_lw(c, A0, S2, KBD_POPUP_SCREEN);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, KBD_FIND_OBJECT, 0x001c_d898)?;
                    c.g[S0][0] = c.g[V0][0];
                    let none = c.g[S0][0] == 0;
                    c.g[V0][0] = 1;
                    if none {
                        c.cycles += 1;
                        c.g[S3][0] = sx(kbd_r(c, S3).wrapping_add(1));
                        break 'key;
                    }
                    c.cycles += 3;
                    kbd_lw(c, V1, S2, 0x43c);
                    if c.g[V1][0] != c.g[V0][0] {
                        // Not the ascii keyboard: colour the frame, then show it.
                        let obj = kbd_r(c, S0);
                        c.g[V0][0] = c.lw(obj.wrapping_add(8));
                        c.cycles += 11;
                        kbd_addi(c, A1, S5, 0x67f8);
                        kbd_vcall(c, obj, 0x58, 0x001c_d8f8)?;
                        let obj = kbd_r(c, S0);
                        c.g[V0][0] = c.lw(obj.wrapping_add(8));
                        c.g[A1][0] = 1;
                        kbd_vcall(c, obj, 0x48, 0x001c_d910)?;
                        c.cycles += 1;
                        c.g[S3][0] = sx(kbd_r(c, S3).wrapping_add(1));
                    } else {
                        // Ascii keyboard: only the first 14 keys' frames are hidden.
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[S3][0] < 0xe) as u64;
                        let show = c.g[V0][0] == 0;
                        c.g[A1][0] = 0;
                        c.cycles += 2;
                        if show {
                            c.g[A1][0] = 1;
                        }
                        let obj = kbd_r(c, S0);
                        c.cycles += 6;
                        kbd_vcall(c, obj, 0x48, 0x001c_d8dc)?;
                        c.g[S3][0] = sx(kbd_r(c, S3).wrapping_add(1));
                    }
                }
                // Next key.
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S3][0] as i64) < 84) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S4][0] = sx(kbd_r(c, S4).wrapping_add(1));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x001c_d768);
                    }
                    continue;
                }
                break;
            }
            // Highlight the cursor key.
            c.cycles += 13;
            kbd_lw(c, A1, S2, 0x140);
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = 1;
            c.g[A0][0] = c.g[S2][0];
            key_widget_name().call(c, 0x001c_d934)?;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, KBD_STR_HASH, 0x001c_d93c)?;
            kbd_lw(c, A0, S2, KBD_POPUP_SCREEN);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, KBD_FIND_OBJECT, 0x001c_d948)?;
            c.g[A2][0] = c.g[V0][0];
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 0x4c_0000;
            if !none {
                c.cycles += 6;
                let obj = kbd_r(c, A2);
                c.g[V1][0] = c.lw(obj.wrapping_add(8));
                kbd_addi(c, A1, A1, 0x67e8);
                let ea = kbd_ea(c, V1, 0x58);
                c.g[A0][0] = c.lh(ea);
                let ea = kbd_ea(c, V1, 0x5c);
                c.g[V0][0] = c.lw(ea);
                let target = kbd_r(c, V0);
                c.g[A0][0] = sx(obj.wrapping_add(kbd_r(c, A0)));
                call_ptr(c, target, 0x001c_d96c)?;
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        frame.restore(c, S2, 0xa0);
        frame.restore(c, S3, 0x90);
        frame.restore(c, S4, 0x80);
        frame.restore(c, S5, 0x70);
        frame.restore(c, S6, 0x60);
        frame.restore(c, S7, 0x50);
        frame.restore(c, FP, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });
}

fn key_widget_name() -> Callee {
    Callee {
        addr: 0x001c_d0d8,
        f: ee::s04::f_001cd0d8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, FAKE};
    use crate::difftest::{elf_image, fill, Rng, DATA};
    use crate::game::hashvalue::str_hash;
    use std::sync::atomic::AtomicUsize;

    /// The fakes' gate word and its magic (both machines of a diff get their own).
    pub(crate) const OC934_CTRL: u32 = DATA + 0x2f_1000;
    pub(crate) const OC934_MAGIC: u32 = 0x4f43_3934;
    /// Per-run salt of the fakes' dice and the count of fake calls so far.
    pub(crate) const OC934_SALT: u32 = OC934_CTRL + 4;
    pub(crate) const OC934_CALLS: u32 = OC934_CTRL + 8;
    /// The widget every object lookup returns, and its vtable.
    pub(crate) const OC934_OBJ: u32 = DATA + 0x2f_2000;
    pub(crate) const OC934_VT: u32 = DATA + 0x2f_3000;

    /// A fake callee: the logged fake call, then the result the function under
    /// test branches on. Lookups return null, keys report absent or disabled
    /// about a quarter of the time (a die per call, salted per run).
    fn oc934_fake(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let (a1, a2) = (c.g[A1][0] as u32, c.g[A2][0] as u32);
        let n = c.bus.read32(OC934_CALLS);
        c.bus.write32(OC934_CALLS, n + 1);
        let salt = c.bus.read32(OC934_SALT) as u64;
        let roll = fakes::mix(salt << 20 ^ n as u64) & 3;
        fakes::fake(c, addr, ra);
        if c.pc != ra {
            return;
        }
        match addr {
            // cUIScreen_getObjectByHashName
            0x0039_d860 => c.g[V0][0] = if roll == 0 { 0 } else { OC934_OBJ as u64 },
            // cKeyboardPopup_GetKeyWidgetName: a name, or no such key.
            0x001c_d0d8 => {
                c.bus.write8(a2, b'k');
                c.bus.write8(a2 + 1, 0);
                c.g[V0][0] = (roll != 0) as u64;
            }
            // cKeyboardPopup_IsKeyEnabled
            0x001c_e560 => c.g[V0][0] = (roll != 0) as u64,
            // cUIScreen_getFrameByLabel: 0xffff means no such frame.
            0x0039_c778 => c.g[V0][0] = if roll == 0 { 0xffff } else { 3 },
            // Font::Measure: ten units per character, into rect.w (+8).
            0x0039_1fb0 => {
                let mut len = 0u32;
                while len < 255 && c.bus.read8(a1 + len) != 0 {
                    len += 1;
                }
                c.bus.write32(a2 + 8, (10.0f32 * len as f32).to_bits());
            }
            _ => {}
        }
    }

    macro_rules! oc934_fakes {
        ($install:ident; $(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC934_CTRL) != OC934_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc934_fake(c, $addr);
                }
            )*
            fn $install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc934_fakes!(
        oc934_install;
        (oc934_fake_317e98, OC934_PREV_317E98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated, 0x0031_7e98),
        (oc934_fake_3180b8, OC934_PREV_3180B8, ee::s10::OVERRIDE_003180B8, ee::s10::f_003180b8__generated, 0x0031_80b8),
        (oc934_fake_198af0, OC934_PREV_198AF0, ee::s03::OVERRIDE_00198AF0, ee::s03::f_00198af0__generated, 0x0019_8af0),
        (oc934_fake_417828, OC934_PREV_417828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated, 0x0041_7828),
        (oc934_fake_39d860, OC934_PREV_39D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated, 0x0039_d860),
        (oc934_fake_39c778, OC934_PREV_39C778, ee::s13::OVERRIDE_0039C778, ee::s13::f_0039c778__generated, 0x0039_c778),
        (oc934_fake_39c870, OC934_PREV_39C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated, 0x0039_c870),
        (oc934_fake_3a0c50, OC934_PREV_3A0C50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated, 0x003a_0c50),
        (oc934_fake_3a0e90, OC934_PREV_3A0E90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated, 0x003a_0e90),
        (oc934_fake_3a1000, OC934_PREV_3A1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated, 0x003a_1000),
        (oc934_fake_3a04f0, OC934_PREV_3A04F0, ee::s13::OVERRIDE_003A04F0, ee::s13::f_003a04f0__generated, 0x003a_04f0),
        (oc934_fake_391fb0, OC934_PREV_391FB0, ee::s12::OVERRIDE_00391FB0, ee::s12::f_00391fb0__generated, 0x0039_1fb0),
        (oc934_fake_1cd0d8, OC934_PREV_1CD0D8, ee::s04::OVERRIDE_001CD0D8, ee::s04::f_001cd0d8__generated, 0x001c_d0d8),
        (oc934_fake_1cd9a0, OC934_PREV_1CD9A0, ee::s04::OVERRIDE_001CD9A0, ee::s04::f_001cd9a0__generated, 0x001c_d9a0),
        (oc934_fake_1ce520, OC934_PREV_1CE520, ee::s04::OVERRIDE_001CE520, ee::s04::f_001ce520__generated, 0x001c_e520),
        (oc934_fake_1ce560, OC934_PREV_1CE560, ee::s04::OVERRIDE_001CE560, ee::s04::f_001ce560__generated, 0x001c_e560),
        (oc934_fake_1ce4c8, OC934_PREV_1CE4C8, ee::s04::OVERRIDE_001CE4C8, ee::s04::f_001ce4c8__generated, 0x001c_e4c8),
        (oc934_fake_1cd020, OC934_PREV_1CD020, ee::s04::OVERRIDE_001CD020, ee::s04::f_001cd020__generated, 0x001c_d020),
        (oc934_fake_1ccf70, OC934_PREV_1CCF70, ee::s04::OVERRIDE_001CCF70, ee::s04::f_001ccf70__generated, 0x001c_cf70),
        (oc934_fake_1ccf00, OC934_PREV_1CCF00, ee::s04::OVERRIDE_001CCF00, ee::s04::f_001ccf00__generated, 0x001c_cf00),
        (oc934_fake_1cb418, OC934_PREV_1CB418, ee::s04::OVERRIDE_001CB418, ee::s04::f_001cb418__generated, 0x001c_b418),
        (oc934_fake_1cce98, OC934_PREV_1CCE98, ee::s04::OVERRIDE_001CCE98, ee::s04::f_001cce98__generated, 0x001c_ce98),
        (oc934_fake_1ccc30, OC934_PREV_1CCC30, ee::s04::OVERRIDE_001CCC30, ee::s04::f_001ccc30__generated, 0x001c_cc30),
        (oc934_fake_1cd708, OC934_PREV_1CD708, ee::s04::OVERRIDE_001CD708, ee::s04::f_001cd708__generated, 0x001c_d708),
    );

    /// A scrambled machine with the game image, the fakes' gate open and the
    /// shared widget (random contents, vtable `OC934_VT` whose slots +0x48,
    /// +0x58, +0xa0 and +0xa8 are fake methods).
    pub(crate) fn oc934_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc934_install();
        let mut r = fakes::machine(c, seed, salt, calls);
        elf_image(c);
        c.bus.write32(OC934_CTRL, OC934_MAGIC);
        c.bus.write32(OC934_SALT, r.next_u32());
        c.bus.write32(OC934_CALLS, 0);
        fill(c, &mut r, OC934_OBJ, 0x40);
        c.bus.write32(OC934_OBJ + 8, OC934_VT);
        for (i, off) in [0x48u32, 0x58, 0xa0, 0xa8].into_iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(OC934_VT + off, delta as u16);
            c.bus.write32(OC934_VT + off + 4, FAKE + 0x10 * i as u32);
        }
        r
    }

    fn oc934_put_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    fn oc934_text(r: &mut Rng, len: u32) -> Vec<u8> {
        (0..len).map(|_| b'a' + (r.next_u32() % 26) as u8).collect()
    }

    /// Buy popup: the widget's hash picks one of the eight handlers (or none),
    /// the string types, the credit count and the callee results vary.
    #[test]
    fn c_ui_state_buy_popup_on_widget_created_matches() {
        const NAMES: [u32; 8] = [
            0x0046_62d8,
            0x0046_62e8,
            0x0046_62f8,
            0x0046_05f0,
            0x0046_0610,
            0x0046_6308,
            0x0046_6318,
            0x0046_6328,
        ];
        covered(
            ee::s04::f_001cac98__generated,
            c_ui_state_buy_popup_on_widget_created,
            162,
            &|c, seed| {
                let mut r = oc934_machine(c, seed, 0x001c_ac98, 3);
                let (this, widget) = (DATA + 0x1000, DATA + 0x2000);
                let which = (seed % 9) as usize;
                let hash = if which < 8 {
                    let a = NAMES[which] as usize;
                    str_hash(&c.bus.ram[a..a + 32])
                } else {
                    r.next_u32() | 1
                };
                c.bus.write32(widget + 0x38, hash);
                c.bus.write32(widget + 8, OC934_VT);
                fill(c, &mut r, this + 0x40, 0x20);
                let types = [1u32, 2, 3, 0, 5, u32::MAX];
                let sel = ((seed / 9) % 6) as usize;
                for (k, off) in [0x5cu32, 0x54, 0x64].into_iter().enumerate() {
                    c.bus.write32(this + off, types[(sel + k) % 6]);
                }
                c.bus
                    .write32(this + 0x50, [0xffff_fffd, 0, 2][((seed / 54) % 3) as usize]);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
            },
        );
    }

    /// Scroll window: window text, margins and direction vary; the measured
    /// width is ten units per character (a fake), so the fit loops end.
    #[test]
    fn c_keyboard_popup_update_scroll_window_matches() {
        covered(
            ee::s04::f_001cc8d8__generated,
            c_keyboard_popup_update_scroll_window,
            160,
            &|c, seed| {
                let mut r = oc934_machine(c, seed, 0x001c_c8d8, 24);
                let this = DATA + 0x1000;
                let len = [0u32, 3, 8, 14, 20][((seed / 2) % 5) as usize];
                let text = oc934_text(&mut r, len);
                oc934_put_str(c, this + 0x74, &text);
                for i in 0..0x40 {
                    c.bus.write8(this + 0xb4 + i, 0);
                }
                let start = if len == 0 {
                    0
                } else {
                    r.next_u32() % (len / 2 + 1)
                };
                let end = start + r.next_u32() % (len - start + 1);
                c.bus.write32(this + 0x138, start);
                c.bus.write32(this + 0x13c, end);
                let cursor = if (seed / 10) % 2 == 0 {
                    len
                } else {
                    r.next_u32() % (len + 1)
                };
                c.bus.write32(this + 0x134, cursor);
                c.bus.write32(this + 0x60, len + (seed / 3) % 3);
                c.bus.write32(this + 0x40, DATA + 0x5000);
                let width = [40.0f32, 90.0, 160.0, 400.0][((seed / 5) % 4) as usize];
                let margin = [0.0f32, 6.0, 12.0][((seed / 7) % 3) as usize];
                c.bus.write32(this + 0x3f4, width.to_bits());
                c.bus.write32(this + 0x400, margin.to_bits());
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    /// Cursor: text object found or not, character under the cursor (none,
    /// letter, space), masked or plain text.
    #[test]
    fn c_keyboard_popup_update_cursor_matches() {
        covered(
            ee::s04::f_001ccc30__generated,
            c_keyboard_popup_update_cursor,
            120,
            &|c, seed| {
                let mut r = oc934_machine(c, seed, 0x001c_cc30, 14);
                let this = DATA + 0x1000;
                fill(c, &mut r, this + 0x3e8, 0x20);
                let cursor = r.next_u32() % 18;
                let start = r.next_u32() % (cursor + 1);
                let mut text = oc934_text(&mut r, 20);
                text[cursor as usize] = [0u8, b'a', b' ', b'@', b'Z'][(seed % 5) as usize];
                // The text ends at the cursor character when it is NUL.
                oc934_put_str(c, this + 0x74, &text);
                c.bus.write8(this + 0x74 + cursor, text[cursor as usize]);
                let plain = oc934_text(&mut r, 20);
                oc934_put_str(c, this + 0xb4, &plain);
                let masked = oc934_text(&mut r, 20);
                oc934_put_str(c, this + 0xf4, &masked);
                c.bus.write32(this + 0x70, (seed / 5) % 2);
                c.bus.write32(this + 0x134, cursor);
                c.bus.write32(this + 0x138, start);
                c.bus.write32(this + 0x40, DATA + 0x5000);
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// Transition in: first or not, caps state, keyboard type, which key-set
    /// objects exist, and which initial text (text, text id, wide string,
    /// string object or the empty default) is given.
    #[test]
    fn c_keyboard_popup_transition_in_matches() {
        covered(
            ee::s04::f_001cd348__generated,
            c_keyboard_popup_transition_in,
            192,
            &|c, seed| {
                let mut r = oc934_machine(c, seed, 0x001c_d348, 16);
                let this = DATA + 0x1000;
                fill(c, &mut r, this, 0x110);
                let ty = [1u32, 0, 2, 3, u32::MAX, 9][(seed % 6) as usize];
                let bits = seed / 6;
                c.bus.write32(this + 0x43c, ty);
                c.bus.write32(this + 0x40, DATA + 0x5000);
                for (k, off) in [0x48u32, 0x4c, 0x50].into_iter().enumerate() {
                    let have = (r.next_u32() >> k) & 1;
                    c.bus
                        .write32(this + off, if have != 0 { OC934_OBJ } else { 0 });
                }
                c.bus
                    .write32(GAME_GP_300, if bits & 1 != 0 { DATA + 0x7000 } else { 0 });
                c.bus.write32(DATA + 0x7000 + 0x88, r.next_u32());
                c.bus.write32(this + 0x418, (bits >> 1) & 1);
                c.bus.write32(
                    this + 0x42c,
                    if bits & 4 != 0 { r.next_u32() | 1 } else { 0 },
                );
                c.bus
                    .write32(this + 0x428, if bits & 8 != 0 { DATA + 0x6100 } else { 0 });
                c.bus.write32(this + 0x424, DATA + 0x6008);
                c.bus.write32(DATA + 0x6000, (bits >> 4) & 1);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 24 != 7) as u64;
            },
        );
    }

    /// `gp + 0x300`: the pointer to the game-wide control block whose flag
    /// word (+0x88) gives the caps state.
    const GAME_GP_300: u32 = crate::difftest::GAME_GP + 0x300;

    /// Refresh keys: 84 keys, each present or absent, plain or special, enabled
    /// or not, with and without its text and frame objects.
    #[test]
    fn c_keyboard_popup_refresh_keys_matches() {
        covered(
            ee::s04::f_001cd708__generated,
            c_keyboard_popup_refresh_keys,
            24,
            &|c, seed| {
                let mut r = oc934_machine(c, seed, 0x001c_d708, 300);
                let this = DATA + 0x1000;
                c.bus
                    .write32(this + 0x40, if seed % 12 == 0 { 0 } else { DATA + 0x5000 });
                c.bus.write32(this + 0x68, r.next_u32() & 1);
                c.bus.write32(this + 0x6c, r.next_u32() & 1);
                c.bus.write32(this + 0x140, r.next_u32() % 84);
                c.bus.write32(
                    this + 0x43c,
                    if seed % 3 == 0 { 1 } else { r.next_u32() % 3 },
                );
                c.g[A0][0] = this as u64;
            },
        );
    }
}
