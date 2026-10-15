//! OpenCode worker module (docs/investigation/phase2-worker-guide.md).
//!
//! Trivial leaf replacements. Each copies the generated body's logic, cycle
//! charging and observable register (and FPU condition) results exactly, with
//! a differential test as its gate.

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001c_7be0, c_fe_popup_confirm_get_centered_x, "cFEPopupConfirm_GetCenteredX"),
    (0x001c_88c8, c_fe_popup_confirm_position_buttons, "cFEPopupConfirm_PositionButtons"),
    (0x001c_9b68, c_fe_popup_confirm_accumulate_size, "cFEPopupConfirm_AccumulateSize"),
    (0x001c_a150, c_fe_popup_confirm_get_selection, "cFEPopupConfirm_GetSelection"),
    (0x001c_d088, c_keyboard_popup_set_max_length, "cKeyboardPopup_SetMaxLength"),
    (0x001c_d0b0, c_keyboard_popup_set_min_length, "cKeyboardPopup_SetMinLength"),
    (0x001c_e3c8, c_keyboard_popup_set_key_enabled, "cKeyboardPopup_SetKeyEnabled"),
    (0x001c_e560, c_keyboard_popup_is_key_enabled, "cKeyboardPopup_IsKeyEnabled"),
    (
        0x001d_a510,
        c_fe_state_net_help_popup_set_button_help,
        "cFEStateNetHelpPopup_SetButtonHelp",
    ),
    (
        0x001d_a530,
        c_fe_state_net_help_popup_set_body_text,
        "cFEStateNetHelpPopup_SetBodyText",
    ),
    (0x001e_2d58, message_center_is_message_of_type, "MessageCenter_IsMessageOfType"),
    (0x001f_30c0, load_indicator_ctor, "LoadIndicator_ctor"),
    (0x001f_3170, load_indicator_clear, "LoadIndicator_Clear"),
    (0x001f_36c0, load_indicator_set_busy, "LoadIndicator_SetBusy"),
    (0x001f_36d8, load_indicator_clear_busy, "LoadIndicator_ClearBusy"),
    (0x0020_2738, event_select_on_widget_notify, "EventSelect_OnWidgetNotify"),
    (0x0020_a8b0, game_ui_get_results_screen_id, "GameUI_GetResultsScreenId"),
    (0x0021_05b0, race_progress_reset_rider, "RaceProgress_ResetRider"),
    (0x0021_0608, race_progress_clear_path_data, "RaceProgress_ClearPathData"),
    (0x0022_9738, crowd_man_2d_add_sprite, "CrowdMan2d_AddSprite"),
];

// ---------------------------------------------------------------------------
// Front-end popup confirm.

/// `float cFEPopupConfirm_GetCenteredX(cFEPopupConfirm *this)` (0x1c7be0):
/// if the override at +0x2cc is set, the centred X is 0; otherwise it is
/// `((640.0 - this->w) * 0.5) - this->x` (the fields at +0x260 and +0x1b8).
pub fn c_fe_popup_confirm_get_centered_x(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 3;
    c.g[V0][0] = this.off(0x2cc).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        // mtc1 $zero -> f0
        c.f[0] = 0;
        abi::ret(c);
    } else {
        c.cycles += 10;
        let w = this.off(0x260).read_fpu(c);
        abi::set_fpr(c, 2, w);
        // mtc1 $at (=0x44200000 = 640.0) -> f0
        let mut f0 = FpuF32(0x4420_0000);
        // mtc1 $at (=0x3f000000 = 0.5) -> f3
        let half = FpuF32(0x3f00_0000);
        c.g[AT][0] = 0x3f00_0000;
        f0 = f0 - w;
        let x = this.off(0x1b8).read_fpu(c);
        abi::set_fpr(c, 1, x);
        f0 = f0 * half;
        abi::set_fpr(c, 3, half);
        f0 = f0 - x;
        abi::set_fpr(c, 0, f0);
        abi::ret(c);
    }
}

/// `void cFEPopupConfirm_PositionButtons(cFEPopupConfirm *this)` (0x1c88c8):
/// when `this->+0x70` holds a button container, adds the two offsets at +0x290
/// and +0x294 to the pair at the loaded doubleword near +0x1ac and copies the
/// result (plus the word at +0x1b4) into the container at +0x44.
pub fn c_fe_popup_confirm_position_buttons(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 3;
    let child = this.off(0x70).lw(c);
    c.g[V0][0] = child;
    c.g[SP][0] = sx(abi::r32(c, SP).wrapping_sub(0x10));
    if child != 0 {
        c.cycles += 20;
        let sp = Addr(abi::r32(c, SP));
        let dword = this.off(0x1ac).read_u64_unaligned(c);
        c.g[V1][0] = dword;
        let extra = this.off(0x1b4).lw(c);
        c.g[A1][0] = extra;
        sp.write_u64_unaligned(c, dword);
        sp.off(8).sw(c, extra);
        let f3v = this.off(0x290).read_fpu(c);
        abi::set_fpr(c, 3, f3v);
        let f2v = this.off(0x294).read_fpu(c);
        abi::set_fpr(c, 2, f2v);
        let f1r = sp.off(4).read_fpu(c);
        let f0r = sp.read_fpu(c);
        let f1 = f1r + f2v;
        abi::set_fpr(c, 1, f1);
        let f0 = f0r + f3v;
        abi::set_fpr(c, 0, f0);
        sp.off(4).write_fpu(c, f1);
        sp.write_fpu(c, f0);
        let out = sp.read_u64_unaligned(c);
        c.g[V1][0] = out;
        let extra2 = sp.off(8).lw(c);
        c.g[A0][0] = extra2;
        Addr(child as u32).off(0x44).write_u64_unaligned(c, out);
        Addr(child as u32).off(0x4c).sw(c, extra2);
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(abi::r32(c, SP).wrapping_add(0x10));
    c.pc = t;
}

/// `void cFEPopupConfirm_AccumulateSize(void *unused, float *dst, const float *src)`
/// (0x1c9b68): `if (dst[0] < src[2]) dst[0] = src[2]; dst[1] += src[3];`.
pub fn c_fe_popup_confirm_accumulate_size(c: &mut Cpu) {
    let dst = Addr(abi::arg(c, 1));
    let src = Addr(abi::arg(c, 2));
    c.cycles += 6;
    let f1 = src.off(8).read_fpu(c);
    abi::set_fpr(c, 1, f1);
    let f0 = dst.read_fpu(c);
    abi::set_fpr(c, 0, f0);
    if abi::c_lt_s(c, f0, f1) {
        dst.write_fpu(c, f1);
    }
    c.cycles += 5;
    let f1b = src.off(0xc).read_fpu(c);
    abi::set_fpr(c, 1, f1b);
    let f0b = dst.off(4).read_fpu(c) + f1b;
    abi::set_fpr(c, 0, f0b);
    dst.off(4).write_fpu(c, f0b);
    abi::ret(c);
}

/// `int cFEPopupConfirm_GetSelection(cFEPopupConfirm *this)` (0x1ca150): if
/// the current selection at +0x6c is set, returns its byte at +0x95 (or one
/// less when the +0x30c sentinel is -1); otherwise the id at +0x168.
pub fn c_fe_popup_confirm_get_selection(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 3;
    let sel = this.off(0x6c).lw(c);
    c.g[A1][0] = sel;
    if sel != 0 {
        let sentinel = this.off(0x30c).lw(c);
        c.g[V1][0] = sentinel;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        let is_minus_one = sentinel == c.g[V0][0];
        c.cycles += 3;
        let byte = Addr(sel as u32).off(0x95).lbu(c);
        c.g[A1][0] = byte;
        c.cycles += 2;
        if is_minus_one {
            c.g[V0][0] = byte;
        } else {
            c.g[V0][0] = sx((byte as u32).wrapping_sub(1));
        }
    } else {
        c.cycles += 2;
        c.g[V0][0] = this.off(0x168).lw(c);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Keyboard popup.

/// `void cKeyboardPopup_SetMaxLength(cKeyboardPopup *this, int max)` (0x1cd088):
/// stores `max` (if 0 <= max < 64), else 0x3f for a non-negative value, else 0.
pub fn c_keyboard_popup_set_max_length(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    let max = c.g[A1][0];
    c.cycles += 2;
    let non_negative = (max as i64) >= 0;
    c.g[V0][0] = ((max as i64) < 64) as u64;
    if non_negative {
        c.cycles += 2;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x3f;
        c.cycles += 2;
        if in_range {
            this.off(0x60).sw(c, max);
        } else {
            this.off(0x60).sw(c, c.g[V0][0]);
        }
    } else {
        c.cycles += 2;
        this.off(0x60).sw(c, 0);
    }
    abi::ret(c);
}

/// `void cKeyboardPopup_SetMinLength(cKeyboardPopup *this, int min)` (0x1cd0b0):
/// same as [`c_keyboard_popup_set_max_length`] but the field is at +0x64.
pub fn c_keyboard_popup_set_min_length(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    let min = c.g[A1][0];
    c.cycles += 2;
    let non_negative = (min as i64) >= 0;
    c.g[V0][0] = ((min as i64) < 64) as u64;
    if non_negative {
        c.cycles += 2;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x3f;
        c.cycles += 2;
        if in_range {
            this.off(0x64).sw(c, min);
        } else {
            this.off(0x64).sw(c, c.g[V0][0]);
        }
    } else {
        c.cycles += 2;
        this.off(0x64).sw(c, 0);
    }
    abi::ret(c);
}

/// `void cKeyboardPopup_SetKeyEnabled(cKeyboardPopup *this, int index, int value, int key)`
/// (0x1ce3c8): stores `value` into the per-key table at +0x148 (key 0 or 2) or
/// +0x298 (key 1 or 2), leaving `index*4 + this` in v0.
pub fn c_keyboard_popup_set_key_enabled(c: &mut Cpu) {
    let this = c.g[A0][0];
    let index = c.g[A1][0];
    let value = c.g[A2][0];
    let key = c.g[A3][0];
    c.cycles += 2;
    let key_is_zero = key == 0;
    c.g[V0][0] = 2;
    let mut store_148 = false;
    if key_is_zero {
        store_148 = true;
    } else {
        c.cycles += 2;
        let key_not_two = key != c.g[V0][0];
        c.g[V0][0] = sx((key as u32).wrapping_sub(1));
        if !key_not_two {
            store_148 = true;
        }
    }
    if store_148 {
        c.cycles += 4;
        c.g[V0][0] = sx((index as u32) << 2);
        c.g[V0][0] = sx((this as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[V0][0] as u32).off(0x148).sw(c, value);
        c.g[V0][0] = sx((key as u32).wrapping_sub(1));
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    let skip = c.g[V0][0] == 0;
    c.g[V0][0] = sx((index as u32) << 2);
    if !skip {
        c.cycles += 2;
        c.g[V0][0] = sx((this as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[V0][0] as u32).off(0x298).sw(c, value);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cKeyboardPopup_IsKeyEnabled(cKeyboardPopup *this, int index, int *out)`
/// (0x1ce560): picks the enabled flag for `index` from the +0x148/+0x298
/// tables (or returns the table word when `out` is null), combining it with
/// the popup's own enabled state.
pub fn c_keyboard_popup_is_key_enabled(c: &mut Cpu) {
    let self_ptr = c.g[A0][0];
    let index = c.g[A1][0];
    let out = c.g[A2][0];
    c.cycles += 5;
    c.g[A3][0] = self_ptr;
    let obj = Addr(self_ptr as u32);
    c.g[V1][0] = obj.off(0x68).lw(c);
    c.g[V0][0] = obj.off(0x6c).lw(c);
    c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V1][0] ^= 1;
    } else {
        c.cycles += 1;
        c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
    }
    c.cycles += 4;
    c.g[V0][0] = sx((index as u32).wrapping_sub(0xf));
    c.g[V0][0] = (c.g[V0][0] < 0xc) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = obj.off(0x6c).lw(c);
        c.g[V1][0] = c.g[V0][0] ^ 1;
    }
    c.cycles += 2;
    c.g[A0][0] = sx((index as u32) << 2);
    if c.g[V1][0] == 0 {
        c.cycles += 2;
        if out != 0 {
            c.cycles += 3;
            c.g[V0][0] = sx((self_ptr as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x148).lw(c);
            Addr(out as u32).sw(c, c.g[V1][0]);
        }
        c.cycles += 3;
        c.g[V1][0] = sx((self_ptr as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x298).lw(c);
        abi::ret(c);
    } else {
        c.cycles += 2;
        if out != 0 {
            c.cycles += 3;
            c.g[V0][0] = sx((self_ptr as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x298).lw(c);
            Addr(out as u32).sw(c, c.g[V1][0]);
        }
        c.cycles += 3;
        c.g[V1][0] = sx((self_ptr as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x148).lw(c);
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// Net-help popup.

/// `void cFEStateNetHelpPopup_SetButtonHelp(cFEStateNetHelpPopup *this, int index, uint help, uint help2)`
/// (0x1da510): stores `help` and `help2` into the button record at
/// `this + index*4` (+0x68/+0x84).
pub fn c_fe_state_net_help_popup_set_button_help(c: &mut Cpu) {
    let this = c.g[A0][0];
    let index = c.g[A1][0];
    let help = c.g[A2][0];
    let help2 = c.g[A3][0];
    c.cycles += 6;
    c.g[A1][0] = sx((index as u32) << 2);
    c.g[V0][0] = sx((this as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = c.g[V0][0];
    Addr(c.g[V0][0] as u32).off(0x68).sw(c, help);
    Addr(c.g[A0][0] as u32).off(0x84).sw(c, help2);
    abi::ret(c);
}

/// `void cFEStateNetHelpPopup_SetBodyText(cFEStateNetHelpPopup *this, int index, uint text)`
/// (0x1da530): stores `text` at `this + index*4 + 0x58`.
pub fn c_fe_state_net_help_popup_set_body_text(c: &mut Cpu) {
    let this = c.g[A0][0];
    let index = c.g[A1][0];
    let text = c.g[A2][0];
    c.cycles += 4;
    c.g[A1][0] = sx((index as u32) << 2);
    c.g[A0][0] = sx((this as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[A0][0] as u32).off(0x58).sw(c, text);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Message center.

/// `bool MessageCenter_IsMessageOfType(uint type, int index)` (0x1e2d58):
/// follows the two lookup tables at 0x4c6c08 (24-byte records, word +4) and
/// the 20-byte records at 0x441630 (word +0xc), and reports whether the
/// resulting type equals `type`.
pub fn message_center_is_message_of_type(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = 0x18;
    c.g[A2][0] = 0x4c0000;
    let first = abi::mult(c, abi::r32(c, A1), 0x18);
    c.g[A1][0] = sx(first);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x6c08));
    c.g[A3][0] = 0x14;
    c.g[V0][0] = sx(0x0044_0000u32.wrapping_add(0x1630));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(4).lw(c);
    let second = abi::mult(c, abi::r32(c, V1), abi::r32(c, A3));
    c.g[V1][0] = sx(second);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
    c.g[V0][0] ^= c.g[A0][0];
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Loading indicator.

/// `LoadIndicator *LoadIndicator_ctor(LoadIndicator *this)` (0x1f30c0):
/// zeroes the four words of the indicator and returns `this`.
pub fn load_indicator_ctor(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 6;
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(4).sw(c, 0);
    this.off(8).sw(c, 0);
    abi::ret(c);
}

/// `void LoadIndicator_Clear(LoadIndicator *this)` (0x1f3170): zeroes the
/// four words of the indicator.
pub fn load_indicator_clear(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 5;
    this.off(8).sw(c, 0);
    this.sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(4).sw(c, 0);
    abi::ret(c);
}

/// `void LoadIndicator_SetBusy(LoadIndicator *this, int bit)` (0x1f36c0):
/// sets bit `bit & 31` in the busy mask at +0x4.
pub fn load_indicator_set_busy(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 6;
    c.g[V1][0] = this.off(4).lw(c);
    c.g[V0][0] = 1;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A1][0] as u32) & 31));
    c.g[V1][0] |= c.g[V0][0];
    this.off(4).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void LoadIndicator_ClearBusy(LoadIndicator *this, int bit)` (0x1f36d8):
/// clears bit `bit & 31` from the busy mask at +0x4, zeroing the +0x8 word if
/// the mask becomes empty.
pub fn load_indicator_clear_busy(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    c.cycles += 7;
    c.g[V0][0] = 1;
    c.g[V1][0] = this.off(4).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << ((c.g[A1][0] as u32) & 31));
    c.g[V0][0] = !(0u64 | c.g[V0][0]);
    c.g[V1][0] &= c.g[V0][0];
    this.off(4).sw(c, c.g[V1][0]);
    if c.g[V1][0] == 0 {
        c.cycles += 1;
        this.off(8).sw(c, 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Menus / race progress / crowd.

/// `int EventSelect_OnWidgetNotify(EventSelect *this, int unused, int notify)`
/// (0x202738): maps widget notifications 6/9/8 to 0, anything else to 0x101.
pub fn event_select_on_widget_notify(c: &mut Cpu) {
    let notify = c.g[A2][0];
    c.cycles += 3;
    c.g[V0][0] = 6;
    let is_six = notify == c.g[V0][0];
    c.g[V0][0] = 9;
    if !is_six {
        c.cycles += 2;
        let is_nine = notify == c.g[V0][0];
        c.g[V0][0] = 8;
        if !is_nine {
            c.cycles += 2;
            if notify != c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0x101;
                abi::ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `int GameUI_GetResultsScreenId(uint state, int mode)` (0x20a8b0): maps a
/// race state to the results screen id (0x16, 0xb or 0xa).
pub fn game_ui_get_results_screen_id(c: &mut Cpu) {
    let state = c.g[A0][0];
    let mode = c.g[A1][0];
    c.cycles += 4;
    c.g[V0][0] = sx((state as u32).wrapping_sub(6));
    c.g[V0][0] = (c.g[V0][0] < 6) as u64;
    let not_six_to_eleven = c.g[V0][0] == 0;
    c.g[V0][0] = sx((state as u32).wrapping_sub(4));
    if not_six_to_eleven {
        c.cycles += 3;
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            let is_zero = state == 0;
            c.g[V0][0] = 3;
            if !is_zero {
                c.cycles += 2;
                if mode != c.g[V0][0] {
                    c.cycles += 2;
                    c.g[V0][0] = 0xa;
                    abi::ret(c);
                    return;
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0xb;
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0x16;
    }
    abi::ret(c);
}

/// `void RaceProgress_ResetRider(int rider)` (0x2105b0): stores 1 into the
/// per-rider record at `rider*0x18 + 0x4c8bc8`.
pub fn race_progress_reset_rider(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V1][0] = 0x18;
    c.g[V0][0] = 0x4d0000;
    let scaled = abi::mult(c, abi::r32(c, A0), 0x18);
    c.g[A0][0] = sx(scaled);
    c.g[V0][0] = sx(0x004d_0000u32.wrapping_sub(0x7438));
    c.g[V1][0] = 1;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void RaceProgress_ClearPathData(void)` (0x210608): clears the three
/// gp-relative path pointers.
pub fn race_progress_clear_path_data(c: &mut Cpu) {
    c.cycles += 4;
    Addr(abi::r32(c, GP).wrapping_sub(0x9a0)).sw(c, 0);
    Addr(abi::r32(c, GP).wrapping_sub(0x99c)).sw(c, 0);
    Addr(abi::r32(c, GP).wrapping_sub(0x998)).sw(c, 0);
    abi::ret(c);
}

/// `uint CrowdMan2d_AddSprite(CrowdMan2d *man, const void *sprite, uint value)`
/// (0x229738): appends `sprite` (a 16-byte quad) and `value` to the pool at
/// `man + (man->count << 5) + 0x2220`, wraps the count modulo 40 and returns
/// the new count.
pub fn crowd_man_2d_add_sprite(c: &mut Cpu) {
    let man = Addr(abi::arg(c, 0));
    let sprite = abi::arg(c, 1);
    let value = c.g[A2][0];
    c.cycles += 16;
    c.g[V1][0] = man.off(0x2214).lw(c);
    c.lq(A3, sprite);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
    c.g[A1][0] = 0x28;
    c.g[V1][0] = sx(abi::r32(c, A0).wrapping_add(c.g[V1][0] as u32));
    c.sq(A3, abi::r32(c, V1).wrapping_add(0x2220));
    c.g[V0][0] = man.off(0x2214).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
    c.g[V0][0] = sx(abi::r32(c, A0).wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V0][0] as u32).off(0x2230).sw(c, value);
    let count = man.off(0x2214).lw(c);
    c.g[V1][0] = sx((count as u32).wrapping_add(1));
    abi::div(c, abi::r32(c, V1), abi::r32(c, A1));
    man.off(0x2214).sw(c, c.g[V1][0]);
    c.cycles += 3;
    c.g[V0][0] = c.hi[0];
    man.off(0x2214).sw(c, c.g[V0][0]);
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, check_lohi, scramble, Rng, DATA, GAME_GP};

    #[test]
    fn c_fe_popup_confirm_get_centered_x_matches() {
        check(ssx3_ee::s03::f_001c7be0, c_fe_popup_confirm_get_centered_x, 300, &|c, seed| {
            let mut r = Rng(0x0c10_0001 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x2cc, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(DATA + 0x260, r.edge_f32());
            c.bus.write32(DATA + 0x1b8, r.edge_f32());
        });
    }

    #[test]
    fn c_fe_popup_confirm_position_buttons_matches() {
        check(ssx3_ee::s03::f_001c88c8, c_fe_popup_confirm_position_buttons, 300, &|c, seed| {
            let mut r = Rng(0x0c10_0002 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let child = if seed % 2 == 0 { 0 } else { DATA + 0x400 };
            c.bus.write32(DATA + 0x70, child);
            c.bus.write32(DATA + 0x1ac, r.next_u32());
            c.bus.write32(DATA + 0x1b0, r.next_u32());
            c.bus.write32(DATA + 0x1b4, r.next_u32());
            c.bus.write32(DATA + 0x290, r.edge_f32());
            c.bus.write32(DATA + 0x294, r.edge_f32());
            for i in 0..4 {
                c.bus.write32(child + 0x44 + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn c_fe_popup_confirm_accumulate_size_matches() {
        check(ssx3_ee::s03::f_001c9b68, c_fe_popup_confirm_accumulate_size, 300, &|c, seed| {
            let mut r = Rng(0x0c10_0003 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = DATA as u64;
            c.g[A2][0] = (DATA + 0x40) as u64;
            // Force both compare outcomes: -1.0 vs 1.0 and the reverse.
            let a = if seed % 2 == 0 { 0xbf80_0000 } else { 0x3f80_0000 };
            let b = if seed % 2 == 0 { 0x3f80_0000 } else { 0xbf80_0000 };
            c.bus.write32(DATA, a);
            c.bus.write32(DATA + 4, r.edge_f32());
            c.bus.write32(DATA + 0x40 + 8, b);
            c.bus.write32(DATA + 0x40 + 0xc, r.edge_f32());
        });
    }

    #[test]
    fn c_fe_popup_confirm_get_selection_matches() {
        check(ssx3_ee::s03::f_001ca150, c_fe_popup_confirm_get_selection, 300, &|c, seed| {
            let mut r = Rng(0x0c10_0004 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let sel = if seed % 2 == 0 { 0 } else { DATA + 0x400 };
            c.bus.write32(DATA + 0x6c, sel);
            c.bus.write32(DATA + 0x168, r.next_u32());
            c.bus.write32(
                DATA + 0x30c,
                if seed % 4 == 0 { 0xffff_ffff } else { r.next_u32() & 0x7fff_ffff },
            );
            c.bus.write8(sel + 0x95, r.next_u32() as u8);
        });
    }

    #[test]
    fn c_keyboard_popup_set_max_length_matches() {
        check(ssx3_ee::s04::f_001cd088, c_keyboard_popup_set_max_length, 300, &|c, seed| {
            let mut r = Rng(0x0c10_0005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = match seed % 4 {
                0 => (r.next_u32() % 64) as u64,
                1 => 64 + (r.next_u32() % 64) as u64,
                2 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                _ => 0xffff_ffff_ffff_ffff,
            };
            c.bus.write32(DATA + 0x60, r.next_u32());
        });
    }

    #[test]
    fn c_keyboard_popup_set_min_length_matches() {
        check(ssx3_ee::s04::f_001cd0b0, c_keyboard_popup_set_min_length, 300, &|c, seed| {
            let mut r = Rng(0x0c10_0006 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = match seed % 4 {
                0 => (r.next_u32() % 64) as u64,
                1 => 64 + (r.next_u32() % 64) as u64,
                2 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                _ => 0xffff_ffff_ffff_ffff,
            };
            c.bus.write32(DATA + 0x64, r.next_u32());
        });
    }

    #[test]
    fn c_keyboard_popup_set_key_enabled_matches() {
        check(ssx3_ee::s04::f_001ce3c8, c_keyboard_popup_set_key_enabled, 400, &|c, seed| {
            let mut r = Rng(0x0c10_0007 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 4) as u64;
            c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.g[A3][0] = match seed % 6 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => 0x1_0000_0000,
                _ => 0xffff_ffff_ffff_ffff,
            };
        });
    }

    #[test]
    fn c_keyboard_popup_is_key_enabled_matches() {
        check(ssx3_ee::s04::f_001ce560, c_keyboard_popup_is_key_enabled, 400, &|c, seed| {
            let mut r = Rng(0x0c10_0008 ^ seed as u64);
            scramble(c, &mut r);
            let key = [15u64, 20, 5, 30][(seed % 4) as usize];
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = key;
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { (DATA + 0x600) as u64 };
            let child = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
            c.bus.write32(DATA + 0x6c, child);
            c.bus.write32(DATA + 0x68, if seed % 5 == 0 { 0 } else { r.next_u32() });
            let base = DATA + (key as u32) * 4;
            c.bus.write32(base + 0x148, r.next_u32());
            c.bus.write32(base + 0x298, r.next_u32());
            c.bus.write32(DATA + 0x600, r.next_u32());
        });
    }

    #[test]
    fn c_fe_state_net_help_popup_set_button_help_matches() {
        check(
            ssx3_ee::s04::f_001da510,
            c_fe_state_net_help_popup_set_button_help,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c10_0009 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.g[A3][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_net_help_popup_set_body_text_matches() {
        check(
            ssx3_ee::s04::f_001da530,
            c_fe_state_net_help_popup_set_body_text,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c10_000a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn message_center_is_message_of_type_matches() {
        check_lohi(ssx3_ee::s04::f_001e2d58, message_center_is_message_of_type, 200, &|c, seed| {
            let mut r = Rng(0x0c10_000b ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 4) as u32;
            c.g[A1][0] = index as u64;
            let entry = 0x004c_6c08u32.wrapping_add(index.wrapping_mul(0x18));
            let p = 0x10u32 + (seed % 3) * 4;
            c.bus.write32(entry + 4, p);
            let q = p.wrapping_mul(0x14).wrapping_add(0x0044_1630);
            let value = r.next_u32();
            c.bus.write32(q + 0xc, value);
            c.g[A0][0] = if seed % 2 == 0 {
                sx(value)
            } else {
                sx(value ^ 0xdead_beef)
            };
        });
    }

    #[test]
    fn load_indicator_ctor_matches() {
        check(ssx3_ee::s04::f_001f30c0, load_indicator_ctor, 50, &|c, seed| {
            let mut r = Rng(0x0c10_000c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..4 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn load_indicator_clear_matches() {
        check(ssx3_ee::s04::f_001f3170, load_indicator_clear, 50, &|c, seed| {
            let mut r = Rng(0x0c10_000d ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..4 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }

    #[test]
    fn load_indicator_set_busy_matches() {
        check(ssx3_ee::s04::f_001f36c0, load_indicator_set_busy, 200, &|c, seed| {
            let mut r = Rng(0x0c10_000e ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = match seed % 4 {
                0 => (r.next_u32() % 32) as u64,
                1 => 32 + (r.next_u32() % 32) as u64,
                2 => 0xffff_ffff_ffff_ffff,
                _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
            };
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn load_indicator_clear_busy_matches() {
        check(ssx3_ee::s04::f_001f36d8, load_indicator_clear_busy, 300, &|c, seed| {
            let mut r = Rng(0x0c10_000f ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let bit = (seed % 32) as u32;
            c.g[A1][0] = bit as u64;
            let flags = match seed % 3 {
                0 => 1u32 << bit,
                1 => (1u32 << bit) | 0x100,
                _ => r.next_u32(),
            };
            c.bus.write32(DATA + 4, flags);
            c.bus.write32(DATA + 8, r.next_u32());
        });
    }

    #[test]
    fn event_select_on_widget_notify_matches() {
        check(ssx3_ee::s05::f_00202738, event_select_on_widget_notify, 200, &|c, seed| {
            let mut r = Rng(0x0c10_0010 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A2][0] = match seed % 4 {
                0 => 6,
                1 => 9,
                2 => 8,
                _ => 0x100,
            };
        });
    }

    #[test]
    fn game_ui_get_results_screen_id_matches() {
        check(ssx3_ee::s05::f_0020a8b0, game_ui_get_results_screen_id, 400, &|c, seed| {
            let mut r = Rng(0x0c10_0011 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = match seed % 6 {
                0 => 6 + (seed as u64 % 6),
                1 => 4 + (seed as u64 % 2),
                2 => 0,
                3 => 2,
                4 => (1u64 << 32) | 2,
                _ => (r.next_u32() % 16) as u64,
            };
            c.g[A1][0] = if seed % 2 == 0 { 3 } else { (r.next_u32() % 5) as u64 };
        });
    }

    #[test]
    fn race_progress_reset_rider_matches() {
        check_lohi(ssx3_ee::s05::f_002105b0, race_progress_reset_rider, 100, &|c, seed| {
            let mut r = Rng(0x0c10_0012 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = (seed % 8) as u64;
        });
    }

    #[test]
    fn race_progress_clear_path_data_matches() {
        check(ssx3_ee::s05::f_00210608, race_progress_clear_path_data, 50, &|c, seed| {
            let mut r = Rng(0x0c10_0013 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP - 0x9a0, r.next_u32());
            c.bus.write32(GAME_GP - 0x99c, r.next_u32());
            c.bus.write32(GAME_GP - 0x998, r.next_u32());
        });
    }

    #[test]
    fn crowd_man_2d_add_sprite_matches() {
        check_lohi(ssx3_ee::s05::f_00229738, crowd_man_2d_add_sprite, 200, &|c, seed| {
            let mut r = Rng(0x0c10_0014 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.bus.write32(DATA + 0x2214, (seed % 8) as u32);
            for i in 0..4 {
                c.bus.write32(DATA + 0x100 + i * 4, r.next_u32());
            }
            c.bus.write32(DATA + 0x2230, r.next_u32());
        });
    }
}
